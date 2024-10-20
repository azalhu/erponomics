workspace extends ../system-catalog.dsl {
    name "Erponomics"
    description "Ergonomic ERP agnostic platform"

    views {
        systemLandscape "Erponomics" {
            include *
            exclude "relationship.tag!=Software System"
            description "The system landscape diagram for Erponomics."
        }
    }

    configuration {
        scope landscape
    }
}
