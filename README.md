# Zero-to-Hero Counter Contract

This contract features a Members list, which is a list of cool addresses that are allowed to reset any counter and add other addresses to the list, a Waiting list where individuals can express interest to join the members list, and 3 counters: first is count which is a counter anyone can increment, x factor which is a counter only users whose address contains an X can increment, and member only count which, as the name suggests, can only be incremented by members.

In addition to exploring the website, you can interact with the smart contract directly using the command-line interface (CLI) commands:

Query Messages:

- Get Count: Retrieve the current count from the smart contract.
    secretcli query compute query secret1gurx9n0v7jnhx4sk2dqs0y6lx06n84ajyj72g7 '{"get_count": {}}'

- Get X Factor: Retrieve the current x factor from the smart contract.
    secretcli query compute query secret1gurx9n0v7jnhx4sk2dqs0y6lx06n84ajyj72g7 '{"get_x_factor": {}}'

- Get Members Only Count: Retrieve the current members only count from the smart contract.
    secretcli query compute query secret1gurx9n0v7jnhx4sk2dqs0y6lx06n84ajyj72g7 '{"get_members_only_count": {}}'

- Get Waiting List: Retrieve the current address on the waiting list from the smart contract.
    secretcli query compute query secret1gurx9n0v7jnhx4sk2dqs0y6lx06n84ajyj72g7 '{"get_waiting_list": {}}'

- Get Members List: Retrieve the current members/admin list from the smart contract.
    secretcli query compute query secret1gurx9n0v7jnhx4sk2dqs0y6lx06n84ajyj72g7 '{"get_member_list": {}}'

Execution Messages:

- Increment Count: Add one to the current count from the smart contract.
    secretcli tx compute execute secret1gurx9n0v7jnhx4sk2dqs0y6lx06n84ajyj72g7 '{"increment": {}}'

- Increment X Factor: Add one to the current x factor from the smart contract.
    secretcli tx compute execute secret1gurx9n0v7jnhx4sk2dqs0y6lx06n84ajyj72g7 '{"increment_x_factor": {}}'

- Increment Members Only Count: Add one to the current members only count from the smart contract.
    secretcli tx compute execute secret1gurx9n0v7jnhx4sk2dqs0y6lx06n84ajyj72g7 '{"increment_members_only_count": {}}'

- Add Me to Waiting List: Add the sender's address to the waiting list from the smart contract.
    secretcli tx compute execute secret1gurx9n0v7jnhx4sk2dqs0y6lx06n84ajyj72g7 '{"add_me_to_waiting_list": {}}'

Execution Messages (for members only):

- Add Member to Club: Add an address to the members list of the smart contract. **Note!** Please replace ADDRESS with the secret network address you want to add.
    secretcli tx compute execute secret1gurx9n0v7jnhx4sk2dqs0y6lx06n84ajyj72g7 '{"add_member_to_club": {"prospect": ADDRESS}}'

- Add Waiting List to Club: Add all addresses on the waiting list to the members list of the smart contract.
    secretcli tx compute execute secret1gurx9n0v7jnhx4sk2dqs0y6lx06n84ajyj72g7 '{"add_waiting_list_to_club": {}}'

- Reset: Reset the count's state on the smart contract.
    secretcli tx compute execute secret1gurx9n0v7jnhx4sk2dqs0y6lx06n84ajyj72g7 '{"reset": {"count": 0}}'

- Reset X Factor: Reset the x factor's state on the smart contract.
    secretcli tx compute execute secret1gurx9n0v7jnhx4sk2dqs0y6lx06n84ajyj72g7 '{"reset_x_factor": {"x_factor": 0}}'

- Reset Member Only Count: Reset the members only count's state on the smart contract.
    secretcli tx compute execute secret1gurx9n0v7jnhx4sk2dqs0y6lx06n84ajyj72g7 '{"reset_members_only_count": {}}'

