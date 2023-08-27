pub const OSMOSIS_VERSION: &str = include_str!("prost/osmosis/OSMOSIS_COMMIT");

pub mod osmosis {
    pub mod accum {
        pub mod v1beta1 {
            include!("prost/osmosis/osmosis.accum.v1beta1.rs");
        }
    }

    pub mod concentratedliquidity {
        include!("prost/osmosis/osmosis.concentratedliquidity.rs");

        pub mod poolmodel {
            pub mod concentrated {
                pub mod v1beta1 {
                    include!("prost/osmosis/osmosis.concentratedliquidity.poolmodel.concentrated.v1beta1.rs");
                }
            }
        }

        pub mod v1beta1 {
            include!("prost/osmosis/osmosis.concentratedliquidity.v1beta1.rs");
        }
    }

    pub mod cosmwasmpool {
        pub mod v1beta1 {
            include!("prost/osmosis/osmosis.cosmwasmpool.v1beta1.rs");
        }
    }

    pub mod downtimedetector {
        pub mod v1beta1 {
            include!("prost/osmosis/osmosis.downtimedetector.v1beta1.rs");
        }
    }

    pub mod epochs {
        pub mod v1beta1 {
            include!("prost/osmosis/osmosis.epochs.v1beta1.rs");
        }
    }

    pub mod gamm {
        pub mod poolmodels {
            pub mod balancer {
                pub mod v1beta1 {
                    include!("prost/osmosis/osmosis.gamm.poolmodels.balancer.v1beta1.rs");
                }
            }

            pub mod stableswap {
                pub mod v1beta1 {
                    include!("prost/osmosis/osmosis.gamm.poolmodels.stableswap.v1beta1.rs");
                }
            }
        }

        pub mod v1beta1 {
            include!("prost/osmosis/osmosis.gamm.v1beta1.rs");
        }

        pub mod v2 {
            include!("prost/osmosis/osmosis.gamm.v2.rs");
        }
    }

    pub mod ibchooks {
        include!("prost/osmosis/osmosis.ibchooks.rs");
    }

    pub mod ibcratelimmit {
        pub mod v1beta1 {
            include!("prost/osmosis/osmosis.ibcratelimit.v1beta1.rs");
        }
    }

    pub mod incentives {
        include!("prost/osmosis/osmosis.incentives.rs");
    }

    pub mod lockup {
        include!("prost/osmosis/osmosis.lockup.rs");
    }

    pub mod mint {
        pub mod v1beta1 {
            include!("prost/osmosis/osmosis.mint.v1beta1.rs");
        }
    }

    pub mod poolincentives {
        pub mod v1beta1 {
            include!("prost/osmosis/osmosis.poolincentives.v1beta1.rs");
        }
    }

    pub mod poolmanager {
        pub mod v1beta1 {
            include!("prost/osmosis/osmosis.poolmanager.v1beta1.rs");
        }
    }

    pub mod protorev {
        pub mod v1beta1 {
            include!("prost/osmosis/osmosis.protorev.v1beta1.rs");
        }
    }

    pub mod store {
        pub mod v1beta1 {
            include!("prost/osmosis/osmosis.store.v1beta1.rs");
        }
    }

    pub mod superfluid {
        include!("prost/osmosis/osmosis.superfluid.rs");

        pub mod v1beta1 {
            include!("prost/osmosis/osmosis.superfluid.v1beta1.rs");
        }
    }

    pub mod tokenfactory {
        pub mod v1beta1 {
            include!("prost/osmosis/osmosis.tokenfactory.v1beta1.rs");
        }
    }

    pub mod twap {
        pub mod v1beta1 {
            include!("prost/osmosis/osmosis.twap.v1beta1.rs");
        }
    }

    pub mod txfees {
        pub mod v1beta1 {
            include!("prost/osmosis/osmosis.txfees.v1beta1.rs");
        }
    }

    pub mod valsetpref {
        pub mod v1beta1 {
            include!("prost/osmosis/osmosis.valsetpref.v1beta1.rs");
        }
    }
}
