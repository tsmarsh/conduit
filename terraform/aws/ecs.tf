resource "aws_ecs_cluster" "main" {
  name = "${local.prefix}-cluster"

  tags = local.tags
}

resource "aws_cloudwatch_log_group" "conduit_api" {
  name              = "/ecs/${local.prefix}-conduit-api"
  retention_in_days = 30

  tags = local.tags
}

resource "aws_ecs_task_definition" "conduit_api" {
  family                   = "${local.prefix}-conduit-api"
  requires_compatibilities = ["FARGATE"]
  network_mode             = "awsvpc"
  cpu                      = var.container_cpu
  memory                   = var.container_memory
  execution_role_arn       = aws_iam_role.ecs_execution.arn
  task_role_arn            = aws_iam_role.ecs_task.arn

  container_definitions = jsonencode([
    {
      name      = "conduit-api"
      image     = "${aws_ecr_repository.conduit_api.repository_url}:latest"
      essential = true

      portMappings = [
        {
          containerPort = var.container_port
          protocol      = "tcp"
        }
      ]

      environment = [
        { name = "FUNCTIONS_CUSTOMHANDLER_PORT", value = tostring(var.container_port) },
        { name = "MERKQL_DATA_PATH", value = "/mnt/merkql" }
      ]

      mountPoints = [
        {
          sourceVolume  = "merkql"
          containerPath = "/mnt/merkql"
          readOnly      = false
        }
      ]

      logConfiguration = {
        logDriver = "awslogs"
        options = {
          "awslogs-group"         = aws_cloudwatch_log_group.conduit_api.name
          "awslogs-region"        = var.aws_region
          "awslogs-stream-prefix" = "conduit-api"
        }
      }

      healthCheck = {
        command     = ["CMD-SHELL", "curl -f http://localhost:${var.container_port}/ || exit 1"]
        interval    = 30
        timeout     = 5
        retries     = 3
        startPeriod = 10
      }
    }
  ])

  volume {
    name = "merkql"

    efs_volume_configuration {
      file_system_id     = aws_efs_file_system.merkql.id
      transit_encryption = "ENABLED"

      authorization_config {
        access_point_id = aws_efs_access_point.merkql.id
        iam             = "ENABLED"
      }
    }
  }

  tags = local.tags
}

resource "aws_security_group" "ecs_tasks" {
  name_prefix = "${local.prefix}-ecs-tasks-"
  vpc_id      = aws_vpc.main.id

  ingress {
    from_port       = var.container_port
    to_port         = var.container_port
    protocol        = "tcp"
    security_groups = [aws_security_group.alb.id]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = merge(local.tags, { Name = "${local.prefix}-ecs-tasks-sg" })
}

resource "aws_ecs_service" "conduit_api" {
  name            = "${local.prefix}-conduit-api"
  cluster         = aws_ecs_cluster.main.id
  task_definition = aws_ecs_task_definition.conduit_api.arn
  desired_count   = var.desired_count
  launch_type     = "FARGATE"

  network_configuration {
    subnets         = aws_subnet.private[*].id
    security_groups = [aws_security_group.ecs_tasks.id]
  }

  load_balancer {
    target_group_arn = aws_lb_target_group.conduit_api.arn
    container_name   = "conduit-api"
    container_port   = var.container_port
  }

  depends_on = [aws_lb_listener.http]
}

# ALB
resource "aws_security_group" "alb" {
  name_prefix = "${local.prefix}-alb-"
  vpc_id      = aws_vpc.main.id

  ingress {
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = merge(local.tags, { Name = "${local.prefix}-alb-sg" })
}

resource "aws_lb" "main" {
  name               = "${local.prefix}-alb"
  internal           = false
  load_balancer_type = "application"
  security_groups    = [aws_security_group.alb.id]
  subnets            = aws_subnet.public[*].id

  tags = local.tags
}

resource "aws_lb_target_group" "conduit_api" {
  name        = "${local.prefix}-tg"
  port        = var.container_port
  protocol    = "HTTP"
  vpc_id      = aws_vpc.main.id
  target_type = "ip"

  health_check {
    path                = "/"
    healthy_threshold   = 2
    unhealthy_threshold = 3
    timeout             = 5
    interval            = 30
    matcher             = "200-404"
  }

  tags = local.tags
}

resource "aws_lb_listener" "http" {
  load_balancer_arn = aws_lb.main.arn
  port              = 80
  protocol          = "HTTP"

  default_action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.conduit_api.arn
  }

  tags = local.tags
}
