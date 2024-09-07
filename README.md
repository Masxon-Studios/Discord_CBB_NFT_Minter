Community Badge Minting Bot
A Rust-based Discord bot that allows community managers and admins to mint unique NFT badges on the Klaytn blockchain. The bot takes the Discord username, wallet address, and the role (e.g., admin, volunteer, moderator) to create and assign badges to community members.

Features
Mint NFT badges directly from a Discord chat
Role-based badge creation (admin, volunteer, moderator)
Integrated with the Klaytn blockchain using the KIP17 standard
Simple and secure interactions with the blockchain using Web3
Uses Discord usernames for ease of role assignments
Requirements
Rust toolchain installed (installation guide)
Discord bot token
Klaytn node access (e.g., via Baobab)
.env file with necessary configurations
Setup and Installation
Step 1: Clone the Repository
bash
Copy code
git clone https://github.com/LurkyLunk/Discord_CBB_NFT_Minter.git
cd community-badge-bot
Step 2: Install Dependencies
bash
Copy code
cargo build
Step 3: Create a .env File
Create a .env file in the root directory and populate it with your credentials:

env
Copy code
DISCORD_TOKEN=your_discord_bot_token
KLAYTN_WALLET_ADDRESS=your_wallet_address
PRIVATE_KEY=your_private_key
CONTRACT_ADDRESS=your_contract_address
CONTRACT_ABI=your_contract_abi_in_json_format
Step 4: Running the Bot
bash
Copy code
cargo run
Usage
Once the bot is running, you can use the following command in any Discord channel:

bash
Copy code
!mintbadge <username> <wallet_address> <role>
Example:

bash
Copy code
!mintbadge @Username 0x1234... Admin
This command will mint an NFT badge to the specified wallet address with the role of "Admin" and include the user's Discord name in the process.

Contributing
Contributions are welcome! Feel free to open issues and pull requests. Please follow the Rust community’s Code of Conduct.

License
This project is licensed under the MIT License - see the LICENSE file for details.

