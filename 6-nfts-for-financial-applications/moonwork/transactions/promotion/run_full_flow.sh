#!/bin/bash

# Creates a promotion service from MoonWork service
source ./transactions/promotion/1_create_promotion_service.sh
# Promotes a contractor
source ./transactions/promotion/2_promote_contractor.sh
# Passes some time (epoch) which expires the promotion, remove expired promotion, ending the contractor's promotion
source ./transactions/promotion/3_remove_expired_promotions.sh

