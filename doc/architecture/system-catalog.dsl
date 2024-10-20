workspace {
    model {
        # Identity Provisioning
        eip = softwareSystem "Identity Provisioning" {
            description "Manages Authentication/Authorization across Systems."
            tags "Integration System" "Identity Provisioning"
        }

        # Manufacturing
        emf = softwareSystem "Manufacturing" {
            description "Manages Data and Domain Requirements."
            tags "Core Domain" "Manufacturing"
            -> eip "Verifies Identity of System" "HTTP/2" {
                tags "Software System" "Manufacturing" "Identity Provisioning"
            }
        }

        # Management
        emn = softwareSystem "Management" {
            description "Manages ERP/User/App settings, licenses, permissions, features, templates, etc."
            tags "Integration System" "Management"
            -> emf "Gets API Metadata" "HTTP/2" {
                tags "Software System" "Management" "Manufacturing"
            }
            -> eip "Verifies Identity of System/User" "HTTP/2" {
                tags "Software System" "Management" "Identity Provisioning"
            }
        }

        # ERP Connectivity
        eec = softwareSystem "ERP Connectivity" {
            description "Manages Connectivity with ERP Connectors."
            tags "Integration System" "ERP Connectivity"
            -> emf "Provides Data from Connector" "HTTP/2" {
                tags "Software System" "ERP Connectivity" "Manufacturing"
            }
            -> emn "Gets ERP Connectivity for Connector" "HTTP/2" {
                tags "Software System" "ERP Connectivity" "Management"
            }
            -> eip "Verifies Identity of System/Connector" "HTTP/2" {
                tags "Software System" "ERP Connectivity" "Identity Provisioning"
            }
        }

        # App Connectivity
        eac = softwareSystem "App Connectivity" {
            description "Manages Connectivity with App Connectors."
            tags "Integration System" "App Connectivity"
            -> emf "Sends Queries and Commands" "HTTP/2" {
                tags "Software System" "App Connectivity" "Manufacturing"
            }
            -> emn "Gets App Connectivity to Erponomics" "HTTP/2" {
                tags "Software System" "App Connectivity" "Management"
            }
            -> eip "Verifies Identity of System/Connector/User" "HTTP/2" {
                tags "Software System" "App Connectivity" "Identity Provisioning"
            }
        }

        # Application Manager
        app-man = person "Application Manager" {
            description "Manages App Connector."
            tags "Manager" "App"
            -> emn "Sets up Connectivity for App Connector" "HTTP/2" {
                tags "Manager" "App" "Software System" "Management"
            }
        }

        # Domain Manager
        dom-man = person "Domain Manager" {
            description "Manages Domain."
            tags "Manager" "Core Domain"
            -> emn "Sets up Domain" "HTTP/2" {
                tags "Manager" "Core Domain" "Software System" "Management"
            }
        }

        # ERP Manager
        erp-man = person "ERP Manager" {
            description "Manages ERP Connector."
            tags "Manager" "ERP"
            -> emn "Sets up Connectivity for ERP Connector" "HTTP/2" {
                tags "Manager" "ERP" "Software System" "Management"
            }
        }

        # Organization Manager
        org-man = person "Organization Manager" {
            description "Manages Organization."
            tags "Manager" "Organization"
            -> emn "Sets up Organization" "HTTP/2" {
                tags "Manager" "Organization" "Software System" "Management"
            }
        }

        # SBO Connector
        sbo = softwareSystem "SBO Connector" {
            description "Implements Erponomics ERP API for SAP Business One."
            tags "External" "ERP"
            -> eec "Synchronizes Data from SBO" "HTTP/2" {
                tags "External" "ERP" "Software System" "ERP Connectivity"
            }
        }

        # MSBC Connector
        msbc = softwareSystem "MSBC Connector" {
            description "Implements Erponomics ERP API for Microsoft Business Central."
            tags "External" "ERP"
            -> eec "Synchronizes Data from MSBC" "HTTP/2" {
                tags "External" "ERP" "Software System" "ERP Connectivity"
            }
        }

        # APS Connector
        aps = softwareSystem "APS Connector" {
            description "Implements Erponomics ERP API for Azalhu Production Services."
            tags "External" "ERP"
            -> eec "Synchronizes Data from APS" "HTTP/2" {
                tags "External" "ERP" "Software System" "ERP Connectivity"
            }
        }

        # Odoo Connector
        odoo = softwareSystem "Odoo Connector" {
            description "Implements Erponomics ERP API for Odoo."
            tags "External" "ERP"
            -> eec "Synchronizes Data from Odoo" "HTTP/2" {
                tags "External" "ERP" "Software System" "ERP Connectivity"
            }
        }

        # Build Connector
        build = softwareSystem "Build Connector" {
            description "Implements Erponomics App API for Build."
            tags "External" "App"
            -> eac "Sends Queries and Commands for Build" "HTTP/2" {
                tags "External" "App" "Software System" "App Connectivity"
            }
        }

        # Schedule Connector
        schd = softwareSystem "Schedule Connector" {
            description "Implements Erponomics App API for Schedule."
            tags "External" "App"
            -> eac "Sends Queries and Commands for Schedule" "HTTP/2" {
                tags "External" "App" "Software System" "App Connectivity"
            }
        }

        # Produce Connector
        prod = softwareSystem "Produce Connector" {
            description "Implements Erponomics App API for Produce."
            tags "External" "App"
            -> eac "Sends Queries and Commands for Produce" "HTTP/2" {
                tags "External" "App" "Software System" "App Connectivity"
            }
        }

        # Inspect Connector
        insp = softwareSystem "Inspect Connector" {
            description "Implements Erponomics App API for Inspect."
            tags "External" "App"
            -> eac "Sends Queries and Commands for Inspect" "HTTP/2" {
                tags "External" "App" "Software System" "App Connectivity"
            }
        }
    }

    views {
        styles {
            element "Person" {
                background #08427b
                shape Person
            }
            element "Software System" {
                background #1168bd
            }
            element "Container" {
                background #438dd5
            }
            element "Database" {
                shape Cylinder
            }
            element "Queue" {
                shape Pipe
            }
            element "Component" {
                background #85bbf0
            }
            element "External" {
                background #999999
            }
        }
    }
}
