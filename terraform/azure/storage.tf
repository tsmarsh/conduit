resource "azurerm_storage_account" "merkql" {
  name                     = replace("${var.project}${var.environment}${var.region_code}sa", "-", "")
  resource_group_name      = azurerm_resource_group.main.name
  location                 = azurerm_resource_group.main.location
  account_tier             = "Standard"
  account_replication_type = "LRS"

  tags = local.tags
}

resource "azurerm_storage_share" "merkql" {
  name               = "merkql"
  storage_account_id = azurerm_storage_account.merkql.id
  quota              = 5
}
