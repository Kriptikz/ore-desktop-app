# Unofficial Ore Mining Desktop App

- OreV2 is under active development. The on-chain program can be changed, updated, or deleted at any time causing this desktop app to stop functioning properly.
- This desktop app is also under active development. Use at your own discretion.


## Getting Started

### Download
To quickly get started, build are provided as releases. This project is currently in alpha. [Download Here](https://github.com/Kriptikz/ore-desktop-app/releases)
Navigate to the Release, download the zip for your platform. Extract the zip into a folder at your desired location (A folder on Desktop will work).
This zip should have an assets folder, and an executable. Make sure the assets are always right next to your executable or it won't be able to find them and your screen will just be blank.

### Build from Source

To get started you will need to have rust installed as well as any additional dependencies for Bevy. 
[Bevy Getting Started Setup](https://bevyengine.org/learn/quick-start/getting-started/setup/)

Drillx and the Ore Program are currently being updated, so you will need to clone those repos. Then make sure to update the paths for these dependencies in the Cargo.toml file for this app.
Once you have Drillx, Ore, and this app cloned, and have ensured the Cargo.toml paths are correct. Finally, run `cargo run --release` for the desktop app to build and run.

### Running
First you will need to download the release and extract it. [releases](https://github.com/Kriptikz/ore-desktop-app/releases)

![GIF 5-16-2024 7-31-25 PM](https://github.com/Kriptikz/ore-desktop-app/assets/17520593/36e847c7-6d55-4b03-a41e-7a307f67fce9)

Once the app is successfully running, you should see a config screen. You can provide your own rpc url, or leave it as the default.
For the best experience when interacting with the Solana network it is recommended that you get your own rpc and don't rely on the free public one for important work.
 

![GIF 5-16-2024 7-32-21 PM](https://github.com/Kriptikz/ore-desktop-app/assets/17520593/cf0c3c43-e145-4b2a-87dd-18dcae63cea2)



After the Config Setup screen you will need to generate/import a wallet. To generate a wallet, click generate, record your seed phrase, add a password or leave it empty for a blank password.
To import a .json wallet. Just drag-and-drop the .json file into the app and it will update the shown public key. Verify it's valid, add a password and click save.

Once the password is complete you will be taken to the Unlock screen. Here you must enter your previous password, and then click unlock or press `enter`.

![GIF 5-16-2024 7-45-07 PM](https://github.com/Kriptikz/ore-desktop-app/assets/17520593/1adc1d1b-8f8b-4828-99fa-1bb2e75befac)


Now you will be on the Mining Screen. Wait for the ui fetch interval to kick in and update all the balance. Then toggle the Mine switch to start mining.

![GIF 5-16-2024 7-52-06 PM](https://github.com/Kriptikz/ore-desktop-app/assets/17520593/595ed325-6a70-4dea-98cc-c0c060c54c5f)

If you do not have any sol, you will need to get some. You can try the `Devnet` Airdrop button that is in the app. But there are heavy rate limits on it.
You can also try the quicknode faucet. [quicknode faucet](https://faucet.quicknode.com/solana/devnet)

#### Notes:
The Orange bus is the last bus that was used.
The Green flash of the bus is when the transaction was sent and processed on it.

Use the `c` key to get to the config screen again from the mining screen.

The `save.data` file is the password encrypted keypair. If you delete it, you will be prompted to generate/import a new one on the next run.
