output "container_app_url" {
  description = "Container App FQDN"
  value       = "https://${azurerm_container_app.conduit_api.ingress[0].fqdn}"
}

output "acr_login_server" {
  description = "ACR login server"
  value       = azurerm_container_registry.main.login_server
}

output "resource_group_name" {
  description = "Resource group name"
  value       = azurerm_resource_group.main.name
}
