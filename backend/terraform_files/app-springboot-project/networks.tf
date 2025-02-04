resource "google_compute_network" "default" {
  name                                      = "default"
  description                               = "Default network for the project"
  auto_create_subnetworks                   = true
  routing_mode                              = "REGIONAL"
  network_firewall_policy_enforcement_order = "AFTER_CLASSIC_FIREWALL"
  bgp_best_path_selection_mode              = "LEGACY"

  peering {
    name                                = "redis-peer-530004490669"
    network                             = "https://www.googleapis.com/compute/v1/projects/qc0316a1a769b1132p-tp/global/networks/default-redis-e619463a-57d4-4053-a70d-b8c275ad4d49"
    export_custom_routes                = false
    import_custom_routes                = false
    export_subnet_routes_with_public_ip = true
    import_subnet_routes_with_public_ip = true
  }
}
