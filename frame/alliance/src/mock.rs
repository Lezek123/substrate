// This file is part of Substrate.

// Copyright (C) 2019-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Test utilities

pub use sp_core::H256;
pub use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};
use sp_std::convert::{TryFrom, TryInto};

pub use frame_support::{
	assert_noop, assert_ok, ord_parameter_types, parameter_types,
	traits::{EitherOfDiverse, GenesisBuild, SortedMembers},
	BoundedVec,
};
use frame_system::{EnsureRoot, EnsureSignedBy};
use pallet_identity::{Data, IdentityInfo, Judgement};

pub use crate as pallet_alliance;

use super::*;

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}
impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
	pub const MaxLocks: u32 = 10;
}
impl pallet_balances::Config for Test {
	type Balance = u64;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = MaxLocks;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
}

parameter_types! {
	pub const MotionDuration: u64 = 3;
	pub const MaxProposals: u32 = 100;
	pub const MaxMembers: u32 = 100;
}
type AllianceCollective = pallet_collective::Instance1;
impl pallet_collective::Config<AllianceCollective> for Test {
	type Origin = Origin;
	type Proposal = Call;
	type Event = Event;
	type MotionDuration = MotionDuration;
	type MaxProposals = MaxProposals;
	type MaxMembers = MaxMembers;
	type DefaultVote = pallet_collective::PrimeDefaultVote;
	type WeightInfo = ();
}

parameter_types! {
	pub const BasicDeposit: u64 = 10;
	pub const FieldDeposit: u64 = 10;
	pub const SubAccountDeposit: u64 = 10;
	pub const MaxSubAccounts: u32 = 2;
	pub const MaxAdditionalFields: u32 = 2;
	pub const MaxRegistrars: u32 = 20;
}
ord_parameter_types! {
	pub const One: u64 = 1;
	pub const Two: u64 = 2;
	pub const Three: u64 = 3;
	pub const Four: u64 = 4;
	pub const Five: u64 = 5;
}
type EnsureOneOrRoot = EitherOfDiverse<EnsureRoot<u64>, EnsureSignedBy<One, u64>>;
type EnsureTwoOrRoot = EitherOfDiverse<EnsureRoot<u64>, EnsureSignedBy<Two, u64>>;

impl pallet_identity::Config for Test {
	type Event = Event;
	type Currency = Balances;
	type BasicDeposit = BasicDeposit;
	type FieldDeposit = FieldDeposit;
	type SubAccountDeposit = SubAccountDeposit;
	type MaxSubAccounts = MaxSubAccounts;
	type MaxAdditionalFields = MaxAdditionalFields;
	type MaxRegistrars = MaxRegistrars;
	type Slashed = ();
	type RegistrarOrigin = EnsureOneOrRoot;
	type ForceOrigin = EnsureTwoOrRoot;
	type WeightInfo = ();
}

pub struct AllianceIdentityVerifier;
impl IdentityVerifier<u64> for AllianceIdentityVerifier {
	fn has_identity(who: &u64, fields: u64) -> bool {
		Identity::has_identity(who, fields)
	}

	fn has_good_judgement(who: &u64) -> bool {
		if let Some(judgements) =
			Identity::identity(who).map(|registration| registration.judgements)
		{
			judgements
				.iter()
				.any(|(_, j)| matches!(j, Judgement::KnownGood | Judgement::Reasonable))
		} else {
			false
		}
	}

	fn super_account_id(who: &u64) -> Option<u64> {
		Identity::super_of(who).map(|parent| parent.0)
	}
}

pub struct AllianceProposalProvider;
impl ProposalProvider<u64, H256, Call> for AllianceProposalProvider {
	fn propose_proposal(
		who: u64,
		threshold: u32,
		proposal: Box<Call>,
		length_bound: u32,
	) -> Result<(u32, u32), DispatchError> {
		AllianceMotion::do_propose_proposed(who, threshold, proposal, length_bound)
	}

	fn vote_proposal(
		who: u64,
		proposal: H256,
		index: ProposalIndex,
		approve: bool,
	) -> Result<bool, DispatchError> {
		AllianceMotion::do_vote(who, proposal, index, approve)
	}

	fn veto_proposal(proposal_hash: H256) -> u32 {
		AllianceMotion::do_disapprove_proposal(proposal_hash)
	}

	fn close_proposal(
		proposal_hash: H256,
		proposal_index: ProposalIndex,
		proposal_weight_bound: Weight,
		length_bound: u32,
	) -> DispatchResultWithPostInfo {
		AllianceMotion::do_close(proposal_hash, proposal_index, proposal_weight_bound, length_bound)
	}

	fn proposal_of(proposal_hash: H256) -> Option<Call> {
		AllianceMotion::proposal_of(proposal_hash)
	}
}

parameter_types! {
	pub const MaxFounders: u32 = 10;
	pub const MaxFellows: u32 = MaxMembers::get() - MaxFounders::get();
	pub const MaxAllies: u32 = 100;
	pub const AllyDeposit: u64 = 25;
}
impl Config for Test {
	type Event = Event;
	type Proposal = Call;
	type AdminOrigin = EnsureSignedBy<One, u64>;
	type MembershipManager = EnsureSignedBy<Two, u64>;
	type AnnouncementOrigin = EnsureSignedBy<Three, u64>;
	type Currency = Balances;
	type Slashed = ();
	type InitializeMembers = AllianceMotion;
	type MembershipChanged = AllianceMotion;
	#[cfg(not(feature = "runtime-benchmarks"))]
	type IdentityVerifier = AllianceIdentityVerifier;
	#[cfg(feature = "runtime-benchmarks")]
	type IdentityVerifier = ();
	type ProposalProvider = AllianceProposalProvider;
	type MaxProposals = MaxProposals;
	type MaxFounders = MaxFounders;
	type MaxFellows = MaxFellows;
	type MaxAllies = MaxAllies;
	type MaxUnscrupulousItems = ConstU32<100>;
	type MaxWebsiteUrlLength = ConstU32<255>;
	type MaxAnnouncementsCount = ConstU32<100>;
	type MaxMembersCount = MaxMembers;
	type AllyDeposit = AllyDeposit;
	type WeightInfo = ();
}

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		Balances: pallet_balances,
		Identity: pallet_identity,
		AllianceMotion: pallet_collective::<Instance1>,
		Alliance: pallet_alliance,
	}
);

pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

	pallet_balances::GenesisConfig::<Test> {
		balances: vec![(1, 50), (2, 50), (3, 50), (4, 50), (5, 30), (6, 50), (7, 50)],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	GenesisBuild::<Test>::assimilate_storage(
		&pallet_alliance::GenesisConfig {
			founders: vec![],
			fellows: vec![],
			allies: vec![],
			phantom: Default::default(),
		},
		&mut t,
	)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| {
		assert_ok!(Identity::add_registrar(Origin::signed(1), 1));

		let info = IdentityInfo {
			additional: BoundedVec::default(),
			display: Data::Raw(b"name".to_vec().try_into().unwrap()),
			legal: Data::default(),
			web: Data::Raw(b"website".to_vec().try_into().unwrap()),
			riot: Data::default(),
			email: Data::default(),
			pgp_fingerprint: None,
			image: Data::default(),
			twitter: Data::default(),
		};
		assert_ok!(Identity::set_identity(Origin::signed(1), Box::new(info.clone())));
		assert_ok!(Identity::provide_judgement(Origin::signed(1), 0, 1, Judgement::KnownGood));
		assert_ok!(Identity::set_identity(Origin::signed(2), Box::new(info.clone())));
		assert_ok!(Identity::provide_judgement(Origin::signed(1), 0, 2, Judgement::KnownGood));
		assert_ok!(Identity::set_identity(Origin::signed(3), Box::new(info.clone())));
		assert_ok!(Identity::provide_judgement(Origin::signed(1), 0, 3, Judgement::KnownGood));
		assert_ok!(Identity::set_identity(Origin::signed(4), Box::new(info.clone())));
		assert_ok!(Identity::provide_judgement(Origin::signed(1), 0, 4, Judgement::KnownGood));
		assert_ok!(Identity::set_identity(Origin::signed(5), Box::new(info.clone())));
		assert_ok!(Identity::provide_judgement(Origin::signed(1), 0, 5, Judgement::KnownGood));
		assert_ok!(Identity::set_identity(Origin::signed(6), Box::new(info.clone())));

		// Joining before init should fail.
		assert_noop!(
			Alliance::join_alliance(Origin::signed(1)),
			Error::<Test, ()>::AllianceNotYetInitialized
		);

		assert_ok!(Alliance::init_members(Origin::root(), vec![1, 2], vec![3], vec![]));

		System::set_block_number(1);
	});
	ext
}

#[cfg(feature = "runtime-benchmarks")]
pub fn new_bench_ext() -> sp_io::TestExternalities {
	GenesisConfig::default().build_storage().unwrap().into()
}

pub fn test_cid() -> Cid {
	use sha2::{Digest, Sha256};
	let mut hasher = Sha256::new();
	hasher.update(b"hello world");
	let result = hasher.finalize();
	Cid::new_v0(&*result)
}

pub fn make_proposal(value: u64) -> Call {
	Call::System(frame_system::Call::remark { remark: value.encode() })
}

pub fn make_set_rule_proposal(rule: Cid) -> Call {
	Call::Alliance(pallet_alliance::Call::set_rule { rule })
}

pub fn make_kick_member_proposal(who: u64) -> Call {
	Call::Alliance(pallet_alliance::Call::kick_member { who })
}
