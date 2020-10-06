use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::*;
use smash::lua2cpp::L2CFighterCommon;
use acmd::{acmd, acmd_func};

//This was my first rust mod I didn't know how to format variables lol
static mut _ZPOS1 : f32 = 0.0;
static mut _ZPOS2 : f32 = 0.0;
static mut _ZPOS3 : f32 = 0.0;
static mut _ZPOS4 : f32 = 0.0;
static mut _ZPOS5 : f32 = 0.0;

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PALUTENA, 
    animation = "attack_s3_s",
    animcmd = "effect_attacks3")]
pub fn palutena_effect_attack_s3(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(9)
        if(is_excute){
            FOOT_EFFECT(0x0d0da6e3c0 as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
        }
        frame(13)
        if(is_excute){
            EFFECT_FOLLOW(0x142a14d3e2 as u64, hash40("stick"), 0, 8.64999962, 0, 0, 0, 0, 1, true)
            EFFECT_FOLLOW(0x1132283c24 as u64, hash40("top"), 0, 15, 10, 90, 0, 0, 1, true)
            LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 15, 10, 90, 0, 0, 1.29999995, 0, 0, 0, 0, 0, 0, true)
        }
        frame(14)        
        if (WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(0)){
            if(is_excute){
                EFFECT_FOLLOW(0x167bfa38bf as u64, hash40("stick"), 0, 0, 0, 0, 0, 0, 1, true)
            }
        }        
        if (WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(1)){
            if(is_excute){
                EFFECT_FOLLOW(0x16e2f36905 as u64, hash40("stick"), 0, 0, 0, 0, 0, 0, 1, true)
            }
        }        
        if (WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(2)){
            if(is_excute){
                EFFECT_FOLLOW(0x1695f45993 as u64, hash40("stick"), 0, 0, 0, 0, 0, 0, 1, true)
            }
        }        
        if (WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(3)){
            if(is_excute){
                EFFECT_FOLLOW(0x160b90cc30 as u64, hash40("stick"), 0, 0, 0, 0, 0, 0, 1, true)
            }
        }        
        if (WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(4)){
            if(is_excute){
                EFFECT_FOLLOW(0x167c97fca6 as u64, hash40("stick"), 0, 0, 0, 0, 0, 0, 1, true)
            }
        }        
        if (WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(5)){
            if(is_excute){
                EFFECT_FOLLOW(0x16e59ead1c as u64, hash40("stick"), 0, 0, 0, 0, 0, 0, 1, true)
            }
        }       
        if (WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(6)){
            if(is_excute){
                EFFECT_FOLLOW(0x1692999d8a as u64, hash40("stick"), 0, 0, 0, 0, 0, 0, 1, true)
            }
        }       
        if (WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(7)){
            if(is_excute){
                EFFECT_FOLLOW(0x160226801b as u64, hash40("stick"), 0, 0, 0, 0, 0, 0, 1, true)
            }
        }
        frame(40)        
        if (WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(0)){
            if(is_excute){
                EFFECT_OFF_KIND(0x167bfa38bf as u64, false, false)
            }
        }       
        if (WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(1)){
            if(is_excute){
                EFFECT_OFF_KIND(0x16e2f36905 as u64, false, false)
            }
        }        
        if (WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(2)){
            if(is_excute){
                EFFECT_OFF_KIND(0x1695f45993 as u64, false, false)
            }
        }        
        if (WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(3)){
            if(is_excute){
                EFFECT_OFF_KIND(0x160b90cc30 as u64, false, false)
            }
        }       
        if (WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(4)){
            if(is_excute){
                EFFECT_OFF_KIND(0x167c97fca6 as u64, false, false)
            }
        }       
        if (WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(5)){
            if(is_excute){
                EFFECT_OFF_KIND(0x16e59ead1c as u64, false, false)
            }
        }        
        if (WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(6)){
            if(is_excute){
                EFFECT_OFF_KIND(0x1692999d8a as u64, false, false)
            }
        }       
        if (WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(7)){
            if(is_excute){
                EFFECT_OFF_KIND(0x160226801b as u64, false, false)
            }
        }
        frame(0, 42)
        if(is_excute){
            EFFECT_OFF_KIND(0x142a14d3e2 as u64, false, false)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PALUTENA, 
    animation = "attack_s3_s",
    animcmd = "sound_attacks3")]
pub fn palutena_sound_attack_s3(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(8)
        if(is_excute){
            PLAY_SEQUENCE(0x171fbaf133 as u64)
            PLAY_SE(0x1a2b683208 as u64)
        }
        frame(16)
        if(is_excute){
	        PLAY_SE(0x15b5e1ed7b as u64)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PALUTENA, 
    animation = "attack_s3_s",
    animcmd = "game_attacks3")]
pub fn palutena_game_attack_s3(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=2)
        FT_MOTION_RATE(FSM=0.8)
        frame(Frame=17)
        if(is_excute){
        HIT_NODE(hash40("armr"), HIT_STATUS_XLU)
            ATTACK(ID=1, Part=0, Bone=hash40("stick"), Damage=6.0, Angle=90, KBG=100, FKB=25, BKB=0, Size=4.1, X=0.0, Y=5.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=0, Part=0, Bone=hash40("stick"), Damage=6.0, Angle=45, KBG=100, FKB=45, BKB=0, Size=3.0, X=0.0, Y=-5.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=2, Part=1, Bone=hash40("top"), Damage=16.0, Angle=45, KBG=84, FKB=0, BKB=53, Size=4.5, X=0.0, Y=14.0, Z=10.0, X2=0.0, Y2=14.0, Z2=31.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=3, Part=1, Bone=hash40("top"), Damage=12.0, Angle=45, KBG=89, FKB=0, BKB=58, Size=2.8, X=0.0, Y=14.0, Z=10.0, X2=0.0, Y2=14.0, Z2=54.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=4, Part=1, Bone=hash40("top"), Damage=9.0, Angle=45, KBG=92, FKB=0, BKB=62, Size=2.8, X=0.0, Y=14.0, Z=10.0, X2=0.0, Y2=14.0, Z2=78.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
        }
        frame(Frame=20)
        if(is_excute){
            ATTACK(ID=1, Part=0, Bone=hash40("stick"), Damage=6.0, Angle=285, KBG=100, FKB=25, BKB=0, Size=4.1, X=0.0, Y=5.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=0, Part=0, Bone=hash40("stick"), Damage=6.0, Angle=100, KBG=100, FKB=40, BKB=0, Size=3.0, X=0.0, Y=-5.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        }
        frame(Frame=25)
        if(is_excute){
            ATTACK(ID=1, Part=0, Bone=hash40("stick"), Damage=6.0, Angle=20, KBG=100, FKB=35, BKB=0, Size=4.1, X=0.0, Y=5.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=0, Part=0, Bone=hash40("stick"), Damage=6.0, Angle=85, KBG=100, FKB=30, BKB=0, Size=3.0, X=0.0, Y=-5.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        }
        frame(Frame=29)
        if(is_excute){
            AttackModule::clear(ID=0, false)
            AttackModule::clear(ID=1, false)
            ATTACK(ID=1, Part=0, Bone=hash40("stick"), Damage=7.0, Angle=40, KBG=80, FKB=0, BKB=78, Size=4.8, X=0.0, Y=5.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=0, Part=0, Bone=hash40("stick"), Damage=7.0, Angle=40, KBG=80, FKB=0, BKB=78, Size=3.9, X=0.0, Y=-5.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=2, Part=1, Bone=hash40("top"), Damage=13.0, Angle=45, KBG=84, FKB=0, BKB=53, Size=2.8, X=0.0, Y=14.0, Z=10.0, X2=0.0, Y2=14.0, Z2=31.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=3, Part=1, Bone=hash40("top"), Damage=9.0, Angle=45, KBG=89, FKB=0, BKB=58, Size=2.8, X=0.0, Y=14.0, Z=10.0, X2=0.0, Y2=14.0, Z2=54.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=4, Part=1, Bone=hash40("top"), Damage=7.0, Angle=45, KBG=92, FKB=0, BKB=62, Size=2.8, X=0.0, Y=14.0, Z=10.0, X2=0.0, Y2=14.0, Z2=78.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
        }
        frame(Frame=35)
        if(is_excute){
            AttackModule::clear(ID=2, false)
            AttackModule::clear(ID=3, false)
            AttackModule::clear(ID=4, false)
        }
        frame(Frame=41)
        FT_MOTION_RATE(FSM=1)
        frame(Frame=44)
        if(is_excute){
            AttackModule::clear_all()
            HIT_NODE(hash40("armr"), HIT_STATUS_NORMAL)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PALUTENA, 
    animation = "attack_hi3",
    animcmd = "effect_attackhi3")]
pub fn palutena_effect_attack_hi3(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(7)
        if(is_excute){
            EFFECT_FOLLOW(0x142a14d3e2 as u64, hash40("stick"), 0, 8.64999962, 0, 0, 0, 0, 1, true)
            EFFECT_FOLLOW(0x1132283c24 as u64, hash40("top"), 0, 20, 0, 0, 0, 0, 1, true)
            LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 20, 0, 0, 0, 0, 1.29999995, 0, 0, 0, 0, 0, 0, true)
        }
        frame(9)
        if(is_excute){
            EFFECT_FOLLOW(0x142a14d3e2 as u64, hash40("stick"), 0, 8.64999962, 0, 0, 0, 0, 1, true)
        }
        frame(10)       
        if(WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(0)){
            if(is_excute){
                EFFECT_FOLLOW(0x167bfa38bf as u64, hash40("stick"), 0, 0, 0, 0, 0, 0, 1, true)
            }
        }       
        if(WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(1)){
            if(is_excute){
                EFFECT_FOLLOW(0x16e2f36905 as u64, hash40("stick"), 0, 0, 0, 0, 0, 0, 1, true)
            }
        }        
        if(WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(2)){
            if(is_excute){
                EFFECT_FOLLOW(0x1695f45993 as u64, hash40("stick"), 0, 0, 0, 0, 0, 0, 1, true)
            }
        }        
        if(WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(3)){
            if(is_excute){
                EFFECT_FOLLOW(0x160b90cc30 as u64, hash40("stick"), 0, 0, 0, 0, 0, 0, 1, true)
            }
        }        
        if(WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(4)){
            if(is_excute){
                EFFECT_FOLLOW(0x167c97fca6 as u64, hash40("stick"), 0, 0, 0, 0, 0, 0, 1, true)
            }
        }       
        if(WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(5)){
            if(is_excute){
                EFFECT_FOLLOW(0x16e59ead1c as u64, hash40("stick"), 0, 0, 0, 0, 0, 0, 1, true)
            }
        }        
        if(WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(6)){
            if(is_excute){
                EFFECT_FOLLOW(0x1692999d8a as u64, hash40("stick"), 0, 0, 0, 0, 0, 0, 1, true)
            }
        }        
        if(WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(7)){
            if(is_excute){
                EFFECT_FOLLOW(0x160226801b as u64, hash40("stick"), 0, 0, 0, 0, 0, 0, 1, true)
            }
        }
        frame(32)        
        if(WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(0)){
            if(is_excute){
                EFFECT_OFF_KIND(0x167bfa38bf as u64, false, false)
            }
        }        
        if(WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(1)){
            if(is_excute){
                EFFECT_OFF_KIND(0x16e2f36905 as u64, false, false)
            }
        }       
        if(WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(2)){
            if(is_excute){
                EFFECT_OFF_KIND(0x1695f45993 as u64, false, false)
            }
        }        
        if(WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(3)){
            if(is_excute){
                EFFECT_OFF_KIND(0x160b90cc30 as u64, false, false)
            }
        }        
        if(WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(4)){
            if(is_excute){
                EFFECT_OFF_KIND(0x167c97fca6 as u64, false, false)
            }
        }       
        if(WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(5)){
            if(is_excute){
                EFFECT_OFF_KIND(0x16e59ead1c as u64, false, false)
            }
        }        
        if(WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(6)){
            if(is_excute){
                EFFECT_OFF_KIND(0x1692999d8a as u64, false, false)
            }
        }        
        if(WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(7)){
            if(is_excute){
                EFFECT_OFF_KIND(0x160226801b as u64, false, false)
            }
        }
        frame(35)
        if(is_excute){
            EFFECT_OFF_KIND(0x142a14d3e2 as u64, false, false)
        }        
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PALUTENA, 
    animation = "attack_hi3",
    animcmd = "sound_attackhi3")]
pub fn palutena_sound_attack_hi3(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(6)
        if(is_excute){
            PLAY_SEQUENCE(0x171fbaf133 as u64)
            PLAY_SE(0x1a3b1b7e99 as u64)
        }
        frame(9)
        if(is_excute){
	        PLAY_SE(0x15b5e1ed7b as u64)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PALUTENA, 
    animation = "attack_hi3",
    animcmd = "game_attackhi3")]
pub fn palutena_game_attack_hi3(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=2)
        FT_MOTION_RATE(FSM=0.75)
        frame(Frame=10)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.4, Angle=367, KBG=100, FKB=20, BKB=0, Size=3.0, X=0.0, Y=18.0, Z=4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=0.3, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.4, Angle=130, KBG=100, FKB=50, BKB=0, Size=2.0, X=0.0, Y=15.0, Z=10.5, X2=0.0, Y2=15.0, Z2=-5.0, Hitlag=0.5, SDI=0.3, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=1.4, Angle=190, KBG=100, FKB=30, BKB=0, Size=2.0, X=0.0, Y=20.0, Z=10.5, X2=0.0, Y2=20.0, Z2=-5.0, Hitlag=0.5, SDI=0.3, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=3, Part=1, Bone=hash40("top"), Damage=16.0, Angle=90, KBG=84, FKB=0, BKB=53, Size=4.5, X=0.0, Y=20.0, Z=0.0, X2=0.0, Y2=41.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=4, Part=1, Bone=hash40("top"), Damage=12.0, Angle=90, KBG=89, FKB=0, BKB=58, Size=2.8, X=0.0, Y=20.0, Z=0.0, X2=0.0, Y2=64.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=5, Part=1, Bone=hash40("top"), Damage=9.0, Angle=90, KBG=92, FKB=0, BKB=62, Size=2.8, X=0.0, Y=20.0, Z=0.0, X2=0.0, Y2=88.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
        }
        frame(Frame=22)
        if(is_excute){
            ATTACK(ID=3, Part=1, Bone=hash40("top"), Damage=13.0, Angle=90, KBG=84, FKB=0, BKB=53, Size=2.8, X=0.0, Y=20.0, Z=0.0, X2=0.0, Y2=41.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=4, Part=1, Bone=hash40("top"), Damage=9.0, Angle=90, KBG=89, FKB=0, BKB=58, Size=2.8, X=0.0, Y=20.0, Z=0.0, X2=0.0, Y2=64.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=5, Part=1, Bone=hash40("top"), Damage=7.0, Angle=90, KBG=92, FKB=0, BKB=62, Size=2.8, X=0.0, Y=20.0, Z=0.0, X2=0.0, Y2=88.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
        }
        frame(Frame=28)
        if(is_excute){
            AttackModule::clear(ID=3, false)
            AttackModule::clear(ID=4, false)
            AttackModule::clear(ID=5, false)
        }
        frame(Frame=30)
        FT_MOTION_RATE(FSM=1)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=31)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.5, Angle=85, KBG=136, FKB=0, BKB=75, Size=3.0, X=0.0, Y=23.0, Z=10.5, X2=0.0, Y2=23.0, Z2=-5.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.5, Angle=85, KBG=136, FKB=0, BKB=75, Size=3.0, X=0.0, Y=17.0, Z=10.5, X2=0.0, Y2=17.0, Z2=-5.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        
        }
        frame(Frame=33)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PALUTENA, 
    animation = "attack_lw3",
    animcmd = "effect_attacklw3")]
pub fn palutena_effect_attack_lw3(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(1)
        if(is_excute){
            FOOT_EFFECT(0x0d0da6e3c0 as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
        }
        frame(11)
        if(is_excute){
            EFFECT_FOLLOW(0x1132283c24 as u64, hash40("top"), 0, 0, 10, 180, 0, 0, 1, true)
            LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 0, 10, 180, 0, 0, 1.29999995, 0, 0, 0, 0, 0, 0, true)
        }
        frame(13)        
        if(WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(0)){
            if(is_excute){
                EFFECT_FOLLOW(0x167bfa38bf as u64, hash40("stick"), 0, 0, 0, 0, 180, 0, 1, true)
            }
        }        
        if(WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(1)){
            if(is_excute){
                EFFECT_FOLLOW(0x16e2f36905 as u64, hash40("stick"), 0, 0, 0, 0, 180, 0, 1, true)
            }
        }        
        if(WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(2)){
            if(is_excute){
                EFFECT_FOLLOW(0x1695f45993 as u64, hash40("stick"), 0, 0, 0, 0, 180, 0, 1, true)
            }
        }        
        if(WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(3)){
            if(is_excute){
                EFFECT_FOLLOW(0x160b90cc30 as u64, hash40("stick"), 0, 0, 0, 0, 180, 0, 1, true)
            }
        }        
        if(WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(4)){
            if(is_excute){
                EFFECT_FOLLOW(0x167c97fca6 as u64, hash40("stick"), 0, 0, 0, 0, 180, 0, 1, true)
            }
        }        
        if(WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(5)){
            if(is_excute){
                EFFECT_FOLLOW(0x16e59ead1c as u64, hash40("stick"), 0, 0, 0, 0, 180, 0, 1, true)
            }
        }        
        if(WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(6)){
            if(is_excute){
                EFFECT_FOLLOW(0x1692999d8a as u64, hash40("stick"), 0, 0, 0, 0, 180, 0, 1, true)
            }
        }       
        if(WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(7)){
            if(is_excute){
                EFFECT_FOLLOW(0x160226801b as u64, hash40("stick"), 0, 0, 0, 0, 180, 0, 1, true)
            }
        }
        if(is_excute){
            EFFECT_FOLLOW(0x142a14d3e2 as u64, hash40("stick"), 0, 8.64999962, 0, 0, 180, 0, 1, true)
        }
        frame(29)        
        if(WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(0)){
            if(is_excute){
                EFFECT_OFF_KIND(0x167bfa38bf as u64, false, false)
            }
        }        
        if(WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(1)){
            if(is_excute){
                EFFECT_OFF_KIND(0x16e2f36905 as u64, false, false)
            }
        }       
        if(WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(2)){
            if(is_excute){
                EFFECT_OFF_KIND(0x1695f45993 as u64, false, false)
            }
        }        
        if(WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(3)){
            if(is_excute){
                EFFECT_OFF_KIND(0x160b90cc30 as u64, false, false)
            }
        }        
        if(WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(4)){
            if(is_excute){
                EFFECT_OFF_KIND(0x167c97fca6 as u64, false, false)
            }
        }        
        if(WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(5)){
            if(is_excute){
                EFFECT_OFF_KIND(0x16e59ead1c as u64, false, false)
            }
        }        
        if(WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(6)){
            if(is_excute){
                EFFECT_OFF_KIND(0x1692999d8a as u64, false, false)
            }
        }        
        if(WorkModule::get_int64(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)==(7)){
            if(is_excute){
                EFFECT_OFF_KIND(0x160226801b as u64, false, false)
            }
        }
        frame(32)
        if(is_excute){
            EFFECT_OFF_KIND(0x142a14d3e2 as u64, false, false)
        }        
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PALUTENA, 
    animation = "attack_lw3",
    animcmd = "sound_attacklw3")]
pub fn palutena_sound_attack_lw3(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(11)
        if(is_excute){
            PLAY_SEQUENCE(0x171fbaf133 as u64)
            PLAY_SE(0x1a3c12d645 as u64)
        }
        frame(13)
        if(is_excute){
            PLAY_SE(0x15b5e1ed7b as u64)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PALUTENA, 
    animation = "attack_lw3",
    animcmd = "game_attacklw3")]
pub fn palutena_game_attack_lw3(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=14)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("stick"), Damage=8.5, Angle=35, KBG=100, FKB=0, BKB=40, Size=2.7, X=-0.5, Y=8.0, Z=0.0, X2=-0.5, Y2=-7.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("stick"), Damage=5.0, Angle=75, KBG=100, FKB=0, BKB=38, Size=2.7, X=-0.5, Y=8.0, Z=0.0, X2=-0.5, Y2=-7.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=4, Part=1, Bone=hash40("top"), Damage=16.0, Angle=270, KBG=84, FKB=0, BKB=53, Size=4.5, X=0.0, Y=-21.0, Z=9.7, X2=0.0, Y2=-4.0, Z2=9.7, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=5, Part=1, Bone=hash40("top"), Damage=12.0, Angle=270, KBG=89, FKB=0, BKB=58, Size=2.8, X=0.0, Y=-44.0, Z=9.7, X2=0.0, Y2=-4.0, Z2=9.7, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=6, Part=1, Bone=hash40("top"), Damage=9.0, Angle=270, KBG=92, FKB=0, BKB=62, Size=2.8, X=0.0, Y=-68.0, Z=9.7, X2=0.0, Y2=-4.0, Z2=9.7, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
        }
        frame(Frame=15)
        if(is_excute){
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=8.5, Angle=35, KBG=100, FKB=0, BKB=40, Size=2.5, X=0.0, Y=1.0, Z=19.5, X2=0.0, Y2=2.0, Z2=9.8, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=5.0, Angle=75, KBG=100, FKB=0, BKB=38, Size=2.5, X=0.0, Y=1.0, Z=19.5, X2=0.0, Y2=2.0, Z2=9.8, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        }
        frame(Frame=26)
        if(is_excute){
            ATTACK(ID=4, Part=1, Bone=hash40("top"), Damage=13.0, Angle=270, KBG=84, FKB=0, BKB=53, Size=2.8, X=0.0, Y=-22.0, Z=9.7, X2=0.0, Y2=-2.0, Z2=9.7, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=5, Part=1, Bone=hash40("top"), Damage=9.0, Angle=270, KBG=89, FKB=0, BKB=58, Size=2.8, X=0.0, Y=-44.0, Z=9.7, X2=0.0, Y2=-4.0, Z2=9.7, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=6, Part=1, Bone=hash40("top"), Damage=7.0, Angle=270, KBG=92, FKB=0, BKB=62, Size=2.8, X=0.0, Y=-68.0, Z=9.7, X2=0.0, Y2=-4.0, Z2=9.7, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
        }
        frame(Frame=28)
        if(is_excute){
            AttackModule::clear(ID=0, false)
            AttackModule::clear(ID=1, false)
            AttackModule::clear(ID=2, false)
            AttackModule::clear(ID=3, false)
        }
        frame(Frame=32)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PALUTENA, 
    animation = "attack_dash",
    animcmd = "effect_attackdash")]
pub fn palutena_effect_attack_dash(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(6)
        if(is_excute){
            EFFECT_FOLLOW(0x15aff06357 as u64, hash40("shield"), 0, 0, 0, 0, 0, 0, 1, true)
            LAST_EFFECT_SET_RATE(0.5)
            EFFECT_FOLLOW_NO_STOP(0x09d58cb98f as u64, hash40("top"), 0, 11, 16, 0, 0, 0, 1, true)
        }
        wait(4)
        if(is_excute){
            LANDING_EFFECT(hash40("sys_h_smoke_b"), hash40("top"), 0, 0, 3, 0, 0, 0, 0.699999988, 0, 0, 0, 0, 0, 0, true)
            LAST_EFFECT_SET_RATE(1.29999995)
        }
        wait(4)
        if(is_excute){
            EFFECT_DETACH_KIND(0x09d58cb98f as u64, -1)
        }
        frame(18)
        if(is_excute){
            EFFECT_FOLLOW(0x1132283c24 as u64, hash40("top"), 0, 0, 20, 0, 0, 0, 1, true)
            LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 0, 20, 0, 0, 0, 1.29999995, 0, 0, 0, 0, 0, 0, true)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PALUTENA, 
    animation = "attack_dash",
    animcmd = "sound_attackdash")]
pub fn palutena_sound_attack_dash(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(5)
        if(is_excute){
            PLAY_SEQUENCE(0x171fbaf133 as u64)
            PLAY_SE(0x166aaa60d4 as u64)
        }
        frame(19)
        if(is_excute){
            PLAY_SE(0x15b5e1ed7b as u64)
        }
        frame(55)
        if(is_excute){
            PLAY_STEP(0x180f3a4457 as u64)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PALUTENA, 
    animation = "attack_dash",
    animcmd = "game_attackdash")]
pub fn palutena_game_attack_dash(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=5)
        if(is_excute){
            HIT_NODE(hash40("bust"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("hip"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("head"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("shoulderr"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("shoulderl"), HIT_STATUS_INVINCIBLE)
            HIT_NODE(hash40("armr"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("arml"), HIT_STATUS_INVINCIBLE)
            HIT_NODE(hash40("legr"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("legl"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("kneer"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("kneel"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("virtualshield"), HIT_STATUS_INVINCIBLE)
            HIT_NODE(hash40("bust"), HIT_STATUS_INVINCIBLE)
            HIT_NODE(hash40("head"), HIT_STATUS_INVINCIBLE)
        }
        frame(Frame=6)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=65, KBG=100, FKB=28, BKB=55, Size=4.5, X=0.0, Y=9.0, Z=13.0, X2=0.0, Y2=11.0, Z2=13.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=11.0, Angle=65, KBG=100, FKB=28, BKB=55, Size=3.5, X=0.0, Y=10.0, Z=5.0, X2=0.0, Y2=10.0, Z2=9.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        }
        frame(Frame=7)
        if(is_excute){
            HIT_NODE(hash40("bust"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("head"), HIT_STATUS_NORMAL)
        }
        frame(Frame=10)
        if(is_excute){
            AttackModule::clear(ID=1, false)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=60, KBG=56, FKB=0, BKB=61, Size=2.5, X=0.0, Y=9.0, Z=10.0, X2=0.0, Y2=12.0, Z2=10.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        }
        frame(Frame=17)
        if(is_excute){
            HIT_NODE(hash40("bust"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("hip"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("head"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("shoulderr"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("shoulderl"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("armr"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("arml"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("legr"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("legl"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("kneer"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("kneel"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("virtualshield"), HIT_STATUS_OFF)
        }
        frame(Frame=18)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=20)
        if(is_excute){
            ATTACK(ID=0, Part=1, Bone=hash40("top"), Damage=16.0, Angle=88, KBG=84, FKB=0, BKB=53, Size=4.5, X=0.0, Y=21.0, Z=18.6, X2=0.0, Y2=4.0, Z2=18.6, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=1, Part=1, Bone=hash40("top"), Damage=12.0, Angle=88, KBG=89, FKB=0, BKB=58, Size=2.8, X=0.0, Y=44.0, Z=18.6, X2=0.0, Y2=4.0, Z2=18.6, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=2, Part=1, Bone=hash40("top"), Damage=9.0, Angle=88, KBG=92, FKB=0, BKB=62, Size=2.8, X=0.0, Y=65.0, Z=18.6, X2=0.0, Y2=4.0, Z2=18.6, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
        }
        frame(Frame=26)
        if(is_excute){
            ATTACK(ID=0, Part=1, Bone=hash40("top"), Damage=13.0, Angle=88, KBG=84, FKB=0, BKB=53, Size=2.8, X=0.0, Y=22.0, Z=18.6, X2=0.0, Y2=2.0, Z2=18.6, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=1, Part=1, Bone=hash40("top"), Damage=9.0, Angle=88, KBG=89, FKB=0, BKB=58, Size=2.8, X=0.0, Y=44.0, Z=18.6, X2=0.0, Y2=4.0, Z2=18.6, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=2, Part=1, Bone=hash40("top"), Damage=7.0, Angle=88, KBG=92, FKB=0, BKB=62, Size=2.8, X=0.0, Y=65.0, Z=18.6, X2=0.0, Y2=4.0, Z2=18.6, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PALUTENA, 
    animation = "attack_air_f",
    animcmd = "effect_attackairf")]
pub fn palutena_effect_attack_air_f(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(7)
        if(is_excute){
            EFFECT_FOLLOW(0x1132283c24 as u64, hash40("top"), 0, 7.5, 10, 90, 0, 0, 1, true)
            LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 7.5, 10, 90, 0, 0, 1.29999995, 0, 0, 0, 0, 0, 0, true)
        }
        frame(9)
        if(is_excute){
            rust{
                if PostureModule::lr(module_accessor)<0.0{
                    acmd!({
                        EFFECT_FOLLOW_NO_STOP(0x09d58cb98f as u64, hash40("top"), -4, 7.5, 17, 0, 0, 0, 1, true)
                        LAST_EFFECT_SET_RATE(1.29999995)
                        EFFECT_FOLLOW(0x1441eaf0b3 as u64, hash40("top"), 0, 8, 4, 0, 0, 0, 0.899999976, true)
                    });
                }else{
                    acmd!({
                        EFFECT_FOLLOW_NO_STOP(0x09d58cb98f as u64, hash40("top"), 4, 7.5, 17, 0, 0, 0, 1, true)
                        LAST_EFFECT_SET_RATE(1.29999995)
                        EFFECT_FOLLOW(0x1441eaf0b3 as u64, hash40("top"), 0, 8, 4, 0, 0, 0, 0.899999976, true)
                    });
                }
            }
        }
        frame(13)
        if(is_excute){
            EFFECT_OFF_KIND(0x1132283c24 as u64, false, false)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PALUTENA, 
    animation = "attack_air_f",
    animcmd = "sound_attack_air_f")]
pub fn palutena_sound_attack_air_f(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(8)
        if(is_excute){
            PLAY_SEQUENCE(0x171fbaf133 as u64)
            PLAY_SE(0x190a0aa0fc as u64)
	        PLAY_SE(0x15b5e1ed7b as u64)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PALUTENA, 
    animation = "attack_air_f",
    animcmd = "game_attack_air_f")]
pub fn palutena_game_attack_air_f(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=4)
        if(is_excute){
            WorkModule::on_flag(*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=9)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=45, KBG=92, FKB=0, BKB=31, Size=5.5, X=0.0, Y=6.9, Z=9.0, X2=0.0, Y2=6.9, Z2=15.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_KICK)
            ATTACK(ID=1, Part=1, Bone=hash40("top"), Damage=16.0, Angle=45, KBG=84, FKB=0, BKB=53, Size=4.5, X=0.0, Y=7.5, Z=10.0, X2=0.0, Y2=7.5, Z2=31.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=2, Part=1, Bone=hash40("top"), Damage=12.0, Angle=45, KBG=89, FKB=0, BKB=58, Size=2.8, X=0.0, Y=7.5, Z=10.0, X2=0.0, Y2=7.5, Z2=54.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=3, Part=1, Bone=hash40("top"), Damage=9.0, Angle=45, KBG=92, FKB=0, BKB=62, Size=2.8, X=0.0, Y=7.5, Z=10.0, X2=0.0, Y2=7.5, Z2=78.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
        }
        frame(Frame=12)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=24)
        if(is_excute){
         WorkModule::off_flag(*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PALUTENA, 
    animation = "attack_air_b",
    animcmd = "effect_attackairb")]
pub fn palutena_effect_attack_air_b(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(6)
        if(is_excute){
            EFFECT_FOLLOW(0x1132283c24 as u64, hash40("top"), 0, 14.0, -10, -90, 0, 0, 1, true)
            LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 14.0, -10, -90, 0, 0, 1.29999995, 0, 0, 0, 0, 0, 0, true)
        }
        frame(7)
        if(is_excute){
            EFFECT_FOLLOW(0x15aff06357 as u64, hash40("shield"), 0, 0, 0, 0, 0, 0, 1, true)
            EFFECT_FOLLOW_NO_STOP(0x0edf825c43 as u64, hash40("top"), -4, 11, -14, 0, 0, 0, 1, true)
        }
        frame(10)
        if(is_excute){
            EFFECT_DETACH_KIND(0x09d58cb98f as u64, -1)
        }
        frame(12)
        if(is_excute){
            EFFECT_OFF_KIND(0x1132283c24 as u64, false, false)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PALUTENA, 
    animation = "attack_air_b",
    animcmd = "sound_attack_air_b")]
pub fn palutena_sound_attack_air_b(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(6)
        if(is_excute){
            PLAY_SEQUENCE(0x171fbaf133 as u64)
            PLAY_SE(0x190d030820 as u64)
        }
        frame(7)
        if(is_excute){
            PLAY_SE(0x15b5e1ed7b as u64)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PALUTENA, 
    animation = "attack_air_b",
    animcmd = "game_attack_air_b")]
pub fn palutena_game_attack_air_b(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=3)
        if(is_excute){
            WorkModule::on_flag(*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=7)
        if(is_excute){
            HIT_NODE(hash40("bust"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("hip"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("head"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("shoulderr"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("shoulderl"), HIT_STATUS_INVINCIBLE)
            HIT_NODE(hash40("armr"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("arml"), HIT_STATUS_INVINCIBLE)
            HIT_NODE(hash40("legr"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("legl"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("kneer"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("kneel"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("virtualshield"), HIT_STATUS_INVINCIBLE)
            HIT_NODE(hash40("bust"), HIT_STATUS_INVINCIBLE)
            HIT_NODE(hash40("head"), HIT_STATUS_INVINCIBLE)
        }
        frame(Frame=8)
        if(is_excute){
            HIT_NODE(hash40("bust"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("head"), HIT_STATUS_NORMAL)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=361, KBG=95, FKB=0, BKB=30, Size=7.0, X=0.0, Y=10.2, Z=-14.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=1, Bone=hash40("top"), Damage=16.0, Angle=45, KBG=84, FKB=0, BKB=53, Size=4.5, X=0.0, Y=14.0, Z=-10.0, X2=0.0, Y2=14.0, Z2=-31.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=2, Part=1, Bone=hash40("top"), Damage=12.0, Angle=45, KBG=89, FKB=0, BKB=58, Size=2.8, X=0.0, Y=14.0, Z=-10.0, X2=0.0, Y2=14.0, Z2=-54.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=3, Part=1, Bone=hash40("top"), Damage=9.0, Angle=45, KBG=92, FKB=0, BKB=62, Size=2.8, X=0.0, Y=14.0, Z=-10.0, X2=0.0, Y2=14.0, Z2=-78.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
        }
        frame(Frame=11)
        if(is_excute){
            HIT_NODE(hash40("bust"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("hip"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("head"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("shoulderr"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("shoulderl"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("armr"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("arml"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("legr"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("legl"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("kneer"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("kneel"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("virtualshield"), HIT_STATUS_OFF)
        }
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=35)
        if(is_excute){
            WorkModule::off_flag(*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PALUTENA, 
    animation = "attack_air_hi",
    animcmd = "effect_attack_air_hi")]
pub fn palutena_effect_attack_air_hi(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(7)
        if(is_excute){
            EFFECT_FOLLOW(0x13001d5af8 as u64, hash40("top"), 0, 23, 0, 0, -90, 0, 1, true)
        }
        frame(24)
        if(is_excute){
            EFFECT_FOLLOW(0x1132283c24 as u64, hash40("top"), 0, 28, 0, 0, 0, 0, 1, true)
            LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 28, 0, 0, 0, 0, 1.29999995, 0, 0, 0, 0, 0, 0, true)
        }
        frame(32)
        if(is_excute){
            EFFECT_OFF_KIND(0x1132283c24 as u64, false, false)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PALUTENA, 
    animation = "attack_air_hi",
    animcmd = "sound_attack_air_hi")]
pub fn palutena_sound_attack_air_hi(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(2)
        if(is_excute){
            PLAY_SEQUENCE(0x171fbaf133 as u64)
        }
        wait(6)
        if(is_excute){
            PLAY_SE(0x1900948df6 as u64)
        }
        frame(25)
        if(is_excute){
            PLAY_SE(0x15b5e1ed7b as u64)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PALUTENA, 
    animation = "attack_air_hi",
    animcmd = "game_attackairhi")]
pub fn palutena_game_attack_air_hi(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=3)
        if(is_excute){
            WorkModule::on_flag(*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=10)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=105, KBG=100, FKB=40, BKB=0, Size=4.0, X=0.0, Y=20.0, Z=6.0, X2=0.0, Y2=20.0, Z2=-6.0, Hitlag=0.6, SDI=0.7, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.0, Angle=367, KBG=100, FKB=20, BKB=0, Size=4.0, X=0.0, Y=22.799999, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=0.7, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=1.0, Angle=367, KBG=100, FKB=20, BKB=0, Size=3.0, X=0.0, Y=18.0, Z=4.0, X2=0.0, Y2=22.0, Z2=9.0, Hitlag=0.6, SDI=0.7, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=1.0, Angle=367, KBG=100, FKB=20, BKB=0, Size=3.0, X=0.0, Y=18.0, Z=-4.0, X2=0.0, Y2=22.0, Z2=-9.0, Hitlag=0.6, SDI=0.7, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
        }
        frame(Frame=23)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=24)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=88, KBG=152, FKB=0, BKB=50, Size=6.0, X=0.0, Y=23.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=0.7, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=5.0, Angle=88, KBG=152, FKB=0, BKB=50, Size=4.0, X=0.0, Y=18.0, Z=4.0, X2=0.0, Y2=23.0, Z2=12.0, Hitlag=1.5, SDI=0.7, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=5.0, Angle=88, KBG=152, FKB=0, BKB=50, Size=4.0, X=0.0, Y=18.0, Z=-4.0, X2=0.0, Y2=23.0, Z2=-12.0, Hitlag=1.5, SDI=0.7, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
        }
        frame(Frame=25)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=26)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=90, KBG=84, FKB=0, BKB=53, Size=4.5, X=0.0, Y=28.0, Z=0.0, X2=0.0, Y2=49.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=12.0, Angle=90, KBG=89, FKB=0, BKB=58, Size=2.8, X=0.0, Y=28.0, Z=0.0, X2=0.0, Y2=72.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=9.0, Angle=90, KBG=92, FKB=0, BKB=62, Size=2.8, X=0.0, Y=28.0, Z=0.0, X2=0.0, Y2=96.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
        }
        frame(Frame=29)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=42)
        if(is_excute){
            WorkModule::off_flag(*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PALUTENA, 
    animation = "attack_air_lw",
    animcmd = "effect_attack_air_lw")]
pub fn palutena_effect_attack_air_lw(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(2)
        if(is_excute){
            EFFECT_FOLLOW_NO_STOP(hash40("sys_smash_flash"), hash40("footl"), 4.5, -1.5, -1.5, 0, 0, 0, 1, true)
        }
        frame(4)
        if(is_excute){
            EFFECT_DETACH_KIND(hash40("sys_smash_flash"), -1)
        }
        frame(8)
        if(is_excute){
            EFFECT_FOLLOW(0x132d1c78f7 as u64, hash40("top"), -2, 6, 0, -60, 40, -120, 0.949999988, true)
            EFFECT_FOLLOW(0x1132283c24 as u64, hash40("top"), 0, 0, 0, 180, 0, 0, 1, true)
            LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 0, 180, 0, 0, 0, 1.29999995, 0, 0, 0, 0, 0, 0, true)
        }
        frame(9)
        if(is_excute){
            EFFECT_FOLLOW(0x1036ddbe9b as u64, hash40("footl"), 1, -1, 0, 0, 0, 0, 1, true)
        }
        frame(10)
        if(is_excute){
            EFFECT(0x09d58cb98f as u64, hash40("top"), 0.5, -6, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
        }
        frame(14)
        if(is_excute){
            EFFECT_OFF_KIND(0x1132283c24 as u64, false, false)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PALUTENA, 
    animation = "attack_air_lw",
    animcmd = "sound_attack_air_lw")]
pub fn palutena_sound_attack_air_lw(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(8)
        if(is_excute){
            PLAY_SEQUENCE(0x171fbaf133 as u64)
            PLAY_SE(0x19079d252a as u64)
        }
        frame(9)
        if(is_excute){
            PLAY_SE(0x15b5e1ed7b as u64)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PALUTENA, 
    animation = "attack_air_lw",
    animcmd = "game_attackairlw")]
pub fn palutena_game_attack_air_lw(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=3)
        if(is_excute){
            WorkModule::on_flag(*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=8)
        if(is_excute){
            JostleModule::set_status(false)
        }
        frame(Frame=10)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=270, KBG=100, FKB=0, BKB=20, Size=4.2, X=0.0, Y=-3.7, Z=0.5, X2=0.0, Y2=-5.0, Z2=0.5, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_KICK)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=11.0, Angle=55, KBG=100, FKB=0, BKB=30, Size=6.3, X=0.0, Y=-5.0, Z=1.5, X2=0.0, Y2=-5.0, Z2=-0.5, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_KICK)
            ATTACK(ID=2, Part=1, Bone=hash40("top"), Damage=16.0, Angle=270, KBG=84, FKB=0, BKB=53, Size=4.5, X=0.0, Y=-21.0, Z=0.0, X2=0.0, Y2=-4.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=3, Part=1, Bone=hash40("top"), Damage=12.0, Angle=270, KBG=89, FKB=0, BKB=58, Size=2.8, X=0.0, Y=-44.0, Z=0.0, X2=0.0, Y2=-4.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=4, Part=1, Bone=hash40("top"), Damage=9.0, Angle=270, KBG=92, FKB=0, BKB=62, Size=2.8, X=0.0, Y=-68.0, Z=0.0, X2=0.0, Y2=-4.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
        }
        frame(Frame=13)
        if(is_excute){
            AttackModule::clear_all()
            JostleModule::set_status(true)
        }
        frame(Frame=48)
        if(is_excute){
            WorkModule::off_flag(*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PALUTENA, 
    animation = "attack_s4_s",
    animcmd = "effect_attacks4")]
pub fn palutena_effect_attack_s4(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(5)
        if(is_excute){
            EFFECT(hash40("sys_smash_flash"), hash40("top"), 0, 24, -3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
        }
        frame(15)
        if(is_excute){
            LANDING_EFFECT(0x0d0679b24d as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
            EFFECT_FOLLOW(0x1652e3b8d9 as u64, hash40("top"), 0, 0, -15, 50, 0, 0, 1, true)
        }
        frame(16)
        if(is_excute){
            rust{
                if PostureModule::lr(module_accessor)<0.0{
                    acmd!({
                        EFFECT_FOLLOW(0x1212308814 as u64, hash40("top"), 1, 21, 2.5, 0, -50, 0, 1, true)
                        EFFECT_FOLLOW(0x1132283c24 as u64, hash40("top"), 0, 13, 10, 90, 0, 0, 1, true)
                        LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 13, 10, 90, 0, 0, 1.29999995, 0, 0, 0, 0, 0, 0, true)
                    });
                }else{
                    acmd!({
                        EFFECT_FOLLOW(0x1212308814 as u64, hash40("top"), 1, 21, 2.5, 0, -55, 0, 1, true)
                        EFFECT_FOLLOW(0x1132283c24 as u64, hash40("top"), 0, 13, 10, 90, 0, 0, 1, true)
                        LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 13, 10, 90, 0, 0, 1.29999995, 0, 0, 0, 0, 0, 0, true)
                    });
                }
            }
        }
        frame(17)
        if(is_excute){
            EFFECT(0x10d701ec01 as u64, hash40("top"), 0, 6, -3, 0, 0, 0, 0.600000024, 0, 0, 0, 0, 0, 0, true)
        }
        frame(19)
        if(is_excute){
            EFFECT(0x10d701ec01 as u64, hash40("top"), 0, 6, -6, 0, 0, 0, 0.600000024, 0, 0, 0, 0, 0, 0, true)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PALUTENA, 
    animation = "attack_s4_s",
    animcmd = "sound_attacks4")]
pub fn palutena_sound_attack_s4(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(14)
        if(is_excute){
            STOP_SE(0x18f7179e75 as u64)
            PLAY_SEQUENCE(0x18953651a8 as u64)
            PLAY_SE(0x15a592a1ea as u64)
        }
        frame(17)
        if(is_excute){
            PLAY_SE(0x15b5e1ed7b as u64)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PALUTENA, 
    animation = "attack_s4_s",
    animcmd = "game_attacks4")]
pub fn palutena_game_attack_s4(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            ArticleModule::generate_article(FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, false, 0)
            ArticleModule::change_motion(FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, smash::phx::Hash40::new("attack_s4_charge"), false, 0.0)
        }
        frame(Frame=11)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        frame(Frame=12)
        if(is_excute){
            ArticleModule::change_motion(FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, smash::phx::Hash40::new("attack_s4s"), false, 0.0)
        }
        frame(Frame=18)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=361, KBG=100, FKB=0, BKB=40, Size=5.1, X=0.0, Y=11.0, Z=6.0, X2=0.0, Y2=9.9, Z2=11.9, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=13.0, Angle=361, KBG=102, FKB=0, BKB=40, Size=5.1, X=0.0, Y=11.0, Z=6.0, X2=0.0, Y2=8.5, Z2=18.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
            ATTACK(ID=2, Part=1, Bone=hash40("top"), Damage=16.0, Angle=45, KBG=84, FKB=0, BKB=53, Size=4.5, X=0.0, Y=14.0, Z=10.0, X2=0.0, Y2=14.0, Z2=31.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=3, Part=1, Bone=hash40("top"), Damage=12.0, Angle=45, KBG=89, FKB=0, BKB=58, Size=2.8, X=0.0, Y=14.0, Z=10.0, X2=0.0, Y2=14.0, Z2=54.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=4, Part=1, Bone=hash40("top"), Damage=9.0, Angle=45, KBG=92, FKB=0, BKB=62, Size=2.8, X=0.0, Y=14.0, Z=10.0, X2=0.0, Y2=14.0, Z2=78.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
        }
        frame(Frame=21)
        if(is_excute){
            AttackModule::clear(ID=1, false)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.0, Angle=361, KBG=100, FKB=60, BKB=0, Size=12.0, X=0.0, Y=10.0, Z=8.0, X2=0.0, Y2=10.0, Z2=25.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=true, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
        }
        frame(Frame=26)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.0, Angle=361, KBG=100, FKB=35, BKB=0, Size=12.0, X=0.0, Y=10.0, Z=8.0, X2=0.0, Y2=10.0, Z2=40.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=true, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
        }
        frame(30)
        if(is_excute){
            ATTACK(ID=2, Part=1, Bone=hash40("top"), Damage=13.0, Angle=45, KBG=84, FKB=0, BKB=53, Size=2.8, X=0.0, Y=14.0, Z=10.0, X2=0.0, Y2=14.0, Z2=31.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=3, Part=1, Bone=hash40("top"), Damage=9.0, Angle=45, KBG=89, FKB=0, BKB=58, Size=2.8, X=0.0, Y=14.0, Z=10.0, X2=0.0, Y2=14.0, Z2=54.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=4, Part=1, Bone=hash40("top"), Damage=7.0, Angle=45, KBG=92, FKB=0, BKB=62, Size=2.8, X=0.0, Y=14.0, Z=10.0, X2=0.0, Y2=14.0, Z2=78.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
        }
        frame(Frame=32)
        if(is_excute){
            AttackModule::clear(ID=0, false)
            AttackModule::clear(ID=1, false)
        }
        frame(Frame=36)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=80)
        if(is_excute){
            //ArticleModule::remove_exist(FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, ARTICLE_OPE_TARGET_ALL)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PALUTENA, 
    animation = "attack_lw4",
    animcmd = "effect_attacklw4")]
pub fn palutena_effect_attack_lw4(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(4)
        if(is_excute){
            EFFECT(hash40("sys_smash_flash"), hash40("top"), 0, 24, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
        }
        frame(15)
        if(is_excute){
            EFFECT_FOLLOW(0x17bc172c22 as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, true)
        }
        frame(16)
        if(is_excute){
            EFFECT_FOLLOW(0x1212308814 as u64, hash40("top"), 3, 13.5, 0, 0, -90, 0, 1, true)
            EFFECT_FOLLOW(0x1132283c24 as u64, hash40("top"), 0, 7, 10, 90, 0, 0, 1, true)
            LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 7, 10, 90, 0, 0, 1.29999995, 0, 0, 0, 0, 0, 0, true)
            EFFECT_FOLLOW(0x1132283c24 as u64, hash40("top"), 0, 7, -10, -90, 0, 0, 1, true)
            LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 7, -10, -90, 0, 0, 1.29999995, 0, 0, 0, 0, 0, 0, true)
        }
        frame(19)
        if(is_excute){
            EFFECT(0x10d701ec01 as u64, hash40("top"), 0, 3, 8, 0, 180, 0, 0.600000024, 0, 0, 0, 0, 0, 0, true)
        }
        frame(20)
        if(is_excute){
            EFFECT(0x10d701ec01 as u64, hash40("top"), 0, 3, -8, 0, 0, 0, 0.600000024, 0, 0, 0, 0, 0, 0, true)
            LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 1.29999995, 0, 0, 0, 0, 0, 0, false)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PALUTENA, 
    animation = "attack_lw4",
    animcmd = "sound_attacklw4")]
pub fn palutena_sound_attack_lw4(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(16)
        if(is_excute){
            STOP_SE(0x18f7179e75 as u64)
            PLAY_SEQUENCE(0x18183e5c5d as u64)
            PLAY_SE(0x15b2e845a7 as u64)
        }
        frame(17)
        if(is_excute){
            PLAY_SE(0x15b5e1ed7b as u64)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PALUTENA, 
    animation = "attack_lw4",
    animcmd = "game_attacklw4")]
pub fn palutena_game_attack_lw4(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            ArticleModule::generate_article(FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, false, 0)
            ArticleModule::change_motion(FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, smash::phx::Hash40::new("attack_lw4_charge"), false, 0.0)
        }
        frame(Frame=14)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        frame(Frame=15)
        if(is_excute){
            ArticleModule::change_motion(FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, smash::phx::Hash40::new("attack_lw4"), false, 0.0)
        }
        frame(Frame=17)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.0, Angle=60, KBG=103, FKB=0, BKB=52, Size=3.7, X=0.0, Y=9.2, Z=7.3, X2=0.0, Y2=9.2, Z2=-7.3, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
        }
        frame(Frame=18)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.0, Angle=28, KBG=90, FKB=0, BKB=30, Size=6.0, X=0.0, Y=4.0, Z=14.0, X2=0.0, Y2=4.0, Z2=-14.0, Hitlag=0.85, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
            ATTACK(ID=1, Part=1, Bone=hash40("top"), Damage=16.0, Angle=45, KBG=84, FKB=0, BKB=53, Size=4.5, X=0.0, Y=7.0, Z=10.0, X2=0.0, Y2=7.0, Z2=31.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=2, Part=1, Bone=hash40("top"), Damage=12.0, Angle=45, KBG=89, FKB=0, BKB=58, Size=2.8, X=0.0, Y=7.0, Z=10.0, X2=0.0, Y2=7.0, Z2=54.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=3, Part=1, Bone=hash40("top"), Damage=9.0, Angle=45, KBG=92, FKB=0, BKB=62, Size=2.8, X=0.0, Y=7.0, Z=10.0, X2=0.0, Y2=7.0, Z2=78.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=4, Part=1, Bone=hash40("top"), Damage=16.0, Angle=45, KBG=84, FKB=0, BKB=53, Size=4.5, X=0.0, Y=7.0, Z=-10.0, X2=0.0, Y2=7.0, Z2=-31.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=5, Part=1, Bone=hash40("top"), Damage=12.0, Angle=45, KBG=89, FKB=0, BKB=58, Size=2.8, X=0.0, Y=7.0, Z=-10.0, X2=0.0, Y2=7.0, Z2=-54.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=6, Part=1, Bone=hash40("top"), Damage=9.0, Angle=45, KBG=92, FKB=0, BKB=62, Size=2.8, X=0.0, Y=7.0, Z=-10.0, X2=0.0, Y2=7.0, Z2=-78.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
        }
        frame(Frame=20)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.0, Angle=361, KBG=100, FKB=55, BKB=0, Size=9.0, X=0.0, Y=4.0, Z=24.0, X2=0.0, Y2=4.0, Z2=-24.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=true, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_BODY)
        }
        frame(Frame=30)
        if(is_excute){
            AttackModule::clear(ID=0, false)
            ATTACK(ID=1, Part=1, Bone=hash40("top"), Damage=13.0, Angle=45, KBG=84, FKB=0, BKB=53, Size=2.8, X=0.0, Y=7.0, Z=10.0, X2=0.0, Y2=7.0, Z2=31.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=2, Part=1, Bone=hash40("top"), Damage=9.0, Angle=45, KBG=89, FKB=0, BKB=58, Size=2.8, X=0.0, Y=7.0, Z=10.0, X2=0.0, Y2=7.0, Z2=54.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=3, Part=1, Bone=hash40("top"), Damage=7.0, Angle=45, KBG=92, FKB=0, BKB=62, Size=2.8, X=0.0, Y=7.0, Z=10.0, X2=0.0, Y2=7.0, Z2=78.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=4, Part=1, Bone=hash40("top"), Damage=13.0, Angle=45, KBG=84, FKB=0, BKB=53, Size=2.8, X=0.0, Y=7.0, Z=-10.0, X2=0.0, Y2=7.0, Z2=-31.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=5, Part=1, Bone=hash40("top"), Damage=9.0, Angle=45, KBG=89, FKB=0, BKB=58, Size=2.8, X=0.0, Y=7.0, Z=-10.0, X2=0.0, Y2=7.0, Z2=-54.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=6, Part=1, Bone=hash40("top"), Damage=7.0, Angle=45, KBG=92, FKB=0, BKB=62, Size=2.8, X=0.0, Y=7.0, Z=-10.0, X2=0.0, Y2=7.0, Z2=-78.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
        }
        frame(Frame=36)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=80)
        if(is_excute){
            //ArticleModule::remove_exist(FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PALUTENA, 
    animation = "attack_hi4",
    animcmd = "effect_attackhi4")]
pub fn palutena_effect_attack_hi4(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(6)
        if(is_excute){
            EFFECT(hash40("sys_smash_flash"), hash40("stick"), 0, 8.5, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
        }
        frame(14)
        if(is_excute){
            FOOT_EFFECT(0x0d0679b24d as u64, hash40("top"), 0, 0, -3, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true)
            EFFECT_FOLLOW(0x14c37776d7 as u64, hash40("stick"), 0, 8.64999962, 0, 0, 0, 0, 1, true)
        }
        frame(16)
        if(is_excute){
            rust{
                if PostureModule::lr(module_accessor)<0.0{
                    acmd!({
                        EFFECT_FOLLOW(0x1132283c24 as u64, hash40("top"), 0, 0, 10, 0, 0, 0, 1, true)
                        LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 0, 10, 0, 0, 0, 1.29999995, 0, 0, 0, 0, 0, 0, true)
                        EFFECT_FOLLOW_ALPHA(0x1212308814 as u64, hash40("top"), 4, 21.5, 2, 0, -60, 0, 1, true, 0.699999988)
                        LAST_EFFECT_SET_RATE(1.10000002)
                        EFFECT_FOLLOW(0x1132283c24 as u64, hash40("top"), 0, 0, -10, 0, 0, 0, 1, true)
                        LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 0, -10, 0, 0, 0, 1.29999995, 0, 0, 0, 0, 0, 0, true)
                        EFFECT_FOLLOW(0x1132283c24 as u64, hash40("top"), 0, 28, 0, 0, 0, 0, 1, true)
                        LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 28, 0, 0, 0, 0, 1.29999995, 0, 0, 0, 0, 0, 0, true)
                    });
                } else{
                    acmd!({
                        EFFECT_FOLLOW(0x1212308814 as u64, hash40("top"), 4, 21.5, 2.5, 0, -60, 0, 1, true)
                        LAST_EFFECT_SET_RATE(1.10000002)
                        EFFECT_FOLLOW(0x1132283c24 as u64, hash40("top"), 0, 0, 10, 0, 0, 0, 1, true)
                        LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 0, 10, 0, 0, 0, 1.29999995, 0, 0, 0, 0, 0, 0, true)
                        EFFECT_FOLLOW(0x1132283c24 as u64, hash40("top"), 0, 0, -10, 0, 0, 0, 1, true)
                        LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 0, -10, 0, 0, 0, 1.29999995, 0, 0, 0, 0, 0, 0, true)
                        EFFECT_FOLLOW(0x1132283c24 as u64, hash40("top"), 0, 28, 0, 0, 0, 0, 1, true)
                        LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 28, 0, 0, 0, 0, 1.29999995, 0, 0, 0, 0, 0, 0, true)
                    });
                }
            }
        }
        frame(50)
        if(is_excute){
            EFFECT_OFF_KIND(0x14c37776d7 as u64, false, false)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PALUTENA, 
    animation = "attack_hi4",
    animcmd = "game_attackhi4")]
pub fn palutena_game_attack_hi4(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=13)
        if(is_excute){
        WorkModule::on_flag(*FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        frame(Frame=18)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=88, KBG=84, FKB=0, BKB=53, Size=4.5, X=0.0, Y=21.0, Z=9.7, X2=0.0, Y2=4.0, Z2=9.7, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=12.0, Angle=88, KBG=89, FKB=0, BKB=58, Size=2.8, X=0.0, Y=44.0, Z=9.7, X2=0.0, Y2=4.0, Z2=9.7, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=9.0, Angle=88, KBG=92, FKB=0, BKB=62, Size=2.8, X=0.0, Y=65.0, Z=9.7, X2=0.0, Y2=4.0, Z2=9.7, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=16.0, Angle=88, KBG=84, FKB=0, BKB=53, Size=4.5, X=0.0, Y=21.0, Z=-9.7, X2=0.0, Y2=4.0, Z2=-9.7, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=12.0, Angle=88, KBG=89, FKB=0, BKB=58, Size=2.8, X=0.0, Y=44.0, Z=-9.7, X2=0.0, Y2=4.0, Z2=-9.7, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=9.0, Angle=88, KBG=92, FKB=0, BKB=62, Size=2.8, X=0.0, Y=65.0, Z=-9.7, X2=0.0, Y2=4.0, Z2=-9.7, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=6, Part=0, Bone=hash40("top"), Damage=16.0, Angle=90, KBG=84, FKB=0, BKB=53, Size=4.5, X=0.0, Y=28.0, Z=0.0, X2=0.0, Y2=49.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=7, Part=0, Bone=hash40("top"), Damage=12.0, Angle=90, KBG=89, FKB=0, BKB=58, Size=2.8, X=0.0, Y=28.0, Z=0.0, X2=0.0, Y2=72.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=8, Part=0, Bone=hash40("top"), Damage=9.0, Angle=90, KBG=92, FKB=0, BKB=62, Size=2.8, X=0.0, Y=28.0, Z=0.0, X2=0.0, Y2=96.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
        }
        frame(Frame=30)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.0, Angle=88, KBG=84, FKB=0, BKB=53, Size=2.8, X=0.0, Y=22.0, Z=9.7, X2=0.0, Y2=2.0, Z2=9.7, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=9.0, Angle=88, KBG=89, FKB=0, BKB=58, Size=2.8, X=0.0, Y=44.0, Z=9.7, X2=0.0, Y2=4.0, Z2=9.7, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=7.0, Angle=88, KBG=92, FKB=0, BKB=62, Size=2.8, X=0.0, Y=65.0, Z=9.7, X2=0.0, Y2=4.0, Z2=9.7, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=13.0, Angle=88, KBG=84, FKB=0, BKB=53, Size=2.8, X=0.0, Y=22.0, Z=-9.7, X2=0.0, Y2=2.0, Z2=-9.7, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=9.0, Angle=88, KBG=89, FKB=0, BKB=58, Size=2.8, X=0.0, Y=44.0, Z=-9.7, X2=0.0, Y2=4.0, Z2=-9.7, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=7.0, Angle=88, KBG=92, FKB=0, BKB=62, Size=2.8, X=0.0, Y=65.0, Z=-9.7, X2=0.0, Y2=4.0, Z2=-9.7, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=6, Part=0, Bone=hash40("top"), Damage=13.0, Angle=88, KBG=84, FKB=0, BKB=53, Size=2.8, X=0.0, Y=28.0, Z=0.0, X2=0.0, Y2=49.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=7, Part=0, Bone=hash40("top"), Damage=9.0, Angle=88, KBG=89, FKB=0, BKB=58, Size=2.8, X=0.0, Y=28.0, Z=0.0, X2=0.0, Y2=72.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=8, Part=0, Bone=hash40("top"), Damage=7.0, Angle=88, KBG=92, FKB=0, BKB=62, Size=2.8, X=0.0, Y=28.0, Z=0.0, X2=0.0, Y2=96.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
        }
        frame(Frame=36)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PALUTENA, 
    animation = "special_n",
    animcmd = "effect_specialn")]
pub fn palutena_effect_special_n(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            rust{
                _ZPOS1 = app::sv_math::randf(hash40("fighter"), 45.0);
                _ZPOS2 = app::sv_math::randf(hash40("fighter"), 45.0);
                _ZPOS3 = app::sv_math::randf(hash40("fighter"), 45.0);
                _ZPOS4 = app::sv_math::randf(hash40("fighter"), 45.0);
                _ZPOS5 = app::sv_math::randf(hash40("fighter"), 45.0);
            }
            LANDING_EFFECT(0x1255c42543 as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 0.800000012, 0, 0, 0, 0, 0, 0, false)
        }
        frame(7)
        if(is_excute){
            WorkModule::on_flag(*FIGHTER_PALUTENA_STATUS_SPECIAL_N_FLAG_SIGHT_EFFECT_ON)
        }
        frame(8)
        if(is_excute){
            rust{
                if WorkModule::is_flag(module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_N_FLAG_TARGET_EXIST)==(true){
                    acmd!({
                        EFFECT_FOLLOW(0x1212308814 as u64, hash40("top"), -0.200000003, 22, -1, 10, 90, 0, 1, true)
                        EFFECT_FOLLOW(0x1941d96f74 as u64, hash40("stick"), 0, 8.64999962, 0, 0, 0, 0, 1, true)
                        EffectModule::enable_sync_init_pos_last()
                        EFFECT_FOLLOW(0x0f8d9fc7fb as u64, hash40("stick"), 0, 0, 0, 0, 0, 0, 1, true)
                    });
                }else{
                    acmd!({
                        EFFECT_FOLLOW(0x1941d96f74 as u64, hash40("stick"), 0, 8.64999962, 0, 0, 0, 0, 1, true)
                        EffectModule::enable_sync_init_pos_last()
                        EFFECT_FOLLOW(0x14c37776d7 as u64, hash40("stick"), 0, 8.64999962, 0, 0, 0, 0, 1, true)
                        frame(13)
                        EFFECT_FOLLOW(0x1212308814 as u64, hash40("top"), -0.200000003, 22, -1, 10, 90, 0, 1, true)
                    });
                }
            }
        }
        frame(10)
        if(is_excute){
            EFFECT_FOLLOW(0x1132283c24 as u64, hash40("top"), 0, 0, _ZPOS1 + 5.0, 0, 0, 0, 1, true)
            LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 0, _ZPOS1 + 5.0, 0, 0, 0, 1.29999995, 0, 0, 0, 0, 0, 0, true)
        }
        frame(18)
        if(is_excute){
            EFFECT_OFF_KIND(0x1132283c24 as u64, false, false)
        }
        frame(20)
        if(is_excute){
            EFFECT_FOLLOW(0x1132283c24 as u64, hash40("top"), 0, 0, _ZPOS2 + 5.0, 0, 0, 0, 1, true)
            LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 0, _ZPOS2 + 5.0, 0, 0, 0, 1.29999995, 0, 0, 0, 0, 0, 0, true)
        }
        frame(28)
        if(is_excute){
            EFFECT_OFF_KIND(0x1132283c24 as u64, false, false)
        }
        frame(30)
        if(is_excute){
            EFFECT_FOLLOW(0x1132283c24 as u64, hash40("top"), 0, 0, _ZPOS3 + 5.0, 0, 0, 0, 1, true)
            LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 0, _ZPOS3 + 5.0, 0, 0, 0, 1.29999995, 0, 0, 0, 0, 0, 0, true)
        }
        frame(38)
        if(is_excute){
            EFFECT_OFF_KIND(0x1132283c24 as u64, false, false)
        }
        frame(40)
        if(is_excute){
            EFFECT_FOLLOW(0x1132283c24 as u64, hash40("top"), 0, 0, _ZPOS4 + 5.0, 0, 0, 0, 1, true)
            LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 0, _ZPOS4 + 5.0, 0, 0, 0, 1.29999995, 0, 0, 0, 0, 0, 0, true)
        }
        frame(48)
        if(is_excute){
            EFFECT_OFF_KIND(0x1132283c24 as u64, false, false)
        }
        frame(50)
        if(is_excute){
            EFFECT_OFF_KIND(0x1941d96f74 as u64, false, false)
            EFFECT_OFF_KIND(0x14c37776d7 as u64, false, false)
            EFFECT_OFF_KIND(0x0f8d9fc7fb as u64, false, false)
            EFFECT_FOLLOW(0x1132283c24 as u64, hash40("top"), 0, 0, _ZPOS5 + 5.0, 0, 0, 0, 1, true)
            LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 0, _ZPOS5 + 5.0, 0, 0, 0, 1.29999995, 0, 0, 0, 0, 0, 0, true)
        }        
        frame(58)
        if(is_excute){
            EFFECT_OFF_KIND(0x1132283c24 as u64, false, false)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PALUTENA, 
    animation = "special_n",
    animcmd = "sound_specialn")]
pub fn palutena_sound_special_n(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(8)
        rust{
            if ArticleModule::is_exist(module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_AUTORETICLE){
                acmd!({
                    PLAY_SE(0x1a868b9d93 as u64)
                });
            }
        }
        frame(9)
        if(is_excute){
            PLAY_STATUS(0x1747148021 as u64)
        }
        frame(11)
        if(is_excute){
            PLAY_SE(0x15b5e1ed7b as u64)
        }
        frame(21)
        if(is_excute){
            PLAY_SE(0x15b5e1ed7b as u64)
        }
        frame(31)
        if(is_excute){
            PLAY_SE(0x15b5e1ed7b as u64)
        }
        frame(41)
        if(is_excute){
            PLAY_SE(0x15b5e1ed7b as u64)
        }
        frame(41)
        if(is_excute){
            STOP_SE(0x1a868b9d93 as u64)
        }
        frame(51)
        if(is_excute){
            PLAY_SE(0x15b5e1ed7b as u64)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PALUTENA, 
    animation = "special_n",
    animcmd = "game_specialn")]
pub fn palutena_game_special_n(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=7)
        if(is_excute){
            SEARCH(0, 0, hash40("bust"), 120.0, 0.0, 0.0, 0.0, COLLISION_KIND_MASK_HIT, HIT_STATUS_MASK_NORMAL, 1, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_FIEB, COLLISION_PART_MASK_BODY_HEAD, false)
        }
        frame(Frame=12)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=88, KBG=84, FKB=0, BKB=53, Size=4.5, X=0.0, Y=21.0, Z=_ZPOS1 + 5.0, X2=0.0, Y2=4.0, Z2=_ZPOS1 + 5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=12.0, Angle=88, KBG=89, FKB=0, BKB=58, Size=2.8, X=0.0, Y=44.0, Z=_ZPOS1 + 5.0, X2=0.0, Y2=4.0, Z2=_ZPOS1 + 5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=9.0, Angle=88, KBG=92, FKB=0, BKB=62, Size=2.8, X=0.0, Y=65.0, Z=_ZPOS1 + 5.0, X2=0.0, Y2=4.0, Z2=_ZPOS1 + 5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
        }
        frame(Frame=15)
        if(is_excute){
            SearchModule::clear_all()
        }
        frame(20)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=22)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=88, KBG=84, FKB=0, BKB=53, Size=4.5, X=0.0, Y=21.0, Z=_ZPOS2 + 5.0, X2=0.0, Y2=4.0, Z2=_ZPOS2 + 5.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=12.0, Angle=88, KBG=89, FKB=0, BKB=58, Size=2.8, X=0.0, Y=44.0, Z=_ZPOS2 + 5.0, X2=0.0, Y2=4.0, Z2=_ZPOS2 + 5.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=9.0, Angle=88, KBG=92, FKB=0, BKB=62, Size=2.8, X=0.0, Y=65.0, Z=_ZPOS2 + 5.0, X2=0.0, Y2=4.0, Z2=_ZPOS2 + 5.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
        }
        frame(30)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=32)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=88, KBG=84, FKB=0, BKB=53, Size=4.5, X=0.0, Y=21.0, Z=_ZPOS3 + 5.0, X2=0.0, Y2=4.0, Z2=_ZPOS3 + 5.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=12.0, Angle=88, KBG=89, FKB=0, BKB=58, Size=2.8, X=0.0, Y=44.0, Z=_ZPOS3 + 5.0, X2=0.0, Y2=4.0, Z2=_ZPOS3 + 5.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=9.0, Angle=88, KBG=92, FKB=0, BKB=62, Size=2.8, X=0.0, Y=68.0, Z=_ZPOS3 + 5.0, X2=0.0, Y2=4.0, Z2=_ZPOS3 + 5.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
        }
        frame(40)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=42)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=88, KBG=84, FKB=0, BKB=53, Size=4.5, X=0.0, Y=21.0, Z=_ZPOS4 + 5.0, X2=0.0, Y2=4.0, Z2=_ZPOS4 + 5.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=12.0, Angle=88, KBG=89, FKB=0, BKB=58, Size=2.8, X=0.0, Y=44.0, Z=_ZPOS4 + 5.0, X2=0.0, Y2=4.0, Z2=_ZPOS4 + 5.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=9.0, Angle=88, KBG=92, FKB=0, BKB=62, Size=2.8, X=0.0, Y=68.0, Z=_ZPOS4 + 5.0, X2=0.0, Y2=4.0, Z2=_ZPOS4 + 5.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
        }
        frame(50)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=52)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=88, KBG=84, FKB=0, BKB=53, Size=4.5, X=0.0, Y=21.0, Z=_ZPOS5 + 5.0, X2=0.0, Y2=4.0, Z2=_ZPOS5 + 5.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=12.0, Angle=88, KBG=89, FKB=0, BKB=58, Size=2.8, X=0.0, Y=44.0, Z=_ZPOS5 + 5.0, X2=0.0, Y2=4.0, Z2=_ZPOS5 + 5.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=9.0, Angle=88, KBG=92, FKB=0, BKB=62, Size=2.8, X=0.0, Y=68.0, Z=_ZPOS5 + 5.0, X2=0.0, Y2=4.0, Z2=_ZPOS5 + 5.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
        }
        frame(60)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

pub fn install() {
    acmd::add_hooks!(
        palutena_effect_attack_s3,
        palutena_sound_attack_s3,
        palutena_game_attack_s3,
        palutena_effect_attack_hi3,
        palutena_sound_attack_hi3,
        palutena_game_attack_hi3,
        palutena_effect_attack_lw3,
        palutena_sound_attack_lw3,
        palutena_game_attack_lw3,
        palutena_effect_attack_dash,
        palutena_sound_attack_dash,
        palutena_game_attack_dash,
        palutena_effect_attack_air_f,
        palutena_sound_attack_air_f,
        palutena_game_attack_air_f,
        palutena_effect_attack_air_b,
        palutena_sound_attack_air_b,
        palutena_game_attack_air_b,
        palutena_effect_attack_air_hi,
        palutena_sound_attack_air_hi,
        palutena_game_attack_air_hi,
        palutena_effect_attack_air_lw,
        palutena_sound_attack_air_lw,
        palutena_game_attack_air_lw,
        palutena_effect_attack_s4,
        palutena_sound_attack_s4,
        palutena_game_attack_s4,
        palutena_effect_attack_lw4,
        palutena_sound_attack_lw4,
        palutena_game_attack_lw4,
        palutena_effect_attack_hi4,
        palutena_game_attack_hi4,
        palutena_effect_special_n,
        palutena_sound_special_n,
        palutena_game_special_n,
    );
}
