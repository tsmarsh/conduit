resource "aws_efs_file_system" "merkql" {
  encrypted        = true
  throughput_mode   = "elastic"
  performance_mode  = "generalPurpose"

  tags = merge(local.tags, { Name = "${local.prefix}-merkql" })
}

resource "aws_efs_mount_target" "merkql" {
  count           = 2
  file_system_id  = aws_efs_file_system.merkql.id
  subnet_id       = aws_subnet.private[count.index].id
  security_groups = [aws_security_group.efs.id]
}

resource "aws_efs_access_point" "merkql" {
  file_system_id = aws_efs_file_system.merkql.id

  posix_user {
    uid = 1001
    gid = 1001
  }

  root_directory {
    path = "/merkql"
    creation_info {
      owner_uid   = 1001
      owner_gid   = 1001
      permissions = "755"
    }
  }

  tags = merge(local.tags, { Name = "${local.prefix}-merkql-ap" })
}

resource "aws_security_group" "efs" {
  name_prefix = "${local.prefix}-efs-"
  vpc_id      = aws_vpc.main.id

  ingress {
    from_port       = 2049
    to_port         = 2049
    protocol        = "tcp"
    security_groups = [aws_security_group.ecs_tasks.id]
  }

  tags = merge(local.tags, { Name = "${local.prefix}-efs-sg" })
}
