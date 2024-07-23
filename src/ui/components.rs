use std::sync::Arc;

use bevy::prelude::*;
use solana_sdk::signature::Keypair;

use crate::NavItemScreen;

// Components
#[derive(Component, Default)]
pub struct ScrollingList {
    pub position: f32,
}


#[derive(Component)]
pub struct ScrollingListNode(pub bool);

#[derive(Component)]
pub struct MovingScrollPanel;

#[derive(Component)]
pub struct TextActiveMinersThisEpoch;

#[derive(Component)]
pub struct TextActiveMinersLastEpoch;

/// Marker to find the container entity so we can show/hide the FPS counter
#[derive(Component)]
pub struct FpsRoot;

/// Marker to find the text entity so we can update it
#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct TextHighestDifficultySeen;

#[derive(Component)]
pub struct TextCrownStakeAmount;

#[derive(Component)]
pub struct TextCrownStakeBy;

#[derive(Component)]
pub struct BaseScreenNode;

#[derive(Component)]
pub struct NavItem(pub NavItemScreen);

#[derive(Component)]
pub struct TxPopUpArea;

#[derive(Component)]
pub struct InitialSetupScreenNode;

#[derive(Component)]
pub struct WalletSetupScreenNode;

#[derive(Component)]
pub struct MiningScreenNode;

#[derive(Component)]
pub struct AppScreenParent;

#[derive(Component)]
pub struct LockedScreenNode;

#[derive(Component)]
pub struct DashboardScreenNode;

#[derive(Component)]
pub struct SettingsConfigScreenNode;

#[derive(Component)]
pub struct SettingsGeneralScreenNode;

#[derive(Component)]
pub struct SettingsWalletScreenNode;

#[derive(Component)]
pub struct CopyableText {
    pub full_text: String,
}

#[derive(Component)]
pub struct TextInput {
    pub hidden: bool,
    pub numbers_only: bool,
    pub text: String,
}

#[derive(Component)]
pub struct TextCursor;

#[derive(Component)]
pub struct TextGeneratedKeypair(pub Arc<Keypair>);

#[derive(Component)]
pub struct TextMnemonicLine1;

#[derive(Component)]
pub struct TextMnemonicLine2;

#[derive(Component)]
pub struct TextMnemonicLine3;

#[derive(Component)]
pub struct TextPasswordInput;

#[derive(Component)]
pub struct TextPasswordLabel;

#[derive(Component)]
pub struct TextWalletPubkey;

#[derive(Component)]
pub struct DashboardProofUpdatesLogsList;

#[derive(Component)]
pub struct DashboardProofUpdatesLogsListItem;

#[derive(Component)]
pub struct MiningScreenTxResultList;

#[derive(Component)]
pub struct ButtonOpenWebTxExplorer;

#[derive(Component)]
pub struct TextWalletSolBalance;

#[derive(Component)]
pub struct TextWalletOreBalance;

#[derive(Component)]
pub struct TextCurrentChallenge;

#[derive(Component)]
pub struct TextTotalHashes;

#[derive(Component)]
pub struct TextTotalRewards;

#[derive(Component)]
pub struct TextLastClaimAt;

#[derive(Component)]
pub struct TextBurnAmount;

#[derive(Component)]
pub struct TextCurrentStake;

#[derive(Component)]
pub struct TextLastHashAt;

#[derive(Component)]
pub struct TextTreasuryBalance;
#[derive(Component)]
pub struct TextTreasuryAdmin;

#[derive(Component)]
pub struct TextTreasuryDifficulty;

#[derive(Component)]
pub struct TextTreasuryLastResetAt;

#[derive(Component)]
pub struct TextTreasuryRewardRate;

#[derive(Component)]
pub struct TextTreasuryTotalClaimedRewards;

#[derive(Component)]
pub struct TextBus1;

#[derive(Component)]
pub struct TextBus2;

#[derive(Component)]
pub struct TextBus3;

#[derive(Component)]
pub struct TextBus4;

#[derive(Component)]
pub struct TextBus5;

#[derive(Component)]
pub struct TextBus6;

#[derive(Component)]
pub struct TextBus7;

#[derive(Component)]
pub struct TextBus8;

#[derive(Component)]
pub struct TextMinerStatusStatus;

#[derive(Component)]
pub struct TextMinerStatusTime;

#[derive(Component)]
pub struct TextMinerStatusThreads;

#[derive(Component)]
pub struct TextMinerStatusCpuUsage;

#[derive(Component)]
pub struct TextMinerStatusRamUsage;

#[derive(Component)]
pub struct TextHashrate;

#[derive(Component)]
pub struct TextCurrentTxSig;

#[derive(Component)]
pub struct TextCurrentTxStatus;

#[derive(Component)]
pub struct TextCurrentTxElapsed;

#[derive(Component)]
pub struct TextConfigInputRpcUrl;

#[derive(Component)]
pub struct TextConfigInputThreads;

#[derive(Component)]
pub struct TextConfigInputRpcFetchAccountsInterval;

#[derive(Component)]
pub struct TextConfigInputRpcSendTxInterval;

#[derive(Component)]
pub struct TextTxProcessorTxType;

#[derive(Component)]
pub struct ButtonAutoScroll(pub bool);

#[derive(Component)]
pub struct ButtonAutoReset(pub bool);

#[derive(Component)]
pub struct AutoScrollCheckIcon;

#[derive(Component)]
pub struct ButtonCopyText;

#[derive(Component)]
pub struct ToggleAutoMine(pub bool);

#[derive(Component)]
pub struct ButtonGenerateWallet;

#[derive(Component)]
pub struct ButtonSaveGeneratedWallet;

#[derive(Component)]
pub struct ButtonUnlock;

#[derive(Component)]
pub struct ButtonLock;

#[derive(Component)]
pub struct ButtonClaimOreRewards;

#[derive(Component)]
pub struct ButtonStakeOre;

#[derive(Component)]
pub struct ButtonRequestAirdrop {
    pub clicked: bool,
    pub timer: Timer,
}

#[derive(Component)]
pub struct ButtonCaptureTextInput;

#[derive(Component)]
pub struct ButtonSaveConfig;

#[derive(Component)]
pub struct SpinnerIcon;

#[derive(Component)]
pub struct ButtonCooldownSpinner;
