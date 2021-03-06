// Creating mock runtime here

use crate::*;
use auction_traits::auction::*;
use frame_support::{
    assert_err, assert_noop, assert_ok, impl_outer_event, impl_outer_origin, parameter_types,
    traits::{BalanceStatus, OnFinalize, OnInitialize},
};
use frame_system::{self as system};
use pallet_balances::{self as balances};
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    Perbill,
};

impl_outer_origin! {
    pub enum Origin for AuctionTestRuntime {}
}

// For testing the pallet, we construct most of a mock runtime. This means
// first constructing a configuration type (`Test`) which `impl`s each of the
// configuration traits of pallets we want to use.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct AuctionTestRuntime;

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaximumBlockWeight: u32 = 1024;
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);

    pub const ExistentialDeposit: u64 = 1;
    pub const TransferFee: u64 = 0;
    pub const CreationFee: u64 = 0;
}

pub type AccountId = u64;
pub type Balance = u64;
pub type BlockNumber = u64;
pub type AuctionId = u64;
pub type GeneralInformationContainer = u64;

impl system::Trait for AuctionTestRuntime {
    type Origin = Origin;
    type Call = ();
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = AuctionTestEvent;
    type BlockHashCount = BlockHashCount;
    type MaximumBlockWeight = MaximumBlockWeight;
    type MaximumBlockLength = MaximumBlockLength;
    type AvailableBlockRatio = AvailableBlockRatio;
    type Version = ();
    type ModuleToIndex = ();
    type AccountData = balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type DbWeight = ();
    type BaseCallFilter = ();
    type BlockExecutionWeight = ();
    type ExtrinsicBaseWeight = ();
    type MaximumExtrinsicWeight = MaximumBlockWeight;
    type SystemWeightInfo = ();
}

pub struct Handler;

impl AuctionHandler<AccountId, Balance, BlockNumber, AuctionId> for Handler {
    fn on_new_bid(
        _now: BlockNumber,
        _id: AuctionId,
        _new_bid: (AccountId, Balance),
        _last_bid: Option<(AccountId, Balance)>,
    ) -> OnNewBidResult<BlockNumber> {
        if let Some(bid) = _last_bid {
            println!(
                "Last bid information [{0:#?}] \
                Current bid information [{1:#?}]",
                bid, _new_bid
            );
        } else if let None = _last_bid {
            println!("First bid on auction [{:#?}]", _id);
        }
        OnNewBidResult {
            accept_bid: true,
            auction_end: None,
        }
    }

    fn on_auction_ended(
        _id: AuctionId,
        _recipients: (AccountId, AccountId),
        _winner: Option<(AccountId, Balance)>,
    ) {
        if let Some(winner) = _winner {
            // Somebody has won, notify
            AuctionModule::transfer_funds(&winner.0, &_recipients.0, winner.1);
            AuctionModule::deposit_event(RawEvent::AuctionEndDecided(winner.0, _id));
            println!("The winner: {:?}", winner);
        } else if let None = _winner {
            // Nobody has won, notify
            AuctionModule::deposit_event(RawEvent::AuctionEndUndecided(_id));
            println!("There were no bids, nobody has won");
        }
    }
}

impl balances::Trait for AuctionTestRuntime {
    type Balance = Balance;
    type Event = AuctionTestEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = system::Module<AuctionTestRuntime>;
    type WeightInfo = ();
}

impl Trait for AuctionTestRuntime {
    type Event = AuctionTestEvent;
    type Currency = balances::Module<Self>;
    type AuctionId = AccountId;
    type Handler = Handler;
    type GeneralInformationContainer = GeneralInformationContainer;
}

pub type System = system::Module<AuctionTestRuntime>;
pub type Balances = balances::Module<AuctionTestRuntime>;
pub type AuctionModule = Module<AuctionTestRuntime>;

mod auction_events {
    pub use crate::Event;
}

impl_outer_event! {
    pub enum AuctionTestEvent for AuctionTestRuntime {
        auction_events<T>,
        system<T>,
        balances<T>,
    }
}

// Simulating block production for the general auction tests
fn run_to_block(n: u64) {
    while System::block_number() < n {
        AuctionModule::on_finalize(System::block_number());
        System::on_finalize(System::block_number());
        System::set_block_number(System::block_number() + 1);

        AuctionModule::on_initialize(System::block_number());
        System::on_initialize(System::block_number());
    }
}

pub struct EnvBuilder {
    balances: Vec<(u64, u64)>,
    auctions: Vec<(
        AccountId,
        AccountId,
        Vec<u64>, //AuctionCoreInfo<<tests::AuctionTestRuntime as Trait>::GeneralInformationContainer>,
        BlockNumber,
        BlockNumber,
    )>,
}

impl EnvBuilder {
    pub fn new() -> Self {
        // Config :: Vec<(T::AccountId | Barge, T::AccountId | Terminal, AuctionCoreInfo, T::BlockNumber | Start, T::BlockNumber | End)>;
        Self {
            balances: vec![
                (0, 5000000), // Hamza
                (1, 20000),   // Barge
                (2, 20000),   // Barge
                (3, 20000),   // Barge
                (4, 20000),   // Barge
                (5, 40000),   // Terminal
                (6, 40000),   // Terminal
                (7, 40000),   // Terminal
                (8, 40000),   // Terminal
            ],
            auctions: vec![
                // Start these auctions from origin
                (
                    1,
                    6,
                    //AuctionCoreInfo {
                    //    timestamp: 1594471764,
                    //    cargo: (22, 22),
                    //},
                    vec![1u64, 2u64, 3u64],
                    0,
                    60000,
                ),
                (
                    1,
                    5,
                    //AuctionCoreInfo {
                    //    timestamp: 1594471764,
                    //    cargo: (22, 22),
                    //},
                    vec![1u64, 2u64, 3u64],
                    0,
                    49,
                ),
                (
                    2,
                    6,
                    //AuctionCoreInfo {
                    //    timestamp: 1594471764,
                    //    cargo: (22, 22),
                    //},
                    vec![1u64, 2u64, 3u64],
                    0,
                    51,
                ),
                (
                    3,
                    7,
                    //AuctionCoreInfo {
                    //    timestamp: 1594471764,
                    //    cargo: (22, 22),
                    //},
                    vec![1u64, 2u64, 3u64],
                    0,
                    150,
                ),
                (
                    4,
                    8,
                    //AuctionCoreInfo {
                    //    timestamp: 1594471764,
                    //    cargo: (22, 22),
                    //},
                    vec![1u64, 2u64, 3u64],
                    0,
                    250,
                ),
                // Start these auctions from block 100+ for the testing of the queues.
                (
                    1,
                    5,
                    //AuctionCoreInfo {
                    //    timestamp: 1594471764,
                    //    cargo: (22, 22),
                    //},
                    vec![1u64, 2u64, 3u64],
                    100,
                    500,
                ),
                (
                    2,
                    6,
                    //AuctionCoreInfo {
                    //    timestamp: 1594471764,
                    //    cargo: (22, 22),
                    //},
                    vec![1u64, 2u64, 3u64],
                    140,
                    600,
                ),
            ],
        }
    }
}

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let core = EnvBuilder::new();
    let mut t = system::GenesisConfig::default()
        .build_storage::<AuctionTestRuntime>()
        .unwrap();
    balances::GenesisConfig::<AuctionTestRuntime> {
        balances: core.balances,
    }
    .assimilate_storage(&mut t)
    .unwrap();
    GenesisConfig::<AuctionTestRuntime> {
        _auctions: core.auctions,
    }
    .assimilate_storage(&mut t)
    .unwrap();
    t.into()
}

#[test]
fn new_test_ext_qqq() {
    new_test_ext().execute_with(|| {

        run_to_block(5);
        AuctionModule::bid(Origin::signed(1), 1, 21000);
        let xxx = AuctionModule::auction_query_formal(1);
        println!("{}", Balance::default());
        println!("{:?}", xxx);
    })
}



#[test]
fn new_test_ext_behaves() {
    new_test_ext().execute_with(|| {
        assert_eq!(Balances::free_balance(&1), 20000);
    })
}

// Currency
/////////////////////////////////////////////////////////
#[test]
fn new_test_ext_transfer() {
    new_test_ext().execute_with(|| {
        assert_ok!(Balances::transfer(Origin::signed(1), 2, 1000));
        assert_eq!(Balances::free_balance(&2), 21000);
        assert_ok!(Balances::transfer(Origin::signed(2), 1, 1000));
        assert_eq!(Balances::free_balance(&1), 20000);
    })
}
/////////////////////////////////////////////////////////
// Reservable
/////////////////////////////////////////////////////////
#[test]
fn new_test_ext_can_reserve() {
    new_test_ext().execute_with(|| {
        assert_eq!(Balances::can_reserve(&1, 29), true);
        assert_eq!(Balances::can_reserve(&1, 20001), false);
    })
}

#[test]
fn new_test_ext_reserve() {
    new_test_ext().execute_with(|| {
        assert_ok!(Balances::reserve(&1, 10000));
        assert_eq!(Balances::free_balance(&1), 10000);
        assert!(Balances::reserve(&1, 31000).is_err());
    })
}
//https://substrate.dev/rustdocs/master/src/frame_support/traits.rs.html#725-783
#[test]
fn new_test_ext_unreserver() {
    new_test_ext().execute_with(|| {
        assert_ok!(Balances::reserve(&1, 10000));
        assert_eq!(Balances::free_balance(&1), 10000);
        assert_eq!(Balances::unreserve(&1, 10000), 0);
        assert_ok!(Balances::reserve(&1, 20000));
    })
}

#[test]
fn new_test_ext_slash_reserve() {
    new_test_ext().execute_with(|| {
        assert_ok!(Balances::reserve(&1, 10000));
        assert_eq!(Balances::free_balance(&1), 10000);
        assert_eq!(Balances::reserved_balance(&1), 10000);
        let slash_res = Balances::slash_reserved(&1, 10000);
        assert_eq!(Balances::reserved_balance(&1), 0);
        assert_eq!(Balances::free_balance(&1), 10000);
    })
}

#[test]
fn new_test_ext_repatriate_reserved() {
    new_test_ext().execute_with(|| {
        assert_ok!(Balances::reserve(&1, 10000));
        assert_eq!(Balances::free_balance(&1), 10000);
        assert_eq!(Balances::reserved_balance(&1), 10000);
        assert_ok!(Balances::repatriate_reserved(
            &1,
            &2,
            Balances::reserved_balance(&1),
            BalanceStatus::Free
        ));
        assert_eq!(Balances::reserved_balance(&1), 0);
        assert_eq!(Balances::free_balance(&1), 10000);
        assert_eq!(Balances::free_balance(&2), 30000);
    })
}
///////////////////////////////////////////////////////
// Auction related tests
///////////////////////////////////////////////////////
#[test]
fn new_test_ext_new_auction() {
    new_test_ext().execute_with(|| {
        assert_eq!(AuctionModule::auction_exists(0), true);
        assert_eq!(AuctionModule::auction_exists(1), true);
        assert_eq!(AuctionModule::auction_exists(2), true);
        assert_eq!(AuctionModule::auction_exists(3), true);
    })
}

#[test]
fn new_test_ext_auction_expire() {
    new_test_ext().execute_with(|| {
        // Auction 0 -- Block 49
        // Auction 1 -- Block 51
        assert_eq!(AuctionModule::auction_exists(0), true);
        assert_eq!(AuctionModule::auction_exists(1), true);
        // Run to block 55, which should end auction 0 && 1
        run_to_block(55);
        // At this point auction 0 and 1 should be dumped.
        assert_eq!(AuctionModule::auction_exists(0), false);
        assert_eq!(AuctionModule::auction_exists(1), false);

        let (expiry_1, expiry_2) = (
            AuctionTestEvent::auction_events(RawEvent::AuctionEndUndecided(0)),
            AuctionTestEvent::auction_events(RawEvent::AuctionEndUndecided(1)),
        );
        assert!(System::events().iter().any(|a| a.event == expiry_1));
        assert!(System::events().iter().any(|a| a.event == expiry_2));
    })
}

#[test]
fn new_test_ext_auction_bidding() {
    new_test_ext().execute_with(|| {
        // Bid(AuctionId, AccountId, Balance)
        // Ensure that Auction 0 exists
        assert_eq!(AuctionModule::auction_exists(0), true);
        // All barges have 40000 currencies so bid in sequences of 5000
        // Bid on auction 0 which has a start block of 0 which should let us pass this bid.
        run_to_block(5);
        assert_ok!(AuctionModule::bid(Origin::signed(5), 0, 5000));
        assert_ok!(AuctionModule::bid(Origin::signed(1), 0, 10000));
        assert_ok!(AuctionModule::bid(Origin::signed(2), 0, 20000));

        assert_eq!(
            System::events()
                .into_iter()
                .filter_map(|e| {
                    if let AuctionTestEvent::auction_events(inner) = e.event {
                        Some(inner)
                    } else {
                        None
                    }
                })
                .last()
                .unwrap(),
            RawEvent::Bid(0, 2, 20000)
        );

        run_to_block(50);
        assert_eq!(
            System::events()
                .into_iter()
                .filter_map(|e| {
                    if let AuctionTestEvent::auction_events(event) = e.event {
                        Some(event)
                    } else {
                        None
                    }
                })
                .last()
                .unwrap(),
            RawEvent::AuctionEndDecided(2, 0)
        );
    })
}

#[test]
fn new_test_ext_auction_queued_bidding() {
    new_test_ext().execute_with(|| {
        // Create new auction set for block 100+
        // Block 0
        run_to_block(1);
        // Block 1 :: Bid should be put up for queue
        assert_ok!(AuctionModule::bid(Origin::signed(1), 5, 10000));
        // Queue should be emptied on block 140 so let's jump ten ahead. At this point the queued
        // bid should be placed on the auction in question and the auction will be updated.
        run_to_block(150);
        // Queue another bid for the same acution. The lower of the two bids should be dropped.
        assert_ok!(AuctionModule::bid(Origin::signed(2), 5, 11000));
        run_to_block(160);
        // When trying to bid lower we should be served a InvalidBidPrice error.
        assert_noop!(
            AuctionModule::bid(Origin::signed(3), 5, 10000),
            Error::<AuctionTestRuntime>::InvalidBidPrice
        );
        // Run to the end block, which for this auction is block 600, jump 10 ahead and everything
        // should be finalized with an AuctionEndDecided().
        run_to_block(610);
        assert_eq!(
            System::events()
                .into_iter()
                .filter_map(|e| {
                    if let AuctionTestEvent::auction_events(event) = e.event {
                        Some(event)
                    } else {
                        None
                    }
                })
                .last()
                .unwrap(),
            RawEvent::AuctionEndDecided(2, 5)
        );
    });
}

#[test]
fn new_test_ext_bidding_currency() {
    new_test_ext().execute_with(|| {
        // Bid and test the reservation capabilities
        // Overbid and test the slashing capabilities
        // Complete an auction and test the sending capabilities
        run_to_block(1);
        assert_ok!(AuctionModule::bid(Origin::signed(3), 0, 10000));
        assert_eq!(Balances::reserved_balance(&3), 10000);
        assert_ok!(AuctionModule::bid(Origin::signed(4), 0, 15000));
        assert_eq!(Balances::reserved_balance(&4), 15000);
        assert_eq!(Balances::reserved_balance(&3), 0);
        assert_eq!(Balances::free_balance(&1), 20000);

        // Now, after the auctions have finished, check the balances to ensure that (1, pb-10000)
        // and (2, pb-15000) where pb stands for previous balance.
        run_to_block(100);
        assert_eq!(Balances::free_balance(&3), 20000);
        assert_eq!(Balances::free_balance(&4), 5000);

        assert_eq!(Balances::free_balance(&1), 35000);
    })
}

#[test]
fn new_test_ext_auction_query() {
    new_test_ext().execute_with(|| {
        // let tau = AuctionModule::auction_query_informal(1);
        run_to_block(60);
        // println!("{:#?}", tau);
        // Retrieve all auctions regardless of status
        // AuctionModule::auction_all_query_informal();
        // println!("{:?}", AuctionModule::auction_query_formal(1));
        // Retrieve all inactive auction
        // AuctionModule::auction_all_query_informal_status(false, 10);
        let inactive_formal_auctions = AuctionModule::auction_query_formal_all_status(true);
        for a in inactive_formal_auctions {
            println!("{:?}", a);
        }
    })
}
