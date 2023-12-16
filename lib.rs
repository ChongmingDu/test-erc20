#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod stbank {
    use ink::storage::Mapping;


    #[ink(storage)]
    pub struct Stbank {
        total_supply: Balance,
        balances: Mapping<AccountID, Balance>,
        allowance: Mapping<(AccountID, AccountID), Balance>,

    }

    pub struct Transfer {
        #[ink(topic)]
        form: Option<AccountID>,
        #[ink(topic)]
        to: Option<AccountID>,
        value: Balance,
    }

    #[ink(event)]
    pub struct TransferEvent {
        #[ink(topic)]
        form: Option<AccountID>,
        #[ink(topic)]
        to: Option<AccountID>,
        value: Balance,
    }

    #[ink(event)]
    pub struct ApprovalEvent {
        #[ink(topic)]
        owner: Option<AccountID>,
        #[ink(topic)]
        spender: Option<AccountID>,
        value: Balance,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
        InsufficientAllowance,
        Overflow,
        NonexistentAccount,
    }

    pub type Result<T> = core::result::Result<T, Error>;


    impl Stbank {
        #[ink(constructor)]
        pub fn new(total_supply: Balance, nft_owner: AccountID, lp_nft: u64) -> Self {
            // 3 pool nft addr
            let verified_nft = (
                "0x89Cf45Bbe0850C7b0D315a48730F6a602420F8Be",
            );
            if lp_nft != verified_nft {
                Error::InsufficientAllowance
            }
            Self.balances.insert(&nft_owner, &(Balance - lp_nft.len() as Balance));
            let mut balances = Mapping::default();
            let caller = Self::env().caller();
            balances.insert(caller, &total_supply);

            Self::env().emit_event(TransferEvent {
                form: None,
                to: Some(caller),
                value: total_supply,

            });
            Self {
                total_supply,
                balances,
                allowance: Mapping::default(),
            }
        }


        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }


        #[ink(message)]
        pub fn balance_of(&self, owner: AccountID) -> Balance {
            self.balances.get(&owner).copied().unwrap_or(0)
        }
    }

    #[inline]
    fn balance_of_impl(&self, owner: &AccountId) -> Balance {
        self.balances.get(owner).unwrap_or_default()
    }

    #[inline]
    fn balance_of(&self, owner: &AccountId) -> &mut Balance {
        self.balances.get(owner).unwrap()
    }
}