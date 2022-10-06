Description: Async Backing Version Test
Network: ./async-backing-test.toml
Creds: config

# General
alice: is up
bob: is up

# Check authority status 
alice: reports node_roles is 4
bob: reports node_roles is 4

# Check peers
alice: reports peers count is at least 2 within 20 seconds
bob: reports peers count is at least 2 within 20 seconds

# Parachain registration
alice: parachain 100 is registered within 225 seconds
bob: parachain 100 is registered within 225 seconds

# Ensure parachain progress
alice: parachain 100 block height is at least 10 within 200 seconds
bob: parachain 100 block height is at least 10 within 200 seconds

# Finality lag
alice: reports polkadot_parachain_approval_checking_finality_lag is 0
bob: reports polkadot_parachain_approval_checking_finality_lag is 0

# Dispute lag
alice: reports polkadot_parachain_disputes_finality_lag is 0
bob: reports polkadot_parachain_disputes_finality_lag is 0
