use serenity::model::prelude::*;
use serenity::prelude::*;

pub fn has_permission(
    cache: &mut Context,
    member: Member,
    guild: Guild,
    permission: Permissions,
) -> Vec<bool> {
    let mut a = vec![false];

    for r in member.roles {
        a.append(&mut vec![r
            .to_role_cached(&cache)
            .unwrap()
            .has_permission(permission)]);
    }

    if guild.owner_id.0 == member.user.read().id.0 {
        a.append(&mut vec![true]);
    }

    return a;
}
