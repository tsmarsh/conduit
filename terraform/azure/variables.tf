variable "project" {
  description = "Project name"
  type        = string
  default     = "conduit"
}

variable "environment" {
  description = "Deployment environment"
  type        = string
  default     = "dev"
}

variable "location" {
  description = "Azure region"
  type        = string
  default     = "eastus2"
}

variable "region_code" {
  description = "Short region code for naming"
  type        = string
  default     = "eus2"
}

variable "subscription_id" {
  description = "Azure subscription ID"
  type        = string
}

variable "container_cpu" {
  description = "Container CPU cores"
  type        = number
  default     = 0.25
}

variable "container_memory" {
  description = "Container memory in Gi"
  type        = string
  default     = "0.5Gi"
}

variable "min_replicas" {
  description = "Minimum number of replicas"
  type        = number
  default     = 1
}

variable "max_replicas" {
  description = "Maximum number of replicas"
  type        = number
  default     = 1
}

variable "container_port" {
  description = "Port the container listens on"
  type        = number
  default     = 3000
}
