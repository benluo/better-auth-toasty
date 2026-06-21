use async_trait::async_trait;
use better_auth_core::{
    AuthResult, CreateApiKey, CreateInvitation, CreateMember, CreateOrganization,
    CreatePasskey, CreateTwoFactor, InvitationOps, InvitationStatus, ApiKeyOps,
    MemberOps, OrganizationOps, PasskeyOps, TwoFactorOps, UpdateApiKey,
    UpdateOrganization,
};
use better_auth_core::types::{
    ApiKey, Invitation, Member, Organization, Passkey, TwoFactor,
};

use crate::adapter::ToastyAdapter;

#[async_trait]
impl OrganizationOps for ToastyAdapter {
    type Organization = Organization;

    async fn create_organization(&self, _org: CreateOrganization) -> AuthResult<Self::Organization> { todo!() }
    async fn get_organization_by_id(&self, _id: &str) -> AuthResult<Option<Self::Organization>> { todo!() }
    async fn get_organization_by_slug(&self, _slug: &str) -> AuthResult<Option<Self::Organization>> { todo!() }
    async fn update_organization(&self, _id: &str, _update: UpdateOrganization) -> AuthResult<Self::Organization> { todo!() }
    async fn delete_organization(&self, _id: &str) -> AuthResult<()> { todo!() }
    async fn list_user_organizations(&self, _user_id: &str) -> AuthResult<Vec<Self::Organization>> { todo!() }
}

#[async_trait]
impl MemberOps for ToastyAdapter {
    type Member = Member;

    async fn create_member(&self, _member: CreateMember) -> AuthResult<Self::Member> { todo!() }
    async fn get_member(&self, _organization_id: &str, _user_id: &str) -> AuthResult<Option<Self::Member>> { todo!() }
    async fn get_member_by_id(&self, _id: &str) -> AuthResult<Option<Self::Member>> { todo!() }
    async fn update_member_role(&self, _member_id: &str, _role: &str) -> AuthResult<Self::Member> { todo!() }
    async fn delete_member(&self, _member_id: &str) -> AuthResult<()> { todo!() }
    async fn list_organization_members(&self, _organization_id: &str) -> AuthResult<Vec<Self::Member>> { todo!() }
    async fn count_organization_members(&self, _organization_id: &str) -> AuthResult<usize> { todo!() }
    async fn count_organization_owners(&self, _organization_id: &str) -> AuthResult<usize> { todo!() }
}

#[async_trait]
impl InvitationOps for ToastyAdapter {
    type Invitation = Invitation;

    async fn create_invitation(&self, _invitation: CreateInvitation) -> AuthResult<Self::Invitation> { todo!() }
    async fn get_invitation_by_id(&self, _id: &str) -> AuthResult<Option<Self::Invitation>> { todo!() }
    async fn get_pending_invitation(&self, _organization_id: &str, _email: &str) -> AuthResult<Option<Self::Invitation>> { todo!() }
    async fn update_invitation_status(&self, _id: &str, _status: InvitationStatus) -> AuthResult<Self::Invitation> { todo!() }
    async fn list_organization_invitations(&self, _organization_id: &str) -> AuthResult<Vec<Self::Invitation>> { todo!() }
    async fn list_user_invitations(&self, _email: &str) -> AuthResult<Vec<Self::Invitation>> { todo!() }
}

#[async_trait]
impl TwoFactorOps for ToastyAdapter {
    type TwoFactor = TwoFactor;

    async fn create_two_factor(&self, _two_factor: CreateTwoFactor) -> AuthResult<Self::TwoFactor> { todo!() }
    async fn get_two_factor_by_user_id(&self, _user_id: &str) -> AuthResult<Option<Self::TwoFactor>> { todo!() }
    async fn update_two_factor_backup_codes(&self, _user_id: &str, _backup_codes: &str) -> AuthResult<Self::TwoFactor> { todo!() }
    async fn delete_two_factor(&self, _user_id: &str) -> AuthResult<()> { todo!() }
}

#[async_trait]
impl ApiKeyOps for ToastyAdapter {
    type ApiKey = ApiKey;

    async fn create_api_key(&self, _input: CreateApiKey) -> AuthResult<Self::ApiKey> { todo!() }
    async fn get_api_key_by_id(&self, _id: &str) -> AuthResult<Option<Self::ApiKey>> { todo!() }
    async fn get_api_key_by_hash(&self, _hash: &str) -> AuthResult<Option<Self::ApiKey>> { todo!() }
    async fn list_api_keys_by_user(&self, _user_id: &str) -> AuthResult<Vec<Self::ApiKey>> { todo!() }
    async fn update_api_key(&self, _id: &str, _update: UpdateApiKey) -> AuthResult<Self::ApiKey> { todo!() }
    async fn delete_api_key(&self, _id: &str) -> AuthResult<()> { todo!() }
    async fn delete_expired_api_keys(&self) -> AuthResult<usize> { todo!() }
}

#[async_trait]
impl PasskeyOps for ToastyAdapter {
    type Passkey = Passkey;

    async fn create_passkey(&self, _input: CreatePasskey) -> AuthResult<Self::Passkey> { todo!() }
    async fn get_passkey_by_id(&self, _id: &str) -> AuthResult<Option<Self::Passkey>> { todo!() }
    async fn get_passkey_by_credential_id(&self, _credential_id: &str) -> AuthResult<Option<Self::Passkey>> { todo!() }
    async fn list_passkeys_by_user(&self, _user_id: &str) -> AuthResult<Vec<Self::Passkey>> { todo!() }
    async fn update_passkey_counter(&self, _id: &str, _counter: u64) -> AuthResult<Self::Passkey> { todo!() }
    async fn update_passkey_name(&self, _id: &str, _name: &str) -> AuthResult<Self::Passkey> { todo!() }
    async fn delete_passkey(&self, _id: &str) -> AuthResult<()> { todo!() }
}
