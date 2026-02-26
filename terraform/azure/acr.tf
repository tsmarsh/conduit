resource "azurerm_container_registry" "main" {
  name                = replace("${var.project}${var.environment}${var.region_code}acr", "-", "")
  resource_group_name = azurerm_resource_group.main.name
  location            = azurerm_resource_group.main.location
  sku                 = "Basic"
  admin_enabled       = true

  tags = local.tags
}
