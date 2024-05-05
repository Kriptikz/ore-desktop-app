use bevy::prelude::*;

// Components
#[derive(Component, Default)]
pub struct ScrollingList {
    pub position: f32,
}

#[derive(Component)]
pub struct MovingScrollPanel;

/// Marker to find the container entity so we can show/hide the FPS counter
#[derive(Component)]
pub struct FpsRoot;

/// Marker to find the text entity so we can update it
#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct BaseScreenNode;

#[derive(Component)]
pub struct InitialSetupScreenNode;

#[derive(Component)]
pub struct MiningScreenNode;

#[derive(Component)]
pub struct LockedScreenNode;

#[derive(Component)]
pub struct CopyableText {
    pub full_text: String,
}

#[derive(Component)]
pub struct TextInput {
    pub hidden: bool,
    pub text: String,
}

#[derive(Component)]
pub struct TextPasswordInput;

#[derive(Component)]
pub struct TextPasswordLabel;

#[derive(Component)]
pub struct TextWalletPubkey;

#[derive(Component)]
pub struct TextWalletSolBalance;

#[derive(Component)]
pub struct TextWalletOreBalance;

#[derive(Component)]
pub struct TextCurrentHash;

#[derive(Component)]
pub struct TextTotalHashes;

#[derive(Component)]
pub struct TextTotalRewards;

#[derive(Component)]
pub struct TextClaimableRewards;

#[derive(Component)]
pub struct TextTreasuryBalance;
#[derive(Component)]
pub struct TextTreasuryAdmin;

#[derive(Component)]
pub struct TextTreasuryDifficulty;

#[derive(Component)]
pub struct TextTreasuryLastResetAt;

#[derive(Component)]
pub struct TextTreasuryNeedEpochReset;

#[derive(Component)]
pub struct TextTreasuryRewardRate;

#[derive(Component)]
pub struct TextTreasuryTotalClaimedRewards;

#[derive(Component)]
pub struct TextMinerStatusStatus;

#[derive(Component)]
pub struct TextMinerStatusTime;

#[derive(Component)]
pub struct TextMinerStatusCpuUsage;

#[derive(Component)]
pub struct TextMinerStatusRamUsage;

#[derive(Component)]
pub struct TextCurrentTxSig;

#[derive(Component)]
pub struct TextCurrentTxStatus;

#[derive(Component)]
pub struct TextCurrentTxElapsed;

#[derive(Component)]
pub struct ButtonCopyText;

#[derive(Component)]
pub struct ButtonStartStopMining;

#[derive(Component)]
pub struct ButtonResetEpoch;

#[derive(Component)]
pub struct ButtonUnlock;

#[derive(Component)]
pub struct ButtonLock;

#[derive(Component)]
pub struct ButtonClaimOreRewards;

#[derive(Component)]
pub struct ButtonCaptureTextInput;

#[derive(Component)]
pub struct ButtonTest;
