resource "azurerm_container_app_environment" "main" {
  name                       = "${local.prefix}-cae"
  location                   = azurerm_resource_group.main.location
  resource_group_name        = azurerm_resource_group.main.name
  infrastructure_subnet_id   = azurerm_subnet.container_apps.id

  tags = local.tags
}

resource "azurerm_container_app_environment_storage" "merkql" {
  name                         = "merkql"
  container_app_environment_id = azurerm_container_app_environment.main.id
  account_name                 = azurerm_storage_account.merkql.name
  share_name                   = azurerm_storage_share.merkql.name
  access_key                   = azurerm_storage_account.merkql.primary_access_key
  access_mode                  = "ReadWrite"
}

resource "azurerm_container_app" "conduit_api" {
  name                         = "${local.prefix}-conduit-api"
  container_app_environment_id = azurerm_container_app_environment.main.id
  resource_group_name          = azurerm_resource_group.main.name
  revision_mode                = "Single"

  ingress {
    external_enabled = true
    target_port      = var.container_port

    traffic_weight {
      latest_revision = true
      percentage      = 100
    }
  }

  template {
    min_replicas = var.min_replicas
    max_replicas = var.max_replicas

    volume {
      name         = "merkql"
      storage_name = azurerm_container_app_environment_storage.merkql.name
      storage_type = "AzureFile"
    }

    container {
      name   = "conduit-api"
      image  = "${azurerm_container_registry.main.login_server}/conduit-api:latest"
      cpu    = var.container_cpu
      memory = var.container_memory

      env {
        name  = "FUNCTIONS_CUSTOMHANDLER_PORT"
        value = tostring(var.container_port)
      }

      env {
        name  = "MERKQL_DATA_PATH"
        value = "/mnt/merkql"
      }

      volume_mounts {
        name = "merkql"
        path = "/mnt/merkql"
      }

      liveness_probe {
        transport = "TCP"
        port      = var.container_port
      }
    }
  }

  registry {
    server               = azurerm_container_registry.main.login_server
    username             = azurerm_container_registry.main.admin_username
    password_secret_name = "acr-password"
  }

  secret {
    name  = "acr-password"
    value = azurerm_container_registry.main.admin_password
  }

  tags = local.tags
}
