//There is no ORM, these are just structs used for passing required fields
//for insertion and gather

pub struct DBRoom{
    pub id:i32,
    pub owner_id:i32,
    pub chat_mode:String
}
pub struct DBRoomPermissions{
    pub id:i32,
    pub user_id:i32,
    pub room_id:i32,
    pub is_mod:bool,
    pub is_speaker:bool,
    pub asked_to_speak:bool,
}
pub struct DBFollower{
    pub id:i32,
    pub follower_id:i32,
    pub user_id:i32
}
pub struct DBUser{
    pub id:i32,
    pub display_name:String,
    pub avatar_url:String,
    pub user_name:String,
    pub last_online:String,
    pub github_id:String,
    pub discord_id:String,
    pub github_access_token:String,
    pub discord_access_token:String,
    pub banned:bool,
    pub banned_reason:String,
    pub bio:String,
    pub contributions:i32,
    pub banner_url:String
}
pub struct DBUserBlock{
    pub id:i32,
    pub owner_user_id:i32,
    pub blocked_user_id:i32
}
pub struct DBRoomBlock{
    pub id:i32,
    pub owner_room_id:i32,
    pub blocked_user_id:i32
}
pub struct DBScheduledRoom{
    pub id:i32,
    pub room_name:String,
    pub num_attending:i32,
    pub scheduled_for:String,
}
pub struct DBScheduledRoomAttendance{
    pub id:i32,
    pub user_id:i32,
    pub scheduled_room_id:i32,
    pub is_owner:bool
}