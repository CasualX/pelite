## Interfaces

```
r5apex.exe!0x01044348 ClientRenderTargets001
r5apex.exe!0x01036be8 EngineTraceClient004
r5apex.exe!0x01034ea8 EngineTraceClientDecals004
r5apex.exe!0x01818430 EventSystem001
r5apex.exe!0x01df0e00 GameUI011
r5apex.exe!0x01038f08 ISoundC002
r5apex.exe!0x01def0c8 RunGameEngine005
r5apex.exe!0x0128c618 ShaderSystem002
r5apex.exe!0x0181a340 VClient018
r5apex.exe!0x01abf168 VClientEntityList003
r5apex.exe!0x01defe00 VClientPrediction001
r5apex.exe!0x01038340 VCvarQuery001
r5apex.exe!0x01036d30 VDebugOverlay004
r5apex.exe!0x01039938 VENGINE_GAMEUIFUNCS_VERSION005
r5apex.exe!0x0117b4a0 VENGINE_LAUNCHER_API_VERSION004
r5apex.exe!0x01047550 VEngineModel016
r5apex.exe!0x010388b8 VEngineRandom001
r5apex.exe!0x010386d8 VEngineRenderView013
r5apex.exe!0x01814390 VGUI_System010
r5apex.exe!0x22d12478 VMaterialSystemConfig004
r5apex.exe!0x01036d38 VPhysicsDebugOverlay001
```

## Miscellaneous

```
TimeDateStamp = 0x5de73ec3
CheckSum = 0x1f20056
GameVersion = "v3.0.11.1353"
NUM_ENT_ENTRIES = 0x10000
r5apex.exe!0x183f118 cl_entitylist
r5apex.exe!0x1039934 LocalEntityHandle
r5apex.exe!0x1d9ab98 LocalPlayer
r5apex.exe!0x1110910 GlobalVars
r5apex.exe!0xcad2248 PlayerResources
r5apex.exe!0xcab13f8 ViewRender + 0x1a93d0 ViewMatrix
r5apex.exe!0x1110c10 ClientState
r5apex.exe!0x1110ca8 SignonState
r5apex.exe!0x1110dc8 LevelName
```

## Buttons

These are addresses to global instances of the [`kbutton_t`](https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/game/client/kbutton.h#L14-L20) struct.

```
r5apex.exe!0x0cad2bb0 kbutton_t in_attack
r5apex.exe!0x0cad2cd0 kbutton_t in_backward
r5apex.exe!0x26df0540 kbutton_t in_break
r5apex.exe!0x271e00d0 kbutton_t in_camin
r5apex.exe!0x26df0870 kbutton_t in_camout
r5apex.exe!0x26df0820 kbutton_t in_campitchdown
r5apex.exe!0x271e0158 kbutton_t in_campitchup
r5apex.exe!0x26df08b0 kbutton_t in_camyawleft
r5apex.exe!0x26df0850 kbutton_t in_camyawright
r5apex.exe!0x0cad2b68 kbutton_t in_commandermousemove
r5apex.exe!0x0cad2c40 kbutton_t in_dodge
r5apex.exe!0x26df0560 kbutton_t in_duck
r5apex.exe!0x0cad2ca8 kbutton_t in_forward
r5apex.exe!0x0cad2bc0 kbutton_t in_graph
r5apex.exe!0x0cad2c28 kbutton_t in_jump
r5apex.exe!0x271e0138 kbutton_t in_klook
r5apex.exe!0x26df0898 kbutton_t in_left
r5apex.exe!0x26df0840 kbutton_t in_lookdown
r5apex.exe!0x26df0550 kbutton_t in_lookup
r5apex.exe!0x271e0148 kbutton_t in_melee
r5apex.exe!0x26df08c0 kbutton_t in_movedown
r5apex.exe!0x0cad2c98 kbutton_t in_moveleft
r5apex.exe!0x0cad2cc0 kbutton_t in_moveright
r5apex.exe!0x271e0118 kbutton_t in_moveup
r5apex.exe!0x271e0128 kbutton_t in_offhand0
r5apex.exe!0x26df0888 kbutton_t in_offhand1
r5apex.exe!0x26df0650 kbutton_t in_offhand2
r5apex.exe!0x26df0630 kbutton_t in_offhand3
r5apex.exe!0x26df05f0 kbutton_t in_offhand4
r5apex.exe!0x0cad2c10 kbutton_t in_pause_menu
r5apex.exe!0x0cad2b80 kbutton_t in_ping
r5apex.exe!0x0cad2bd0 kbutton_t in_reload
r5apex.exe!0x26df0860 kbutton_t in_right
r5apex.exe!0x26df0660 kbutton_t in_score
r5apex.exe!0x26df0660 kbutton_t in_showscores
r5apex.exe!0x0cad2b50 kbutton_t in_speed
r5apex.exe!0x0cad2be0 kbutton_t in_strafe
r5apex.exe!0x271e00f0 kbutton_t in_toggle_duck
r5apex.exe!0x271e00e0 kbutton_t in_toggle_zoom
r5apex.exe!0x26df0640 kbutton_t in_use
r5apex.exe!0x26df05e0 kbutton_t in_useAndReload
r5apex.exe!0x271e0100 kbutton_t in_use_alt
r5apex.exe!0x26df05d0 kbutton_t in_use_long
r5apex.exe!0x0cad2bf8 kbutton_t in_variableScopeToggle
r5apex.exe!0x26df0610 kbutton_t in_walk
r5apex.exe!0x26df0830 kbutton_t in_weaponCycle
r5apex.exe!0x26df0600 kbutton_t in_weapon_discard
r5apex.exe!0x26df0620 kbutton_t in_zoom
```

## ClientClasses

<details>
<summary><code>client_class CAI_BaseNPC</code></summary>

class_id: `0`  
sizeof: `6720`  
</details>
<details>
<summary><code>client_class CAmbientGeneric</code></summary>

class_id: `1`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CBaseAnimating</code></summary>

class_id: `2`  
sizeof: `4864`  
</details>
<details>
<summary><code>client_class CBaseAnimatingOverlay</code></summary>

class_id: `3`  
sizeof: `5760`  
</details>
<details>
<summary><code>client_class CBaseButton</code></summary>

class_id: `0`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CBaseCombatCharacter</code></summary>

class_id: `4`  
sizeof: `6272`  
</details>
<details>
<summary><code>client_class CBaseEntity</code></summary>

class_id: `5`  
sizeof: `2560`  
</details>
<details>
<summary><code>client_class CBaseGrenade</code></summary>

class_id: `6`  
sizeof: `10432`  
</details>
<details>
<summary><code>client_class CBaseParticleEntity</code></summary>

class_id: `0`  
sizeof: `2880`  
</details>
<details>
<summary><code>client_class CBaseTempEntity</code></summary>

class_id: `7`  
sizeof: `40`  
</details>
<details>
<summary><code>client_class CBaseToggle</code></summary>

class_id: `8`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CBaseTrigger</code></summary>

class_id: `9`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CBaseVPhysicsTrigger</code></summary>

class_id: `11`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CBaseViewModel</code></summary>

class_id: `10`  
sizeof: `19520`  
</details>
<details>
<summary><code>client_class CBoneFollower</code></summary>

class_id: `12`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CBreakableProp</code></summary>

class_id: `13`  
sizeof: `4864`  
</details>
<details>
<summary><code>client_class CBreakableSurface</code></summary>

class_id: `14`  
sizeof: `3840`  
</details>
<details>
<summary><code>client_class CCascadeLight</code></summary>

class_id: `15`  
sizeof: `2944`  
</details>
<details>
<summary><code>client_class CColorCorrection</code></summary>

class_id: `16`  
sizeof: `2944`  
</details>
<details>
<summary><code>client_class CCrossbowBolt</code></summary>

class_id: `17`  
sizeof: `10304`  
</details>
<details>
<summary><code>client_class CDeathBoxProp</code></summary>

class_id: `18`  
sizeof: `4992`  
</details>
<details>
<summary><code>client_class CDynamicLight</code></summary>

class_id: `19`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CDynamicProp</code></summary>

class_id: `20`  
sizeof: `4928`  
</details>
<details>
<summary><code>client_class CDynamicPropLightweight</code></summary>

class_id: `21`  
sizeof: `4928`  
</details>
<details>
<summary><code>client_class CEntityBlocker</code></summary>

class_id: `22`  
sizeof: `2560`  
</details>
<details>
<summary><code>client_class CEntityDissolve</code></summary>

class_id: `23`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CEntityLinkPage</code></summary>

class_id: `24`  
sizeof: `4672`  
</details>
<details>
<summary><code>client_class CEnvDecoy</code></summary>

class_id: `25`  
sizeof: `4864`  
</details>
<details>
<summary><code>client_class CEnvWind</code></summary>

class_id: `26`  
sizeof: `2944`  
</details>
<details>
<summary><code>client_class CFirstPersonProxy</code></summary>

class_id: `27`  
sizeof: `4992`  
</details>
<details>
<summary><code>client_class CFuncBrush</code></summary>

class_id: `28`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CFuncBrushLightweight</code></summary>

class_id: `29`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CFuncMoveLinear</code></summary>

class_id: `30`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CGameRulesProxy</code></summary>

class_id: `31`  
sizeof: `2560`  
</details>
<details>
<summary><code>client_class CGlobalNonRewinding</code></summary>

class_id: `32`  
sizeof: `3648`  
</details>
<details>
<summary><code>client_class CGrappleHook</code></summary>

class_id: `33`  
sizeof: `4928`  
</details>
<details>
<summary><code>client_class CHardPointEntity</code></summary>

class_id: `34`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CHardPointFrontierEntity</code></summary>

class_id: `35`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CHealthKit</code></summary>

class_id: `36`  
sizeof: `4864`  
</details>
<details>
<summary><code>client_class CImportantOnEntSound</code></summary>

class_id: `37`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CInfoPlacementHelper</code></summary>

class_id: `38`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CInfoTarget</code></summary>

class_id: `39`  
sizeof: `2560`  
</details>
<details>
<summary><code>client_class CInfoTargetGravity</code></summary>

class_id: `40`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CInfoTargetMinimap</code></summary>

class_id: `41`  
sizeof: `2560`  
</details>
<details>
<summary><code>client_class CLootGrabber</code></summary>

class_id: `42`  
sizeof: `4992`  
</details>
<details>
<summary><code>client_class CMissile</code></summary>

class_id: `43`  
sizeof: `10496`  
</details>
<details>
<summary><code>client_class CMovieDisplay</code></summary>

class_id: `44`  
sizeof: `2944`  
</details>
<details>
<summary><code>client_class CNPC_Drone</code></summary>

class_id: `45`  
sizeof: `6784`  
</details>
<details>
<summary><code>client_class CNPC_Dropship</code></summary>

class_id: `46`  
sizeof: `6848`  
</details>
<details>
<summary><code>client_class CNPC_SentryTurret</code></summary>

class_id: `47`  
sizeof: `6784`  
</details>
<details>
<summary><code>client_class CNPC_Titan</code></summary>

class_id: `48`  
sizeof: `6912`  
</details>
<details>
<summary><code>client_class CParticleSystem</code></summary>

class_id: `49`  
sizeof: `2752`  
</details>
<details>
<summary><code>client_class CPhysBox</code></summary>

class_id: `50`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CPhysicsProp</code></summary>

class_id: `51`  
sizeof: `4992`  
</details>
<details>
<summary><code>client_class CPlayer</code></summary>

class_id: `52`  
sizeof: `16384`  
</details>
<details>
<summary><code>client_class CPlayerDecoy</code></summary>

class_id: `53`  
sizeof: `4992`  
</details>
<details>
<summary><code>client_class CPlayerResource</code></summary>

class_id: `54`  
sizeof: `12416`  
</details>
<details>
<summary><code>client_class CPlayerTasklist</code></summary>

class_id: `55`  
sizeof: `3968`  
</details>
<details>
<summary><code>client_class CPlayerVehicle</code></summary>

class_id: `56`  
sizeof: `5056`  
</details>
<details>
<summary><code>client_class CPlayerWaypoint</code></summary>

class_id: `57`  
sizeof: `3328`  
</details>
<details>
<summary><code>client_class CPointCamera</code></summary>

class_id: `58`  
sizeof: `2752`  
</details>
<details>
<summary><code>client_class CPortal_PointPush</code></summary>

class_id: `59`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CPostProcessController</code></summary>

class_id: `60`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CPredictedFirstPersonProxy</code></summary>

class_id: `61`  
sizeof: `5056`  
</details>
<details>
<summary><code>client_class CProjectile</code></summary>

class_id: `62`  
sizeof: `10240`  
</details>
<details>
<summary><code>client_class CPropDoor</code></summary>

class_id: `63`  
sizeof: `5120`  
</details>
<details>
<summary><code>client_class CPropSurvival</code></summary>

class_id: `64`  
sizeof: `4928`  
</details>
<details>
<summary><code>client_class CRopeKeyframe</code></summary>

class_id: `65`  
sizeof: `3840`  
</details>
<details>
<summary><code>client_class CScriptMover</code></summary>

class_id: `66`  
sizeof: `5440`  
</details>
<details>
<summary><code>client_class CScriptMoverTrainNode</code></summary>

class_id: `67`  
sizeof: `4160`  
</details>
<details>
<summary><code>client_class CScriptNetData</code></summary>

class_id: `68`  
sizeof: `3136`  
</details>
<details>
<summary><code>client_class CScriptNetDataGlobal</code></summary>

class_id: `74`  
sizeof: `3456`  
</details>
<details>
<summary><code>client_class CScriptNetData_SNDC_DEATH_BOX</code></summary>

class_id: `69`  
sizeof: `3200`  
</details>
<details>
<summary><code>client_class CScriptNetData_SNDC_GLOBAL</code></summary>

class_id: `70`  
sizeof: `3456`  
</details>
<details>
<summary><code>client_class CScriptNetData_SNDC_PLAYER_EXCLUSIVE</code></summary>

class_id: `71`  
sizeof: `3328`  
</details>
<details>
<summary><code>client_class CScriptNetData_SNDC_PLAYER_GLOBAL</code></summary>

class_id: `72`  
sizeof: `3328`  
</details>
<details>
<summary><code>client_class CScriptNetData_SNDC_TITAN_SOUL</code></summary>

class_id: `73`  
sizeof: `3264`  
</details>
<details>
<summary><code>client_class CScriptProp</code></summary>

class_id: `75`  
sizeof: `5120`  
</details>
<details>
<summary><code>client_class CScriptTraceVolume</code></summary>

class_id: `76`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CShieldProp</code></summary>

class_id: `77`  
sizeof: `4992`  
</details>
<details>
<summary><code>client_class CSkyCamera</code></summary>

class_id: `78`  
sizeof: `2560`  
</details>
<details>
<summary><code>client_class CStatueProp</code></summary>

class_id: `0`  
sizeof: `5056`  
</details>
<details>
<summary><code>client_class CStatusEffectPlugin</code></summary>

class_id: `79`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CTEBreakModel</code></summary>

class_id: `81`  
sizeof: `112`  
</details>
<details>
<summary><code>client_class CTEEffectDispatch</code></summary>

class_id: `82`  
sizeof: `208`  
</details>
<details>
<summary><code>client_class CTEExplosion</code></summary>

class_id: `83`  
sizeof: `136`  
</details>
<details>
<summary><code>client_class CTEGibEvent</code></summary>

class_id: `84`  
sizeof: `56`  
</details>
<details>
<summary><code>client_class CTEParticleSystem</code></summary>

class_id: `85`  
sizeof: `56`  
</details>
<details>
<summary><code>client_class CTEPhysicsProp</code></summary>

class_id: `86`  
sizeof: `96`  
</details>
<details>
<summary><code>client_class CTEProjectileTrail</code></summary>

class_id: `87`  
sizeof: `88`  
</details>
<details>
<summary><code>client_class CTEScriptParticleSystem</code></summary>

class_id: `88`  
sizeof: `80`  
</details>
<details>
<summary><code>client_class CTEScriptParticleSystemOnEntity</code></summary>

class_id: `89`  
sizeof: `64`  
</details>
<details>
<summary><code>client_class CTEScriptParticleSystemOnEntityWithPos</code></summary>

class_id: `90`  
sizeof: `88`  
</details>
<details>
<summary><code>client_class CTEShatterSurface</code></summary>

class_id: `91`  
sizeof: `120`  
</details>
<details>
<summary><code>client_class CTESoundDispatch</code></summary>

class_id: `92`  
sizeof: `72`  
</details>
<details>
<summary><code>client_class CTeam</code></summary>

class_id: `80`  
sizeof: `2944`  
</details>
<details>
<summary><code>client_class CTitanSoul</code></summary>

class_id: `93`  
sizeof: `3456`  
</details>
<details>
<summary><code>client_class CTriggerCylinderHeavy</code></summary>

class_id: `94`  
sizeof: `2816`  
</details>
<details>
<summary><code>client_class CTriggerNoGrapple</code></summary>

class_id: `95`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CTriggerNoZipline</code></summary>

class_id: `96`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CTriggerPlayerMovement</code></summary>

class_id: `97`  
sizeof: `2752`  
</details>
<details>
<summary><code>client_class CTriggerPointGravity</code></summary>

class_id: `98`  
sizeof: `2752`  
</details>
<details>
<summary><code>client_class CTriggerSlip</code></summary>

class_id: `99`  
sizeof: `2752`  
</details>
<details>
<summary><code>client_class CTurret</code></summary>

class_id: `100`  
sizeof: `5056`  
</details>
<details>
<summary><code>client_class CVGuiScreen</code></summary>

class_id: `101`  
sizeof: `2816`  
</details>
<details>
<summary><code>client_class CVortexSphere</code></summary>

class_id: `102`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CWaterLODControl</code></summary>

class_id: `103`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CWeaponX</code></summary>

class_id: `104`  
sizeof: `25088`  
</details>
<details>
<summary><code>client_class CWorld</code></summary>

class_id: `105`  
sizeof: `3008`  
</details>
<details>
<summary><code>client_class CZipline</code></summary>

class_id: `106`  
sizeof: `4160`  
</details>
<details>
<summary><code>client_class CZiplineEnd</code></summary>

class_id: `107`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class DoorMover</code></summary>

class_id: `108`  
sizeof: `5504`  
</details>
<details>
<summary><code>client_class ScriptMoverLightweight</code></summary>

class_id: `109`  
sizeof: `5504`  
</details>
<details>
<summary><code>client_class Titan_Cockpit</code></summary>

class_id: `0`  
sizeof: `5440`  
</details>

### Addresses

```
r5apex.exe!0x010482d8 ClientClass CAI_BaseNPC
r5apex.exe!0x010452d8 ClientClass CAmbientGeneric
r5apex.exe!0x01047478 ClientClass CBaseAnimating
r5apex.exe!0x01045c48 ClientClass CBaseAnimatingOverlay
r5apex.exe!0x01047e38 ClientClass CBaseButton
r5apex.exe!0x01042378 ClientClass CBaseCombatCharacter
r5apex.exe!0x01047518 ClientClass CBaseEntity
r5apex.exe!0x0117d7b8 ClientClass CBaseGrenade
r5apex.exe!0x01deb498 ClientClass CBaseParticleEntity
r5apex.exe!0x01047698 ClientClass CBaseTempEntity
r5apex.exe!0x010467f8 ClientClass CBaseToggle
r5apex.exe!0x0110eb88 ClientClass CBaseTrigger
r5apex.exe!0x01048a28 ClientClass CBaseVPhysicsTrigger
r5apex.exe!0x01bd3068 ClientClass CBaseViewModel
r5apex.exe!0x01040a78 ClientClass CBoneFollower
r5apex.exe!0x01041838 ClientClass CBreakableProp
r5apex.exe!0x01043e48 ClientClass CBreakableSurface
r5apex.exe!0x01041518 ClientClass CCascadeLight
r5apex.exe!0x01047158 ClientClass CColorCorrection
r5apex.exe!0x01df5a88 ClientClass CCrossbowBolt
r5apex.exe!0x01040578 ClientClass CDeathBoxProp
r5apex.exe!0x01046bb8 ClientClass CDynamicLight
r5apex.exe!0x01047338 ClientClass CDynamicProp
r5apex.exe!0x01040ed8 ClientClass CDynamicPropLightweight
r5apex.exe!0x01dc87d0 ClientClass CEntityBlocker
r5apex.exe!0x01045698 ClientClass CEntityDissolve
r5apex.exe!0x0104de18 ClientClass CEntityLinkPage
r5apex.exe!0x01dd73c0 ClientClass CEnvDecoy
r5apex.exe!0x010413d8 ClientClass CEnvWind
r5apex.exe!0x01df4ea8 ClientClass CFirstPersonProxy
r5apex.exe!0x01045b08 ClientClass CFuncBrush
r5apex.exe!0x01043578 ClientClass CFuncBrushLightweight
r5apex.exe!0x01043098 ClientClass CFuncMoveLinear
r5apex.exe!0x01dcd788 ClientClass CGameRulesProxy
r5apex.exe!0x01bdaec8 ClientClass CGlobalNonRewinding
r5apex.exe!0x01df6728 ClientClass CGrappleHook
r5apex.exe!0x01049ee8 ClientClass CHardPointEntity
r5apex.exe!0x0104af68 ClientClass CHardPointFrontierEntity
r5apex.exe!0x01dee238 ClientClass CHealthKit
r5apex.exe!0x0110f108 ClientClass CImportantOnEntSound
r5apex.exe!0x01df0dc8 ClientClass CInfoPlacementHelper
r5apex.exe!0x01dca048 ClientClass CInfoTarget
r5apex.exe!0x0117b468 ClientClass CInfoTargetGravity
r5apex.exe!0x01bd4938 ClientClass CInfoTargetMinimap
r5apex.exe!0x01046e38 ClientClass CLootGrabber
r5apex.exe!0x01df3d58 ClientClass CMissile
r5apex.exe!0x01041dd8 ClientClass CMovieDisplay
r5apex.exe!0x01042d78 ClientClass CNPC_Drone
r5apex.exe!0x01042c38 ClientClass CNPC_Dropship
r5apex.exe!0x01040f78 ClientClass CNPC_SentryTurret
r5apex.exe!0x01044dd8 ClientClass CNPC_Titan
r5apex.exe!0x010454b8 ClientClass CParticleSystem
r5apex.exe!0x01044208 ClientClass CPhysBox
r5apex.exe!0x01046328 ClientClass CPhysicsProp
r5apex.exe!0x01041338 ClientClass CPlayer
r5apex.exe!0x01de2b28 ClientClass CPlayerDecoy
r5apex.exe!0x01046148 ClientClass CPlayerResource
r5apex.exe!0x01dd8f48 ClientClass CPlayerTasklist
r5apex.exe!0x0104c298 ClientClass CPlayerVehicle
r5apex.exe!0x01dd8ea0 ClientClass CPlayerWaypoint
r5apex.exe!0x01044978 ClientClass CPointCamera
r5apex.exe!0x01df1c88 ClientClass CPortal_PointPush
r5apex.exe!0x01041298 ClientClass CPostProcessController
r5apex.exe!0x01df46e8 ClientClass CPredictedFirstPersonProxy
r5apex.exe!0x01df3838 ClientClass CProjectile
r5apex.exe!0x01814718 ClientClass CPropDoor
r5apex.exe!0x01043938 ClientClass CPropSurvival
r5apex.exe!0x01dd3418 ClientClass CRopeKeyframe
r5apex.exe!0x01de13d8 ClientClass CScriptMover
r5apex.exe!0x01dd3238 ClientClass CScriptMoverTrainNode
r5apex.exe!0x01ddb568 ClientClass CScriptNetData
r5apex.exe!0x01dd73f8 ClientClass CScriptNetDataGlobal
r5apex.exe!0x01dd2e78 ClientClass CScriptNetData_SNDC_DEATH_BOX
r5apex.exe!0x01dd8ed8 ClientClass CScriptNetData_SNDC_GLOBAL
r5apex.exe!0x01dd8810 ClientClass CScriptNetData_SNDC_PLAYER_EXCLUSIVE
r5apex.exe!0x01dd83a0 ClientClass CScriptNetData_SNDC_PLAYER_GLOBAL
r5apex.exe!0x01dd8f10 ClientClass CScriptNetData_SNDC_TITAN_SOUL
r5apex.exe!0x010434d8 ClientClass CScriptProp
r5apex.exe!0x01df36f8 ClientClass CScriptTraceVolume
r5apex.exe!0x01040e38 ClientClass CShieldProp
r5apex.exe!0x01046758 ClientClass CSkyCamera
r5apex.exe!0x01047018 ClientClass CStatueProp
r5apex.exe!0x01ddb4c8 ClientClass CStatusEffectPlugin
r5apex.exe!0x0104cf18 ClientClass CTEBreakModel
r5apex.exe!0x01049bc8 ClientClass CTEEffectDispatch
r5apex.exe!0x01049948 ClientClass CTEExplosion
r5apex.exe!0x01045ce8 ClientClass CTEGibEvent
r5apex.exe!0x0110ed48 ClientClass CTEParticleSystem
r5apex.exe!0x0104a808 ClientClass CTEPhysicsProp
r5apex.exe!0x01df57c8 ClientClass CTEProjectileTrail
r5apex.exe!0x01bdabe8 ClientClass CTEScriptParticleSystem
r5apex.exe!0x01bda328 ClientClass CTEScriptParticleSystemOnEntity
r5apex.exe!0x01dcbb30 ClientClass CTEScriptParticleSystemOnEntityWithPos
r5apex.exe!0x0110e9c8 ClientClass CTEShatterSurface
r5apex.exe!0x01048ec8 ClientClass CTESoundDispatch
r5apex.exe!0x0110efc8 ClientClass CTeam
r5apex.exe!0x01042b98 ClientClass CTitanSoul
r5apex.exe!0x01dd83d8 ClientClass CTriggerCylinderHeavy
r5apex.exe!0x010498a8 ClientClass CTriggerNoGrapple
r5apex.exe!0x0104dfd8 ClientClass CTriggerNoZipline
r5apex.exe!0x01dde188 ClientClass CTriggerPlayerMovement
r5apex.exe!0x01dda378 ClientClass CTriggerPointGravity
r5apex.exe!0x01dd8848 ClientClass CTriggerSlip
r5apex.exe!0x01df5658 ClientClass CTurret
r5apex.exe!0x0110f2c8 ClientClass CVGuiScreen
r5apex.exe!0x01df6988 ClientClass CVortexSphere
r5apex.exe!0x0104bc58 ClientClass CWaterLODControl
r5apex.exe!0x01df9608 ClientClass CWeaponX
r5apex.exe!0x0110f228 ClientClass CWorld
r5apex.exe!0x01df4968 ClientClass CZipline
r5apex.exe!0x01df42d8 ClientClass CZiplineEnd
r5apex.exe!0x01dd6348 ClientClass DoorMover
r5apex.exe!0x01dde078 ClientClass ScriptMoverLightweight
r5apex.exe!0x01df36c0 ClientClass Titan_Cockpit
```

## RecvTables

<details>
<summary><code>class DT_AI_BaseNPC extends DT_BaseCombatCharacter</code></summary>

```
{
	statuseffectsdata_npc: DT_AI_BaseNPC_StatusEffects,
	m_localOrigin: Vector,
	m_hGroundEntity: Int,
	m_iHealth: Int,
	m_localAngles: Vector,
	m_iMaxHealth: Int,
	m_lifeState: Int,
	m_inventory: DT_WeaponInventoryActiveWeaponOnly,
	m_fireteamSlotIndex: Int,
	m_aiSprinting: Int,
	m_aiNetworkFlags: Int,
	m_isHologram: Int,
	m_title: String,
	m_aiSettingsIndex: Int,
	m_subclass: Int,
}
```

### Offsets

```
DT_AI_BaseNPC!0x0000 statuseffectsdata_npc
DT_AI_BaseNPC!0x0004 m_localOrigin
DT_AI_BaseNPC!0x03dc m_hGroundEntity
DT_AI_BaseNPC!0x03e0 m_iHealth
DT_AI_BaseNPC!0x0428 m_localAngles
DT_AI_BaseNPC!0x0510 m_iMaxHealth
DT_AI_BaseNPC!0x0730 m_lifeState
DT_AI_BaseNPC!0x16b0 m_inventory
DT_AI_BaseNPC!0x1880 m_fireteamSlotIndex
DT_AI_BaseNPC!0x19ea m_aiSprinting
DT_AI_BaseNPC!0x1a0c m_aiNetworkFlags
DT_AI_BaseNPC!0x1a10 m_isHologram
DT_AI_BaseNPC!0x1a11 m_title
DT_AI_BaseNPC!0x1a34 m_aiSettingsIndex
DT_AI_BaseNPC!0x1a38 m_subclass
```
</details>
<details>
<summary><code>class DT_AI_BaseNPC_StatusEffects</code></summary>

```
{
	m_statusEffectsTimedNPCNV: DataTable,
	m_statusEffectsEndlessNPCNV: DataTable,
}
```

### Offsets

```
DT_AI_BaseNPC_StatusEffects!0x1888 m_statusEffectsTimedNPCNV
DT_AI_BaseNPC_StatusEffects!0x18d0 m_statusEffectsEndlessNPCNV
```
</details>
<details>
<summary><code>class DT_AmbientGeneric extends DT_BaseEntity</code></summary>

```
{
	m_radius: Float,
	m_isEnabled: Int,
	m_networkTableSoundID: Int,
	m_networkedSegmentEndpointWorldSpace: Vector,
	m_hasPolylineSegments: Int,
}
```

### Offsets

```
DT_AmbientGeneric!0x0a00 m_radius
DT_AmbientGeneric!0x0a04 m_isEnabled
DT_AmbientGeneric!0x0a10 m_networkTableSoundID
DT_AmbientGeneric!0x0a18 m_networkedSegmentEndpointWorldSpace
DT_AmbientGeneric!0x0a24 m_hasPolylineSegments
```
</details>
<details>
<summary><code>class DT_AnimRelativeData</code></summary>

```
{
	m_animInitialPos: Vector,
	m_animInitialVel: Vector,
	m_animInitialRot: Rotation,
	m_animInitialCorrectPos: Vector,
	m_animInitialCorrectRot: Rotation,
	m_animEntityToRefOffset: Vector,
	m_animEntityToRefRotation: Rotation,
	m_animBlendBeginTime: Time,
	m_animBlendEndTime: Time,
	m_animScriptSequence: Int,
	m_animScriptModel: Int,
	m_animIgnoreParentRot: Int,
	m_animMotionMode: Int,
}
```

### Offsets

```
DT_AnimRelativeData!0x0000 m_animInitialPos
DT_AnimRelativeData!0x000c m_animInitialVel
DT_AnimRelativeData!0x0018 m_animInitialRot
DT_AnimRelativeData!0x0028 m_animInitialCorrectPos
DT_AnimRelativeData!0x0034 m_animInitialCorrectRot
DT_AnimRelativeData!0x0044 m_animEntityToRefOffset
DT_AnimRelativeData!0x0050 m_animEntityToRefRotation
DT_AnimRelativeData!0x0060 m_animBlendBeginTime
DT_AnimRelativeData!0x0064 m_animBlendEndTime
DT_AnimRelativeData!0x0068 m_animScriptSequence
DT_AnimRelativeData!0x006c m_animScriptModel
DT_AnimRelativeData!0x0070 m_animIgnoreParentRot
DT_AnimRelativeData!0x0074 m_animMotionMode
```
</details>
<details>
<summary><code>class DT_BaseAnimating extends DT_BaseEntity</code></summary>

```
{
	serveranimdata: DT_ServerAnimationData,
	m_animPlaybackRate: Float,
	m_animFrozen: Int,
	m_animModelIndex: Int,
	m_animSequence: Int,
	m_nNewSequenceParity: Int,
	m_flPoseParameter: DataTable,
	m_bClientSideRagdoll: Int,
	m_vecForce: Vector,
	m_flEstIkOffset: Float,
	m_passDamageToParent: Int,
	m_animNetworkFlags: Int,
	m_animActive: Int,
	m_animCollisionEnabled: Int,
	m_animPlantingEnabled: Int,
	m_animRelativeData: DT_AnimRelativeData,
	m_syncingWithEntity: Int,
	m_predictedAnimEventData: DT_PredictedAnimEventData,
	m_nRagdollImpactFXTableId: Int,
	m_flSkyScaleStartValue: Float,
	m_flSkyScaleEndValue: Float,
	m_flSkyScaleStartTime: Time,
	m_flSkyScaleEndTime: Time,
	m_SequenceTransitioner: DT_SequenceTransitioner,
	m_nSkin: Int,
	m_skinMod: Int,
	m_nBody: Int,
	m_camoIndex: Int,
	m_nForceBone: Int,
	m_bSequenceFinished: Int,
	m_lockedAnimDeltaYaw: Float,
	m_flModelScale: Float,
}
```

### Offsets

```
DT_BaseAnimating!0x0000 serveranimdata
DT_BaseAnimating!0x0010 m_animPlaybackRate
DT_BaseAnimating!0x0014 m_animFrozen
DT_BaseAnimating!0x0018 m_animModelIndex
DT_BaseAnimating!0x001c m_animSequence
DT_BaseAnimating!0x0020 m_nNewSequenceParity
DT_BaseAnimating!0x0024 m_flPoseParameter
DT_BaseAnimating!0x0084 m_bClientSideRagdoll
DT_BaseAnimating!0x0088 m_vecForce
DT_BaseAnimating!0x0094 m_flEstIkOffset
DT_BaseAnimating!0x075c m_passDamageToParent
DT_BaseAnimating!0x0a28 m_animNetworkFlags
DT_BaseAnimating!0x0a2c m_animActive
DT_BaseAnimating!0x0a2f m_animCollisionEnabled
DT_BaseAnimating!0x0a30 m_animPlantingEnabled
DT_BaseAnimating!0x0a34 m_animRelativeData
DT_BaseAnimating!0x0b24 m_syncingWithEntity
DT_BaseAnimating!0x0b28 m_predictedAnimEventData
DT_BaseAnimating!0x0b94 m_nRagdollImpactFXTableId
DT_BaseAnimating!0x0b98 m_flSkyScaleStartValue
DT_BaseAnimating!0x0b9c m_flSkyScaleEndValue
DT_BaseAnimating!0x0ba0 m_flSkyScaleStartTime
DT_BaseAnimating!0x0ba4 m_flSkyScaleEndTime
DT_BaseAnimating!0x0bc0 m_SequenceTransitioner
DT_BaseAnimating!0x0e48 m_nSkin
DT_BaseAnimating!0x0e4c m_skinMod
DT_BaseAnimating!0x0e50 m_nBody
DT_BaseAnimating!0x0e54 m_camoIndex
DT_BaseAnimating!0x0e90 m_nForceBone
DT_BaseAnimating!0x0f00 m_bSequenceFinished
DT_BaseAnimating!0x0f04 m_lockedAnimDeltaYaw
DT_BaseAnimating!0x0f0c m_flModelScale
```
</details>
<details>
<summary><code>class DT_BaseAnimatingOverlay extends DT_BaseAnimating</code></summary>

```
{
	overlay_vars: DT_OverlayVars,
	m_animOverlayIsActive: DataTable,
	m_animOverlayStartTime: DataTable,
	m_animOverlayStartCycle: DataTable,
	m_animOverlayPlaybackRate: DataTable,
	m_animOverlayModelIndex: DataTable,
	m_animOverlaySequence: DataTable,
	m_animOverlayWeight: DataTable,
	m_animOverlayOrder: DataTable,
	m_animOverlayAnimTime: DataTable,
	m_animOverlayFadeInDuration: DataTable,
	m_animOverlayFadeOutDuration: DataTable,
}
```

### Offsets

```
DT_BaseAnimatingOverlay!0x0000 overlay_vars
DT_BaseAnimatingOverlay!0x0008 m_animOverlayIsActive
DT_BaseAnimatingOverlay!0x0014 m_animOverlayStartTime
DT_BaseAnimatingOverlay!0x0038 m_animOverlayStartCycle
DT_BaseAnimatingOverlay!0x005c m_animOverlayPlaybackRate
DT_BaseAnimatingOverlay!0x0080 m_animOverlayModelIndex
DT_BaseAnimatingOverlay!0x00a4 m_animOverlaySequence
DT_BaseAnimatingOverlay!0x00c8 m_animOverlayWeight
DT_BaseAnimatingOverlay!0x00ec m_animOverlayOrder
DT_BaseAnimatingOverlay!0x0110 m_animOverlayAnimTime
DT_BaseAnimatingOverlay!0x0134 m_animOverlayFadeInDuration
DT_BaseAnimatingOverlay!0x0158 m_animOverlayFadeOutDuration
```
</details>
<details>
<summary><code>class DT_BaseCombatCharacter extends DT_BaseAnimatingOverlay</code></summary>

```
{
	bcc_localdata: DT_BCCLocalPlayerExclusive,
	m_weaponGettingSwitchedOut: DataTable,
	m_showActiveWeapon3p: DataTable,
	m_vecViewOffset.x: Float,
	m_vecViewOffset.y: Float,
	m_vecViewOffset.z: Float,
	m_cloakEndTime: Float,
	m_cloakFadeInEndTime: Time,
	m_cloakFadeOutStartTime: Float,
	m_cloakFadeInDuration: Float,
	m_cloakFlickerAmount: Float,
	m_cloakFlickerEndTime: Time,
	m_networkedFlags: Int,
	m_deathVelocity: Vector,
	m_minimapData: DT_MinimapBaseEntityData,
	m_nameVisibilityFlags: Int,
	m_lastFiredTime: Time,
	m_lastFiredWeapon: Int,
	m_raiseFromMeleeEndTime: Time,
	m_sharedEnergyCount: Int,
	m_sharedEnergyTotal: Int,
	m_sharedEnergyLockoutThreshold: Int,
	m_lastSharedEnergyRegenTime: Time,
	m_sharedEnergyRegenRate: Time,
	m_sharedEnergyRegenDelay: Float,
	m_lastSharedEnergyTakeTime: Time,
	m_selectedWeapons: DataTable,
	m_latestPrimaryWeapons: DataTable,
	m_latestPrimaryWeaponsIndexZeroOrOne: DataTable,
	m_latestNonOffhandWeapons: DataTable,
	m_lastCycleSlot: Int,
	m_weaponPermission: Int,
	m_weaponDelayEnableTime: Time,
	m_weaponDisabledInScript: Int,
	m_weaponDisabledFlags: Int,
	m_hudInfo_visibilityTestAlwaysPasses: Int,
	m_contextAction: Int,
	m_phaseShiftTimeStart: Time,
	m_phaseShiftTimeEnd: Time,
	m_targetInfoIconName: String,
}
```

### Offsets

```
DT_BaseCombatCharacter!0x0000 bcc_localdata
DT_BaseCombatCharacter!0x0008 m_weaponGettingSwitchedOut
DT_BaseCombatCharacter!0x0010 m_showActiveWeapon3p
DT_BaseCombatCharacter!0x0038 m_vecViewOffset.x
DT_BaseCombatCharacter!0x003c m_vecViewOffset.y
DT_BaseCombatCharacter!0x0040 m_vecViewOffset.z
DT_BaseCombatCharacter!0x019c m_cloakEndTime
DT_BaseCombatCharacter!0x01a0 m_cloakFadeInEndTime
DT_BaseCombatCharacter!0x01a4 m_cloakFadeOutStartTime
DT_BaseCombatCharacter!0x01a8 m_cloakFadeInDuration
DT_BaseCombatCharacter!0x01ac m_cloakFlickerAmount
DT_BaseCombatCharacter!0x01b0 m_cloakFlickerEndTime
DT_BaseCombatCharacter!0x0394 m_networkedFlags
DT_BaseCombatCharacter!0x0410 m_deathVelocity
DT_BaseCombatCharacter!0x0908 m_minimapData
DT_BaseCombatCharacter!0x0958 m_nameVisibilityFlags
DT_BaseCombatCharacter!0x1684 m_lastFiredTime
DT_BaseCombatCharacter!0x1688 m_lastFiredWeapon
DT_BaseCombatCharacter!0x168c m_raiseFromMeleeEndTime
DT_BaseCombatCharacter!0x1690 m_sharedEnergyCount
DT_BaseCombatCharacter!0x1694 m_sharedEnergyTotal
DT_BaseCombatCharacter!0x1698 m_sharedEnergyLockoutThreshold
DT_BaseCombatCharacter!0x169c m_lastSharedEnergyRegenTime
DT_BaseCombatCharacter!0x16a0 m_sharedEnergyRegenRate
DT_BaseCombatCharacter!0x16a4 m_sharedEnergyRegenDelay
DT_BaseCombatCharacter!0x16a8 m_lastSharedEnergyTakeTime
DT_BaseCombatCharacter!0x1700 m_selectedWeapons
DT_BaseCombatCharacter!0x1704 m_latestPrimaryWeapons
DT_BaseCombatCharacter!0x170c m_latestPrimaryWeaponsIndexZeroOrOne
DT_BaseCombatCharacter!0x1714 m_latestNonOffhandWeapons
DT_BaseCombatCharacter!0x171c m_lastCycleSlot
DT_BaseCombatCharacter!0x1724 m_weaponPermission
DT_BaseCombatCharacter!0x1728 m_weaponDelayEnableTime
DT_BaseCombatCharacter!0x172c m_weaponDisabledInScript
DT_BaseCombatCharacter!0x1751 m_weaponDisabledFlags
DT_BaseCombatCharacter!0x1752 m_hudInfo_visibilityTestAlwaysPasses
DT_BaseCombatCharacter!0x1764 m_contextAction
DT_BaseCombatCharacter!0x1790 m_phaseShiftTimeStart
DT_BaseCombatCharacter!0x1794 m_phaseShiftTimeEnd
DT_BaseCombatCharacter!0x17e4 m_targetInfoIconName
```
</details>
<details>
<summary><code>class DT_BaseEntity</code></summary>

```
{
	movetype: Int,
	movecollide: Int,
	predictable_id: DT_PredictableId,
	HighlightSettings: DT_HighlightSettings,
	moveparent: Int,
	m_parentAttachmentIndex: Int,
	m_fEffects: Int,
	m_usableType: Int,
	m_cellX: Int,
	m_cellY: Int,
	m_clrRender: Int,
	m_cellZ: Int,
	m_clIntensity: Int,
	m_localOrigin: Vector,
	m_nModelIndex: Int,
	m_bossPlayer: Int,
	m_shieldHealth: Int,
	m_shieldHealthMax: Int,
	m_bIsSoundCodeControllerValueSet: Int,
	m_flSoundCodeControllerValue: Float,
	m_networkedFlags: Int,
	m_visibilityFlags: Int,
	m_iTeamNum: Int,
	m_teamMemberIndex: Int,
	m_squadID: Int,
	m_grade: Int,
	m_ignorePredictedTriggerFlags: Int,
	m_passThroughFlags: Int,
	m_passThroughThickness: Int,
	m_passThroughDirection: Float,
	m_localAngles: Vector,
	m_hOwnerEntity: Int,
	m_bRenderWithViewModels: Int,
	m_nRenderFX: Int,
	m_nRenderMode: Int,
	m_Collision: DT_CollisionProperty,
	m_CollisionGroup: Int,
	m_contents: Int,
	m_collideWithOwner: Int,
	m_iSignifierName: String,
	m_iName: String,
	m_scriptNameIndex: Int,
	m_instanceNameIndex: Int,
	m_holdUsePrompt: String,
	m_pressUsePrompt: String,
	m_phaseShiftFlags: Int,
	m_baseTakeDamage: Int,
	m_invulnerableToDamageCount: Int,
	m_attachmentLerpStartTime: Time,
	m_attachmentLerpEndTime: Time,
	m_attachmentLerpStartOrigin: Vector,
	m_attachmentLerpStartAngles: Vector,
	m_parentAttachmentModel: Int,
	m_fadeDist: Float,
	m_dissolveEffectEntityHandle: Int,
	m_usablePriority: Int,
	m_usableDistanceOverride: Float,
	m_usableFOV: Float,
	m_usePromptSize: Float,
	m_spottedByTeams: DataTable,
	m_firstChildEntityLink: Int,
	m_firstParentEntityLink: Int,
	m_realmsBitMask: BitMask,
}
```

### Offsets

```
DT_BaseEntity!0x0000 movetype
DT_BaseEntity!0x0000 movecollide
DT_BaseEntity!0x0000 predictable_id
DT_BaseEntity!0x0000 HighlightSettings
DT_BaseEntity!0x001c moveparent
DT_BaseEntity!0x0024 m_parentAttachmentIndex
DT_BaseEntity!0x0044 m_fEffects
DT_BaseEntity!0x0048 m_usableType
DT_BaseEntity!0x004c m_cellX
DT_BaseEntity!0x0050 m_cellY
DT_BaseEntity!0x0050 m_clrRender
DT_BaseEntity!0x0054 m_cellZ
DT_BaseEntity!0x0054 m_clIntensity
DT_BaseEntity!0x0058 m_localOrigin
DT_BaseEntity!0x0064 m_nModelIndex
DT_BaseEntity!0x0124 m_bossPlayer
DT_BaseEntity!0x0170 m_shieldHealth
DT_BaseEntity!0x0174 m_shieldHealthMax
DT_BaseEntity!0x038c m_bIsSoundCodeControllerValueSet
DT_BaseEntity!0x0390 m_flSoundCodeControllerValue
DT_BaseEntity!0x0394 m_networkedFlags
DT_BaseEntity!0x03e8 m_visibilityFlags
DT_BaseEntity!0x03f0 m_iTeamNum
DT_BaseEntity!0x03f4 m_teamMemberIndex
DT_BaseEntity!0x03f8 m_squadID
DT_BaseEntity!0x03fc m_grade
DT_BaseEntity!0x0400 m_ignorePredictedTriggerFlags
DT_BaseEntity!0x0404 m_passThroughFlags
DT_BaseEntity!0x0408 m_passThroughThickness
DT_BaseEntity!0x040c m_passThroughDirection
DT_BaseEntity!0x0428 m_localAngles
DT_BaseEntity!0x043c m_hOwnerEntity
DT_BaseEntity!0x0440 m_bRenderWithViewModels
DT_BaseEntity!0x0441 m_nRenderFX
DT_BaseEntity!0x0451 m_nRenderMode
DT_BaseEntity!0x0458 m_Collision
DT_BaseEntity!0x04d8 m_CollisionGroup
DT_BaseEntity!0x04dc m_contents
DT_BaseEntity!0x04e0 m_collideWithOwner
DT_BaseEntity!0x0518 m_iSignifierName
DT_BaseEntity!0x0521 m_iName
DT_BaseEntity!0x0628 m_scriptNameIndex
DT_BaseEntity!0x062c m_instanceNameIndex
DT_BaseEntity!0x06b0 m_holdUsePrompt
DT_BaseEntity!0x06b8 m_pressUsePrompt
DT_BaseEntity!0x0750 m_phaseShiftFlags
DT_BaseEntity!0x0754 m_baseTakeDamage
DT_BaseEntity!0x0758 m_invulnerableToDamageCount
DT_BaseEntity!0x07cc m_attachmentLerpStartTime
DT_BaseEntity!0x07d0 m_attachmentLerpEndTime
DT_BaseEntity!0x07d4 m_attachmentLerpStartOrigin
DT_BaseEntity!0x07e0 m_attachmentLerpStartAngles
DT_BaseEntity!0x07f8 m_parentAttachmentModel
DT_BaseEntity!0x0804 m_fadeDist
DT_BaseEntity!0x08b8 m_dissolveEffectEntityHandle
DT_BaseEntity!0x08c8 m_usablePriority
DT_BaseEntity!0x08cc m_usableDistanceOverride
DT_BaseEntity!0x08d0 m_usableFOV
DT_BaseEntity!0x08d4 m_usePromptSize
DT_BaseEntity!0x08e8 m_spottedByTeams
DT_BaseEntity!0x09e0 m_firstChildEntityLink
DT_BaseEntity!0x09e4 m_firstParentEntityLink
DT_BaseEntity!0x09e8 m_realmsBitMask
```
</details>
<details>
<summary><code>class DT_BaseGrenade extends DT_Projectile</code></summary>

```
{
	moveparent: Int,
	m_parentAttachmentType: Int,
	m_parentAttachmentIndex: Int,
	m_cloakEndTime: Float,
	m_cloakFadeInEndTime: Time,
	m_cloakFadeOutStartTime: Float,
	m_cloakFadeInDuration: Float,
	m_cloakFlickerAmount: Float,
	m_cloakFlickerEndTime: Time,
	m_baseTakeDamage: Int,
	m_invulnerableToDamageCount: Int,
	m_parentAttachmentHitbox: Int,
	m_parentAttachmentModel: Int,
	m_doesExplode: Int,
	m_DmgRadius: Float,
	m_ziplineGrenadeExpectedEndPosition: Vector,
}
```

### Offsets

```
DT_BaseGrenade!0x001c moveparent
DT_BaseGrenade!0x0020 m_parentAttachmentType
DT_BaseGrenade!0x0024 m_parentAttachmentIndex
DT_BaseGrenade!0x019c m_cloakEndTime
DT_BaseGrenade!0x01a0 m_cloakFadeInEndTime
DT_BaseGrenade!0x01a4 m_cloakFadeOutStartTime
DT_BaseGrenade!0x01a8 m_cloakFadeInDuration
DT_BaseGrenade!0x01ac m_cloakFlickerAmount
DT_BaseGrenade!0x01b0 m_cloakFlickerEndTime
DT_BaseGrenade!0x0754 m_baseTakeDamage
DT_BaseGrenade!0x0758 m_invulnerableToDamageCount
DT_BaseGrenade!0x07f4 m_parentAttachmentHitbox
DT_BaseGrenade!0x07f8 m_parentAttachmentModel
DT_BaseGrenade!0x2801 m_doesExplode
DT_BaseGrenade!0x2804 m_DmgRadius
DT_BaseGrenade!0x2890 m_ziplineGrenadeExpectedEndPosition
```
</details>
<details>
<summary><code>class DT_BaseViewModel</code></summary>

```
{
	overlay_vars: DT_OverlayVars,
	m_animStartTime: Time,
	m_animOverlayIsActive: DataTable,
	m_animStartCycle: Float,
	m_animPlaybackRate: Float,
	m_animFrozen: Int,
	m_animOverlayStartTime: DataTable,
	m_animModelIndex: Int,
	m_animSequence: Int,
	m_nNewSequenceParity: Int,
	m_animOverlayStartCycle: DataTable,
	m_fEffects: Int,
	m_clrRender: Int,
	m_animOverlayPlaybackRate: DataTable,
	m_nModelIndex: Int,
	m_animOverlayModelIndex: DataTable,
	m_animOverlaySequence: DataTable,
	m_animOverlayWeight: DataTable,
	m_animOverlayOrder: DataTable,
	m_animOverlayAnimTime: DataTable,
	m_animOverlayFadeInDuration: DataTable,
	m_animOverlayFadeOutDuration: DataTable,
	m_nRenderMode: Int,
	m_nBody: Int,
	m_nResetEventsParity: Int,
	m_bSequenceFinished: Int,
	m_flModelScale: Float,
	m_overlayEventParity: DataTable,
	m_viewModelOwner: Int,
	m_projectileIsVisible: Int,
	m_bBlockEventLayer: Int,
	m_isAdsTransition: Int,
	m_hWeapon: Int,
	m_tracerAttachments: DataTable,
	m_tracerAttachmentsScoped: DataTable,
}
```

### Offsets

```
DT_BaseViewModel!0x0000 overlay_vars
DT_BaseViewModel!0x0008 m_animStartTime
DT_BaseViewModel!0x0008 m_animOverlayIsActive
DT_BaseViewModel!0x000c m_animStartCycle
DT_BaseViewModel!0x0010 m_animPlaybackRate
DT_BaseViewModel!0x0014 m_animFrozen
DT_BaseViewModel!0x0014 m_animOverlayStartTime
DT_BaseViewModel!0x0018 m_animModelIndex
DT_BaseViewModel!0x001c m_animSequence
DT_BaseViewModel!0x0020 m_nNewSequenceParity
DT_BaseViewModel!0x0038 m_animOverlayStartCycle
DT_BaseViewModel!0x0044 m_fEffects
DT_BaseViewModel!0x0050 m_clrRender
DT_BaseViewModel!0x005c m_animOverlayPlaybackRate
DT_BaseViewModel!0x0064 m_nModelIndex
DT_BaseViewModel!0x0080 m_animOverlayModelIndex
DT_BaseViewModel!0x00a4 m_animOverlaySequence
DT_BaseViewModel!0x00c8 m_animOverlayWeight
DT_BaseViewModel!0x00ec m_animOverlayOrder
DT_BaseViewModel!0x0110 m_animOverlayAnimTime
DT_BaseViewModel!0x0134 m_animOverlayFadeInDuration
DT_BaseViewModel!0x0158 m_animOverlayFadeOutDuration
DT_BaseViewModel!0x0451 m_nRenderMode
DT_BaseViewModel!0x0e50 m_nBody
DT_BaseViewModel!0x0e5c m_nResetEventsParity
DT_BaseViewModel!0x0f00 m_bSequenceFinished
DT_BaseViewModel!0x0f0c m_flModelScale
DT_BaseViewModel!0x1411 m_overlayEventParity
DT_BaseViewModel!0x16d8 m_viewModelOwner
DT_BaseViewModel!0x16dc m_projectileIsVisible
DT_BaseViewModel!0x1ac0 m_bBlockEventLayer
DT_BaseViewModel!0x1ac1 m_isAdsTransition
DT_BaseViewModel!0x1ac4 m_hWeapon
DT_BaseViewModel!0x1ac8 m_tracerAttachments
DT_BaseViewModel!0x1ad0 m_tracerAttachmentsScoped
```
</details>
<details>
<summary><code>class DT_BoneFollower</code></summary>

```
{
	m_cellX: Int,
	m_cellY: Int,
	m_cellZ: Int,
	m_localOrigin: Vector,
	m_nModelIndex: Int,
	m_networkedFlags: Int,
	m_localAngles: Vector,
	m_hOwnerEntity: Int,
	m_Collision: DT_CollisionProperty,
	m_CollisionGroup: Int,
	m_modelIndex: Int,
	m_boneIndex: Int,
}
```

### Offsets

```
DT_BoneFollower!0x004c m_cellX
DT_BoneFollower!0x0050 m_cellY
DT_BoneFollower!0x0054 m_cellZ
DT_BoneFollower!0x0058 m_localOrigin
DT_BoneFollower!0x0064 m_nModelIndex
DT_BoneFollower!0x0394 m_networkedFlags
DT_BoneFollower!0x0428 m_localAngles
DT_BoneFollower!0x043c m_hOwnerEntity
DT_BoneFollower!0x0458 m_Collision
DT_BoneFollower!0x04d8 m_CollisionGroup
DT_BoneFollower!0x0a00 m_modelIndex
DT_BoneFollower!0x0a04 m_boneIndex
```
</details>
<details>
<summary><code>class DT_BreakableSurface extends DT_BaseEntity</code></summary>

```
{
	m_nNumWide: Int,
	m_nNumHigh: Int,
	m_flPanelWidth: Float,
	m_flPanelHeight: Float,
	m_vNormal: Vector,
	m_vUp: Vector,
	m_vCorner: Vector,
	m_bIsBroken: Int,
	m_nSurfaceType: Int,
	m_RawPanelBitVec: DataTable,
}
```

### Offsets

```
DT_BreakableSurface!0x0a08 m_nNumWide
DT_BreakableSurface!0x0a0c m_nNumHigh
DT_BreakableSurface!0x0a10 m_flPanelWidth
DT_BreakableSurface!0x0a14 m_flPanelHeight
DT_BreakableSurface!0x0a18 m_vNormal
DT_BreakableSurface!0x0a24 m_vUp
DT_BreakableSurface!0x0a3c m_vCorner
DT_BreakableSurface!0x0a48 m_bIsBroken
DT_BreakableSurface!0x0a4c m_nSurfaceType
DT_BreakableSurface!0x0a88 m_RawPanelBitVec
```
</details>
<details>
<summary><code>class DT_CPropDoor</code></summary>

```
{
	HighlightSettings: DT_HighlightSettings,
	m_fEffects: Int,
	m_usableType: Int,
	m_cellX: Int,
	m_cellY: Int,
	m_cellZ: Int,
	m_localOrigin: Vector,
	m_nModelIndex: Int,
	m_networkedFlags: Int,
	m_localAngles: Vector,
	m_nSkin: Int,
	m_skinMod: Int,
	m_closedAngle: Float,
	m_angle: Float,
	m_startAngle: Float,
	m_startAngleVel: Float,
	m_startMoveTime: Time,
	m_isLocked: Int,
	m_oppositeDoor: Int,
	m_interactingPlayer: Int,
	m_interactingPlayerWantsOpen: Int,
}
```

### Offsets

```
DT_CPropDoor!0x0000 HighlightSettings
DT_CPropDoor!0x0044 m_fEffects
DT_CPropDoor!0x0048 m_usableType
DT_CPropDoor!0x004c m_cellX
DT_CPropDoor!0x0050 m_cellY
DT_CPropDoor!0x0054 m_cellZ
DT_CPropDoor!0x0058 m_localOrigin
DT_CPropDoor!0x0064 m_nModelIndex
DT_CPropDoor!0x0394 m_networkedFlags
DT_CPropDoor!0x0428 m_localAngles
DT_CPropDoor!0x0e48 m_nSkin
DT_CPropDoor!0x0e4c m_skinMod
DT_CPropDoor!0x1370 m_closedAngle
DT_CPropDoor!0x1374 m_angle
DT_CPropDoor!0x1378 m_startAngle
DT_CPropDoor!0x137c m_startAngleVel
DT_CPropDoor!0x1380 m_startMoveTime
DT_CPropDoor!0x1384 m_isLocked
DT_CPropDoor!0x1388 m_oppositeDoor
DT_CPropDoor!0x13d8 m_interactingPlayer
DT_CPropDoor!0x13dc m_interactingPlayerWantsOpen
```
</details>
<details>
<summary><code>class DT_CascadeLight extends DT_BaseEntity</code></summary>

```
{
	m_shadowDirection: Vector,
	m_envLightShadowDirection: Vector,
	m_bEnabled: Int,
	m_bEnableShadows: Int,
	m_LightColor: Int,
	m_cloudMaskName: String,
	m_cloudOffset: Vector,
	m_cloudScale: Float,
}
```

### Offsets

```
DT_CascadeLight!0x0a00 m_shadowDirection
DT_CascadeLight!0x0a18 m_envLightShadowDirection
DT_CascadeLight!0x0a2c m_bEnabled
DT_CascadeLight!0x0a2d m_bEnableShadows
DT_CascadeLight!0x0a2f m_LightColor
DT_CascadeLight!0x0a33 m_cloudMaskName
DT_CascadeLight!0x0b38 m_cloudOffset
DT_CascadeLight!0x0b44 m_cloudScale
```
</details>
<details>
<summary><code>class DT_CollisionProperty</code></summary>

```
{
	m_vecMins: Vector,
	m_vecMaxs: Vector,
	m_usSolidFlags: Int,
	m_nSolidType: Int,
	m_triggerBloat: Int,
	m_collisionDetailLevel: Int,
	m_nSurroundType: Int,
	m_vecSpecifiedSurroundingMins: Vector,
	m_vecSpecifiedSurroundingMaxs: Vector,
}
```

### Offsets

```
DT_CollisionProperty!0x0010 m_vecMins
DT_CollisionProperty!0x001c m_vecMaxs
DT_CollisionProperty!0x0028 m_usSolidFlags
DT_CollisionProperty!0x002c m_nSolidType
DT_CollisionProperty!0x002d m_triggerBloat
DT_CollisionProperty!0x002e m_collisionDetailLevel
DT_CollisionProperty!0x003c m_nSurroundType
DT_CollisionProperty!0x0048 m_vecSpecifiedSurroundingMins
DT_CollisionProperty!0x0054 m_vecSpecifiedSurroundingMaxs
```
</details>
<details>
<summary><code>class DT_ColorCorrection extends DT_BaseEntity</code></summary>

```
{
	m_hOwnerEntity: Int,
	m_localOrigin: Vector,
	m_MinFalloff: Float,
	m_MaxFalloff: Float,
	m_flFadeInDuration: Float,
	m_flFadeOutDuration: Float,
	m_flMaxWeight: Float,
	m_flCurWeight: Float,
	m_netLookupFilename: String,
	m_bEnabled: Int,
	m_bMaster: Int,
	m_bClientSide: Int,
	m_bExclusive: Int,
}
```

### Offsets

```
DT_ColorCorrection!0x043c m_hOwnerEntity
DT_ColorCorrection!0x0a00 m_localOrigin
DT_ColorCorrection!0x0a0c m_MinFalloff
DT_ColorCorrection!0x0a10 m_MaxFalloff
DT_ColorCorrection!0x0a14 m_flFadeInDuration
DT_ColorCorrection!0x0a18 m_flFadeOutDuration
DT_ColorCorrection!0x0a1c m_flMaxWeight
DT_ColorCorrection!0x0a20 m_flCurWeight
DT_ColorCorrection!0x0a24 m_netLookupFilename
DT_ColorCorrection!0x0b28 m_bEnabled
DT_ColorCorrection!0x0b29 m_bMaster
DT_ColorCorrection!0x0b2a m_bClientSide
DT_ColorCorrection!0x0b2b m_bExclusive
```
</details>
<details>
<summary><code>class DT_CurrentData_LocalPlayer</code></summary>

```
{
	m_viewConeAngleMin: Vector,
	m_viewConeAngleMax: Vector,
	m_stepSmoothingOffset: Vector,
	m_duckTransitionRemainderMsec: Int,
	m_vecPunchBase_Angle: Vector,
	m_vecPunchBase_AngleVel: Vector,
	m_vecPunchWeapon_Angle: Vector,
	m_vecPunchWeapon_AngleVel: Vector,
	m_pushedFixedPointOffset: DataTable,
	m_pushedFixedPointOffsetReplayCompensated: DataTable,
	m_localGravityRotation: Rotation,
}
```

### Offsets

```
DT_CurrentData_LocalPlayer!0x0000 m_viewConeAngleMin
DT_CurrentData_LocalPlayer!0x000c m_viewConeAngleMax
DT_CurrentData_LocalPlayer!0x0018 m_stepSmoothingOffset
DT_CurrentData_LocalPlayer!0x0024 m_duckTransitionRemainderMsec
DT_CurrentData_LocalPlayer!0x0028 m_vecPunchBase_Angle
DT_CurrentData_LocalPlayer!0x0034 m_vecPunchBase_AngleVel
DT_CurrentData_LocalPlayer!0x0040 m_vecPunchWeapon_Angle
DT_CurrentData_LocalPlayer!0x004c m_vecPunchWeapon_AngleVel
DT_CurrentData_LocalPlayer!0x0058 m_pushedFixedPointOffset
DT_CurrentData_LocalPlayer!0x0064 m_pushedFixedPointOffsetReplayCompensated
DT_CurrentData_LocalPlayer!0x007c m_localGravityRotation
```
</details>
<details>
<summary><code>class DT_CurrentData_Player</code></summary>

```
{
	m_flHullHeight: Float,
	m_angEyeAngles.x: Float,
	m_angEyeAngles.y: Float,
	m_traversalAnimProgress: Float,
	m_sprintTiltFrac: Float,
	m_ammoPoolCount: DataTable,
}
```

### Offsets

```
DT_CurrentData_Player!0x0014 m_flHullHeight
DT_CurrentData_Player!0x0018 m_angEyeAngles.x
DT_CurrentData_Player!0x001c m_angEyeAngles.y
DT_CurrentData_Player!0x0024 m_traversalAnimProgress
DT_CurrentData_Player!0x0028 m_sprintTiltFrac
DT_CurrentData_Player!0x002c m_ammoPoolCount
```
</details>
<details>
<summary><code>class DT_DoorMover</code></summary>

```
{
	moveparent: Int,
	m_parentAttachmentType: Int,
	m_fEffects: Int,
	m_usableType: Int,
	m_cellX: Int,
	m_cellY: Int,
	m_cellZ: Int,
	m_localOrigin: Vector,
	m_nModelIndex: Int,
	m_vecAngVelocity: Vector,
	m_networkedFlags: Int,
	m_vecVelocity: Vector,
	m_localAngles: Vector,
	m_Collision: DT_CollisionProperty,
	m_CollisionGroup: Int,
	m_iSignifierName: String,
	m_scriptNameIndex: Int,
	m_holdUsePrompt: String,
	m_pressUsePrompt: String,
	m_parentAttachmentIndex: Int,
	m_parentAttachmentHitbox: Int,
	m_parentAttachmentModel: Int,
	m_fadeDist: Float,
	m_usablePriority: Int,
	m_usableDistanceOverride: Float,
	m_usableFOV: Float,
	m_usePromptSize: Float,
	m_doorFlags: Int,
}
```

### Offsets

```
DT_DoorMover!0x001c moveparent
DT_DoorMover!0x0020 m_parentAttachmentType
DT_DoorMover!0x0044 m_fEffects
DT_DoorMover!0x0048 m_usableType
DT_DoorMover!0x004c m_cellX
DT_DoorMover!0x0050 m_cellY
DT_DoorMover!0x0054 m_cellZ
DT_DoorMover!0x0058 m_localOrigin
DT_DoorMover!0x0064 m_nModelIndex
DT_DoorMover!0x0128 m_vecAngVelocity
DT_DoorMover!0x0394 m_networkedFlags
DT_DoorMover!0x041c m_vecVelocity
DT_DoorMover!0x0428 m_localAngles
DT_DoorMover!0x0458 m_Collision
DT_DoorMover!0x04d8 m_CollisionGroup
DT_DoorMover!0x0518 m_iSignifierName
DT_DoorMover!0x0628 m_scriptNameIndex
DT_DoorMover!0x06b0 m_holdUsePrompt
DT_DoorMover!0x06b8 m_pressUsePrompt
DT_DoorMover!0x07f0 m_parentAttachmentIndex
DT_DoorMover!0x07f4 m_parentAttachmentHitbox
DT_DoorMover!0x07f8 m_parentAttachmentModel
DT_DoorMover!0x0804 m_fadeDist
DT_DoorMover!0x08c8 m_usablePriority
DT_DoorMover!0x08cc m_usableDistanceOverride
DT_DoorMover!0x08d0 m_usableFOV
DT_DoorMover!0x08d4 m_usePromptSize
DT_DoorMover!0x1540 m_doorFlags
```
</details>
<details>
<summary><code>class DT_DynamicLight extends DT_BaseEntity</code></summary>

```
{
	m_Flags: Int,
	m_LightStyle: Int,
	m_Radius: Float,
	m_Exponent: Int,
	m_InnerAngle: Float,
	m_OuterAngle: Float,
	m_SpotRadius: Float,
}
```

### Offsets

```
DT_DynamicLight!0x0a00 m_Flags
DT_DynamicLight!0x0a01 m_LightStyle
DT_DynamicLight!0x0a04 m_Radius
DT_DynamicLight!0x0a08 m_Exponent
DT_DynamicLight!0x0a0c m_InnerAngle
DT_DynamicLight!0x0a10 m_OuterAngle
DT_DynamicLight!0x0a14 m_SpotRadius
```
</details>
<details>
<summary><code>class DT_DynamicProp extends DT_BreakableProp</code></summary>

```
{
	m_iTeamNum: Int,
	m_lifeState: Int,
	m_bUseHitboxesForRenderBox: Int,
	m_bAnimateInStaticShadow: Int,
	m_wantsScopeHighlight: Int,
}
```

### Offsets

```
DT_DynamicProp!0x03f0 m_iTeamNum
DT_DynamicProp!0x0730 m_lifeState
DT_DynamicProp!0x1301 m_bUseHitboxesForRenderBox
DT_DynamicProp!0x1302 m_bAnimateInStaticShadow
DT_DynamicProp!0x1303 m_wantsScopeHighlight
```
</details>
<details>
<summary><code>class DT_DynamicPropLightweight</code></summary>

```
{
	moveparent: Int,
	m_parentAttachmentType: Int,
	m_parentAttachmentIndex: Int,
	m_fEffects: Int,
	m_cellX: Int,
	m_cellY: Int,
	m_cellZ: Int,
	m_localOrigin: Vector,
	m_nModelIndex: Int,
	m_networkedFlags: Int,
	m_visibilityFlags: Int,
	m_localAngles: Vector,
	m_Collision: DT_CollisionProperty,
	m_CollisionGroup: Int,
	m_parentAttachmentModel: Int,
	m_fadeDist: Float,
	m_nSkin: Int,
	m_skinMod: Int,
}
```

### Offsets

```
DT_DynamicPropLightweight!0x001c moveparent
DT_DynamicPropLightweight!0x0020 m_parentAttachmentType
DT_DynamicPropLightweight!0x0024 m_parentAttachmentIndex
DT_DynamicPropLightweight!0x0044 m_fEffects
DT_DynamicPropLightweight!0x004c m_cellX
DT_DynamicPropLightweight!0x0050 m_cellY
DT_DynamicPropLightweight!0x0054 m_cellZ
DT_DynamicPropLightweight!0x0058 m_localOrigin
DT_DynamicPropLightweight!0x0064 m_nModelIndex
DT_DynamicPropLightweight!0x0394 m_networkedFlags
DT_DynamicPropLightweight!0x03e8 m_visibilityFlags
DT_DynamicPropLightweight!0x0428 m_localAngles
DT_DynamicPropLightweight!0x0458 m_Collision
DT_DynamicPropLightweight!0x04d8 m_CollisionGroup
DT_DynamicPropLightweight!0x07f8 m_parentAttachmentModel
DT_DynamicPropLightweight!0x0804 m_fadeDist
DT_DynamicPropLightweight!0x0e48 m_nSkin
DT_DynamicPropLightweight!0x0e4c m_skinMod
```
</details>
<details>
<summary><code>class DT_EffectData</code></summary>

```
{
	m_vOrigin.x: Float,
	m_vOrigin.y: Float,
	m_vOrigin.z: Float,
	m_vStart.x: Float,
	m_vStart.y: Float,
	m_vStart.z: Float,
	m_vNormal: Vector,
	m_vAngles: Vector,
	m_effectFlags: Int,
	m_effectEntHandle: Int,
	m_otherEntHandle: Int,
	m_flScale: Float,
	m_flMagnitude: Float,
	m_flRadius: Float,
	m_nAttachmentIndex: Int,
	m_attachmentIndexForViewmodel: Int,
	m_nSurfaceProp: Int,
	m_nDamageType: Int,
	m_nOtherEntIndex: Int,
	m_sharedInt32_A: Int,
	m_sharedInt32_B: Int,
	m_iImpactEffectTableIndex: Int,
	m_nColor: Int,
	m_persistentWeaponEffect: Int,
	m_iEffectName: Int,
}
```

### Offsets

```
DT_EffectData!0x0000 m_vOrigin.x
DT_EffectData!0x0004 m_vOrigin.y
DT_EffectData!0x0008 m_vOrigin.z
DT_EffectData!0x000c m_vStart.x
DT_EffectData!0x0010 m_vStart.y
DT_EffectData!0x0014 m_vStart.z
DT_EffectData!0x0018 m_vNormal
DT_EffectData!0x0024 m_vAngles
DT_EffectData!0x0030 m_effectFlags
DT_EffectData!0x0050 m_effectEntHandle
DT_EffectData!0x0054 m_otherEntHandle
DT_EffectData!0x0058 m_flScale
DT_EffectData!0x005c m_flMagnitude
DT_EffectData!0x0060 m_flRadius
DT_EffectData!0x0064 m_nAttachmentIndex
DT_EffectData!0x0068 m_attachmentIndexForViewmodel
DT_EffectData!0x006c m_nSurfaceProp
DT_EffectData!0x0070 m_nDamageType
DT_EffectData!0x0074 m_nOtherEntIndex
DT_EffectData!0x007c m_sharedInt32_A
DT_EffectData!0x0080 m_sharedInt32_B
DT_EffectData!0x0084 m_iImpactEffectTableIndex
DT_EffectData!0x0088 m_nColor
DT_EffectData!0x009c m_persistentWeaponEffect
DT_EffectData!0x00a0 m_iEffectName
```
</details>
<details>
<summary><code>class DT_EntityDissolve extends DT_BaseEntity</code></summary>

```
{
	m_flStartTime: Time,
	m_flFadeOutStart: Float,
	m_flFadeOutLength: Float,
	m_flFadeOutModelStart: Float,
	m_flFadeOutModelLength: Float,
	m_flFadeInStart: Float,
	m_flFadeInLength: Float,
	m_nDissolveType: Int,
	fadeColorR: Int,
	fadeColorG: Int,
	fadeColorB: Int,
	m_isLethal: Int,
	m_vDissolverOrigin: Vector,
	m_nMagnitude: Int,
}
```

### Offsets

```
DT_EntityDissolve!0x0a08 m_flStartTime
DT_EntityDissolve!0x0a0c m_flFadeOutStart
DT_EntityDissolve!0x0a10 m_flFadeOutLength
DT_EntityDissolve!0x0a14 m_flFadeOutModelStart
DT_EntityDissolve!0x0a18 m_flFadeOutModelLength
DT_EntityDissolve!0x0a1c m_flFadeInStart
DT_EntityDissolve!0x0a20 m_flFadeInLength
DT_EntityDissolve!0x0a24 m_nDissolveType
DT_EntityDissolve!0x0a2c fadeColorR
DT_EntityDissolve!0x0a30 fadeColorG
DT_EntityDissolve!0x0a34 fadeColorB
DT_EntityDissolve!0x0a38 m_isLethal
DT_EntityDissolve!0x0a3c m_vDissolverOrigin
DT_EntityDissolve!0x0a48 m_nMagnitude
```
</details>
<details>
<summary><code>class DT_EntityLinkPage</code></summary>

```
{
	pageIndex: Int,
	next: DataTable,
	entity: DataTable,
}
```

### Offsets

```
DT_EntityLinkPage!0x0a00 pageIndex
DT_EntityLinkPage!0x0a04 next
DT_EntityLinkPage!0x0e04 entity
```
</details>
<details>
<summary><code>class DT_EnvWindShared</code></summary>

```
{
	m_flStartTime: Time,
	m_iWindSeed: Int,
	m_iMinWind: Int,
	m_iMaxWind: Int,
	m_iMinGust: Int,
	m_iMaxGust: Int,
	m_flMinGustDelay: Float,
	m_flMaxGustDelay: Float,
	m_flGustDuration: Float,
	m_iGustDirChange: Int,
	m_iInitialWindDir: Int,
	m_flInitialWindSpeed: Float,
}
```

### Offsets

```
DT_EnvWindShared!0x0008 m_flStartTime
DT_EnvWindShared!0x000c m_iWindSeed
DT_EnvWindShared!0x0010 m_iMinWind
DT_EnvWindShared!0x0014 m_iMaxWind
DT_EnvWindShared!0x001c m_iMinGust
DT_EnvWindShared!0x0020 m_iMaxGust
DT_EnvWindShared!0x0024 m_flMinGustDelay
DT_EnvWindShared!0x0028 m_flMaxGustDelay
DT_EnvWindShared!0x002c m_flGustDuration
DT_EnvWindShared!0x0030 m_iGustDirChange
DT_EnvWindShared!0x0070 m_iInitialWindDir
DT_EnvWindShared!0x0074 m_flInitialWindSpeed
```
</details>
<details>
<summary><code>class DT_FuncBrushLightweight</code></summary>

```
{
	moveparent: Int,
	m_parentAttachmentType: Int,
	m_parentAttachmentIndex: Int,
	m_cellX: Int,
	m_cellY: Int,
	m_cellZ: Int,
	m_localOrigin: Vector,
	m_nModelIndex: Int,
	m_networkedFlags: Int,
	m_visibilityFlags: Int,
	m_localAngles: Vector,
	m_Collision: DT_CollisionProperty,
	m_CollisionGroup: Int,
	m_parentAttachmentHitbox: Int,
	m_parentAttachmentModel: Int,
}
```

### Offsets

```
DT_FuncBrushLightweight!0x001c moveparent
DT_FuncBrushLightweight!0x0020 m_parentAttachmentType
DT_FuncBrushLightweight!0x0024 m_parentAttachmentIndex
DT_FuncBrushLightweight!0x004c m_cellX
DT_FuncBrushLightweight!0x0050 m_cellY
DT_FuncBrushLightweight!0x0054 m_cellZ
DT_FuncBrushLightweight!0x0058 m_localOrigin
DT_FuncBrushLightweight!0x0064 m_nModelIndex
DT_FuncBrushLightweight!0x0394 m_networkedFlags
DT_FuncBrushLightweight!0x03e8 m_visibilityFlags
DT_FuncBrushLightweight!0x0428 m_localAngles
DT_FuncBrushLightweight!0x0458 m_Collision
DT_FuncBrushLightweight!0x04d8 m_CollisionGroup
DT_FuncBrushLightweight!0x07f4 m_parentAttachmentHitbox
DT_FuncBrushLightweight!0x07f8 m_parentAttachmentModel
```
</details>
<details>
<summary><code>class DT_GlobalNonRewinding</code></summary>

```
{
	m_playerObserver: DataTable,
}
```

### Offsets

```
DT_GlobalNonRewinding!0x0a00 m_playerObserver
```
</details>
<details>
<summary><code>class DT_GrappleData</code></summary>

```
{
	m_grapplePoints: Array,
	m_grappleVel: Vector,
	m_grapplePoints[0]: Vector,
	m_grapplePointCount: Int,
	m_grappleAttached: Int,
	m_grapplePulling: Int,
	m_grappleSwinging: Int,
	m_grappleRetracting: Int,
	m_grappleForcedRetracting: Int,
	m_grappleGracePeriodFinished: Int,
	m_grappleUsedPower: Float,
	m_grappleActivateTime: Time,
	m_grapplePullTime: Time,
	m_grappleAttachTime: Time,
	m_grappleDetachTime: Time,
	m_grappleMeleeTarget: Int,
	m_grappleAutoAimTarget: Int,
	m_grappleHasGoodVelocity: Int,
	m_grappleLastGoodVelocityTime: Time,
	m_grappleSwingDetachLowSpeed: Float,
	m_grappleSwingHoldTime: Time,
}
```

### Offsets

```
DT_GrappleData!0x0000 m_grapplePoints
DT_GrappleData!0x0008 m_grappleVel
DT_GrappleData!0x0014 m_grapplePoints[0]
DT_GrappleData!0x0044 m_grapplePointCount
DT_GrappleData!0x0048 m_grappleAttached
DT_GrappleData!0x0049 m_grapplePulling
DT_GrappleData!0x004a m_grappleSwinging
DT_GrappleData!0x004b m_grappleRetracting
DT_GrappleData!0x004c m_grappleForcedRetracting
DT_GrappleData!0x004d m_grappleGracePeriodFinished
DT_GrappleData!0x0050 m_grappleUsedPower
DT_GrappleData!0x0054 m_grappleActivateTime
DT_GrappleData!0x0058 m_grapplePullTime
DT_GrappleData!0x005c m_grappleAttachTime
DT_GrappleData!0x0060 m_grappleDetachTime
DT_GrappleData!0x0064 m_grappleMeleeTarget
DT_GrappleData!0x0068 m_grappleAutoAimTarget
DT_GrappleData!0x006c m_grappleHasGoodVelocity
DT_GrappleData!0x0070 m_grappleLastGoodVelocityTime
DT_GrappleData!0x0074 m_grappleSwingDetachLowSpeed
DT_GrappleData!0x0078 m_grappleSwingHoldTime
```
</details>
<details>
<summary><code>class DT_GrappleHook</code></summary>

```
{
	moveparent: Int,
	m_parentAttachmentType: Int,
	m_parentAttachmentIndex: Int,
	m_cellX: Int,
	m_cellY: Int,
	m_cellZ: Int,
	m_localOrigin: Vector,
	m_nModelIndex: Int,
	m_visibilityFlags: Int,
	m_localAngles: Vector,
	m_hOwnerEntity: Int,
	m_parentAttachmentHitbox: Int,
	m_realmsBitMask: BitMask,
	m_grappleZipline: Int,
}
```

### Offsets

```
DT_GrappleHook!0x001c moveparent
DT_GrappleHook!0x0020 m_parentAttachmentType
DT_GrappleHook!0x0024 m_parentAttachmentIndex
DT_GrappleHook!0x004c m_cellX
DT_GrappleHook!0x0050 m_cellY
DT_GrappleHook!0x0054 m_cellZ
DT_GrappleHook!0x0058 m_localOrigin
DT_GrappleHook!0x0064 m_nModelIndex
DT_GrappleHook!0x03e8 m_visibilityFlags
DT_GrappleHook!0x0428 m_localAngles
DT_GrappleHook!0x043c m_hOwnerEntity
DT_GrappleHook!0x07f4 m_parentAttachmentHitbox
DT_GrappleHook!0x09e8 m_realmsBitMask
DT_GrappleHook!0x1300 m_grappleZipline
```
</details>
<details>
<summary><code>class DT_HardPointEntity</code></summary>

```
{
	m_localOrigin: Vector,
	m_iTeamNum: Int,
	m_minimapData: DT_MinimapBaseEntityData,
	m_state: Int,
	m_estimatedCaptureTime: Float,
	m_progressRefPoint: Float,
	m_teamMilitiaAICount: Int,
	m_teamIMCAICount: Int,
	m_teamMilitiaPlayerCount: Int,
	m_teamIMCPlayerCount: Int,
	m_teamMilitiaPlayerTitanCount: Int,
	m_teamIMCPlayerTitanCount: Int,
	m_hardpointID: Int,
	m_terminal: Int,
}
```

### Offsets

```
DT_HardPointEntity!0x0004 m_localOrigin
DT_HardPointEntity!0x03f0 m_iTeamNum
DT_HardPointEntity!0x0908 m_minimapData
DT_HardPointEntity!0x0a04 m_state
DT_HardPointEntity!0x0a08 m_estimatedCaptureTime
DT_HardPointEntity!0x0a0c m_progressRefPoint
DT_HardPointEntity!0x0a10 m_teamMilitiaAICount
DT_HardPointEntity!0x0a14 m_teamIMCAICount
DT_HardPointEntity!0x0a18 m_teamMilitiaPlayerCount
DT_HardPointEntity!0x0a1c m_teamIMCPlayerCount
DT_HardPointEntity!0x0a20 m_teamMilitiaPlayerTitanCount
DT_HardPointEntity!0x0a24 m_teamIMCPlayerTitanCount
DT_HardPointEntity!0x0a28 m_hardpointID
DT_HardPointEntity!0x0a30 m_terminal
```
</details>
<details>
<summary><code>class DT_HighlightSettings</code></summary>

```
{
	m_highlightParams: DataTable,
	m_highlightFunctionBits: DataTable,
	m_highlightServerFadeBases: DataTable,
	m_highlightServerFadeStartTimes: DataTable,
	m_highlightServerFadeEndTimes: DataTable,
	m_highlightServerContextID: Int,
	m_highlightTeamBits: Int,
}
```

### Offsets

```
DT_HighlightSettings!0x01b8 m_highlightParams
DT_HighlightSettings!0x0278 m_highlightFunctionBits
DT_HighlightSettings!0x02b8 m_highlightServerFadeBases
DT_HighlightSettings!0x02c0 m_highlightServerFadeStartTimes
DT_HighlightSettings!0x02c8 m_highlightServerFadeEndTimes
DT_HighlightSettings!0x0308 m_highlightServerContextID
DT_HighlightSettings!0x0314 m_highlightTeamBits
```
</details>
<details>
<summary><code>class DT_ImportantOnEntSound extends DT_BaseEntity</code></summary>

```
{
	m_networkTableSoundID: Int,
	m_hAttachedToEntity: Int,
	m_beginTime: Time,
	m_hSuppressedClient: Int,
	m_milesSignal: Int,
}
```

### Offsets

```
DT_ImportantOnEntSound!0x0a00 m_networkTableSoundID
DT_ImportantOnEntSound!0x0a04 m_hAttachedToEntity
DT_ImportantOnEntSound!0x0a08 m_beginTime
DT_ImportantOnEntSound!0x0a0c m_hSuppressedClient
DT_ImportantOnEntSound!0x0a10 m_milesSignal
```
</details>
<details>
<summary><code>class DT_InfoPlacementHelper</code></summary>

```
{
	m_localOrigin: Vector,
	moveparent: Int,
	m_parentAttachmentType: Int,
	m_parentAttachmentIndex: Int,
	m_localAngles: Vector,
	m_parentAttachmentHitbox: Int,
	m_parentAttachmentModel: Int,
}
```

### Offsets

```
DT_InfoPlacementHelper!0x0004 m_localOrigin
DT_InfoPlacementHelper!0x001c moveparent
DT_InfoPlacementHelper!0x0020 m_parentAttachmentType
DT_InfoPlacementHelper!0x0024 m_parentAttachmentIndex
DT_InfoPlacementHelper!0x0428 m_localAngles
DT_InfoPlacementHelper!0x07f4 m_parentAttachmentHitbox
DT_InfoPlacementHelper!0x07f8 m_parentAttachmentModel
```
</details>
<details>
<summary><code>class DT_InfoTarget</code></summary>

```
{
	moveparent: Int,
	m_parentAttachmentType: Int,
	m_parentAttachmentIndex: Int,
	m_cellX: Int,
	m_cellY: Int,
	m_cellZ: Int,
	m_localOrigin: Vector,
	m_bIsSoundCodeControllerValueSet: Int,
	m_flSoundCodeControllerValue: Float,
	m_iTeamNum: Int,
	m_localAngles: Vector,
	m_hOwnerEntity: Int,
	m_iSignifierName: String,
	m_iName: String,
	m_scriptNameIndex: Int,
	m_instanceNameIndex: Int,
	m_parentAttachmentHitbox: Int,
	m_parentAttachmentModel: Int,
	m_firstChildEntityLink: Int,
	m_firstParentEntityLink: Int,
}
```

### Offsets

```
DT_InfoTarget!0x001c moveparent
DT_InfoTarget!0x0020 m_parentAttachmentType
DT_InfoTarget!0x0024 m_parentAttachmentIndex
DT_InfoTarget!0x004c m_cellX
DT_InfoTarget!0x0050 m_cellY
DT_InfoTarget!0x0054 m_cellZ
DT_InfoTarget!0x0058 m_localOrigin
DT_InfoTarget!0x038c m_bIsSoundCodeControllerValueSet
DT_InfoTarget!0x0390 m_flSoundCodeControllerValue
DT_InfoTarget!0x03f0 m_iTeamNum
DT_InfoTarget!0x0428 m_localAngles
DT_InfoTarget!0x043c m_hOwnerEntity
DT_InfoTarget!0x0518 m_iSignifierName
DT_InfoTarget!0x0521 m_iName
DT_InfoTarget!0x0628 m_scriptNameIndex
DT_InfoTarget!0x062c m_instanceNameIndex
DT_InfoTarget!0x07f4 m_parentAttachmentHitbox
DT_InfoTarget!0x07f8 m_parentAttachmentModel
DT_InfoTarget!0x09e0 m_firstChildEntityLink
DT_InfoTarget!0x09e4 m_firstParentEntityLink
```
</details>
<details>
<summary><code>class DT_Local</code></summary>

```
{
	m_airMoveBlockPlanes: Array,
	m_iHideHUD: Int,
	m_superJumpsUsed: Int,
	m_jumpedOffRodeo: Int,
	m_jumpPressTime: Time,
	m_jetpackActivateTime: Time,
	m_jetpackDeactivateTime: Time,
	m_flSuitPower: Float,
	m_flSuitJumpPower: Float,
	m_flSuitGrapplePower: Float,
	m_flFallVelocity: Float,
	m_flStepSize: Float,
	m_airSlowMoFrac: Float,
	predictableFlags: Int,
	m_bitsActiveDevices: Int,
	m_forceStance: Int,
	m_duckToggleOn: Int,
	m_bDrawViewmodel: Int,
	m_bAllowAutoMovement: Int,
	m_accelScale: Float,
	m_powerRegenRateScale: Float,
	m_dodgePowerDelayScale: Float,
	m_hSkyCamera: Int,
	m_skybox3d.scale: Int,
	m_skybox3d.cellNum: Int,
	m_skybox3d.useWorldFog: Int,
	m_skybox3d.fog.botAlt: Float,
	m_skybox3d.fog.topAlt: Float,
	m_skybox3d.fog.halfDistBot: Float,
	m_skybox3d.fog.halfDistTop: Float,
	m_skybox3d.fog.distColorStr: Float,
	m_skybox3d.fog.dirColorStr: Float,
	m_skybox3d.fog.distOffset: Float,
	m_skybox3d.fog.densityScale: Float,
	m_skybox3d.fog.halfAngleDeg: Float,
	m_skybox3d.fog.HDRColorScale: Float,
	m_skybox3d.fog.distColor: Int,
	m_skybox3d.fog.dirColor: Int,
	m_skybox3d.fog.direction: Vector,
	m_skybox3d.fog.enable: Int,
	m_audio.localSound[0]: Vector,
	m_audio.localSound[1]: Vector,
	m_audio.localSound[2]: Vector,
	m_audio.localSound[3]: Vector,
	m_audio.localSound[4]: Vector,
	m_audio.localSound[5]: Vector,
	m_audio.localSound[6]: Vector,
	m_audio.localSound[7]: Vector,
	m_audio.soundscapeIndex: Int,
	m_audio.localBits: Int,
	m_audio.entIndex: Int,
	m_animNearZ: Float,
	lastAttacker: Int,
	attackedCount: Int,
	m_airMoveBlockPlanes[0]: Vector,
	m_airMoveBlockPlaneTime: Time,
	m_airMoveBlockPlaneCount: Int,
	m_queuedMeleePressTime: Time,
	m_queuedGrappleMeleeTime: Time,
	m_disableMeleeUntilRelease: Int,
	m_meleePressTime: Time,
	m_meleeDisabledCounter: Int,
	m_meleeInputIndex: Int,
	m_trackedChildProjectileCount: Int,
	m_oneHandedWeaponUsage: Int,
	m_flCockpitEntryTime: Time,
	m_ejectStartTime: Time,
	m_disembarkStartTime: Time,
	m_hotDropImpactTime: Time,
	m_outOfBoundsDeadTime: Time,
	m_objectiveIndex: Int,
	m_objectiveEntity: Int,
	m_objectiveEndTime: Time,
	m_cinematicEventFlags: Int,
	m_forcedDialogueOnly: Int,
	m_titanBuildTime: Time,
	m_titanBubbleShieldTime: Time,
	m_titanEmbarkEnabled: Int,
	m_titanDisembarkEnabled: Int,
	m_voicePackIndex: Int,
	m_playerAnimStationaryGoalFeetYaw: Float,
	m_playerAnimJumping: Int,
	m_playerAnimJumpStartTime: Time,
	m_playerAnimFirstJumpFrame: Int,
	m_playerAnimDodging: Int,
	m_playerAnimJumpActivity: Int,
	m_playerAnimLanding: Int,
	m_playerAnimShouldLand: Int,
	m_playerAnimLandStartTime: Time,
	m_playerAnimInAirWalk: Int,
	m_playerAnimPrevFrameSequenceMotionYaw: Float,
	m_playerAnimMeleeParity: Int,
	m_playerAnimMeleeStartTime: Time,
	m_playerLocalGravityBlendStartRotation: Rotation,
	m_playerLocalGravityBlendEndRotation: Rotation,
	m_playerLocalGravityBlendEndDirection: Vector,
	m_playerLocalGravityBlendStartTime: Time,
	m_playerLocalGravityBlendEndTime: Time,
	m_playerLocalGravityBlendStrength: Float,
	m_playerLocalGravityStrength: Float,
	m_playerLocalGravityType: Int,
	m_playerLocalGravityPoint: Vector,
	m_playerLocalGravityLineStart: Vector,
	m_playerLocalGravityLineEnd: Vector,
	m_playerLocalGravityEntity: Int,
	m_playerLocalGravityLineStartEntity: Int,
	m_playerLocalGravityLineEndEntity: Int,
	m_playerFloatLookStartTime: Time,
	m_playerFloatLookEndTime: Time,
	m_wallrunLatestFloorHeight: Float,
	m_groundNormal: Vector,
	m_continuousUseBlocked: Int,
	m_useEnt: Int,
}
```

### Offsets

```
DT_Local!0x0000 m_airMoveBlockPlanes
DT_Local!0x0010 m_iHideHUD
DT_Local!0x0014 m_superJumpsUsed
DT_Local!0x0018 m_jumpedOffRodeo
DT_Local!0x001c m_jumpPressTime
DT_Local!0x0020 m_jetpackActivateTime
DT_Local!0x0024 m_jetpackDeactivateTime
DT_Local!0x0028 m_flSuitPower
DT_Local!0x002c m_flSuitJumpPower
DT_Local!0x0030 m_flSuitGrapplePower
DT_Local!0x0034 m_flFallVelocity
DT_Local!0x0038 m_flStepSize
DT_Local!0x003c m_airSlowMoFrac
DT_Local!0x0040 predictableFlags
DT_Local!0x0044 m_bitsActiveDevices
DT_Local!0x0048 m_forceStance
DT_Local!0x004c m_duckToggleOn
DT_Local!0x004d m_bDrawViewmodel
DT_Local!0x004e m_bAllowAutoMovement
DT_Local!0x0050 m_accelScale
DT_Local!0x0054 m_powerRegenRateScale
DT_Local!0x0058 m_dodgePowerDelayScale
DT_Local!0x0070 m_hSkyCamera
DT_Local!0x0074 m_skybox3d.scale
DT_Local!0x0078 m_skybox3d.cellNum
DT_Local!0x007c m_skybox3d.useWorldFog
DT_Local!0x0080 m_skybox3d.fog.botAlt
DT_Local!0x0084 m_skybox3d.fog.topAlt
DT_Local!0x0088 m_skybox3d.fog.halfDistBot
DT_Local!0x008c m_skybox3d.fog.halfDistTop
DT_Local!0x0090 m_skybox3d.fog.distColorStr
DT_Local!0x0094 m_skybox3d.fog.dirColorStr
DT_Local!0x0098 m_skybox3d.fog.distOffset
DT_Local!0x009c m_skybox3d.fog.densityScale
DT_Local!0x00a0 m_skybox3d.fog.halfAngleDeg
DT_Local!0x00a4 m_skybox3d.fog.HDRColorScale
DT_Local!0x00a8 m_skybox3d.fog.distColor
DT_Local!0x00ac m_skybox3d.fog.dirColor
DT_Local!0x00b0 m_skybox3d.fog.direction
DT_Local!0x00c1 m_skybox3d.fog.enable
DT_Local!0x00c8 m_audio.localSound[0]
DT_Local!0x00d4 m_audio.localSound[1]
DT_Local!0x00e0 m_audio.localSound[2]
DT_Local!0x00ec m_audio.localSound[3]
DT_Local!0x00f8 m_audio.localSound[4]
DT_Local!0x0104 m_audio.localSound[5]
DT_Local!0x0110 m_audio.localSound[6]
DT_Local!0x011c m_audio.localSound[7]
DT_Local!0x0128 m_audio.soundscapeIndex
DT_Local!0x012c m_audio.localBits
DT_Local!0x0130 m_audio.entIndex
DT_Local!0x014c m_animNearZ
DT_Local!0x0150 lastAttacker
DT_Local!0x0154 attackedCount
DT_Local!0x0180 m_airMoveBlockPlanes[0]
DT_Local!0x0198 m_airMoveBlockPlaneTime
DT_Local!0x019c m_airMoveBlockPlaneCount
DT_Local!0x01a0 m_queuedMeleePressTime
DT_Local!0x01a4 m_queuedGrappleMeleeTime
DT_Local!0x01a9 m_disableMeleeUntilRelease
DT_Local!0x01ac m_meleePressTime
DT_Local!0x01b0 m_meleeDisabledCounter
DT_Local!0x01b4 m_meleeInputIndex
DT_Local!0x01b8 m_trackedChildProjectileCount
DT_Local!0x01bc m_oneHandedWeaponUsage
DT_Local!0x01c0 m_flCockpitEntryTime
DT_Local!0x01c4 m_ejectStartTime
DT_Local!0x01c8 m_disembarkStartTime
DT_Local!0x01cc m_hotDropImpactTime
DT_Local!0x01d0 m_outOfBoundsDeadTime
DT_Local!0x01d4 m_objectiveIndex
DT_Local!0x01d8 m_objectiveEntity
DT_Local!0x01dc m_objectiveEndTime
DT_Local!0x01e0 m_cinematicEventFlags
DT_Local!0x01e4 m_forcedDialogueOnly
DT_Local!0x01e8 m_titanBuildTime
DT_Local!0x01ec m_titanBubbleShieldTime
DT_Local!0x01f0 m_titanEmbarkEnabled
DT_Local!0x01f1 m_titanDisembarkEnabled
DT_Local!0x01f4 m_voicePackIndex
DT_Local!0x01f8 m_playerAnimStationaryGoalFeetYaw
DT_Local!0x01fc m_playerAnimJumping
DT_Local!0x0200 m_playerAnimJumpStartTime
DT_Local!0x0204 m_playerAnimFirstJumpFrame
DT_Local!0x0205 m_playerAnimDodging
DT_Local!0x0208 m_playerAnimJumpActivity
DT_Local!0x020c m_playerAnimLanding
DT_Local!0x020d m_playerAnimShouldLand
DT_Local!0x0210 m_playerAnimLandStartTime
DT_Local!0x0214 m_playerAnimInAirWalk
DT_Local!0x0218 m_playerAnimPrevFrameSequenceMotionYaw
DT_Local!0x021c m_playerAnimMeleeParity
DT_Local!0x0220 m_playerAnimMeleeStartTime
DT_Local!0x0254 m_playerLocalGravityBlendStartRotation
DT_Local!0x0264 m_playerLocalGravityBlendEndRotation
DT_Local!0x0274 m_playerLocalGravityBlendEndDirection
DT_Local!0x0280 m_playerLocalGravityBlendStartTime
DT_Local!0x0284 m_playerLocalGravityBlendEndTime
DT_Local!0x0288 m_playerLocalGravityBlendStrength
DT_Local!0x028c m_playerLocalGravityStrength
DT_Local!0x0290 m_playerLocalGravityType
DT_Local!0x0294 m_playerLocalGravityPoint
DT_Local!0x02a0 m_playerLocalGravityLineStart
DT_Local!0x02ac m_playerLocalGravityLineEnd
DT_Local!0x02b8 m_playerLocalGravityEntity
DT_Local!0x02bc m_playerLocalGravityLineStartEntity
DT_Local!0x02c0 m_playerLocalGravityLineEndEntity
DT_Local!0x02c4 m_playerFloatLookStartTime
DT_Local!0x02c8 m_playerFloatLookEndTime
DT_Local!0x02cc m_wallrunLatestFloorHeight
DT_Local!0x02d0 m_groundNormal
DT_Local!0x02dc m_continuousUseBlocked
DT_Local!0x02e0 m_useEnt
```
</details>
<details>
<summary><code>class DT_LocalPlayerExclusive</code></summary>

```
{
	NearbyPushers: DT_NearbyPushers,
	m_localOrigin: VectorXY,
	m_localOrigin.z: Float,
	m_vecAbsVelocity: Vector,
	m_vecBaseVelocity: Vector,
	m_vecVelocity.x: Float,
	m_vecVelocity.y: Float,
	m_vecVelocity.z: Float,
	m_flFriction: Float,
	m_tethers: DataTable,
	m_lastUCmdSimulationTicks: Cycle,
	m_lastUCmdSimulationRemainderTime: Float,
	m_Local: DT_Local,
	m_currentFrameLocalPlayer: DT_CurrentData_LocalPlayer,
	m_modInventory: DataTable,
	m_consumableInventory: DataTable,
	m_fStickySprintMinTime: Float,
	m_sprintStartedTime: Time,
	m_sprintStartedFrac: Float,
	m_sprintEndedTime: Time,
	m_sprintEndedFrac: Float,
	m_stickySprintStartTime: Time,
	m_upDirPredicted: Vector,
	m_lastWallRunStartPos: Vector,
	m_wallrunFrictionScale: Float,
	m_groundFrictionScale: Float,
	m_traversalBegin: Vector,
	m_traversalMid: Vector,
	m_traversalEnd: Vector,
	m_traversalMidFrac: Float,
	m_traversalForwardDir: Vector,
	m_traversalProgress: Float,
	m_traversalStartTime: Time,
	m_traversalHandAppearTime: Time,
	m_traversalReleaseTime: Time,
	m_traversalBlendOutStartTime: Time,
	m_traversalBlendOutStartOffset: Vector,
	m_wallDangleJumpOffTime: Time,
	m_wallDangleMayHangHere: Int,
	m_wallDangleForceFallOff: Int,
	m_wallDangleLastPushedForward: Int,
	m_wallDangleDisableWeapon: Int,
	m_slowMoEnabled: Int,
	m_sliding: Int,
	m_slideLongJumpAllowed: Int,
	m_bIsStickySprinting: Int,
	m_prevMoveYaw: Float,
	m_sprintTiltVel: Float,
	m_sprintTiltPoseParameter: Int,
	m_sprintFracPoseParameter: Int,
	m_ziplineAllowed: Int,
	m_lastZipline: Int,
	m_lastZiplineDetachTime: Time,
	m_zipline: DT_PlayerZipline,
	m_ziplineViewOffsetPosition: Vector,
	m_ziplineViewOffsetVelocity: Vector,
	m_ziplineGrenadeEntity: Int,
	m_highSpeedViewmodelAnims: Int,
	m_playAnimationType: Int,
	m_detachGrappleOnPlayAnimationEnd: Int,
	m_playAnimationNext: DataTable,
	m_playAnimationEntityBlocker: Int,
	m_playAnimationEntityBlockerDucking: Int,
	m_boosting: Int,
	m_activateBoost: Int,
	m_repeatedBoost: Int,
	m_boostMeter: Float,
	m_jetpack: Int,
	m_activateJetpack: Int,
	m_jetpackAfterburner: Int,
	m_gliding: Int,
	m_glideMeter: Float,
	m_glideRechargeDelayAccumulator: Float,
	m_hovering: Int,
	m_lastJumpHeight: Float,
	m_slipAirRestrictDirection: Vector,
	m_slipAirRestrictTime: Time,
	m_replayImportantSounds_networkTableSoundID: DataTable,
	m_replayImportantSounds_beginTime: DataTable,
	m_viewConeActive: Int,
	m_viewConeParented: Int,
	m_viewConeParity: Int,
	m_hConstraintEntity: Int,
	m_vecConstraintCenter: Vector,
	m_flConstraintRadius: Float,
	m_flConstraintWidth: Float,
	m_flConstraintSpeedFactor: Float,
	m_bConstraintPastRadius: Int,
	m_observerModeStaticPosition: Vector,
	m_observerModeStaticAngles: Vector,
	m_observerModeStaticFOVOverride: Float,
	m_lastKillTime: Time,
	m_wallRunStartTime: Time,
	m_wallRunClearTime: Time,
	m_dodging: Int,
	m_dodgingInAir: Int,
	m_airSpeed: Float,
	m_airAcceleration: Float,
	m_firstPersonProxy: Int,
	m_predictedFirstPersonProxy: Int,
	m_hardpointEntity: Int,
	m_petTitanMode: Int,
	m_hThirdPersonEnt: Int,
	m_thirdPersonShoulderView: Int,
	m_thirdPerson: DT_ThirdPersonView,
	m_viewConeLerpTime: Float,
	m_flLaggedMovementValue: Float,
	m_lastMoveInputTime: Time,
	m_ignoreEntityForMovementUntilNotTouching: Int,
	m_lungeTargetEntity: Int,
	m_isLungingToPosition: Int,
	m_lungeTargetPosition: Vector,
	m_lungeStartPositionOffset: Vector,
	m_lungeEndPositionOffset: Vector,
	m_lungeStartTime: Time,
	m_lungeEndTime: Time,
	m_lungeCanFly: Int,
	m_lungeLockPitch: Int,
	m_lungeStartPitch: Float,
	m_lungeSmoothTime: Float,
	m_lungeMaxTime: Float,
	m_lungeMaxEndSpeed: Float,
	m_nearbyPusherCount: Int,
	m_pushAwayFromTopAcceleration: Vector,
	m_minimapTargetZoomScale: Float,
	m_minimapTargetLerpTime: Time,
	m_playerScriptNetDataExclusive: Int,
	m_skydiveForwardPoseValueVelocity: Float,
	m_skydiveForwardPoseValueCurrent: Float,
	m_skydiveSidePoseValueVelocity: Float,
	m_skydiveSidePoseValueCurrent: Float,
	m_skydiveYawVelocity: Float,
	m_freefallStartTime: Time,
	m_freefallEndTime: Time,
	m_freefallAnticipateStartTime: Time,
	m_freefallAnticipateEndTime: Time,
	m_freefallDistanceToLand: Float,
	m_skydiveFreelookEnabled: Int,
	m_skydiveFreelookLockedAngle: Vector,
	m_skydiveFollowing: Int,
	m_skydiveUnfollowVelocity: Vector,
	m_skydiveIsNearLeviathan: Int,
	m_skydiveLeviathanHitPosition: Vector,
	m_skydiveLeviathanHitNormal: Vector,
	m_skydiveSlipVelocity: Vector,
	m_skydiveFromUpdraft: Int,
	m_twitchRewardBits: BitMask,
	m_playerKnockBacks: DataTable,
	m_updraftCount: Int,
	m_updraftStage: Int,
	m_updraftEnterTime: Time,
	m_updraftLeaveTime: Time,
	m_updraftMinShakeActivationHeight: Float,
	m_updraftMaxShakeActivationHeight: Float,
	m_updraftLiftActivationHeight: Float,
	m_updraftLiftSpeed: Float,
	m_updraftLiftAcceleration: Float,
	m_updraftLiftExitDuration: Float,
	m_updraftSlowTime: Time,
	m_updraftEndSkydiveAtPeak: Int,
	m_armsModelIndex: Int,
}
```

### Offsets

```
DT_LocalPlayerExclusive!0x0000 NearbyPushers
DT_LocalPlayerExclusive!0x0004 m_localOrigin
DT_LocalPlayerExclusive!0x000c m_localOrigin.z
DT_LocalPlayerExclusive!0x0140 m_vecAbsVelocity
DT_LocalPlayerExclusive!0x03d0 m_vecBaseVelocity
DT_LocalPlayerExclusive!0x041c m_vecVelocity.x
DT_LocalPlayerExclusive!0x0420 m_vecVelocity.y
DT_LocalPlayerExclusive!0x0424 m_vecVelocity.z
DT_LocalPlayerExclusive!0x0434 m_flFriction
DT_LocalPlayerExclusive!0x1824 m_tethers
DT_LocalPlayerExclusive!0x1910 m_lastUCmdSimulationTicks
DT_LocalPlayerExclusive!0x1914 m_lastUCmdSimulationRemainderTime
DT_LocalPlayerExclusive!0x1a30 m_Local
DT_LocalPlayerExclusive!0x1ee8 m_currentFrameLocalPlayer
DT_LocalPlayerExclusive!0x2264 m_modInventory
DT_LocalPlayerExclusive!0x22e4 m_consumableInventory
DT_LocalPlayerExclusive!0x25f8 m_fStickySprintMinTime
DT_LocalPlayerExclusive!0x2608 m_sprintStartedTime
DT_LocalPlayerExclusive!0x260c m_sprintStartedFrac
DT_LocalPlayerExclusive!0x2610 m_sprintEndedTime
DT_LocalPlayerExclusive!0x2614 m_sprintEndedFrac
DT_LocalPlayerExclusive!0x2618 m_stickySprintStartTime
DT_LocalPlayerExclusive!0x2678 m_upDirPredicted
DT_LocalPlayerExclusive!0x2684 m_lastWallRunStartPos
DT_LocalPlayerExclusive!0x26a8 m_wallrunFrictionScale
DT_LocalPlayerExclusive!0x26ac m_groundFrictionScale
DT_LocalPlayerExclusive!0x26f0 m_traversalBegin
DT_LocalPlayerExclusive!0x26fc m_traversalMid
DT_LocalPlayerExclusive!0x2708 m_traversalEnd
DT_LocalPlayerExclusive!0x2714 m_traversalMidFrac
DT_LocalPlayerExclusive!0x2718 m_traversalForwardDir
DT_LocalPlayerExclusive!0x2730 m_traversalProgress
DT_LocalPlayerExclusive!0x2734 m_traversalStartTime
DT_LocalPlayerExclusive!0x2738 m_traversalHandAppearTime
DT_LocalPlayerExclusive!0x273c m_traversalReleaseTime
DT_LocalPlayerExclusive!0x2740 m_traversalBlendOutStartTime
DT_LocalPlayerExclusive!0x2744 m_traversalBlendOutStartOffset
DT_LocalPlayerExclusive!0x275c m_wallDangleJumpOffTime
DT_LocalPlayerExclusive!0x2760 m_wallDangleMayHangHere
DT_LocalPlayerExclusive!0x2761 m_wallDangleForceFallOff
DT_LocalPlayerExclusive!0x2762 m_wallDangleLastPushedForward
DT_LocalPlayerExclusive!0x2764 m_wallDangleDisableWeapon
DT_LocalPlayerExclusive!0x2814 m_slowMoEnabled
DT_LocalPlayerExclusive!0x2815 m_sliding
DT_LocalPlayerExclusive!0x2816 m_slideLongJumpAllowed
DT_LocalPlayerExclusive!0x2824 m_bIsStickySprinting
DT_LocalPlayerExclusive!0x2828 m_prevMoveYaw
DT_LocalPlayerExclusive!0x282c m_sprintTiltVel
DT_LocalPlayerExclusive!0x2830 m_sprintTiltPoseParameter
DT_LocalPlayerExclusive!0x2834 m_sprintFracPoseParameter
DT_LocalPlayerExclusive!0x299c m_ziplineAllowed
DT_LocalPlayerExclusive!0x29a4 m_lastZipline
DT_LocalPlayerExclusive!0x29a8 m_lastZiplineDetachTime
DT_LocalPlayerExclusive!0x29b8 m_zipline
DT_LocalPlayerExclusive!0x2a28 m_ziplineViewOffsetPosition
DT_LocalPlayerExclusive!0x2a34 m_ziplineViewOffsetVelocity
DT_LocalPlayerExclusive!0x2a40 m_ziplineGrenadeEntity
DT_LocalPlayerExclusive!0x2a50 m_highSpeedViewmodelAnims
DT_LocalPlayerExclusive!0x2a54 m_playAnimationType
DT_LocalPlayerExclusive!0x2a58 m_detachGrappleOnPlayAnimationEnd
DT_LocalPlayerExclusive!0x2a5c m_playAnimationNext
DT_LocalPlayerExclusive!0x2a64 m_playAnimationEntityBlocker
DT_LocalPlayerExclusive!0x2a68 m_playAnimationEntityBlockerDucking
DT_LocalPlayerExclusive!0x2a70 m_boosting
DT_LocalPlayerExclusive!0x2a71 m_activateBoost
DT_LocalPlayerExclusive!0x2a72 m_repeatedBoost
DT_LocalPlayerExclusive!0x2a74 m_boostMeter
DT_LocalPlayerExclusive!0x2a78 m_jetpack
DT_LocalPlayerExclusive!0x2a79 m_activateJetpack
DT_LocalPlayerExclusive!0x2a7a m_jetpackAfterburner
DT_LocalPlayerExclusive!0x2a7b m_gliding
DT_LocalPlayerExclusive!0x2a7c m_glideMeter
DT_LocalPlayerExclusive!0x2a80 m_glideRechargeDelayAccumulator
DT_LocalPlayerExclusive!0x2a84 m_hovering
DT_LocalPlayerExclusive!0x2a88 m_lastJumpHeight
DT_LocalPlayerExclusive!0x2ad8 m_slipAirRestrictDirection
DT_LocalPlayerExclusive!0x2ae4 m_slipAirRestrictTime
DT_LocalPlayerExclusive!0x2c20 m_replayImportantSounds_networkTableSoundID
DT_LocalPlayerExclusive!0x2c30 m_replayImportantSounds_beginTime
DT_LocalPlayerExclusive!0x2c6d m_viewConeActive
DT_LocalPlayerExclusive!0x2c6e m_viewConeParented
DT_LocalPlayerExclusive!0x2c70 m_viewConeParity
DT_LocalPlayerExclusive!0x2ecc m_hConstraintEntity
DT_LocalPlayerExclusive!0x2ed0 m_vecConstraintCenter
DT_LocalPlayerExclusive!0x2edc m_flConstraintRadius
DT_LocalPlayerExclusive!0x2ee0 m_flConstraintWidth
DT_LocalPlayerExclusive!0x2ee4 m_flConstraintSpeedFactor
DT_LocalPlayerExclusive!0x2ee8 m_bConstraintPastRadius
DT_LocalPlayerExclusive!0x2f44 m_observerModeStaticPosition
DT_LocalPlayerExclusive!0x2f50 m_observerModeStaticAngles
DT_LocalPlayerExclusive!0x2f5c m_observerModeStaticFOVOverride
DT_LocalPlayerExclusive!0x2fc8 m_lastKillTime
DT_LocalPlayerExclusive!0x2fec m_wallRunStartTime
DT_LocalPlayerExclusive!0x2ff0 m_wallRunClearTime
DT_LocalPlayerExclusive!0x3004 m_dodging
DT_LocalPlayerExclusive!0x305e m_dodgingInAir
DT_LocalPlayerExclusive!0x3078 m_airSpeed
DT_LocalPlayerExclusive!0x307c m_airAcceleration
DT_LocalPlayerExclusive!0x30a8 m_firstPersonProxy
DT_LocalPlayerExclusive!0x30ac m_predictedFirstPersonProxy
DT_LocalPlayerExclusive!0x30bc m_hardpointEntity
DT_LocalPlayerExclusive!0x30f4 m_petTitanMode
DT_LocalPlayerExclusive!0x30fc m_hThirdPersonEnt
DT_LocalPlayerExclusive!0x3100 m_thirdPersonShoulderView
DT_LocalPlayerExclusive!0x315c m_thirdPerson
DT_LocalPlayerExclusive!0x3238 m_viewConeLerpTime
DT_LocalPlayerExclusive!0x345c m_flLaggedMovementValue
DT_LocalPlayerExclusive!0x3460 m_lastMoveInputTime
DT_LocalPlayerExclusive!0x3464 m_ignoreEntityForMovementUntilNotTouching
DT_LocalPlayerExclusive!0x39ac m_lungeTargetEntity
DT_LocalPlayerExclusive!0x39b0 m_isLungingToPosition
DT_LocalPlayerExclusive!0x39b4 m_lungeTargetPosition
DT_LocalPlayerExclusive!0x39c0 m_lungeStartPositionOffset
DT_LocalPlayerExclusive!0x39cc m_lungeEndPositionOffset
DT_LocalPlayerExclusive!0x39d8 m_lungeStartTime
DT_LocalPlayerExclusive!0x39dc m_lungeEndTime
DT_LocalPlayerExclusive!0x39e0 m_lungeCanFly
DT_LocalPlayerExclusive!0x39e1 m_lungeLockPitch
DT_LocalPlayerExclusive!0x39e4 m_lungeStartPitch
DT_LocalPlayerExclusive!0x39e8 m_lungeSmoothTime
DT_LocalPlayerExclusive!0x39ec m_lungeMaxTime
DT_LocalPlayerExclusive!0x39f0 m_lungeMaxEndSpeed
DT_LocalPlayerExclusive!0x3e34 m_nearbyPusherCount
DT_LocalPlayerExclusive!0x3e44 m_pushAwayFromTopAcceleration
DT_LocalPlayerExclusive!0x3e54 m_minimapTargetZoomScale
DT_LocalPlayerExclusive!0x3e58 m_minimapTargetLerpTime
DT_LocalPlayerExclusive!0x3e60 m_playerScriptNetDataExclusive
DT_LocalPlayerExclusive!0x3e84 m_skydiveForwardPoseValueVelocity
DT_LocalPlayerExclusive!0x3e8c m_skydiveForwardPoseValueCurrent
DT_LocalPlayerExclusive!0x3e90 m_skydiveSidePoseValueVelocity
DT_LocalPlayerExclusive!0x3e98 m_skydiveSidePoseValueCurrent
DT_LocalPlayerExclusive!0x3e9c m_skydiveYawVelocity
DT_LocalPlayerExclusive!0x3ebc m_freefallStartTime
DT_LocalPlayerExclusive!0x3ec0 m_freefallEndTime
DT_LocalPlayerExclusive!0x3ec4 m_freefallAnticipateStartTime
DT_LocalPlayerExclusive!0x3ec8 m_freefallAnticipateEndTime
DT_LocalPlayerExclusive!0x3ecc m_freefallDistanceToLand
DT_LocalPlayerExclusive!0x3ee0 m_skydiveFreelookEnabled
DT_LocalPlayerExclusive!0x3ee4 m_skydiveFreelookLockedAngle
DT_LocalPlayerExclusive!0x3ef8 m_skydiveFollowing
DT_LocalPlayerExclusive!0x3efc m_skydiveUnfollowVelocity
DT_LocalPlayerExclusive!0x3f09 m_skydiveIsNearLeviathan
DT_LocalPlayerExclusive!0x3f0c m_skydiveLeviathanHitPosition
DT_LocalPlayerExclusive!0x3f18 m_skydiveLeviathanHitNormal
DT_LocalPlayerExclusive!0x3f24 m_skydiveSlipVelocity
DT_LocalPlayerExclusive!0x3f30 m_skydiveFromUpdraft
DT_LocalPlayerExclusive!0x3f38 m_twitchRewardBits
DT_LocalPlayerExclusive!0x3f48 m_playerKnockBacks
DT_LocalPlayerExclusive!0x3fc8 m_updraftCount
DT_LocalPlayerExclusive!0x3fcc m_updraftStage
DT_LocalPlayerExclusive!0x3fd0 m_updraftEnterTime
DT_LocalPlayerExclusive!0x3fd4 m_updraftLeaveTime
DT_LocalPlayerExclusive!0x3fd8 m_updraftMinShakeActivationHeight
DT_LocalPlayerExclusive!0x3fdc m_updraftMaxShakeActivationHeight
DT_LocalPlayerExclusive!0x3fe0 m_updraftLiftActivationHeight
DT_LocalPlayerExclusive!0x3fe4 m_updraftLiftSpeed
DT_LocalPlayerExclusive!0x3fe8 m_updraftLiftAcceleration
DT_LocalPlayerExclusive!0x3fec m_updraftLiftExitDuration
DT_LocalPlayerExclusive!0x3ff0 m_updraftSlowTime
DT_LocalPlayerExclusive!0x3ff4 m_updraftEndSkydiveAtPeak
DT_LocalPlayerExclusive!0x3ff8 m_armsModelIndex
```
</details>
<details>
<summary><code>class DT_MinimapBaseEntityData</code></summary>

```
{
	visibilityDefaultFlag: DataTable,
	visibilityShowFlag: DataTable,
	flags: Int,
	zOrder: Int,
	customState: Int,
	objectScale: Float,
}
```

### Offsets

```
DT_MinimapBaseEntityData!0x0000 visibilityDefaultFlag
DT_MinimapBaseEntityData!0x0020 visibilityShowFlag
DT_MinimapBaseEntityData!0x0040 flags
DT_MinimapBaseEntityData!0x0044 zOrder
DT_MinimapBaseEntityData!0x0048 customState
DT_MinimapBaseEntityData!0x004c objectScale
```
</details>
<details>
<summary><code>class DT_MovieDisplay extends DT_BaseEntity</code></summary>

```
{
	m_bEnabled: Int,
	m_bLooping: Int,
	m_szMovieFilename: String,
	m_szGroupName: String,
	m_szExternalAudioFilename: String,
	m_bStretchToFill: Int,
	m_bLetterbox: Int,
	m_bPausesWithClient: Int,
	m_bForcedSlave: Int,
	m_bUseCustomUVs: Int,
	m_flUMin: Float,
	m_flUMax: Float,
	m_flVMin: Float,
	m_flVMax: Float,
}
```

### Offsets

```
DT_MovieDisplay!0x0a00 m_bEnabled
DT_MovieDisplay!0x0a01 m_bLooping
DT_MovieDisplay!0x0a03 m_szMovieFilename
DT_MovieDisplay!0x0a83 m_szGroupName
DT_MovieDisplay!0x0b03 m_szExternalAudioFilename
DT_MovieDisplay!0x0b43 m_bStretchToFill
DT_MovieDisplay!0x0b44 m_bLetterbox
DT_MovieDisplay!0x0b45 m_bPausesWithClient
DT_MovieDisplay!0x0b46 m_bForcedSlave
DT_MovieDisplay!0x0b47 m_bUseCustomUVs
DT_MovieDisplay!0x0b4c m_flUMin
DT_MovieDisplay!0x0b50 m_flUMax
DT_MovieDisplay!0x0b54 m_flVMin
DT_MovieDisplay!0x0b58 m_flVMax
```
</details>
<details>
<summary><code>class DT_NPC_SentryTurret extends DT_AI_BaseNPC</code></summary>

```
{
	m_turretState: Int,
	m_killCount: Int,
	m_titanKillCount: Int,
	m_eyeAttach: Int,
	m_controlPanel: Int,
}
```

### Offsets

```
DT_NPC_SentryTurret!0x1a40 m_turretState
DT_NPC_SentryTurret!0x1a44 m_killCount
DT_NPC_SentryTurret!0x1a48 m_titanKillCount
DT_NPC_SentryTurret!0x1a4c m_eyeAttach
DT_NPC_SentryTurret!0x1a50 m_controlPanel
```
</details>
<details>
<summary><code>class DT_NPC_Titan extends DT_AI_BaseNPC</code></summary>

```
{
	m_decalIndex: Int,
	m_inventory: DT_WeaponInventory,
	m_selectedOffhands: DataTable,
	m_titanSoul: Int,
	m_grappleHook: Int,
	m_grapple: DT_GrappleData,
	m_grappleActive: Int,
	m_canStand: Int,
}
```

### Offsets

```
DT_NPC_Titan!0x0e58 m_decalIndex
DT_NPC_Titan!0x16b0 m_inventory
DT_NPC_Titan!0x1716 m_selectedOffhands
DT_NPC_Titan!0x1788 m_titanSoul
DT_NPC_Titan!0x1a40 m_grappleHook
DT_NPC_Titan!0x1a48 m_grapple
DT_NPC_Titan!0x1ad8 m_grappleActive
DT_NPC_Titan!0x1ad9 m_canStand
```
</details>
<details>
<summary><code>class DT_NearbyPushers</code></summary>

```
{
	m_nearbyPushers: DataTable,
}
```

### Offsets

```
DT_NearbyPushers!0x3ce4 m_nearbyPushers
```
</details>
<details>
<summary><code>class DT_OverlayVars</code></summary>

```
{
	m_AnimOverlay: DataTable,
	m_AnimOverlayCount: Int,
}
```

### Offsets

```
DT_OverlayVars!0x1308 m_AnimOverlay
DT_OverlayVars!0x13e0 m_AnimOverlayCount
```
</details>
<details>
<summary><code>class DT_ParticleSystem</code></summary>

```
{
	m_localOrigin: Vector,
	moveparent: Int,
	m_parentAttachmentType: Int,
	m_parentAttachmentIndex: Int,
	m_fEffects: Int,
	m_visibilityFlags: Int,
	m_iTeamNum: Int,
	m_localAngles: Vector,
	m_hOwnerEntity: Int,
	m_parentAttachmentHitbox: Int,
	m_parentAttachmentModel: Int,
	m_realmsBitMask: BitMask,
	m_iEffectIndex: Int,
	m_nStopType: Int,
	m_bActive: Int,
	m_bForceRenderAlways: Int,
	m_flStartTime: Time,
	m_bInSkybox: Int,
	m_killForReplay: Int,
	m_killIfOverLimit: Int,
	m_vServerControlPoints: DataTable,
	m_hControlPointEnts: DataTable,
	m_controlPointAttachTypes: DataTable,
	m_controlPoint1AttachmentIndex: Int,
	m_vServerControlPointColorIds: DataTable,
	m_parentAttachType: Int,
}
```

### Offsets

```
DT_ParticleSystem!0x0004 m_localOrigin
DT_ParticleSystem!0x001c moveparent
DT_ParticleSystem!0x0020 m_parentAttachmentType
DT_ParticleSystem!0x0024 m_parentAttachmentIndex
DT_ParticleSystem!0x0044 m_fEffects
DT_ParticleSystem!0x03e8 m_visibilityFlags
DT_ParticleSystem!0x03f0 m_iTeamNum
DT_ParticleSystem!0x0428 m_localAngles
DT_ParticleSystem!0x043c m_hOwnerEntity
DT_ParticleSystem!0x07f4 m_parentAttachmentHitbox
DT_ParticleSystem!0x07f8 m_parentAttachmentModel
DT_ParticleSystem!0x09e8 m_realmsBitMask
DT_ParticleSystem!0x0a00 m_iEffectIndex
DT_ParticleSystem!0x0a04 m_nStopType
DT_ParticleSystem!0x0a09 m_bActive
DT_ParticleSystem!0x0a0b m_bForceRenderAlways
DT_ParticleSystem!0x0a0c m_flStartTime
DT_ParticleSystem!0x0a15 m_bInSkybox
DT_ParticleSystem!0x0a16 m_killForReplay
DT_ParticleSystem!0x0a17 m_killIfOverLimit
DT_ParticleSystem!0x0a1c m_vServerControlPoints
DT_ParticleSystem!0x0a58 m_hControlPointEnts
DT_ParticleSystem!0x0a6c m_controlPointAttachTypes
DT_ParticleSystem!0x0a84 m_controlPoint1AttachmentIndex
DT_ParticleSystem!0x0a94 m_vServerControlPointColorIds
DT_ParticleSystem!0x0a9c m_parentAttachType
```
</details>
<details>
<summary><code>class DT_PhysicsProp extends DT_BreakableProp</code></summary>

```
{
	m_spawnflags: Int,
	m_bAwake: Int,
	m_ignoresCollisionWithPlayers: Int,
	m_isRolling: Int,
	m_networkTableRollSoundId: Int,
	m_iPhysicsMode: Int,
	m_fMass: Float,
	m_collisionMins: Vector,
	m_collisionMaxs: Vector,
}
```

### Offsets

```
DT_PhysicsProp!0x0094 m_spawnflags
DT_PhysicsProp!0x1308 m_bAwake
DT_PhysicsProp!0x1309 m_ignoresCollisionWithPlayers
DT_PhysicsProp!0x130a m_isRolling
DT_PhysicsProp!0x130c m_networkTableRollSoundId
DT_PhysicsProp!0x1338 m_iPhysicsMode
DT_PhysicsProp!0x133c m_fMass
DT_PhysicsProp!0x1340 m_collisionMins
DT_PhysicsProp!0x134c m_collisionMaxs
```
</details>
<details>
<summary><code>class DT_Player extends DT_BaseCombatCharacter</code></summary>

```
{
	localdata: DT_LocalPlayerExclusive,
	teamshareddata: DT_PlayerTeamShared,
	m_passives: Array,
	portalnonlocaldata: DT_PortalNonLocalPlayerExclusive,
	m_vecAbsOrigin: Vector,
	isLocalOriginLocal: Int,
	m_fFlags: Int,
	m_hGroundEntity: Int,
	m_iHealth: Int,
	m_flMaxspeed: Float,
	m_iMaxHealth: Int,
	m_lifeState: Int,
	m_decalIndex: Int,
	m_inventory: DT_WeaponInventory,
	m_selectedOffhands: DataTable,
	m_selectedOffhandsPendingHybridAction: DataTable,
	m_titanSoul: Int,
	m_bZooming: Int,
	m_zoomToggleOnStartTime: Time,
	m_zoomBaseFrac: Float,
	m_zoomBaseTime: Time,
	m_zoomFullStartTime: Time,
	m_currentFramePlayer: DT_CurrentData_Player,
	pl: DT_PlayerState,
	m_ammoPoolCapacity: Int,
	m_hasBadReputation: Int,
	m_happyHourActive: Int,
	m_communityName: String,
	m_communityClanTag: String,
	m_factionName: String,
	m_hardwareIcon: String,
	m_hardware: Int,
	m_platformUserId: BitMask,
	m_classModsActive: BitMask,
	m_passives[ 0 ]: BitMask,
	m_bleedoutState: Int,
	m_statusEffectsTimedPlayerNV: DataTable,
	m_statusEffectsEndlessPlayerNV: DataTable,
	m_damageComboLatestUpdateTime: Time,
	m_damageComboStartHealth: Int,
	m_gestureSequences: DataTable,
	m_gestureStartTimes: DataTable,
	m_gestureBlendInDuration: DataTable,
	m_gestureBlendOutDuration: DataTable,
	m_gestureFadeOutStartTime: DataTable,
	m_gestureFadeOutDuration: DataTable,
	m_gestureAutoKillBitfield: Int,
	m_autoSprintForced: Int,
	m_fIsSprinting: Int,
	m_damageImpulseNoDecelEndTime: Time,
	m_playerVehicle: Int,
	m_duckState: Int,
	m_leanState: Int,
	m_canStand: Int,
	m_StandHullMin: Vector,
	m_StandHullMax: Vector,
	m_DuckHullMin: Vector,
	m_DuckHullMax: Vector,
	m_entitySyncingWithMe: Int,
	m_upDir: Vector,
	m_traversalState: Int,
	m_traversalType: Int,
	m_traversalForwardDir: Vector,
	m_traversalRefPos: Vector,
	m_traversalYawDelta: Float,
	m_traversalYawPoseParameter: Int,
	m_wallClimbSetUp: Int,
	m_wallHanging: Int,
	m_grapple: DT_GrappleData,
	m_grappleActive: Int,
	m_turret: Int,
	m_hViewModels: DataTable,
	m_viewOffsetEntity: DT_Player_ViewOffsetEntityData,
	m_animViewEntity: DT_Player_AnimViewEntityData,
	m_activeZipline: Int,
	m_ziplineValid3pWeaponLayerAnim: Int,
	m_ziplineState: Int,
	m_ziplineGrenadeBeginStationEntity: Int,
	m_ziplineGrenadeBeginStationAttachmentIndex: Int,
	m_isPerformingBoostAction: Int,
	m_lastJumpPadTouched: Int,
	m_launchCount: Int,
	m_melee: DT_PlayerMelee_PlayerData,
	m_useCredit: Int,
	m_playerFlags: Int,
	m_hasMic: Int,
	m_inPartyChat: Int,
	m_playerMoveSpeedScale: Float,
	m_bShouldDrawPlayerWhileUsingViewEntity: Int,
	m_iSpawnParity: Int,
	m_iObserverMode: Int,
	m_hObserverTarget: Int,
	m_flDeathTime: Time,
	m_lastDodgeTime: Time,
	m_timeJetpackHeightActivateCheckPassed: Time,
	m_grappleHook: Int,
	m_petTitan: Int,
	m_xp: Int,
	m_skill_mu: Float,
	m_bHasMatchAdminRole: Int,
	m_ubEFNoInterpParity: Int,
	m_hColorCorrectionCtrl: Int,
	m_title: String,
	m_Shared: DT_PlayerShared,
	m_pilotClassIndex: Int,
	m_pilotClassActivityModifier: Int,
	m_playerScriptNetDataGlobal: Int,
	m_helmetType: Int,
	m_armorType: Int,
	m_controllerModeActive: Int,
	m_skydiveForwardPoseValueTarget: Float,
	m_skydiveSidePoseValueTarget: Float,
	m_freefallState: Int,
	m_skydiveDiveAngle: Float,
	m_skydiveIsDiving: Int,
	m_skydiveSpeed: Float,
	m_skydiveStrafeAngle: Float,
	m_skydivePlayerPitch: Float,
	m_skydivePlayerYaw: Float,
}
```

### Offsets

```
DT_Player!0x0000 localdata
DT_Player!0x0000 teamshareddata
DT_Player!0x0000 m_passives
DT_Player!0x0000 portalnonlocaldata
DT_Player!0x0004 m_vecAbsOrigin
DT_Player!0x0010 isLocalOriginLocal
DT_Player!0x0098 m_fFlags
DT_Player!0x03dc m_hGroundEntity
DT_Player!0x03e0 m_iHealth
DT_Player!0x03e4 m_flMaxspeed
DT_Player!0x0510 m_iMaxHealth
DT_Player!0x0730 m_lifeState
DT_Player!0x0e58 m_decalIndex
DT_Player!0x16b0 m_inventory
DT_Player!0x1716 m_selectedOffhands
DT_Player!0x1719 m_selectedOffhandsPendingHybridAction
DT_Player!0x1788 m_titanSoul
DT_Player!0x1881 m_bZooming
DT_Player!0x1884 m_zoomToggleOnStartTime
DT_Player!0x1888 m_zoomBaseFrac
DT_Player!0x188c m_zoomBaseTime
DT_Player!0x1890 m_zoomFullStartTime
DT_Player!0x1d18 m_currentFramePlayer
DT_Player!0x2118 pl
DT_Player!0x219c m_ammoPoolCapacity
DT_Player!0x21a0 m_hasBadReputation
DT_Player!0x21a1 m_happyHourActive
DT_Player!0x21a9 m_communityName
DT_Player!0x21e9 m_communityClanTag
DT_Player!0x21f9 m_factionName
DT_Player!0x2209 m_hardwareIcon
DT_Player!0x2219 m_hardware
DT_Player!0x2220 m_platformUserId
DT_Player!0x2230 m_classModsActive
DT_Player!0x2328 m_passives[ 0 ]
DT_Player!0x2348 m_bleedoutState
DT_Player!0x2350 m_statusEffectsTimedPlayerNV
DT_Player!0x2440 m_statusEffectsEndlessPlayerNV
DT_Player!0x24f4 m_damageComboLatestUpdateTime
DT_Player!0x24f8 m_damageComboStartHealth
DT_Player!0x24fc m_gestureSequences
DT_Player!0x251c m_gestureStartTimes
DT_Player!0x253c m_gestureBlendInDuration
DT_Player!0x255c m_gestureBlendOutDuration
DT_Player!0x257c m_gestureFadeOutStartTime
DT_Player!0x259c m_gestureFadeOutDuration
DT_Player!0x25bc m_gestureAutoKillBitfield
DT_Player!0x2600 m_autoSprintForced
DT_Player!0x2604 m_fIsSprinting
DT_Player!0x261c m_damageImpulseNoDecelEndTime
DT_Player!0x2628 m_playerVehicle
DT_Player!0x262c m_duckState
DT_Player!0x2630 m_leanState
DT_Player!0x2635 m_canStand
DT_Player!0x2638 m_StandHullMin
DT_Player!0x2644 m_StandHullMax
DT_Player!0x2650 m_DuckHullMin
DT_Player!0x265c m_DuckHullMax
DT_Player!0x2668 m_entitySyncingWithMe
DT_Player!0x266c m_upDir
DT_Player!0x26e8 m_traversalState
DT_Player!0x26ec m_traversalType
DT_Player!0x2718 m_traversalForwardDir
DT_Player!0x2724 m_traversalRefPos
DT_Player!0x2750 m_traversalYawDelta
DT_Player!0x2754 m_traversalYawPoseParameter
DT_Player!0x276c m_wallClimbSetUp
DT_Player!0x276d m_wallHanging
DT_Player!0x2770 m_grapple
DT_Player!0x2800 m_grappleActive
DT_Player!0x2848 m_turret
DT_Player!0x284c m_hViewModels
DT_Player!0x2860 m_viewOffsetEntity
DT_Player!0x28a0 m_animViewEntity
DT_Player!0x29a0 m_activeZipline
DT_Player!0x29ac m_ziplineValid3pWeaponLayerAnim
DT_Player!0x29b0 m_ziplineState
DT_Player!0x2a44 m_ziplineGrenadeBeginStationEntity
DT_Player!0x2a48 m_ziplineGrenadeBeginStationAttachmentIndex
DT_Player!0x2a85 m_isPerformingBoostAction
DT_Player!0x2b40 m_lastJumpPadTouched
DT_Player!0x2b48 m_launchCount
DT_Player!0x2c78 m_melee
DT_Player!0x2cb0 m_useCredit
DT_Player!0x2cb4 m_playerFlags
DT_Player!0x2cb8 m_hasMic
DT_Player!0x2cb9 m_inPartyChat
DT_Player!0x2cbc m_playerMoveSpeedScale
DT_Player!0x2ec8 m_bShouldDrawPlayerWhileUsingViewEntity
DT_Player!0x2f34 m_iSpawnParity
DT_Player!0x2f3c m_iObserverMode
DT_Player!0x2f40 m_hObserverTarget
DT_Player!0x2fcc m_flDeathTime
DT_Player!0x3008 m_lastDodgeTime
DT_Player!0x3028 m_timeJetpackHeightActivateCheckPassed
DT_Player!0x30b0 m_grappleHook
DT_Player!0x30b4 m_petTitan
DT_Player!0x30dc m_xp
DT_Player!0x30e4 m_skill_mu
DT_Player!0x30e8 m_bHasMatchAdminRole
DT_Player!0x3960 m_ubEFNoInterpParity
DT_Player!0x3964 m_hColorCorrectionCtrl
DT_Player!0x3988 m_title
DT_Player!0x3c90 m_Shared
DT_Player!0x3cd8 m_pilotClassIndex
DT_Player!0x3cdc m_pilotClassActivityModifier
DT_Player!0x3e5c m_playerScriptNetDataGlobal
DT_Player!0x3e64 m_helmetType
DT_Player!0x3e68 m_armorType
DT_Player!0x3e6c m_controllerModeActive
DT_Player!0x3e88 m_skydiveForwardPoseValueTarget
DT_Player!0x3e94 m_skydiveSidePoseValueTarget
DT_Player!0x3eb8 m_freefallState
DT_Player!0x3ed0 m_skydiveDiveAngle
DT_Player!0x3ed4 m_skydiveIsDiving
DT_Player!0x3ed8 m_skydiveSpeed
DT_Player!0x3edc m_skydiveStrafeAngle
DT_Player!0x3ef0 m_skydivePlayerPitch
DT_Player!0x3ef4 m_skydivePlayerYaw
```
</details>
<details>
<summary><code>class DT_PlayerDecoy extends DT_BaseAnimating</code></summary>

```
{
	m_cloakEndTime: Float,
	m_cloakFadeInEndTime: Time,
	m_cloakFadeOutStartTime: Float,
	m_cloakFadeInDuration: Float,
	m_cloakFlickerAmount: Float,
	m_cloakFlickerEndTime: Time,
	m_iHealth: Int,
	m_iMaxHealth: Int,
	m_nameVisibilityFlags: Int,
	m_currentState: Int,
	m_decoyFlags: Int,
	m_lastPulseTime: Time,
	m_currentClass: BitMask,
	m_classModsActive: BitMask,
}
```

### Offsets

```
DT_PlayerDecoy!0x019c m_cloakEndTime
DT_PlayerDecoy!0x01a0 m_cloakFadeInEndTime
DT_PlayerDecoy!0x01a4 m_cloakFadeOutStartTime
DT_PlayerDecoy!0x01a8 m_cloakFadeInDuration
DT_PlayerDecoy!0x01ac m_cloakFlickerAmount
DT_PlayerDecoy!0x01b0 m_cloakFlickerEndTime
DT_PlayerDecoy!0x03e0 m_iHealth
DT_PlayerDecoy!0x0510 m_iMaxHealth
DT_PlayerDecoy!0x0958 m_nameVisibilityFlags
DT_PlayerDecoy!0x1300 m_currentState
DT_PlayerDecoy!0x1304 m_decoyFlags
DT_PlayerDecoy!0x130c m_lastPulseTime
DT_PlayerDecoy!0x1310 m_currentClass
DT_PlayerDecoy!0x1318 m_classModsActive
```
</details>
<details>
<summary><code>class DT_PlayerMelee_PlayerData</code></summary>

```
{
	meleeAttackParity: Int,
	attackActive: Int,
	attackRecoveryShouldBeQuick: Int,
	isSprintAttack: Int,
	attackStartTime: Time,
	attackHitEntity: Int,
	attackHitEntityTime: Time,
	attackLastHitNonWorldEntity: Time,
	scriptedState: Int,
	pendingMeleePress: Int,
	lungeBoost: Vector,
}
```

### Offsets

```
DT_PlayerMelee_PlayerData!0x0008 meleeAttackParity
DT_PlayerMelee_PlayerData!0x000c attackActive
DT_PlayerMelee_PlayerData!0x000d attackRecoveryShouldBeQuick
DT_PlayerMelee_PlayerData!0x000e isSprintAttack
DT_PlayerMelee_PlayerData!0x0010 attackStartTime
DT_PlayerMelee_PlayerData!0x0014 attackHitEntity
DT_PlayerMelee_PlayerData!0x0018 attackHitEntityTime
DT_PlayerMelee_PlayerData!0x001c attackLastHitNonWorldEntity
DT_PlayerMelee_PlayerData!0x0020 scriptedState
DT_PlayerMelee_PlayerData!0x0024 pendingMeleePress
DT_PlayerMelee_PlayerData!0x0028 lungeBoost
```
</details>
<details>
<summary><code>class DT_PlayerResource</code></summary>

```
{
	m_boolStats: DataTable,
	m_iPing: DataTable,
	m_bConnected: DataTable,
}
```

### Offsets

```
DT_PlayerResource!0x1410 m_boolStats
DT_PlayerResource!0x2c40 m_iPing
DT_PlayerResource!0x2e44 m_bConnected
```
</details>
<details>
<summary><code>class DT_PlayerTasklist extends DT_BaseEntity</code></summary>

```
{
	m_notifyTime: Time,
	m_customInt: Int,
	m_taskStatus: DataTable,
	m_taskType: DataTable,
	m_taskCountGoal: DataTable,
	m_taskCountNow: DataTable,
	m_taskFlags: DataTable,
	m_taskGameTimes: DataTable,
	m_taskInts: DataTable,
	m_taskFloats: DataTable,
	m_taskEnts: DataTable,
	m_taskStringA: String,
	m_taskStringB: String,
	m_taskStringC: String,
	m_taskStringD: String,
	m_taskStringE: String,
	m_taskStringF: String,
	m_taskStringG: String,
	m_taskStringH: String,
	m_taskStringI: String,
	m_taskStringJ: String,
	m_taskStringK: String,
	m_taskStringL: String,
	m_taskStringM: String,
}
```

### Offsets

```
DT_PlayerTasklist!0x0a00 m_notifyTime
DT_PlayerTasklist!0x0a04 m_customInt
DT_PlayerTasklist!0x0a08 m_taskStatus
DT_PlayerTasklist!0x0a3c m_taskType
DT_PlayerTasklist!0x0a70 m_taskCountGoal
DT_PlayerTasklist!0x0aa4 m_taskCountNow
DT_PlayerTasklist!0x0ad8 m_taskFlags
DT_PlayerTasklist!0x0b0c m_taskGameTimes
DT_PlayerTasklist!0x0b40 m_taskInts
DT_PlayerTasklist!0x0b74 m_taskFloats
DT_PlayerTasklist!0x0ba8 m_taskEnts
DT_PlayerTasklist!0x0bdc m_taskStringA
DT_PlayerTasklist!0x0c1c m_taskStringB
DT_PlayerTasklist!0x0c5c m_taskStringC
DT_PlayerTasklist!0x0c9c m_taskStringD
DT_PlayerTasklist!0x0cdc m_taskStringE
DT_PlayerTasklist!0x0d1c m_taskStringF
DT_PlayerTasklist!0x0d5c m_taskStringG
DT_PlayerTasklist!0x0d9c m_taskStringH
DT_PlayerTasklist!0x0ddc m_taskStringI
DT_PlayerTasklist!0x0e1c m_taskStringJ
DT_PlayerTasklist!0x0e5c m_taskStringK
DT_PlayerTasklist!0x0e9c m_taskStringL
DT_PlayerTasklist!0x0edc m_taskStringM
```
</details>
<details>
<summary><code>class DT_PlayerTeamShared</code></summary>

```
{
	m_healResources_healthTarget: Int,
	m_lastTimeDamagedByOtherPlayer: Time,
	m_lastTimeDamagedByNPC: Time,
	m_lastTimeDidDamageToOtherPlayer: Time,
	m_lastTimeDidDamageToNPC: Time,
}
```

### Offsets

```
DT_PlayerTeamShared!0x21a4 m_healResources_healthTarget
DT_PlayerTeamShared!0x2ae8 m_lastTimeDamagedByOtherPlayer
DT_PlayerTeamShared!0x2aec m_lastTimeDamagedByNPC
DT_PlayerTeamShared!0x2af0 m_lastTimeDidDamageToOtherPlayer
DT_PlayerTeamShared!0x2af4 m_lastTimeDidDamageToNPC
```
</details>
<details>
<summary><code>class DT_PlayerVehicle extends DT_BaseAnimating</code></summary>

```
{
	vehicledriverdata: DT_VehicleDriverExclusive,
	vehiclenondriverdata: DT_VehicleNonDriverExclusive,
	m_vehiclePlayers: Array,
	m_vecViewOffset.x: Float,
	m_vecViewOffset.y: Float,
	m_vecViewOffset.z: Float,
	m_localAngles: Vector,
	m_vehicleDriver: Int,
	m_vehiclePlayers[0]: Int,
	m_vehiclePlayerCount: Int,
	m_vehicleActivated: Int,
	m_blockDuckInput: Int,
	m_vehicleFlags: Int,
	m_vehicleType: Int,
	m_vehicleLaunchTime: Float,
	m_vehicleVelocity: Vector,
	m_overrideVehicleAngles: Vector,
	m_overrideVehicleAnglesUntilTick: Int,
	m_pushingEnt: Int,
}
```

### Offsets

```
DT_PlayerVehicle!0x0000 vehicledriverdata
DT_PlayerVehicle!0x0000 vehiclenondriverdata
DT_PlayerVehicle!0x0000 m_vehiclePlayers
DT_PlayerVehicle!0x0038 m_vecViewOffset.x
DT_PlayerVehicle!0x003c m_vecViewOffset.y
DT_PlayerVehicle!0x0040 m_vecViewOffset.z
DT_PlayerVehicle!0x0428 m_localAngles
DT_PlayerVehicle!0x1304 m_vehicleDriver
DT_PlayerVehicle!0x130c m_vehiclePlayers[0]
DT_PlayerVehicle!0x131c m_vehiclePlayerCount
DT_PlayerVehicle!0x1320 m_vehicleActivated
DT_PlayerVehicle!0x1321 m_blockDuckInput
DT_PlayerVehicle!0x1324 m_vehicleFlags
DT_PlayerVehicle!0x1328 m_vehicleType
DT_PlayerVehicle!0x1330 m_vehicleLaunchTime
DT_PlayerVehicle!0x1338 m_vehicleVelocity
DT_PlayerVehicle!0x1358 m_overrideVehicleAngles
DT_PlayerVehicle!0x1364 m_overrideVehicleAnglesUntilTick
DT_PlayerVehicle!0x138c m_pushingEnt
```
</details>
<details>
<summary><code>class DT_PlayerWaypoint</code></summary>

```
{
	moveparent: Int,
	m_parentAttachmentIndex: Int,
	m_cellX: Int,
	m_cellY: Int,
	m_cellZ: Int,
	m_localOrigin: Vector,
	m_networkedFlags: Int,
	m_visibilityFlags: Int,
	m_iTeamNum: Int,
	m_teamMemberIndex: Int,
	m_hOwnerEntity: Int,
	m_iSignifierName: String,
	m_parentAttachmentModel: Int,
	m_realmsBitMask: BitMask,
	m_waypointType: Int,
	m_waypointBitfield: Int,
	m_waypointEnts: DataTable,
	m_waypointVectors: DataTable,
	m_waypointGameTimes: DataTable,
	m_waypointInts: DataTable,
	m_waypointFloats: DataTable,
	m_objectivePackedInt: Int,
	m_waypointGroupName: String,
	m_waypointGroupFlags: Int,
	m_waypointCustomType: String,
	m_waypointStringA: String,
	m_waypointStringB: String,
	m_waypointAssetA: String,
	m_waypointAssetB: String,
}
```

### Offsets

```
DT_PlayerWaypoint!0x001c moveparent
DT_PlayerWaypoint!0x0024 m_parentAttachmentIndex
DT_PlayerWaypoint!0x004c m_cellX
DT_PlayerWaypoint!0x0050 m_cellY
DT_PlayerWaypoint!0x0054 m_cellZ
DT_PlayerWaypoint!0x0058 m_localOrigin
DT_PlayerWaypoint!0x0394 m_networkedFlags
DT_PlayerWaypoint!0x03e8 m_visibilityFlags
DT_PlayerWaypoint!0x03f0 m_iTeamNum
DT_PlayerWaypoint!0x03f4 m_teamMemberIndex
DT_PlayerWaypoint!0x043c m_hOwnerEntity
DT_PlayerWaypoint!0x0518 m_iSignifierName
DT_PlayerWaypoint!0x07f8 m_parentAttachmentModel
DT_PlayerWaypoint!0x09e8 m_realmsBitMask
DT_PlayerWaypoint!0x0a00 m_waypointType
DT_PlayerWaypoint!0x0a04 m_waypointBitfield
DT_PlayerWaypoint!0x0a08 m_waypointEnts
DT_PlayerWaypoint!0x0a28 m_waypointVectors
DT_PlayerWaypoint!0x0a88 m_waypointGameTimes
DT_PlayerWaypoint!0x0aa8 m_waypointInts
DT_PlayerWaypoint!0x0ac8 m_waypointFloats
DT_PlayerWaypoint!0x0ae8 m_objectivePackedInt
DT_PlayerWaypoint!0x0aec m_waypointGroupName
DT_PlayerWaypoint!0x0b0c m_waypointGroupFlags
DT_PlayerWaypoint!0x0b10 m_waypointCustomType
DT_PlayerWaypoint!0x0b30 m_waypointStringA
DT_PlayerWaypoint!0x0b70 m_waypointStringB
DT_PlayerWaypoint!0x0bb8 m_waypointAssetA
DT_PlayerWaypoint!0x0c38 m_waypointAssetB
```
</details>
<details>
<summary><code>class DT_PlayerZipline</code></summary>

```
{
	m_ziplineReenableWeapons: Int,
	m_mountingZiplineDuration: Float,
	m_mountingZiplineAlpha: Float,
	m_ziplineStartTime: Time,
	m_ziplineEndTime: Time,
	m_mountingZiplineSourcePosition: Vector,
	m_mountingZiplineSourceVelocity: Vector,
	m_mountingZiplineTargetPosition: Vector,
	m_ziplineUsePosition: Vector,
	m_slidingZiplineAlpha: Float,
	m_lastMoveDir2D: Vector,
	m_ziplineReverse: Int,
}
```

### Offsets

```
DT_PlayerZipline!0x0008 m_ziplineReenableWeapons
DT_PlayerZipline!0x000c m_mountingZiplineDuration
DT_PlayerZipline!0x0010 m_mountingZiplineAlpha
DT_PlayerZipline!0x0014 m_ziplineStartTime
DT_PlayerZipline!0x0018 m_ziplineEndTime
DT_PlayerZipline!0x001c m_mountingZiplineSourcePosition
DT_PlayerZipline!0x0028 m_mountingZiplineSourceVelocity
DT_PlayerZipline!0x0034 m_mountingZiplineTargetPosition
DT_PlayerZipline!0x004c m_ziplineUsePosition
DT_PlayerZipline!0x0058 m_slidingZiplineAlpha
DT_PlayerZipline!0x005c m_lastMoveDir2D
DT_PlayerZipline!0x0068 m_ziplineReverse
```
</details>
<details>
<summary><code>class DT_Player_AnimViewEntityData</code></summary>

```
{
	animViewEntityHandle: Int,
	animViewEntityAngleLerpInDuration: Float,
	animViewEntityOriginLerpInDuration: Float,
	animViewEntityLerpOutDuration: Float,
	animViewEntityStabilizePlayerEyeAngles: Int,
	animViewEntityThirdPersonCameraParity: Int,
	animViewEntityThirdPersonCameraAttachment: DataTable,
	animViewEntityNumThirdPersonCameraAttachments: Int,
	animViewEntityThirdPersonCameraVisibilityChecks: Int,
	animViewEntityDrawPlayer: Int,
	fovTarget: Float,
	fovSmoothTime: Float,
	animViewEntityParity: Int,
}
```

### Offsets

```
DT_Player_AnimViewEntityData!0x0000 animViewEntityHandle
DT_Player_AnimViewEntityData!0x0004 animViewEntityAngleLerpInDuration
DT_Player_AnimViewEntityData!0x0008 animViewEntityOriginLerpInDuration
DT_Player_AnimViewEntityData!0x000c animViewEntityLerpOutDuration
DT_Player_AnimViewEntityData!0x0010 animViewEntityStabilizePlayerEyeAngles
DT_Player_AnimViewEntityData!0x0014 animViewEntityThirdPersonCameraParity
DT_Player_AnimViewEntityData!0x0018 animViewEntityThirdPersonCameraAttachment
DT_Player_AnimViewEntityData!0x0030 animViewEntityNumThirdPersonCameraAttachments
DT_Player_AnimViewEntityData!0x0034 animViewEntityThirdPersonCameraVisibilityChecks
DT_Player_AnimViewEntityData!0x0035 animViewEntityDrawPlayer
DT_Player_AnimViewEntityData!0x0038 fovTarget
DT_Player_AnimViewEntityData!0x003c fovSmoothTime
DT_Player_AnimViewEntityData!0x0048 animViewEntityParity
```
</details>
<details>
<summary><code>class DT_PortalNonLocalPlayerExclusive</code></summary>

```
{
	m_pusher: Int,
	m_originRelativeToPusher: Vector,
	m_cellX: Int,
	m_cellY: Int,
	m_cellZ: Int,
	m_localOrigin: VectorXY,
	m_localOrigin.z: Float,
}
```

### Offsets

```
DT_PortalNonLocalPlayerExclusive!0x0028 m_pusher
DT_PortalNonLocalPlayerExclusive!0x002c m_originRelativeToPusher
DT_PortalNonLocalPlayerExclusive!0x004c m_cellX
DT_PortalNonLocalPlayerExclusive!0x0050 m_cellY
DT_PortalNonLocalPlayerExclusive!0x0054 m_cellZ
DT_PortalNonLocalPlayerExclusive!0x0058 m_localOrigin
DT_PortalNonLocalPlayerExclusive!0x0060 m_localOrigin.z
```
</details>
<details>
<summary><code>class DT_PortalPointPush extends DT_BaseEntity</code></summary>

```
{
	m_bEnabled: Int,
	m_flMagnitude: Float,
	m_flRadius: Float,
	m_flInnerRadius: Float,
	m_flConeOfInfluence: Float,
}
```

### Offsets

```
DT_PortalPointPush!0x0a00 m_bEnabled
DT_PortalPointPush!0x0a04 m_flMagnitude
DT_PortalPointPush!0x0a08 m_flRadius
DT_PortalPointPush!0x0a0c m_flInnerRadius
DT_PortalPointPush!0x0a10 m_flConeOfInfluence
```
</details>
<details>
<summary><code>class DT_PostProcessController extends DT_BaseEntity</code></summary>

```
{
	m_flPostProcessParameters: DataTable,
	m_bMaster: Int,
}
```

### Offsets

```
DT_PostProcessController!0x0a00 m_flPostProcessParameters
DT_PostProcessController!0x0a18 m_bMaster
```
</details>
<details>
<summary><code>class DT_PredictedAnimEventData</code></summary>

```
{
	m_predictedAnimEventTimes: DataTable,
	m_predictedAnimEventIndices: DataTable,
	m_predictedAnimEventCount: Int,
	m_predictedAnimEventTarget: Int,
	m_predictedAnimEventSequence: Int,
	m_predictedAnimEventModel: Int,
	m_predictedAnimEventsReadyToFireTime: Time,
}
```

### Offsets

```
DT_PredictedAnimEventData!0x0008 m_predictedAnimEventTimes
DT_PredictedAnimEventData!0x0028 m_predictedAnimEventIndices
DT_PredictedAnimEventData!0x0048 m_predictedAnimEventCount
DT_PredictedAnimEventData!0x004c m_predictedAnimEventTarget
DT_PredictedAnimEventData!0x0050 m_predictedAnimEventSequence
DT_PredictedAnimEventData!0x0054 m_predictedAnimEventModel
DT_PredictedAnimEventData!0x0058 m_predictedAnimEventsReadyToFireTime
```
</details>
<details>
<summary><code>class DT_Projectile</code></summary>

```
{
	m_cellX: Int,
	m_cellY: Int,
	m_cellZ: Int,
	m_localOrigin: Vector,
	m_nModelIndex: Int,
	m_networkedFlags: Int,
	m_iTeamNum: Int,
	m_vecVelocity: Vector,
	m_localAngles: Vector,
	m_hOwnerEntity: Int,
	m_CollisionGroup: Int,
	m_PredictableID: Int,
	m_realmsBitMask: BitMask,
	m_weaponDataIsSet: Int,
	m_forceAdjustToGunBarrelDisabled: Int,
	m_weaponClassIndex: Int,
	m_destructionDistance: Float,
	m_passThroughDepthTotal: Int,
	m_modBitfield: Int,
	m_overrideMods: Int,
	m_projectileTrailIndex: Int,
	m_impactEffectTable: Int,
	m_reducedEffects: Int,
	m_projectileCreationTimeServer: Time,
	m_weaponSource: Int,
}
```

### Offsets

```
DT_Projectile!0x004c m_cellX
DT_Projectile!0x0050 m_cellY
DT_Projectile!0x0054 m_cellZ
DT_Projectile!0x0058 m_localOrigin
DT_Projectile!0x0064 m_nModelIndex
DT_Projectile!0x0394 m_networkedFlags
DT_Projectile!0x03f0 m_iTeamNum
DT_Projectile!0x041c m_vecVelocity
DT_Projectile!0x0428 m_localAngles
DT_Projectile!0x043c m_hOwnerEntity
DT_Projectile!0x04d8 m_CollisionGroup
DT_Projectile!0x0764 m_PredictableID
DT_Projectile!0x09e8 m_realmsBitMask
DT_Projectile!0x1300 m_weaponDataIsSet
DT_Projectile!0x1301 m_forceAdjustToGunBarrelDisabled
DT_Projectile!0x1304 m_weaponClassIndex
DT_Projectile!0x1308 m_destructionDistance
DT_Projectile!0x130c m_passThroughDepthTotal
DT_Projectile!0x1310 m_modBitfield
DT_Projectile!0x1314 m_overrideMods
DT_Projectile!0x1318 m_projectileTrailIndex
DT_Projectile!0x131c m_impactEffectTable
DT_Projectile!0x1320 m_reducedEffects
DT_Projectile!0x1324 m_projectileCreationTimeServer
DT_Projectile!0x1328 m_weaponSource
```
</details>
<details>
<summary><code>class DT_PropSurvival</code></summary>

```
{
	moveparent: Int,
	m_parentAttachmentType: Int,
	m_parentAttachmentIndex: Int,
	m_fEffects: Int,
	m_usableType: Int,
	m_cellX: Int,
	m_cellY: Int,
	m_cellZ: Int,
	m_localOrigin: Vector,
	m_nModelIndex: Int,
	m_networkedFlags: Int,
	m_visibilityFlags: Int,
	m_localAngles: Vector,
	m_Collision: DT_CollisionProperty,
	m_CollisionGroup: Int,
	m_iSignifierName: String,
	m_parentAttachmentModel: Int,
	m_fadeDist: Float,
	m_usablePriority: Int,
	m_usableDistanceOverride: Float,
	m_usableFOV: Float,
	m_usePromptSize: Float,
	m_realmsBitMask: BitMask,
	m_nSkin: Int,
	m_skinMod: Int,
	m_nBody: Int,
	m_camoIndex: Int,
	m_ammoInClip: Int,
	m_customScriptInt: Int,
	m_survivalProperty: Int,
	m_weaponNameIndex: Int,
	m_modBitField: Int,
}
```

### Offsets

```
DT_PropSurvival!0x001c moveparent
DT_PropSurvival!0x0020 m_parentAttachmentType
DT_PropSurvival!0x0024 m_parentAttachmentIndex
DT_PropSurvival!0x0044 m_fEffects
DT_PropSurvival!0x0048 m_usableType
DT_PropSurvival!0x004c m_cellX
DT_PropSurvival!0x0050 m_cellY
DT_PropSurvival!0x0054 m_cellZ
DT_PropSurvival!0x0058 m_localOrigin
DT_PropSurvival!0x0064 m_nModelIndex
DT_PropSurvival!0x0394 m_networkedFlags
DT_PropSurvival!0x03e8 m_visibilityFlags
DT_PropSurvival!0x0428 m_localAngles
DT_PropSurvival!0x0458 m_Collision
DT_PropSurvival!0x04d8 m_CollisionGroup
DT_PropSurvival!0x0518 m_iSignifierName
DT_PropSurvival!0x07f8 m_parentAttachmentModel
DT_PropSurvival!0x0804 m_fadeDist
DT_PropSurvival!0x08c8 m_usablePriority
DT_PropSurvival!0x08cc m_usableDistanceOverride
DT_PropSurvival!0x08d0 m_usableFOV
DT_PropSurvival!0x08d4 m_usePromptSize
DT_PropSurvival!0x09e8 m_realmsBitMask
DT_PropSurvival!0x0e48 m_nSkin
DT_PropSurvival!0x0e4c m_skinMod
DT_PropSurvival!0x0e50 m_nBody
DT_PropSurvival!0x0e54 m_camoIndex
DT_PropSurvival!0x1304 m_ammoInClip
DT_PropSurvival!0x1308 m_customScriptInt
DT_PropSurvival!0x130c m_survivalProperty
DT_PropSurvival!0x1310 m_weaponNameIndex
DT_PropSurvival!0x1314 m_modBitField
```
</details>
<details>
<summary><code>class DT_RopeKeyframe</code></summary>

```
{
	m_localOrigin: Vector,
	moveparent: Int,
	m_parentAttachmentType: Int,
	m_parentAttachmentIndex: Int,
	m_visibilityFlags: Int,
	m_hOwnerEntity: Int,
	m_parentAttachmentHitbox: Int,
	m_parentAttachmentModel: Int,
	m_fadeDist: Float,
	m_ropeZiplineAutoDetachDistance: Float,
	m_ziplineSagEnable: Int,
	m_ziplineSagHeight: Float,
	m_ziplineMoveSpeedScale: Float,
	m_wiggleFadeStartTime: Time,
	m_wiggleEndTime: Time,
	m_wiggleMaxLen: Float,
	m_wiggleMagnitude: Float,
	m_wiggleSpeed: Float,
	m_flScrollSpeed: Float,
	m_RopeFlags: Int,
	m_iRopeMaterialModelIndex: Int,
	m_nSegments: Int,
	m_hStartPoint: Int,
	m_hEndPoint: Int,
	m_hPrevPoint: Int,
	m_iStartAttachment: Int,
	m_iEndAttachment: Int,
	m_subdivStackCount: Int,
	m_subdivSliceCount: Int,
	m_ropeLength: Int,
	m_constraintIterations: Int,
	m_ropeDampening: Float,
	m_Slack: Int,
	m_TextureScale: Float,
	m_TextureScale: Float,
	m_fLockedPoints: Int,
	m_lockDirectionCutoffLength: Int,
	m_lockDirectionStrength: Float,
	m_nChangeCount: Int,
	m_Width: Float,
	m_bConstrainBetweenEndpoints: Int,
}
```

### Offsets

```
DT_RopeKeyframe!0x0004 m_localOrigin
DT_RopeKeyframe!0x001c moveparent
DT_RopeKeyframe!0x0020 m_parentAttachmentType
DT_RopeKeyframe!0x0024 m_parentAttachmentIndex
DT_RopeKeyframe!0x03e8 m_visibilityFlags
DT_RopeKeyframe!0x043c m_hOwnerEntity
DT_RopeKeyframe!0x07f4 m_parentAttachmentHitbox
DT_RopeKeyframe!0x07f8 m_parentAttachmentModel
DT_RopeKeyframe!0x0804 m_fadeDist
DT_RopeKeyframe!0x0a00 m_ropeZiplineAutoDetachDistance
DT_RopeKeyframe!0x0a04 m_ziplineSagEnable
DT_RopeKeyframe!0x0a08 m_ziplineSagHeight
DT_RopeKeyframe!0x0b00 m_ziplineMoveSpeedScale
DT_RopeKeyframe!0x0b04 m_wiggleFadeStartTime
DT_RopeKeyframe!0x0b08 m_wiggleEndTime
DT_RopeKeyframe!0x0b0c m_wiggleMaxLen
DT_RopeKeyframe!0x0b10 m_wiggleMagnitude
DT_RopeKeyframe!0x0b14 m_wiggleSpeed
DT_RopeKeyframe!0x0b4c m_flScrollSpeed
DT_RopeKeyframe!0x0b50 m_RopeFlags
DT_RopeKeyframe!0x0b54 m_iRopeMaterialModelIndex
DT_RopeKeyframe!0x0dd8 m_nSegments
DT_RopeKeyframe!0x0ddc m_hStartPoint
DT_RopeKeyframe!0x0de0 m_hEndPoint
DT_RopeKeyframe!0x0de4 m_hPrevPoint
DT_RopeKeyframe!0x0de8 m_iStartAttachment
DT_RopeKeyframe!0x0dea m_iEndAttachment
DT_RopeKeyframe!0x0e14 m_subdivStackCount
DT_RopeKeyframe!0x0e18 m_subdivSliceCount
DT_RopeKeyframe!0x0e1c m_ropeLength
DT_RopeKeyframe!0x0e24 m_constraintIterations
DT_RopeKeyframe!0x0e28 m_ropeDampening
DT_RopeKeyframe!0x0e2c m_Slack
DT_RopeKeyframe!0x0e30 m_TextureScale
DT_RopeKeyframe!0x0e30 m_TextureScale
DT_RopeKeyframe!0x0e34 m_fLockedPoints
DT_RopeKeyframe!0x0e38 m_lockDirectionCutoffLength
DT_RopeKeyframe!0x0e3c m_lockDirectionStrength
DT_RopeKeyframe!0x0e40 m_nChangeCount
DT_RopeKeyframe!0x0e44 m_Width
DT_RopeKeyframe!0x0ed8 m_bConstrainBetweenEndpoints
```
</details>
<details>
<summary><code>class DT_ScriptMover extends DT_ScriptProp</code></summary>

```
{
	m_parentAttachmentType: Int,
	m_vecAngVelocity: Vector,
	m_vecVelocity: Vector,
	m_localAngles: Vector,
	m_parentAttachmentHitbox: Int,
}
```

### Offsets

```
DT_ScriptMover!0x0020 m_parentAttachmentType
DT_ScriptMover!0x0128 m_vecAngVelocity
DT_ScriptMover!0x041c m_vecVelocity
DT_ScriptMover!0x0428 m_localAngles
DT_ScriptMover!0x07f4 m_parentAttachmentHitbox
```
</details>
<details>
<summary><code>class DT_ScriptMoverLightweight</code></summary>

```
{
	moveparent: Int,
	m_parentAttachmentType: Int,
	m_fEffects: Int,
	m_moverNetworkCellX: Int,
	m_moverNetworkCellY: Int,
	m_moverNetworkCellZ: Int,
	m_moverNetworkLocalOrigin: Vector,
	m_nModelIndex: Int,
	m_moverNetworkAngularVelocity: Vector,
	m_networkedFlags: Int,
	m_moverNetworkLinearVelocity: Vector,
	m_moverNetworkLocalAngles: Vector,
	m_scriptNameIndex: Int,
	m_parentAttachmentIndex: Int,
	m_parentAttachmentHitbox: Int,
	m_parentAttachmentModel: Int,
	m_fadeDist: Float,
	m_moveModeNonPhysics: Int,
	m_moveModeIsLocal: Int,
	m_moveToStartPos: Vector,
	m_moveToEndPos: Vector,
	m_moveToTimeStart: Time,
	m_moveToTimeEnd: Time,
	m_moveToTimeEaseIn: Float,
	m_moveToTimeEaseOut: Float,
	m_moveVelocity: Vector,
	m_moveGravity: Vector,
	m_trainStartTime: Time,
	m_trainStopTime: Time,
	m_trainStartDistance: Float,
	m_trainCurrentNode: Int,
	m_trainStopNode: Int,
	m_trainInitialSpeed: Float,
	m_trainGoalSpeed: Float,
	m_trainAcceleration: Float,
	m_trainLastNode: Int,
	m_trainLastDistance: Float,
	m_trainLastSpeed: Float,
	m_trainFollowMover: Int,
	m_trainFollowDistance: Float,
	m_trainBreadcrumb: DataTable,
	m_trainBreadcrumbBegin: Int,
	m_trainBreadcrumbCount: Int,
	m_trainAutoRollStrength: Float,
	m_trainAutoRollLookAheadDistance: Float,
	m_trainAutoRollMax: Float,
	m_rotateModeNonPhysics: Int,
	m_rotateModeIsLocal: Int,
	m_RotateToAnglesStart: Vector,
	m_RotateToAnglesEnd: Vector,
	m_rotateToTimeStart: Time,
	m_rotateToTimeEnd: Time,
	m_rotateToTimeEaseIn: Float,
	m_rotateToTimeEaseOut: Float,
	m_rotateAxis: Vector,
	m_rotateSpeed: Float,
	m_useNonPhysicsMoveInterpolation: Int,
}
```

### Offsets

```
DT_ScriptMoverLightweight!0x001c moveparent
DT_ScriptMoverLightweight!0x0020 m_parentAttachmentType
DT_ScriptMoverLightweight!0x0044 m_fEffects
DT_ScriptMoverLightweight!0x004c m_moverNetworkCellX
DT_ScriptMoverLightweight!0x0050 m_moverNetworkCellY
DT_ScriptMoverLightweight!0x0054 m_moverNetworkCellZ
DT_ScriptMoverLightweight!0x0058 m_moverNetworkLocalOrigin
DT_ScriptMoverLightweight!0x0064 m_nModelIndex
DT_ScriptMoverLightweight!0x0128 m_moverNetworkAngularVelocity
DT_ScriptMoverLightweight!0x0394 m_networkedFlags
DT_ScriptMoverLightweight!0x041c m_moverNetworkLinearVelocity
DT_ScriptMoverLightweight!0x0428 m_moverNetworkLocalAngles
DT_ScriptMoverLightweight!0x0628 m_scriptNameIndex
DT_ScriptMoverLightweight!0x07f0 m_parentAttachmentIndex
DT_ScriptMoverLightweight!0x07f4 m_parentAttachmentHitbox
DT_ScriptMoverLightweight!0x07f8 m_parentAttachmentModel
DT_ScriptMoverLightweight!0x0804 m_fadeDist
DT_ScriptMoverLightweight!0x1444 m_moveModeNonPhysics
DT_ScriptMoverLightweight!0x1448 m_moveModeIsLocal
DT_ScriptMoverLightweight!0x144c m_moveToStartPos
DT_ScriptMoverLightweight!0x1458 m_moveToEndPos
DT_ScriptMoverLightweight!0x1464 m_moveToTimeStart
DT_ScriptMoverLightweight!0x1468 m_moveToTimeEnd
DT_ScriptMoverLightweight!0x146c m_moveToTimeEaseIn
DT_ScriptMoverLightweight!0x1470 m_moveToTimeEaseOut
DT_ScriptMoverLightweight!0x1474 m_moveVelocity
DT_ScriptMoverLightweight!0x1480 m_moveGravity
DT_ScriptMoverLightweight!0x148c m_trainStartTime
DT_ScriptMoverLightweight!0x1490 m_trainStopTime
DT_ScriptMoverLightweight!0x1494 m_trainStartDistance
DT_ScriptMoverLightweight!0x1498 m_trainCurrentNode
DT_ScriptMoverLightweight!0x149c m_trainStopNode
DT_ScriptMoverLightweight!0x14a0 m_trainInitialSpeed
DT_ScriptMoverLightweight!0x14a4 m_trainGoalSpeed
DT_ScriptMoverLightweight!0x14a8 m_trainAcceleration
DT_ScriptMoverLightweight!0x14ac m_trainLastNode
DT_ScriptMoverLightweight!0x14b0 m_trainLastDistance
DT_ScriptMoverLightweight!0x14b4 m_trainLastSpeed
DT_ScriptMoverLightweight!0x14b8 m_trainFollowMover
DT_ScriptMoverLightweight!0x14bc m_trainFollowDistance
DT_ScriptMoverLightweight!0x14c0 m_trainBreadcrumb
DT_ScriptMoverLightweight!0x14e0 m_trainBreadcrumbBegin
DT_ScriptMoverLightweight!0x14e4 m_trainBreadcrumbCount
DT_ScriptMoverLightweight!0x14e8 m_trainAutoRollStrength
DT_ScriptMoverLightweight!0x14ec m_trainAutoRollLookAheadDistance
DT_ScriptMoverLightweight!0x14f0 m_trainAutoRollMax
DT_ScriptMoverLightweight!0x14f4 m_rotateModeNonPhysics
DT_ScriptMoverLightweight!0x14f8 m_rotateModeIsLocal
DT_ScriptMoverLightweight!0x14fc m_RotateToAnglesStart
DT_ScriptMoverLightweight!0x1508 m_RotateToAnglesEnd
DT_ScriptMoverLightweight!0x1514 m_rotateToTimeStart
DT_ScriptMoverLightweight!0x1518 m_rotateToTimeEnd
DT_ScriptMoverLightweight!0x151c m_rotateToTimeEaseIn
DT_ScriptMoverLightweight!0x1520 m_rotateToTimeEaseOut
DT_ScriptMoverLightweight!0x1524 m_rotateAxis
DT_ScriptMoverLightweight!0x1530 m_rotateSpeed
DT_ScriptMoverLightweight!0x1544 m_useNonPhysicsMoveInterpolation
```
</details>
<details>
<summary><code>class DT_ScriptMoverTrainNode</code></summary>

```
{
	m_cellX: Int,
	m_cellY: Int,
	m_cellZ: Int,
	m_localOrigin: Vector,
	m_scriptNameIndex: Int,
	m_firstChildEntityLink: Int,
	m_firstParentEntityLink: Int,
	m_numSmoothPoints: Int,
	m_trainNodeMakeSmoothPointsParity: Int,
	m_tangentType: Int,
	m_perfectCircularRotation: Int,
}
```

### Offsets

```
DT_ScriptMoverTrainNode!0x004c m_cellX
DT_ScriptMoverTrainNode!0x0050 m_cellY
DT_ScriptMoverTrainNode!0x0054 m_cellZ
DT_ScriptMoverTrainNode!0x0058 m_localOrigin
DT_ScriptMoverTrainNode!0x0628 m_scriptNameIndex
DT_ScriptMoverTrainNode!0x09e0 m_firstChildEntityLink
DT_ScriptMoverTrainNode!0x09e4 m_firstParentEntityLink
DT_ScriptMoverTrainNode!0x0a00 m_numSmoothPoints
DT_ScriptMoverTrainNode!0x0a04 m_trainNodeMakeSmoothPointsParity
DT_ScriptMoverTrainNode!0x0a08 m_tangentType
DT_ScriptMoverTrainNode!0x0a0c m_perfectCircularRotation
```
</details>
<details>
<summary><code>class DT_ScriptNetData_SNDC_DEATH_BOX extends DT_ScriptNetData</code></summary>

```
{
	m_bools: Array,
	m_ranges: Array,
	m_int32s: Array,
	m_times: Array,
	m_entities: Array,
	m_bools[0]: Int,
	m_ranges[0]: Int,
	m_int32s[0]: Int,
	m_times[0]: Time,
	m_entities[0]: Int,
}
```

### Offsets

```
DT_ScriptNetData_SNDC_DEATH_BOX!0x0000 m_bools
DT_ScriptNetData_SNDC_DEATH_BOX!0x0000 m_ranges
DT_ScriptNetData_SNDC_DEATH_BOX!0x0000 m_int32s
DT_ScriptNetData_SNDC_DEATH_BOX!0x0000 m_times
DT_ScriptNetData_SNDC_DEATH_BOX!0x0000 m_entities
DT_ScriptNetData_SNDC_DEATH_BOX!0x0c40 m_bools[0]
DT_ScriptNetData_SNDC_DEATH_BOX!0x0c42 m_ranges[0]
DT_ScriptNetData_SNDC_DEATH_BOX!0x0c64 m_int32s[0]
DT_ScriptNetData_SNDC_DEATH_BOX!0x0c68 m_times[0]
DT_ScriptNetData_SNDC_DEATH_BOX!0x0c6c m_entities[0]
```
</details>
<details>
<summary><code>class DT_ScriptNetData_SNDC_GLOBAL extends DT_ScriptNetData</code></summary>

```
{
	m_bools: Array,
	m_ranges: Array,
	m_int32s: Array,
	m_times: Array,
	m_entities: Array,
	m_bools[0]: Int,
	m_ranges[0]: Int,
	m_int32s[0]: Int,
	m_times[0]: Time,
	m_entities[0]: Int,
}
```

### Offsets

```
DT_ScriptNetData_SNDC_GLOBAL!0x0000 m_bools
DT_ScriptNetData_SNDC_GLOBAL!0x0000 m_ranges
DT_ScriptNetData_SNDC_GLOBAL!0x0000 m_int32s
DT_ScriptNetData_SNDC_GLOBAL!0x0000 m_times
DT_ScriptNetData_SNDC_GLOBAL!0x0000 m_entities
DT_ScriptNetData_SNDC_GLOBAL!0x0c40 m_bools[0]
DT_ScriptNetData_SNDC_GLOBAL!0x0c50 m_ranges[0]
DT_ScriptNetData_SNDC_GLOBAL!0x0c90 m_int32s[0]
DT_ScriptNetData_SNDC_GLOBAL!0x0cb0 m_times[0]
DT_ScriptNetData_SNDC_GLOBAL!0x0d10 m_entities[0]
```
</details>
<details>
<summary><code>class DT_ScriptNetData_SNDC_PLAYER_EXCLUSIVE extends DT_ScriptNetData</code></summary>

```
{
	m_bools: Array,
	m_ranges: Array,
	m_int32s: Array,
	m_times: Array,
	m_entities: Array,
	m_bools[0]: Int,
	m_ranges[0]: Int,
	m_int32s[0]: Int,
	m_times[0]: Time,
	m_entities[0]: Int,
}
```

### Offsets

```
DT_ScriptNetData_SNDC_PLAYER_EXCLUSIVE!0x0000 m_bools
DT_ScriptNetData_SNDC_PLAYER_EXCLUSIVE!0x0000 m_ranges
DT_ScriptNetData_SNDC_PLAYER_EXCLUSIVE!0x0000 m_int32s
DT_ScriptNetData_SNDC_PLAYER_EXCLUSIVE!0x0000 m_times
DT_ScriptNetData_SNDC_PLAYER_EXCLUSIVE!0x0000 m_entities
DT_ScriptNetData_SNDC_PLAYER_EXCLUSIVE!0x0c40 m_bools[0]
DT_ScriptNetData_SNDC_PLAYER_EXCLUSIVE!0x0c50 m_ranges[0]
DT_ScriptNetData_SNDC_PLAYER_EXCLUSIVE!0x0c90 m_int32s[0]
DT_ScriptNetData_SNDC_PLAYER_EXCLUSIVE!0x0cb0 m_times[0]
DT_ScriptNetData_SNDC_PLAYER_EXCLUSIVE!0x0cd0 m_entities[0]
```
</details>
<details>
<summary><code>class DT_ScriptNetData_SNDC_PLAYER_GLOBAL extends DT_ScriptNetData</code></summary>

```
{
	m_bools: Array,
	m_ranges: Array,
	m_int32s: Array,
	m_times: Array,
	m_entities: Array,
	m_bools[0]: Int,
	m_ranges[0]: Int,
	m_int32s[0]: Int,
	m_times[0]: Time,
	m_entities[0]: Int,
}
```

### Offsets

```
DT_ScriptNetData_SNDC_PLAYER_GLOBAL!0x0000 m_bools
DT_ScriptNetData_SNDC_PLAYER_GLOBAL!0x0000 m_ranges
DT_ScriptNetData_SNDC_PLAYER_GLOBAL!0x0000 m_int32s
DT_ScriptNetData_SNDC_PLAYER_GLOBAL!0x0000 m_times
DT_ScriptNetData_SNDC_PLAYER_GLOBAL!0x0000 m_entities
DT_ScriptNetData_SNDC_PLAYER_GLOBAL!0x0c40 m_bools[0]
DT_ScriptNetData_SNDC_PLAYER_GLOBAL!0x0c48 m_ranges[0]
DT_ScriptNetData_SNDC_PLAYER_GLOBAL!0x0c88 m_int32s[0]
DT_ScriptNetData_SNDC_PLAYER_GLOBAL!0x0cb8 m_times[0]
DT_ScriptNetData_SNDC_PLAYER_GLOBAL!0x0cd8 m_entities[0]
```
</details>
<details>
<summary><code>class DT_ScriptNetData_SNDC_TITAN_SOUL extends DT_ScriptNetData</code></summary>

```
{
	m_bools: Array,
	m_ranges: Array,
	m_int32s: Array,
	m_times: Array,
	m_entities: Array,
	m_bools[0]: Int,
	m_ranges[0]: Int,
	m_int32s[0]: Int,
	m_times[0]: Time,
	m_entities[0]: Int,
}
```

### Offsets

```
DT_ScriptNetData_SNDC_TITAN_SOUL!0x0000 m_bools
DT_ScriptNetData_SNDC_TITAN_SOUL!0x0000 m_ranges
DT_ScriptNetData_SNDC_TITAN_SOUL!0x0000 m_int32s
DT_ScriptNetData_SNDC_TITAN_SOUL!0x0000 m_times
DT_ScriptNetData_SNDC_TITAN_SOUL!0x0000 m_entities
DT_ScriptNetData_SNDC_TITAN_SOUL!0x0c40 m_bools[0]
DT_ScriptNetData_SNDC_TITAN_SOUL!0x0c48 m_ranges[0]
DT_ScriptNetData_SNDC_TITAN_SOUL!0x0c68 m_int32s[0]
DT_ScriptNetData_SNDC_TITAN_SOUL!0x0c70 m_times[0]
DT_ScriptNetData_SNDC_TITAN_SOUL!0x0c90 m_entities[0]
```
</details>
<details>
<summary><code>class DT_ScriptProp extends DT_DynamicProp</code></summary>

```
{
	m_networkedFlags: Int,
	m_iHealth: Int,
	m_iMaxHealth: Int,
	m_minimapData: DT_MinimapBaseEntityData,
	m_nameVisibilityFlags: Int,
	m_title: String,
	m_footstepType: String,
	m_renderColorFriendlyIsValid: Int,
	m_renderColorFriendly: Int,
	m_armorType: Int,
	m_scriptPropFlags: Int,
	m_scriptPropSmartAmmoLockType: Int,
}
```

### Offsets

```
DT_ScriptProp!0x0394 m_networkedFlags
DT_ScriptProp!0x03e0 m_iHealth
DT_ScriptProp!0x0510 m_iMaxHealth
DT_ScriptProp!0x0908 m_minimapData
DT_ScriptProp!0x0958 m_nameVisibilityFlags
DT_ScriptProp!0x1360 m_title
DT_ScriptProp!0x1380 m_footstepType
DT_ScriptProp!0x13c0 m_renderColorFriendlyIsValid
DT_ScriptProp!0x13c1 m_renderColorFriendly
DT_ScriptProp!0x13c8 m_armorType
DT_ScriptProp!0x13cc m_scriptPropFlags
DT_ScriptProp!0x13d0 m_scriptPropSmartAmmoLockType
```
</details>
<details>
<summary><code>class DT_ScriptTraceVolume extends DT_BaseEntity</code></summary>

```
{
	m_shapeType: Int,
	m_sphereRadius: Float,
	m_boxMins: Vector,
	m_boxMaxs: Vector,
	m_drawDebug: Int,
}
```

### Offsets

```
DT_ScriptTraceVolume!0x0a00 m_shapeType
DT_ScriptTraceVolume!0x0a04 m_sphereRadius
DT_ScriptTraceVolume!0x0a08 m_boxMins
DT_ScriptTraceVolume!0x0a14 m_boxMaxs
DT_ScriptTraceVolume!0x0a20 m_drawDebug
```
</details>
<details>
<summary><code>class DT_SequenceTransitioner</code></summary>

```
{
	m_sequenceTransitionerLayers: DataTable,
	m_sequenceTransitionerLayerCount: Int,
}
```

### Offsets

```
DT_SequenceTransitioner!0x0050 m_sequenceTransitionerLayers
DT_SequenceTransitioner!0x01a0 m_sequenceTransitionerLayerCount
```
</details>
<details>
<summary><code>class DT_SequenceTransitionerLayer</code></summary>

```
{
	m_sequenceTransitionerLayerActive: Int,
	m_sequenceTransitionerLayerStartCycle: Cycle,
	m_sequenceTransitionerLayerSequence: Int,
	m_sequenceTransitionerLayerPlaybackRate: Float,
	m_sequenceTransitionerLayerStartTime: Time,
	m_sequenceTransitionerLayerFadeOutDuration: Cycle,
}
```

### Offsets

```
DT_SequenceTransitionerLayer!0x0018 m_sequenceTransitionerLayerActive
DT_SequenceTransitionerLayer!0x001c m_sequenceTransitionerLayerStartCycle
DT_SequenceTransitionerLayer!0x0020 m_sequenceTransitionerLayerSequence
DT_SequenceTransitionerLayer!0x0028 m_sequenceTransitionerLayerPlaybackRate
DT_SequenceTransitionerLayer!0x002c m_sequenceTransitionerLayerStartTime
DT_SequenceTransitionerLayer!0x0030 m_sequenceTransitionerLayerFadeOutDuration
```
</details>
<details>
<summary><code>class DT_SoundData</code></summary>

```
{
	m_targetEnt: Int,
	m_soundID: BitMask,
	m_networkTableID: Int,
	m_soundIsStart: Int,
	m_seek: Float,
}
```

### Offsets

```
DT_SoundData!0x0000 m_targetEnt
DT_SoundData!0x0008 m_soundID
DT_SoundData!0x0010 m_networkTableID
DT_SoundData!0x0014 m_soundIsStart
DT_SoundData!0x0018 m_seek
```
</details>
<details>
<summary><code>class DT_StatueProp extends DT_PhysicsProp</code></summary>

```
{
	m_hInitBaseAnimating: Int,
	m_bShatter: Int,
	m_nShatterFlags: Int,
	m_vShatterPosition: Vector,
	m_vShatterForce: Vector,
}
```

### Offsets

```
DT_StatueProp!0x1380 m_hInitBaseAnimating
DT_StatueProp!0x1384 m_bShatter
DT_StatueProp!0x1388 m_nShatterFlags
DT_StatueProp!0x138c m_vShatterPosition
DT_StatueProp!0x1398 m_vShatterForce
```
</details>
<details>
<summary><code>class DT_StatusEffectPlugin</code></summary>

```
{
	m_hOwnerEntity: Int,
	m_statusEffectsTimedPluginNV: DataTable,
	m_statusEffectsEndlessPluginNV: DataTable,
}
```

### Offsets

```
DT_StatusEffectPlugin!0x043c m_hOwnerEntity
DT_StatusEffectPlugin!0x0a00 m_statusEffectsTimedPluginNV
DT_StatusEffectPlugin!0x0a18 m_statusEffectsEndlessPluginNV
```
</details>
<details>
<summary><code>class DT_TEBreakModel extends DT_BaseTempEntity</code></summary>

```
{
	m_vecOrigin: Vector,
	m_angRotation.x: Float,
	m_angRotation.y: Float,
	m_angRotation.z: Float,
	m_vecSize: Vector,
	m_vecVelocity: Vector,
	m_nRandomization: Int,
	m_nModelIndex: Int,
	m_nCount: Int,
	m_fTime: Float,
	m_nFlags: Int,
}
```

### Offsets

```
DT_TEBreakModel!0x0028 m_vecOrigin
DT_TEBreakModel!0x0034 m_angRotation.x
DT_TEBreakModel!0x0038 m_angRotation.y
DT_TEBreakModel!0x003c m_angRotation.z
DT_TEBreakModel!0x0040 m_vecSize
DT_TEBreakModel!0x004c m_vecVelocity
DT_TEBreakModel!0x0058 m_nRandomization
DT_TEBreakModel!0x005c m_nModelIndex
DT_TEBreakModel!0x0060 m_nCount
DT_TEBreakModel!0x0064 m_fTime
DT_TEBreakModel!0x0068 m_nFlags
```
</details>
<details>
<summary><code>class DT_TEExplosion extends DT_TEParticleSystem</code></summary>

```
{
	m_fScale: Float,
	m_nFrameRate: Int,
	m_nFlags: Int,
	m_vecNormal: Vector,
	m_chMaterialType: Int,
	m_nRadius: Int,
	m_nInnerRadius: Int,
	m_nMagnitude: Int,
	m_impactEffectTableIndex: Int,
	m_surfaceProp: Int,
	m_owner: Int,
	m_victim: Int,
}
```

### Offsets

```
DT_TEExplosion!0x0038 m_fScale
DT_TEExplosion!0x003c m_nFrameRate
DT_TEExplosion!0x0040 m_nFlags
DT_TEExplosion!0x0044 m_vecNormal
DT_TEExplosion!0x0050 m_chMaterialType
DT_TEExplosion!0x0054 m_nRadius
DT_TEExplosion!0x0058 m_nInnerRadius
DT_TEExplosion!0x005c m_nMagnitude
DT_TEExplosion!0x0060 m_impactEffectTableIndex
DT_TEExplosion!0x0064 m_surfaceProp
DT_TEExplosion!0x0068 m_owner
DT_TEExplosion!0x006c m_victim
```
</details>
<details>
<summary><code>class DT_TEPhysicsProp extends DT_BaseTempEntity</code></summary>

```
{
	m_vecOrigin: Vector,
	m_angRotation.x: Float,
	m_angRotation.y: Float,
	m_angRotation.z: Float,
	m_vecVelocity: Vector,
	m_nModelIndex: Int,
	m_nSkin: Int,
	m_nFlags: Int,
	m_nEffects: Int,
}
```

### Offsets

```
DT_TEPhysicsProp!0x0028 m_vecOrigin
DT_TEPhysicsProp!0x0034 m_angRotation.x
DT_TEPhysicsProp!0x0038 m_angRotation.y
DT_TEPhysicsProp!0x003c m_angRotation.z
DT_TEPhysicsProp!0x0040 m_vecVelocity
DT_TEPhysicsProp!0x004c m_nModelIndex
DT_TEPhysicsProp!0x0050 m_nSkin
DT_TEPhysicsProp!0x0054 m_nFlags
DT_TEPhysicsProp!0x0058 m_nEffects
```
</details>
<details>
<summary><code>class DT_TEProjectileTrail extends DT_BaseTempEntity</code></summary>

```
{
	m_owner: Int,
	m_startPos: Vector,
	m_endPos: Vector,
	m_weaponClassIndex: Int,
	m_modBitfield: Int,
	m_projectileTrailIndex: Int,
	m_impactEffectTable: Int,
}
```

### Offsets

```
DT_TEProjectileTrail!0x0028 m_owner
DT_TEProjectileTrail!0x002c m_startPos
DT_TEProjectileTrail!0x0038 m_endPos
DT_TEProjectileTrail!0x0044 m_weaponClassIndex
DT_TEProjectileTrail!0x0048 m_modBitfield
DT_TEProjectileTrail!0x004c m_projectileTrailIndex
DT_TEProjectileTrail!0x0050 m_impactEffectTable
```
</details>
<details>
<summary><code>class DT_TEScriptParticleSystem extends DT_BaseTempEntity</code></summary>

```
{
	m_effectIndex: Int,
	m_origin: Vector,
	m_angles: Vector,
	m_controlPoint1: Vector,
}
```

### Offsets

```
DT_TEScriptParticleSystem!0x0028 m_effectIndex
DT_TEScriptParticleSystem!0x002c m_origin
DT_TEScriptParticleSystem!0x0038 m_angles
DT_TEScriptParticleSystem!0x0044 m_controlPoint1
```
</details>
<details>
<summary><code>class DT_TEScriptParticleSystemOnEntity extends DT_BaseTempEntity</code></summary>

```
{
	m_effectIndex: Int,
	m_ent: Int,
	m_attachType: Int,
	m_attachmentIndex: Int,
	m_attachType2: Int,
	m_attachmentIndex2: Int,
}
```

### Offsets

```
DT_TEScriptParticleSystemOnEntity!0x0028 m_effectIndex
DT_TEScriptParticleSystemOnEntity!0x002c m_ent
DT_TEScriptParticleSystemOnEntity!0x0030 m_attachType
DT_TEScriptParticleSystemOnEntity!0x0034 m_attachmentIndex
DT_TEScriptParticleSystemOnEntity!0x0038 m_attachType2
DT_TEScriptParticleSystemOnEntity!0x003c m_attachmentIndex2
```
</details>
<details>
<summary><code>class DT_TEShatterSurface extends DT_BaseTempEntity</code></summary>

```
{
	m_vecOrigin: Vector,
	m_vecAngles: Vector,
	m_vecForce: Vector,
	m_vecForcePos: Vector,
	m_flWidth: Float,
	m_flHeight: Float,
	m_flShardSize: Float,
	m_nSurfaceType: Int,
}
```

### Offsets

```
DT_TEShatterSurface!0x0028 m_vecOrigin
DT_TEShatterSurface!0x0034 m_vecAngles
DT_TEShatterSurface!0x0040 m_vecForce
DT_TEShatterSurface!0x004c m_vecForcePos
DT_TEShatterSurface!0x0058 m_flWidth
DT_TEShatterSurface!0x005c m_flHeight
DT_TEShatterSurface!0x0060 m_flShardSize
DT_TEShatterSurface!0x0070 m_nSurfaceType
```
</details>
<details>
<summary><code>class DT_Team</code></summary>

```
{
	player_array_element: Int,
	"player_array": Array,
	m_score: Int,
	m_score2: Int,
	m_kills: Int,
	m_deaths: Int,
	m_iRoundsWon: Int,
	m_iTeamTeamNum: Int,
	m_szTeamname: String,
	m_reservedPlayerCount: Int,
	m_connectingPlayerCount: Int,
	m_loadingPlayerCount: Int,
}
```

### Offsets

```
DT_Team!0x0000 player_array_element
DT_Team!0x0000 "player_array"
DT_Team!0x0a00 m_score
DT_Team!0x0a04 m_score2
DT_Team!0x0a08 m_kills
DT_Team!0x0a0c m_deaths
DT_Team!0x0a10 m_iRoundsWon
DT_Team!0x0a14 m_iTeamTeamNum
DT_Team!0x0a38 m_szTeamname
DT_Team!0x0b38 m_reservedPlayerCount
DT_Team!0x0b3c m_connectingPlayerCount
DT_Team!0x0b40 m_loadingPlayerCount
```
</details>
<details>
<summary><code>class DT_ThirdPersonView</code></summary>

```
{
	m_thirdPersonEntViewOffset.x: Float,
	m_thirdPersonEntViewOffset.y: Float,
	m_thirdPersonEntViewOffset.z: Float,
	m_thirdPersonEntShouldViewAnglesFollowThirdPersonEnt: Int,
	m_thirdPersonEntPitchIsFreelook: Int,
	m_thirdPersonEntYawIsFreelook: Int,
	m_thirdPersonEntUseFixedDist: Int,
	m_thirdPersonEntFixedClientOnly: Int,
	m_thirdPersonEntPushedInByGeo: Int,
	m_thirdPersonEntDrawViewmodel: Int,
	m_thirdPersonEntBlendInTotalDuration: Float,
	m_thirdPersonEntBlendInEaseInDuration: Float,
	m_thirdPersonEntBlendInEaseOutDuration: Float,
	m_thirdPersonEntBlendOutDuration: Float,
	m_thirdPersonEntFixedPitch: Float,
	m_thirdPersonEntFixedYaw: Float,
	m_thirdPersonEntFixedDist: Float,
	m_thirdPersonEntFixedHeight: Float,
	m_thirdPersonEntFixedRight: Float,
	m_thirdPersonEntMinYaw: Float,
	m_thirdPersonEntMaxYaw: Float,
	m_thirdPersonEntMinPitch: Float,
	m_thirdPersonEntMaxPitch: Float,
	m_thirdPersonEntSpringToCenterRate: Float,
	m_thirdPersonEntLookaheadLowerEntSpeed: Float,
	m_thirdPersonEntLookaheadUpperEntSpeed: Float,
	m_thirdPersonEntLookaheadMaxAngle: Float,
	m_thirdPersonEntLookaheadLerpAheadRate: Float,
	m_thirdPersonEntLookaheadLerpToCenterRate: Float,
}
```

### Offsets

```
DT_ThirdPersonView!0x0000 m_thirdPersonEntViewOffset.x
DT_ThirdPersonView!0x0004 m_thirdPersonEntViewOffset.y
DT_ThirdPersonView!0x0008 m_thirdPersonEntViewOffset.z
DT_ThirdPersonView!0x000c m_thirdPersonEntShouldViewAnglesFollowThirdPersonEnt
DT_ThirdPersonView!0x000d m_thirdPersonEntPitchIsFreelook
DT_ThirdPersonView!0x000e m_thirdPersonEntYawIsFreelook
DT_ThirdPersonView!0x000f m_thirdPersonEntUseFixedDist
DT_ThirdPersonView!0x0010 m_thirdPersonEntFixedClientOnly
DT_ThirdPersonView!0x0011 m_thirdPersonEntPushedInByGeo
DT_ThirdPersonView!0x0012 m_thirdPersonEntDrawViewmodel
DT_ThirdPersonView!0x0014 m_thirdPersonEntBlendInTotalDuration
DT_ThirdPersonView!0x0018 m_thirdPersonEntBlendInEaseInDuration
DT_ThirdPersonView!0x001c m_thirdPersonEntBlendInEaseOutDuration
DT_ThirdPersonView!0x0020 m_thirdPersonEntBlendOutDuration
DT_ThirdPersonView!0x0024 m_thirdPersonEntFixedPitch
DT_ThirdPersonView!0x0028 m_thirdPersonEntFixedYaw
DT_ThirdPersonView!0x002c m_thirdPersonEntFixedDist
DT_ThirdPersonView!0x0030 m_thirdPersonEntFixedHeight
DT_ThirdPersonView!0x0034 m_thirdPersonEntFixedRight
DT_ThirdPersonView!0x004c m_thirdPersonEntMinYaw
DT_ThirdPersonView!0x0050 m_thirdPersonEntMaxYaw
DT_ThirdPersonView!0x0054 m_thirdPersonEntMinPitch
DT_ThirdPersonView!0x0058 m_thirdPersonEntMaxPitch
DT_ThirdPersonView!0x005c m_thirdPersonEntSpringToCenterRate
DT_ThirdPersonView!0x0060 m_thirdPersonEntLookaheadLowerEntSpeed
DT_ThirdPersonView!0x0064 m_thirdPersonEntLookaheadUpperEntSpeed
DT_ThirdPersonView!0x0068 m_thirdPersonEntLookaheadMaxAngle
DT_ThirdPersonView!0x006c m_thirdPersonEntLookaheadLerpAheadRate
DT_ThirdPersonView!0x0070 m_thirdPersonEntLookaheadLerpToCenterRate
```
</details>
<details>
<summary><code>class DT_TitanSoul</code></summary>

```
{
	statuseffectsdata_soul: DT_TitanSoul_StatusEffects,
	m_bossPlayer: Int,
	m_shieldHealth: Int,
	m_shieldHealthMax: Int,
	m_networkedFlags: Int,
	m_titan: Int,
	m_titanSoulScriptNetData: Int,
	m_lastRodeoHitTime: Time,
	m_nextCoreChargeAvailable: Time,
	m_coreChargeExpireTime: Time,
	m_coreChargeStartTime: Time,
	m_coreUseDuration: Time,
	m_damageComboLatestUpdateTime: Time,
	m_damageComboStartHealth: Int,
	m_stance: Int,
	m_doomed: Int,
	m_playerSettingsNum: BitMask,
	m_invalidHealthBarEnt: Int,
	m_bEjecting: Int,
	m_isValidRodeoTarget: Int,
}
```

### Offsets

```
DT_TitanSoul!0x0000 statuseffectsdata_soul
DT_TitanSoul!0x0124 m_bossPlayer
DT_TitanSoul!0x0170 m_shieldHealth
DT_TitanSoul!0x0174 m_shieldHealthMax
DT_TitanSoul!0x0394 m_networkedFlags
DT_TitanSoul!0x0a00 m_titan
DT_TitanSoul!0x0a08 m_titanSoulScriptNetData
DT_TitanSoul!0x0ba0 m_lastRodeoHitTime
DT_TitanSoul!0x0ba8 m_nextCoreChargeAvailable
DT_TitanSoul!0x0bb0 m_coreChargeExpireTime
DT_TitanSoul!0x0bb8 m_coreChargeStartTime
DT_TitanSoul!0x0bbc m_coreUseDuration
DT_TitanSoul!0x0bc0 m_damageComboLatestUpdateTime
DT_TitanSoul!0x0bc4 m_damageComboStartHealth
DT_TitanSoul!0x0d68 m_stance
DT_TitanSoul!0x0d6c m_doomed
DT_TitanSoul!0x0d70 m_playerSettingsNum
DT_TitanSoul!0x0d78 m_invalidHealthBarEnt
DT_TitanSoul!0x0d79 m_bEjecting
DT_TitanSoul!0x0d7a m_isValidRodeoTarget
```
</details>
<details>
<summary><code>class DT_TitanSoul_StatusEffects</code></summary>

```
{
	m_statusEffectsTimedTitanSoulNV: DataTable,
	m_statusEffectsEndlessTitanSoulNV: DataTable,
}
```

### Offsets

```
DT_TitanSoul_StatusEffects!0x0bc8 m_statusEffectsTimedTitanSoulNV
DT_TitanSoul_StatusEffects!0x0cb8 m_statusEffectsEndlessTitanSoulNV
```
</details>
<details>
<summary><code>class DT_TriggerCylinderHeavy extends DT_BaseTrigger</code></summary>

```
{
	m_triggerFilterMask: BitMask,
	m_radius: Float,
	m_aboveHeight: Float,
	m_belowHeight: Float,
	m_teslaTrapBaseHeight: Float,
	m_vertOverride: Float,
	m_launchPower: Float,
	m_punchSoftAmount: Float,
	m_punchHardAmount: Float,
	m_punchRandomBoost: Float,
	m_triggerType: Int,
	m_teslaTrapFXVisible: Int,
	m_teslaTrapObstructedEndTime: Time,
	m_teslaTrapStart: Int,
	m_teslaTrapEnd: Int,
	m_teslaTrapUp: Vector,
	m_launchDir: Vector,
}
```

### Offsets

```
DT_TriggerCylinderHeavy!0x0a80 m_triggerFilterMask
DT_TriggerCylinderHeavy!0x0a88 m_radius
DT_TriggerCylinderHeavy!0x0a8c m_aboveHeight
DT_TriggerCylinderHeavy!0x0a90 m_belowHeight
DT_TriggerCylinderHeavy!0x0a94 m_teslaTrapBaseHeight
DT_TriggerCylinderHeavy!0x0a98 m_vertOverride
DT_TriggerCylinderHeavy!0x0a9c m_launchPower
DT_TriggerCylinderHeavy!0x0aa0 m_punchSoftAmount
DT_TriggerCylinderHeavy!0x0aa4 m_punchHardAmount
DT_TriggerCylinderHeavy!0x0aa8 m_punchRandomBoost
DT_TriggerCylinderHeavy!0x0aac m_triggerType
DT_TriggerCylinderHeavy!0x0ab0 m_teslaTrapFXVisible
DT_TriggerCylinderHeavy!0x0ab8 m_teslaTrapObstructedEndTime
DT_TriggerCylinderHeavy!0x0abc m_teslaTrapStart
DT_TriggerCylinderHeavy!0x0ac0 m_teslaTrapEnd
DT_TriggerCylinderHeavy!0x0ac4 m_teslaTrapUp
DT_TriggerCylinderHeavy!0x0ad0 m_launchDir
```
</details>
<details>
<summary><code>class DT_TriggerPointGravity extends DT_BaseTrigger</code></summary>

```
{
	m_pullOuterRadius: Float,
	m_pullInnerRadius: Float,
	m_reduceSpeedOuterRadius: Float,
	m_reduceSpeedInnerRadius: Float,
	m_pullAccel: Float,
	m_pullSpeed: Float,
}
```

### Offsets

```
DT_TriggerPointGravity!0x0a80 m_pullOuterRadius
DT_TriggerPointGravity!0x0a84 m_pullInnerRadius
DT_TriggerPointGravity!0x0a88 m_reduceSpeedOuterRadius
DT_TriggerPointGravity!0x0a8c m_reduceSpeedInnerRadius
DT_TriggerPointGravity!0x0a90 m_pullAccel
DT_TriggerPointGravity!0x0a94 m_pullSpeed
```
</details>
<details>
<summary><code>class DT_Turret extends DT_BaseAnimating</code></summary>

```
{
	m_iHealth: Int,
	m_iMaxHealth: Int,
	m_settingsIndex: Int,
	m_driver: Int,
	m_forceAimPitch: Float,
	m_forceAimYaw: Float,
	m_turretWeapon: Int,
	m_title: String,
}
```

### Offsets

```
DT_Turret!0x03e0 m_iHealth
DT_Turret!0x0510 m_iMaxHealth
DT_Turret!0x1308 m_settingsIndex
DT_Turret!0x1320 m_driver
DT_Turret!0x1348 m_forceAimPitch
DT_Turret!0x134c m_forceAimYaw
DT_Turret!0x1350 m_turretWeapon
DT_Turret!0x1354 m_title
```
</details>
<details>
<summary><code>class DT_VGuiScreen extends DT_BaseEntity</code></summary>

```
{
	m_flWidth: Float,
	m_flHeight: Float,
	m_nPanelName: Int,
	m_nAttachmentIndex: Int,
	m_nOverlayMaterial: Int,
	m_fScreenFlags: Int,
	m_hPlayerOwner: Int,
}
```

### Offsets

```
DT_VGuiScreen!0x0a00 m_flWidth
DT_VGuiScreen!0x0a04 m_flHeight
DT_VGuiScreen!0x0a10 m_nPanelName
DT_VGuiScreen!0x0a2c m_nAttachmentIndex
DT_VGuiScreen!0x0a30 m_nOverlayMaterial
DT_VGuiScreen!0x0a34 m_fScreenFlags
DT_VGuiScreen!0x0ab8 m_hPlayerOwner
```
</details>
<details>
<summary><code>class DT_VehicleDriverExclusive</code></summary>

```
{
	m_localOrigin: VectorXY,
	m_localOrigin.z: Float,
	m_pushedFixedPointOffset: DataTable,
}
```

### Offsets

```
DT_VehicleDriverExclusive!0x0004 m_localOrigin
DT_VehicleDriverExclusive!0x000c m_localOrigin.z
DT_VehicleDriverExclusive!0x1374 m_pushedFixedPointOffset
```
</details>
<details>
<summary><code>class DT_VehicleNonDriverExclusive</code></summary>

```
{
	m_cellX: Int,
	m_cellY: Int,
	m_cellZ: Int,
	m_localOrigin: VectorXY,
	m_localOrigin.z: Float,
}
```

### Offsets

```
DT_VehicleNonDriverExclusive!0x004c m_cellX
DT_VehicleNonDriverExclusive!0x0050 m_cellY
DT_VehicleNonDriverExclusive!0x0054 m_cellZ
DT_VehicleNonDriverExclusive!0x0058 m_localOrigin
DT_VehicleNonDriverExclusive!0x0060 m_localOrigin.z
```
</details>
<details>
<summary><code>class DT_VortexSphere extends DT_BaseEntity</code></summary>

```
{
	m_spawnflags: Int,
	m_iHealth: Int,
	m_enabled: Int,
	m_radius: Float,
	m_height: Float,
	m_bulletFov: Float,
	m_bulletAbsorbedCount: Int,
	m_projectileAbsorbedCount: Int,
	m_ownerWeapon: Int,
	m_vortexEffect: Int,
	m_vortexLocalAngles: Vector,
	m_gunAttachment: String,
}
```

### Offsets

```
DT_VortexSphere!0x0094 m_spawnflags
DT_VortexSphere!0x03e0 m_iHealth
DT_VortexSphere!0x0a00 m_enabled
DT_VortexSphere!0x0a04 m_radius
DT_VortexSphere!0x0a08 m_height
DT_VortexSphere!0x0a0c m_bulletFov
DT_VortexSphere!0x0a10 m_bulletAbsorbedCount
DT_VortexSphere!0x0a14 m_projectileAbsorbedCount
DT_VortexSphere!0x0a18 m_ownerWeapon
DT_VortexSphere!0x0a1c m_vortexEffect
DT_VortexSphere!0x0a20 m_vortexLocalAngles
DT_VortexSphere!0x0a30 m_gunAttachment
```
</details>
<details>
<summary><code>class DT_WeaponInventory</code></summary>

```
{
	weapons: DataTable,
	offhandWeapons: DataTable,
	activeWeapons: DataTable,
}
```

### Offsets

```
DT_WeaponInventory!0x0008 weapons
DT_WeaponInventory!0x002c offhandWeapons
DT_WeaponInventory!0x0068 activeWeapons
```
</details>
<details>
<summary><code>class DT_WeaponInventoryActiveWeaponOnly</code></summary>

```
{
	activeWeapons: DataTable,
}
```

### Offsets

```
DT_WeaponInventoryActiveWeaponOnly!0x0068 activeWeapons
```
</details>
<details>
<summary><code>class DT_WeaponPlayerData</code></summary>

```
{
	m_moveSpread: Float,
	m_spreadStartTime: Time,
	m_spreadStartFracHip: Float,
	m_spreadStartFracADS: Float,
	m_kickSpreadHipfire: Float,
	m_kickSpreadADS: Float,
	m_kickTime: Time,
	m_kickScaleBasePitch: Float,
	m_kickScaleBaseYaw: Float,
	m_kickPatternScaleBase: Float,
	m_kickSpringHeatBaseTime: Time,
	m_kickSpringHeatBaseValue: Float,
	m_semiAutoTriggerHoldTime: Time,
	m_semiAutoTriggerDown: Int,
	m_pendingTriggerPull: Int,
	m_semiAutoNeedsRechamber: Int,
	m_pendingReloadAttempt: Int,
	m_offhandHybridNormalMode: Int,
	m_pendingoffhandHybridToss: Int,
	m_fastHolster: Int,
	m_didFirstDeploy: Int,
	m_shouldCatch: Int,
	m_clipModelIsHidden: Int,
	m_segmentedReloadEndSeqRequired: Int,
	m_reloadStartedEmpty: Int,
	m_segmentedAnimStartedOneHanded: Int,
	m_segmentedReloadCanRestartLoop: Int,
	m_segmentedReloadLoopFireLocked: Int,
	m_realtimeModCmds: DataTable,
	m_realtimeModCmdHead: Int,
	m_realtimeModCmdCount: Int,
	m_customActivityAttachedModelIndex: Int,
	m_customActivityAttachedModelAttachmentIndex: Int,
	m_fireRateLerp_startTime: Time,
	m_fireRateLerp_startFraction: Float,
	m_fireRateLerp_stopTime: Time,
	m_fireRateLerp_stopFraction: Float,
	m_chargeAnimIndex: Int,
	m_chargeAnimIndexOld: Int,
	m_proScreen_owner: Int,
	m_proScreen_int0: Int,
	m_proScreen_int1: Int,
	m_proScreen_int2: Int,
	m_proScreen_float0: Float,
	m_proScreen_float1: Float,
	m_proScreen_float2: Float,
	m_reloadMilestone: Int,
	m_rechamberMilestone: Int,
	m_cooldownMilestone: Int,
	m_fullReloadStartTime: Time,
	m_scriptTime0: Time,
	m_scriptFlags0: Int,
	m_scriptInt0: Int,
	m_curZoomFOV: Float,
	m_targetZoomFOV: Float,
	m_zoomFOVLerpTime: Float,
	m_zoomFOVLerpEndTime: Time,
	m_latestDryfireTime: Time,
	m_requestedAttackEndTime: Time,
	m_currentAltFireAnimIndex: Int,
	m_legendaryModelIndex: Int,
	m_charmModelIndex: Int,
	m_charmAttachment: Int,
	m_charmScriptIndex: Int,
}
```

### Offsets

```
DT_WeaponPlayerData!0x0008 m_moveSpread
DT_WeaponPlayerData!0x000c m_spreadStartTime
DT_WeaponPlayerData!0x0010 m_spreadStartFracHip
DT_WeaponPlayerData!0x0014 m_spreadStartFracADS
DT_WeaponPlayerData!0x0018 m_kickSpreadHipfire
DT_WeaponPlayerData!0x001c m_kickSpreadADS
DT_WeaponPlayerData!0x0020 m_kickTime
DT_WeaponPlayerData!0x0024 m_kickScaleBasePitch
DT_WeaponPlayerData!0x0028 m_kickScaleBaseYaw
DT_WeaponPlayerData!0x002c m_kickPatternScaleBase
DT_WeaponPlayerData!0x0030 m_kickSpringHeatBaseTime
DT_WeaponPlayerData!0x0034 m_kickSpringHeatBaseValue
DT_WeaponPlayerData!0x0038 m_semiAutoTriggerHoldTime
DT_WeaponPlayerData!0x003c m_semiAutoTriggerDown
DT_WeaponPlayerData!0x003d m_pendingTriggerPull
DT_WeaponPlayerData!0x003e m_semiAutoNeedsRechamber
DT_WeaponPlayerData!0x003f m_pendingReloadAttempt
DT_WeaponPlayerData!0x0040 m_offhandHybridNormalMode
DT_WeaponPlayerData!0x0041 m_pendingoffhandHybridToss
DT_WeaponPlayerData!0x0042 m_fastHolster
DT_WeaponPlayerData!0x0043 m_didFirstDeploy
DT_WeaponPlayerData!0x0044 m_shouldCatch
DT_WeaponPlayerData!0x0045 m_clipModelIsHidden
DT_WeaponPlayerData!0x0046 m_segmentedReloadEndSeqRequired
DT_WeaponPlayerData!0x0047 m_reloadStartedEmpty
DT_WeaponPlayerData!0x0048 m_segmentedAnimStartedOneHanded
DT_WeaponPlayerData!0x0049 m_segmentedReloadCanRestartLoop
DT_WeaponPlayerData!0x004a m_segmentedReloadLoopFireLocked
DT_WeaponPlayerData!0x004b m_realtimeModCmds
DT_WeaponPlayerData!0x0053 m_realtimeModCmdHead
DT_WeaponPlayerData!0x0054 m_realtimeModCmdCount
DT_WeaponPlayerData!0x0058 m_customActivityAttachedModelIndex
DT_WeaponPlayerData!0x005c m_customActivityAttachedModelAttachmentIndex
DT_WeaponPlayerData!0x0060 m_fireRateLerp_startTime
DT_WeaponPlayerData!0x0064 m_fireRateLerp_startFraction
DT_WeaponPlayerData!0x0068 m_fireRateLerp_stopTime
DT_WeaponPlayerData!0x006c m_fireRateLerp_stopFraction
DT_WeaponPlayerData!0x0070 m_chargeAnimIndex
DT_WeaponPlayerData!0x0074 m_chargeAnimIndexOld
DT_WeaponPlayerData!0x0078 m_proScreen_owner
DT_WeaponPlayerData!0x007c m_proScreen_int0
DT_WeaponPlayerData!0x0080 m_proScreen_int1
DT_WeaponPlayerData!0x0084 m_proScreen_int2
DT_WeaponPlayerData!0x0088 m_proScreen_float0
DT_WeaponPlayerData!0x008c m_proScreen_float1
DT_WeaponPlayerData!0x0090 m_proScreen_float2
DT_WeaponPlayerData!0x0094 m_reloadMilestone
DT_WeaponPlayerData!0x0098 m_rechamberMilestone
DT_WeaponPlayerData!0x009c m_cooldownMilestone
DT_WeaponPlayerData!0x00a0 m_fullReloadStartTime
DT_WeaponPlayerData!0x00a4 m_scriptTime0
DT_WeaponPlayerData!0x00a8 m_scriptFlags0
DT_WeaponPlayerData!0x00ac m_scriptInt0
DT_WeaponPlayerData!0x00b0 m_curZoomFOV
DT_WeaponPlayerData!0x00b4 m_targetZoomFOV
DT_WeaponPlayerData!0x00b8 m_zoomFOVLerpTime
DT_WeaponPlayerData!0x00bc m_zoomFOVLerpEndTime
DT_WeaponPlayerData!0x00c0 m_latestDryfireTime
DT_WeaponPlayerData!0x00c4 m_requestedAttackEndTime
DT_WeaponPlayerData!0x00c8 m_currentAltFireAnimIndex
DT_WeaponPlayerData!0x00cc m_legendaryModelIndex
DT_WeaponPlayerData!0x00d0 m_charmModelIndex
DT_WeaponPlayerData!0x00d4 m_charmAttachment
DT_WeaponPlayerData!0x00d8 m_charmScriptIndex
```
</details>
<details>
<summary><code>class DT_WeaponX extends DT_BaseAnimating</code></summary>

```
{
	LocalWeaponData: DT_WeaponX_LocalWeaponData,
	predictingClientOnly: DT_WeaponX_PredictingClientOnly,
	m_networkedFlags: Int,
	m_bClientSideAnimation: Int,
	m_weaponOwner: Int,
	m_worldModelIndexOverride: Int,
	m_iWorldModelIndex: Int,
	m_holsterModelIndex: Int,
	m_droppedModelIndex: Int,
	m_nIdealSequence: Int,
	m_IdealActivity: Int,
	m_weaponActivity: Int,
	m_ActiveState: Int,
	m_weapState: Int,
	m_allowedToUse: Int,
	m_discarded: Int,
	m_forcedADS: Int,
	m_tossRelease: Int,
	m_customActivity: Int,
	m_customActivitySequence: Int,
	m_customActivityOwner: Int,
	m_customActivityEndTime: Time,
	m_customActivityFlags: Int,
	m_playerData: DT_WeaponPlayerData,
	m_needsReloadCheck: Int,
	m_needsEmptyCycleCheck: Int,
	m_skinOverride: Int,
	m_skinOverrideIsValid: Int,
	m_chargeStartTime: Time,
	m_chargeEndTime: Time,
	m_lastChargeFrac: Float,
	m_sustainedDischargeEndTime: Time,
	m_sustainedLaserCurrentSpread: Float,
	m_sustainedDischargeIsInPrimaryAttack: Int,
	m_sustainedLaserNextRandomSeed: Int,
	m_modBitfieldFromPlayer: Int,
	m_modBitfieldInternal: Int,
	m_modBitfieldCurrent: Int,
	m_curSharedEnergyCost: Int,
	m_grappleWeaponNeedsDryfire: Int,
	m_scriptActivated: Int,
	m_isLoadoutPickup: Int,
	m_utilityEnt: Int,
	m_weaponNameIndex: Int,
	m_shouldPlayIdleAnims: Int,
	m_oaActiveOverride: Int,
}
```

### Offsets

```
DT_WeaponX!0x0000 LocalWeaponData
DT_WeaponX!0x0000 predictingClientOnly
DT_WeaponX!0x0394 m_networkedFlags
DT_WeaponX!0x0ff0 m_bClientSideAnimation
DT_WeaponX!0x1300 m_weaponOwner
DT_WeaponX!0x1314 m_worldModelIndexOverride
DT_WeaponX!0x1318 m_iWorldModelIndex
DT_WeaponX!0x131c m_holsterModelIndex
DT_WeaponX!0x1320 m_droppedModelIndex
DT_WeaponX!0x1324 m_nIdealSequence
DT_WeaponX!0x1328 m_IdealActivity
DT_WeaponX!0x132c m_weaponActivity
DT_WeaponX!0x1330 m_ActiveState
DT_WeaponX!0x1344 m_weapState
DT_WeaponX!0x1348 m_allowedToUse
DT_WeaponX!0x1349 m_discarded
DT_WeaponX!0x134c m_forcedADS
DT_WeaponX!0x1350 m_tossRelease
DT_WeaponX!0x1354 m_customActivity
DT_WeaponX!0x1358 m_customActivitySequence
DT_WeaponX!0x135c m_customActivityOwner
DT_WeaponX!0x1360 m_customActivityEndTime
DT_WeaponX!0x1364 m_customActivityFlags
DT_WeaponX!0x1368 m_playerData
DT_WeaponX!0x1448 m_needsReloadCheck
DT_WeaponX!0x1449 m_needsEmptyCycleCheck
DT_WeaponX!0x144c m_skinOverride
DT_WeaponX!0x1450 m_skinOverrideIsValid
DT_WeaponX!0x1454 m_chargeStartTime
DT_WeaponX!0x1458 m_chargeEndTime
DT_WeaponX!0x145c m_lastChargeFrac
DT_WeaponX!0x1480 m_sustainedDischargeEndTime
DT_WeaponX!0x1484 m_sustainedLaserCurrentSpread
DT_WeaponX!0x1488 m_sustainedDischargeIsInPrimaryAttack
DT_WeaponX!0x1489 m_sustainedLaserNextRandomSeed
DT_WeaponX!0x148c m_modBitfieldFromPlayer
DT_WeaponX!0x1490 m_modBitfieldInternal
DT_WeaponX!0x1494 m_modBitfieldCurrent
DT_WeaponX!0x1498 m_curSharedEnergyCost
DT_WeaponX!0x149c m_grappleWeaponNeedsDryfire
DT_WeaponX!0x149d m_scriptActivated
DT_WeaponX!0x149e m_isLoadoutPickup
DT_WeaponX!0x14a0 m_utilityEnt
DT_WeaponX!0x14a8 m_weaponNameIndex
DT_WeaponX!0x14b4 m_shouldPlayIdleAnims
DT_WeaponX!0x14b8 m_oaActiveOverride
```
</details>
<details>
<summary><code>class DT_WeaponX_LocalWeaponData</code></summary>

```
{
	m_nNextThinkTick: Int,
	m_lastPrimaryAttack: Time,
	m_nextReadyTime: Time,
	m_nextPrimaryAttackTime: Time,
	m_attackTimeThisFrame: Time,
	m_ammoInClip: Int,
	m_ammoInStockpile: Int,
	m_lifetimeShots: Int,
	m_flTimeWeaponIdle: Time,
	m_bInReload: Int,
}
```

### Offsets

```
DT_WeaponX_LocalWeaponData!0x050c m_nNextThinkTick
DT_WeaponX_LocalWeaponData!0x1304 m_lastPrimaryAttack
DT_WeaponX_LocalWeaponData!0x1308 m_nextReadyTime
DT_WeaponX_LocalWeaponData!0x130c m_nextPrimaryAttackTime
DT_WeaponX_LocalWeaponData!0x1310 m_attackTimeThisFrame
DT_WeaponX_LocalWeaponData!0x1334 m_ammoInClip
DT_WeaponX_LocalWeaponData!0x1338 m_ammoInStockpile
DT_WeaponX_LocalWeaponData!0x133c m_lifetimeShots
DT_WeaponX_LocalWeaponData!0x1340 m_flTimeWeaponIdle
DT_WeaponX_LocalWeaponData!0x134a m_bInReload
```
</details>
<details>
<summary><code>class DT_WeaponX_PredictingClientOnly</code></summary>

```
{
	m_lastRegenTime: Time,
	m_cooldownEndTime: Time,
	m_stockPileWasDraining: Int,
	m_weaponIsCharging: Int,
	m_weaponChargeLevelIncreasedAnimPlaying: Int,
	m_lastChargeLevel: Int,
	m_chargeEnergyDepleteStepCounter: Int,
	m_burstFireCount: Int,
	m_burstFireIndex: Int,
	m_shotCount: Int,
	m_animModelIndexPredictingClientOnly: Int,
	m_animSequencePredictingClientOnly: Int,
}
```

### Offsets

```
DT_WeaponX_PredictingClientOnly!0x1460 m_lastRegenTime
DT_WeaponX_PredictingClientOnly!0x1464 m_cooldownEndTime
DT_WeaponX_PredictingClientOnly!0x1468 m_stockPileWasDraining
DT_WeaponX_PredictingClientOnly!0x1469 m_weaponIsCharging
DT_WeaponX_PredictingClientOnly!0x146a m_weaponChargeLevelIncreasedAnimPlaying
DT_WeaponX_PredictingClientOnly!0x146c m_lastChargeLevel
DT_WeaponX_PredictingClientOnly!0x1470 m_chargeEnergyDepleteStepCounter
DT_WeaponX_PredictingClientOnly!0x1474 m_burstFireCount
DT_WeaponX_PredictingClientOnly!0x1478 m_burstFireIndex
DT_WeaponX_PredictingClientOnly!0x147c m_shotCount
DT_WeaponX_PredictingClientOnly!0x14ac m_animModelIndexPredictingClientOnly
DT_WeaponX_PredictingClientOnly!0x14b0 m_animSequencePredictingClientOnly
```
</details>
<details>
<summary><code>class DT_World extends DT_BaseEntity</code></summary>

```
{
	m_WorldMins: Vector,
	m_WorldMaxs: Vector,
	m_bStartDark: Int,
	m_statusEffectsGenerationNV: Int,
	m_worldFlags: Int,
	m_timeshiftArmDeviceSkin: Int,
	m_spTitanLoadoutUnlocks: Int,
	m_deathFieldIsActive: Int,
	m_deathFieldOrigin: Vector,
	m_deathFieldRadiusStart: Float,
	m_deathFieldRadiusEnd: Float,
	m_deathFieldTimeStart: Time,
	m_deathFieldTimeEnd: Time,
	m_teamRelationRulesForPVE: Int,
	m_civilTeamsMaskA: DataTable,
	m_civilTeamsMaskB: DataTable,
	m_rabidTeamsMask: DataTable,
}
```

### Offsets

```
DT_World!0x0a00 m_WorldMins
DT_World!0x0a0c m_WorldMaxs
DT_World!0x0a18 m_bStartDark
DT_World!0x0a2c m_statusEffectsGenerationNV
DT_World!0x0a34 m_worldFlags
DT_World!0x0a38 m_timeshiftArmDeviceSkin
DT_World!0x0a3c m_spTitanLoadoutUnlocks
DT_World!0x0a40 m_deathFieldIsActive
DT_World!0x0a44 m_deathFieldOrigin
DT_World!0x0a50 m_deathFieldRadiusStart
DT_World!0x0a54 m_deathFieldRadiusEnd
DT_World!0x0a58 m_deathFieldTimeStart
DT_World!0x0a5c m_deathFieldTimeEnd
DT_World!0x0a60 m_teamRelationRulesForPVE
DT_World!0x0a68 m_civilTeamsMaskA
DT_World!0x0a78 m_civilTeamsMaskB
DT_World!0x0a90 m_rabidTeamsMask
```
</details>
<details>
<summary><code>class DT_Zipline extends DT_BaseEntity</code></summary>

```
{
	m_numZiplinePoints: Int,
	m_ziplinePositions: DataTable,
	m_ziplinePhysics: DT_ZiplinePhysics,
	m_ziplineMaterialIndex: Int,
	m_prevZipline: Int,
	m_nextZipline: Int,
	m_detachEndOnUse: Int,
	m_dropToBottom: Int,
	m_ziplineAutoDetachDistance: Float,
	m_ziplineVerticalPushOffInDirectionX: Int,
	m_ziplineVerticalPreserveVelocity: Int,
	m_ziplineWidth: Float,
	m_ziplineEnabled: Int,
	m_ziplineRestPositions: DataTable,
	m_numZiplineRestPositions: Int,
	m_ziplineFadeDist: Float,
	m_ziplineSpeedScale: Float,
}
```

### Offsets

```
DT_Zipline!0x0008 m_numZiplinePoints
DT_Zipline!0x000c m_ziplinePositions
DT_Zipline!0x0a00 m_ziplinePhysics
DT_Zipline!0x0d48 m_ziplineMaterialIndex
DT_Zipline!0x0d4c m_prevZipline
DT_Zipline!0x0d50 m_nextZipline
DT_Zipline!0x0d54 m_detachEndOnUse
DT_Zipline!0x0d55 m_dropToBottom
DT_Zipline!0x0d58 m_ziplineAutoDetachDistance
DT_Zipline!0x0d5c m_ziplineVerticalPushOffInDirectionX
DT_Zipline!0x0d5d m_ziplineVerticalPreserveVelocity
DT_Zipline!0x0d60 m_ziplineWidth
DT_Zipline!0x0d64 m_ziplineEnabled
DT_Zipline!0x0d68 m_ziplineRestPositions
DT_Zipline!0x0e28 m_numZiplineRestPositions
DT_Zipline!0x0e2c m_ziplineFadeDist
DT_Zipline!0x0e30 m_ziplineSpeedScale
```
</details>
<details>
<summary><code>class DT_ZiplinePhysics</code></summary>

```
{
	ziplinephysicsexclusive: DT_ZiplinePhysicsExlusive,
	m_isInit: Int,
	m_ziplineType: Int,
	m_ziplineStart: Vector,
	m_ziplineEnd: Vector,
	m_springDistance: Float,
	m_springDistanceScale: Float,
	m_outerZiplineEntity: Int,
	m_attachedEntities: DataTable,
	m_numAttachedEntities: Int,
	m_ziplineOwner: Int,
}
```

### Offsets

```
DT_ZiplinePhysics!0x0000 ziplinephysicsexclusive
DT_ZiplinePhysics!0x0008 m_isInit
DT_ZiplinePhysics!0x000c m_ziplineType
DT_ZiplinePhysics!0x0010 m_ziplineStart
DT_ZiplinePhysics!0x001c m_ziplineEnd
DT_ZiplinePhysics!0x022c m_springDistance
DT_ZiplinePhysics!0x0230 m_springDistanceScale
DT_ZiplinePhysics!0x0238 m_outerZiplineEntity
DT_ZiplinePhysics!0x0240 m_attachedEntities
DT_ZiplinePhysics!0x0340 m_numAttachedEntities
DT_ZiplinePhysics!0x0344 m_ziplineOwner
```
</details>
<details>
<summary><code>class DT_ZiplinePhysicsExlusive</code></summary>

```
{
	m_nodes: DataTable,
	m_numNodes: Int,
	m_remainingUnsimulatedTime: Float,
}
```

### Offsets

```
DT_ZiplinePhysicsExlusive!0x0028 m_nodes
DT_ZiplinePhysicsExlusive!0x0228 m_numNodes
DT_ZiplinePhysicsExlusive!0x0234 m_remainingUnsimulatedTime
```
</details>

## Datamaps

<details>
<summary><code>class CBaseGrenade extends C_BaseAnimating</code></summary>

```
{
	m_vecVelocity: Vector,
	m_doesExplode: Bool,
	m_DmgRadius: Bool,
	m_grenadeCreationTime: Float,
	m_grenadeCreationOrigin: Vector,
	m_flDamage: Float,
	m_hThrower: EHANDLE,
}
```

### Offsets

```
CBaseGrenade!0x041c m_vecVelocity
CBaseGrenade!0x2801 m_doesExplode
CBaseGrenade!0x2804 m_DmgRadius
CBaseGrenade!0x2814 m_grenadeCreationTime
CBaseGrenade!0x2818 m_grenadeCreationOrigin
CBaseGrenade!0x28a8 m_flDamage
CBaseGrenade!0x28ac m_hThrower
```
</details>
<details>
<summary><code>class CBaseViewModel</code></summary>

```
{
	m_currentFrame.modelIndex: Short,
	m_currentFrame.animCycle: Float,
	m_angAbsRotation: Vector,
	m_vecAbsOrigin: Vector,
	m_localOrigin: Vector,
	m_localAngles: Vector,
	m_fEffects: Int,
	m_angNetworkAngles: Vector,
	m_nBody: Int,
	m_nResetEventsParity: Int,
	m_bSequenceFinished: Bool,
	m_currentFrameBaseAnimating.animStartTime: Float,
	m_currentFrameBaseAnimating.animStartCycle: Float,
	m_currentFrameBaseAnimating.animPlaybackRate: Float,
	m_currentFrameBaseAnimating.animModelIndex: Int,
	m_currentFrameBaseAnimating.animSequence: Int,
	m_currentFrameBaseAnimating.animSequenceParity: Int,
	m_currentFrameBaseAnimating.m_flPoseParameters: Float,
	m_currentFrameAnimatingOverlay.animOverlayIsActive: Bool,
	m_currentFrameAnimatingOverlay.animOverlayStartTime: Float,
	m_currentFrameAnimatingOverlay.animOverlayStartCycle: Float,
	m_currentFrameAnimatingOverlay.animOverlayPlaybackRate: Float,
	m_currentFrameAnimatingOverlay.animOverlayModelIndex: Int,
	m_currentFrameAnimatingOverlay.animOverlaySequence: Int,
	m_currentFrameAnimatingOverlay.animOverlayWeight: Float,
	m_currentFrameAnimatingOverlay.animOverlayAnimTime: Float,
	m_currentFrameAnimatingOverlay.animOverlayFadeInDuration: Float,
	m_currentFrameAnimatingOverlay.animOverlayFadeOutDuration: Float,
	m_currentFrameAnimatingOverlay.animOverlayCycle: Float,
	m_viewModelOwner: EHANDLE,
	m_projectileIsVisible: Bool,
	m_bBlockEventLayer: Bool,
	m_isAdsTransition: Bool,
	m_hWeapon: EHANDLE,
	m_tracerAttachments: Int,
	m_tracerAttachments: Int,
	m_tracerAttachmentsScoped: Int,
	m_tracerAttachmentsScoped: Int,
}
```

### Offsets

```
CBaseViewModel!0x00a8 m_currentFrame.modelIndex
CBaseViewModel!0x00c4 m_currentFrame.animCycle
CBaseViewModel!0x0134 m_angAbsRotation
CBaseViewModel!0x014c m_vecAbsOrigin
CBaseViewModel!0x0158 m_localOrigin
CBaseViewModel!0x0164 m_localAngles
CBaseViewModel!0x03ec m_fEffects
CBaseViewModel!0x0428 m_angNetworkAngles
CBaseViewModel!0x0e50 m_nBody
CBaseViewModel!0x0e5c m_nResetEventsParity
CBaseViewModel!0x0f00 m_bSequenceFinished
CBaseViewModel!0x0f18 m_currentFrameBaseAnimating.animStartTime
CBaseViewModel!0x0f1c m_currentFrameBaseAnimating.animStartCycle
CBaseViewModel!0x0f20 m_currentFrameBaseAnimating.animPlaybackRate
CBaseViewModel!0x0f28 m_currentFrameBaseAnimating.animModelIndex
CBaseViewModel!0x0f2c m_currentFrameBaseAnimating.animSequence
CBaseViewModel!0x0f30 m_currentFrameBaseAnimating.animSequenceParity
CBaseViewModel!0x0f34 m_currentFrameBaseAnimating.m_flPoseParameters
CBaseViewModel!0x1424 m_currentFrameAnimatingOverlay.animOverlayIsActive
CBaseViewModel!0x1430 m_currentFrameAnimatingOverlay.animOverlayStartTime
CBaseViewModel!0x1454 m_currentFrameAnimatingOverlay.animOverlayStartCycle
CBaseViewModel!0x1478 m_currentFrameAnimatingOverlay.animOverlayPlaybackRate
CBaseViewModel!0x149c m_currentFrameAnimatingOverlay.animOverlayModelIndex
CBaseViewModel!0x14c0 m_currentFrameAnimatingOverlay.animOverlaySequence
CBaseViewModel!0x14e4 m_currentFrameAnimatingOverlay.animOverlayWeight
CBaseViewModel!0x152c m_currentFrameAnimatingOverlay.animOverlayAnimTime
CBaseViewModel!0x1550 m_currentFrameAnimatingOverlay.animOverlayFadeInDuration
CBaseViewModel!0x1574 m_currentFrameAnimatingOverlay.animOverlayFadeOutDuration
CBaseViewModel!0x1598 m_currentFrameAnimatingOverlay.animOverlayCycle
CBaseViewModel!0x16d8 m_viewModelOwner
CBaseViewModel!0x16dc m_projectileIsVisible
CBaseViewModel!0x1ac0 m_bBlockEventLayer
CBaseViewModel!0x1ac1 m_isAdsTransition
CBaseViewModel!0x1ac4 m_hWeapon
CBaseViewModel!0x1ac8 m_tracerAttachments
CBaseViewModel!0x1ac8 m_tracerAttachments
CBaseViewModel!0x1ad0 m_tracerAttachmentsScoped
CBaseViewModel!0x1ad0 m_tracerAttachmentsScoped
```
</details>
<details>
<summary><code>class CCollisionProperty</code></summary>

```
{
	m_vecMins: Vector,
	m_vecMaxs: Vector,
	m_usSolidFlags: Int,
	m_nSolidType: Char,
	m_triggerBloat: Char,
	m_collisionDetailLevel: Char,
}
```

### Offsets

```
CCollisionProperty!0x0010 m_vecMins
CCollisionProperty!0x001c m_vecMaxs
CCollisionProperty!0x0028 m_usSolidFlags
CCollisionProperty!0x002c m_nSolidType
CCollisionProperty!0x002d m_triggerBloat
CCollisionProperty!0x002e m_collisionDetailLevel
```
</details>
<details>
<summary><code>class CGrappleHook</code></summary>

```
{
	m_pMoveParent: EHANDLE,
	m_localOrigin: Vector,
	m_localAngles: Vector,
	m_visibilityFlags: Int,
	m_parentAttachmentType: Int,
	m_parentAttachmentIndex: Int,
	m_parentAttachmentHitbox: Int,
	m_grappleZipline: EHANDLE,
}
```

### Offsets

```
CGrappleHook!0x0118 m_pMoveParent
CGrappleHook!0x0158 m_localOrigin
CGrappleHook!0x0164 m_localAngles
CGrappleHook!0x03e8 m_visibilityFlags
CGrappleHook!0x07ec m_parentAttachmentType
CGrappleHook!0x07f0 m_parentAttachmentIndex
CGrappleHook!0x07f4 m_parentAttachmentHitbox
CGrappleHook!0x1300 m_grappleZipline
```
</details>
<details>
<summary><code>class CPlayerShared</code></summary>

```
{
	m_nPlayerCond: Int,
}
```

### Offsets

```
CPlayerShared!0x0008 m_nPlayerCond
```
</details>
<details>
<summary><code>class CPlayerState</code></summary>

```
{
	deadflag: Bool,
}
```

### Offsets

```
CPlayerState!0x006c deadflag
```
</details>
<details>
<summary><code>class CPredictedFirstPersonProxy extends C_BaseAnimating</code></summary>

```
{
	m_localOrigin: Vector,
	m_localAngles: Vector,
	m_vecVelocity: Vector,
	m_angNetworkAngles: Vector,
	m_SequenceTransitioner: C_SequenceTransitioner,
	m_camoIndex: Int,
}
```

### Offsets

```
CPredictedFirstPersonProxy!0x0158 m_localOrigin
CPredictedFirstPersonProxy!0x0164 m_localAngles
CPredictedFirstPersonProxy!0x041c m_vecVelocity
CPredictedFirstPersonProxy!0x0428 m_angNetworkAngles
CPredictedFirstPersonProxy!0x0bc0 m_SequenceTransitioner
CPredictedFirstPersonProxy!0x0e54 m_camoIndex
```
</details>
<details>
<summary><code>class CRagdoll</code></summary>

```
{
	m_ragdoll.listCount: Int,
	m_ragdoll.allowStretch: Bool,
	m_ragdoll.list[0 + 0].originParentSpace: Vector,
	m_ragdoll.list[0 + 0].pObject: Custom,
	m_ragdoll.list[0 + 0].pConstraint: Custom,
	m_ragdoll.list[0 + 0].parentIndex: Int,
	m_ragdoll.list[0 + 1].originParentSpace: Vector,
	m_ragdoll.list[0 + 1].pObject: Custom,
	m_ragdoll.list[0 + 1].pConstraint: Custom,
	m_ragdoll.list[0 + 1].parentIndex: Int,
	m_ragdoll.list[0 + 2].originParentSpace: Vector,
	m_ragdoll.list[0 + 2].pObject: Custom,
	m_ragdoll.list[0 + 2].pConstraint: Custom,
	m_ragdoll.list[0 + 2].parentIndex: Int,
	m_ragdoll.list[0 + 3].originParentSpace: Vector,
	m_ragdoll.list[0 + 3].pObject: Custom,
	m_ragdoll.list[0 + 3].pConstraint: Custom,
	m_ragdoll.list[0 + 3].parentIndex: Int,
	m_ragdoll.list[0 + 4].originParentSpace: Vector,
	m_ragdoll.list[0 + 4].pObject: Custom,
	m_ragdoll.list[0 + 4].pConstraint: Custom,
	m_ragdoll.list[0 + 4].parentIndex: Int,
	m_ragdoll.list[0 + 5].originParentSpace: Vector,
	m_ragdoll.list[0 + 5].pObject: Custom,
	m_ragdoll.list[0 + 5].pConstraint: Custom,
	m_ragdoll.list[0 + 5].parentIndex: Int,
	m_ragdoll.list[0 + 6].originParentSpace: Vector,
	m_ragdoll.list[0 + 6].pObject: Custom,
	m_ragdoll.list[0 + 6].pConstraint: Custom,
	m_ragdoll.list[0 + 6].parentIndex: Int,
	m_ragdoll.list[0 + 7].originParentSpace: Vector,
	m_ragdoll.list[0 + 7].pObject: Custom,
	m_ragdoll.list[0 + 7].pConstraint: Custom,
	m_ragdoll.list[0 + 7].parentIndex: Int,
	m_ragdoll.list[8 + 0].originParentSpace: Vector,
	m_ragdoll.list[8 + 0].pObject: Custom,
	m_ragdoll.list[8 + 0].pConstraint: Custom,
	m_ragdoll.list[8 + 0].parentIndex: Int,
	m_ragdoll.list[8 + 1].originParentSpace: Vector,
	m_ragdoll.list[8 + 1].pObject: Custom,
	m_ragdoll.list[8 + 1].pConstraint: Custom,
	m_ragdoll.list[8 + 1].parentIndex: Int,
	m_ragdoll.list[8 + 2].originParentSpace: Vector,
	m_ragdoll.list[8 + 2].pObject: Custom,
	m_ragdoll.list[8 + 2].pConstraint: Custom,
	m_ragdoll.list[8 + 2].parentIndex: Int,
	m_ragdoll.list[8 + 3].originParentSpace: Vector,
	m_ragdoll.list[8 + 3].pObject: Custom,
	m_ragdoll.list[8 + 3].pConstraint: Custom,
	m_ragdoll.list[8 + 3].parentIndex: Int,
	m_ragdoll.list[8 + 4].originParentSpace: Vector,
	m_ragdoll.list[8 + 4].pObject: Custom,
	m_ragdoll.list[8 + 4].pConstraint: Custom,
	m_ragdoll.list[8 + 4].parentIndex: Int,
	m_ragdoll.list[8 + 5].originParentSpace: Vector,
	m_ragdoll.list[8 + 5].pObject: Custom,
	m_ragdoll.list[8 + 5].pConstraint: Custom,
	m_ragdoll.list[8 + 5].parentIndex: Int,
	m_ragdoll.list[8 + 6].originParentSpace: Vector,
	m_ragdoll.list[8 + 6].pObject: Custom,
	m_ragdoll.list[8 + 6].pConstraint: Custom,
	m_ragdoll.list[8 + 6].parentIndex: Int,
	m_ragdoll.list[8 + 7].originParentSpace: Vector,
	m_ragdoll.list[8 + 7].pObject: Custom,
	m_ragdoll.list[8 + 7].pConstraint: Custom,
	m_ragdoll.list[8 + 7].parentIndex: Int,
	m_ragdoll.list[16 + 0].originParentSpace: Vector,
	m_ragdoll.list[16 + 0].pObject: Custom,
	m_ragdoll.list[16 + 0].pConstraint: Custom,
	m_ragdoll.list[16 + 0].parentIndex: Int,
	m_ragdoll.list[16 + 1].originParentSpace: Vector,
	m_ragdoll.list[16 + 1].pObject: Custom,
	m_ragdoll.list[16 + 1].pConstraint: Custom,
	m_ragdoll.list[16 + 1].parentIndex: Int,
	m_ragdoll.list[16 + 2].originParentSpace: Vector,
	m_ragdoll.list[16 + 2].pObject: Custom,
	m_ragdoll.list[16 + 2].pConstraint: Custom,
	m_ragdoll.list[16 + 2].parentIndex: Int,
	m_ragdoll.list[16 + 3].originParentSpace: Vector,
	m_ragdoll.list[16 + 3].pObject: Custom,
	m_ragdoll.list[16 + 3].pConstraint: Custom,
	m_ragdoll.list[16 + 3].parentIndex: Int,
	m_ragdoll.list[16 + 4].originParentSpace: Vector,
	m_ragdoll.list[16 + 4].pObject: Custom,
	m_ragdoll.list[16 + 4].pConstraint: Custom,
	m_ragdoll.list[16 + 4].parentIndex: Int,
	m_ragdoll.list[16 + 5].originParentSpace: Vector,
	m_ragdoll.list[16 + 5].pObject: Custom,
	m_ragdoll.list[16 + 5].pConstraint: Custom,
	m_ragdoll.list[16 + 5].parentIndex: Int,
	m_ragdoll.list[16 + 6].originParentSpace: Vector,
	m_ragdoll.list[16 + 6].pObject: Custom,
	m_ragdoll.list[16 + 6].pConstraint: Custom,
	m_ragdoll.list[16 + 6].parentIndex: Int,
	m_ragdoll.list[16 + 7].originParentSpace: Vector,
	m_ragdoll.list[16 + 7].pObject: Custom,
	m_ragdoll.list[16 + 7].pConstraint: Custom,
	m_ragdoll.list[16 + 7].parentIndex: Int,
	m_ragdoll.list[24 + 0].originParentSpace: Vector,
	m_ragdoll.list[24 + 0].pObject: Custom,
	m_ragdoll.list[24 + 0].pConstraint: Custom,
	m_ragdoll.list[24 + 0].parentIndex: Int,
	m_ragdoll.list[24 + 1].originParentSpace: Vector,
	m_ragdoll.list[24 + 1].pObject: Custom,
	m_ragdoll.list[24 + 1].pConstraint: Custom,
	m_ragdoll.list[24 + 1].parentIndex: Int,
	m_ragdoll.list[24 + 2].originParentSpace: Vector,
	m_ragdoll.list[24 + 2].pObject: Custom,
	m_ragdoll.list[24 + 2].pConstraint: Custom,
	m_ragdoll.list[24 + 2].parentIndex: Int,
	m_ragdoll.list[24 + 3].originParentSpace: Vector,
	m_ragdoll.list[24 + 3].pObject: Custom,
	m_ragdoll.list[24 + 3].pConstraint: Custom,
	m_ragdoll.list[24 + 3].parentIndex: Int,
	m_ragdoll.list[24 + 4].originParentSpace: Vector,
	m_ragdoll.list[24 + 4].pObject: Custom,
	m_ragdoll.list[24 + 4].pConstraint: Custom,
	m_ragdoll.list[24 + 4].parentIndex: Int,
	m_ragdoll.list[24 + 5].originParentSpace: Vector,
	m_ragdoll.list[24 + 5].pObject: Custom,
	m_ragdoll.list[24 + 5].pConstraint: Custom,
	m_ragdoll.list[24 + 5].parentIndex: Int,
	m_ragdoll.list[24 + 6].originParentSpace: Vector,
	m_ragdoll.list[24 + 6].pObject: Custom,
	m_ragdoll.list[24 + 6].pConstraint: Custom,
	m_ragdoll.list[24 + 6].parentIndex: Int,
	m_ragdoll.list[24 + 7].originParentSpace: Vector,
	m_ragdoll.list[24 + 7].pObject: Custom,
	m_ragdoll.list[24 + 7].pConstraint: Custom,
	m_ragdoll.list[24 + 7].parentIndex: Int,
	m_ragdoll.boneIndex: Int,
}
```

### Offsets

```
CRagdoll!0x0000 m_ragdoll.listCount
CRagdoll!0x0004 m_ragdoll.allowStretch
CRagdoll!0x0008 m_ragdoll.list[0 + 0].originParentSpace
CRagdoll!0x0018 m_ragdoll.list[0 + 0].pObject
CRagdoll!0x0020 m_ragdoll.list[0 + 0].pConstraint
CRagdoll!0x0028 m_ragdoll.list[0 + 0].parentIndex
CRagdoll!0x0030 m_ragdoll.list[0 + 1].originParentSpace
CRagdoll!0x0040 m_ragdoll.list[0 + 1].pObject
CRagdoll!0x0048 m_ragdoll.list[0 + 1].pConstraint
CRagdoll!0x0050 m_ragdoll.list[0 + 1].parentIndex
CRagdoll!0x0058 m_ragdoll.list[0 + 2].originParentSpace
CRagdoll!0x0068 m_ragdoll.list[0 + 2].pObject
CRagdoll!0x0070 m_ragdoll.list[0 + 2].pConstraint
CRagdoll!0x0078 m_ragdoll.list[0 + 2].parentIndex
CRagdoll!0x0080 m_ragdoll.list[0 + 3].originParentSpace
CRagdoll!0x0090 m_ragdoll.list[0 + 3].pObject
CRagdoll!0x0098 m_ragdoll.list[0 + 3].pConstraint
CRagdoll!0x00a0 m_ragdoll.list[0 + 3].parentIndex
CRagdoll!0x00a8 m_ragdoll.list[0 + 4].originParentSpace
CRagdoll!0x00b8 m_ragdoll.list[0 + 4].pObject
CRagdoll!0x00c0 m_ragdoll.list[0 + 4].pConstraint
CRagdoll!0x00c8 m_ragdoll.list[0 + 4].parentIndex
CRagdoll!0x00d0 m_ragdoll.list[0 + 5].originParentSpace
CRagdoll!0x00e0 m_ragdoll.list[0 + 5].pObject
CRagdoll!0x00e8 m_ragdoll.list[0 + 5].pConstraint
CRagdoll!0x00f0 m_ragdoll.list[0 + 5].parentIndex
CRagdoll!0x00f8 m_ragdoll.list[0 + 6].originParentSpace
CRagdoll!0x0108 m_ragdoll.list[0 + 6].pObject
CRagdoll!0x0110 m_ragdoll.list[0 + 6].pConstraint
CRagdoll!0x0118 m_ragdoll.list[0 + 6].parentIndex
CRagdoll!0x0120 m_ragdoll.list[0 + 7].originParentSpace
CRagdoll!0x0130 m_ragdoll.list[0 + 7].pObject
CRagdoll!0x0138 m_ragdoll.list[0 + 7].pConstraint
CRagdoll!0x0140 m_ragdoll.list[0 + 7].parentIndex
CRagdoll!0x0148 m_ragdoll.list[8 + 0].originParentSpace
CRagdoll!0x0158 m_ragdoll.list[8 + 0].pObject
CRagdoll!0x0160 m_ragdoll.list[8 + 0].pConstraint
CRagdoll!0x0168 m_ragdoll.list[8 + 0].parentIndex
CRagdoll!0x0170 m_ragdoll.list[8 + 1].originParentSpace
CRagdoll!0x0180 m_ragdoll.list[8 + 1].pObject
CRagdoll!0x0188 m_ragdoll.list[8 + 1].pConstraint
CRagdoll!0x0190 m_ragdoll.list[8 + 1].parentIndex
CRagdoll!0x0198 m_ragdoll.list[8 + 2].originParentSpace
CRagdoll!0x01a8 m_ragdoll.list[8 + 2].pObject
CRagdoll!0x01b0 m_ragdoll.list[8 + 2].pConstraint
CRagdoll!0x01b8 m_ragdoll.list[8 + 2].parentIndex
CRagdoll!0x01c0 m_ragdoll.list[8 + 3].originParentSpace
CRagdoll!0x01d0 m_ragdoll.list[8 + 3].pObject
CRagdoll!0x01d8 m_ragdoll.list[8 + 3].pConstraint
CRagdoll!0x01e0 m_ragdoll.list[8 + 3].parentIndex
CRagdoll!0x01e8 m_ragdoll.list[8 + 4].originParentSpace
CRagdoll!0x01f8 m_ragdoll.list[8 + 4].pObject
CRagdoll!0x0200 m_ragdoll.list[8 + 4].pConstraint
CRagdoll!0x0208 m_ragdoll.list[8 + 4].parentIndex
CRagdoll!0x0210 m_ragdoll.list[8 + 5].originParentSpace
CRagdoll!0x0220 m_ragdoll.list[8 + 5].pObject
CRagdoll!0x0228 m_ragdoll.list[8 + 5].pConstraint
CRagdoll!0x0230 m_ragdoll.list[8 + 5].parentIndex
CRagdoll!0x0238 m_ragdoll.list[8 + 6].originParentSpace
CRagdoll!0x0248 m_ragdoll.list[8 + 6].pObject
CRagdoll!0x0250 m_ragdoll.list[8 + 6].pConstraint
CRagdoll!0x0258 m_ragdoll.list[8 + 6].parentIndex
CRagdoll!0x0260 m_ragdoll.list[8 + 7].originParentSpace
CRagdoll!0x0270 m_ragdoll.list[8 + 7].pObject
CRagdoll!0x0278 m_ragdoll.list[8 + 7].pConstraint
CRagdoll!0x0280 m_ragdoll.list[8 + 7].parentIndex
CRagdoll!0x0288 m_ragdoll.list[16 + 0].originParentSpace
CRagdoll!0x0298 m_ragdoll.list[16 + 0].pObject
CRagdoll!0x02a0 m_ragdoll.list[16 + 0].pConstraint
CRagdoll!0x02a8 m_ragdoll.list[16 + 0].parentIndex
CRagdoll!0x02b0 m_ragdoll.list[16 + 1].originParentSpace
CRagdoll!0x02c0 m_ragdoll.list[16 + 1].pObject
CRagdoll!0x02c8 m_ragdoll.list[16 + 1].pConstraint
CRagdoll!0x02d0 m_ragdoll.list[16 + 1].parentIndex
CRagdoll!0x02d8 m_ragdoll.list[16 + 2].originParentSpace
CRagdoll!0x02e8 m_ragdoll.list[16 + 2].pObject
CRagdoll!0x02f0 m_ragdoll.list[16 + 2].pConstraint
CRagdoll!0x02f8 m_ragdoll.list[16 + 2].parentIndex
CRagdoll!0x0300 m_ragdoll.list[16 + 3].originParentSpace
CRagdoll!0x0310 m_ragdoll.list[16 + 3].pObject
CRagdoll!0x0318 m_ragdoll.list[16 + 3].pConstraint
CRagdoll!0x0320 m_ragdoll.list[16 + 3].parentIndex
CRagdoll!0x0328 m_ragdoll.list[16 + 4].originParentSpace
CRagdoll!0x0338 m_ragdoll.list[16 + 4].pObject
CRagdoll!0x0340 m_ragdoll.list[16 + 4].pConstraint
CRagdoll!0x0348 m_ragdoll.list[16 + 4].parentIndex
CRagdoll!0x0350 m_ragdoll.list[16 + 5].originParentSpace
CRagdoll!0x0360 m_ragdoll.list[16 + 5].pObject
CRagdoll!0x0368 m_ragdoll.list[16 + 5].pConstraint
CRagdoll!0x0370 m_ragdoll.list[16 + 5].parentIndex
CRagdoll!0x0378 m_ragdoll.list[16 + 6].originParentSpace
CRagdoll!0x0388 m_ragdoll.list[16 + 6].pObject
CRagdoll!0x0390 m_ragdoll.list[16 + 6].pConstraint
CRagdoll!0x0398 m_ragdoll.list[16 + 6].parentIndex
CRagdoll!0x03a0 m_ragdoll.list[16 + 7].originParentSpace
CRagdoll!0x03b0 m_ragdoll.list[16 + 7].pObject
CRagdoll!0x03b8 m_ragdoll.list[16 + 7].pConstraint
CRagdoll!0x03c0 m_ragdoll.list[16 + 7].parentIndex
CRagdoll!0x03c8 m_ragdoll.list[24 + 0].originParentSpace
CRagdoll!0x03d8 m_ragdoll.list[24 + 0].pObject
CRagdoll!0x03e0 m_ragdoll.list[24 + 0].pConstraint
CRagdoll!0x03e8 m_ragdoll.list[24 + 0].parentIndex
CRagdoll!0x03f0 m_ragdoll.list[24 + 1].originParentSpace
CRagdoll!0x0400 m_ragdoll.list[24 + 1].pObject
CRagdoll!0x0408 m_ragdoll.list[24 + 1].pConstraint
CRagdoll!0x0410 m_ragdoll.list[24 + 1].parentIndex
CRagdoll!0x0418 m_ragdoll.list[24 + 2].originParentSpace
CRagdoll!0x0428 m_ragdoll.list[24 + 2].pObject
CRagdoll!0x0430 m_ragdoll.list[24 + 2].pConstraint
CRagdoll!0x0438 m_ragdoll.list[24 + 2].parentIndex
CRagdoll!0x0440 m_ragdoll.list[24 + 3].originParentSpace
CRagdoll!0x0450 m_ragdoll.list[24 + 3].pObject
CRagdoll!0x0458 m_ragdoll.list[24 + 3].pConstraint
CRagdoll!0x0460 m_ragdoll.list[24 + 3].parentIndex
CRagdoll!0x0468 m_ragdoll.list[24 + 4].originParentSpace
CRagdoll!0x0478 m_ragdoll.list[24 + 4].pObject
CRagdoll!0x0480 m_ragdoll.list[24 + 4].pConstraint
CRagdoll!0x0488 m_ragdoll.list[24 + 4].parentIndex
CRagdoll!0x0490 m_ragdoll.list[24 + 5].originParentSpace
CRagdoll!0x04a0 m_ragdoll.list[24 + 5].pObject
CRagdoll!0x04a8 m_ragdoll.list[24 + 5].pConstraint
CRagdoll!0x04b0 m_ragdoll.list[24 + 5].parentIndex
CRagdoll!0x04b8 m_ragdoll.list[24 + 6].originParentSpace
CRagdoll!0x04c8 m_ragdoll.list[24 + 6].pObject
CRagdoll!0x04d0 m_ragdoll.list[24 + 6].pConstraint
CRagdoll!0x04d8 m_ragdoll.list[24 + 6].parentIndex
CRagdoll!0x04e0 m_ragdoll.list[24 + 7].originParentSpace
CRagdoll!0x04f0 m_ragdoll.list[24 + 7].pObject
CRagdoll!0x04f8 m_ragdoll.list[24 + 7].pConstraint
CRagdoll!0x0500 m_ragdoll.list[24 + 7].parentIndex
CRagdoll!0x0508 m_ragdoll.boneIndex
```
</details>
<details>
<summary><code>class CTurret extends C_BaseAnimating</code></summary>

```
{
	m_aimAngle: Float,
	m_minConeAngle: Float,
	m_maxConeAngle: Float,
}
```

### Offsets

```
CTurret!0x1324 m_aimAngle
CTurret!0x1330 m_minConeAngle
CTurret!0x133c m_maxConeAngle
```
</details>
<details>
<summary><code>class CWeaponX extends C_BaseAnimating</code></summary>

```
{
	m_localOrigin: Vector,
	m_nNextThinkTick: Int,
	m_SequenceTransitioner: C_SequenceTransitioner,
	m_weaponOwner: EHANDLE,
	m_lastPrimaryAttack: Time,
	m_nextReadyTime: Time,
	m_nextPrimaryAttackTime: Time,
	m_attackTimeThisFrame: Time,
	m_worldModelIndexOverride: Int,
	m_iWorldModelIndex: Int,
	m_holsterModelIndex: Int,
	m_droppedModelIndex: Int,
	m_nIdealSequence: Int,
	m_IdealActivity: Int,
	m_weaponActivity: Int,
	m_ActiveState: Int,
	m_ammoInClip: Int,
	m_ammoInStockpile: Int,
	m_lifetimeShots: Int,
	m_flTimeWeaponIdle: Time,
	m_weapState: Int,
	m_discarded: Bool,
	m_bInReload: Bool,
	m_tossRelease: Int,
	m_customActivity: Int,
	m_customActivitySequence: Int,
	m_customActivityOwner: EHANDLE,
	m_customActivityEndTime: Time,
	m_customActivityFlags: Char,
	m_playerData: WeaponPlayerData,
	m_needsReloadCheck: Bool,
	m_needsEmptyCycleCheck: Bool,
	m_skinOverride: Int,
	m_skinOverrideIsValid: Bool,
	m_chargeStartTime: Time,
	m_chargeEndTime: Time,
	m_lastChargeFrac: Float,
	m_lastRegenTime: Time,
	m_cooldownEndTime: Time,
	m_stockPileWasDraining: Bool,
	m_weaponIsCharging: Bool,
	m_weaponChargeLevelIncreasedAnimPlaying: Bool,
	m_lastChargeLevel: Int,
	m_chargeEnergyDepleteStepCounter: Int,
	m_burstFireCount: Int,
	m_burstFireIndex: Int,
	m_shotCount: Int,
	m_sustainedDischargeEndTime: Time,
	m_sustainedLaserCurrentSpread: Float,
	m_sustainedDischargeIsInPrimaryAttack: Bool,
	m_sustainedLaserNextRandomSeed: Char,
	m_modBitfieldFromPlayer: Int,
	m_modBitfieldInternal: Int,
	m_modBitfieldCurrent: Int,
	m_curSharedEnergyCost: Int,
	m_grappleWeaponNeedsDryfire: Bool,
	m_scriptActivated: Bool,
	m_flNextEmptySoundTime: Float,
	m_bRemoveable: Bool,
}
```

### Offsets

```
CWeaponX!0x0158 m_localOrigin
CWeaponX!0x050c m_nNextThinkTick
CWeaponX!0x0bc0 m_SequenceTransitioner
CWeaponX!0x1300 m_weaponOwner
CWeaponX!0x1304 m_lastPrimaryAttack
CWeaponX!0x1308 m_nextReadyTime
CWeaponX!0x130c m_nextPrimaryAttackTime
CWeaponX!0x1310 m_attackTimeThisFrame
CWeaponX!0x1314 m_worldModelIndexOverride
CWeaponX!0x1318 m_iWorldModelIndex
CWeaponX!0x131c m_holsterModelIndex
CWeaponX!0x1320 m_droppedModelIndex
CWeaponX!0x1324 m_nIdealSequence
CWeaponX!0x1328 m_IdealActivity
CWeaponX!0x132c m_weaponActivity
CWeaponX!0x1330 m_ActiveState
CWeaponX!0x1334 m_ammoInClip
CWeaponX!0x1338 m_ammoInStockpile
CWeaponX!0x133c m_lifetimeShots
CWeaponX!0x1340 m_flTimeWeaponIdle
CWeaponX!0x1344 m_weapState
CWeaponX!0x1349 m_discarded
CWeaponX!0x134a m_bInReload
CWeaponX!0x1350 m_tossRelease
CWeaponX!0x1354 m_customActivity
CWeaponX!0x1358 m_customActivitySequence
CWeaponX!0x135c m_customActivityOwner
CWeaponX!0x1360 m_customActivityEndTime
CWeaponX!0x1364 m_customActivityFlags
CWeaponX!0x1368 m_playerData
CWeaponX!0x1448 m_needsReloadCheck
CWeaponX!0x1449 m_needsEmptyCycleCheck
CWeaponX!0x144c m_skinOverride
CWeaponX!0x1450 m_skinOverrideIsValid
CWeaponX!0x1454 m_chargeStartTime
CWeaponX!0x1458 m_chargeEndTime
CWeaponX!0x145c m_lastChargeFrac
CWeaponX!0x1460 m_lastRegenTime
CWeaponX!0x1464 m_cooldownEndTime
CWeaponX!0x1468 m_stockPileWasDraining
CWeaponX!0x1469 m_weaponIsCharging
CWeaponX!0x146a m_weaponChargeLevelIncreasedAnimPlaying
CWeaponX!0x146c m_lastChargeLevel
CWeaponX!0x1470 m_chargeEnergyDepleteStepCounter
CWeaponX!0x1474 m_burstFireCount
CWeaponX!0x1478 m_burstFireIndex
CWeaponX!0x147c m_shotCount
CWeaponX!0x1480 m_sustainedDischargeEndTime
CWeaponX!0x1484 m_sustainedLaserCurrentSpread
CWeaponX!0x1488 m_sustainedDischargeIsInPrimaryAttack
CWeaponX!0x1489 m_sustainedLaserNextRandomSeed
CWeaponX!0x148c m_modBitfieldFromPlayer
CWeaponX!0x1490 m_modBitfieldInternal
CWeaponX!0x1494 m_modBitfieldCurrent
CWeaponX!0x1498 m_curSharedEnergyCost
CWeaponX!0x149c m_grappleWeaponNeedsDryfire
CWeaponX!0x149d m_scriptActivated
CWeaponX!0x27e0 m_flNextEmptySoundTime
CWeaponX!0x2806 m_bRemoveable
```
</details>
<details>
<summary><code>class C_BaseAnimating extends C_BaseEntity</code></summary>

```
{
	m_currentFrame.animCycle: Float,
	m_animNetworkFlags: Int,
	m_networkAnimActive: Bool,
	m_animActive: Bool,
	m_animCollisionEnabled: Bool,
	m_animPlantingEnabled: Bool,
	m_predictedAnimEventData: PredictedAnimEventData,
	m_SequenceTransitioner: C_SequenceTransitioner,
	m_nSkin: Int,
	m_skinMod: Short,
	m_nBody: Int,
	m_nResetEventsParity: Int,
	m_bSequenceFinished: Bool,
	m_bSequenceLooped: Bool,
	m_bSequenceLoops: Bool,
	m_flModelScale: Float,
	m_currentFrameBaseAnimating.animStartTime: Float,
	m_currentFrameBaseAnimating.animStartCycle: Float,
	m_currentFrameBaseAnimating.animPlaybackRate: Float,
	m_currentFrameBaseAnimating.animModelIndex: Int,
	m_currentFrameBaseAnimating.animSequence: Int,
	m_currentFrameBaseAnimating.animSequenceParity: Int,
	m_currentFrameBaseAnimating.m_flPoseParameters: Float,
}
```

### Offsets

```
C_BaseAnimating!0x00c4 m_currentFrame.animCycle
C_BaseAnimating!0x0a28 m_animNetworkFlags
C_BaseAnimating!0x0a2c m_networkAnimActive
C_BaseAnimating!0x0a2e m_animActive
C_BaseAnimating!0x0a2f m_animCollisionEnabled
C_BaseAnimating!0x0a30 m_animPlantingEnabled
C_BaseAnimating!0x0b28 m_predictedAnimEventData
C_BaseAnimating!0x0bc0 m_SequenceTransitioner
C_BaseAnimating!0x0e48 m_nSkin
C_BaseAnimating!0x0e4c m_skinMod
C_BaseAnimating!0x0e50 m_nBody
C_BaseAnimating!0x0e5c m_nResetEventsParity
C_BaseAnimating!0x0f00 m_bSequenceFinished
C_BaseAnimating!0x0f08 m_bSequenceLooped
C_BaseAnimating!0x0f09 m_bSequenceLoops
C_BaseAnimating!0x0f0c m_flModelScale
C_BaseAnimating!0x0f18 m_currentFrameBaseAnimating.animStartTime
C_BaseAnimating!0x0f1c m_currentFrameBaseAnimating.animStartCycle
C_BaseAnimating!0x0f20 m_currentFrameBaseAnimating.animPlaybackRate
C_BaseAnimating!0x0f28 m_currentFrameBaseAnimating.animModelIndex
C_BaseAnimating!0x0f2c m_currentFrameBaseAnimating.animSequence
C_BaseAnimating!0x0f30 m_currentFrameBaseAnimating.animSequenceParity
C_BaseAnimating!0x0f34 m_currentFrameBaseAnimating.m_flPoseParameters
```
</details>
<details>
<summary><code>class C_BaseAnimatingOverlay extends C_BaseAnimating</code></summary>

```
{
	m_AnimOverlay: C_AnimationLayer,
	m_AnimOverlayCount: Int,
	m_currentFrameAnimatingOverlay.animOverlayIsActive: Bool,
	m_currentFrameAnimatingOverlay.animOverlayStartTime: Float,
	m_currentFrameAnimatingOverlay.animOverlayStartCycle: Float,
	m_currentFrameAnimatingOverlay.animOverlayPlaybackRate: Float,
	m_currentFrameAnimatingOverlay.animOverlayModelIndex: Int,
	m_currentFrameAnimatingOverlay.animOverlaySequence: Int,
	m_currentFrameAnimatingOverlay.animOverlayWeight: Float,
	m_currentFrameAnimatingOverlay.animOverlayOrder: Int,
	m_currentFrameAnimatingOverlay.animOverlayAnimTime: Float,
	m_currentFrameAnimatingOverlay.animOverlayFadeInDuration: Float,
	m_currentFrameAnimatingOverlay.animOverlayFadeOutDuration: Float,
	m_currentFrameAnimatingOverlay.animOverlayCycle: Float,
}
```

### Offsets

```
C_BaseAnimatingOverlay!0x1308 m_AnimOverlay
C_BaseAnimatingOverlay!0x13e0 m_AnimOverlayCount
C_BaseAnimatingOverlay!0x1424 m_currentFrameAnimatingOverlay.animOverlayIsActive
C_BaseAnimatingOverlay!0x1430 m_currentFrameAnimatingOverlay.animOverlayStartTime
C_BaseAnimatingOverlay!0x1454 m_currentFrameAnimatingOverlay.animOverlayStartCycle
C_BaseAnimatingOverlay!0x1478 m_currentFrameAnimatingOverlay.animOverlayPlaybackRate
C_BaseAnimatingOverlay!0x149c m_currentFrameAnimatingOverlay.animOverlayModelIndex
C_BaseAnimatingOverlay!0x14c0 m_currentFrameAnimatingOverlay.animOverlaySequence
C_BaseAnimatingOverlay!0x14e4 m_currentFrameAnimatingOverlay.animOverlayWeight
C_BaseAnimatingOverlay!0x1508 m_currentFrameAnimatingOverlay.animOverlayOrder
C_BaseAnimatingOverlay!0x152c m_currentFrameAnimatingOverlay.animOverlayAnimTime
C_BaseAnimatingOverlay!0x1550 m_currentFrameAnimatingOverlay.animOverlayFadeInDuration
C_BaseAnimatingOverlay!0x1574 m_currentFrameAnimatingOverlay.animOverlayFadeOutDuration
C_BaseAnimatingOverlay!0x1598 m_currentFrameAnimatingOverlay.animOverlayCycle
```
</details>
<details>
<summary><code>class C_BaseCombatCharacter extends C_BaseAnimatingOverlay</code></summary>

```
{
	m_currentFrame.weaponGettingSwitchedOut: EHANDLE,
	m_currentFrame.showActiveWeapon3p: Bool,
	m_deathVelocity: Float,
	m_phaseShiftFlags: Int,
	m_flNextAttack: Time,
	m_lastFiredTime: Time,
	m_lastFiredWeapon: EHANDLE,
	m_raiseFromMeleeEndTime: Time,
	m_sharedEnergyCount: Int,
	m_sharedEnergyTotal: Int,
	m_sharedEnergyLockoutThreshold: Int,
	m_lastSharedEnergyRegenTime: Time,
	m_sharedEnergyRegenRate: Time,
	m_sharedEnergyRegenDelay: Float,
	m_lastSharedEnergyTakeTime: Time,
	m_inventory: WeaponInventory_Client,
	m_selectedWeapons: Char,
	m_latestPrimaryWeapons: EHANDLE,
	m_latestPrimaryWeaponsIndexZeroOrOne: EHANDLE,
	m_latestNonOffhandWeapons: Char,
	m_selectedOffhands: Char,
	m_selectedOffhandsPendingHybridAction: Char,
	m_lastCycleSlot: Char,
	m_latestMeleeWeapon: EHANDLE,
	m_weaponPermission: Int,
	m_weaponDelayEnableTime: Time,
	m_weaponDisabledInScript: Bool,
	m_weaponDisabledFlags: Char,
	m_hudInfo_visibilityTestAlwaysPasses: Bool,
	m_contextAction: Int,
	m_phaseShiftTimeStart: Time,
	m_phaseShiftTimeEnd: Time,
}
```

### Offsets

```
C_BaseCombatCharacter!0x00c8 m_currentFrame.weaponGettingSwitchedOut
C_BaseCombatCharacter!0x00d0 m_currentFrame.showActiveWeapon3p
C_BaseCombatCharacter!0x0410 m_deathVelocity
C_BaseCombatCharacter!0x0750 m_phaseShiftFlags
C_BaseCombatCharacter!0x1680 m_flNextAttack
C_BaseCombatCharacter!0x1684 m_lastFiredTime
C_BaseCombatCharacter!0x1688 m_lastFiredWeapon
C_BaseCombatCharacter!0x168c m_raiseFromMeleeEndTime
C_BaseCombatCharacter!0x1690 m_sharedEnergyCount
C_BaseCombatCharacter!0x1694 m_sharedEnergyTotal
C_BaseCombatCharacter!0x1698 m_sharedEnergyLockoutThreshold
C_BaseCombatCharacter!0x169c m_lastSharedEnergyRegenTime
C_BaseCombatCharacter!0x16a0 m_sharedEnergyRegenRate
C_BaseCombatCharacter!0x16a4 m_sharedEnergyRegenDelay
C_BaseCombatCharacter!0x16a8 m_lastSharedEnergyTakeTime
C_BaseCombatCharacter!0x16b0 m_inventory
C_BaseCombatCharacter!0x1700 m_selectedWeapons
C_BaseCombatCharacter!0x1704 m_latestPrimaryWeapons
C_BaseCombatCharacter!0x170c m_latestPrimaryWeaponsIndexZeroOrOne
C_BaseCombatCharacter!0x1714 m_latestNonOffhandWeapons
C_BaseCombatCharacter!0x1716 m_selectedOffhands
C_BaseCombatCharacter!0x1719 m_selectedOffhandsPendingHybridAction
C_BaseCombatCharacter!0x171c m_lastCycleSlot
C_BaseCombatCharacter!0x1720 m_latestMeleeWeapon
C_BaseCombatCharacter!0x1724 m_weaponPermission
C_BaseCombatCharacter!0x1728 m_weaponDelayEnableTime
C_BaseCombatCharacter!0x172c m_weaponDisabledInScript
C_BaseCombatCharacter!0x1751 m_weaponDisabledFlags
C_BaseCombatCharacter!0x1752 m_hudInfo_visibilityTestAlwaysPasses
C_BaseCombatCharacter!0x1764 m_contextAction
C_BaseCombatCharacter!0x1790 m_phaseShiftTimeStart
C_BaseCombatCharacter!0x1794 m_phaseShiftTimeEnd
```
</details>
<details>
<summary><code>class C_BaseEntity</code></summary>

```
{
	m_iEFlags: Int,
	m_fFlags: Int,
	m_currentFrame.modelIndex: Short,
	m_currentFrame.viewOffset: Vector,
	m_vecAngVelocity: Vector,
	m_angAbsRotation: Vector,
	m_vecAbsVelocity: Vector,
	m_vecAbsOrigin: Vector,
	m_localOrigin: Vector,
	m_localAngles: Vector,
	m_flGravity: Float,
	m_flProxyRandomValue: Float,
	m_hGroundEntity: EHANDLE,
	m_flMaxspeed: Float,
	m_visibilityFlags: Int,
	m_fEffects: Int,
	m_iTeamNum: Int,
	m_passThroughFlags: Int,
	m_passThroughThickness: Int,
	m_passThroughDirection: Float,
	m_deathVelocity: Vector,
	m_vecVelocity: Vector,
	m_angNetworkAngles: Vector,
	m_flFriction: Float,
	m_hOwnerEntity: EHANDLE,
	m_bRenderWithViewModels: Bool,
	m_nRenderFX: Char,
	m_nRenderMode: Char,
	m_MoveType: Char,
	m_MoveCollide: Char,
	m_Collision: CCollisionProperty,
}
```

### Offsets

```
C_BaseEntity!0x0058 m_iEFlags
C_BaseEntity!0x0098 m_fFlags
C_BaseEntity!0x00a8 m_currentFrame.modelIndex
C_BaseEntity!0x00b8 m_currentFrame.viewOffset
C_BaseEntity!0x0128 m_vecAngVelocity
C_BaseEntity!0x0134 m_angAbsRotation
C_BaseEntity!0x0140 m_vecAbsVelocity
C_BaseEntity!0x014c m_vecAbsOrigin
C_BaseEntity!0x0158 m_localOrigin
C_BaseEntity!0x0164 m_localAngles
C_BaseEntity!0x03c8 m_flGravity
C_BaseEntity!0x03cc m_flProxyRandomValue
C_BaseEntity!0x03dc m_hGroundEntity
C_BaseEntity!0x03e4 m_flMaxspeed
C_BaseEntity!0x03e8 m_visibilityFlags
C_BaseEntity!0x03ec m_fEffects
C_BaseEntity!0x03f0 m_iTeamNum
C_BaseEntity!0x0404 m_passThroughFlags
C_BaseEntity!0x0408 m_passThroughThickness
C_BaseEntity!0x040c m_passThroughDirection
C_BaseEntity!0x0410 m_deathVelocity
C_BaseEntity!0x041c m_vecVelocity
C_BaseEntity!0x0428 m_angNetworkAngles
C_BaseEntity!0x0434 m_flFriction
C_BaseEntity!0x043c m_hOwnerEntity
C_BaseEntity!0x0440 m_bRenderWithViewModels
C_BaseEntity!0x0441 m_nRenderFX
C_BaseEntity!0x0451 m_nRenderMode
C_BaseEntity!0x0452 m_MoveType
C_BaseEntity!0x0453 m_MoveCollide
C_BaseEntity!0x0458 m_Collision
```
</details>
<details>
<summary><code>class C_BaseEntity</code></summary>

```
{
	m_ModelName: String,
	m_fFlags: Int,
	m_angAbsRotation: Vector,
	m_vecAbsOrigin: PositionVector,
	m_vecPrevAbsOrigin: PositionVector,
	m_flGravity: Float,
	m_rgflCoordinateFrame: Float,
}
```

### Offsets

```
C_BaseEntity!0x0030 m_ModelName
C_BaseEntity!0x0098 m_fFlags
C_BaseEntity!0x0134 m_angAbsRotation
C_BaseEntity!0x014c m_vecAbsOrigin
C_BaseEntity!0x03bc m_vecPrevAbsOrigin
C_BaseEntity!0x03c8 m_flGravity
C_BaseEntity!0x0870 m_rgflCoordinateFrame
```
</details>
<details>
<summary><code>class C_BreakableSurface extends C_BaseEntity</code></summary>

```
{
	m_nPanelBits: Char,
}
```

### Offsets

```
C_BreakableSurface!0x0c88 m_nPanelBits
```
</details>
<details>
<summary><code>class C_ClientRagdoll extends C_BaseEntity</code></summary>

```
{
	m_clrRender: Color32,
	m_nRenderFX: Char,
	m_nRenderMode: Char,
	m_pRagdoll: CRagdoll,
	m_nSkin: Int,
	m_skinMod: Short,
	m_nBody: Int,
	m_bFadeOut: Bool,
	m_bImportant: Bool,
	m_flEffectTime: Time,
	m_iCurrentFriction: Int,
	m_iMinFriction: Int,
	m_iMaxFriction: Int,
	m_flFrictionModTime: Float,
	m_flFrictionTime: Time,
	m_iFrictionAnimState: Int,
	m_bReleaseRagdoll: Bool,
	m_bFadingOut: Bool,
	m_flScaleEnd: Float,
	m_flScaleTimeStart: Float,
	m_flScaleTimeEnd: Float,
}
```

### Offsets

```
C_ClientRagdoll!0x0050 m_clrRender
C_ClientRagdoll!0x0441 m_nRenderFX
C_ClientRagdoll!0x0451 m_nRenderMode
C_ClientRagdoll!0x0b88 m_pRagdoll
C_ClientRagdoll!0x0e48 m_nSkin
C_ClientRagdoll!0x0e4c m_skinMod
C_ClientRagdoll!0x0e50 m_nBody
C_ClientRagdoll!0x1300 m_bFadeOut
C_ClientRagdoll!0x1301 m_bImportant
C_ClientRagdoll!0x1304 m_flEffectTime
C_ClientRagdoll!0x1308 m_iCurrentFriction
C_ClientRagdoll!0x130c m_iMinFriction
C_ClientRagdoll!0x1310 m_iMaxFriction
C_ClientRagdoll!0x1314 m_flFrictionModTime
C_ClientRagdoll!0x1318 m_flFrictionTime
C_ClientRagdoll!0x131c m_iFrictionAnimState
C_ClientRagdoll!0x1320 m_bReleaseRagdoll
C_ClientRagdoll!0x1321 m_bFadingOut
C_ClientRagdoll!0x1324 m_flScaleEnd
C_ClientRagdoll!0x134c m_flScaleTimeStart
C_ClientRagdoll!0x1374 m_flScaleTimeEnd
```
</details>
<details>
<summary><code>class C_CrossbowBolt extends C_Projectile</code></summary>

```
{
	m_bounceCount: Int,
	m_maxBounceCount: Int,
	m_doesGrow: Bool,
	m_growStartSize: Float,
	m_growStage1Tick: Tick,
	m_growStage1Size: Float,
	m_growStage2Tick: Tick,
	m_growStage2Size: Float,
	m_growStageFinalTick: Tick,
	m_growStageFinalSize: Float,
}
```

### Offsets

```
C_CrossbowBolt!0x2800 m_bounceCount
C_CrossbowBolt!0x2804 m_maxBounceCount
C_CrossbowBolt!0x2808 m_doesGrow
C_CrossbowBolt!0x280c m_growStartSize
C_CrossbowBolt!0x2810 m_growStage1Tick
C_CrossbowBolt!0x2814 m_growStage1Size
C_CrossbowBolt!0x2818 m_growStage2Tick
C_CrossbowBolt!0x281c m_growStage2Size
C_CrossbowBolt!0x2820 m_growStageFinalTick
C_CrossbowBolt!0x2824 m_growStageFinalSize
```
</details>
<details>
<summary><code>class C_DynamicProp extends C_BaseEntity</code></summary>

```
{
	m_bClientSide: Bool,
}
```

### Offsets

```
C_DynamicProp!0x1300 m_bClientSide
```
</details>
<details>
<summary><code>class C_EnvWindShared</code></summary>

```
{
	m_flStartTime: Float,
	m_iWindSeed: Int,
	m_iMinWind: Int,
	m_iMaxWind: Int,
	m_windRadius: Int,
	m_iMinGust: Int,
	m_iMaxGust: Int,
	m_flMinGustDelay: Float,
	m_flMaxGustDelay: Float,
	m_flGustDuration: Float,
	m_iGustDirChange: Int,
	m_location: Vector,
	m_iszGustSound: Int,
	m_iWindDir: Int,
	m_flWindSpeed: Float,
	m_currentWindVector: Vector,
	m_CurrentSwayVector: Vector,
	m_PrevSwayVector: Vector,
	m_iInitialWindDir: Int,
	m_flInitialWindSpeed: Float,
	m_flVariationTime: Float,
	m_flSimTime: Float,
	m_flSwitchTime: Float,
	m_flAveWindSpeed: Float,
	m_bGusting: Bool,
	m_flWindAngleVariation: Float,
	m_flWindSpeedVariation: Float,
	m_iEntIndex: Int,
	m_Stream: Void,
	m_WindVariationStream: Void,
	m_WindAveQueue: Void,
	m_WindVariationQueue: Void,
}
```

### Offsets

```
C_EnvWindShared!0x0008 m_flStartTime
C_EnvWindShared!0x000c m_iWindSeed
C_EnvWindShared!0x0010 m_iMinWind
C_EnvWindShared!0x0014 m_iMaxWind
C_EnvWindShared!0x0018 m_windRadius
C_EnvWindShared!0x001c m_iMinGust
C_EnvWindShared!0x0020 m_iMaxGust
C_EnvWindShared!0x0024 m_flMinGustDelay
C_EnvWindShared!0x0028 m_flMaxGustDelay
C_EnvWindShared!0x002c m_flGustDuration
C_EnvWindShared!0x0030 m_iGustDirChange
C_EnvWindShared!0x0034 m_location
C_EnvWindShared!0x0040 m_iszGustSound
C_EnvWindShared!0x0044 m_iWindDir
C_EnvWindShared!0x0048 m_flWindSpeed
C_EnvWindShared!0x004c m_currentWindVector
C_EnvWindShared!0x0058 m_CurrentSwayVector
C_EnvWindShared!0x0064 m_PrevSwayVector
C_EnvWindShared!0x0070 m_iInitialWindDir
C_EnvWindShared!0x0074 m_flInitialWindSpeed
C_EnvWindShared!0x0078 m_flVariationTime
C_EnvWindShared!0x007c m_flSimTime
C_EnvWindShared!0x0080 m_flSwitchTime
C_EnvWindShared!0x0084 m_flAveWindSpeed
C_EnvWindShared!0x0088 m_bGusting
C_EnvWindShared!0x008c m_flWindAngleVariation
C_EnvWindShared!0x0090 m_flWindSpeedVariation
C_EnvWindShared!0x0094 m_iEntIndex
C_EnvWindShared!0x0098 m_Stream
C_EnvWindShared!0x00d0 m_WindVariationStream
C_EnvWindShared!0x0108 m_WindAveQueue
C_EnvWindShared!0x0140 m_WindVariationQueue
```
</details>
<details>
<summary><code>class C_FogController extends C_BaseEntity</code></summary>

```
{
	m_fogParams: fogplayerparamsstate_t,
	m_fogAngles: Vector,
	m_useAbsAngles: Bool,
}
```

### Offsets

```
C_FogController!0x0a00 m_fogParams
C_FogController!0x0a68 m_fogAngles
C_FogController!0x0a74 m_useAbsAngles
```
</details>
<details>
<summary><code>class C_FogVolume extends C_BaseEntity</code></summary>

```
{
	m_volumeTester: Outer,
	m_fogTarget: ClassPtr,
	m_fogTargetName: String,
	m_fogPriority: Int,
}
```

### Offsets

```
C_FogVolume!0x0a00 m_volumeTester
C_FogVolume!0x0a08 m_fogTarget
C_FogVolume!0x0a10 m_fogTargetName
C_FogVolume!0x0a18 m_fogPriority
```
</details>
<details>
<summary><code>class C_GlobalNonRewinding extends C_BaseEntity</code></summary>

```
{
	m_playerObserver: C_ObserverMode,
}
```

### Offsets

```
C_GlobalNonRewinding!0x0a00 m_playerObserver
```
</details>
<details>
<summary><code>class C_KnockBack</code></summary>

```
{
	velocity: Vector,
	beginTime: Time,
	endTime: Time,
}
```

### Offsets

```
C_KnockBack!0x0008 velocity
C_KnockBack!0x0014 beginTime
C_KnockBack!0x0018 endTime
```
</details>
<details>
<summary><code>class C_Missile extends C_Projectile</code></summary>

```
{
	m_hasPlayedWhizby: Bool,
	m_whizByStart: Vector,
	m_whizBySoundName: Char,
	m_homingSpeed: Float,
	m_homingSpeedDodgingPlayer: Float,
	m_launchDir: Vector,
	m_hSpecificTarget: EHANDLE,
	m_targetOffset: Vector,
	m_targetPosition: Vector,
	m_useTargetPosition: Bool,
	m_postIgnitionSpeed: Float,
	m_flGracePeriodEndsAt: Time,
	m_pathSettingsInitialized: Bool,
	m_expandContractMissile: Bool,
	m_spiralMissile: Bool,
	m_spiralSettings: Void,
	m_expandContractSettings: MissilePathExpandContractSettings_Client,
	m_lastThinkTime: Time,
	m_explosionIgnoreEntity: EHANDLE,
}
```

### Offsets

```
C_Missile!0x2800 m_hasPlayedWhizby
C_Missile!0x2804 m_whizByStart
C_Missile!0x2810 m_whizBySoundName
C_Missile!0x2850 m_homingSpeed
C_Missile!0x2854 m_homingSpeedDodgingPlayer
C_Missile!0x2858 m_launchDir
C_Missile!0x2864 m_hSpecificTarget
C_Missile!0x2868 m_targetOffset
C_Missile!0x2874 m_targetPosition
C_Missile!0x2880 m_useTargetPosition
C_Missile!0x2884 m_postIgnitionSpeed
C_Missile!0x2888 m_flGracePeriodEndsAt
C_Missile!0x288c m_pathSettingsInitialized
C_Missile!0x288d m_expandContractMissile
C_Missile!0x288e m_spiralMissile
C_Missile!0x2890 m_spiralSettings
C_Missile!0x28b4 m_expandContractSettings
C_Missile!0x28ec m_lastThinkTime
C_Missile!0x28f0 m_explosionIgnoreEntity
```
</details>
<details>
<summary><code>class C_NPC_SentryTurret extends C_BaseEntity</code></summary>

```
{
	m_killCount: Int,
	m_titanKillCount: Int,
}
```

### Offsets

```
C_NPC_SentryTurret!0x1a44 m_killCount
C_NPC_SentryTurret!0x1a48 m_titanKillCount
```
</details>
<details>
<summary><code>class C_ObserverMode</code></summary>

```
{
	m_observerMode: Int,
	m_observerTarget: EHANDLE,
}
```

### Offsets

```
C_ObserverMode!0x0000 m_observerMode
C_ObserverMode!0x0004 m_observerTarget
```
</details>
<details>
<summary><code>class C_ParticleSystem extends C_BaseEntity</code></summary>

```
{
	m_bClientSide: Bool,
	m_bActive: Bool,
	m_warmUpTime: Float,
	m_pauseAfterWarmup: Bool,
	m_bInSkybox: Bool,
	m_killForReplay: Bool,
	m_killIfOverLimit: Bool,
}
```

### Offsets

```
C_ParticleSystem!0x0a08 m_bClientSide
C_ParticleSystem!0x0a09 m_bActive
C_ParticleSystem!0x0a10 m_warmUpTime
C_ParticleSystem!0x0a14 m_pauseAfterWarmup
C_ParticleSystem!0x0a15 m_bInSkybox
C_ParticleSystem!0x0a16 m_killForReplay
C_ParticleSystem!0x0a17 m_killIfOverLimit
```
</details>
<details>
<summary><code>class C_Player extends C_BaseCombatCharacter</code></summary>

```
{
	m_fFlags: Int,
	m_pMoveParent: EHANDLE,
	m_vecAbsVelocity: Vector,
	m_hGroundEntity: EHANDLE,
	m_flMaxspeed: Int,
	m_vecVelocity: Vector,
	m_flFriction: Float,
	m_nNextThinkTick: Int,
	m_SequenceTransitioner: C_SequenceTransitioner,
	m_bZooming: Bool,
	m_zoomToggleOnStartTime: Time,
	m_zoomBaseFrac: Float,
	m_zoomBaseTime: Time,
	m_zoomFullStartTime: Time,
	m_lastUCmdSimulationTicks: Int,
	m_lastUCmdSimulationRemainderTime: Float,
	m_Local: C_PlayerLocalData,
	m_currentFramePlayer.timeBase: Float,
	m_currentFramePlayer.statusEffectsTimedPlayerCUR: StatusEffectTimedData,
	m_currentFramePlayer.statusEffectsEndlessPlayerCUR: StatusEffectEndlessData,
	m_currentFramePlayer.m_flHullHeight: Float,
	m_currentFramePlayer.m_traversalAnimProgress: Float,
	m_currentFramePlayer.m_sprintTiltFrac: Float,
	m_currentFramePlayer.m_ammoPoolCount: Int,
	m_currentFrameLocalPlayer.m_stepSmoothingOffset: Vector,
	m_currentFrameLocalPlayer.m_duckTransitionRemainderMsec: Int,
	m_currentFrameLocalPlayer.m_vecPunchBase_Angle: Vector,
	m_currentFrameLocalPlayer.m_vecPunchBase_AngleVel: Vector,
	m_currentFrameLocalPlayer.m_vecPunchWeapon_Angle: Vector,
	m_currentFrameLocalPlayer.m_vecPunchWeapon_AngleVel.x: Float,
	m_currentFrameLocalPlayer.m_vecPunchWeapon_AngleVel.y: Float,
	m_currentFrameLocalPlayer.m_vecPunchWeapon_AngleVel.z: Float,
	m_currentFrameLocalPlayer.m_localGravityRotation: Quaternion,
	pl: CPlayerState,
	m_ammoPoolCapacity: Int,
	m_gestureSequences: Int,
	m_gestureStartTimes: Time,
	m_gestureBlendInDuration: Float,
	m_gestureBlendOutDuration: Float,
	m_gestureFadeOutStartTime: Time,
	m_gestureFadeOutDuration: Float,
	m_gestureAutoKillBitfield: Int,
	m_afButtonLast: Int,
	m_afButtonPressed: Int,
	m_afButtonReleased: Int,
	m_nButtons: Int,
	m_nImpulse: Int,
	m_flPhysics: Int,
	m_flStepSoundTime: Float,
	m_flTimeAllSuitDevicesOff: Float,
	m_fStickySprintMinTime: Float,
	m_bPlayedSprintStartEffects: Bool,
	m_fIsSprinting: Bool,
	m_fIsWalking: Bool,
	m_sprintStartedTime: Time,
	m_sprintStartedFrac: Float,
	m_sprintEndedTime: Time,
	m_sprintEndedFrac: Float,
	m_stickySprintStartTime: Time,
	m_damageImpulseNoDecelEndTime: Time,
	m_duckState: Int,
	m_leanState: Int,
	m_doingHalfDuck: Bool,
	m_canStand: Bool,
	m_StandHullMin: Vector,
	m_StandHullMax: Vector,
	m_DuckHullMin: Vector,
	m_DuckHullMax: Vector,
	m_upDir: Vector,
	m_upDirPredicted: Vector,
	m_lastWallRunStartPos: Vector,
	m_wallRunCount: Int,
	m_wallRunWeak: Bool,
	m_shouldBeOneHanded: Bool,
	m_oneHandFraction: Float,
	m_animAimPitch: Float,
	m_animAimYaw: Float,
	m_wallRunPushAwayTime: Float,
	m_wallrunRetryTime: Time,
	m_wallrunRetryPos: Vector,
	m_wallrunRetryNormal: Vector,
	m_wallHangTime: Float,
	m_traversalState: Int,
	m_traversalType: Int,
	m_traversalBegin: Vector,
	m_traversalMid: Vector,
	m_traversalEnd: Vector,
	m_traversalMidFrac: Float,
	m_traversalForwardDir: Vector,
	m_traversalRefPos: Vector,
	m_traversalProgress: Float,
	m_traversalStartTime: Time,
	m_traversalHandAppearTime: Time,
	m_traversalReleaseTime: Time,
	m_traversalBlendOutStartTime: Time,
	m_traversalBlendOutStartOffset: Vector,
	m_traversalYawDelta: Float,
	m_wallDangleJumpOffTime: Time,
	m_wallDangleMayHangHere: Bool,
	m_wallDangleForceFallOff: Bool,
	m_wallDangleLastPushedForward: Bool,
	m_wallDangleDisableWeapon: Int,
	m_wallDangleClimbProgressFloor: Float,
	m_wallClimbSetUp: Bool,
	m_wallHanging: Bool,
	m_grapple: GrappleData,
	m_grapple: GrappleData,
	m_grappleActive: Bool,
	m_grappleActive: Bool,
	m_grappleNeedWindowCheck: Bool,
	m_grappleNextWindowHint: EHANDLE,
	m_slowMoEnabled: Bool,
	m_sliding: Bool,
	m_slideLongJumpAllowed: Bool,
	m_lastSlideTime: Time,
	m_lastSlideBoost: Float,
	m_gravityGrenadeStatusEffect: Int,
	m_bIsStickySprinting: Bool,
	m_prevMoveYaw: Float,
	m_sprintTiltVel: Float,
	m_turret: EHANDLE,
	m_hViewModels: EHANDLE,
	m_viewOffsetEntity: Player_ViewOffsetEntityData,
	m_activeZipline: EHANDLE,
	m_lastZipline: EHANDLE,
	m_lastZiplineDetachTime: Time,
	m_ziplineValid3pWeaponLayerAnim: Bool,
	m_ziplineState: Int,
	m_zipline: PlayerZiplineData_Client,
	m_ziplineViewOffsetPosition: Vector,
	m_ziplineViewOffsetVelocity: Vector,
	m_ziplineGrenadeEntity: EHANDLE,
	m_ziplineGrenadeBeginStationEntity: EHANDLE,
	m_ziplineGrenadeBeginStationAttachmentIndex: Int,
	m_playAnimationType: Int,
	m_detachGrappleOnPlayAnimationEnd: Bool,
	m_playAnimationNext: Int,
	m_boosting: Bool,
	m_activateBoost: Bool,
	m_repeatedBoost: Bool,
	m_boostMeter: Float,
	m_jetpack: Bool,
	m_activateJetpack: Bool,
	m_jetpackAfterburner: Bool,
	m_gliding: Bool,
	m_glideMeter: Float,
	m_glideRechargeDelayAccumulator: Float,
	m_hovering: Bool,
	m_isPerformingBoostAction: Bool,
	m_lastJumpHeight: Float,
	m_slipAirRestrictDirection: Vector,
	m_slipAirRestrictTime: Time,
	m_melee: PlayerMelee_PlayerData,
	m_useCredit: Bool,
	m_wallRunStartTime: Time,
	m_wallRunClearTime: Time,
	m_onSlopeTime: Float,
	m_lastWallNormal: Vector,
	m_dodging: Bool,
	m_lastDodgeTime: Time,
	m_vecPreviouslyPredictedOrigin: Vector,
	m_flTimeLastTouchedWall: Float,
	m_timeJetpackHeightActivateCheckPassed: Time,
	m_flTimeLastTouchedGround: Float,
	m_flTimeLastJumped: Float,
	m_flTimeLastLanded: Float,
	m_flLastLandFromHeight: Float,
	m_usePressedTime: Float,
	m_lastUseTime: Float,
	m_lastFakeFloorPos: Vector,
	m_bHasJumpedSinceTouchedGround: Bool,
	m_bDoMultiJumpPenalty: Bool,
	m_dodgingInAir: Bool,
	m_activeViewmodelModifiers: Bool,
	m_lastMoveInputTime: Time,
	m_ignoreEntityForMovementUntilNotTouching: EHANDLE,
	m_gameMovementUtil.m_surfaceFriction: Float,
	m_lungeTargetEntity: EHANDLE,
	m_isLungingToPosition: Bool,
	m_lungeTargetPosition: Vector,
	m_lungeStartPositionOffset: Vector,
	m_lungeEndPositionOffset: Vector,
	m_lungeStartTime: Time,
	m_lungeEndTime: Time,
	m_lungeCanFly: Bool,
	m_lungeLockPitch: Bool,
	m_lungeStartPitch: Float,
	m_lungeSmoothTime: Float,
	m_lungeMaxTime: Float,
	m_lungeMaxEndSpeed: Float,
	m_vPrevGroundNormal: Vector,
	m_pushAwayFromTopAcceleration: Vector,
	m_controllerModeActive: Bool,
	m_skydiveForwardPoseValueVelocity: Float,
	m_skydiveForwardPoseValueTarget: Float,
	m_skydiveForwardPoseValueCurrent: Float,
	m_skydiveSidePoseValueVelocity: Float,
	m_skydiveSidePoseValueTarget: Float,
	m_skydiveSidePoseValueCurrent: Float,
	m_skydiveYawVelocity: Float,
	m_freefallState: Int,
	m_freefallStartTime: Time,
	m_freefallEndTime: Time,
	m_freefallAnticipateStartTime: Time,
	m_freefallAnticipateEndTime: Time,
	m_freefallDistanceToLand: Float,
	m_skydiveDiveAngle: Float,
	m_skydiveIsDiving: Bool,
	m_skydiveSpeed: Float,
	m_skydiveStrafeAngle: Float,
	m_skydiveFreelookEnabled: Bool,
	m_skydiveFreelookLockedAngle: Vector,
	m_skydivePlayerPitch: Float,
	m_skydivePlayerYaw: Float,
	m_skydiveFollowing: Bool,
	m_skydiveUnfollowVelocity: Vector,
	m_skydiveIsNearLeviathan: Bool,
	m_skydiveLeviathanHitPosition: Vector,
	m_skydiveLeviathanHitNormal: Vector,
	m_skydiveSlipVelocity: Vector,
	m_playerKnockBacks: C_KnockBack,
	m_updraftStage: Int,
	m_updraftSlowTime: Time,
}
```

### Offsets

```
C_Player!0x0098 m_fFlags
C_Player!0x0118 m_pMoveParent
C_Player!0x0140 m_vecAbsVelocity
C_Player!0x03dc m_hGroundEntity
C_Player!0x03e4 m_flMaxspeed
C_Player!0x041c m_vecVelocity
C_Player!0x0434 m_flFriction
C_Player!0x050c m_nNextThinkTick
C_Player!0x0bc0 m_SequenceTransitioner
C_Player!0x1881 m_bZooming
C_Player!0x1884 m_zoomToggleOnStartTime
C_Player!0x1888 m_zoomBaseFrac
C_Player!0x188c m_zoomBaseTime
C_Player!0x1890 m_zoomFullStartTime
C_Player!0x1910 m_lastUCmdSimulationTicks
C_Player!0x1914 m_lastUCmdSimulationRemainderTime
C_Player!0x1a30 m_Local
C_Player!0x1d18 m_currentFramePlayer.timeBase
C_Player!0x1d20 m_currentFramePlayer.statusEffectsTimedPlayerCUR
C_Player!0x1e10 m_currentFramePlayer.statusEffectsEndlessPlayerCUR
C_Player!0x1eb0 m_currentFramePlayer.m_flHullHeight
C_Player!0x1eb4 m_currentFramePlayer.m_traversalAnimProgress
C_Player!0x1eb8 m_currentFramePlayer.m_sprintTiltFrac
C_Player!0x1ec8 m_currentFramePlayer.m_ammoPoolCount
C_Player!0x2098 m_currentFrameLocalPlayer.m_stepSmoothingOffset
C_Player!0x20a4 m_currentFrameLocalPlayer.m_duckTransitionRemainderMsec
C_Player!0x20a8 m_currentFrameLocalPlayer.m_vecPunchBase_Angle
C_Player!0x20b4 m_currentFrameLocalPlayer.m_vecPunchBase_AngleVel
C_Player!0x20c0 m_currentFrameLocalPlayer.m_vecPunchWeapon_Angle
C_Player!0x20cc m_currentFrameLocalPlayer.m_vecPunchWeapon_AngleVel.x
C_Player!0x20d0 m_currentFrameLocalPlayer.m_vecPunchWeapon_AngleVel.y
C_Player!0x20d4 m_currentFrameLocalPlayer.m_vecPunchWeapon_AngleVel.z
C_Player!0x2108 m_currentFrameLocalPlayer.m_localGravityRotation
C_Player!0x2118 pl
C_Player!0x219c m_ammoPoolCapacity
C_Player!0x24fc m_gestureSequences
C_Player!0x251c m_gestureStartTimes
C_Player!0x253c m_gestureBlendInDuration
C_Player!0x255c m_gestureBlendOutDuration
C_Player!0x257c m_gestureFadeOutStartTime
C_Player!0x259c m_gestureFadeOutDuration
C_Player!0x25bc m_gestureAutoKillBitfield
C_Player!0x25d8 m_afButtonLast
C_Player!0x25dc m_afButtonPressed
C_Player!0x25e0 m_afButtonReleased
C_Player!0x25e4 m_nButtons
C_Player!0x25e8 m_nImpulse
C_Player!0x25ec m_flPhysics
C_Player!0x25f0 m_flStepSoundTime
C_Player!0x25f4 m_flTimeAllSuitDevicesOff
C_Player!0x25f8 m_fStickySprintMinTime
C_Player!0x25fc m_bPlayedSprintStartEffects
C_Player!0x2604 m_fIsSprinting
C_Player!0x2605 m_fIsWalking
C_Player!0x2608 m_sprintStartedTime
C_Player!0x260c m_sprintStartedFrac
C_Player!0x2610 m_sprintEndedTime
C_Player!0x2614 m_sprintEndedFrac
C_Player!0x2618 m_stickySprintStartTime
C_Player!0x261c m_damageImpulseNoDecelEndTime
C_Player!0x262c m_duckState
C_Player!0x2630 m_leanState
C_Player!0x2634 m_doingHalfDuck
C_Player!0x2635 m_canStand
C_Player!0x2638 m_StandHullMin
C_Player!0x2644 m_StandHullMax
C_Player!0x2650 m_DuckHullMin
C_Player!0x265c m_DuckHullMax
C_Player!0x266c m_upDir
C_Player!0x2678 m_upDirPredicted
C_Player!0x2684 m_lastWallRunStartPos
C_Player!0x2690 m_wallRunCount
C_Player!0x2694 m_wallRunWeak
C_Player!0x2695 m_shouldBeOneHanded
C_Player!0x2698 m_oneHandFraction
C_Player!0x269c m_animAimPitch
C_Player!0x26a0 m_animAimYaw
C_Player!0x26a4 m_wallRunPushAwayTime
C_Player!0x26b0 m_wallrunRetryTime
C_Player!0x26b4 m_wallrunRetryPos
C_Player!0x26c0 m_wallrunRetryNormal
C_Player!0x26e4 m_wallHangTime
C_Player!0x26e8 m_traversalState
C_Player!0x26ec m_traversalType
C_Player!0x26f0 m_traversalBegin
C_Player!0x26fc m_traversalMid
C_Player!0x2708 m_traversalEnd
C_Player!0x2714 m_traversalMidFrac
C_Player!0x2718 m_traversalForwardDir
C_Player!0x2724 m_traversalRefPos
C_Player!0x2730 m_traversalProgress
C_Player!0x2734 m_traversalStartTime
C_Player!0x2738 m_traversalHandAppearTime
C_Player!0x273c m_traversalReleaseTime
C_Player!0x2740 m_traversalBlendOutStartTime
C_Player!0x2744 m_traversalBlendOutStartOffset
C_Player!0x2750 m_traversalYawDelta
C_Player!0x275c m_wallDangleJumpOffTime
C_Player!0x2760 m_wallDangleMayHangHere
C_Player!0x2761 m_wallDangleForceFallOff
C_Player!0x2762 m_wallDangleLastPushedForward
C_Player!0x2764 m_wallDangleDisableWeapon
C_Player!0x2768 m_wallDangleClimbProgressFloor
C_Player!0x276c m_wallClimbSetUp
C_Player!0x276d m_wallHanging
C_Player!0x2770 m_grapple
C_Player!0x2770 m_grapple
C_Player!0x2800 m_grappleActive
C_Player!0x2800 m_grappleActive
C_Player!0x2801 m_grappleNeedWindowCheck
C_Player!0x2804 m_grappleNextWindowHint
C_Player!0x2814 m_slowMoEnabled
C_Player!0x2815 m_sliding
C_Player!0x2816 m_slideLongJumpAllowed
C_Player!0x2818 m_lastSlideTime
C_Player!0x281c m_lastSlideBoost
C_Player!0x2820 m_gravityGrenadeStatusEffect
C_Player!0x2824 m_bIsStickySprinting
C_Player!0x2828 m_prevMoveYaw
C_Player!0x282c m_sprintTiltVel
C_Player!0x2848 m_turret
C_Player!0x284c m_hViewModels
C_Player!0x2860 m_viewOffsetEntity
C_Player!0x29a0 m_activeZipline
C_Player!0x29a4 m_lastZipline
C_Player!0x29a8 m_lastZiplineDetachTime
C_Player!0x29ac m_ziplineValid3pWeaponLayerAnim
C_Player!0x29b0 m_ziplineState
C_Player!0x29b8 m_zipline
C_Player!0x2a28 m_ziplineViewOffsetPosition
C_Player!0x2a34 m_ziplineViewOffsetVelocity
C_Player!0x2a40 m_ziplineGrenadeEntity
C_Player!0x2a44 m_ziplineGrenadeBeginStationEntity
C_Player!0x2a48 m_ziplineGrenadeBeginStationAttachmentIndex
C_Player!0x2a54 m_playAnimationType
C_Player!0x2a58 m_detachGrappleOnPlayAnimationEnd
C_Player!0x2a5c m_playAnimationNext
C_Player!0x2a70 m_boosting
C_Player!0x2a71 m_activateBoost
C_Player!0x2a72 m_repeatedBoost
C_Player!0x2a74 m_boostMeter
C_Player!0x2a78 m_jetpack
C_Player!0x2a79 m_activateJetpack
C_Player!0x2a7a m_jetpackAfterburner
C_Player!0x2a7b m_gliding
C_Player!0x2a7c m_glideMeter
C_Player!0x2a80 m_glideRechargeDelayAccumulator
C_Player!0x2a84 m_hovering
C_Player!0x2a85 m_isPerformingBoostAction
C_Player!0x2a88 m_lastJumpHeight
C_Player!0x2ad8 m_slipAirRestrictDirection
C_Player!0x2ae4 m_slipAirRestrictTime
C_Player!0x2c78 m_melee
C_Player!0x2cb0 m_useCredit
C_Player!0x2fec m_wallRunStartTime
C_Player!0x2ff0 m_wallRunClearTime
C_Player!0x2ff4 m_onSlopeTime
C_Player!0x2ff8 m_lastWallNormal
C_Player!0x3004 m_dodging
C_Player!0x3008 m_lastDodgeTime
C_Player!0x300c m_vecPreviouslyPredictedOrigin
C_Player!0x3024 m_flTimeLastTouchedWall
C_Player!0x3028 m_timeJetpackHeightActivateCheckPassed
C_Player!0x302c m_flTimeLastTouchedGround
C_Player!0x3030 m_flTimeLastJumped
C_Player!0x3034 m_flTimeLastLanded
C_Player!0x3038 m_flLastLandFromHeight
C_Player!0x303c m_usePressedTime
C_Player!0x3040 m_lastUseTime
C_Player!0x3050 m_lastFakeFloorPos
C_Player!0x305c m_bHasJumpedSinceTouchedGround
C_Player!0x305d m_bDoMultiJumpPenalty
C_Player!0x305e m_dodgingInAir
C_Player!0x3214 m_activeViewmodelModifiers
C_Player!0x3460 m_lastMoveInputTime
C_Player!0x3464 m_ignoreEntityForMovementUntilNotTouching
C_Player!0x3930 m_gameMovementUtil.m_surfaceFriction
C_Player!0x39ac m_lungeTargetEntity
C_Player!0x39b0 m_isLungingToPosition
C_Player!0x39b4 m_lungeTargetPosition
C_Player!0x39c0 m_lungeStartPositionOffset
C_Player!0x39cc m_lungeEndPositionOffset
C_Player!0x39d8 m_lungeStartTime
C_Player!0x39dc m_lungeEndTime
C_Player!0x39e0 m_lungeCanFly
C_Player!0x39e1 m_lungeLockPitch
C_Player!0x39e4 m_lungeStartPitch
C_Player!0x39e8 m_lungeSmoothTime
C_Player!0x39ec m_lungeMaxTime
C_Player!0x39f0 m_lungeMaxEndSpeed
C_Player!0x3c80 m_vPrevGroundNormal
C_Player!0x3e44 m_pushAwayFromTopAcceleration
C_Player!0x3e6c m_controllerModeActive
C_Player!0x3e84 m_skydiveForwardPoseValueVelocity
C_Player!0x3e88 m_skydiveForwardPoseValueTarget
C_Player!0x3e8c m_skydiveForwardPoseValueCurrent
C_Player!0x3e90 m_skydiveSidePoseValueVelocity
C_Player!0x3e94 m_skydiveSidePoseValueTarget
C_Player!0x3e98 m_skydiveSidePoseValueCurrent
C_Player!0x3e9c m_skydiveYawVelocity
C_Player!0x3eb8 m_freefallState
C_Player!0x3ebc m_freefallStartTime
C_Player!0x3ec0 m_freefallEndTime
C_Player!0x3ec4 m_freefallAnticipateStartTime
C_Player!0x3ec8 m_freefallAnticipateEndTime
C_Player!0x3ecc m_freefallDistanceToLand
C_Player!0x3ed0 m_skydiveDiveAngle
C_Player!0x3ed4 m_skydiveIsDiving
C_Player!0x3ed8 m_skydiveSpeed
C_Player!0x3edc m_skydiveStrafeAngle
C_Player!0x3ee0 m_skydiveFreelookEnabled
C_Player!0x3ee4 m_skydiveFreelookLockedAngle
C_Player!0x3ef0 m_skydivePlayerPitch
C_Player!0x3ef4 m_skydivePlayerYaw
C_Player!0x3ef8 m_skydiveFollowing
C_Player!0x3efc m_skydiveUnfollowVelocity
C_Player!0x3f09 m_skydiveIsNearLeviathan
C_Player!0x3f0c m_skydiveLeviathanHitPosition
C_Player!0x3f18 m_skydiveLeviathanHitNormal
C_Player!0x3f24 m_skydiveSlipVelocity
C_Player!0x3f48 m_playerKnockBacks
C_Player!0x3fcc m_updraftStage
C_Player!0x3ff0 m_updraftSlowTime
```
</details>
<details>
<summary><code>class C_PlayerLocalData</code></summary>

```
{
	m_nStepside: Int,
	m_nOldButtons: Int,
	m_iHideHUD: Int,
	m_superJumpsUsed: Int,
	m_jumpedOffRodeo: Bool,
	m_jumpPressTime: Time,
	m_jetpackActivateTime: Time,
	m_jetpackDeactivateTime: Time,
	m_flSuitPower: Float,
	m_flSuitJumpPower: Float,
	m_flSuitGrapplePower: Float,
	m_flFallVelocity: Float,
	m_flStepSize: Float,
	m_airSlowMoFrac: Float,
	predictableFlags: Int,
	m_bitsActiveDevices: Int,
	m_forceStance: Int,
	m_duckToggleOn: Bool,
	m_bDrawViewmodel: Bool,
	m_bAllowAutoMovement: Bool,
	m_airMoveBlockPlanes: Vector,
	m_airMoveBlockPlaneTime: Time,
	m_airMoveBlockPlaneCount: Int,
	m_queuedMeleePressTime: Time,
	m_queuedGrappleMeleeTime: Time,
	m_disableMeleeUntilRelease: Bool,
	m_meleePressTime: Time,
	m_meleeDisabledCounter: Int,
	m_meleeInputIndex: Int,
	m_oneHandedWeaponUsage: Bool,
	m_prevOneHandedWeaponUsage: Bool,
	m_titanEmbarkEnabled: Bool,
	m_titanDisembarkEnabled: Bool,
	m_playerAnimStationaryGoalFeetYaw: Float,
	m_playerAnimJumping: Bool,
	m_playerAnimJumpStartTime: Time,
	m_playerAnimFirstJumpFrame: Bool,
	m_playerAnimDodging: Bool,
	m_playerAnimJumpActivity: Int,
	m_playerAnimLanding: Bool,
	m_playerAnimShouldLand: Bool,
	m_playerAnimLandStartTime: Time,
	m_playerAnimInAirWalk: Bool,
	m_playerAnimPrevFrameSequenceMotionYaw: Float,
	m_playerAnimMeleeParity: Int,
	m_playerAnimMeleeStartTime: Time,
	m_playerLocalGravityToWorldTransform: Quaternion,
	m_playerLocalGravityBlendStartRotation: Quaternion,
	m_playerLocalGravityBlendEndRotation: Quaternion,
	m_playerLocalGravityBlendEndDirection: Vector,
	m_playerLocalGravityBlendStartTime: Time,
	m_playerLocalGravityBlendEndTime: Time,
	m_playerLocalGravityBlendStrength: Float,
	m_playerLocalGravityStrength: Float,
	m_playerLocalGravityType: Int,
	m_playerLocalGravityPoint: Vector,
	m_playerLocalGravityLineStart: Vector,
	m_playerLocalGravityLineEnd: Vector,
	m_playerLocalGravityEntity: EHANDLE,
	m_playerLocalGravityLineStartEntity: EHANDLE,
	m_playerLocalGravityLineEndEntity: EHANDLE,
	m_playerFloatLookStartTime: Time,
	m_playerFloatLookEndTime: Time,
	m_wallrunLatestFloorHeight: Float,
	m_groundNormal: Vector,
	m_continuousUseBlocked: Bool,
	m_useEnt: EHANDLE,
}
```

### Offsets

```
C_PlayerLocalData!0x0008 m_nStepside
C_PlayerLocalData!0x000c m_nOldButtons
C_PlayerLocalData!0x0010 m_iHideHUD
C_PlayerLocalData!0x0014 m_superJumpsUsed
C_PlayerLocalData!0x0018 m_jumpedOffRodeo
C_PlayerLocalData!0x001c m_jumpPressTime
C_PlayerLocalData!0x0020 m_jetpackActivateTime
C_PlayerLocalData!0x0024 m_jetpackDeactivateTime
C_PlayerLocalData!0x0028 m_flSuitPower
C_PlayerLocalData!0x002c m_flSuitJumpPower
C_PlayerLocalData!0x0030 m_flSuitGrapplePower
C_PlayerLocalData!0x0034 m_flFallVelocity
C_PlayerLocalData!0x0038 m_flStepSize
C_PlayerLocalData!0x003c m_airSlowMoFrac
C_PlayerLocalData!0x0040 predictableFlags
C_PlayerLocalData!0x0044 m_bitsActiveDevices
C_PlayerLocalData!0x0048 m_forceStance
C_PlayerLocalData!0x004c m_duckToggleOn
C_PlayerLocalData!0x004d m_bDrawViewmodel
C_PlayerLocalData!0x004e m_bAllowAutoMovement
C_PlayerLocalData!0x0180 m_airMoveBlockPlanes
C_PlayerLocalData!0x0198 m_airMoveBlockPlaneTime
C_PlayerLocalData!0x019c m_airMoveBlockPlaneCount
C_PlayerLocalData!0x01a0 m_queuedMeleePressTime
C_PlayerLocalData!0x01a4 m_queuedGrappleMeleeTime
C_PlayerLocalData!0x01a9 m_disableMeleeUntilRelease
C_PlayerLocalData!0x01ac m_meleePressTime
C_PlayerLocalData!0x01b0 m_meleeDisabledCounter
C_PlayerLocalData!0x01b4 m_meleeInputIndex
C_PlayerLocalData!0x01bc m_oneHandedWeaponUsage
C_PlayerLocalData!0x01bd m_prevOneHandedWeaponUsage
C_PlayerLocalData!0x01f0 m_titanEmbarkEnabled
C_PlayerLocalData!0x01f1 m_titanDisembarkEnabled
C_PlayerLocalData!0x01f8 m_playerAnimStationaryGoalFeetYaw
C_PlayerLocalData!0x01fc m_playerAnimJumping
C_PlayerLocalData!0x0200 m_playerAnimJumpStartTime
C_PlayerLocalData!0x0204 m_playerAnimFirstJumpFrame
C_PlayerLocalData!0x0205 m_playerAnimDodging
C_PlayerLocalData!0x0208 m_playerAnimJumpActivity
C_PlayerLocalData!0x020c m_playerAnimLanding
C_PlayerLocalData!0x020d m_playerAnimShouldLand
C_PlayerLocalData!0x0210 m_playerAnimLandStartTime
C_PlayerLocalData!0x0214 m_playerAnimInAirWalk
C_PlayerLocalData!0x0218 m_playerAnimPrevFrameSequenceMotionYaw
C_PlayerLocalData!0x021c m_playerAnimMeleeParity
C_PlayerLocalData!0x0220 m_playerAnimMeleeStartTime
C_PlayerLocalData!0x0224 m_playerLocalGravityToWorldTransform
C_PlayerLocalData!0x0254 m_playerLocalGravityBlendStartRotation
C_PlayerLocalData!0x0264 m_playerLocalGravityBlendEndRotation
C_PlayerLocalData!0x0274 m_playerLocalGravityBlendEndDirection
C_PlayerLocalData!0x0280 m_playerLocalGravityBlendStartTime
C_PlayerLocalData!0x0284 m_playerLocalGravityBlendEndTime
C_PlayerLocalData!0x0288 m_playerLocalGravityBlendStrength
C_PlayerLocalData!0x028c m_playerLocalGravityStrength
C_PlayerLocalData!0x0290 m_playerLocalGravityType
C_PlayerLocalData!0x0294 m_playerLocalGravityPoint
C_PlayerLocalData!0x02a0 m_playerLocalGravityLineStart
C_PlayerLocalData!0x02ac m_playerLocalGravityLineEnd
C_PlayerLocalData!0x02b8 m_playerLocalGravityEntity
C_PlayerLocalData!0x02bc m_playerLocalGravityLineStartEntity
C_PlayerLocalData!0x02c0 m_playerLocalGravityLineEndEntity
C_PlayerLocalData!0x02c4 m_playerFloatLookStartTime
C_PlayerLocalData!0x02c8 m_playerFloatLookEndTime
C_PlayerLocalData!0x02cc m_wallrunLatestFloorHeight
C_PlayerLocalData!0x02d0 m_groundNormal
C_PlayerLocalData!0x02dc m_continuousUseBlocked
C_PlayerLocalData!0x02e0 m_useEnt
```
</details>
<details>
<summary><code>class C_PlayerResource extends C_BaseEntity</code></summary>

```
{
	m_szName: String,
	m_boolStats: Int,
	m_killStats: Int,
	m_scoreStats: Int,
	m_iPing: Int,
	m_bConnected: Bool,
}
```

### Offsets

```
C_PlayerResource!0x0a00 m_szName
C_PlayerResource!0x1410 m_boolStats
C_PlayerResource!0x1614 m_killStats
C_PlayerResource!0x222c m_scoreStats
C_PlayerResource!0x2c40 m_iPing
C_PlayerResource!0x2e44 m_bConnected
```
</details>
<details>
<summary><code>class C_PlayerVehicle extends C_BaseAnimating</code></summary>

```
{
	m_localOrigin: Vector,
	m_vehicleDriver: EHANDLE,
	m_vehicleActivated: Bool,
	m_blockDuckInput: Bool,
	m_vehicleLaunchTime: Float,
	m_vehicleVelocity: Vector,
}
```

### Offsets

```
C_PlayerVehicle!0x0158 m_localOrigin
C_PlayerVehicle!0x1304 m_vehicleDriver
C_PlayerVehicle!0x1320 m_vehicleActivated
C_PlayerVehicle!0x1321 m_blockDuckInput
C_PlayerVehicle!0x1330 m_vehicleLaunchTime
C_PlayerVehicle!0x1338 m_vehicleVelocity
```
</details>
<details>
<summary><code>class C_Projectile extends C_BaseEntity</code></summary>

```
{
	m_weaponDataIsSet: Bool,
	m_forceAdjustToGunBarrelDisabled: Bool,
	m_weaponClassIndex: Int,
	m_destructionDistance: Float,
	m_passThroughDepthTotal: Int,
	m_modBitfield: Int,
	m_overrideMods: Int,
	m_projectileTrailIndex: Int,
	m_impactEffectTable: Int,
	m_reducedEffects: Bool,
	m_projectileCreationTimeServer: Float,
	m_weaponSource: EHANDLE,
	m_wpnData: Outer,
	m_hWeaponFileInfo: Short,
	m_weaponChargeLevel: Int,
	m_modVars: Void,
	m_modVarsAreValid: Bool,
	m_launchOrigin: Vector,
	m_scriptCB: Void,
	m_hasPlayedTrailEffect: Bool,
	m_projectileLifeTimeEndTick: Tick,
	m_projectileCreationTime: Float,
	m_isVortexRefired: Bool,
	m_damageAliveOnly: Bool,
	m_usesPositionFunction: Bool,
	m_lastCollisionNormal: Vector,
	m_bounceIndex: Int,
	m_randomInt: Int,
	m_thrownByAI: Bool,
	m_perPolyRadius: Float,
	m_posBeforePhysicsSimulate: Vector,
	m_hasIgnited: Bool,
	m_inLagCompensation: Bool,
	m_passEntities: EHANDLE,
	m_projectileSpeed: Float,
	m_wantStartTrailEffect: Bool,
	m_hasCalledPostDataUpdate: Bool,
}
```

### Offsets

```
C_Projectile!0x1300 m_weaponDataIsSet
C_Projectile!0x1301 m_forceAdjustToGunBarrelDisabled
C_Projectile!0x1304 m_weaponClassIndex
C_Projectile!0x1308 m_destructionDistance
C_Projectile!0x130c m_passThroughDepthTotal
C_Projectile!0x1310 m_modBitfield
C_Projectile!0x1314 m_overrideMods
C_Projectile!0x1318 m_projectileTrailIndex
C_Projectile!0x131c m_impactEffectTable
C_Projectile!0x1320 m_reducedEffects
C_Projectile!0x1324 m_projectileCreationTimeServer
C_Projectile!0x1328 m_weaponSource
C_Projectile!0x1330 m_wpnData
C_Projectile!0x1338 m_hWeaponFileInfo
C_Projectile!0x133c m_weaponChargeLevel
C_Projectile!0x1340 m_modVars
C_Projectile!0x2498 m_modVarsAreValid
C_Projectile!0x249c m_launchOrigin
C_Projectile!0x24a8 m_scriptCB
C_Projectile!0x24d0 m_hasPlayedTrailEffect
C_Projectile!0x24d4 m_projectileLifeTimeEndTick
C_Projectile!0x24d8 m_projectileCreationTime
C_Projectile!0x24dc m_isVortexRefired
C_Projectile!0x24dd m_damageAliveOnly
C_Projectile!0x24de m_usesPositionFunction
C_Projectile!0x24e0 m_lastCollisionNormal
C_Projectile!0x24ec m_bounceIndex
C_Projectile!0x24f0 m_randomInt
C_Projectile!0x24f4 m_thrownByAI
C_Projectile!0x24f8 m_perPolyRadius
C_Projectile!0x2500 m_posBeforePhysicsSimulate
C_Projectile!0x250c m_hasIgnited
C_Projectile!0x250d m_inLagCompensation
C_Projectile!0x2510 m_passEntities
C_Projectile!0x2578 m_projectileSpeed
C_Projectile!0x2598 m_wantStartTrailEffect
C_Projectile!0x259a m_hasCalledPostDataUpdate
```
</details>
<details>
<summary><code>class C_PropDoor</code></summary>

```
{
	m_localOrigin: Vector,
	m_localAngles: Vector,
	m_nNextThinkTick: Int,
	m_angle: Float,
	m_startAngle: Float,
	m_startAngleVel: Float,
	m_startMoveTime: Time,
	m_nextHitSoundTime: Float,
	m_lastThinkTime: Float,
	m_interactingPlayer: EHANDLE,
	m_interactingPlayerWantsOpen: Bool,
	m_useDebounceEndTime: Time,
	m_prevAngle: Float,
}
```

### Offsets

```
C_PropDoor!0x0158 m_localOrigin
C_PropDoor!0x0164 m_localAngles
C_PropDoor!0x050c m_nNextThinkTick
C_PropDoor!0x1374 m_angle
C_PropDoor!0x1378 m_startAngle
C_PropDoor!0x137c m_startAngleVel
C_PropDoor!0x1380 m_startMoveTime
C_PropDoor!0x138c m_nextHitSoundTime
C_PropDoor!0x1390 m_lastThinkTime
C_PropDoor!0x13d8 m_interactingPlayer
C_PropDoor!0x13dc m_interactingPlayerWantsOpen
C_PropDoor!0x13e0 m_useDebounceEndTime
C_PropDoor!0x13e8 m_prevAngle
```
</details>
<details>
<summary><code>class C_SequenceTransitioner</code></summary>

```
{
	m_sequenceTransitionerLayers: C_SequenceTransitionerLayer,
	m_sequenceTransitionerLayerCount: Int,
}
```

### Offsets

```
C_SequenceTransitioner!0x0050 m_sequenceTransitionerLayers
C_SequenceTransitioner!0x01a0 m_sequenceTransitionerLayerCount
```
</details>
<details>
<summary><code>class C_SequenceTransitionerLayer</code></summary>

```
{
	m_sequenceTransitionerLayerActive: Bool,
	m_sequenceTransitionerLayerStartCycle: Float,
	m_sequenceTransitionerLayerSequence: Int,
	m_weight: Float,
	m_sequenceTransitionerLayerPlaybackRate: Float,
	m_sequenceTransitionerLayerStartTime: Time,
	m_sequenceTransitionerLayerFadeOutDuration: Float,
}
```

### Offsets

```
C_SequenceTransitionerLayer!0x0018 m_sequenceTransitionerLayerActive
C_SequenceTransitionerLayer!0x001c m_sequenceTransitionerLayerStartCycle
C_SequenceTransitionerLayer!0x0020 m_sequenceTransitionerLayerSequence
C_SequenceTransitionerLayer!0x0024 m_weight
C_SequenceTransitionerLayer!0x0028 m_sequenceTransitionerLayerPlaybackRate
C_SequenceTransitionerLayer!0x002c m_sequenceTransitionerLayerStartTime
C_SequenceTransitionerLayer!0x0030 m_sequenceTransitionerLayerFadeOutDuration
```
</details>
<details>
<summary><code>class C_Team extends C_BaseEntity</code></summary>

```
{
	m_score: Int,
	m_score2: Int,
	m_kills: Int,
	m_deaths: Int,
	m_iRoundsWon: Int,
	m_iTeamTeamNum: Int,
	m_szTeamname: Char,
}
```

### Offsets

```
C_Team!0x0a00 m_score
C_Team!0x0a04 m_score2
C_Team!0x0a08 m_kills
C_Team!0x0a0c m_deaths
C_Team!0x0a10 m_iRoundsWon
C_Team!0x0a14 m_iTeamTeamNum
C_Team!0x0a38 m_szTeamname
```
</details>
<details>
<summary><code>class C_TriggerCylinderHeavy</code></summary>

```
{
	m_teslaTrapObstructedEndTime: Time,
}
```

### Offsets

```
C_TriggerCylinderHeavy!0x0ab8 m_teslaTrapObstructedEndTime
```
</details>
<details>
<summary><code>class C_VortexSphere extends C_BaseEntity</code></summary>

```
{
	m_enabled: Bool,
	m_radius: Float,
	m_height: Float,
	m_bulletFov: Float,
	m_bulletAbsorbedCount: Int,
	m_projectileAbsorbedCount: Int,
	m_ownerWeapon: EHANDLE,
	m_vortexEffect: EHANDLE,
	m_vortexLocalAngles: Vector,
	m_gunAttachment: String,
	m_listPrev: Outer,
	m_listNext: Outer,
}
```

### Offsets

```
C_VortexSphere!0x0a00 m_enabled
C_VortexSphere!0x0a04 m_radius
C_VortexSphere!0x0a08 m_height
C_VortexSphere!0x0a0c m_bulletFov
C_VortexSphere!0x0a10 m_bulletAbsorbedCount
C_VortexSphere!0x0a14 m_projectileAbsorbedCount
C_VortexSphere!0x0a18 m_ownerWeapon
C_VortexSphere!0x0a1c m_vortexEffect
C_VortexSphere!0x0a20 m_vortexLocalAngles
C_VortexSphere!0x0a30 m_gunAttachment
C_VortexSphere!0x0a38 m_listPrev
C_VortexSphere!0x0a40 m_listNext
```
</details>
<details>
<summary><code>class C_WallrunCurve extends C_GameplayHint</code></summary>

```
{
	width: Int,
	height: Int,
}
```

### Offsets

```
C_WallrunCurve!0x0a40 width
C_WallrunCurve!0x0a44 height
```
</details>
<details>
<summary><code>class C_WindowHint extends C_GameplayHint</code></summary>

```
{
	normal: Vector,
	right: Vector,
	halfSize: Float,
	halfSize[0]: Float,
	halfSize[1]: Float,
}
```

### Offsets

```
C_WindowHint!0x0a40 normal
C_WindowHint!0x0a4c right
C_WindowHint!0x0a58 halfSize
C_WindowHint!0x0a58 halfSize[0]
C_WindowHint!0x0a5c halfSize[1]
```
</details>
<details>
<summary><code>class C_Zipline extends C_BaseEntity</code></summary>

```
{
	m_ziplinePhysics: C_ZiplinePhysics,
	m_detachEndOnUse: Bool,
	m_currentFrameZipline.numZiplinePoints: Int,
	m_currentFrameZipline.ziplinePositions: Vector,
	m_currentFrameZipline.ziplinePreviousPositions: Vector,
	m_currentFrameZipline.ziplineDistances: Float,
}
```

### Offsets

```
C_Zipline!0x0a00 m_ziplinePhysics
C_Zipline!0x0d54 m_detachEndOnUse
C_Zipline!0x0e40 m_currentFrameZipline.numZiplinePoints
C_Zipline!0x0e44 m_currentFrameZipline.ziplinePositions
C_Zipline!0x0f04 m_currentFrameZipline.ziplinePreviousPositions
C_Zipline!0x0fc4 m_currentFrameZipline.ziplineDistances
```
</details>
<details>
<summary><code>class C_ZiplinePhysics</code></summary>

```
{
	m_ziplineType: Int,
	m_ziplineStart: Vector,
	m_ziplineEnd: Vector,
	m_nodes: C_ZiplinePhysicsNode,
	m_numNodes: Int,
	m_springDistance: Int,
	m_remainingUnsimulatedTime: Float,
	m_attachedEntities: C_ZiplinePhysicsAttachedEntity,
	m_numAttachedEntities: Int,
	m_ziplineOwner: EHANDLE,
}
```

### Offsets

```
C_ZiplinePhysics!0x000c m_ziplineType
C_ZiplinePhysics!0x0010 m_ziplineStart
C_ZiplinePhysics!0x001c m_ziplineEnd
C_ZiplinePhysics!0x0028 m_nodes
C_ZiplinePhysics!0x0228 m_numNodes
C_ZiplinePhysics!0x022c m_springDistance
C_ZiplinePhysics!0x0234 m_remainingUnsimulatedTime
C_ZiplinePhysics!0x0240 m_attachedEntities
C_ZiplinePhysics!0x0340 m_numAttachedEntities
C_ZiplinePhysics!0x0344 m_ziplineOwner
```
</details>
<details>
<summary><code>class C_ZiplinePhysicsAttachedEntity</code></summary>

```
{
	entity: EHANDLE,
	attachAcceleration: Vector,
	attachTime: Float,
}
```

### Offsets

```
C_ZiplinePhysicsAttachedEntity!0x0008 entity
C_ZiplinePhysicsAttachedEntity!0x000c attachAcceleration
C_ZiplinePhysicsAttachedEntity!0x0018 attachTime
```
</details>
<details>
<summary><code>class C_ZiplinePhysicsNode</code></summary>

```
{
	position: Vector,
	prevPosition: Vector,
}
```

### Offsets

```
C_ZiplinePhysicsNode!0x0008 position
C_ZiplinePhysicsNode!0x0014 prevPosition
```
</details>
<details>
<summary><code>class GrappleData</code></summary>

```
{
	m_grappleVel: Vector,
	m_grapplePoints: Vector,
	m_grapplePointCount: Int,
	m_grappleAttached: Bool,
	m_grapplePulling: Bool,
	m_grappleSwinging: Bool,
	m_grappleRetracting: Bool,
	m_grappleForcedRetracting: Bool,
	m_grappleGracePeriodFinished: Bool,
	m_grappleUsedPower: Float,
	m_grappleActivateTime: Time,
	m_grapplePullTime: Time,
	m_grappleAttachTime: Time,
	m_grappleDetachTime: Time,
	m_grappleMeleeTarget: EHANDLE,
	m_grappleAutoAimTarget: EHANDLE,
	m_grappleSwingDetachLowSpeed: Float,
	m_grappleSwingHoldTime: Time,
}
```

### Offsets

```
GrappleData!0x0008 m_grappleVel
GrappleData!0x0014 m_grapplePoints
GrappleData!0x0044 m_grapplePointCount
GrappleData!0x0048 m_grappleAttached
GrappleData!0x0049 m_grapplePulling
GrappleData!0x004a m_grappleSwinging
GrappleData!0x004b m_grappleRetracting
GrappleData!0x004c m_grappleForcedRetracting
GrappleData!0x004d m_grappleGracePeriodFinished
GrappleData!0x0050 m_grappleUsedPower
GrappleData!0x0054 m_grappleActivateTime
GrappleData!0x0058 m_grapplePullTime
GrappleData!0x005c m_grappleAttachTime
GrappleData!0x0060 m_grappleDetachTime
GrappleData!0x0064 m_grappleMeleeTarget
GrappleData!0x0068 m_grappleAutoAimTarget
GrappleData!0x0074 m_grappleSwingDetachLowSpeed
GrappleData!0x0078 m_grappleSwingHoldTime
```
</details>
<details>
<summary><code>class MissilePathExpandContractSettings_Client</code></summary>

```
{
	launchOutVec: Vector,
	launchInVec: Vector,
	launchOutTime: Time,
	launchInLerpTime: Time,
	launchInTime: Time,
	launchStraightLerpTime: Time,
	endPos: Vector,
	applyRandSpread: Bool,
}
```

### Offsets

```
MissilePathExpandContractSettings_Client!0x0000 launchOutVec
MissilePathExpandContractSettings_Client!0x000c launchInVec
MissilePathExpandContractSettings_Client!0x0018 launchOutTime
MissilePathExpandContractSettings_Client!0x001c launchInLerpTime
MissilePathExpandContractSettings_Client!0x0020 launchInTime
MissilePathExpandContractSettings_Client!0x0024 launchStraightLerpTime
MissilePathExpandContractSettings_Client!0x0028 endPos
MissilePathExpandContractSettings_Client!0x0034 applyRandSpread
```
</details>
<details>
<summary><code>class PlayerMelee_PlayerData</code></summary>

```
{
	meleeAttackParity: Int,
	attackActive: Bool,
	attackRecoveryShouldBeQuick: Bool,
	isSprintAttack: Bool,
	attackStartTime: Time,
	attackHitEntity: EHANDLE,
	attackHitEntityTime: Time,
	attackLastHitNonWorldEntity: Time,
	scriptedState: Int,
	pendingMeleePress: Bool,
	lungeBoost: Vector,
}
```

### Offsets

```
PlayerMelee_PlayerData!0x0008 meleeAttackParity
PlayerMelee_PlayerData!0x000c attackActive
PlayerMelee_PlayerData!0x000d attackRecoveryShouldBeQuick
PlayerMelee_PlayerData!0x000e isSprintAttack
PlayerMelee_PlayerData!0x0010 attackStartTime
PlayerMelee_PlayerData!0x0014 attackHitEntity
PlayerMelee_PlayerData!0x0018 attackHitEntityTime
PlayerMelee_PlayerData!0x001c attackLastHitNonWorldEntity
PlayerMelee_PlayerData!0x0020 scriptedState
PlayerMelee_PlayerData!0x0024 pendingMeleePress
PlayerMelee_PlayerData!0x0028 lungeBoost
```
</details>
<details>
<summary><code>class PlayerZiplineData_Client</code></summary>

```
{
	m_ziplineReenableWeapons: Bool,
	m_mountingZiplineDuration: Float,
	m_mountingZiplineAlpha: Float,
	m_ziplineStartTime: Time,
	m_ziplineEndTime: Time,
	m_mountingZiplineSourcePosition: Vector,
	m_mountingZiplineSourceVelocity: Vector,
	m_mountingZiplineTargetPosition: Vector,
	m_ziplineUsePosition: Vector,
	m_slidingZiplineAlpha: Float,
	m_lastMoveDir2D: Vector,
	m_ziplineReverse: Bool,
}
```

### Offsets

```
PlayerZiplineData_Client!0x0008 m_ziplineReenableWeapons
PlayerZiplineData_Client!0x000c m_mountingZiplineDuration
PlayerZiplineData_Client!0x0010 m_mountingZiplineAlpha
PlayerZiplineData_Client!0x0014 m_ziplineStartTime
PlayerZiplineData_Client!0x0018 m_ziplineEndTime
PlayerZiplineData_Client!0x001c m_mountingZiplineSourcePosition
PlayerZiplineData_Client!0x0028 m_mountingZiplineSourceVelocity
PlayerZiplineData_Client!0x0034 m_mountingZiplineTargetPosition
PlayerZiplineData_Client!0x004c m_ziplineUsePosition
PlayerZiplineData_Client!0x0058 m_slidingZiplineAlpha
PlayerZiplineData_Client!0x005c m_lastMoveDir2D
PlayerZiplineData_Client!0x0068 m_ziplineReverse
```
</details>
<details>
<summary><code>class Player_ViewOffsetEntityData</code></summary>

```
{
	viewOffsetEntityHandle: EHANDLE,
	lerpInDuration: Float,
	lerpOutDuration: Float,
	stabilizePlayerEyeAngles: Bool,
}
```

### Offsets

```
Player_ViewOffsetEntityData!0x0008 viewOffsetEntityHandle
Player_ViewOffsetEntityData!0x000c lerpInDuration
Player_ViewOffsetEntityData!0x0010 lerpOutDuration
Player_ViewOffsetEntityData!0x0014 stabilizePlayerEyeAngles
```
</details>
<details>
<summary><code>class PredictedAnimEventData</code></summary>

```
{
	m_predictedAnimEventTimes: Time,
	m_predictedAnimEventIndices: Int,
	m_predictedAnimEventCount: Int,
	m_predictedAnimEventTarget: EHANDLE,
	m_predictedAnimEventSequence: Int,
	m_predictedAnimEventModel: Int,
	m_predictedAnimEventsReadyToFireTime: Time,
}
```

### Offsets

```
PredictedAnimEventData!0x0008 m_predictedAnimEventTimes
PredictedAnimEventData!0x0028 m_predictedAnimEventIndices
PredictedAnimEventData!0x0048 m_predictedAnimEventCount
PredictedAnimEventData!0x004c m_predictedAnimEventTarget
PredictedAnimEventData!0x0050 m_predictedAnimEventSequence
PredictedAnimEventData!0x0054 m_predictedAnimEventModel
PredictedAnimEventData!0x0058 m_predictedAnimEventsReadyToFireTime
```
</details>
<details>
<summary><code>class StatusEffectEndlessData</code></summary>

```
{
	seComboVars: Int,
}
```

### Offsets

```
StatusEffectEndlessData!0x0008 seComboVars
```
</details>
<details>
<summary><code>class StatusEffectTimedData</code></summary>

```
{
	seComboVars: Int,
	seTimeEnd: Float,
	seEaseOut: Float,
}
```

### Offsets

```
StatusEffectTimedData!0x0008 seComboVars
StatusEffectTimedData!0x000c seTimeEnd
StatusEffectTimedData!0x0010 seEaseOut
```
</details>
<details>
<summary><code>class WeaponInventory_Client</code></summary>

```
{
	weapons: EHANDLE,
	activeWeapons: EHANDLE,
}
```

### Offsets

```
WeaponInventory_Client!0x0008 weapons
WeaponInventory_Client!0x0044 activeWeapons
```
</details>
<details>
<summary><code>class WeaponPlayerData</code></summary>

```
{
	m_moveSpread: Float,
	m_spreadStartTime: Time,
	m_spreadStartFracHip: Float,
	m_spreadStartFracADS: Float,
	m_kickSpreadHipfire: Float,
	m_kickSpreadADS: Float,
	m_kickTime: Time,
	m_kickScaleBasePitch: Float,
	m_kickScaleBaseYaw: Float,
	m_kickPatternScaleBase: Float,
	m_kickSpringHeatBaseTime: Time,
	m_kickSpringHeatBaseValue: Float,
	m_semiAutoTriggerHoldTime: Time,
	m_semiAutoTriggerDown: Bool,
	m_pendingTriggerPull: Bool,
	m_semiAutoNeedsRechamber: Bool,
	m_pendingReloadAttempt: Bool,
	m_offhandHybridNormalMode: Bool,
	m_pendingoffhandHybridToss: Bool,
	m_fastHolster: Bool,
	m_didFirstDeploy: Bool,
	m_shouldCatch: Bool,
	m_clipModelIsHidden: Bool,
	m_segmentedReloadEndSeqRequired: Bool,
	m_reloadStartedEmpty: Bool,
	m_segmentedAnimStartedOneHanded: Bool,
	m_segmentedReloadCanRestartLoop: Bool,
	m_segmentedReloadLoopFireLocked: Bool,
	m_realtimeModCmds: Char,
	m_realtimeModCmdHead: Char,
	m_realtimeModCmdCount: Char,
	m_customActivityAttachedModelIndex: Int,
	m_customActivityAttachedModelAttachmentIndex: Int,
	m_fireRateLerp_startTime: Time,
	m_fireRateLerp_startFraction: Float,
	m_fireRateLerp_stopTime: Time,
	m_fireRateLerp_stopFraction: Float,
	m_chargeAnimIndex: Int,
	m_chargeAnimIndexOld: Int,
	m_reloadMilestone: Int,
	m_rechamberMilestone: Int,
	m_cooldownMilestone: Int,
	m_fullReloadStartTime: Time,
	m_scriptTime0: Time,
	m_scriptFlags0: Int,
	m_scriptInt0: Int,
	m_curZoomFOV: Float,
	m_targetZoomFOV: Float,
	m_zoomFOVLerpTime: Float,
	m_zoomFOVLerpEndTime: Time,
	m_latestDryfireTime: Time,
	m_requestedAttackEndTime: Time,
	m_currentAltFireAnimIndex: Int,
	m_legendaryModelIndex: Int,
	m_charmModelIndex: Int,
	m_charmAttachment: Int,
	m_charmScriptIndex: Int,
}
```

### Offsets

```
WeaponPlayerData!0x0008 m_moveSpread
WeaponPlayerData!0x000c m_spreadStartTime
WeaponPlayerData!0x0010 m_spreadStartFracHip
WeaponPlayerData!0x0014 m_spreadStartFracADS
WeaponPlayerData!0x0018 m_kickSpreadHipfire
WeaponPlayerData!0x001c m_kickSpreadADS
WeaponPlayerData!0x0020 m_kickTime
WeaponPlayerData!0x0024 m_kickScaleBasePitch
WeaponPlayerData!0x0028 m_kickScaleBaseYaw
WeaponPlayerData!0x002c m_kickPatternScaleBase
WeaponPlayerData!0x0030 m_kickSpringHeatBaseTime
WeaponPlayerData!0x0034 m_kickSpringHeatBaseValue
WeaponPlayerData!0x0038 m_semiAutoTriggerHoldTime
WeaponPlayerData!0x003c m_semiAutoTriggerDown
WeaponPlayerData!0x003d m_pendingTriggerPull
WeaponPlayerData!0x003e m_semiAutoNeedsRechamber
WeaponPlayerData!0x003f m_pendingReloadAttempt
WeaponPlayerData!0x0040 m_offhandHybridNormalMode
WeaponPlayerData!0x0041 m_pendingoffhandHybridToss
WeaponPlayerData!0x0042 m_fastHolster
WeaponPlayerData!0x0043 m_didFirstDeploy
WeaponPlayerData!0x0044 m_shouldCatch
WeaponPlayerData!0x0045 m_clipModelIsHidden
WeaponPlayerData!0x0046 m_segmentedReloadEndSeqRequired
WeaponPlayerData!0x0047 m_reloadStartedEmpty
WeaponPlayerData!0x0048 m_segmentedAnimStartedOneHanded
WeaponPlayerData!0x0049 m_segmentedReloadCanRestartLoop
WeaponPlayerData!0x004a m_segmentedReloadLoopFireLocked
WeaponPlayerData!0x004b m_realtimeModCmds
WeaponPlayerData!0x0053 m_realtimeModCmdHead
WeaponPlayerData!0x0054 m_realtimeModCmdCount
WeaponPlayerData!0x0058 m_customActivityAttachedModelIndex
WeaponPlayerData!0x005c m_customActivityAttachedModelAttachmentIndex
WeaponPlayerData!0x0060 m_fireRateLerp_startTime
WeaponPlayerData!0x0064 m_fireRateLerp_startFraction
WeaponPlayerData!0x0068 m_fireRateLerp_stopTime
WeaponPlayerData!0x006c m_fireRateLerp_stopFraction
WeaponPlayerData!0x0070 m_chargeAnimIndex
WeaponPlayerData!0x0074 m_chargeAnimIndexOld
WeaponPlayerData!0x0094 m_reloadMilestone
WeaponPlayerData!0x0098 m_rechamberMilestone
WeaponPlayerData!0x009c m_cooldownMilestone
WeaponPlayerData!0x00a0 m_fullReloadStartTime
WeaponPlayerData!0x00a4 m_scriptTime0
WeaponPlayerData!0x00a8 m_scriptFlags0
WeaponPlayerData!0x00ac m_scriptInt0
WeaponPlayerData!0x00b0 m_curZoomFOV
WeaponPlayerData!0x00b4 m_targetZoomFOV
WeaponPlayerData!0x00b8 m_zoomFOVLerpTime
WeaponPlayerData!0x00bc m_zoomFOVLerpEndTime
WeaponPlayerData!0x00c0 m_latestDryfireTime
WeaponPlayerData!0x00c4 m_requestedAttackEndTime
WeaponPlayerData!0x00c8 m_currentAltFireAnimIndex
WeaponPlayerData!0x00cc m_legendaryModelIndex
WeaponPlayerData!0x00d0 m_charmModelIndex
WeaponPlayerData!0x00d4 m_charmAttachment
WeaponPlayerData!0x00d8 m_charmScriptIndex
```
</details>
<details>
<summary><code>class fogplayerparamsstate_t</code></summary>

```
{
	enable: Bool,
	botAlt: Float,
	topAlt: Float,
	halfDistBot: Float,
	halfDistTop: Float,
	distOffset: Float,
	densityScale: Float,
	halfAngleDeg: Float,
	distColorStr: Float,
	dirColorStr: Float,
	HDRColorScale: Float,
	minFadeTime: Float,
	forceOntoSky: Bool,
	distColor: Color32,
	dirColor: Color32,
	vlParams.color: Vector,
	vlParams.distFalloff: Float,
	vlParams.intensity: Float,
	vlParams.scatter: Float,
	vlParams.inShadowScatter: Float,
	direction: Vector,
	id: Int,
}
```

### Offsets

```
fogplayerparamsstate_t!0x0000 enable
fogplayerparamsstate_t!0x0004 botAlt
fogplayerparamsstate_t!0x0008 topAlt
fogplayerparamsstate_t!0x000c halfDistBot
fogplayerparamsstate_t!0x0010 halfDistTop
fogplayerparamsstate_t!0x0014 distOffset
fogplayerparamsstate_t!0x0018 densityScale
fogplayerparamsstate_t!0x001c halfAngleDeg
fogplayerparamsstate_t!0x0020 distColorStr
fogplayerparamsstate_t!0x0024 dirColorStr
fogplayerparamsstate_t!0x0028 HDRColorScale
fogplayerparamsstate_t!0x002c minFadeTime
fogplayerparamsstate_t!0x0030 forceOntoSky
fogplayerparamsstate_t!0x0031 distColor
fogplayerparamsstate_t!0x0035 dirColor
fogplayerparamsstate_t!0x003c vlParams.color
fogplayerparamsstate_t!0x0048 vlParams.distFalloff
fogplayerparamsstate_t!0x004c vlParams.intensity
fogplayerparamsstate_t!0x0050 vlParams.scatter
fogplayerparamsstate_t!0x0054 vlParams.inShadowScatter
fogplayerparamsstate_t!0x0058 direction
fogplayerparamsstate_t!0x0064 id
```
</details>

## ConVars

<details>
<summary><code></code></summary>

default: `"1.0"`  
flags: `0x0`  
</details>
<details>
<summary><code>Allow_auto_Party</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>BlendBonesMode</code></summary>



default: `"2"`  
flags: `0x2002`  
</details>
<details>
<summary><code>DoorSoundPrefixDouble</code></summary>

Sound prefix for door sounds for double doors

default: `"Door_Single_"`  
flags: `0x2`  
</details>
<details>
<summary><code>DoorSoundPrefixSingle</code></summary>

Sound prefix for door sounds for single doors

default: `"Door_Single_"`  
flags: `0x2`  
</details>
<details>
<summary><code>ScriptDisallowedToUsePersistenceOnSP</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>ScriptSaveAllowed</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>StreamMicDisabled</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>TalkIsStream</code></summary>



default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>VoiceNeedsReset</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>When set to 0, player always returns false when asked if it has a vehicle</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>ai_titan_grapple_max_len</code></summary>



default: `"3000"`  
flags: `0x2002`  
</details>
<details>
<summary><code>airslowmo_enabled</code></summary>

Enables air slowmo

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>airslowmo_enter_time</code></summary>

Duration it takes to reach full slowmo

default: `"0.25"`  
flags: `0x2002`  
</details>
<details>
<summary><code>airslowmo_ground_immediate_end</code></summary>

Controls whether air slowmo fades out after landing or immediately stops

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>airslowmo_leave_time</code></summary>

Duration it takes to leave full slowmo

default: `"1.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>airslowmo_scripted_speed</code></summary>



default: `"0.8"`  
flags: `0x2002`  
</details>
<details>
<summary><code>airslowmo_when_hovering</code></summary>

Replaces hovering with air slowmo

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>animEvent_debug</code></summary>

1 = sparse, 2 = verbose

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>animEvent_debugEnt</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>animEvent_debug_cl</code></summary>

1 = sparse, 2 = verbose

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>anim_estimateVelocity</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>anim_playerMovementAngleMargin</code></summary>



default: `"10"`  
flags: `0x2002`  
</details>
<details>
<summary><code>anim_player_ragdoll_fix</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>anim_print_transition_overflow</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>anim_runGestureAnimEventsToCompletionOnReset_client</code></summary>



default: `"0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>anim_showPoseParamErrors</code></summary>



default: `"0"`  
flags: `0x82`  
</details>
<details>
<summary><code>anim_showstate</code></summary>

Show the (client) animation state for the specified entity (-1 for none).

default: `"-1"`  
flags: `0x6002`  
</details>
<details>
<summary><code>anim_showstatelog</code></summary>

1 to output anim_showstate to Msg(). 2 to store in AnimState.log. 3 for both.

default: `"0"`  
flags: `0x6002`  
</details>
<details>
<summary><code>anim_transitionsequences</code></summary>

Enables blended transitions between sequences.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>anim_view_entity_third_person_camera_use_move_parent</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>announcement</code></summary>



default: `""`  
flags: `0x12`  
</details>
<details>
<summary><code>announcementImage</code></summary>



default: `""`  
flags: `0x12`  
</details>
<details>
<summary><code>announcementVersion</code></summary>



default: `"0"`  
flags: `0x12`  
</details>
<details>
<summary><code>assetdownloads_desiredState</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>assetdownloads_enabled</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>assetdownloads_hostname</code></summary>



default: `"r5-assets.stryder.respawn.com"`  
flags: `0x2`  
</details>
<details>
<summary><code>assetdownloads_useProdPreview</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>async_serialize</code></summary>

Force async reads to serialize for profiling

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>automantle_backoff_anim_maxfrac</code></summary>

Fraction of mantle after which pulling back simply aborts the mantle

default: `"0.7"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_cooldown</code></summary>

Minimum time between mantles

default: `".25"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_dangle_required_space</code></summary>

Required space under the ledge to dangle

default: `"60"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_debug</code></summary>

Debugs player auto-mantle behavior

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_enable</code></summary>

Enables player auto-mantle behavior

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_forwarddist</code></summary>

Distance forward to do the ground check from when auto-mantling

default: `"26.f"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_gun_enable_height</code></summary>

Eye height above ledge at which gun is reenabled

default: `"33"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_height_above</code></summary>

Mantle height above ledge below which the "above" animation is used and above which the "high" animation is used

default: `"30"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_height_below</code></summary>

Mantle height above ledge below which the "below" animation is used

default: `"-10"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_height_level</code></summary>

Mantle height above ledge below which the "level" animation is used

default: `"10"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_jumpoff_anim_maxfrac</code></summary>

Maximum fraction of mantle at which jump off animation is played

default: `"0.5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_jumpoff_duration</code></summary>

Duration of jump off animation when jumping off

default: `"0.1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_max_frac</code></summary>

Fractional amount (0-1) player can move forward without hitting jump.

default: `"0.5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_maxangle_push</code></summary>

Max angle the player can be pushing from the wall normal to auto-mantle

default: `"50"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_maxangle_view</code></summary>

Max angle the player can be facing from the wall to auto-mantle

default: `"50"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_min_frac</code></summary>

Fractional amount (0-1) player can move backward without hitting jump.

default: `"0.1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_mindist</code></summary>

Minimum forward distance when auto-mantling

default: `"18.f"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_rest_frac</code></summary>

Fractional amount (0-1) player will tend toward when no input is given.

default: `"0.4"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_rest_frac_below</code></summary>

Replaces rest_frac when using the "below" animation

default: `"0.3"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_searchdist</code></summary>

Forward distance within which to look for a ledge to auto-mantle

default: `"5.f"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_view_correction_speed</code></summary>

Speed at which view direction is clamped when mantling

default: `"180"`  
flags: `0x4000`  
</details>
<details>
<summary><code>automantle_view_high_yaw_max</code></summary>

Max view yaw when mantling with the "high" mantle animation

default: `"90"`  
flags: `0x4000`  
</details>
<details>
<summary><code>automantle_view_pitch_max</code></summary>

Max view pitch when mantling

default: `"35"`  
flags: `0x4000`  
</details>
<details>
<summary><code>automantle_view_pitch_min</code></summary>

Min view pitch when mantling

default: `"-80"`  
flags: `0x4000`  
</details>
<details>
<summary><code>automantle_view_yaw_max</code></summary>

Max view yaw when mantling

default: `"60"`  
flags: `0x4000`  
</details>
<details>
<summary><code>automantle_wallrun_maxangle_view</code></summary>

Max angle the player can be facing from the wall to auto-mantle while wall running

default: `"45"`  
flags: `0x2002`  
</details>
<details>
<summary><code>baseanimatingoverlay_playbackRateThreshold</code></summary>



default: `"0.05"`  
flags: `0x2`  
</details>
<details>
<summary><code>baselines_print</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>bhit_enable</code></summary>

Enables bhit commands from the client

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>bhit_reliable</code></summary>

Makes bhit commands reliable messages

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>bink_materials_enabled</code></summary>

Allows materials with 'Emissive Uses Video' checked to play video on the material 

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>bink_preload_videopanel_movies</code></summary>

Preload Bink movies used by VideoPanel.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>boost_jetwash_prediction_factor</code></summary>

Factor used to scale player's velocity when finding jetwash trace point.

default: `"20.0f"`  
flags: `0x2002`  
</details>
<details>
<summary><code>bot_lagOut</code></summary>

Cause bots to lag out

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>budget_animatingEntities</code></summary>



default: `"5000"`  
flags: `0x2`  
</details>
<details>
<summary><code>budget_animationOverlayEntities</code></summary>



default: `"260"`  
flags: `0x2`  
</details>
<details>
<summary><code>budget_combatCharEntities</code></summary>



default: `"200"`  
flags: `0x2`  
</details>
<details>
<summary><code>budget_weaponEntities</code></summary>



default: `"1200"`  
flags: `0x2`  
</details>
<details>
<summary><code>budget_ziplineEntities</code></summary>



default: `"250"`  
flags: `0x2`  
</details>
<details>
<summary><code>bug_reproNum</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>buildcubemaps_async</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>buildcubemaps_index</code></summary>



default: `"-1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>buildcubemaps_pvs_start_early</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>buildcubemaps_single_step</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>building_cubemaps</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>bulletPredictionDebug</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>bullet_trace_test_debug</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>bullet_trace_test_enable</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>c_dropship_ground_fx_dist_interval</code></summary>



default: `"256"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_dropship_ground_fx_time_interval</code></summary>



default: `"0.25"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_dropship_rope_debug</code></summary>

 Used to visualize the drop ship rope interaction.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_dropship_rope_events</code></summary>

Turn on client side drop ship rope interaction detection.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_dropship_rope_magnitude</code></summary>

Used to scale the interaction of a drop ship and a rope.

default: `"128"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_dropship_rope_range</code></summary>

Max distance away from a drop ship that a Rope is effected.

default: `"1024"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_maxdistance</code></summary>



default: `"400"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_maxpitch</code></summary>



default: `"90"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_maxyaw</code></summary>



default: `"135"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_mindistance</code></summary>



default: `"30"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_minpitch</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_minyaw</code></summary>



default: `"-135"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_orthoheight</code></summary>



default: `"100"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_orthowidth</code></summary>



default: `"100"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_thirdpersonshoulderaimdist_110</code></summary>



default: `"60.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_thirdpersonshoulderaimdist_70</code></summary>



default: `"100.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_thirdpersonshoulderaimdist_90</code></summary>



default: `"75.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_thirdpersonshoulderdist</code></summary>



default: `"0.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_thirdpersonshouldergetsviewpunch</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_thirdpersonshoulderheight</code></summary>



default: `"30.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_thirdpersonshoulderoffset</code></summary>



default: `"20.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_threadedAnimPostData</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cam_collision</code></summary>

When in thirdperson and cam_collision is set to 1, an attempt is made to keep the camera from passing though walls.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cam_idealdelta</code></summary>

Controls the speed when matching offset to ideal angles in thirdperson view

default: `"4.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cam_idealdist</code></summary>



default: `"150"`  
flags: `0x2`  
</details>
<details>
<summary><code>cam_ideallag</code></summary>

Amount of lag used when matching offset to ideal angles in thirdperson view

default: `"4.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cam_idealpitch</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cam_idealyaw</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cam_pitchLock_feetRelative</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cam_pitchlock_on</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cam_pitchlock_period</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cam_pitchlock_phase</code></summary>



default: `"0.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cam_pitchlock_pitchBase</code></summary>



default: `"0.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cam_pitchlock_pitchRange</code></summary>



default: `"0.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cam_pitchlock_pitchWiggleRoom</code></summary>



default: `"0.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cam_player_viewheight_scale</code></summary>



default: `"0.75"`  
flags: `0x2`  
</details>
<details>
<summary><code>cam_showangles</code></summary>

When in thirdperson, print viewangles/idealangles/cameraoffsets to the console.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cc_captiontrace</code></summary>

Show missing closecaptions (0 = no, 1 = devconsole, 2 = show in hud)

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cc_global_norepeat</code></summary>

How often a caption can repeat, unless overriden by norepeat. (or 0)

default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>cc_linger_time</code></summary>

Close caption linger time in seconds.

default: `"1.0"`  
flags: `0x80`  
</details>
<details>
<summary><code>cc_max_duration</code></summary>

The max duration in seconds for a closed caption if event doesn't stop playing.

default: `"30.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cc_minvisibleitems</code></summary>

Minimum number of caption items to show.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cc_predisplay_time</code></summary>

Close caption delay in seconds before showing caption.

default: `"0.25"`  
flags: `0x80`  
</details>
<details>
<summary><code>cc_rui</code></summary>

Use RUI to draw closecaption text.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cc_text_size</code></summary>

Changes the size of subtitles and closed captions text. 0 = normal, 1 = large, 2 = huge.

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>cc_timeshift_norepeat</code></summary>

How often a caption can repeat, unless overriden by norepeat. (timeshift only) (or 0)

default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>chasecam_distanceMax_override</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>chatroom_console_ptt</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>chatroom_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>chatroom_doRealNameLookups</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>chatroom_min_status_send_interval</code></summary>



default: `"16"`  
flags: `0x2`  
</details>
<details>
<summary><code>chatroom_nameLength</code></summary>



default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>chatroom_namePaddingX</code></summary>



default: `"12"`  
flags: `0x2`  
</details>
<details>
<summary><code>chatroom_nameWidth</code></summary>



default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>chatroom_onlyWhenActive</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>chatroom_useSlopSpace</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>chatroom_voiceMode</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>chatroom_voiceMode</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cheap_captions_fadetime</code></summary>



default: `"0.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>cheap_captions_test</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>chroma_enable</code></summary>



default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>cl_NotifyAllLevelAssetsLoaded_endframe</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_RunClientConnectScripts_Before_ProcessOnDataChangedEvents</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_SetupAllBones</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_ShowBoneSetupEnts</code></summary>

Show which entities are having their bones setup each frame.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_adjustTimeEntsPerJob</code></summary>



default: `"10"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_aggregate_particles</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_allowABSCalculationDuringSnapshotScriptCalls</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_allowABSDuringSnapshotScriptCalls</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_allowAnimsToInterpolateBackward</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_always_draw_3p_player</code></summary>

Always draw the 3p player model, even when in first-person view

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_always_ragdoll_radius</code></summary>

Always create client ragdoll if within this distance to viewer

default: `"500"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_anglespeedkey</code></summary>



default: `"0.67"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_anim_blend_transition_dist</code></summary>



default: `"2500"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_anim_detail_dist</code></summary>



default: `"1500"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_anim_face_dist</code></summary>



default: `"250"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_anim_sequence_transition_full_weight_optimization</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_anim_sounds_seek</code></summary>



default: `"1"`  
flags: `0xa`  
</details>
<details>
<summary><code>cl_approx_footstep_origin</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_approx_tracer_origin</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_async_bone_setup</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_base_entity_effect_lock</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_bones_incremental_blend</code></summary>

Don't reblend bones which we don't need to in SetupBones.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_bones_incremental_transform</code></summary>

Don't retransform bones which we don't need to in SetupBones.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_bones_oldhack</code></summary>

Redo all previously transformed bones in SetupBones--old 'hack'.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_bounds_show_errors</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_burninggibs</code></summary>

A burning player that gibs has burning gibs.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_clock_correction</code></summary>

Enable/disable clock correction on the client.

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_clock_correction_ahead_correct_interval</code></summary>

Minimum interval over which the clock will try to correct to ideal when it's ahead

default: `"20"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_clock_correction_behind_correct_interval</code></summary>

Interval over which the clock will try to correct to ideal when it's behind

default: `"200"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_clock_correction_force_server_tick</code></summary>

Force clock correction to match the server tick + this offset (-999 disables it).

default: `"999"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_cmdbackup</code></summary>

Number of redundant usercmds to send, to cover client->server packet loss

default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_cmdrate</code></summary>

Max number of command packets sent to server per second

default: `"60"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_configversion</code></summary>

Configuration layout version.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_configversion_dummy</code></summary>

Configuration layout version dummy.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_cull_weapon_fx</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_dataBlockFragmentPL</code></summary>



default: `"0.0"`  
flags: `0x2`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>cl_deathhints_enabled</code></summary>



default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>cl_debugClientEntities</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_debug_deferred_trace</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_debug_deferred_trace_overlay</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_debug_model_fx_sounds</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_decal_alwayswhite</code></summary>

Force FX decals to white (1), or white full alpha (2).

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_decal_backoff</code></summary>

Amount to back off FX decal trace by.

default: `"4"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_deferred_effects</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_deferred_trace_normal_priority</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_demoviewoverride</code></summary>

Override view during demo playback

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_disable_ragdolls</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_disable_splitscreen_cpu_level_cfgs_in_pip</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_disconnectOnTooManySnapshotFrames</code></summary>

Disconnect when the client gets too many snapshot messages from the server without the server getting any messages from the client.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_doNetworkAsserts</code></summary>

Turn off to disable some client asserts that fail rarely, presumably due to network bugs.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_doRecreateEnts</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_draw_player_model</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_drawhud</code></summary>

Enable the rendering of the hud

default: `"1"`  
flags: `0x40004000`  
</details>
<details>
<summary><code>cl_drawmonitors</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_ejectbrass</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_enable_remote_splitscreen</code></summary>

Allows viewing of nonlocal players in a split screen fashion

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_entCreateDeleteDebug</code></summary>

If true, print out when we create or delete an entity on the client

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_events_ignore_invalidate</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_failremoteconnections</code></summary>

Force connection attempts to time out

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_fasttempentcollision</code></summary>



default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_flip_vis_bits</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_flushentitypacket</code></summary>

For debugging. Force the engine to flush an entity packet.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_footstep_event_max_dist</code></summary>



default: `"2500"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_footstep_event_max_dist_titan</code></summary>



default: `"4000"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_forceAdjustTime</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_fovScale</code></summary>



default: `"1.27216005"`  
flags: `0x41000200`  
min value: `1`  
max value: `1.7`  
</details>
<details>
<summary><code>cl_gib_allow</code></summary>



default: `"1"`  
flags: `0x40000000`  
</details>
<details>
<summary><code>cl_gib_attack_dir_scale</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_gib_lifetime</code></summary>



default: `"3"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_idealpitchscale</code></summary>

0 to turn off. 0.8 is a good starting value

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_ignorepackets</code></summary>

Force client to ignore packets (for debugging).

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_interp_all</code></summary>

Disable interpolation list optimizations.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_interpolate</code></summary>

Interpolate entities on the client.

default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_interpolate</code></summary>

Interpolate entities on the client.

default: `"1.0f"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_interpolateSoAllAnimsLoop</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_interpolation_before_prediction</code></summary>

Interpolate entities before doing prediction

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_isUnderAge</code></summary>



default: `"0"`  
flags: `0x80000200`  
</details>
<details>
<summary><code>cl_is_softened_locale</code></summary>



default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>cl_jiggle_bone_debug</code></summary>

Display physics-based 'jiggle bone' debugging information

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_jiggle_bone_debug_pitch_constraints</code></summary>

Display physics-based 'jiggle bone' debugging information

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_jiggle_bone_debug_yaw_constraints</code></summary>

Display physics-based 'jiggle bone' debugging information

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_jiggle_bone_invert</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_jiggle_bone_sanity</code></summary>

Prevent jiggle bones from pointing directly away from their target in case of numerical instability.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_keepPersistentDataOnDisconnect</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_lagcompensation</code></summary>

Perform server side lag compensation of weapon firing events.

default: `"1"`  
flags: `0x200`  
</details>
<details>
<summary><code>cl_language</code></summary>

Language

default: `"english"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_leafsystemvis</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_lerpIfChildrenLerp</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_loadBspFromServerInfo</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_loadPostProcessShadersEarly</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_loadStaticPropsInJob</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_matchmaking_timeout</code></summary>

Total time allowed for the client to resend the 'connect' attempt when matchmaking

default: `"1"`  
flags: `0x80000`  
min value: `0.5`  
max value: `20000`  
</details>
<details>
<summary><code>cl_minimal_rtt_shadows</code></summary>



default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>cl_model_fx_gib_cull_front_dist</code></summary>



default: `"3000"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_model_fx_gib_cull_radius</code></summary>



default: `"1000"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_mouseenable</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_move_use_dt</code></summary>

Use the actual delta time for motion instead some super complicated system based on the server frame rate.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_noTimeoutLocalHost</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_overrideEventTimes</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_parallelParticlePreDrawWork</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_parallel_clientside_animations</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_particle_batch_mode</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_particle_fallback_base</code></summary>

Base for falling back to cheaper effects under load.

default: `"0"`  
flags: `0x40000000`  
</details>
<details>
<summary><code>cl_particle_fallback_multiplier</code></summary>

Multiplier for falling back to cheaper effects under load.

default: `"1"`  
flags: `0x40000000`  
</details>
<details>
<summary><code>cl_particle_limiter_display_killed</code></summary>

Display a red box around killed fx.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_particle_limiter_hide_killable</code></summary>

Hide fx than could be killed if over limit.

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>cl_particle_limiter_max_particle_count</code></summary>

Limit the total number of active particles. 0 to not limit.

default: `"10000"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_particle_limiter_max_system_count</code></summary>

Limit the total number of active particle systems. 0 to not limit.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_particle_limiter_min_kill_distance</code></summary>

Only kill fx that are further than this distance from the player.

default: `"4000"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_particle_limiter_overlay</code></summary>

Display particle limiter infos.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_particle_max_count</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_particle_sim_fallback_base_multiplier</code></summary>

How aggressive the switch to fallbacks will be depending on how far over the cl_particle_sim_fallback_threshold_ms the sim time is.  Higher numbers are more aggressive.

default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_particle_sim_fallback_threshold_ms</code></summary>

Amount of simulation time that can elapse before new systems start falling back to cheaper versions

default: `"6.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_particle_snoozetime</code></summary>

Particle snooze time in seconds (0 is off)

default: `"0.166667"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_particles_show_bbox</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_particles_show_controlpoints</code></summary>

1 to show parent effects, 2 shows all children effects too

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_pclass</code></summary>

Dump entity by prediction classname.

default: `""`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_pdump</code></summary>

Dump info about this entity to screen.

default: `"-1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_phys_maxticks</code></summary>

Sets the max number of physics ticks allowed for client-side physics (ragdolls)

default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_phys_show_active</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_phys_timescale</code></summary>

Sets the scale of time for client-side physics (ragdolls)

default: `"1.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_physics_invalidate_ents</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_physics_maxvelocity</code></summary>

Max velocity of a vphysics object on the client

default: `"4000.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_physicsshadowupdate_render</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_pitchspeed</code></summary>



default: `"225"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_playback_screenshots</code></summary>

Allows the client to playback screenshot and jpeg commands in demos.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_player_fullupdate_predicted_origin_fix</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_player_touch_triggers</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_postSnapshotTransitionBlockCount</code></summary>



default: `"20"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_preSnapshotTransitionBlockCount</code></summary>



default: `"10"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_pred_error_verbose</code></summary>

Show more field info when spewing prediction errors.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_pred_optimize</code></summary>

Optimize for not rerunning prediction if there was no difference between what we predicted and the incoming networked state

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_predict</code></summary>

Perform client side prediction.

default: `"1"`  
flags: `0x200`  
</details>
<details>
<summary><code>cl_predict_basetoggles</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_predict_cmdlimit</code></summary>

Artificially limits the number of remembered commands that can be used for prediction

default: `"750"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_predict_error_icon_duration</code></summary>

Duration for prediction error icon to stay visible

default: `"0.5"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_predict_error_icon_show</code></summary>

Whether to show the prediction error icon

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_predict_error_icon_threshold_angle</code></summary>

Angle error required to show prediction error icon

default: `"0.01"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_predict_error_icon_threshold_dist</code></summary>

Distance error required to show prediction error icon

default: `"1.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_predict_motioncontrol</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_predict_viewangles</code></summary>

Predict view angles even if cl_predict is 0.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_prediction_error_timestamps</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_predictionlist</code></summary>

Show which entities are predicting


default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_predictweapons</code></summary>

Perform client side prediction of weapon effects.

default: `"1"`  
flags: `0x200`  
</details>
<details>
<summary><code>cl_prevent_weapon_text_hints</code></summary>

stops weapon text hints from appearing

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_ragdoll_force_fade_time</code></summary>

Fade out ragdoll even if in players view after this many seconds

default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_ragdoll_force_fade_time_local_view_player</code></summary>

If the ragdoll is of the local view player then use the max of this and cl_ragdoll_force_fade_time for the fade time

default: `"20"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_ragdoll_force_fade_time_on_moving_geo</code></summary>

Fade out ragdoll even if in players view after this many seconds when touching moving geo.

default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_ragdoll_force_fade_time_titan</code></summary>

Fade out titan ragdoll even if in players view after this many seconds

default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_ragdoll_maxcount</code></summary>



default: `"8"`  
flags: `0x40000000`  
min value: `0`  
max value: `8`  
</details>
<details>
<summary><code>cl_ragdoll_self_collision</code></summary>



default: `"1"`  
flags: `0x40000002`  
</details>
<details>
<summary><code>cl_replayDelayTolerance</code></summary>



default: `"4"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_requireAnimForAnimEventsHdr</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_resend</code></summary>

Delay in seconds before the client will resend the 'connect' attempt

default: `"0.5"`  
flags: `0x80000`  
min value: `0.5`  
max value: `20`  
</details>
<details>
<summary><code>cl_resend_timeout</code></summary>

Total time allowed for the client to resend the 'connect' attempt

default: `"10"`  
flags: `0x80000`  
min value: `0.5`  
max value: `20000`  
</details>
<details>
<summary><code>cl_retire_low_priority_lights</code></summary>

Low priority dlights are replaced by high priority ones

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_runWeaponCloneThinkWhenHidden</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_safearea</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>cl_screenshotname</code></summary>

Custom Screenshot name

default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_scriptCompileAsync</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_script_perf_dump_on_shutdown</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_shadowupdatespacing</code></summary>



default: `"10.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_showClanTags</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_show_splashes</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_showerror</code></summary>

Show prediction errors, 2 for above plus detailed field deltas.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_showerror_watchfield</code></summary>

When showing prediction errors, only show fields that match this name

default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_showfiredbullets</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_showfps</code></summary>

Draw fps meter (1 = fps, 2 = smooth, 3 = server, 4 = Show+LogToFile, +10 = detailed )

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>cl_showfps_altframetime</code></summary>

Use the showfps_enabled time instead of the old cl_showfps time.

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>cl_showpausedimage</code></summary>

Show the 'Paused' image when game is paused.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_showpos</code></summary>

Draw current position at top of screen

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>cl_showsounds</code></summary>

Print server to client networked sounds to the console

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_showtime</code></summary>

Draw current demo time if recording a demo

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>cl_simulateAllModelsRegardless</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_simulationtimefix</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_skipAnimEventsOnProps</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_skipfastpath</code></summary>

Set to 1 to stop all models that go through the model fast path from rendering

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_smooth</code></summary>

Smooth view/eye origin after prediction errors

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_smooth_debug</code></summary>

Show prediction errors that are being smoothed

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_smoothtime</code></summary>

Smooth client's view after prediction error over this many seconds

default: `"0.25"`  
flags: `0x2`  
min value: `0.01`  
max value: `2`  
</details>
<details>
<summary><code>cl_threaded_bone_setup</code></summary>

Enable parallel processing of C_BaseAnimating::SetupBones()

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_updatedirty_async</code></summary>

Call UpdateDirtySpatialPartitionEntities on a worker thread.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_updatedirty_early</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_updaterate_mp</code></summary>

Number of packets per second of updates you are requesting from the server in mp

default: `"20"`  
flags: `0x10202`  
</details>
<details>
<summary><code>cl_upspeed</code></summary>



default: `"320"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_useFutureSnapForEvents</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_useLobbyTypeForChatroom</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_view_cone</code></summary>

Enable clamping view to animated/scripted viewcone

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_view_cone_debug</code></summary>

Show view cone debugging window

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_viewmodel_pre_animate</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_warnAboutSoundsOnInvalidEntities</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_yawspeed</code></summary>



default: `"210"`  
flags: `0x2`  
</details>
<details>
<summary><code>clampHostFrameTimeToOneTick_enable</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>clearOnAnimChange</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>client_deferredSnapshotScriptCalls</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>clientport</code></summary>

Host game client port

default: `"37005"`  
flags: `0x80000`  
</details>
<details>
<summary><code>cloak_enabled</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cloak_pilotNoiseFactor</code></summary>

Intensity of noise in pilot cloak aberration

default: `"0.25"`  
flags: `0x2002`  
</details>
<details>
<summary><code>cloak_pilotTint1</code></summary>

Brightness factor for center-left sample

default: `"0.35"`  
flags: `0x2002`  
</details>
<details>
<summary><code>cloak_pilotTint2</code></summary>

Brightness factor for upper-right sample

default: `"0.5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>cloak_pilotTint3</code></summary>

Brightness factor for lower-right sample

default: `"0.65"`  
flags: `0x2002`  
</details>
<details>
<summary><code>clock_bias_mp</code></summary>



default: `"-18.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>clock_bias_sp</code></summary>



default: `"-2.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>clock_showcorrections</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>clock_showdebuginfo</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>closecaption</code></summary>

Enable close captioning. 1 = dialogue only, 2 = dialogue and sound effects.

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>cockpitDrift_scalePitch</code></summary>



default: `"0.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cockpitDrift_scaleYaw</code></summary>



default: `"0.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cockpitDrift_speedPitch</code></summary>



default: `"0.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cockpitDrift_speedYaw</code></summary>



default: `"0.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cockpitShake_sourceRollRange</code></summary>

The range of weapon kick roll that will be sampled for cockpit shake.

default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>cockpitShake_translateRange</code></summary>

Max amount of cockpit shake.

default: `"0.6"`  
flags: `0x2`  
</details>
<details>
<summary><code>cockpit_damage_chroma_scale</code></summary>



default: `"0.4"`  
flags: `0x2`  
</details>
<details>
<summary><code>cockpit_hit_chroma_max_time</code></summary>

Time to get rid of the most recent hit_chroma adjustment when at near 0 health.

default: `"0.6"`  
flags: `0x2`  
</details>
<details>
<summary><code>cockpit_hit_chroma_scale</code></summary>



default: `"0.2"`  
flags: `0x2`  
</details>
<details>
<summary><code>cockpit_pitch_down_frac</code></summary>

fractional amount that cockpit pitches as you look down

default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cockpit_pitch_up_frac</code></summary>

fractional amount that cockpit pitches as you look up

default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cockpit_screen_boot_chroma_scale</code></summary>



default: `"0.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cockpit_screen_boot_delay_bottom</code></summary>



default: `"1.25"`  
flags: `0x2`  
</details>
<details>
<summary><code>cockpit_screen_boot_delay_left</code></summary>



default: `"0.25"`  
flags: `0x2`  
</details>
<details>
<summary><code>cockpit_screen_boot_delay_mid</code></summary>



default: `"0.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>cockpit_screen_boot_delay_right</code></summary>



default: `"0.75"`  
flags: `0x2`  
</details>
<details>
<summary><code>cockpit_screen_boot_delay_top</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>coll_spatial_entry_limit_client</code></summary>

How many entries are used in the spatial acceleration structure for dynamic entities on the client.

default: `"140"`  
flags: `0x2`  
</details>
<details>
<summary><code>coll_spatial_optimize_prefetch</code></summary>

Prefetch memory into the cache before optimizing spatial acceleration trees. This does more work, but tends to be faster overall.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>coll_use_bolt_size</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>colorblind_mode</code></summary>



default: `"0"`  
flags: `0x41000000`  
min value: `0`  
max value: `3`  
</details>
<details>
<summary><code>communities_doRealNameLookupsForCommunityCreators</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>communities_enabled</code></summary>

Enable communities

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>communities_hostname</code></summary>



default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>community</code></summary>

Our current community

default: `""`  
flags: `0x200`  
</details>
<details>
<summary><code>community_abortCommunitySettingsTime</code></summary>



default: `"20"`  
flags: `0x2`  
</details>
<details>
<summary><code>community_abortUserInfoTime</code></summary>



default: `"20"`  
flags: `0x2`  
</details>
<details>
<summary><code>community_browse_excludeMine</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>community_clantags</code></summary>

put community name in the clan tag

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>community_doRealNameLookupsForInbox</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>community_frame_run</code></summary>

Communities should run it's frame update.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>community_queryServerWhenOrphaned</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>community_replaceInboxTokens</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>community_replaceInboxTokens</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>community_resolveNames</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>community_resolveNames</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>community_send_server_voice</code></summary>

Communities will route voice data to the chat server!

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>community_spam</code></summary>

Whether communities should spam to the console log

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>community_staleCommunitySettingsTime</code></summary>



default: `"60"`  
flags: `0x2`  
</details>
<details>
<summary><code>community_staleUserInfoTime</code></summary>



default: `"120"`  
flags: `0x2`  
</details>
<details>
<summary><code>con_logfile</code></summary>

Console output gets written to this file

default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>con_timestamp</code></summary>

Prefix console.log entries with timestamps

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cpu_level</code></summary>

CPU Level - Default: High

default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>cpu_level</code></summary>

CPU Level - Default: High

default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>createentitydecals</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>csm0_on_worker</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_cascade_res</code></summary>

Set the cascading shadow maps rendertarget resolution

default: `"1024"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_cascade_res</code></summary>

Set the cascading shadow maps rendertarget resolution

default: `"1024"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_coverage</code></summary>

Set the cascading shadow maps coverage

default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_culling_use_base_planes</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_culling_use_exclusion_planes</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_culling_use_inclusion_planes</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_culling_use_planes</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_debug_2d</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_debug_culling</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_debug_vis_hi_range</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_debug_vis_lo_range</code></summary>



default: `".35"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_depth_bias</code></summary>



default: `"-0.000005f"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_dropsequence_adjusted_coverage</code></summary>

Coverage for csm_dropsequence_adjustment

default: `"6400"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_dropsequence_adjustment</code></summary>

Adjust CSM 2 coverage during drop sequence for STATICSHADOWMODE_GENERATE_ONCE in order to prevent drop ship shadow from being clamped.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_enabled</code></summary>

Set whether to render cascading shadow maps

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_fadeModels</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_force_no_csm_in_reflections</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_frustum_draw</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_frustum_draw_lock</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_ignore_cascade12</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_ignore_edge_planes</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_ignore_face_planes</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_max_z_offset</code></summary>

Note csm_z_cover_world expands Z range as well

default: `"1000"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_min_z_offset</code></summary>

Note csm_z_cover_world expands Z range as well

default: `"-1000"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_renderable_shadows</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_rope_shadows</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_rot_override</code></summary>

map_settings_override MUST BE ENABLED FOR THIS TO BE FUNCTIONAL.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_rot_x</code></summary>

map_settings_override MUST BE ENABLED FOR THIS TO BE FUNCTIONAL.

default: `"50"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_rot_y</code></summary>

map_settings_override MUST BE ENABLED FOR THIS TO BE FUNCTIONAL.

default: `"43"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_shadow_split_lerp_factor_range</code></summary>



default: `".1"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_texel_size_cascade_0</code></summary>



default: `"0.25"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_texel_size_cascade_1</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_texel_size_cascade_2</code></summary>



default: `"4.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_texel_size_cascade_onecascade</code></summary>



default: `"2.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_use_env_light_direction</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_world_shadow_meshes</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_world_shadows</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_z_cover_world</code></summary>

Expands CSM Depth coverage. 1 - Sea Height to Jump Height by Script, 2 - Static shadow's depth range

default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>curl_allowHTTPS</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>curl_preloadDlls</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>curl_spamAllQueryStates</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>cursorWide</code></summary>



default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>damageIndicatorReplayTimeOffset</code></summary>

Artificial delay of damage indicator in replay

default: `"0.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>damage_indicator_style_pilot</code></summary>



default: `"2"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>damage_indicator_style_titan</code></summary>



default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>damageinfo_defendInvalidValues</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>debugFootstepEffects</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>debug_debug_overlay</code></summary>

Enable debug of the debug overlays

default: `"0"`  
flags: `0x4004`  
</details>
<details>
<summary><code>debug_force_textRestriction</code></summary>



default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>debug_force_ugcRestriction</code></summary>



default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>debug_force_voiceRestriction</code></summary>



default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>debug_map_crc</code></summary>

Prints CRC for each map lump loaded

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>decal_clip_debug_draw</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>decal_clip_debug_groups</code></summary>

this kicks off this many work groups when a decal is spawned instead of one for each triangle on the model. 0 is disabled

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>defer_weapon_effects</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>delayPostSnapshotNotificationsToAfterInterpolation</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>demo_autoRecord</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>demo_autoRecordName</code></summary>



default: `"demo"`  
flags: `0x2`  
</details>
<details>
<summary><code>demo_connect_string</code></summary>

Connect string for demo UI

default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>demo_ui_enable</code></summary>

Suffix for the demo UI

default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>devStats</code></summary>

True if game should report dev stats.

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>developer</code></summary>

Set developer message level

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>disable_player_use_prompts</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>discord_largeImage</code></summary>



default: `"default"`  
flags: `0x2`  
</details>
<details>
<summary><code>discord_smallImage</code></summary>



default: `"default_small"`  
flags: `0x2`  
</details>
<details>
<summary><code>discord_updatePresence</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>dlight_default_falloff</code></summary>

default half-distance fraction for legacy dlights.

default: `"0.3"`  
flags: `0x80`  
</details>
<details>
<summary><code>dlight_enable</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>dlight_overlay</code></summary>

Draw debug overlay of dlight array

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>dodge_cockpitHack</code></summary>

Hack to avoid eye moving too far back in cockpit

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>dodge_cockpitOffsetMax</code></summary>

Cockpit translation while dodging

default: `"3"`  
flags: `0x2002`  
</details>
<details>
<summary><code>dodge_cockpitTiltMax</code></summary>

Additional view tilt applied to the cockpit while dodging

default: `"4"`  
flags: `0x2002`  
</details>
<details>
<summary><code>dodge_vertical_enable</code></summary>

Enables vertical dodge

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>dodge_vertical_horzspeedscale</code></summary>

Horizontal speed retained when dodging vertically

default: `"0.5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>dodge_vertical_in_air</code></summary>

Allow dodge to still apply vertical acceleration when player is in the air

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>dodge_vertical_threshold</code></summary>

Stick deflection before dodge becomes vertical

default: `"0.1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>dodge_viewTiltDecreaseSpeed</code></summary>

Speed at which view tilt decreases while dodging (degrees/sec)

default: `"2.5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>dodge_viewTiltFalloffTime</code></summary>

Time during which view tilt decays to zero while dodging

default: `".7"`  
flags: `0x2002`  
</details>
<details>
<summary><code>dodge_viewTiltIncreaseSpeed</code></summary>

Speed at which view tilt increases while dodging (degrees/sec)

default: `"5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>dodge_viewTiltMax</code></summary>

Amount of view tilt while dodging in degrees

default: `"10"`  
flags: `0x2002`  
</details>
<details>
<summary><code>dof_enable</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>dof_farDepthEnd</code></summary>



default: `"3000"`  
flags: `0x2`  
</details>
<details>
<summary><code>dof_farDepthStart</code></summary>



default: `"2000"`  
flags: `0x2`  
</details>
<details>
<summary><code>dof_monitorFarDepthEnd</code></summary>



default: `"3000"`  
flags: `0x2`  
</details>
<details>
<summary><code>dof_monitorFarDepthStart</code></summary>



default: `"2000"`  
flags: `0x2`  
</details>
<details>
<summary><code>dof_monitorNearDepthEnd</code></summary>



default: `"7.7"`  
flags: `0x2`  
</details>
<details>
<summary><code>dof_monitorNearDepthStart</code></summary>



default: `"7.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>dof_nearDepthEnd</code></summary>



default: `"7.7"`  
flags: `0x2`  
</details>
<details>
<summary><code>dof_nearDepthStart</code></summary>



default: `"7.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>dof_overrideParams</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>dof_variable_blur</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>dormant_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>draw_target_info_offscreen</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>dtwatchclass</code></summary>

Watch all fields encoded with this table.

default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>dtwatchdecode</code></summary>

When watching show decode.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>dtwatchencode</code></summary>

When watching show encode.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>dtwatchent</code></summary>

Watch this entities data table encoding.

default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>dtwatchvar</code></summary>

Watch the named variable.

default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>dump_varsights_calculations</code></summary>

Dumps one frame of variable sights calculations and turns itself off.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>durango_voice_chat_team_only</code></summary>

Only turn on voice chat for players on the same team

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>dvs_enable</code></summary>

Enable dynamic viewport scaling.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>dvs_gpuframetime_max</code></summary>

GPU frametime threshold above which DVS will start decreasing the scale. Specified in microseconds.

default: `"16500"`  
flags: `0x2`  
</details>
<details>
<summary><code>dvs_gpuframetime_min</code></summary>

GPU frametime threshold below which DVS will start increasing the scale. Specified in microseconds.

default: `"15000"`  
flags: `0x2`  
</details>
<details>
<summary><code>dvs_scale_min</code></summary>

Smallest scale the viewport dimensions can be scaled by.

default: `"0.5f"`  
flags: `0x2`  
min value: `0.01`  
max value: `1`  
</details>
<details>
<summary><code>edge_override_depth</code></summary>

edge_override_depth -1|0|1

default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>edge_override_depth</code></summary>

edge_override_depth -1|0|1

default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>edge_override_silhouette</code></summary>

edge_override_silhouette -1|0|1

default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>edge_override_silhouette</code></summary>

edge_override_silhouette -1|0|1

default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>enable_KVFileOverrides</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>enable_debug_overlays</code></summary>

Enable rendering of debug overlays

default: `"1"`  
flags: `0x4004`  
</details>
<details>
<summary><code>enable_height_based_land_anims</code></summary>

Enables different land animations based on the height of the fall. These may just be duplicates of each other.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>enable_height_based_land_anims_titans</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>enable_skeleton_draw</code></summary>

Render skeletons in wireframe

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>encrypt_multiKey</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>ent_lightweightEnts</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>ent_repack_almostFull</code></summary>



default: `"3000"`  
flags: `0x2`  
</details>
<details>
<summary><code>ent_repack_threshhold</code></summary>



default: `"0.0001"`  
flags: `0x2`  
</details>
<details>
<summary><code>entity_skipRedundantAddEffects</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>entity_useNetworkFieldBuffer</code></summary>



default: `"1"`  
flags: `0x400002`  
</details>
<details>
<summary><code>error_if_non_standard_ent_create</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>eula_version</code></summary>

What the current version of the EULA is

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>eula_version_accepted</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>eventseq_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>everything_unlocked</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>fakelag_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>fast_poly_convert</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>fatal_script_error_prompt</code></summary>



default: `""`  
flags: `0x2002`  
</details>
<details>
<summary><code>fatal_script_errors</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>fatal_script_errors_client</code></summary>

Enable fatal errors for client script. -1 will revert to using "fatal_script_errors"

default: `"-1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>fatal_script_errors_server</code></summary>

Enable fatal errors for server script. -1 will revert to using "fatal_script_errors"

default: `"-1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>fd_playlist_bits</code></summary>

which frontier defense playlists difficulties are active

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>filesystem_buffer_size</code></summary>

Size of per file buffers. 0 for none

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>filesystem_max_stdio_read</code></summary>



default: `"16"`  
flags: `0x2`  
</details>
<details>
<summary><code>filesystem_native</code></summary>

Use native FS or STDIO

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>filesystem_report_buffered_io</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>filesystem_unbuffered_io</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>filesystem_use_overlapped_io</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>fire_animevents_overlay_not_active</code></summary>

fires anim events even if the overlay isn't active

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>first_person_bullet_delay</code></summary>

Set the amount of additional delay for first person bullets fired with net_optimize_weapons in seconds. Required so bullets match animations with cl_predict 0 and in kill replay

default: `"0.1f"`  
flags: `0x2002`  
</details>
<details>
<summary><code>first_person_proxy_blend_distance</code></summary>



default: `"50"`  
flags: `0x2`  
</details>
<details>
<summary><code>first_person_proxy_debug</code></summary>



default: `"0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>firsttime_mp_message</code></summary>

first time joining multiplayer

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>fog_enable</code></summary>

map_settings_override MUST BE ENABLED FOR THIS TO BE FUNCTIONAL.

default: `"1"`  
flags: `0x40000002`  
</details>
<details>
<summary><code>fog_enable_water_fog</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>fog_enableskybox</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>force3PLaserAttachment</code></summary>



default: `"HEADFOCUS"`  
flags: `0x2002`  
</details>
<details>
<summary><code>force_EAAccess</code></summary>



default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>fps_max</code></summary>

Frame rate limiter. -1 indicates use the desktop refresh. 0 is unlocked.

default: `"-1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>fps_max_use_refresh</code></summary>

Use refresh rate for fps_max.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>fps_max_vsync</code></summary>

Frame rate limiter with vsync is enabled.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>freecam_swallowButtonInput</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>freefall_sound_autoplay_time</code></summary>

If the player falls for longer than this amount of time freefall sounds will automatically start playing.

default: `"1.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>freefall_sound_height</code></summary>

Height player must be falling from to trigger freefall sound effects.

default: `"200.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>friends_onlineUpdateInterval</code></summary>



default: `"10"`  
flags: `0x2`  
</details>
<details>
<summary><code>fs_intralevel_reads</code></summary>

Internal var to tell the file system that we are in an intraread state...

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>fs_monitor_read_from_pack</code></summary>

0:Off, 1:Any, 2:Sync only

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>fs_report_intra_level_readopens</code></summary>

0:Off, 1:NotAudio, 2:All

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>fs_report_long_reads</code></summary>

0:Off, 1:All (for tracking accumulated duplicate read times), >1:Microsecond threshold

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>fs_report_sync_opens</code></summary>

0:Off, 1:Always, 2:Not during map load

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>fs_report_sync_opens_callstack</code></summary>

0 to not display the call-stack when we hit a fs_report_sync_opens warning. Set to 1 to display the call-stack.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>fs_report_sync_opens_fatal</code></summary>



default: `"0"`  
flags: `0x40000002`  
</details>
<details>
<summary><code>fs_showAllReads</code></summary>

0:Off, 1:On

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>fs_vpk_file_open</code></summary>

0: No reporting, 1: Patch:VPKFilePath, 2: Patch:VPKFilePath:PartialPath

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>fs_warning_mode</code></summary>

0:Off, 1:Warn main thread, 2:Warn other threads

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>func_break_max_pieces</code></summary>



default: `"15"`  
flags: `0x2080`  
</details>
<details>
<summary><code>fx_debug</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>fx_deferWorldTraceConstraint</code></summary>

'Collision via traces' ops using collision mode 0 use deferred traces.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>fx_glass_velocity_cap</code></summary>

Maximum downwards speed of shattered glass particles

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>fx_impact_ally</code></summary>



default: `"0.49 0.76 1.0 1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>fx_impact_enemy</code></summary>



default: `"1.0 0.47 0.13 1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>fx_impact_neutral</code></summary>



default: `"0.86 0.86 0.86 1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>fx_screenspacepass</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>g_debug_ragdoll_removal</code></summary>



default: `"0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>g_ragdoll_fadespeed</code></summary>



default: `"600"`  
flags: `0x2`  
</details>
<details>
<summary><code>g_ragdoll_important_maxcount</code></summary>



default: `"2"`  
flags: `0x2002`  
</details>
<details>
<summary><code>g_ragdoll_lvfadespeed</code></summary>



default: `"100"`  
flags: `0x2`  
</details>
<details>
<summary><code>gameCursor_ModeActive</code></summary>

Globally activates/deactivates game cursor mode

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>gameCursor_Velocity</code></summary>

Game cursor velocity under joystick control

default: `"1300.0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_ads_advanced_sensitivity_scalar_0</code></summary>

Gamepad ads sensitivity for 1x scopes / ironsights.

default: `"1.0"`  
flags: `0x1000000`  
min value: `0.1`  
max value: `20`  
</details>
<details>
<summary><code>gamepad_ads_advanced_sensitivity_scalar_1</code></summary>

Gamepad ads sensitivity for 2x scopes.

default: `"1.0"`  
flags: `0x1000000`  
min value: `0.1`  
max value: `20`  
</details>
<details>
<summary><code>gamepad_ads_advanced_sensitivity_scalar_2</code></summary>

Gamepad ads sensitivity for 3x scopes.

default: `"1.0"`  
flags: `0x1000000`  
min value: `0.1`  
max value: `20`  
</details>
<details>
<summary><code>gamepad_ads_advanced_sensitivity_scalar_3</code></summary>

Gamepad ads sensitivity for 4x scopes.

default: `"1.0"`  
flags: `0x1000000`  
min value: `0.1`  
max value: `20`  
</details>
<details>
<summary><code>gamepad_ads_advanced_sensitivity_scalar_4</code></summary>

Gamepad ads sensitivity for 6x scopes.

default: `"1.0"`  
flags: `0x1000000`  
min value: `0.1`  
max value: `20`  
</details>
<details>
<summary><code>gamepad_ads_advanced_sensitivity_scalar_5</code></summary>

Gamepad ads sensitivity for 8x scopes.

default: `"1.0"`  
flags: `0x1000000`  
min value: `0.1`  
max value: `20`  
</details>
<details>
<summary><code>gamepad_ads_advanced_sensitivity_scalar_6</code></summary>

Gamepad ads sensitivity for 10x scopes.

default: `"1.0"`  
flags: `0x1000000`  
min value: `0.1`  
max value: `20`  
</details>
<details>
<summary><code>gamepad_ads_advanced_sensitivity_scalar_7</code></summary>

Gamepad ads sensitivity for an unused scope.

default: `"1.0"`  
flags: `0x1000000`  
min value: `0.1`  
max value: `20`  
</details>
<details>
<summary><code>gamepad_aim_assist_hip_high_power_scopes</code></summary>

Gamepad uses aim assist in Hip with high powered scopes

default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_aim_assist_hip_low_power_scopes</code></summary>

Gamepad uses aim assist in Hip with low powered scopes

default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_aim_assist_melee</code></summary>

Gamepad uses aim assist with melee weapons

default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_aim_assist_scope_0</code></summary>

Aim Assist Enabled for 1x scopes / ironsights.

default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_aim_assist_scope_1</code></summary>

Aim Assist Enabled for 2x scopes.

default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_aim_assist_scope_2</code></summary>

Aim Assist Enabled for 3x scopes.

default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_aim_assist_scope_3</code></summary>

Aim Assist Enabled for 4x scopes.

default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_aim_assist_scope_4</code></summary>

Aim Assist Enabled for 6x scopes.

default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_aim_assist_scope_5</code></summary>

Aim Assist Enabled for 8x scopes.

default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_aim_assist_scope_6</code></summary>

Aim Assist Enabled for 10x scopes.

default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_aim_assist_scope_7</code></summary>

Aim Assist Enabled for an unused scope.

default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_aim_assist_sniper_scopes</code></summary>

Enable / Disable Gamepad options to adjust aim assist with sniper scopes

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_aim_speed</code></summary>



default: `"2"`  
flags: `0x1000000`  
min value: `0`  
max value: `7`  
</details>
<details>
<summary><code>gamepad_aim_speed_ads_0</code></summary>



default: `"-1"`  
flags: `0x1000000`  
min value: `-1`  
max value: `7`  
</details>
<details>
<summary><code>gamepad_aim_speed_ads_1</code></summary>



default: `"-1"`  
flags: `0x1000000`  
min value: `-1`  
max value: `7`  
</details>
<details>
<summary><code>gamepad_aim_speed_ads_2</code></summary>



default: `"-1"`  
flags: `0x1000000`  
min value: `-1`  
max value: `7`  
</details>
<details>
<summary><code>gamepad_aim_speed_ads_3</code></summary>



default: `"-1"`  
flags: `0x1000000`  
min value: `-1`  
max value: `7`  
</details>
<details>
<summary><code>gamepad_aim_speed_ads_4</code></summary>



default: `"-1"`  
flags: `0x1000000`  
min value: `-1`  
max value: `7`  
</details>
<details>
<summary><code>gamepad_aim_speed_ads_5</code></summary>



default: `"-1"`  
flags: `0x1000000`  
min value: `-1`  
max value: `7`  
</details>
<details>
<summary><code>gamepad_aim_speed_ads_6</code></summary>



default: `"-1"`  
flags: `0x1000000`  
min value: `-1`  
max value: `7`  
</details>
<details>
<summary><code>gamepad_aim_speed_ads_7</code></summary>



default: `"-1"`  
flags: `0x1000000`  
min value: `-1`  
max value: `7`  
</details>
<details>
<summary><code>gamepad_button_layout</code></summary>

Gamepad button layout (used by menus)

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_buttons_are_southpaw</code></summary>

Gamepad button layouts should use southpaw variants (used by menus)

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_custom_ads_pitch</code></summary>



default: `"75.0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_custom_ads_turn_delay</code></summary>



default: `"0.25"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_custom_ads_turn_pitch</code></summary>



default: `"30.0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_custom_ads_turn_time</code></summary>



default: `"1.0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_custom_ads_turn_yaw</code></summary>



default: `"30.0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_custom_ads_yaw</code></summary>



default: `"110.0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_custom_assist_on</code></summary>



default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_custom_curve</code></summary>



default: `"10.0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_custom_deadzone_in</code></summary>



default: `"0.15"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_custom_deadzone_out</code></summary>



default: `"0.02"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_custom_enabled</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_custom_hip_pitch</code></summary>



default: `"120.0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_custom_hip_turn_delay</code></summary>



default: `"0.0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_custom_hip_turn_pitch</code></summary>



default: `"0.0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_custom_hip_turn_time</code></summary>



default: `"0.33"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_custom_hip_turn_yaw</code></summary>



default: `"220.0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_custom_hip_yaw</code></summary>



default: `"160.0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_custom_pilot</code></summary>



default: `"0,1,2,3,4,5,6,7,8,9,10,11,12,13"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_custom_titan</code></summary>



default: `"0,1,2,3,4,5,6,7,8,9,10,11,12,13"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_deadzone_index_look</code></summary>



default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_deadzone_index_move</code></summary>



default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_enabled</code></summary>

True if the gamepad is enabled, false otherwise.

default: `"1"`  
flags: `0x2`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>gamepad_look_curve</code></summary>



default: `"0"`  
flags: `0x1000000`  
min value: `0`  
max value: `4`  
</details>
<details>
<summary><code>gamepad_stick_layout</code></summary>

Gamepad stick layout (used by menus)

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_toggle_ads</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_togglecrouch_hold</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_trigger_threshold</code></summary>



default: `"30"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_use_per_scope_ads_settings</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_use_per_scope_sensitivity_scalars</code></summary>

Gamepad uses the per scope scalars

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gamepad_use_type</code></summary>

Gamepad use scheme (used by menus), 0: hold use, tap reload, 1: tap use, hold reload, 2: tap use/reload

default: `"2"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gameui_xbox</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>gamma_adjusted</code></summary>

Whether player has done gamma adjustment

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>gatherprops_no_wait</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>gfx_desaturate_force</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>gl_clear_color_buffer</code></summary>

Enable or disable the clearing of the main color buffer.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>gl_clear_fogcolor</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>gl_clear_randomcolor</code></summary>

Clear the back buffer to random colors every frame. Helps spot open seams in geometry.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>glass_break_required_speed</code></summary>



default: `"150"`  
flags: `0x6000`  
</details>
<details>
<summary><code>glass_shatter_direction_force_scale</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>glass_shatter_force_scale</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>glass_shatter_size_scale</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>glass_shatter_use_real_direction</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>glitch_aberrationScale</code></summary>

How far apart the glitch cloak samples should be.

default: `"10"`  
flags: `0x2002`  
</details>
<details>
<summary><code>global_lighting_partial_update</code></summary>

Allow partial uploads of GPU lights (optimization.)

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>gpu_count</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>gpu_driven_tex_stream</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>gpu_driven_tex_stream_single_thread</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>gpu_level</code></summary>

GPU Level - Default: High

default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>gpu_level</code></summary>

GPU Level - Default: High

default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>gpu_mem_level</code></summary>

Memory Level - Default: Normal

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>gpu_mem_level</code></summary>

Memory Level - Default: Normal

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>gpu_vram_size_mb</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>grapple_accel_human</code></summary>

Speed added per second from grapple, up to the grapple_speedRamp* speed

default: `"1000"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_accel_titan</code></summary>

Speed added per second from grapple, up to the grapple_speedRamp* speed

default: `"1800"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_around_obstacle_accel</code></summary>

Acceleration around obstacles while grappling

default: `"1000"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_autoMantle</code></summary>

After detaching from grapple, how long to keep trying to automantle

default: `"0.25"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_autoMeleeConvergeTime</code></summary>

Simplify relative velocities when the enemy is this many seconds away from hitting us (increases chances of a hit)

default: `"1.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_autoMeleeOnDetach</code></summary>

Starts a melee sequence when the grapple detaches.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_autoMeleePredict</code></summary>

Whether to run grapple melee logic on the client (tends to mispredict anyway)

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_autoMeleePredictTime</code></summary>

Melee begins when the enemy is this many seconds away from hitting us

default: `"0.13"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_autoMeleeViewRotateSpeedFar</code></summary>

Speed at which view rotates toward grapple melee target

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_autoMeleeViewRotateSpeedNear</code></summary>

Speed at which view rotates toward grapple melee target

default: `"3"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_debug</code></summary>

Show grapple debug info

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_decelMeleeStrength</code></summary>

Strength of extra deceleration that forces melee targets to come to you

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_decel_human</code></summary>

Deceleration of player's speed that doesn't go toward the grapple point

default: `"425"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_decel_titan</code></summary>

Deceleration of player's speed that doesn't go toward the grapple point

default: `"200"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_detachExtraAllowedLength</code></summary>

Extra allowed grapple length before detaching once it's attached

default: `"256"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_disableMeleeWhenActive</code></summary>

Disallows melee when the grapple is out.

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_dontFightGravity</code></summary>

Ignores downward speed when applying deceleration, so that gravity continues to pull you down

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_fallSpeed</code></summary>

Fall speed of the grapple hook while it's returning

default: `"300"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_forcedRetractVel</code></summary>

Return speed of grapple hook when grapple is finished or cancelled

default: `"3000"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_gracePeriod</code></summary>

Length of time player can grapple without using a charge, in case they mess up

default: `"0.25"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_gravityPushUnderContribution</code></summary>

Pushing forward while looking "under" the grapple point increases gravity this much

default: `"2"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_initialImpulseOffGround_human</code></summary>

Initial launch speed off the ground when grapple connects

default: `"50"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_initialImpulseOffGround_human_npc</code></summary>

Initial launch speed off the ground when grapple connects

default: `"150"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_initialImpulseOffGround_titan</code></summary>

Initial launch speed off the ground when grapple connects

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_initialImpulse_human</code></summary>

Initial launch speed when grapple connects

default: `"350"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_initialImpulse_titan</code></summary>

Initial launch speed when grapple connects

default: `"350"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_initialSlowFracVert_human</code></summary>

Fraction of vertical speed that is retained when grapple connects

default: `"0.4"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_initialSlowFracVert_titan</code></summary>

Fraction of vertical speed that is retained when grapple connects

default: `"0.1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_initialSlowFrac_human</code></summary>

Fraction of XY speed that is retained when grapple connects

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_initialSlowFrac_titan</code></summary>

Fraction of XY speed that is retained when grapple connects

default: `"0.5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_initialSpeedMin_human</code></summary>

When grapple connects, player speed is immediately set to at least this value (negative = away, positive = towards)

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_initialSpeedMin_titan</code></summary>

When grapple connects, player speed is immediately set to at least this value (negative = away, positive = towards)

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_jumpFrac</code></summary>

Jump velocity multiplier when grappled

default: `"1.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_letGravityHelpCosAngle</code></summary>

Don't ignore gravity when grappling downward this much (0 is horizontal, 1 is straight down)

default: `"0.8"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_lift</code></summary>

Distance above grapple hook that player is pulled to

default: `"25"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_pullDelay_human</code></summary>

Grapple delay between attachment and acceleration

default: `"0.2"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_pullDelay_titan</code></summary>

Grapple delay between attachment and acceleration

default: `"0.2"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_retractVel</code></summary>

Return speed of grapple hook when it hasn't hit anything yet

default: `"6000"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_rodeoVerticalImpulse</code></summary>

Vertical impulse applied to the player when grappling off of a rodeo.

default: `"750"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_shootVel</code></summary>

Outward speed of grapple hook

default: `"2000"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_speedRampMax_human</code></summary>

Player will accelerate to this speed after grapple_speedRampTime has passed

default: `"400"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_speedRampMax_titan</code></summary>

Player will accelerate to this speed after grapple_speedRampTime has passed

default: `"750"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_speedRampMin_human</code></summary>

Player will accelerate to this speed while grappling; lerps to grapple_speedRampMax over grapple_speedRampTime

default: `"50"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_speedRampMin_titan</code></summary>

Player will accelerate to this speed while grappling; lerps to grapple_speedRampMax over grapple_speedRampTime

default: `"400"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_speedRampTime_human</code></summary>

Time from grapple_speedRampMin to grapple_speedRampMax

default: `"1.5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_speedRampTime_titan</code></summary>

Time from grapple_speedRampMin to grapple_speedRampMax

default: `"1.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_swingAngle</code></summary>

Maximum angle from vertical that swinging will generate acceleration (it will tend to zero acceleration at this angle)

default: `"45"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_swingPullAngle</code></summary>

If the player is pushing forward within this angle of the pull direction, then switch out of swinging mode.

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_swingPullSpeedLength</code></summary>

When swinging, the grapple pull speed scale begins to scale back to 1.0 at lengths below this

default: `"300.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_swingPullSpeedScale</code></summary>

When swinging, the grapple pull speed is scaled by this much

default: `"0.025"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_titanEmbarkDist</code></summary>

Distance at which to begin embark when grappling to your own titan.

default: `"250"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grapple_windowCheckDist</code></summary>

Check for window hints at this distance from grapple point

default: `"150"`  
flags: `0x2002`  
</details>
<details>
<summary><code>gravity_grenade_decel</code></summary>

Deceleration applied by gravity grenade to nearby objects

default: `"20000"`  
flags: `0x2002`  
</details>
<details>
<summary><code>gravity_grenade_projectile_min_speed</code></summary>

Gravity grenade never slows projectiles below this speed

default: `"600"`  
flags: `0x2002`  
</details>
<details>
<summary><code>ground_debug</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>ground_trace_hull_radius</code></summary>

How wide of a sphere is the trace for getting a character's ground surface

default: `"12.0f"`  
flags: `0x2002`  
</details>
<details>
<summary><code>grx_hasUnknownItems</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>gtao_angle_bias</code></summary>

angle in degree [0-90)

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>gtao_intensity</code></summary>



default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>gtao_intensity_in_lobby</code></summary>



default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>gtao_thickness_heuristic</code></summary>

in range of [0,1)

default: `"0.2"`  
flags: `0x2`  
</details>
<details>
<summary><code>hasAnyAssetsWithDiscardedStreamableData</code></summary>



default: `"0"`  
flags: `0x200`  
</details>
<details>
<summary><code>hasMic</code></summary>



default: `"0"`  
flags: `0x200`  
</details>
<details>
<summary><code>hasPartialInstall</code></summary>



default: `"0"`  
flags: `0x200`  
</details>
<details>
<summary><code>hbao_angle_bias</code></summary>

angle in degree [0-90)

default: `"6"`  
flags: `0x2`  
</details>
<details>
<summary><code>hbao_intensity</code></summary>



default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>hbao_stepsize_random</code></summary>



default: `"0.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>hbaobasic_tangent_bias</code></summary>

angle in degree [0-90)

default: `"25"`  
flags: `0x2`  
</details>
<details>
<summary><code>hidehud</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>highlight_deferred_update</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>highlight_draw</code></summary>

highlight_draw 0|1

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>highlight_lazy_clear_buffers</code></summary>

highlight_lazy_clear_buffers 0|1

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>highlight_object_max_count</code></summary>

highlight_object_max_count OBJECT_MAX_COUNT

default: `"255"`  
flags: `0x2`  
</details>
<details>
<summary><code>hitbox_bodygroup_check</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>hitch_alert_active</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>hitch_alert_color</code></summary>

The hitch/choke allerts will use this color.

default: `"255 255 0 255"`  
flags: `0x2`  
</details>
<details>
<summary><code>hitch_alert_show_large_snapshots</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>host_RunFrameServerAlways</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>host_ShowIPCCallCount</code></summary>

Print # of IPC calls this number of times per second. If set to -1, the # of IPC calls is shown every frame.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>host_flush_threshold</code></summary>

Memory threshold below which the host should flush caches between server instances

default: `"12"`  
flags: `0x80000`  
</details>
<details>
<summary><code>host_framerate</code></summary>

Set to lock per-frame time elapse.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>host_hasIrreversibleShutdown</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>host_limitlocal</code></summary>

Apply cl_cmdrate and cl_updaterate to loopback connection

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>host_map</code></summary>

Current map name.

default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>host_preload_shaders</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>host_print_frame_times</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>host_profile</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>host_runframe_input_parcelremainder</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>host_server_thread_min_ticks</code></summary>

Only run the server thread when it needs this many ticks.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>host_sleep</code></summary>

Force the host to sleep a certain number of milliseconds each frame.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>host_speeds</code></summary>

Show general system running times.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>host_syncfps</code></summary>

Synchronize real render time to host_framerate if possible.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>host_thread_join_fast</code></summary>

If true we force the server thread join before existing '_Host_RunFrame'

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>host_thread_mode</code></summary>

Run the host in threaded mode, (0 == off, 1 == if multicore, 2 == force)

default: `"1"`  
flags: `0x40000002`  
</details>
<details>
<summary><code>host_threaded_sound</code></summary>

Run the sound on a thread (independent of mix)

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>host_timescale</code></summary>

Prescale the clock by this amount.

default: `"1.0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>hostip</code></summary>

Host game server ip

default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>hostname</code></summary>

Hostname for server.

default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>hostport</code></summary>

Host game server port

default: `"37015"`  
flags: `0x80000`  
</details>
<details>
<summary><code>http_StryderKey</code></summary>



default: `"LABj38NWSTxHUhdYaP62ZU6HtutCas3L"`  
flags: `0x12`  
</details>
<details>
<summary><code>http_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>http_debug_forceFailRate</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>http_debug_forceFailStatus</code></summary>



default: `"429"`  
flags: `0x2`  
</details>
<details>
<summary><code>http_failuresAsErrors</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>http_maxAllocateAttempts</code></summary>



default: `"10"`  
flags: `0x2`  
</details>
<details>
<summary><code>http_recv_fail_realloc</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>http_sandbox</code></summary>



default: `"EARW.50"`  
flags: `0x2`  
</details>
<details>
<summary><code>http_showQueries</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>hud_autoreloadscript</code></summary>

Automatically reloads the animation script each time one is ran

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>hud_setting_accessibleChat</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_adsDof</code></summary>



default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_compactOverHeadNames</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_damageIndicatorStyle</code></summary>



default: `"2"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_damageTextStyle</code></summary>



default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_enableModWheel</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_healthUseOnHold</code></summary>

use health by holding button

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_healthWheelToggle</code></summary>

toggle health wheel on press

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_healthWheelUseOnRelease</code></summary>

use health after selecting it

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_lootPromptStyle</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_minimapRotate</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_ordnanceUseOnHold</code></summary>

use ordnance by holding button

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_ordnanceWheelToggle</code></summary>

toggle ordnance wheel on press

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_ordnanceWheelUseOnRelease</code></summary>

use ordnance after selecting it

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_pingAlpha</code></summary>



default: `"1.0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_pingDoubleTapEnemy</code></summary>



default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_pingWheelToggle</code></summary>

toggle ping wheel on press

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_showButtonHints</code></summary>



default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_showCallsigns</code></summary>



default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_showLevelUp</code></summary>



default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_showMedals</code></summary>



default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_showMeter</code></summary>



default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_showObituary</code></summary>



default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_showTips</code></summary>



default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_showWeaponFlyouts</code></summary>



default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hud_setting_streamerMode</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hudchat_new_message_fade_duration</code></summary>

How long messages added to the text chat will take to fade from opaque to not visible

default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>hudchat_new_message_shown_duration</code></summary>

How long messages added to the text chat stick around with the panel not focused

default: `"12"`  
flags: `0x2`  
</details>
<details>
<summary><code>hudchat_play_text_to_speech</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hudchat_transition_message_mode_fade_duration</code></summary>

When switching message mode of the text chat panel how long it takes to transition visibility

default: `"0.25"`  
flags: `0x2`  
</details>
<details>
<summary><code>hudchat_visibility</code></summary>



default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>hudwarp_chopsize</code></summary>

Number of pixels to a primitive before chopping for warping.

default: `"60.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>hudwarp_override</code></summary>

Use convar settings for hud warp (instead of script-provided settings)

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>hudwarp_viewDist</code></summary>

Distance back from sphere center to use when 2d projecting.

default: `"1.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>hudwarp_xScale</code></summary>

Final scale for X (after projecting sphere surface to 2d.)

default: `"1.2"`  
flags: `0x4000`  
</details>
<details>
<summary><code>hudwarp_xWarp</code></summary>

Degrees of arc of sphere to use (0-90, low distortion to high.)

default: `"45.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>hudwarp_yScale</code></summary>

Final scale for Y (after projecting sphere surface to 2d.)

default: `"1.1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>hudwarp_yWarp</code></summary>

Degrees of arc for Y warp (0-90, low distortion to high.)

default: `"30.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>idcolor_ally</code></summary>



default: `"0.34 0.59 0.86 3"`  
flags: `0x2`  
</details>
<details>
<summary><code>idcolor_ally_cb1</code></summary>



default: `"0.24 0.50 0.96 3"`  
flags: `0x2`  
</details>
<details>
<summary><code>idcolor_ally_cb2</code></summary>



default: `"0.0 0.58 0.77 3"`  
flags: `0x2`  
</details>
<details>
<summary><code>idcolor_ally_cb3</code></summary>



default: `"0.28 0.52 0.97 3"`  
flags: `0x2`  
</details>
<details>
<summary><code>idcolor_enemy</code></summary>



default: `"0.8 0.25 0.15 3"`  
flags: `0x2`  
</details>
<details>
<summary><code>idcolor_enemy_cb1</code></summary>



default: `"0.89 0.78 0.0 3"`  
flags: `0x2`  
</details>
<details>
<summary><code>idcolor_enemy_cb2</code></summary>



default: `"1.0 0.627 0.68 3"`  
flags: `0x2`  
</details>
<details>
<summary><code>idcolor_enemy_cb3</code></summary>



default: `"0.82 0.74 0.06 3"`  
flags: `0x2`  
</details>
<details>
<summary><code>idcolor_neutral</code></summary>



default: `"1.0 1.0 1.0 0.6"`  
flags: `0x2`  
</details>
<details>
<summary><code>ik_debug</code></summary>

Enables debug lines for IK

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>ik_debug_chain</code></summary>

Allows specifying a single IK chain name for IK debugging

default: `""`  
flags: `0x2002`  
</details>
<details>
<summary><code>ik_debug_ent</code></summary>

Allows specifying a single entity for IK debugging

default: `""`  
flags: `0x2002`  
</details>
<details>
<summary><code>ik_debug_text</code></summary>

Enables IK debug text; requires ik_debug

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>ik_enable</code></summary>

Enables IK

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>ik_enable_client</code></summary>

Enables IK on the client

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>ik_height_adjust</code></summary>

Enable ik height adjustment

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>ik_height_adjust_debug</code></summary>

Debugging for ik height adjustment

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>ik_height_adjust_move_speed</code></summary>

IK height adjustment speed per unit of horizontal velocity in units per second

default: `"2.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>ik_height_adjust_sine</code></summary>

Test ik height adjustment with a sine wave

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>ik_height_adjust_speed</code></summary>

IK height adjustment speed as a fraction of step size per second

default: `"2.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>ik_latch</code></summary>

Enables IK latching to ground during footsteps

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>ik_normal_lerp_rate</code></summary>

Rate at which feet adjust to a new ground orientation in angles per second

default: `"100"`  
flags: `0x2002`  
</details>
<details>
<summary><code>ik_unlatch_max_rate</code></summary>

Maximum rate an IK'd bone can unlatch; prevents pop on animation transition

default: `"5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>ime_enabled</code></summary>

Enabled the IME

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>imgui_buildmode</code></summary>

Show the imgui implementation of the Build Mode dialog

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>imgui_buildmode</code></summary>

Show the imgui implementation of the Build Mode dialog

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>impact_allow</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>impact_debug_info</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>impact_victim_offset_dist</code></summary>

Distance to offset impact sounds from the victim, when requested

default: `"256"`  
flags: `0x2`  
</details>
<details>
<summary><code>impulse_low_decel_duration_scalar</code></summary>

Impulse magnitude is multiplied by this to give a length of time that the player can't decelerate

default: `"0.003"`  
flags: `0x2002`  
</details>
<details>
<summary><code>inPartyChat</code></summary>



default: `"0"`  
flags: `0x200`  
</details>
<details>
<summary><code>in_forceuser</code></summary>

Force user input to this split screen player.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>in_syncRT</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>in_usekeyboardsampletime</code></summary>

Use keyboard sample time smoothing.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>inbox_enabled</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>infoblock_requestInterval</code></summary>

Time between info block requests

default: `"300"`  
flags: `0x2`  
</details>
<details>
<summary><code>intro_viewed</code></summary>

Whether the introduction video has been viewed by this player

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>ip</code></summary>

Overrides IP for multihomed hosts

default: `"localhost"`  
flags: `0x80000`  
</details>
<details>
<summary><code>joy_advaxisr</code></summary>



default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>joy_advaxisu</code></summary>



default: `"4"`  
flags: `0x2`  
</details>
<details>
<summary><code>joy_advaxisv</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>joy_advaxisx</code></summary>



default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>joy_advaxisy</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>joy_advaxisz</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>joy_inverty</code></summary>

Whether to invert the Y axis of the joystick for looking.

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>joy_legacy</code></summary>

Turn on/off 'Legacy' mapping for control sticks.

default: `"0"`  
flags: `0x40000000`  
</details>
<details>
<summary><code>joy_movement_stick</code></summary>

Which stick controls movement (0 is left stick)

default: `"0"`  
flags: `0x40000000`  
</details>
<details>
<summary><code>joy_requireFocus</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>joy_rumble</code></summary>

Controller rumble.

default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>joy_xcontroller_cfg_loaded</code></summary>

If 0, the 360controller.cfg file will be executed on startup & option changes.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>jpeg_quality</code></summary>

jpeg screenshot quality.

default: `"90"`  
flags: `0x2`  
</details>
<details>
<summary><code>jt_help_with_anything_ignore_preference</code></summary>

This let's JT_HelpWithAnything() work on tasks that are not preferred.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>jump_graceperiod</code></summary>

Extra time during which a player can jump after falling off a ledge

default: `"0.2"`  
flags: `0x2002`  
</details>
<details>
<summary><code>jump_keyboardgrace_max</code></summary>

Amount of velocity change allowed during jump_keyboardgraceperiod, as a fraction of sprinting speed

default: `"0.7"`  
flags: `0x2002`  
</details>
<details>
<summary><code>jump_keyboardgrace_strength</code></summary>

Fraction of change toward the new direction when pressing a direction during jump_keyboardgraceperiod

default: `"0.7"`  
flags: `0x2002`  
</details>
<details>
<summary><code>jump_keyboardgraceperiodmax</code></summary>

Extra time during which a player can change their direction with keyboard input after jumping (fades to 0 strength at this time)

default: `"0.5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>jump_keyboardgraceperiodmin</code></summary>

Extra time during which a player can change their direction with keyboard input after jumping (at full strength)

default: `"0.2"`  
flags: `0x2002`  
</details>
<details>
<summary><code>killReplay_lagCompensate</code></summary>

Adjust player timing to try to match what the client saw rather than what the server saw.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>killReplay_playNonReplayRemoteCallsOnLocalClientPlayer</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>leaf_threadedRecompute</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>leaf_threadedRecompute_batchSize</code></summary>



default: `"12"`  
flags: `0x2`  
</details>
<details>
<summary><code>leech_npc_angle_cos</code></summary>

Cos(angle) allowed for leeching npcs

default: `"-1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>lerp_careAboutAttachmentBonePosition</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>lerp_debugEnt</code></summary>



default: `"-2"`  
flags: `0x2002`  
</details>
<details>
<summary><code>lerp_opt</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>lerp_threaded</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>lerp_threaded_numEntsPerTask</code></summary>



default: `"6"`  
flags: `0x2`  
</details>
<details>
<summary><code>light_maxcone</code></summary>

Max light cone limit.  Cone limit is half angle in degrees.

default: `"85"`  
flags: `0x2`  
</details>
<details>
<summary><code>lightmap_realtimelight</code></summary>

If true use the real-time light lightmap for selecting real-time lights.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>lightmap_realtimeshadows</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>load_during_video</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>loaderrorsCount</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>loaderrorsNeedShown</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>localClientPlayerCachedLevel</code></summary>



default: `"1"`  
flags: `0x1000010`  
</details>
<details>
<summary><code>locationInfo</code></summary>

What OS(on PC and Durango) or PSN account(on PS4) reports as the user's location

default: `""`  
flags: `0x210`  
</details>
<details>
<summary><code>locationInfo_nucleus</code></summary>

What origin(on PC) or nucleus(on console) reports as the user's location

default: `""`  
flags: `0x210`  
</details>
<details>
<summary><code>locator_background_border_color</code></summary>

The default color for the border.

default: `"255 255 255 15"`  
flags: `0x2`  
</details>
<details>
<summary><code>locator_background_border_thickness</code></summary>

How many pixels the background borders the left and right.

default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>locator_background_color</code></summary>

The default color for the background.

default: `"255 255 255 5"`  
flags: `0x2`  
</details>
<details>
<summary><code>locator_background_shift_x</code></summary>

How many pixels the background is shifted right.

default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>locator_background_shift_y</code></summary>

How many pixels the background is shifted down.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>locator_background_style</code></summary>

Setting this to 1 will show rectangle backgrounds behind the items word-bubble pointers.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>locator_background_thickness_x</code></summary>

How many pixels the background borders the left and right.

default: `"8"`  
flags: `0x2`  
</details>
<details>
<summary><code>locator_background_thickness_y</code></summary>

How many pixels the background borders the top and bottom.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>locator_fade_time</code></summary>

Number of seconds it takes for a lesson to fully fade in/out.

default: `"0.3"`  
flags: `0x2`  
</details>
<details>
<summary><code>locator_icon_max_size_non_ss</code></summary>

Minimum scale of the icon on the screen

default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>locator_icon_min_size_non_ss</code></summary>

Minimum scale of the icon on the screen

default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>locator_lerp_rest</code></summary>

Number of seconds before moving from the center.

default: `"2.25f"`  
flags: `0x2`  
</details>
<details>
<summary><code>locator_lerp_speed</code></summary>

Speed that static lessons move along the Y axis.

default: `"5.0f"`  
flags: `0x2`  
</details>
<details>
<summary><code>locator_lerp_time</code></summary>

Number of seconds to lerp before reaching final destination

default: `"1.75f"`  
flags: `0x2`  
</details>
<details>
<summary><code>locator_pulse_time</code></summary>

Number of seconds to pulse after changing icon or position

default: `"1.0f"`  
flags: `0x2`  
</details>
<details>
<summary><code>locator_split_len</code></summary>



default: `"0.5f"`  
flags: `0x4000`  
</details>
<details>
<summary><code>locator_split_maxwide_percent</code></summary>



default: `"0.80f"`  
flags: `0x4000`  
</details>
<details>
<summary><code>locator_start_at_crosshair</code></summary>

Start position at the crosshair instead of the top middle of the screen.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>locator_target_offset_x</code></summary>

How many pixels to offset the locator from the target position.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>locator_target_offset_y</code></summary>

How many pixels to offset the locator from the target position.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>locator_topdown_style</code></summary>

Topdown games set this to handle distance and offscreen location differently.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>lookspring</code></summary>



default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>lookstrafe</code></summary>



default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>m_acceleration</code></summary>

Mouse acceleration.

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>m_forward</code></summary>

Mouse forward factor.

default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>m_invert_pitch</code></summary>

Whether to invert the pitch axis of the mouse.

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>m_side</code></summary>

Mouse side factor.

default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mainmenu_background_movie</code></summary>



default: `"media/frontend.bik"`  
flags: `0x2`  
</details>
<details>
<summary><code>map_settings_override</code></summary>

If this is enabled then the following ConVars will be functional and override the maps current value: fog_enable, mat_bloomscale

default: `"0"`  
flags: `0x40004002`  
</details>
<details>
<summary><code>mat_autoexposure_compensation</code></summary>

This works like exposure compensation on a camera, in EV units. 0EV is no compensation, -1EV gives half the light, +2EV gives 4x the light, etc. The exposure range is still subject to the min/max, so you might want to use mat_autoexposure_uncap 1.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_autoexposure_force_value</code></summary>



default: `"0.0"`  
flags: `0x40004000`  
</details>
<details>
<summary><code>mat_autoexposure_max</code></summary>



default: `"6"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_autoexposure_max_multiplier</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_autoexposure_min</code></summary>



default: `"0.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_autoexposure_min_multiplier</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_autoexposure_speed</code></summary>

Changes the speed at which exposure adapts to changes in scene luminance.

default: `"0.1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_autoexposure_uncap</code></summary>

mat_autoexposure_min and mat_autoexposure_max are ignored when this is set.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_bloom_cutoff</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_bloom_max_lighting_value</code></summary>



default: `"5.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_bloom_scalefactor_scalar</code></summary>



default: `"0.1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>mat_bloom_streak_amount</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_bloom_streak_cutoff</code></summary>



default: `"5.0f"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_bloom_streak_cutoff_exposure_adapt</code></summary>

Whether streak cutoff value should scale with exposure values. R2 behavior is 0.0, R5 behavior is 1.0

default: `"0.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_bloom_streak_exponent_post</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_bloom_streak_exponent_pre</code></summary>



default: `"1.0f"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_bloom_wide_amount</code></summary>



default: `"1.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_bloom_wide_exponent_pre</code></summary>



default: `"1.5f"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_bloomamount_rate</code></summary>



default: `"0.05f"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_bloomscale</code></summary>

map_settings_override MUST BE ENABLED FOR THIS TO BE FUNCTIONAL.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_checkStalls</code></summary>

If true, flushes then syncs the render thread to the GPU at various spots of code to find hidden GPU stalls.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_cloudmask</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_colcorrection_disableentities</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_colcorrection_disableentities</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_colcorrection_disableentities</code></summary>

Disable map color-correction entities

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_colcorrection_editor</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_colcorrection_editor</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_colcorrection_forceentitiesclientside</code></summary>

Forces color correction entities to be updated on the client

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_colorcorrection</code></summary>



default: `"1"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mat_debug_postprocess_allowed</code></summary>

Allow postprocessing when debug views are enabled.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_debug_postprocessing_effects</code></summary>

0 = off, 1 = show post-processing in top left corner of screen

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_debug_tonemapping</code></summary>



default: `"4"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_debug_tonemapping_disable</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_debug_tonemapping_mid1</code></summary>



default: `"10.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_debug_tonemapping_mid2</code></summary>



default: `"1.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_debug_tonemapping_shoulder</code></summary>



default: `"0.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_debug_tonemapping_toe</code></summary>



default: `"0.3"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_debugalttab</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_depthbias_decal</code></summary>

use integer value

default: `"-16"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mat_depthbias_normal</code></summary>

use integer value

default: `"0"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mat_depthbias_shadowmap</code></summary>

use integer value

default: `"0"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mat_depthbias_tightshadowmap</code></summary>

use integer value. effective on View model selfshadow

default: `"10000"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mat_depthbias_ui</code></summary>

use integer value

default: `"-50"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mat_depthbias_zfill</code></summary>

use integer value

default: `"16"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mat_depthbiasclamp_decal</code></summary>



default: `"-0.001"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mat_depthbiasclamp_normal</code></summary>



default: `"0"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mat_depthbiasclamp_shadowmap</code></summary>



default: `"0"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mat_depthbiasclamp_ui</code></summary>



default: `"-0.001"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mat_depthbiasclamp_zfill</code></summary>



default: `"0"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mat_depthtest_force_disabled</code></summary>

only works on PC and XB1 for now

default: `"0"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mat_detail_tex</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_diffuse</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_disable_bloom</code></summary>



default: `"0"`  
flags: `0x40000002`  
</details>
<details>
<summary><code>mat_disable_lightmap_ambient</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_disable_lightmaps</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_disable_model_ambient</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_drawMenuGrid</code></summary>

Enable menu grid guide overlay. Only accurate for 16:9 aspect ratio.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_drawTitleSafe</code></summary>

Enable title safe overlay

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_drawflat</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_dxlevel</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_dynamic_tonemapping</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_dynamic_tonemapping</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_enable_ssr</code></summary>

NOTE - UNABLE TO ENABLE - Toggle Screen Space Reflections.
If you want to use SSR again, uncomment the line with (1u << MTLENVOPT_SSR) in shader.cpp of bakery and then rebuild shaders.

default: `"0"`  
flags: `0x2`  
max value: `1`  
</details>
<details>
<summary><code>mat_envmap_scale</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_envmaptgasize</code></summary>

Final envmap size for "envmap" console command; should be <= 128.

default: `"CUBEMAP_SCREENSHOT_RES"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_fastnobump</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_fastspecular</code></summary>

Enable/Disable specularity for visual testing.  Will not reload materials and will not affect perf.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_filterlightmaps</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_filtertextures</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_force_bloom</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_forceaniso</code></summary>



default: `"2"`  
flags: `0x40000000`  
min value: `0`  
max value: `16`  
</details>
<details>
<summary><code>mat_frame_color_bias</code></summary>

Add a constant value to the average frame color.

default: `"0.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_frame_color_enabled</code></summary>

Update the average frame color each frame.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_frame_color_scale</code></summary>

Scale the average frame color.

default: `"5.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_frame_color_spot_metering_screen_ratio</code></summary>

Use a percentage of the screen around the center to compute the average frame color.

default: `"0.8"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_fullbright</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_fxaa_enable</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_global_lighting</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_global_lighting</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_global_lighting</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_hdr_level</code></summary>

Set to 0 for no HDR, 1 for LDR+bloom on HDR maps, and 2 for full HDR on HDR maps.

default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_hdrcolcorrection_editor</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_hdrcolorcorrection</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_hide_sun_in_last_cascade</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_instancing</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_letterbox_aspect_goal</code></summary>

Letterbox when the window aspect ratio is below this threshold

default: `"1.6"`  
flags: `0x80000`  
</details>
<details>
<summary><code>mat_letterbox_aspect_threshold</code></summary>

Letterbox when the window aspect ratio is below this threshold

default: `"1.59"`  
flags: `0x80000`  
</details>
<details>
<summary><code>mat_lightcull_subview</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_lightcull_subviews</code></summary>

Re-cull lighting for subviews (monitors etc.)

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_local_contrast_edge_scale_override</code></summary>



default: `"-1000.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_local_contrast_midtone_mask_override</code></summary>



default: `"-1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_local_contrast_scale_override</code></summary>



default: `"0.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_local_contrast_vignette_end_override</code></summary>



default: `"-1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_local_contrast_vignette_start_override</code></summary>



default: `"-1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_character_0</code></summary>



default: `"0 0 0 0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_character_1</code></summary>



default: `"0 0 1 1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_character_2</code></summary>



default: `"1 0 1 1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_character_3</code></summary>



default: `"1 1 2 2"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_character_4</code></summary>



default: `"3 3 3 3"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_cockpit_0</code></summary>



default: `"0 0 0 0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_cockpit_1</code></summary>



default: `"0 0 0 0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_cockpit_2</code></summary>



default: `"0 0 0 0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_cockpit_3</code></summary>



default: `"1 0 0 1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_cockpit_4</code></summary>



default: `"3 3 3 3"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_model_0</code></summary>



default: `"0 0 0 0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_model_1</code></summary>



default: `"1 0 1 1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_model_2</code></summary>



default: `"1 1 1 1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_model_3</code></summary>



default: `"1 1 2 2"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_model_4</code></summary>



default: `"3 3 3 3"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_other_0</code></summary>



default: `"0 0 0 0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_other_1</code></summary>



default: `"1 1 1 1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_other_2</code></summary>



default: `"1 1 1 1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_other_3</code></summary>



default: `"1 1 1 1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_other_4</code></summary>



default: `"3 3 3 3"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_world_0</code></summary>



default: `"0 0 0 0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_world_1</code></summary>



default: `"0 0 1 1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_world_2</code></summary>



default: `"0 1 1 1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_world_3</code></summary>



default: `"1 1 2 2"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_materialmip_world_4</code></summary>



default: `"3 3 3 3"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_maxframelatency</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_mip_linear</code></summary>



default: `"1"`  
flags: `0x40000000`  
</details>
<details>
<summary><code>mat_mipmaptextures</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_norendering</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_norendering</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_phong</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_picmip</code></summary>



default: `"0"`  
flags: `0x40000000`  
min value: `0`  
max value: `4`  
</details>
<details>
<summary><code>mat_postprocess_enable</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_postprocess_enable</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_processtoolvars</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_proxy</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_reducefillrate</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_reduceparticles</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_remoteshadercompile</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_report_queue_status</code></summary>



default: `"0"`  
flags: `0x800002`  
</details>
<details>
<summary><code>mat_reversedepth</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_screen_blur_enabled</code></summary>

Enables screen blur render step

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_screen_blur_override</code></summary>



default: `"-1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_shadowstate</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_sharpen_amount</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_sharpen_threshold</code></summary>



default: `"0.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_sharpen_width</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_show_texture_memory_usage</code></summary>

Display the texture memory usage on the HUD.

default: `"0"`  
flags: `0x5000`  
</details>
<details>
<summary><code>mat_showenvmapmask</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_showlowresimage</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_showmiplevels</code></summary>

color-code miplevels 2: normalmaps, 1: everything else

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_skipid</code></summary>

Don't draw a particular mesh id. Helps track down which mesh you care about.

default: `"-1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_sky_color</code></summary>

forces the color of sky ambient; the alpha value of 0 means no override.

default: `"0.0 0.0 0.0 0.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_sky_scale</code></summary>

scales all sky ambient light by a constant factor

default: `"1.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_slopescaledepthbias_decal</code></summary>



default: `"-4"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mat_slopescaledepthbias_normal</code></summary>



default: `"0.0f"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mat_slopescaledepthbias_shadowmap</code></summary>



default: `"2"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mat_slopescaledepthbias_ui</code></summary>



default: `"-1.7"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mat_slopescaledepthbias_zfill</code></summary>



default: `"2"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mat_sun_color</code></summary>

forces the color of the sun directional light; the alpha value of 0 means no override.

default: `"0.0 0.0 0.0 0.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_sun_scale</code></summary>

scales all sun direct light by a constant factor

default: `"1.0"`  
flags: `0x40004000`  
</details>
<details>
<summary><code>mat_surfacefilter</code></summary>

If set, limits surfaces shown by mat_surfaceid and mat_surfacemat to those containing the substring.

default: `""`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_surfaceid</code></summary>

Draws the index of world surfaces. Can be filtered with mat_surfacefilter.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_surfacemat</code></summary>

Draws the material name of world surfaces. Can be filtered with mat_surfacefilter.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_syncGPU</code></summary>

If true, syncs the render thread to the GPU at the end of each frame, instead of letting the render thread get one frame ahead.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_syncInterval</code></summary>

Number of frames to skip per sync. 0 = novsync, 1 = 60 fps, 2 = 30, 3 == 20, 4 = 15, etc.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_sync_rt</code></summary>

Sync the render thread after each queued call. This is really slow, but makes debugging much easier.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_sync_rt_flushes_gpu</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_texture_list</code></summary>

For debugging, show a list of used textures per frame

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_texture_list_view</code></summary>

If this is nonzero, then the texture list panel will render thumbnails of currently-loaded textures.

default: `"1"`  
flags: `0x1002`  
</details>
<details>
<summary><code>mat_translucency_errors</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_use_compressed_hdr_textures</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_vignette_enable</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mat_warn_texture_convert</code></summary>

Print warnings for textures that had to be converted at load time, slowing down loads. 0 = off, 1 = old size not smaller, 2 = any change

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>match_backingOutMaxTimeToWait</code></summary>



default: `"60"`  
flags: `0x2`  
</details>
<details>
<summary><code>match_backoutslow</code></summary>

Forces empty server queries (for backing out of a lobby) to take this long

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>match_connect</code></summary>

If set to 0, we won't actually connect to any matchmaking results we get back

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>match_defaultMap_party</code></summary>

Default map to load if the dedicated server is empty

default: `"mp_lobby"`  
flags: `0x2`  
</details>
<details>
<summary><code>match_dir</code></summary>

What dir to look in for the matchmaking scripts

default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>match_dumpSearchResults</code></summary>

Dumps search result text to the console

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>match_emptyUpdateRate</code></summary>



default: `"30"`  
flags: `0x2`  
</details>
<details>
<summary><code>match_enabled</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>match_fakePort</code></summary>

Lie about our port number (so players can't connect)

default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>match_fakeS2SPort</code></summary>

Lie about our s2s port number (so servers can't connect)

default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>match_forceVerboseSearches</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>match_goodReputation</code></summary>



default: `"1"`  
flags: `0x80000202`  
</details>
<details>
<summary><code>match_maxPingsSent</code></summary>



default: `"50"`  
flags: `0x2`  
</details>
<details>
<summary><code>match_mixtape_unchecked</code></summary>



default: `""`  
flags: `0x1000000`  
</details>
<details>
<summary><code>match_mixtape_unchecked_version</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>match_mixtape_version</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>match_mixtape_warnOnPlay</code></summary>



default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>match_myBestDatacenter</code></summary>

Which datacenter we have the lowest ping to

default: `""`  
flags: `0x80080200`  
</details>
<details>
<summary><code>match_myDatacenter</code></summary>

Which datacenter we prefer (same as match_myBestDatacenter unless user changes it)

default: `""`  
flags: `0x80080200`  
</details>
<details>
<summary><code>match_myRankedDatacenter</code></summary>

Which datacenter we prefer for Ranked play (same as match_myBestDatacenter unless user changes it)

default: `""`  
flags: `0x80080200`  
</details>
<details>
<summary><code>match_myTeam</code></summary>



default: `"0"`  
flags: `0x200`  
</details>
<details>
<summary><code>match_partyChangeNum</code></summary>

The int that represents the change num of our party struct (did it change?)

default: `""`  
flags: `0x200`  
</details>
<details>
<summary><code>match_partySize</code></summary>

The size of our party

default: `""`  
flags: `0x200`  
</details>
<details>
<summary><code>match_partySub</code></summary>

The name of our party subscription

default: `""`  
flags: `0x200`  
</details>
<details>
<summary><code>match_pingWaveInterval</code></summary>



default: `"0.2"`  
flags: `0x2`  
</details>
<details>
<summary><code>match_playlist</code></summary>

The playlist we are looking for

default: `""`  
flags: `0x80200`  
</details>
<details>
<summary><code>match_precachemap</code></summary>

Whether to precache the map for the selected playlist

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>match_privateMatchListWithStryder</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>match_rankedMaxPing</code></summary>



default: `"200"`  
flags: `0x2002`  
</details>
<details>
<summary><code>match_rankedSwitchETA</code></summary>



default: `"300"`  
flags: `0x2002`  
</details>
<details>
<summary><code>match_resetPlaylistBetweenMatches</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>match_roleToken</code></summary>

The role token used when matchmaking (e.g. for private match).

default: `""`  
flags: `0x80200`  
</details>
<details>
<summary><code>match_searchInterval</code></summary>

How often to repeat searches

default: `"2"`  
flags: `0x80000`  
</details>
<details>
<summary><code>match_searching</code></summary>

Whether or not we want the system to be actively searching right now

default: `"0"`  
flags: `0x80200`  
</details>
<details>
<summary><code>match_teamNoFill</code></summary>

If set, matchmaking won't fill the player's team with non-party members

default: `"0"`  
flags: `0x80200`  
</details>
<details>
<summary><code>match_updateNotableRate</code></summary>



default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>match_updateRate</code></summary>



default: `"30"`  
flags: `0x2`  
</details>
<details>
<summary><code>match_useMatchmaking</code></summary>

This dedi is a matchmaking dedi

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>match_verbosePrintsInterval</code></summary>



default: `"60"`  
flags: `0x2`  
</details>
<details>
<summary><code>match_visiblePlaylists</code></summary>



default: `""`  
flags: `0x2002`  
</details>
<details>
<summary><code>matchmaking_hostname</code></summary>



default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>max_explosive_damage_mass</code></summary>

Anything heavier than this will be clamped. (units kg)

default: `"100"`  
flags: `0x2002`  
</details>
<details>
<summary><code>max_explosive_damage_velocity</code></summary>

inches/sec

default: `"200"`  
flags: `0x2002`  
</details>
<details>
<summary><code>max_tweak_shadow_updates</code></summary>



default: `"8"`  
flags: `0x2`  
</details>
<details>
<summary><code>melee_aim_assist_can_lock_pitch</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>melee_aim_assist_use_target_velocity</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>melee_attack_trace_can_use_lunge_distance</code></summary>



default: `"0.5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>melee_cone_trace_box_check</code></summary>



default: `"0.5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>melee_lunge_abort_distance</code></summary>

Abort the lunge if the distance moved in one frame is less than this much of the expected lunge distance.

default: `"0.25"`  
flags: `0x2002`  
</details>
<details>
<summary><code>melee_lunge_abort_if_blocked</code></summary>

Lunging can abort if the player hits something that blocks their lunge movement.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>melee_lunge_adjust_trace_distance</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>melee_lunge_align_eye_position</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>melee_lunge_dot_check</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>melee_lunge_force_enable_flying</code></summary>

Lunging will always ignore gravity.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>melee_lunge_lag_compensate_target</code></summary>

Lunging will apply lag compensation the target's position.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>melee_lunge_scale_by_speed</code></summary>

Increase lunge range (by up to the given scale) if the player is going fast enough.

default: `"2.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>melee_lunge_slide</code></summary>

When lunging, try slide along surfaces

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>melee_lunge_use_closest_distance_between_cylinders</code></summary>

When calculating distance to the lunge target, treat them as cylinders rather than points.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>melee_lunge_use_command_time</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>melee_queue_attack_anim_event</code></summary>

Run melee attacks after the player has moved this frame

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>mem_dumpstats</code></summary>

Dump current and max heap usage info to console at end of frame ( set to 2 for continuous output )


default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mem_force_flush</code></summary>

Force cache flush of unlocked resources on every alloc

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mem_force_flush_section</code></summary>

Cache section to restrict mem_force_flush

default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>mem_incremental_compact_rate</code></summary>

Rate at which to attempt internal heap compaction

default: `".5"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mem_level</code></summary>

Memory Level - Default: High

default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>mem_level</code></summary>

Memory Level - Default: High

default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>mem_max_heapsize</code></summary>

Maximum amount of memory to dedicate to engine hunk and datacache (in mb)

default: `"1024"`  
flags: `0x2`  
</details>
<details>
<summary><code>mem_max_heapsize_dedicated</code></summary>

Maximum amount of memory to dedicate to engine hunk and datacache, for dedicated server (in mb)

default: `"64"`  
flags: `0x2`  
</details>
<details>
<summary><code>mem_min_heapsize</code></summary>

Minimum amount of memory to dedicate to engine hunk and datacache (in mb)

default: `"48"`  
flags: `0x2`  
</details>
<details>
<summary><code>mem_runheapchecks</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mem_test_each_frame</code></summary>

Run heap check at end of every frame


default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mem_test_every_n_seconds</code></summary>

Run heap check at a specified interval


default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mem_test_quiet</code></summary>

Don't print stats when memtesting

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>menu_faq_community_version</code></summary>



default: `"-1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>menu_faq_patchnotes_version</code></summary>



default: `"-1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>menu_faq_viewed</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>menu_was_multiplayer_played_last</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>migrate_attempt_interval</code></summary>



default: `"5.0f"`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_actor_occlusion_radius</code></summary>

Distance which must be penetrated for one of the entity check points to be considered occluded.

default: `"8.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_channels</code></summary>

Number of audio channels, commonly 2(stereo), 6(5.1), 8(7.1). (0 is default)

default: `"0"`  
flags: `0x40000000`  
</details>
<details>
<summary><code>miles_flip_active_window_logic</code></summary>

Only hear audio when NOT the active window.

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>miles_force_emitter_environment</code></summary>

Force Environment on played sounds and entities (per-event controllers and suffixes.)

default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_force_listener_environment</code></summary>

Force environment on listener (i.e., global controller changes only)

default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_freeze</code></summary>

When 1, sound is paused and incoming play events are ignored.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_initial_occlusion_delay</code></summary>

Time (in msec) to delay new sounds when we defer their traces.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_language</code></summary>

Language to use for audio (requires a miles restart to change.)

default: `""`  
flags: `0x1000000`  
</details>
<details>
<summary><code>miles_listener_freeze</code></summary>

When 1, stop updating listener position.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_nonactor_occlusion</code></summary>

Do traces to determine when non-entity sounds are occluded.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_nonactor_occlusion_radius</code></summary>

Distance which must be penetrated for a non-entity sound to be considered occluded.

default: `"8.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_nopandist</code></summary>

Distance at which panning is forced to center-front.

default: `"10"`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_occlusion</code></summary>

When nonzero, perform occlusion checks

default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>miles_occlusion_force</code></summary>

0 to 100: Force all sounds to have occlusion values of 0 (unoccluded) to 100 (completely occluded). -1 for normal.

default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_occlusion_partial</code></summary>

When zero, occlusion state is binary. When nonzero, allow partial occlusion of audio.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_occlusion_use_reset_after_deferred_initial</code></summary>

For A/B testing feature. Enable permanently eventually.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_samplerate</code></summary>

Sample rate, commonly 48000, 44100, 22050, or 11025 (0 is default)

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_server_sounds_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_server_sounds_print</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_solo_ents</code></summary>

Only play sounds from this entity index (or space-separated list of indices.)

default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_soundscape_imgui</code></summary>

Show imgui-based soundscape debugging window

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_spatialize_front_degrees</code></summary>

Front panning field angle

default: `"45.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_spatialize_offplane_strength</code></summary>

Offplane omni-fication strength

default: `"0.7"`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_spatialize_on</code></summary>

Enable hard spatialization test

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_spatialize_rear_degrees</code></summary>

Rear panning field angle

default: `"120.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_suffixes</code></summary>

Use emitter suffixed versions of sounds.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>min_explosive_damage_mass</code></summary>

Anything lighter than this will be clamped. (units kg)

default: `"20"`  
flags: `0x2002`  
</details>
<details>
<summary><code>missile_default_speed</code></summary>



default: `"2500"`  
flags: `0x2002`  
</details>
<details>
<summary><code>missile_homing_speed</code></summary>



default: `"150"`  
flags: `0x2002`  
</details>
<details>
<summary><code>mod_check_vcollide</code></summary>

Check all vcollides on load

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mod_trace_load</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>model_defaultFadeDistMin</code></summary>

Default minimum fade distance.

default: `"400"`  
flags: `0x4000`  
</details>
<details>
<summary><code>model_defaultFadeDistMin</code></summary>

Default minimum fade distance.

default: `"400"`  
flags: `0x4000`  
</details>
<details>
<summary><code>model_defaultFadeDistScale</code></summary>

Factor that is multiplied by the model's radius to get the default fade distance.

default: `"40"`  
flags: `0x4000`  
</details>
<details>
<summary><code>model_defaultFadeDistScale</code></summary>

Factor that is multiplied by the model's radius to get the default fade distance.

default: `"40"`  
flags: `0x4000`  
</details>
<details>
<summary><code>model_fadeRangeFraction</code></summary>

Fraction of the fade distance to fade over.

default: `"0.1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>model_fadeRangeFractionNear</code></summary>

Fraction of the near fade distance at which impostors are invisible.

default: `"0.9"`  
flags: `0x4000`  
</details>
<details>
<summary><code>monitor_cc</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>monitor_mat_sharpen_amount</code></summary>



default: `"2.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>monitor_postfx</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>monitor_rui_world_enabled</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>monitor_snapshot_frame_delay</code></summary>



default: `"20"`  
flags: `0x2`  
</details>
<details>
<summary><code>monitor_zfar_default</code></summary>



default: `"642"`  
flags: `0x2`  
</details>
<details>
<summary><code>monitor_zfar_override</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>monitor_zfar_override_enabled</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>motd</code></summary>



default: `""`  
flags: `0x12`  
</details>
<details>
<summary><code>mouse_sensitivity</code></summary>

Mouse sensitivity.

default: `"5"`  
flags: `0x80`  
min value: `0.1`  
max value: `20`  
</details>
<details>
<summary><code>mouse_use_per_scope_sensitivity_scalars</code></summary>

Uses the per scope scalars

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>mouse_zoomed_sensitivity_scalar_0</code></summary>

Mouse sensitivity.

default: `"1.0"`  
flags: `0x80`  
min value: `0.1`  
max value: `20`  
</details>
<details>
<summary><code>mouse_zoomed_sensitivity_scalar_1</code></summary>

Mouse sensitivity.

default: `"1.0"`  
flags: `0x80`  
min value: `0.1`  
max value: `20`  
</details>
<details>
<summary><code>mouse_zoomed_sensitivity_scalar_2</code></summary>

Mouse sensitivity.

default: `"1.0"`  
flags: `0x80`  
min value: `0.1`  
max value: `20`  
</details>
<details>
<summary><code>mouse_zoomed_sensitivity_scalar_3</code></summary>

Mouse sensitivity.

default: `"1.0"`  
flags: `0x80`  
min value: `0.1`  
max value: `20`  
</details>
<details>
<summary><code>mouse_zoomed_sensitivity_scalar_4</code></summary>

Mouse sensitivity.

default: `"1.0"`  
flags: `0x80`  
min value: `0.1`  
max value: `20`  
</details>
<details>
<summary><code>mouse_zoomed_sensitivity_scalar_5</code></summary>

Mouse sensitivity.

default: `"1.0"`  
flags: `0x80`  
min value: `0.1`  
max value: `20`  
</details>
<details>
<summary><code>mouse_zoomed_sensitivity_scalar_6</code></summary>

Mouse sensitivity.

default: `"1.0"`  
flags: `0x80`  
min value: `0.1`  
max value: `20`  
</details>
<details>
<summary><code>mouse_zoomed_sensitivity_scalar_7</code></summary>

Mouse sensitivity.

default: `"1.0"`  
flags: `0x80`  
min value: `0.1`  
max value: `20`  
</details>
<details>
<summary><code>move_one_cmd_per_client_frame</code></summary>

Force clients to generate exactly one user command per client frame. There will not be a one-to-one relationship between cmds and ticks.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>movement_anim_downed_playback_maxrate</code></summary>



default: `"2.5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>movement_anim_playback_maxrate</code></summary>



default: `"10"`  
flags: `0x2002`  
</details>
<details>
<summary><code>movement_anim_playback_minrate</code></summary>



default: `"0.25"`  
flags: `0x2002`  
</details>
<details>
<summary><code>movement_anim_sprint_playback_maxrate</code></summary>



default: `"1.25"`  
flags: `0x2002`  
</details>
<details>
<summary><code>mp_accountLink_requestInterval</code></summary>



default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>mp_allowed</code></summary>



default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mp_bodyyawrate</code></summary>



default: `"400"`  
flags: `0x2002`  
</details>
<details>
<summary><code>mp_class_max_dronecontroller</code></summary>

Max allowed dronecontrollers.

default: `"-1"`  
flags: `0x2102`  
</details>
<details>
<summary><code>mp_class_max_fireteam</code></summary>

Max allowed fireteams.

default: `"-1"`  
flags: `0x2102`  
</details>
<details>
<summary><code>mp_class_max_pilot</code></summary>

Max allowed pilots.

default: `"-1"`  
flags: `0x2102`  
</details>
<details>
<summary><code>mp_class_max_titan</code></summary>

Max allowed titans.

default: `"-1"`  
flags: `0x2102`  
</details>
<details>
<summary><code>mp_countRRNobodyAsLobby</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mp_enablematchending</code></summary>

When set to 0, match will not end

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>mp_enabletimelimit</code></summary>

enable mp_timelimit timer in games

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>mp_gamemode</code></summary>

Current game mode name

default: `""`  
flags: `0x12002`  
</details>
<details>
<summary><code>mp_huge_threshhold</code></summary>

How many players we need before considering the server to be a 'huge' server

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mp_linkingAccountTime</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>mp_linkingAccountWindow</code></summary>



default: `"300"`  
flags: `0x2`  
</details>
<details>
<summary><code>mp_maxbodyyaw</code></summary>



default: `"60"`  
flags: `0x2002`  
</details>
<details>
<summary><code>mp_permission_requestInterval</code></summary>



default: `"30"`  
flags: `0x2`  
</details>
<details>
<summary><code>mp_permission_rerequestInterval</code></summary>



default: `"21600"`  
flags: `0x2`  
</details>
<details>
<summary><code>mp_player_level</code></summary>

To read mp player level in SP

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>mp_scaleAnimationSpeeds</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>mp_showgestureslots</code></summary>

Show multiplayer client/server gesture slot information for the specified player index (-1 for no one).

default: `"-1"`  
flags: `0x6002`  
</details>
<details>
<summary><code>mtx_svEdition</code></summary>



default: `"55"`  
flags: `0x2002`  
</details>
<details>
<summary><code>muteWeaponSounds</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>name</code></summary>

Current user name

default: `"unnamed"`  
flags: `0x480`  
</details>
<details>
<summary><code>net_RunInvalidatePhysics</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_async_sendto</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_autoUnthrottle</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_bandwidthPrintThreshold</code></summary>

Percentage where it's worth printing spam about this message in the bandwidth tracker prints

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_bindToSpecificAddress</code></summary>

Only bind to a certain interface

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>net_blockmsg</code></summary>

Discards incoming message: <0|1|name>

default: `"none"`  
flags: `0x4000`  
</details>
<details>
<summary><code>net_chatThroughChatserver</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_chokeloop</code></summary>

Apply bandwidth choke to loopback packets (only in MP)

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_clearReliableDataOnReset</code></summary>

Whether we should erase unsent reliable data when we call netchan->Reset()

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_client_side_weapon_animations</code></summary>

Enable/disable client side weapon animations. Only apply to already optimized weapons, eg. rapid fire instant hit weapons like xo16, r101 etc.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>net_compressDataBlock</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_compressLZValue</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_compresspackets</code></summary>

Use lz compression on game packets.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_compresspackets_minsize</code></summary>

Don't bother compressing packets below this size.

default: `"1000"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_connectPacketWarningThreshhold</code></summary>



default: `"0.9"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_connectingDataRate</code></summary>



default: `"128000"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_createUndoDeltas</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_data_block_enabled</code></summary>

Enable/disable net data block optimization for load times. When disabled large chunks are sent down via existing netchan reliability system instead of net data blocks.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>net_datablockPrintSummaries</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_datablock_fastRate</code></summary>



default: `"128000"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_datablock_longSendTime</code></summary>



default: `"10"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_datablock_minResendInterval</code></summary>



default: `"0.1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_datablock_networkLossForSlowSpeed</code></summary>



default: `"0.1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_datablock_resendRateForSlowSpeed</code></summary>



default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_datablock_slowRate</code></summary>



default: `"64000"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_debugDataBlockReceiver</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_debugDataBlockSender</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_debugLerping</code></summary>



default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_deltaFieldEntityBlockSize</code></summary>



default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_disconnectIfDeltaBufferIsFull</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_drawslider</code></summary>

Draw completion slider during signon

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_droppackets</code></summary>

Drops next n packets on client

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>net_dumpChangesPrecise</code></summary>

Prints floats at full precision

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_encrypt_copyCtx</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_encryptionDebug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_fakelag</code></summary>

Lag all incoming network data (including loopback) by this many milliseconds.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>net_fakelag_clientOnly</code></summary>

Fakelag won't affect the server, only clients

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>net_fakelagjitter</code></summary>

Jitter net_fakelag packet time

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_fakeloss</code></summary>

Simulate packet loss as a percentage (negative means drop 1/n packets)

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_forceDeltaBufferToOverflow</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_forceUnnecessaryUndoDeltas</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_forcetimeout</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>net_fullyConnectedDataRate</code></summary>



default: `"256000"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_highPacketLatencyThreshold</code></summary>



default: `"0.200"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_highPacketLossThreshold</code></summary>



default: `"0.05"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_ignoreAllSnapshots</code></summary>

Drop all snapshot messages

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_largeSnapshotThreshold</code></summary>

The size of a snapshot that qualifies as a large snapshot

default: `"15000"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_lerpFields</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_lowBandwidthConnect</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_maxAccumulatedClearTimeBalance</code></summary>

Max time (in seconds) to count not sending data to this player towards their 'remaining bandwidth' balance [if we haven't sent a packet in 2 minutes, that doesn't mean they have 2 minutes of bandwidth remaining to use]

default: `"0.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_maxcleartime</code></summary>

Max # of seconds we can wait for next packets to be sent based on rate setting (0 == no limit).

default: `"4.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_maxfilesize</code></summary>

Maximum allowed file size for uploading in MB

default: `"16"`  
flags: `0x2`  
min value: `0`  
max value: `64`  
</details>
<details>
<summary><code>net_maxfragments</code></summary>

Max fragment bytes per packet

default: `"1200"`  
flags: `0x2`  
min value: `256`  
max value: `1200`  
</details>
<details>
<summary><code>net_maxroutable</code></summary>

Requested max packet size before packets are 'split'.

default: `"1200"`  
flags: `0x202`  
min value: `576`  
max value: `1200`  
</details>
<details>
<summary><code>net_minConnectionTimeForSpam</code></summary>



default: `"10"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_minQueuedPacketsForPrint</code></summary>



default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_minResetIdleTimerInterval</code></summary>



default: `"10"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_minimumPacketLossDC</code></summary>

The lowest packet loss we have to any datacenter

default: `"100"`  
flags: `0x200`  
</details>
<details>
<summary><code>net_minroutable</code></summary>

Forces larger payloads.

default: `"16"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_noPostDataForDeletedEnts</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_old_seed_generation</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>net_optimize_persistent_data</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>net_optimize_playlists</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>net_optimize_weapons</code></summary>

Enable/disable bandwidth optimizations made to weapons. Additional experimental optimizations can be enabled values 2 (weapon player data) and 3 (client side weapon animation)

default: `"2"`  
flags: `0x2002`  
</details>
<details>
<summary><code>net_predictParentEntities</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_predictedEntsUseFirstAvailableSnapshot</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_predictionDebug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_pretendSnapshotArrayFull</code></summary>

Pretend the client snapshot array is full even when it isn't

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_printCompression</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_printOutOfSnapshots</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_printUnnecessaryDeltas</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_propSkipPrintThreshold</code></summary>

Show prop skips more than this many apart

default: `"1000"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_public_adr</code></summary>

For servers behind NAT/DHCP meant to be exposed to the public internet, this is the public facing ip address string: ("x.x.x.x" )

default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>net_queue_trace</code></summary>



default: `"0"`  
flags: `0x2000002`  
</details>
<details>
<summary><code>net_queuedPackets_PrintOversleeps</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_queuedPackets_SkipSmallSleeps</code></summary>



default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_queued_packet_sender_nopacket_sleep</code></summary>



default: `"10"`  
flags: `0x80000`  
</details>
<details>
<summary><code>net_queued_packet_thread</code></summary>

Use a high priority thread to send queued packets out instead of sending them each frame.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_recentNetworkGapWindow</code></summary>



default: `"2.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_recentNetworkGapsNeeded</code></summary>



default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_recreateScriptInstanceOnReplayTransition</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_recv_dumpChanges</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_recv_dumpNetworkedChangesOnEntCreate</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_recv_watchEnt</code></summary>



default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_recv_watchField1</code></summary>



default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>net_recv_watchField2</code></summary>



default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>net_resourcePrintMinimum</code></summary>

Minimum count for printing bandwidth info about a resource (sound, effect)

default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_sendFloatDeltas</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_sendProfileTotals</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_sendtoInJob</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_showFailedAuth</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_showLargeSnapshot</code></summary>

Show console spam when we get large snapshots from the server

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_showQueued</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_showUndoDeltas</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_showUserWarnings</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_showchoke</code></summary>

Show console spam when we get choked snapshots from the server


default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_showchokeInterval</code></summary>

The minimum time interval between spam about going above our network budget

default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_showdrop</code></summary>

Show dropped packets in console

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_showfragments</code></summary>

Show netchannel fragments

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_showmsg</code></summary>

Show incoming message: <0|1|name>

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_showpeaks</code></summary>

Show messages for large packets only: <size>

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_showsendrecv</code></summary>

Show sendto and recvfrom calls

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_showsplits</code></summary>

Show info about packet splits

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>net_showudp</code></summary>

Dump UDP packets summary to console

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>net_showudp_oob</code></summary>

Dump OOB UDP packets summary to console

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>net_showudp_remoteonly</code></summary>

Dump non-loopback udp only

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>net_showusercmd</code></summary>

Show user command encoding

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_skipUnnecessaryDeltas</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_splitrate</code></summary>

Number of fragments for a splitpacket that can be sent per frame

default: `"3"`  
flags: `0x80000`  
</details>
<details>
<summary><code>net_splitrateDefaultMP</code></summary>

Default MP number of fragments for a splitpacket that can be sent per frame

default: `"3"`  
flags: `0x80000`  
</details>
<details>
<summary><code>net_splitrateDefaultSP</code></summary>

Default SP number of fragments for a splitpacket that can be sent per frame

default: `"10000"`  
flags: `0x80000`  
</details>
<details>
<summary><code>net_tamperPackets</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_threadedEntityDeltas</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_threadedProcessPacket</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_timeoutUsesLastReadTime</code></summary>

Don't let us time out if we haven't been actually checking the socket for packets (inside a loop, for example)

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_trackerWarningInterval</code></summary>



default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_usesocketsforloopback</code></summary>

Use network sockets layer even for listen server local player's packets (multiplayer only).

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_verifyEncryption</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_voiceEchoFromChatServer</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_warnAboutSocketReadGaps</code></summary>

Warn if we are waiting longer than this to check a socket for new packets

default: `"0.200"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_warnGapTime</code></summary>



default: `"0.4"`  
flags: `0x2`  
</details>
<details>
<summary><code>net_wifi</code></summary>

0 = ethernet, 1 = wifi, -1 = unknown

default: `"-1"`  
flags: `0x80200`  
</details>
<details>
<summary><code>net_worldHitchSlopTime</code></summary>



default: `"0.031"`  
flags: `0x2`  
</details>
<details>
<summary><code>next</code></summary>

Set to 1 to advance to next frame ( when singlestep == 1 )

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>noReloadAfterUse</code></summary>

Disables reloads for "+useAndReload" input if a use is triggered.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>noise_filter_scale</code></summary>



default: `"0.006"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>not_focus_sleep</code></summary>

MS to sleep while window doesn't have focus

default: `"50"`  
flags: `0x2`  
</details>
<details>
<summary><code>notification_displayTime</code></summary>

How long notifications should wait before auto-hiding

default: `"10"`  
flags: `0x2`  
</details>
<details>
<summary><code>nucleus_id</code></summary>



default: `"0"`  
flags: `0x80000200`  
</details>
<details>
<summary><code>nucleus_pid</code></summary>



default: `"unknown"`  
flags: `0x80000200`  
</details>
<details>
<summary><code>number_shortenToMillionsAfter</code></summary>



default: `"2000000"`  
flags: `0x2`  
</details>
<details>
<summary><code>offhandTossOverheadPitchThreshold</code></summary>



default: `"-1.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>offhand_alignEndAnim1p3p</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>old_culling</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>old_gather_props</code></summary>



default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>one_handed_change_rate</code></summary>

The rate at which the transition to and from one handed weapon usage takes place

default: `"1.25"`  
flags: `0xa`  
</details>
<details>
<summary><code>openInvite_spam</code></summary>

Whether open invites should spam to the console log

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>openInvites_filterByLanguage</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>openInvites_filterByRegion</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>openinvite_duration_default</code></summary>



default: `"10"`  
flags: `0x2`  
</details>
<details>
<summary><code>ordnanceSwapSelectCooldown</code></summary>



default: `"0.25"`  
flags: `0x2`  
</details>
<details>
<summary><code>origin_Errorlevel_OldBehaviour</code></summary>

Enables Setting errorlevel for as in the old code base did.


default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>origin_Errorlevel_Telementry</code></summary>

Enables sending host Telemetry event for Origin errorLevel


default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>origin_authCodeFailureMaxBackoffSeconds</code></summary>



default: `"10"`  
flags: `0x2`  
</details>
<details>
<summary><code>origin_autoRefreshToken</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>origin_debug</code></summary>

Enable Origin HTTP debug logging (all HTTP queries and responses, token data etc.)

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>origin_disconnectWhenOffline</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>origin_ignoreInvitesOnLoadScreen</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>origin_igo_mutes_sound_enabled</code></summary>

Enables feature for optionally muting game sound when Origin overlays are launched.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>origin_igo_muting_sound</code></summary>

True if game sound was muted when launching an Origin overlay.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>origin_presense_updateRate</code></summary>

Minimum time between origin updates in seconds.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>origin_tokenFailureMaxBackoffSeconds</code></summary>



default: `"10"`  
flags: `0x2`  
</details>
<details>
<summary><code>panel_showVisChanges</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>panel_test_title_safe</code></summary>

Test vgui panel positioning with title safe indentation

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>parenting_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>particleEffect_checkShouldStillPlay</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_alwayswakeonstop</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>particle_cpu_level</code></summary>



default: `"0"`  
flags: `0x40000000`  
min value: `0`  
max value: `2`  
</details>
<details>
<summary><code>particle_delete_all_except</code></summary>



default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_dlights_enable</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_dlights_spew</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_gpu_level</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_lighting_clear_enable</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_lighting_size</code></summary>

The size of each particle in the atlas

default: `"32"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_lighting_viewmodel_enable</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_overlay</code></summary>

Show particle overlay (2 for same as particle_overlay_list_tally)

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_overlay_detail_attributes</code></summary>

Space separated list of attributes to show per particle - 'all id duration xyz prev_xyz radius color alpha length'

default: `"id"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_overlay_detail_filter</code></summary>

Filters which particles to see in detail - can be id or substring or *

default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_overlay_detail_list_particles</code></summary>

List individual particles in detail view

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_overlay_detail_scroll</code></summary>

Skip this many rows in particle overlay detail

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_overlay_hide_sleeping</code></summary>

Hide sleeping effects in particle overlay

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_overlay_list_filter</code></summary>

Filters which particles to see in list - can be id or substring or *

default: `"*"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_overlay_list_tally</code></summary>

Show tally of particle counts, rather than list (same as particle_overlay 2)

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_overlay_list_tally_collapse_children</code></summary>

Collapse children in tally-- only show totals at top level.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_overlay_old</code></summary>

Draw particle overlay the old way (no imgui)

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_overlay_scroll</code></summary>

Skip this many rows in particle overlay

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_remap_vol2cp_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_script_dump</code></summary>

particle_script_dump SCRIPT_HANDLE

default: `"-1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>particle_script_list</code></summary>



default: `""`  
flags: `0x2002`  
</details>
<details>
<summary><code>particle_script_log</code></summary>

particle_script_log SCRIPT_HANDLE

default: `"-1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>particle_scrub_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_scrub_debug_effect</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_scrub_is_using_time_scrub</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>particle_scrub_max_dt</code></summary>



default: `"0.02"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_scrub_play_speed</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_scrub_quality</code></summary>



default: `"6"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_scrub_time</code></summary>



default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>particle_simulateoverflow</code></summary>

Used for stress-testing particle systems. Randomly denies creation of particles.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>particles_cull_dlights</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>particles_max_passes</code></summary>



default: `"10"`  
flags: `0x2002`  
</details>
<details>
<summary><code>particles_spawncull</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>particles_spawncull_report</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>parties_alwaysReadSubs</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>party_autoCreatePartyAlways</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>party_autoCreatePartyDelay</code></summary>



default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>party_color_enabled</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>party_doRealNameLookups</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>party_doRealNameLookupsForOwner</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>party_hostname</code></summary>



default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>party_httpHandleTimeout</code></summary>



default: `"10.0f"`  
flags: `0x2`  
</details>
<details>
<summary><code>party_keepAliveTime</code></summary>

How often party clients should send a keepalive packet

default: `"60"`  
flags: `0x2`  
</details>
<details>
<summary><code>party_keepAliveTime</code></summary>

How often party clients should send a keepalive packet

default: `"60"`  
flags: `0x2`  
</details>
<details>
<summary><code>party_leaderAlwaysDetectsChanges</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>party_leaveMatchOnJoin</code></summary>

Whether a player should quit the match they're in when they join a party

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>party_lookupRealNamesForOpenInvites</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>party_lookupRealNamesForOpenInvitesForOwner</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>party_minSize</code></summary>



default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>party_privacy</code></summary>

our privacy setting for parties

default: `"open"`  
flags: `0x2`  
</details>
<details>
<summary><code>party_readyToSearch</code></summary>

our ready-up status

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>party_relyOnPartyForMemberUserInfo</code></summary>

If true, we won't re-request userinfo speculatively, only when their version changes in our party block

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>party_requireConsensusForSearch</code></summary>

Whether everyone in the party has to ready up before finding a match

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>perTriangleCollisionForced</code></summary>

Forces all traces on static models to use high detail traces.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>persistenceDef_hostname</code></summary>



default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>persistenceDef_queryMaxHttpRetries</code></summary>



default: `"4"`  
flags: `0x6`  
</details>
<details>
<summary><code>persistenceDef_readMaxHttpRetries</code></summary>



default: `"2"`  
flags: `0x6`  
</details>
<details>
<summary><code>persistenceDef_retryReadAfterErrorTime</code></summary>



default: `"15"`  
flags: `0x6`  
</details>
<details>
<summary><code>persistenceDef_writeMaxHttpRetries</code></summary>



default: `"4"`  
flags: `0x6`  
</details>
<details>
<summary><code>persistence_clForceNew</code></summary>



default: `"0"`  
flags: `0x200`  
</details>
<details>
<summary><code>persistence_disableForBuildProcess</code></summary>



default: `"0"`  
flags: `0x6`  
</details>
<details>
<summary><code>persistence_enforce_manifest</code></summary>

Enable validating against manifest.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>persistence_hostname</code></summary>



default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>persistence_new_player_if_upgrade_fails</code></summary>

Create a new player if upgrade fails. (dev only)

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>persistence_upload_def</code></summary>



default: `"1"`  
flags: `0x6`  
</details>
<details>
<summary><code>persistence_upload_failure_is_error</code></summary>



default: `"1"`  
flags: `0x6`  
</details>
<details>
<summary><code>persistent_warningRate</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>pertrianglecollision</code></summary>

Enables per-triangle collision with TRACEDETAILLEVEL_HIGH (i.e., bullets) on static models.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>phys_bounce</code></summary>



default: `"0.2"`  
flags: `0x2002`  
</details>
<details>
<summary><code>phys_cfm</code></summary>

Constraint Force Mixing value. Softens the force applied to resolve constraints. ode.org/ode-latest-userguide.html: "If CFM is set to zero, the constraint will be hard .... the constraint is allowed to be violated by an amount proportional to CFM times the restoring force that is needed to enforce the constraint"

default: `"0.0001"`  
flags: `0x2002`  
</details>
<details>
<summary><code>phys_cfm_anglejointstop</code></summary>



default: `"0.0001"`  
flags: `0x2002`  
</details>
<details>
<summary><code>phys_drawContacts</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>phys_drawContactsDuration</code></summary>



default: `"0.016666"`  
flags: `0x2002`  
</details>
<details>
<summary><code>phys_drawGeoms</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>phys_drawTunnelChecks</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>phys_enableObjectPairCollidePrototype</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>phys_erp</code></summary>

Fraction of penetration that physics tries to resolve per time step. At 1.0, all contacts add a velocity that will end the penetration in a single frame, though this is unstable. At 0.0, contacts create no outward force (though they still provide friction).

default: `"0.05"`  
flags: `0x2002`  
</details>
<details>
<summary><code>phys_erp_anglejointstop</code></summary>



default: `"0.05"`  
flags: `0x2002`  
</details>
<details>
<summary><code>phys_frictionDefault</code></summary>



default: `"0.82"`  
flags: `0x2002`  
</details>
<details>
<summary><code>phys_showObjectCount</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>phys_threadGoWide</code></summary>

Go wide across threads with Physics.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>physics_async_cl</code></summary>

Run physics simulation asynchronously from the main thread.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>physics_autoSleepAngularThreshold</code></summary>

Angular speed below which a physic object goes to sleep. (in degrees / second)

default: `"120"`  
flags: `0x2002`  
</details>
<details>
<summary><code>physics_autoSleepDebug</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>physics_autoSleepGroundHysteresis</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>physics_autoSleepSpeedThreshold</code></summary>

Speed below which a physic object goes to sleep.

default: `"20"`  
flags: `0x2002`  
</details>
<details>
<summary><code>physics_collideWithMovingGeo</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>physics_defaultMaxAngularSpeed</code></summary>



default: `"10000"`  
flags: `0x2002`  
</details>
<details>
<summary><code>physics_defaultMaxSpeed</code></summary>



default: `"10000"`  
flags: `0x2002`  
</details>
<details>
<summary><code>physics_scaled_mem</code></summary>

Amout of extra memory taken by scaled collision meshes

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>physics_tunnelChecks</code></summary>

Do traces to prevent physics objects from falling through the world.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>physics_tunnelChecksForceAlways</code></summary>

Require objects to do tunnel checks every frame.

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>pin_opt_in</code></summary>

Enables sending PIN telemetry data to EA

default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>pin_plat_id</code></summary>

Platform user id for PIN

default: `"0"`  
flags: `0x80000202`  
</details>
<details>
<summary><code>pin_sid</code></summary>

session id

default: `"unknown"`  
flags: `0x80000200`  
</details>
<details>
<summary><code>pin_telemetry_actually_send</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>pin_telemetry_debug_code</code></summary>

Shows unformatted json of all messages

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>pin_telemetry_debug_payload</code></summary>

Shows final payloads being sent to PIN server, including header

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>pin_telemetry_debug_script</code></summary>

Shows nicely formatted json of script messages

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>pin_telemetry_dont_send_events</code></summary>

List of PIN events to suppress

default: `""`  
flags: `0x2002`  
</details>
<details>
<summary><code>pin_telemetry_hostname</code></summary>



default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>pin_telemetry_inactivity_send_time</code></summary>

Interval at which client PIN messages are sent. (Client only)

default: `"300"`  
flags: `0x2`  
</details>
<details>
<summary><code>pin_telemetry_max_payload_size</code></summary>



default: `"30720"`  
flags: `0x2`  
</details>
<details>
<summary><code>pin_telemetry_send_debug</code></summary>

Enables x-ea-lint-level 2 for useful error messages

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ping_debug</code></summary>

Debug latency calculation.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ping_max_green</code></summary>



default: `"70"`  
flags: `0x2`  
</details>
<details>
<summary><code>ping_max_red</code></summary>



default: `"250"`  
flags: `0x2`  
</details>
<details>
<summary><code>ping_max_yellow</code></summary>



default: `"140"`  
flags: `0x2`  
</details>
<details>
<summary><code>ping_minSentForChoice</code></summary>

Minimum number of pings sent to this target (not received) before we are willing to say the player can matchmake because we're confident that this data is useful

default: `"10"`  
flags: `0x2`  
</details>
<details>
<summary><code>ping_qos_units</code></summary>

Divisor to use for pings, so we don't think a 3 ping is wildly better than a 4 ping, but we do think a 33 ping is worse than a 31 ping (at 60fps, that's another frame of latency)

default: `"32"`  
flags: `0x2`  
</details>
<details>
<summary><code>ping_usePacketLoss</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>pixvis_enable</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>pixvis_maxquads</code></summary>

Change the upper bound on how many 2x2 quads to sample for pixel visibility

default: `"256"`  
flags: `0x4000`  
</details>
<details>
<summary><code>pixvis_spew</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>plat_environment</code></summary>



default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>plat_retryNameLookups</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>platform_user_id</code></summary>

Platform user id (origin user id on PC, xuid on xboxone)

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>playerListPartyColorB</code></summary>



default: `"204"`  
flags: `0x2`  
</details>
<details>
<summary><code>playerListPartyColorG</code></summary>



default: `"255"`  
flags: `0x2`  
</details>
<details>
<summary><code>playerListPartyColorR</code></summary>



default: `"179"`  
flags: `0x2`  
</details>
<details>
<summary><code>playerListUseFriendColor</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>player_ADS_buffer_time_seconds</code></summary>

How long (in seconds) will the game buffer a Toggle Zoom attempt if the player cannot ADS when they press the button.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>player_debugPredictedPosition</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>player_deltaAnimsMakeMeUnpredicted</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>player_doJetwashEffects</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>player_extraairaccelleration</code></summary>

Extra air acceleration given to players, even if they're already at max speed. Helps to start wall running

default: `"2.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>player_highFrequencyThinkDistance</code></summary>



default: `"6000"`  
flags: `0x2`  
</details>
<details>
<summary><code>player_movementBounds_predictionShare</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>player_movingDeathThreshold</code></summary>



default: `"50"`  
flags: `0x6000`  
</details>
<details>
<summary><code>player_respawnInputDebounceDuration</code></summary>

How long after respawning will certain player inputs be debounced for

default: `"0.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>player_setting_autosprint</code></summary>

Automatically sprint when walking forward.

default: `"0"`  
flags: `0x41000000`  
</details>
<details>
<summary><code>player_setting_damage_closes_deathbox_menu</code></summary>

Controls whether death box automatically closes when taking damage (used for menus).

default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>player_showEyePosition</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>player_useMovementBounds</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>player_viewchange_debug_pitch</code></summary>



default: `"20"`  
flags: `0x2`  
</details>
<details>
<summary><code>player_viewchange_debug_roll</code></summary>



default: `"9"`  
flags: `0x2`  
</details>
<details>
<summary><code>player_viewchange_debug_yaw</code></summary>



default: `"160"`  
flags: `0x2`  
</details>
<details>
<summary><code>playlist_changeGamemodeAutomatically</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>playlist_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>playlist_debug_getvar</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>playlist_debug_localization</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>playlist_variableErrorsChecks</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>portal_pointpush_debug</code></summary>

Debug the portal_pointpush.

default: `"0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>portal_pointpush_think_rate</code></summary>

The amount of time between thinks for the portal_pointpush.

default: `"0.05f"`  
flags: `0x6000`  
</details>
<details>
<summary><code>portal_use_player_avoidance</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>postdataupdate_threaded</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>postdataupdate_threaded_chunksize</code></summary>



default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>printConnectTimings</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>print_timeprefix</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>process_pending_vm_effects</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>progressbar_allow_wrap</code></summary>

Allow loading bar to wrap.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>progressbar_high_precision</code></summary>

Use a higher precision bar.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>progressbar_single_bar</code></summary>

Use a single bar.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>projectile_fake_prediction_in_kill_replay</code></summary>

Calls weapon primary-attack callbacks on client during replay to create predicted projectiles

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>projectile_faketrails</code></summary>

Enables fake projectile trails when the projectile impacts on the server before lag compensation is complete

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>projectile_filltrails</code></summary>

Fill the gap between the gun barrel and the first seen projectile position for trail Fx (1: 1st person only, 2: 3rd person only, 3: 1st and 3rd persons)

default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>projectile_lagCompensationDebug</code></summary>

Draws lag compensation on projectiles

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>projectile_lagCompensationDebugDrawTime</code></summary>

Amount of time debug drawing persists with projectile_lagCompensationDebug enabled.

default: `"3.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>projectile_lagCompensationDebugExtra</code></summary>

Draws the "real" arc the projectile would take, as well as an extra simple simulation to compare with the actual path

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>projectile_lagCompensationDebugServerOffset</code></summary>

Offset the server debug lines by this many units vertically

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>projectile_lagCompensationMissileTimeStepScalar</code></summary>

Scales the time step used for seeking missiles in lag compensation

default: `"1.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>projectile_muzzleOffsetFirstPersonDecayDist</code></summary>

Distance over which projectiles fake their origin to come out of the gun muzzle

default: `"1000"`  
flags: `0x2`  
</details>
<details>
<summary><code>projectile_muzzleOffsetFirstPersonDecayMaxTime</code></summary>

Max time over which projectiles fake their origin to come out of the gun muzzle

default: `"0.3"`  
flags: `0x2`  
</details>
<details>
<summary><code>projectile_muzzleOffsetThirdPersonDecayDist</code></summary>

Distance over which projectiles fake their origin to come out of the gun muzzle

default: `"1000"`  
flags: `0x2`  
</details>
<details>
<summary><code>projectile_muzzleOffsetThirdPersonDecayMaxTime</code></summary>

Max time over which projectiles fake their origin to come out of the gun muzzle

default: `"0.1"`  
flags: `0x2`  
</details>
<details>
<summary><code>projectile_prediction</code></summary>

Performs client-side prediction and lag compensation on projectiles

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>projectile_predictionErrorCorrectTime</code></summary>

Time over which prediction errors are corrected for projectiles

default: `"0.3"`  
flags: `0x2`  
</details>
<details>
<summary><code>prop_lightweightPropsSkipAnimData</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>prop_survivalSkipsAnimData</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>props_break_burst_rotation</code></summary>

Rate of rotation in degrees per second.

default: `"100"`  
flags: `0x2002`  
</details>
<details>
<summary><code>props_break_max_pieces</code></summary>

Maximum prop breakable piece count (-1 = model default)

default: `"-1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>props_break_max_pieces_perframe</code></summary>

Maximum prop breakable piece count per frame (-1 = model default)

default: `"20"`  
flags: `0x2002`  
</details>
<details>
<summary><code>publication_hostname</code></summary>



default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>push_cl</code></summary>

1: Moving geo pushes client entities, 0: Client entities do not get pushed

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>push_cl_always_update_prev_matrix</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>push_debug</code></summary>

Debug all pushing entities

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>push_debug_ent</code></summary>

Debug pushing entity

default: `"-1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>push_ragdolls</code></summary>

Toggles whether to push ragdoll entities

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>pve_debug</code></summary>



default: `""`  
flags: `0x2002`  
</details>
<details>
<summary><code>pvs_addWorkItemsAccum</code></summary>

Accumulate this many work items from the main PVS job before adding them to the worker thread array, which can be slow

default: `"100"`  
flags: `0x2`  
</details>
<details>
<summary><code>pvs_addWorkItemsThreshold_edges</code></summary>

load balancing threshold; if a node has more than this many leaves, it will spread the work across threads

default: `"50"`  
flags: `0x2`  
</details>
<details>
<summary><code>pvs_addWorkItemsThreshold_leaves</code></summary>

load balancing threshold; if a node has more than this many leaves, it will spread the work across threads

default: `"5000"`  
flags: `0x2`  
</details>
<details>
<summary><code>pvs_cullBoxes</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>pvs_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>pvs_drawPortals</code></summary>

Draw portal access paths. when N > 0, access paths with edge count >= N will show up in green color. when N < 0, access paths with edge count == -N will show up in green.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>pvs_frustumCullOnly</code></summary>

0 - Off, 1 - On by Script, 2 - forced On

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>pvs_start_early</code></summary>

0 not early, 1 after view setup, 2 after threaded bone setup

default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_AirboatViewDampenDamp</code></summary>



default: `"1.0"`  
flags: `0x6100`  
</details>
<details>
<summary><code>r_AirboatViewDampenFreq</code></summary>



default: `"7.0"`  
flags: `0x6100`  
</details>
<details>
<summary><code>r_AirboatViewZHeight</code></summary>



default: `"0.0"`  
flags: `0x6100`  
</details>
<details>
<summary><code>r_JeepViewDampenDamp</code></summary>



default: `"1.0"`  
flags: `0x6100`  
</details>
<details>
<summary><code>r_JeepViewDampenFreq</code></summary>



default: `"7.0"`  
flags: `0x6100`  
</details>
<details>
<summary><code>r_VehicleViewDampen</code></summary>



default: `"1"`  
flags: `0x6100`  
</details>
<details>
<summary><code>r_WaterDrawReflection</code></summary>

Enable water reflection

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_WaterDrawRefraction</code></summary>

Enable water refraction

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_aspectratio</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_bloomtintb</code></summary>



default: `"0.11"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_bloomtintexponent</code></summary>



default: `"2.2"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_bloomtintg</code></summary>



default: `"0.59"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_bloomtintr</code></summary>



default: `"0.3"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_blurmenubg</code></summary>

Blurs background when menus are open

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_bone_matrix_bulk_update_threshold</code></summary>



default: `"64"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_brush_queue_mode</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_createmodeldecals</code></summary>



default: `"1"`  
flags: `0x40000000`  
</details>
<details>
<summary><code>r_cullshadowworldmeshes</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_debug_draw_box_depth_test</code></summary>

Toggle depth test for debug draw box functionality

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_decal_cover_count</code></summary>



default: `"4"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_decal_cull_stretch_limit</code></summary>

Reciprocal of per-tri limit on decal stretching (0 is most permissive, 1 is most restrictive.)

default: `"0.707"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_decal_draw_basis</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_decal_drawclipped</code></summary>

A bit-field! 1:Draw decal debug triangle overlays of *all* potential hits, 2:Draw actual hits, 4:Draw clipped hits

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_decal_overlap_area</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_decal_overlap_count</code></summary>



default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_decal_test_scale</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_decals</code></summary>



default: `"256"`  
flags: `0x40000000`  
min value: `0`  
max value: `256`  
</details>
<details>
<summary><code>r_delay_texture_destroy</code></summary>

immediate call on Destroy() may cause GPU hang as it can still be used by GPU. this will make it delayed by one frame.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_ditherAlpha</code></summary>

If true, does a dither pattern for alpha fading.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_ditherFade</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_ditherFade</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_ditherFadeShadows</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_ditherFadeShadows</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawallrenderables</code></summary>

Draw all renderables, even ones inside solid leaves.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawalphasort</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawbrushmodels</code></summary>

Render brush models. 0=Off, 1=Normal, 2=Wireframe

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawbrushmodels</code></summary>

Render brush models. 0=Off, 1=Normal, 2=Wireframe

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawdecals</code></summary>

Render decals.

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawdepth_of_blend2transparent</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_drawdlights</code></summary>

whether to debug draw dlights

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawentities</code></summary>

0: dont' draw; 1: draw normal; 2: draw bones; 3: draw hulls

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawlightdist</code></summary>

If r_drawstaticlight is -1, only include draw lights within this radius

default: `"4000"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawlightinfo</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawmodelsinzfill</code></summary>

Draw models in the zfill pass where they will affect light tile culling

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawmodelstatsoverlay</code></summary>

Draws information for static props and other models (0,1,2)

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawmodelstatsoverlay</code></summary>

Draws information for static props and other models (0,1,2)

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawmodelstatsoverlaydistance</code></summary>



default: `"500"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawmodelstatsoverlayfilter</code></summary>



default: `"-1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawmodelstatsoverlaymax</code></summary>

time in milliseconds beyond which a model overlay is fully red in r_drawmodelstatsoverlay 2

default: `"1.5"`  
flags: `0x80`  
</details>
<details>
<summary><code>r_drawmodelstatsoverlaymin</code></summary>

time in milliseconds that a model must take to render before showing an overlay in r_drawmodelstatsoverlay 2

default: `"0.1"`  
flags: `0x80`  
</details>
<details>
<summary><code>r_drawopaquerenderables</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawothermodels</code></summary>

Enables drawing 'other' (C_BaseAnimating) models

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawparticles</code></summary>

Enable/disable particle rendering

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawrenderboxes</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawscreenspaceparticles</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_drawsky</code></summary>

Enable the rendering of sky

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawskybox_deprecated</code></summary>

Enable the rendering of sky boxes

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawstaticlight</code></summary>

0 = none, -1 = all within r_drawlightdist, other draws that light index

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawstaticprops</code></summary>

Toggle drawing of static props

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawtracers</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawvgui</code></summary>

Enable the rendering of vgui panels

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawviewmodel</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawworld</code></summary>

Render the world (0 = none, 1 = opaque only, 2 = trans only, 3 = both).

default: `"3"`  
flags: `0x40004000`  
</details>
<details>
<summary><code>r_dynamic</code></summary>



default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>r_earlyRenderables</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_enableOriginSort</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_fadeincode</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_farz</code></summary>

Override the far clipping plane. -1 means to use the value in env_fog_controller.

default: `"-1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_fastzreject</code></summary>

Activate/deactivates a fast z-setting algorithm to take advantage of hardware with fast z reject. Use -1 to default to hardware settings

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_forcecheapwater</code></summary>

Force all water to be cheap water, will show old renders if enabled after water has been seen

default: `"0"`  
flags: `0x4008`  
</details>
<details>
<summary><code>r_jiggle_bones</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_lightmap</code></summary>



default: `"-1"`  
flags: `0x804000`  
</details>
<details>
<summary><code>r_lightprobe_force_trans_dist</code></summary>

if an entity moves this distance or greater in one frame it is automatically transitioned to a new probe

default: `"2000"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_lightstyle</code></summary>



default: `"-1"`  
flags: `0x804000`  
</details>
<details>
<summary><code>r_lod</code></summary>



default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_lod</code></summary>



default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_lod_switch_scale</code></summary>



default: `"1"`  
flags: `0x40000000`  
</details>
<details>
<summary><code>r_mapextents</code></summary>

Set the max dimension for the map.  This determines the far clipping plane

default: `"16384"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_modeldecal_maxtotal</code></summary>



default: `"75"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_nearz</code></summary>

Near clipping plane distance

default: `"7"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_no_stalls</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_no_stalls</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_no_stalls</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_norefresh</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_particle_lighting_debug</code></summary>

Toggle Particle Lighting Debug Texture

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_particle_lighting_enable</code></summary>

Toggle Particle Lighting

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_particle_lighting_enable</code></summary>

Toggle Particle Lighting

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_particle_lighting_force</code></summary>

Force all particles to be lit

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_particle_lighting_force</code></summary>

Force all particles to be lit

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_particle_low_res_debug</code></summary>

Toggle Low Res Paricle Debug Texture

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_particle_low_res_draw_weight_tex</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_particle_low_res_enable</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_particle_low_res_force</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_particle_low_res_tiled_composite</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_particle_sim_spike_increment_ms</code></summary>



default: `"0.25"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_particle_sim_spike_threshold_ms</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_particle_timescale</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_pos_debug</code></summary>



default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>r_render_pos_debug</code></summary>



default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>r_rimlight</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_rootlod</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_rootlod</code></summary>

Root LOD

default: `"0"`  
flags: `0x4800002`  
min value: `0`  
max value: `2`  
</details>
<details>
<summary><code>r_ropetranslucent</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_setupBoneWorkerThreadhold</code></summary>

minimum ModelListByType_t::m_nSetupBoneCount value to be threaded for SetupBoneForModelList() call

default: `"10"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_shadowrendertotexture</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_sky_ignoreAngles</code></summary>

Ignore the angle of the sky (for debugging)

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_sort_trans_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_sort_trans_debug_dist</code></summary>



default: `"2000"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_threaded_particles</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_updaterefracttexture</code></summary>

When disabled, supresses any update of refract texture.

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_updaterefracttexture_allowmultiple</code></summary>

Allows multiple updates of refract texture per frame.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_visambient</code></summary>

Draw leaf ambient lighting samples. Mask of VIS_AMBIENT = 1, VIS_SKY = 2, VIS_SUN = 4, VIS_CLOUDMASK = 8, VIS_LIGHTS_1ST = 16, etc.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_visambient_orig</code></summary>

Show original lighting probes instead of the improved ones the game actually uses

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_visambient_point</code></summary>

Draw leaf ambient lighting samples, for a point (like particles).

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_vislighting_sphereradius</code></summary>



default: `"12.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_vismodellighting</code></summary>

Visualize model lighting.  Mask of VIS_AMBIENT = 1, VIS_SKY = 2, VIS_SUN = 4, VIS_CLOUDMASK = 8, VIS_LIGHTS_1ST = 16, etc.  Use -1 to see all lights.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_vismodellighting_lightpos</code></summary>

Draw a line from the point light to the model lighting origin for this many of the closest lights that have r_vismodellighting enabled.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_vismodellighting_maxdist</code></summary>



default: `"1000.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_vismodellighting_maxdist</code></summary>



default: `"1000.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_vismodellighting_mindist</code></summary>



default: `"48.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_vismodellighting_mindist</code></summary>



default: `"48.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_vismodellighting_offset_x</code></summary>

Offset the model lighting spheres by this amount.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_vismodellighting_offset_y</code></summary>

Offset the model lighting spheres by this amount.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_vismodellighting_offset_z</code></summary>

Offset the model lighting spheres by this amount.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_visualizeproplightcaching</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_visualizetraces</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_visualizetraces_duration</code></summary>



default: `"0.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_volumetric_lighting_blur_count</code></summary>



default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_volumetric_lighting_blur_type</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_volumetric_lighting_enabled</code></summary>

0 - never, 1 - if required (controlled by script), 2 - always

default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_volumetric_lighting_numSteps</code></summary>



default: `"10"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_volumetric_lighting_rotate_dither</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_waterforceexpensive</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_waterforcereflectentities</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_zfill</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>ragdoll_debug</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>ragdoll_sleepaftertime</code></summary>

After this many seconds of being basically stationary, the ragdoll will go to sleep.

default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>rankedplay_display_enabled</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>rankedplay_voice_enabled</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>rate</code></summary>

Max bytes/sec the host can receive data

default: `"128000"`  
flags: `0x2`  
</details>
<details>
<summary><code>real_time_update_dt</code></summary>



default: `"0.001"`  
flags: `0x2`  
</details>
<details>
<summary><code>recalculateOrigin_threaded_chunksize</code></summary>



default: `"50"`  
flags: `0x2`  
</details>
<details>
<summary><code>remoteCalls_requireConnectionScriptsForViewPlayer</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>remoteMatchInfo_print</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>replay_enable</code></summary>

Enable Replay recording on server

default: `"1"`  
flags: `0x402002`  
</details>
<details>
<summary><code>replay_prediction_smooth</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>report_cliententitysim</code></summary>

List all clientside simulations and time - will report and turn itself off.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>report_clientthinklist</code></summary>

List all clientside entities thinking and time - will report and turn itself off.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>rodeo_camera_smooth_blend_out_time</code></summary>



default: `"0.4"`  
flags: `0x2`  
</details>
<details>
<summary><code>rodeo_camera_smooth_enable</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>rodeoed_anims_enabled</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>rope_collide</code></summary>

Collide rope with the world

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>rope_debug_shake</code></summary>

Helps visualize ropes effected by a shake.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>rope_parallelMeshBuilder</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>rope_regenMeshEachDraw</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>rope_shake</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>rope_texels_per_world_unit</code></summary>



default: `"8"`  
flags: `0x2`  
</details>
<details>
<summary><code>rope_wiggle_harmonic_falloff</code></summary>

Falloff for oscillation magnitude of wave of increasing frequency (ropes and grapple)

default: `"0.75"`  
flags: `0x2`  
</details>
<details>
<summary><code>rope_wiggle_magnitude_loose</code></summary>

Fraction of rope (including grapple) distance used as max wiggle distance while the rope is loose (shooting)

default: `".04"`  
flags: `0x2`  
</details>
<details>
<summary><code>rope_wiggle_magnitude_tight</code></summary>

Fraction of rope (including grapple) distance used as max wiggle distance while the rope is tight (pulling or retracting)

default: `".002"`  
flags: `0x2`  
</details>
<details>
<summary><code>rope_wiggle_oscillate_speed</code></summary>

Speed at which rope (including grapple) wiggle oscillates

default: `"64"`  
flags: `0x2`  
</details>
<details>
<summary><code>rope_wiggle_rotate_speed</code></summary>

Speed at which rope (including grapple) wiggle rotates

default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>rope_wiggle_zipline_min_points</code></summary>

Increases point count for ziplines that are wiggling

default: `"80"`  
flags: `0x2`  
</details>
<details>
<summary><code>rope_wind_dist</code></summary>

Don't use CPU applying small wind gusts to ropes when they're past this distance.

default: `"1000"`  
flags: `0x2`  
</details>
<details>
<summary><code>rotate_ents</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>rspn_motd</code></summary>



default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>rt_sync_message_pump</code></summary>

If 1 render thread with process message queue before starting main thread processing. If 2 then main thread will wait for the next frames message queue processing before gathering mouse input. If 3 then the main thread will wait for the current frames message queue processing before gathering mouse input...this may have a problem with the queue message pump sometimes starting before gathering mouse input and sometimes after causing hitches, but it has less latency and lower frame times when compared to the other methods. If 0 then gathering mouse input may happen before, during, or after the last frames mouse messages were process causing horrible hitch mouse response.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>rt_worker</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>ruiPanel_resArgName</code></summary>



default: `"actualRes"`  
flags: `0x2`  
</details>
<details>
<summary><code>rui_asyncTracks</code></summary>

Toggles async update of RUI tracks

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>rui_defaultDebugFontFace</code></summary>

Default font face for rui text in debug messages

default: `"PTMonoFont"`  
flags: `0x2`  
</details>
<details>
<summary><code>rui_defaultFontFace</code></summary>

Default font face for rui text

default: `"DefaultRegularFont"`  
flags: `0x2`  
</details>
<details>
<summary><code>rui_defaultFontHeight</code></summary>

Default font height for rui text

default: `"28"`  
flags: `0x2`  
</details>
<details>
<summary><code>rui_overrideVguiTextRendering</code></summary>

Use rui for rendering all vgui text

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>rui_padDist</code></summary>



default: `"0.7"`  
flags: `0x2`  
</details>
<details>
<summary><code>rui_safeAreaFrac</code></summary>

Fraction of safe area to use

default: `"0.0"`  
flags: `0x2`  
max value: `1`  
</details>
<details>
<summary><code>rui_standardTextHeight</code></summary>



default: `"64.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>s2sPort</code></summary>

S2S communication port

default: `"(37015 + 1)"`  
flags: `0x80000`  
</details>
<details>
<summary><code>save_enable</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>scheme_manager_font_debug</code></summary>

0:Off, 1:On

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>scr_centertime</code></summary>



default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>screen_indicator_back_range</code></summary>

Number of degrees behind the player that is considered more behind than to the side

default: `"60.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>screen_indicator_ellipse_height</code></summary>



default: `"0.75"`  
flags: `0x2`  
</details>
<details>
<summary><code>screen_indicator_ellipse_width</code></summary>



default: `"0.85"`  
flags: `0x2`  
</details>
<details>
<summary><code>screen_indicator_pitch_limit</code></summary>

The maximum pitch difference that will affect the indicator position

default: `"75.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>screen_indicator_pitch_scale</code></summary>



default: `"2.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>screenfade_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>script_compile_all_levels</code></summary>

Compiles all level scripts when loading a map.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>script_debugger_connect_client_on_mapspawn</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>script_debugger_connect_ui_auto</code></summary>

Only takes effect after uiscript_reset. Use command line argument -script_debugger_connect_ui for startup.

default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>script_debugger_host</code></summary>



default: `"localhost"`  
flags: `0x2002`  
</details>
<details>
<summary><code>script_debugger_port_client</code></summary>



default: `"15101"`  
flags: `0x2`  
</details>
<details>
<summary><code>script_debugger_port_server</code></summary>



default: `"15100"`  
flags: `0x2`  
</details>
<details>
<summary><code>script_debugger_port_ui</code></summary>



default: `"15102"`  
flags: `0x2`  
</details>
<details>
<summary><code>script_disallow_newslot_on_globals</code></summary>

Throws compile errors for global variables assigned with <-

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>script_dump_simple</code></summary>

If enabled then script dump format will skip null array/table entries and display each non-container value on a single line with the fully scoped key name.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>script_error_on_midgame_load</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>script_infinite_loop_ms</code></summary>

If script runs for more than this many milliseconds at one time then you will get a script error.


default: `"10000"`  
flags: `0x2002`  
</details>
<details>
<summary><code>script_parallel_trace_LOS_multiple</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>script_precache_errors</code></summary>



default: `"1"`  
flags: `0x6000`  
</details>
<details>
<summary><code>script_printDeferredCalls</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>script_retry_after_compile_errors</code></summary>

After a compile error, tries compiling again immediately.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>script_seasonNameQueryInterval</code></summary>



default: `"30"`  
flags: `0x2`  
</details>
<details>
<summary><code>script_showErrorDialogs</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>script_slopTimeBeforeBudgetEnforcement</code></summary>

How long to wait before we start complaining about slow budgets

default: `"30"`  
flags: `0x2`  
</details>
<details>
<summary><code>sequence_transitioner_enable</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>serverFilter</code></summary>

Only connects to servers with the same value

default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>serverReports_hostname</code></summary>



default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>server_concommands_allways_network</code></summary>

When set to 1 , server commands with listen server pass down the network layer.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>server_query_interval</code></summary>



default: `"0.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>sfm_record_hz</code></summary>



default: `"30"`  
flags: `0x2`  
</details>
<details>
<summary><code>shadow_always_update</code></summary>

Set to 1 to make shadow maps regenerate every frame.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>shadow_bleedfudge</code></summary>

Fudge value to decrease shadow map light bleeding

default: `"0.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>shadow_capable</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>shadow_clear_dist</code></summary>



default: `"1.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>shadow_dbg_draw</code></summary>

Visualize shadow atlas texture (1 .. 4, larger numbers for smaller sizes)Tweak - Purple, Dirty - Red, Dynamic - Green, Old Dynamic - Blue

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>shadow_default_filter_size</code></summary>

Size of the blur filter applied to spot shadows that don't request a different size. Odd integer only.

default: `"3"`  
flags: `0x4000`  
</details>
<details>
<summary><code>shadow_depth_dimen_min</code></summary>

Minimum resolution of a spot shadow map in width and height

default: `"256"`  
flags: `0x40000000`  
</details>
<details>
<summary><code>shadow_depth_upres_factor_max</code></summary>

Maximum requested upres factor of spot shadows (dimen_min << this) == largest spot shadow dimen

default: `"2"`  
flags: `0x40000000`  
</details>
<details>
<summary><code>shadow_drawfrustum</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>shadow_dynamic_blendfactor</code></summary>

Blend dynamic shadows over time. Low value: long history, 1: no history

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>shadow_enable</code></summary>



default: `"1"`  
flags: `0x40000000`  
</details>
<details>
<summary><code>shadow_esm_enable</code></summary>

(EXPERIMENTAL) Use exponential spot shadow maps instead of variance maps

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>shadow_filter_maxstep</code></summary>

Max step threshold for shadow map blend

default: `"0.18"`  
flags: `0x4000`  
</details>
<details>
<summary><code>shadow_info</code></summary>

Information about currently active depth shadows

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>shadow_lobby_mode_allowed</code></summary>

allow special mode for lobby that does some tricks to improve spotlight shadow quality. 0 - disallowed, 1- allowed, 2 - forced

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>shadow_max_dynamic_lobby</code></summary>

Maximum number of shadows that should update every frame in Lobby.

default: `"5"`  
flags: `0x40000000`  
</details>
<details>
<summary><code>shadow_max_old_dynamic</code></summary>

Maximum number of old shadows that should update every frame. It's a part of shadow_maxdynamic

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>shadow_max_spot_updates</code></summary>

Maximum number of dynamic shadow maps to update on any given frame

default: `"4"`  
flags: `0x4000`  
</details>
<details>
<summary><code>shadow_maxdynamic</code></summary>

Maximum number of shadows that should update every frame.

default: `"4"`  
flags: `0x40000000`  
</details>
<details>
<summary><code>shadow_maxdynamic</code></summary>

Maximum number of shadows that should update every frame.

default: `"4"`  
flags: `0x40000000`  
</details>
<details>
<summary><code>shadow_min_count_smallest</code></summary>

Represents the minimum number of min resolution spot shadows to allocate in the shadow atlas.This will be adjusted upward to a multiple of the max sized spot shadow to find legal sized atlas dimensions.

default: `"576"`  
flags: `0x4000`  
</details>
<details>
<summary><code>shadow_minvariance</code></summary>

Minimum variance for shadow maps (controls edge softness)

default: `"0.00001"`  
flags: `0x4000`  
</details>
<details>
<summary><code>shadow_multisampled</code></summary>

Enable multisampling for shadows.

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>shadow_noLOD</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>shadow_show_spot_udpate_infos</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>shadow_tools_depth_dimen_min</code></summary>

Minimum tools mode (lightedit) resolution of a spot shadow map in width and height

default: `"256"`  
flags: `0x4000`  
</details>
<details>
<summary><code>shadow_tools_depth_upres_factor_max</code></summary>

Maximum requested tools mode upres factor of spot shadows (dimen_min << this) == largest spot shadow dimen

default: `"3"`  
flags: `0x4000`  
</details>
<details>
<summary><code>shadow_tools_min_count_smallest</code></summary>

Represents the minimum number of min resolution spot shadows to allocate in the shadow atlas in tools mode.This will be adjusted upward to a multiple of the max sized spot shadow to find legal sized atlas dimensions.

default: `"4096"`  
flags: `0x4000`  
</details>
<details>
<summary><code>shadow_tools_mode</code></summary>

Turn on shadow tools mode rendering (higher atlas size limits, running out of shadows does not spam

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>shadow_update_culling</code></summary>

Don't update shadows that aren't in the view frustum.

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>shake_angleFactor_human</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>shake_angleFactor_titan</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>shake_basicPitchFactor</code></summary>



default: `"0.20"`  
flags: `0x2`  
</details>
<details>
<summary><code>shake_basicRandomRollFactor</code></summary>



default: `"0.15"`  
flags: `0x2`  
</details>
<details>
<summary><code>shake_offsetFactor_human</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>shake_offsetFactor_titan</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>shake_viewmodelFactor_ads_human</code></summary>



default: `"0.01"`  
flags: `0x2`  
</details>
<details>
<summary><code>shake_viewmodelFactor_ads_titan</code></summary>



default: `"0.10"`  
flags: `0x2`  
</details>
<details>
<summary><code>shake_viewmodelFactor_human</code></summary>



default: `"0.10"`  
flags: `0x2`  
</details>
<details>
<summary><code>shake_viewmodelFactor_titan</code></summary>



default: `"0.10"`  
flags: `0x2`  
</details>
<details>
<summary><code>showfps_enabled</code></summary>



default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>showfps_heightpercent</code></summary>



default: `"0.25"`  
flags: `0x2`  
</details>
<details>
<summary><code>showfps_mouse_latency</code></summary>

If 1 showfps_enabled will show mouse input latency instead of the time from before the move command.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>showfps_smoothtime</code></summary>



default: `"0.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>showfps_spinner</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>showmem_enabled</code></summary>



default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>showmem_mode_bottom</code></summary>

Mode 0 is total free memory(excluding garlic), 1 is small block heap, 2 is Client Script, 3 is  UI Script

default: `"3"`  
flags: `0x80000`  
</details>
<details>
<summary><code>showmem_mode_top</code></summary>

Mode 0 is total free memory(excluding garlic), 1 is small block heap, 2 is Client Script, 3 is  UI Script

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>shownet_enabled</code></summary>



default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>showsnapshot_enabled</code></summary>



default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sidearmSwapSelectCooldown</code></summary>



default: `"0.25"`  
flags: `0x2`  
</details>
<details>
<summary><code>sidearmSwapSelectDoubleTapTime</code></summary>



default: `"0.25"`  
flags: `0x2`  
</details>
<details>
<summary><code>single_frame_shutdown_for_reload</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>singlestep</code></summary>

Run engine in single step mode ( set next to 1 to advance a frame )

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>skill_arena</code></summary>

The arena that skill should be read from / written to (eg. fnf, experimental, etc)

default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>skill_dediOnly</code></summary>

Only do skill for dedicated servers

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>skill_enabled</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>skill_hostname</code></summary>



default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>skip_jump_height_fraction</code></summary>

Jump height fraction when skipping

default: `"1.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>skip_jump_height_speed</code></summary>

Jump height loss only applies above this speed

default: `"450"`  
flags: `0x2002`  
</details>
<details>
<summary><code>skip_replenish_double_jump</code></summary>

Whether the player can double jump after skipping

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>skip_sounds</code></summary>

Enables skip-specific sounds

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>skip_speed_reduce</code></summary>

Speed lost when skipping

default: `"100"`  
flags: `0x2002`  
</details>
<details>
<summary><code>skip_speed_retain</code></summary>

Speed loss doesn't go below this

default: `"-1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>skip_time</code></summary>

Time after landing that is considered "skipping" if the player jumps again

default: `"1.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sleep_when_meeting_framerate</code></summary>

Sleep instead of spinning if we're meeting the desired framerate.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sleep_when_meeting_framerate_headroom_ms</code></summary>

Only sleep if the current frame has at least this much time remaining, otherwise spin.

default: `"2.25"`  
flags: `0x2`  
</details>
<details>
<summary><code>slide_auto_stand</code></summary>

Automatically stand when slide ends

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>slide_max_angle_dot</code></summary>

Cosine of max angle from forward that you can slide when sprinting

default: `"0.6"`  
flags: `0x2002`  
</details>
<details>
<summary><code>slide_step_velocity_reduction</code></summary>

Velocity reduction when going up a step (is multiplied by step height)

default: `"10"`  
flags: `0x2002`  
</details>
<details>
<summary><code>slide_viewTiltDecreaseSpeed</code></summary>

Speed at which view tilt decreases while sliding in degrees per second

default: `"2.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>slide_viewTiltIncreaseSpeed</code></summary>

Speed at which view tilt increases while sliding in degrees per second

default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>slide_viewTiltPlayerSpeed</code></summary>

Speed at which view tilt is full while sliding

default: `"400"`  
flags: `0x2`  
</details>
<details>
<summary><code>slide_viewTiltSide</code></summary>

View tilt when looking to the side while sliding in degrees

default: `"15"`  
flags: `0x2`  
</details>
<details>
<summary><code>slide_whileInAir</code></summary>

Allows beginning a slide (including the boost) while still in the air

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>slowconsolelog_old_logic</code></summary>

Flush console.log after each write.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>smoothstairs_lunge</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sort_opaque_meshes</code></summary>

Sort opaque meshes front to back to try to improve rendering speed.  This may not be worth the CPU cost.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sound_classic_music</code></summary>

classic music volume

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>sound_entity_seek_snap</code></summary>

Play C_ImporantOnEntSound entity sound from beginning if we get it within this many seconds of its begin time.

default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sound_musicReduced</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>sound_num_speakers</code></summary>

2 - headphones or stereo, 6 - 5.1 surround, 8 - 7.1 surround.  All other values invalid

default: `"2"`  
flags: `0x80`  
</details>
<details>
<summary><code>sound_only_warn_on_missing_sound_events_in_client_script</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sound_printloaderrors</code></summary>

Set to 1 to print sound errors on load.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sound_volume</code></summary>

master game volume

default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>sound_volume_dialogue</code></summary>

dialogue volume (mp)

default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>sound_volume_dialogue_sp</code></summary>

dialogue volume (sp)

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sound_volume_music_game</code></summary>

music volume in game (mp)

default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>sound_volume_music_game_sp</code></summary>

music volume in game (sp)

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sound_volume_music_lobby</code></summary>

music volume in lobby

default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>sound_volume_sfx</code></summary>

sound effect volume (mp)

default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>sound_volume_sfx_sp</code></summary>

sound effect volume (sp)

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sound_volume_voice</code></summary>

voice chat volume

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>sound_without_focus</code></summary>

Play sounds even when the app doesn't have focus.

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>soundscape_fadetime</code></summary>

Time to crossfade sounds between soundscapes

default: `"2.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>soundscape_message</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>soundscape_radius_debug</code></summary>

Prints current volume of radius sounds

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>soundtrigger_repeat_interval</code></summary>

Decides how long to wait before repeating a soundtrigger event on the given player. Set to 0 to wait until the current sound ends.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sp_not_focus_pause</code></summary>

Pause the singleplayer game when the window is not in focus

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>spam_skinning_matrices_used</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>spam_skinning_matrices_used_detailed</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>spatial_partition_deadlock_assert</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>speech_queue_bytes</code></summary>



default: `"33000"`  
flags: `0x2`  
</details>
<details>
<summary><code>speechtotext_audioenabled</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>speechtotext_enabled</code></summary>



default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>speechtotext_forcedisabled</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>speechtotext_hostname</code></summary>



default: `"gateway-wdc.watsonplatform.net"`  
flags: `0x2`  
</details>
<details>
<summary><code>speechtotext_msg_droptimeout</code></summary>



default: `"30.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>speechtotext_path</code></summary>



default: `"speech-to-text/api/v1/recognize?profanity_filter=true&smart_formatting=true"`  
flags: `0x2`  
</details>
<details>
<summary><code>speechtotext_quiettime</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>speechtotext_stats_errorspermin</code></summary>



default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>speechtotext_stats_interval</code></summary>



default: `"60.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>speechtotext_stats_senderrors</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>speechtotext_stats_sendrequests</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>speechtotext_stats_sendsuccess</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>speechtotexttoken_hostname</code></summary>



default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>speex_audio_recording</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>speex_audio_value</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>speex_preprocess_agc_max_gain</code></summary>

Set maximal gain in dB. ( High values Risks swamping noise filter)

default: `"13"`  
flags: `0x2`  
</details>
<details>
<summary><code>speex_preprocess_noise_suppress</code></summary>

Set maximum attenuation of the noise in dB (negative number)

default: `"-20"`  
flags: `0x2`  
</details>
<details>
<summary><code>speex_preprocess_set_agc_decrenment</code></summary>

Set maximal gain decrease in dB/second.

default: `"-10"`  
flags: `0x2`  
</details>
<details>
<summary><code>speex_preprocess_set_agc_increment</code></summary>

Set maximal gain increase in dB/second.

default: `"4"`  
flags: `0x2`  
</details>
<details>
<summary><code>speex_preprocess_set_agc_target</code></summary>

Set Automatic Gain Control target. 0/32767

default: `"8000"`  
flags: `0x2`  
</details>
<details>
<summary><code>speex_quiet_threshold</code></summary>



default: `"1300"`  
flags: `0x80`  
</details>
<details>
<summary><code>speex_quiet_window</code></summary>



default: `"40"`  
flags: `0x2`  
</details>
<details>
<summary><code>speex_set_enh</code></summary>

Set enhancement on/off (decoder only)

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>speex_use_highpass</code></summary>

Controlls the running o a lowpass filter do help remove DC.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>speex_use_preproser</code></summary>

Controls the running of voice preprocessor.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>spinner_debug_info</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sprint_powerdrain</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sprint_view_shake_style</code></summary>



default: `"0"`  
flags: `0x41000000`  
</details>
<details>
<summary><code>sprinttilt_accel</code></summary>

Acceleration of sprint view tilt fraction

default: `"35"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sprinttilt_maxvel</code></summary>

Maximum speed of sprint view tilt

default: `"2"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sprinttilt_turnrange</code></summary>

Max turn rate that creates view tilt when sprinting

default: `"120"`  
flags: `0x2002`  
</details>
<details>
<summary><code>ss_enable</code></summary>

Enables Split Screen support. Play Single Player now launches into split screen mode. NO ONLINE SUPPORT

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ss_force_primary_fullscreen</code></summary>

If enabled, all splitscreen users will only see the first user's screen full screen

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ss_mimic</code></summary>

Split screen users mimic base player's CUserCmds

default: `"0"`  
flags: `0x4002`  
</details>
<details>
<summary><code>ss_splitmode</code></summary>

Two player split screen mode (0 - recommended settings base on the width, 1 - horizontal, 2 - vertical (only allowed in widescreen)

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ss_verticalsplit</code></summary>

Two player split screen uses vertical split (do not set this directly, use ss_splitmode instead).

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ss_viewmodelfov</code></summary>

Client-side viewmodel fov control that is global for all splitscreen players on this machine.  This gets overridden via splitscreen_config.txt for splitscreen.

default: `"54"`  
flags: `0x2002`  
</details>
<details>
<summary><code>ss_voice_hearpartner</code></summary>

Route voice between splitscreen players on same system.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ssao_allow_partial</code></summary>

When it's enabled, Partial SSAO could run when dynamic viewport is smaller than SSAO targets.
It doesn't look good and has a problem of some flickering. Try it with viewportscale_rand to see the problem.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ssao_blur</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>ssao_blur_edge_sharpness</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ssao_depth_max</code></summary>



default: `"10000"`  
flags: `0x2`  
</details>
<details>
<summary><code>ssao_downsample</code></summary>

0 = 1:1, 1 = 2:1, 2 = 4:1

default: `"0"`  
flags: `0x40000000`  
</details>
<details>
<summary><code>ssao_enabled</code></summary>



default: `"1"`  
flags: `0x40000000`  
</details>
<details>
<summary><code>ssao_exponent</code></summary>



default: `"1.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>ssao_jitter_scale</code></summary>

in range of [0,1]

default: `"0.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>ssao_max_res</code></summary>

SSAO render target size will be enforced to be this size when it's going to be ssao_max_res_threshold or greater

default: `"1080"`  
flags: `0x2`  
</details>
<details>
<summary><code>ssao_max_res_threshold</code></summary>

ssao_max_res is enforced when SSAO render target size is at this size or greater

default: `"1440"`  
flags: `0x2`  
</details>
<details>
<summary><code>ssao_num_directions</code></summary>



default: `"8"`  
flags: `0x2`  
</details>
<details>
<summary><code>ssao_num_steps</code></summary>



default: `"4"`  
flags: `0x2`  
</details>
<details>
<summary><code>ssao_on_everything</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ssao_radius</code></summary>

occlusion hemisphere radius in world space unit

default: `"118"`  
flags: `0x2`  
</details>
<details>
<summary><code>ssao_radius_in_lobby</code></summary>

occlusion hemisphere radius in world space unit

default: `"4"`  
flags: `0x2`  
</details>
<details>
<summary><code>ssao_show</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ssao_show</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ssao_show</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ssao_snap_uv</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>ssao_tech</code></summary>

0 = Off, 1 = HBAO, 2 = GTAO uni, 3 = GTAO cos, 4 = HBAO basic, 5 = HBAO1x1, 6 = GTAOuni1x1, 7 = GTAOcos1x1

default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>ssao_tech</code></summary>

0 = Off, 1 = HBAO, 2 = GTAO uni, 3 = GTAO cos, 4 = HBAO basic, 5 = HBAO1x1, 6 = GTAOuni1x1, 7 = GTAOcos1x1

default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>ssao_upsample_ranged</code></summary>

It improves downsampled SSAO quality. it works for GTAO 4x4 mode only.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>startButtonCommand</code></summary>

What command to send when start is pressed

default: `"ingamemenu_activate"`  
flags: `0x2`  
</details>
<details>
<summary><code>staticProp_budget</code></summary>

The maximum number of static props that will be drawn.

default: `"8192"`  
flags: `0x2`  
min value: `100`  
max value: `8192`  
</details>
<details>
<summary><code>staticProp_buildlists_on_worker</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>staticProp_debug_draw</code></summary>

Orange - regular culled prop. Red - "do not fade" prop Green - out of range

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>staticProp_earlyDepthPrepass</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>staticProp_earlyDepthPrepassDist</code></summary>



default: `"1500000"`  
flags: `0x2`  
</details>
<details>
<summary><code>staticProp_earlyDepthPrepassIncludeOpaques</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>staticProp_earlyDepthPrepassIncludeOpaquesDist</code></summary>



default: `"1000"`  
flags: `0x2`  
</details>
<details>
<summary><code>staticProp_gather_size_weight</code></summary>



default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>staticProp_max_scaled_dist</code></summary>



default: `"2500.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>staticProp_no_fade_scalar</code></summary>



default: `"0.7"`  
flags: `0x2`  
</details>
<details>
<summary><code>staticProp_refineDrawOnWorker</code></summary>

0 - none, 1 - mainview, 2 - depth-prepass, 3 - mainview & depth-prepass

default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>static_shadow</code></summary>

0 : off, 1 : generate once, 2 : minimum update with cache, 3 : update dirty rects with cache, 4 : update dirty rects without cache, 5 : always refresh

default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>static_shadow</code></summary>

0 : off, 1 : generate once, 2 : minimum update with cache, 3 : update dirty rects with cache, 4 : update dirty rects without cache, 5 : always refresh

default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>static_shadow_bounds_per_env</code></summary>

0 - use world min/max, 1 - use current light environment's head box

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>static_shadow_debug_2d</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>static_shadow_debug_dirty_rects</code></summary>

only works with static_shadow_debug_2d = 1

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>static_shadow_depth_bias_scale</code></summary>

only effective on materials with non-zero shadowBiasStatic values

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>static_shadow_expand_z</code></summary>

z range should be inflated to be able to cover flying objects higher than world min/max

default: `"30000"`  
flags: `0x2`  
</details>
<details>
<summary><code>static_shadow_good_merge_ratio</code></summary>

merge ratio = merged extent / bigger one's extent. when merge ratio is less than this value, the pair can be merged even when merged extent > m_StaticShadowMaxExtentForDirtyRect

default: `"1.01"`  
flags: `0x2`  
</details>
<details>
<summary><code>static_shadow_good_merge_score</code></summary>

score = merged extent + wasted extent, where 4.0 is full screen

default: `"0.1"`  
flags: `0x2`  
</details>
<details>
<summary><code>static_shadow_prop_min_size</code></summary>

Minimum size of prop to be drawn in static shadow

default: `"40.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>static_shadow_res</code></summary>

Set the static shadow maps rendertarget resolution

default: `"4096"`  
flags: `0x2`  
</details>
<details>
<summary><code>static_shadow_shrink_culler</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>static_shadow_use_d16</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>static_shadow_uses_shadow_lod</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>staticfile_hostname</code></summary>



default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>stats_hostname</code></summary>



default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>status_effect_warning_level</code></summary>

Set to 0 for nothing, 1 for warnings, 2 for script errors

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>steam_id</code></summary>



default: `""`  
flags: `0x12`  
</details>
<details>
<summary><code>steam_name</code></summary>



default: `""`  
flags: `0x12`  
</details>
<details>
<summary><code>steamlink_hostname</code></summary>



default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_addnoise</code></summary>

Adds corruption to streamed-in MIP levels for debugging.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_bsp_bucket_bias</code></summary>

Tweak MIP of BSP coverage (higher = blurrier mips)

default: `"-0.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_bsp_dist_scale</code></summary>

Scale BSP coverage (relative to models) (higher = more important)

default: `"100"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_cache_capacity</code></summary>

Stream Cache Capacity in MiB

default: `"400"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_cache_capacity_while_loading</code></summary>

Stream Cache Capacity in MiB while loading, if < 0 will be ignored.

default: `"200"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_cache_high_priority_static_models</code></summary>

Try never to drop (and always prioritize loading) static model geo.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_cache_multithreaded</code></summary>

Use jobs to do upload for model geo.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_cache_preload_from_rpak</code></summary>

0 = Never preload; 1 = Preload static models

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_cache_read_buffer_cap</code></summary>

Concurrent read buffer capacity in MiB.

default: `"32"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_cache_read_count_cap</code></summary>

Concurrent read limit.

default: `"24"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_cache_speculative_add_level</code></summary>

Attempt to add models to reach this fraction of stream_cache_capacity.

default: `"0.75"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_cache_speculative_drop</code></summary>

Attempt to drop models to reach this fraction of stream_cache_capacity.

default: `"0.9"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_drop_unused</code></summary>

Drop unused textures aggressively

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_enable</code></summary>

Enable texture streaming

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_freeze_camera</code></summary>

Freezes camera for purposes of streaming map textures.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_load_after_drop</code></summary>

Allow us to continue loading in a frame after dropping any textures.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_memory</code></summary>

Stream memory to target (in kb).

default: `"300000"`  
flags: `0x40000000`  
</details>
<details>
<summary><code>stream_memory_ignore</code></summary>

Ignore stream_memory limit when streaming is enabled.

default: `"0"`  
flags: `0x40000002`  
</details>
<details>
<summary><code>stream_memory_ignore_vram</code></summary>

Ignore vram size when setting streaming buffer size.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_memory_while_loading</code></summary>

Stream memory to target (in kb).

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_mode</code></summary>

Stream mode: default all none

default: `"default"`  
flags: `0x40000002`  
</details>
<details>
<summary><code>stream_never_high_priority_frac</code></summary>

Never assign 'high priority' to a texture that uses more than this fraction of total streaming buffer.

default: `"0.0125"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_overlay</code></summary>

Texture streaming debug overlay.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_overlay_mode</code></summary>

Which debug view to show (tex mtl bsp short)

default: `"short"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_pause</code></summary>

Pause texture streaming

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_picmip</code></summary>

Picmip used when stream mode is picmip. (Or the map doesn't have streaming data.)

default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_resource_max_commits_per_frame</code></summary>

Cap on number of streaming texture commits allowed in a GPU frame. (0 disables cap)

default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_resource_thread</code></summary>

Create resources on separate thread, and delay copy and binding of those resources.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_resource_wait_copy_to_commit</code></summary>

Number of frames to wait between copying old texture data and actually using a new texture.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_resource_wait_creation_to_copy</code></summary>

Number of frames to wait between creating a texture and copying old texture data in.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>stream_resource_wait_for_additional_gpus</code></summary>

Enable to reset the commit counter less frequently when you have multiple GPUs.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>stringtable_alwaysrebuilddictionaries</code></summary>

Rebuild dictionary file on every level load


default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>stringtable_compress</code></summary>

Compress string table for networking


default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>stringtable_showsizes</code></summary>

Show sizes of string tables when building for signon


default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>stryder_forceOriginUsersInvisible</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>stryder_security</code></summary>



default: `""`  
flags: `0x80000200`  
</details>
<details>
<summary><code>stuck_debugging</code></summary>

Debug getting stuck

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>stuck_debugging_world_only</code></summary>

Only check for stuck in world geo

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>studiobonecache_unlimited</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>subscription_hostname</code></summary>



default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>superjump_disabled_from_water</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>superjump_drain_power_onfail</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>superjump_fail_sound_when_jump_limit</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>superjump_limit</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>superjump_limitreset_onwallrun</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>superjump_max_power_use</code></summary>



default: `"100"`  
flags: `0x2002`  
</details>
<details>
<summary><code>superjump_min_height_fraction</code></summary>

Minimum fraction of desired superjump height that is acheived, even if already moving quickly upwards

default: `"0.25"`  
flags: `0x2002`  
</details>
<details>
<summary><code>superjump_min_power_use</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>superjump_powerreset_onground</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_airaccelerate</code></summary>



default: `"10"`  
flags: `0x2102`  
</details>
<details>
<summary><code>sv_allTicksFinal</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_allowSendTableTransmitToClients</code></summary>

Allow transmission of sendtable data to clients.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_allowSpectatorClients</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_asyncSendSnapshot</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_backspeed</code></summary>

How much to slow down backwards motion

default: `"0.6"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_balanceTeams</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_bounce</code></summary>

Bounce multiplier for when physically simulated objects collide with other objects.

default: `"0"`  
flags: `0x2102`  
</details>
<details>
<summary><code>sv_cheats</code></summary>

Allow cheats on server

default: `"0"`  
flags: `0x82100`  
</details>
<details>
<summary><code>sv_checkPropBudgets</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_compressPlaylists</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_compressTimeValEpsilon</code></summary>



default: `"0.0005"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_compressTimeVals</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_connectingClientDelay</code></summary>

Amount of time to wait between resends of data to a connecting client

default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_debug_prop_send</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_debugmanualmode</code></summary>

Make sure entities correctly report whether or not their network data has changed.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_disconnectOnScriptError</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_disconnectOnTooManySnapshotFrames</code></summary>

Disconnect client when the server has sent 128 snapshot messages to client without the server getting any message from the client.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_dumpstringtables</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>sv_earlyPersistenceRead</code></summary>

Should the server try to read persistence earlier in the connection process


default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_everyThirdTick</code></summary>

Do networking every third tick, regardless of how backed up we are


default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_extra_client_connect_time</code></summary>

Seconds after client connect during which extra frames are buffered to prevent non-delta'd update

default: `"60.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_fakeClientBaseId</code></summary>

Base platform user ID for created fake clients. Useful, for example, when running multiple dedis with matchmaking bots; create a different base for each dedi to get unique IDs -- matchmaking uses user IDs as a primary key.

default: `"9990000"`  
flags: `0x26`  
</details>
<details>
<summary><code>sv_footsteps</code></summary>

Play footstep sound for players

default: `"1"`  
flags: `0x2102`  
</details>
<details>
<summary><code>sv_friction</code></summary>

World friction. (Equivalent player setting is in player settings files)

default: `"4"`  
flags: `0x2102`  
</details>
<details>
<summary><code>sv_gravity</code></summary>

World gravity.

default: `"750"`  
flags: `0x2102`  
</details>
<details>
<summary><code>sv_hibernate_ms</code></summary>

# of milliseconds to sleep per frame while hibernating

default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_hibernate_ms_vgui</code></summary>

# of milliseconds to sleep per frame while hibernating but running the vgui dedicated server frontend

default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_hibernate_postgame_delay</code></summary>

# of seconds to wait after final client leaves before hibernating.

default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_hibernate_when_empty</code></summary>

Puts the server into extremely low CPU usage mode when no clients connected

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_instancebaselines</code></summary>

Enable instanced baselines. Saves network overhead.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_loadMapModelEarly</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_lobbyType</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_max_prop_data_dwords_huge_lobby</code></summary>

Maximum amount of prop data per-snapshot in dwords (huge lobby)

default: `"300000"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_max_prop_data_dwords_huge_multiplayer</code></summary>

Maximum amount of prop data per-snapshot in dwords (huge multiplayer)

default: `"2500000"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_max_prop_data_dwords_lobby</code></summary>

Maximum amount of prop data per-snapshot in dwords (lobby)

default: `"200000"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_max_prop_data_dwords_multiplayer</code></summary>

Maximum amount of prop data per-snapshot in dwords (normal multiplayer)

default: `"150500"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_max_prop_data_dwords_singleplayer</code></summary>

Maximum amount of prop data per-snapshot in dwords (singleplayer)

default: `"400000"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_max_props_huge_lobby</code></summary>

Maximum amount of props per-snapshot (lobby)

default: `"250000"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_max_props_huge_multiplayer</code></summary>

Maximum number of props per-snapshot (huge multiplayer)

default: `"1250000"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_max_props_lobby</code></summary>

Maximum amount of props per-snapshot (lobby)

default: `"50000"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_max_props_multiplayer</code></summary>

Maximum number of props per-snapshot (normal multiplayer)

default: `"100000"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_max_props_singleplayer</code></summary>

Maximum number of props per-snapshot (singleplayer)

default: `"300000"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_max_snapshots_lobby</code></summary>

Maximum number of snapshots for the lobby

default: `"100"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_max_snapshots_multiplayer</code></summary>

Maximum number of snapshots for multiplayer levels

default: `"160"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_max_snapshots_singleplayer</code></summary>

Maximum number of snapshots for singleplayer levels

default: `"10"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_maxclientframes</code></summary>



default: `"300"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_maxrate</code></summary>

Max bandwidth rate allowed on server, 0 == unlimited

default: `"0"`  
flags: `0x82000`  
min value: `0`  
max value: `1000000000`  
</details>
<details>
<summary><code>sv_maxroutable</code></summary>

Server upper bound on net_maxroutable that a client can use.

default: `"1200"`  
flags: `0x2`  
min value: `576`  
max value: `1200`  
</details>
<details>
<summary><code>sv_maxspeed</code></summary>



default: `"320"`  
flags: `0x2102`  
</details>
<details>
<summary><code>sv_maxupdaterate</code></summary>

Maximum updates per second that the server will allow

default: `"60"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_maxvelocity</code></summary>

Maximum speed any ballistically moving object is allowed to attain per axis.

default: `"34000"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_minrate</code></summary>

Min bandwidth rate allowed on server, 0 == unlimited

default: `"128000"`  
flags: `0x82000`  
min value: `0`  
max value: `1000000000`  
</details>
<details>
<summary><code>sv_minupdaterate</code></summary>

Minimum updates per second that the server will allow

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_noclipaccelerate</code></summary>



default: `"10000"`  
flags: `0x2102`  
</details>
<details>
<summary><code>sv_noclipaccelerate_fast</code></summary>



default: `"50000"`  
flags: `0x2102`  
</details>
<details>
<summary><code>sv_noclipaccelerate_slow</code></summary>



default: `"10000"`  
flags: `0x2102`  
</details>
<details>
<summary><code>sv_noclipspeed</code></summary>



default: `"5"`  
flags: `0x2180`  
</details>
<details>
<summary><code>sv_noclipspeed_fast</code></summary>



default: `"30"`  
flags: `0x2180`  
</details>
<details>
<summary><code>sv_noclipspeed_slow</code></summary>



default: `"0.25"`  
flags: `0x2180`  
</details>
<details>
<summary><code>sv_optimizedmovement</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_parallel_sendsnapshot</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_pausable</code></summary>

Whether the server is allowed to pause

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>sv_playerNameAppendCheater</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_players</code></summary>



default: `"1"`  
flags: `0x2012`  
</details>
<details>
<summary><code>sv_printHighWaterMark</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_pushaway_accel</code></summary>

How hard physics objects are pushed away from the players.

default: `"400"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_pushaway_clientside</code></summary>

Clientside physics push away (0=off, 1=only localplayer, 2=all players)

default: `"2"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_pushaway_clientside_size</code></summary>

Physics props below this size are made client side

default: `"1000"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_pushaway_debug</code></summary>

Debug physics object pushaway

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_pushaway_dist</code></summary>

Max distance at which physics objects are pushed from players.

default: `"15"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_pushaway_min_player_speed</code></summary>

If a player is moving slower than this, don't push away physics objects (enables ducking behind things).

default: `"75"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_pushaway_player_accel</code></summary>

How hard the player is pushed away from physics objects

default: `"3000"`  
flags: `0x6002`  
</details>
<details>
<summary><code>sv_pushaway_player_dist</code></summary>

Max distance at which player is pushed from physics objects

default: `"5"`  
flags: `0x6002`  
</details>
<details>
<summary><code>sv_rejectClientConnects</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_rejectConnections</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_requireOriginToken</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_resendSignonData</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_rollangle</code></summary>

Max view roll angle

default: `"0"`  
flags: `0x2102`  
</details>
<details>
<summary><code>sv_rollspeed</code></summary>



default: `"200"`  
flags: `0x2102`  
</details>
<details>
<summary><code>sv_runSpatialOptimizeInJob</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_scarySnapDeltaPrints</code></summary>



default: `"50"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_sendEarlyServerInfo</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_sendReplayNetMessagesOnNoDeltaSnaps</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_separate_freq_change_prop_send</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_showClientTickCmds</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_showLargeSnapshotSize</code></summary>



default: `"10000"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_showSnapshots</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_showUserCmds</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_single_core_dedi</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_skipSendingUnnecessaryPersistence</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_skyname</code></summary>

Current name of the skybox texture

default: `""`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_snapshot_uniform_interval</code></summary>

A snapshot is created at uniform intervals, rather than according to final_tick

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_specaccelerate</code></summary>



default: `"1000.0"`  
flags: `0x2180`  
</details>
<details>
<summary><code>sv_specnoclip</code></summary>



default: `"1"`  
flags: `0x2180`  
</details>
<details>
<summary><code>sv_specspeed</code></summary>



default: `"5.0"`  
flags: `0x2180`  
</details>
<details>
<summary><code>sv_stats</code></summary>

Collect CPU usage stats

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_stopspeed</code></summary>

Minimum stopping speed when on ground. (Equivalent player setting is in player settings files)

default: `"100"`  
flags: `0x2102`  
</details>
<details>
<summary><code>sv_stressbots</code></summary>

If set to 1, the server calculates data and fills packets to bots. Used for perf testing.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_struggleCheck</code></summary>

How long ago the 60th server frame can have been. 1.0 means the server is running in realtime. Higher means small hitches are ok.

default: `"1.016"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_struggleSpam</code></summary>

How long ago the 60th server frame can have been before it starts yelling. 1.0 means the server is running in realtime. Higher means small hitches are ok.

default: `"1.4"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_struggleSpamInterval</code></summary>



default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_tempents_send_from_delta</code></summary>

Causes snapshot send code to walk back to delta, instead of always sending just current snapshot.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_tempents_send_from_last_sent</code></summary>

Causes snapshot send code to walk back to last m_lastSnapshotTick, instead of always sending just current snapshot.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_testLargeDatablock</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_teststepsimulation</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_transmitToAllPlayersMask_allBitsSet</code></summary>

This enables the legacy behavior of setting all bits inside of PerPlayerBitMask when we want to transmit an entities to all clients. This includes setting bits to clients that can't even exist(compare GetMaxClients to ABSOLUTE_PLAYER_LIMIT)

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_unnecessaryConnectDelay</code></summary>

Amount of time to wait before responding to a connecting client (or malicious hacker)

default: `"60"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_unreliableSnapMaxSize</code></summary>

If we're sending a snapshot this size or larger, send it via the datablock sender. If a player has 4% packet loss, 10k of data would have a 40% chance of making it across with no resends

default: `"10000"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_updaterate_mp</code></summary>

Maximum update rate at which server sends packets to clients in MP (updates per-second).

default: `"20"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_updaterate_sp</code></summary>

Maximum update rate at which server sends packets to clients in SP (updates per-second).

default: `"20"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_useRK4forprojectiles</code></summary>

Use the RK4 method for projectiles rather than simple average velocity.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_useReputation</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_useThreadsForSnapshots</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_voiceDebug</code></summary>

Show who is getting each voice packet from the server

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_voiceEcho</code></summary>

Server will return a voice chat message back to the sending client.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_voiceenable</code></summary>



default: `"1"`  
flags: `0x80180`  
</details>
<details>
<summary><code>sv_warnAboutCmdNumJumps</code></summary>



default: `"20"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_watchdogTimer</code></summary>



default: `"20"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_wateraccelerate</code></summary>



default: `"10"`  
flags: `0x2102`  
</details>
<details>
<summary><code>sv_waterdist</code></summary>

Vertical view fixup when eyes are near water plane.

default: `"12"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_writePersistenceOnShutdown</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sys_attract_mode_timeout</code></summary>



default: `"30"`  
flags: `0x2`  
</details>
<details>
<summary><code>system_alt_f4_closes_window</code></summary>

If set to true, alt+f4 will close the window

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>teams_unassigned_are_friendly</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>telemetry_client_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>telemetry_client_enable</code></summary>

Enable sending telemetry data

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>telemetry_client_sendInterval</code></summary>

How often to send telemetry data (seconds)

default: `"10.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>test_fakeTimeDays</code></summary>

Days worth of seconds that will be added to the result of GetUnixTimestamp(). Server authoritive.

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>tether_damageScale</code></summary>

amount that stretching the tether damages it

default: `"0.00"`  
flags: `0x2002`  
</details>
<details>
<summary><code>tether_dodge_damage</code></summary>

Damage done to tether by dodging away from it

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>tether_healthDrain</code></summary>

rate at which tether health drains even if it isn't stretched

default: `"200"`  
flags: `0x2002`  
</details>
<details>
<summary><code>tether_healthDrainNPC</code></summary>

rate at which tether health drains even if it isn't stretched (when attached to an NPC)

default: `"200"`  
flags: `0x2002`  
</details>
<details>
<summary><code>tether_maxvel</code></summary>

max velocity with which tether pulls you back

default: `"200"`  
flags: `0x2002`  
</details>
<details>
<summary><code>tether_radius</code></summary>

radius below which the tether does nothing

default: `"250"`  
flags: `0x2002`  
</details>
<details>
<summary><code>tether_strength</code></summary>

strength with which tether pulls back (per unit past the radius)

default: `"25"`  
flags: `0x2002`  
</details>
<details>
<summary><code>think_limit</code></summary>

Maximum think time in milliseconds, warning is printed if this is exceeded.

default: `"10"`  
flags: `0x82000`  
</details>
<details>
<summary><code>thirdperson_mayamode</code></summary>

Set to 1 to enable maya-like controls in game (only in third person) [Also don't move the camera when the mouse moves.]

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>thirdperson_override</code></summary>

Set to -1 to stop overriding. Set to 0 to force first person, 1 to force third person

default: `"-1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>thirdperson_screenspace</code></summary>

Movement will be relative to the camera, eg: left means screen-left

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>timeout</code></summary>

Seconds without communication before clients or servers will decide to disconnect.

default: `"15"`  
flags: `0x2`  
</details>
<details>
<summary><code>timeout_during_load</code></summary>

Seconds without communication during a level load before clients or servers will decide to disconnect.

default: `"60"`  
flags: `0x2`  
</details>
<details>
<summary><code>titan_sprint_sound</code></summary>



default: `"titan_eject_servos_3p"`  
flags: `0x2`  
</details>
<details>
<summary><code>tracehull_height_error_check</code></summary>

Error checking for hull traces requiring extents with larger heights than widths. 0 = none, 1 = warnings, 2 = assert and script errors

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>tracer_debug</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>tracer_extra</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>trail_optimizedRemove</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>traversal_anim</code></summary>

Enables automantle animation

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>traversal_cooldown</code></summary>

Minimum time between traversals (in seconds)

default: `"0.5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>traversal_enable</code></summary>

Enables player traversals

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>traversal_hand_debug</code></summary>

Enables debugging of traversal hand positioning

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>traversal_hand_required_width</code></summary>

Required width of geometry for hands (from center)

default: `"6"`  
flags: `0x2002`  
</details>
<details>
<summary><code>traversal_viewLerpInDuration</code></summary>

Duration of view lerp from normal at the start of a traversal

default: `"0.15"`  
flags: `0x2002`  
</details>
<details>
<summary><code>traversal_viewLerpOut</code></summary>

Controls whether traversal view position and angle lerp back to normal at the end of a traversal

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>traversal_viewLerpOutAngle</code></summary>

Controls whether traversal view angle lerps back to normal at the end of a traversal

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>traversal_viewLerpOutDebug</code></summary>

Debugs traversal view position lerping

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>traversal_viewLerpOutPos</code></summary>

Controls whether traversal view position lerps back to normal at the end of a traversal

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>traversal_window_duration</code></summary>

Duration of window side traversal animation

default: `"0.3"`  
flags: `0x2002`  
</details>
<details>
<summary><code>traversal_window_enable</code></summary>

Enables window traversals

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>traversal_window_finish_angle</code></summary>

Finishing yaw relative to the window's forward direction when starting at a 90 degree angle

default: `"45"`  
flags: `0x2002`  
</details>
<details>
<summary><code>traversal_window_forward_offset</code></summary>

Distance of player through the window after completing window traversal

default: `"6"`  
flags: `0x2002`  
</details>
<details>
<summary><code>traversal_window_hand_vertical_offset</code></summary>

Vertical distance from hand position to eye position at start and end of window traversal

default: `"22"`  
flags: `0x2002`  
</details>
<details>
<summary><code>traversal_window_sideways_offset</code></summary>

Distance of player from the edge of the window toward the center of the window after completing window traversal

default: `"18"`  
flags: `0x2002`  
</details>
<details>
<summary><code>traversal_window_view_pitch_max</code></summary>

Max view pitch when doing window traversal

default: `"35"`  
flags: `0x4000`  
</details>
<details>
<summary><code>traversal_window_view_pitch_min</code></summary>

Min view pitch when doing window traversal

default: `"-80"`  
flags: `0x4000`  
</details>
<details>
<summary><code>traversal_window_yaw_max</code></summary>

Max view yaw when doing window traversal

default: `"80"`  
flags: `0x4000`  
</details>
<details>
<summary><code>trigger_ignore_nonsolids</code></summary>

If set to false, non solid objects will activate triggers.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>tsaa_blendfactorincreaseatmaxvelocity</code></summary>



default: `"4.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>tsaa_blendfactorincreasewhenunoccluded</code></summary>



default: `"5.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>tsaa_blendfactormaxesoutatvelocity</code></summary>



default: `"0.25"`  
flags: `0x2`  
</details>
<details>
<summary><code>tsaa_blendfactormodulationonsparklesandunocclusion</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>tsaa_blendfactoroverride</code></summary>



default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>tsaa_curframeblendamount</code></summary>



default: `"0.05"`  
flags: `0x2`  
</details>
<details>
<summary><code>tsaa_debugresponsiveflag</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>tsaa_neighborhoodclamping</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>tsaa_neighborhoodclampingsoftened</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>tsaa_numsamples</code></summary>



default: `"64"`  
flags: `0x2`  
</details>
<details>
<summary><code>tweak_light_shadows_every_frame</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>twitch_check_interval</code></summary>

how often we ask if this user has a linked twitch prime account if we think they don't have one

default: `"3600"`  
flags: `0x2`  
</details>
<details>
<summary><code>twitch_prime_rewards</code></summary>



default: `""`  
flags: `0x210`  
</details>
<details>
<summary><code>twitch_shouldQuery</code></summary>

true if we should check to see if this user has a linked twitch prime account

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>ui_fadecloud_time</code></summary>



default: `"1.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>ui_fadexui_time</code></summary>



default: `"0.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>ui_gameui_ctrlr_title</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ui_gameui_modal</code></summary>

If set, the game UI pages will take modal input focus.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ui_loadingscreen_autotransition_time</code></summary>



default: `"5.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ui_loadingscreen_fadein_time</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ui_loadingscreen_fadeout_time</code></summary>



default: `"0.2"`  
flags: `0x2`  
</details>
<details>
<summary><code>ui_loadingscreen_fadeout_time</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ui_loadingscreen_mintransition_time</code></summary>



default: `"0.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>ui_loadingscreen_transition_time</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ui_lobby_jointimeout</code></summary>



default: `"75"`  
flags: `0x2`  
</details>
<details>
<summary><code>ui_lobby_noautostart</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ui_lobby_noresults_create_msg_time</code></summary>



default: `"2.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>ui_posedebug_fade_in_time</code></summary>

Time during which a new pose activity layer is shown in green in +posedebug UI

default: `"0.2"`  
flags: `0x24000`  
</details>
<details>
<summary><code>ui_posedebug_fade_out_time</code></summary>

Time to keep a no longer active pose activity layer in red until removing it from +posedebug UI

default: `"0.8"`  
flags: `0x24000`  
</details>
<details>
<summary><code>ui_virtualnav_render</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>unique_entity_names</code></summary>

Should entities have permanently unique entity names.  Or just concurrently unique?

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>usePromptBaseColor</code></summary>



default: `"255 255 255 255"`  
flags: `0x2`  
</details>
<details>
<summary><code>usePromptButtonTextColor</code></summary>



default: `"255 255 255 255"`  
flags: `0x2`  
</details>
<details>
<summary><code>usePromptImageScale</code></summary>



default: `"1.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>usePromptImageYOffset</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>usePromptTextColor</code></summary>



default: `"220 215 210 255"`  
flags: `0x2`  
</details>
<details>
<summary><code>use_monitors</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>use_valve_auto_gain</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>user_tracking_enabled</code></summary>



default: `"0"`  
flags: `0x12`  
</details>
<details>
<summary><code>users_hostname</code></summary>



default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>v_centermove</code></summary>



default: `"0.15"`  
flags: `0x2`  
</details>
<details>
<summary><code>v_centerspeed</code></summary>



default: `"500"`  
flags: `0x2`  
</details>
<details>
<summary><code>variable_sights_gravity_scale_override</code></summary>

Projectile Gravity Scale to be used for variable sights.

default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vehicle_predictViaPlayer</code></summary>

Predict this vehicle if the player's data says they're driving this vehicle...rather than checking if this vehicle has a driver

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vgui_EnableFixedAspectScaling</code></summary>

Enables fixed screen size for vgui elements

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>vgui_drawPolyShapes</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vgui_drawfocus</code></summary>

Report which panel is under the mouse.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vgui_drawfocus</code></summary>

Report which panel is under the mouse.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vgui_drawkeyfocus</code></summary>

Report which panel has keyboard focus.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vgui_drawtree</code></summary>

Draws the vgui panel hiearchy to the specified depth level.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vgui_drawtree_bounds</code></summary>

Show panel bounds.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vgui_drawtree_draw_selected</code></summary>

Highlight the selected panel

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vgui_drawtree_freeze</code></summary>

Set to 1 to stop updating the vgui_drawtree view.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vgui_drawtree_hidden</code></summary>

Draw the hidden panels.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vgui_drawtree_panelalpha</code></summary>

Show the panel alpha values in the vgui_drawtree view.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vgui_drawtree_panelptr</code></summary>

Show the panel pointer values in the vgui_drawtree view.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vgui_drawtree_popupsonly</code></summary>

Draws the vgui popup list in hierarchy(1) or most recently used(2) order.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vgui_drawtree_render_order</code></summary>

List the vgui_drawtree panels in render order.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vgui_drawtree_scheme</code></summary>

Show scheme file for each panel

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vgui_drawtree_visible</code></summary>

Draw the visible panels.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>vgui_interactive</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vgui_noquads</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>vgui_notext</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>vgui_resize_on_resolution_change</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vgui_show_glyph_miss</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vgui_simulate_during_bone_setup</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>video_menu_uiscript_reset</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>viewDrift</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_ads_delay_debounce_time</code></summary>

Time between zoom-out and zoom-in before viewdrift_ads_delay is reset.

default: `"0.5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_pitch_base1_amp</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_pitch_base1_freq</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_pitch_base1_phase</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_pitch_base2_amp</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_pitch_base2_freq</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_pitch_base2_phase</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_pitch_scaler_amp</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_pitch_scaler_base</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_pitch_scaler_freq</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_pitch_scaler_phase</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_pitch_shifter_amp</code></summary>



default: `"0.6"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_pitch_shifter_freq</code></summary>



default: `"2.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_pitch_shifter_phase</code></summary>



default: `"1.6"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_yaw_base1_amp</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_yaw_base1_freq</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_yaw_base1_phase</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_yaw_base2_amp</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_yaw_base2_freq</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_yaw_base2_phase</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_yaw_scaler_amp</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_yaw_scaler_base</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_yaw_scaler_freq</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_yaw_scaler_phase</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_yaw_shifter_amp</code></summary>



default: `"0.7"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_yaw_shifter_freq</code></summary>



default: `"1.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewDrift_yaw_shifter_phase</code></summary>



default: `"-0.6"`  
flags: `0x2002`  
</details>
<details>
<summary><code>view_offset_entity_enable</code></summary>

Whether to apply camera animations from the view offset entity

default: `"1"`  
flags: `0x6000`  
</details>
<details>
<summary><code>viewangle_debug</code></summary>



default: `"0"`  
flags: `0x4002`  
</details>
<details>
<summary><code>viewangles_simpler</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>viewmodelShake</code></summary>

Enables viewmodel shake.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>viewmodelShake_sourceRollRange</code></summary>

The range of weapon kick roll that will be sampled for viewmodel shake.

default: `"3"`  
flags: `0x2`  
</details>
<details>
<summary><code>viewmodel_attachment_fov_fix</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>viewmodel_bounds_draw</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>viewmodel_bounds_draw_lock</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>viewmodel_selfshadow</code></summary>

Set whether to use viewmodel self shadow

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>viewmodel_selfshadow_debug_2d</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>viewmodel_selfshadow_tightbounds</code></summary>

Viewmodel bounds are sliced by Main view frustum

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>viewportscale</code></summary>

Scale down the main viewport (to reduce GPU impact on CPU profiling)

default: `"1.0"`  
flags: `0x4000`  
min value: `0.0015625`  
max value: `2`  
</details>
<details>
<summary><code>viewpunch_base_springConstantX</code></summary>

Default. Bigger number increases the speed at which the view corrects.

default: `"65.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewpunch_base_springConstantY</code></summary>

Default. Bigger number increases the speed at which the view corrects.

default: `"65.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewpunch_base_springConstantZ</code></summary>

Default. Bigger number increases the speed at which the view corrects.

default: `"65.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewpunch_base_springDampingX</code></summary>

Default. Bigger number makes the response more damped.

default: `"9.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewpunch_base_springDampingY</code></summary>

Default. Bigger number makes the response more damped.

default: `"9.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>viewpunch_base_springDampingZ</code></summary>

Default. Bigger number makes the response more damped.

default: `"9.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>violence_ablood</code></summary>

Draw alien blood

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>violence_ablood</code></summary>

Draw alien blood

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>violence_agibs</code></summary>

Show alien gib entities

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>violence_agibs</code></summary>

Show alien gib entities

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>violence_hblood</code></summary>

Draw human blood

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>violence_hblood</code></summary>

Draw human blood

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>violence_hgibs</code></summary>

Show human gib entities

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>violence_hgibs</code></summary>

Show human gib entities

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>visible_ent_cone_debug_duration_client</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_absTriggerAmount</code></summary>



default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_allow_mute_self</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_avggain</code></summary>



default: `"0.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_clientdebug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_debugAddSecondTalker</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_debugThresholds</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_debugfeedback</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_decimate_at_bytes</code></summary>



default: `"22050"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_decimate_rate</code></summary>



default: `"10"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_enabled</code></summary>

Toggle voice transmit and receive.

default: `"1"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>voice_energyPerZeroThreshold</code></summary>



default: `"8000"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_energyThreshold</code></summary>



default: `"12000"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_forcemicrecord</code></summary>



default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>voice_inputfromfile</code></summary>

Get voice input from 'voice_input.wav' rather than from the microphone.

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>voice_late_update</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_loopback</code></summary>



default: `"0"`  
flags: `0x200`  
</details>
<details>
<summary><code>voice_maxgain</code></summary>



default: `"10"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_minEnergyPerZeroThreshold</code></summary>



default: `"1000"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_mixer_boost</code></summary>



default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>voice_mixer_mute</code></summary>



default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>voice_mixer_volume</code></summary>



default: `"1.0"`  
flags: `0x80`  
</details>
<details>
<summary><code>voice_modenable</code></summary>

Enable/disable voice in this mod.

default: `"1"`  
flags: `0x40000080`  
</details>
<details>
<summary><code>voice_noxplat</code></summary>

Only send voice data to players on the same platform as the talker

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_profile</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_recordtofile</code></summary>

Record mic data and decompressed voice data into 'voice_micdata.wav' and 'voice_decompressed.wav'

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>voice_scale</code></summary>



default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>voice_showchannels</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_showincoming</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_threshold_delay</code></summary>



default: `"0.3"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_triggerCrossingRate</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_triggerRate</code></summary>



default: `"50"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_vox</code></summary>

Voice chat uses a vox-style always on

default: `"1"`  
flags: `0x80`  
max value: `1`  
</details>
<details>
<summary><code>voice_writevoices</code></summary>

Saves each speaker's voice data into separate .wav files


default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_xsend_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>voice_zeroCrossingThreshold</code></summary>



default: `"0.02"`  
flags: `0x2`  
</details>
<details>
<summary><code>vortex_damageimpulsescale</code></summary>

Scales impulse force from bullets when using the vortex

default: `"0.5"`  
flags: `0x6000`  
</details>
<details>
<summary><code>vprof_server_spike_threshold</code></summary>



default: `"999.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vprof_server_thread</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vscript_ui_do_delay_init</code></summary>



default: `"1"`  
flags: `0x12`  
</details>
<details>
<summary><code>vsm_culling</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>vsm_ignore_edge_planes</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vsm_ignore_face_planes</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vx_do_not_throttle_events</code></summary>

Force VXConsole updates every frame; smoother vprof data but at a slight (~0.2ms) perf cost.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>wall_climb_pose_paramteter_hands_enabled</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallclimb_vertical_gain_reduction</code></summary>

Amount of height the player loses when falling off a wall climb that can't be regained by future wall climbs before touching the ground.

default: `"128"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_angleChangeMinCos</code></summary>

Cosine of maximum angle the wall can change away from you without falling off

default: `"0.8"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_avoid_wall_top_decel</code></summary>

Deceleration applied to prevent the player from wall running too close to the top of a wall and falling off

default: `"3000"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_curveDebug</code></summary>

Draws debugging information for wallrun curves

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_curveEnable</code></summary>

Enables usage of wallrun curve hints

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_debug</code></summary>

Shows wall run debug info

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_enable</code></summary>

Enables wall running

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_fallAwaySpeed</code></summary>

Velocity away from the wall when falling off

default: `"70.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_hangStopTime</code></summary>

Length of time to come to a stop when zooming

default: `"0.5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_hangslipduration</code></summary>

Time it takes for slipping to become completely gravity based

default: `"1.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_hangslipstarttime</code></summary>

Time wall hanging before you start to slip down

default: `"3.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_hangslipvel</code></summary>

Impulse downward when slipping starts while wall hanging

default: `"70"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_maxViewTilt</code></summary>

Amount of roll applied to the view in degrees while wall running

default: `"10.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_minAngle_air</code></summary>

Angle at which you can start wall running when hitting a wall from a jump (0 to 180)

default: `"180.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_noInputSlipFrac</code></summary>

Min fraction of slip behavior when not pushing in any direction (applies more gravity)

default: `"0.7"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_pushAwayFallOffTime</code></summary>

Pushing away from the wall for this many seconds causes you to fall off

default: `"0.05"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_repelEnable</code></summary>

Enables repelling players from walls they have jumped off of

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_repelSoftness</code></summary>

Softness of wall jump repel: higher values make it easier for players to reduce their speed away from the wall

default: `"5.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_repelTimeMax</code></summary>

Time after jumping off the wall that player is no longer repelled from the wall

default: `"0.4"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_repelTimeMin</code></summary>

Time after jumping off the wall that player is repelled from the wall

default: `"0.2"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_retry_interval</code></summary>

Length of time between checking for the ability to wallrun after hitting a wall in air movement

default: `"0.07"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_rotateMaxRate</code></summary>

Maximum rotation speed around a wall in radians per second; avoids sticking to walls that do tight curves

default: `"3"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_sameWallDist</code></summary>

Within this distance of the previous wall run, wall run is prevented at a higher point on the same wall

default: `"100"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_sameWallDot</code></summary>

Dot product threshold for preventing wall running on the same wall twice

default: `"0.9"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_sameWallSlope</code></summary>

Beyond wallrun_samewalldist, wall running is permitted at higher points with this slope

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_slipduration</code></summary>

Time it takes for slipping to become completely gravity based

default: `"1.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_slipslowdown</code></summary>

Fraction of velocity lost when slipping starts

default: `"0.5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_slipstarttime</code></summary>

Time wall running before you start to slip down

default: `"1.5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_slipvel</code></summary>

Impulse downward when slipping starts

default: `"70"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_strengthLossEnd</code></summary>

Number of wall runs at which point upward strength is fully lost (scales upWallBoost, jumpUpSpeed, and gravityRampUpTime to zero)

default: `"1000"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_strengthLossStart</code></summary>

Number of wall runs allowed before starting to lose upward strength (scales upWallBoost, jumpUpSpeed, and gravityRampUpTime)

default: `"1000"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_upwardAutoPush</code></summary>

The amount of automatic up-the-wall input applied when the player pushes forward along the wall. Helps to fight gravity when pushing forward.

default: `"0.65"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_viewTiltPredictTime</code></summary>

Time before you start wall running where your view starts tilting. Predicts upcoming wall running

default: `"0.25"`  
flags: `0x2002`  
</details>
<details>
<summary><code>wallrun_viewTiltSpeed</code></summary>

Speed at which the view tilts while wall running

default: `"6.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>was_loaded</code></summary>

Current game from a restore?

default: `""`  
flags: `0x12002`  
</details>
<details>
<summary><code>weaponFastHolsterScale</code></summary>

Scales holster animations if swapping to a weapon with "fast_swap_to" enabled.

default: `"0.25"`  
flags: `0x2002`  
</details>
<details>
<summary><code>weaponSwitch3p_checkNewWeapon</code></summary>

Only play 3p weapon switch if there is a new weapon.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>weaponSwitch3p_onHolster</code></summary>

Start third person weapon switch animation as soon as the current weapon starts being holstered.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>weapon_auto_swap_ordnance_no_ammo</code></summary>

If you touch a new ordnance weapon with no ammo in your current it will auto replace it

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>weapon_debugScript</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>weapon_doIdleForSurvivalMelee</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>weapon_friendly_fire_prevent_ui</code></summary>

UI to show on friendly fire prevention

default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>weapon_meleeButtonPressProtection</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>weapon_parentingFixLerp</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>weapon_pickup_allow_dupes</code></summary>

Whether or not you are allowed 2 of the same weapon in your inventory

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>weapon_poseParamMaxDistance</code></summary>



default: `"6000"`  
flags: `0x2`  
</details>
<details>
<summary><code>weapon_render_with_fastpath</code></summary>

Allow weapons to draw using the fast path.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>weapon_setting_autocycle_on_empty</code></summary>



default: `"1"`  
flags: `0x41000200`  
</details>
<details>
<summary><code>weapon_sprint_raise_delay</code></summary>

Enables weapon delay between sprint and shooting

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>weaponx_predicting_client_only_optimization</code></summary>

Enable/disable weaponx optimization for burst fire, shot count and charge data only being sent to predicting client

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>weaponx_smartammo_data_optimization</code></summary>

Enable/disable weaponx smartammo data optimization. Only applies with net_optimize_weapons >= 2

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>window_hint_debug</code></summary>

Debugs search for window hints

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>window_hint_fov_down</code></summary>

Window hints below this vertical FOV will be ignored

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>window_hint_fov_horz</code></summary>

Window hints beyond this horizontal FOV will be ignored

default: `"60"`  
flags: `0x2002`  
</details>
<details>
<summary><code>window_hint_fov_up</code></summary>

Window hints above this vertical FOV will be ignored

default: `"60"`  
flags: `0x2002`  
</details>
<details>
<summary><code>window_hint_keyboard_fov_horz</code></summary>

Window hints beyond this horizontal FOV will be ignored

default: `"7"`  
flags: `0x2002`  
</details>
<details>
<summary><code>window_hint_lookahead_time</code></summary>

Lookahead prediction time for window checks

default: `"0.8"`  
flags: `0x2002`  
</details>
<details>
<summary><code>window_hint_max_horz_vel_change_dot</code></summary>

Min dot product of velocity change when adjusting for windows

default: `"0.966f"`  
flags: `0x2002`  
</details>
<details>
<summary><code>window_hint_max_vel_change_down</code></summary>

Max removed vertical velocity when adjusting for windows

default: `"150"`  
flags: `0x2002`  
</details>
<details>
<summary><code>window_hint_max_vel_change_up</code></summary>

Max added vertical velocity when adjusting for windows

default: `"80"`  
flags: `0x2002`  
</details>
<details>
<summary><code>window_hint_min_horz_vel</code></summary>

Horizontal velocity is increased to at least this when adjusting for windows

default: `"100"`  
flags: `0x2002`  
</details>
<details>
<summary><code>window_hint_permissive_max_horz_vel_change_dot</code></summary>

Min dot product of velocity change when adjusting for windows (off grapple)

default: `"0.88f"`  
flags: `0x2002`  
</details>
<details>
<summary><code>window_hint_permissive_max_vel_change_down</code></summary>

Max removed vertical velocity when adjusting for windows (off grapple)

default: `"300"`  
flags: `0x2002`  
</details>
<details>
<summary><code>window_hint_permissive_max_vel_change_up</code></summary>

Max added vertical velocity when adjusting for windows (off grapple)

default: `"300"`  
flags: `0x2002`  
</details>
<details>
<summary><code>z_ragdoll_impact_strength</code></summary>



default: `"500"`  
flags: `0x2`  
</details>
<details>
<summary><code>zipline_fade_dist</code></summary>



default: `"6000"`  
flags: `0x2`  
</details>
<details>
<summary><code>zipline_subdiv_lod_dist_base</code></summary>

The base distance that ziplines will begin using zipline_subdiv_slices_lod. This value is scaled by the diameter of the zipline.

default: `"150"`  
flags: `0x2`  
</details>
<details>
<summary><code>zipline_subdiv_slices</code></summary>

Zipline subdivision amount around the rope, affects roundedness.

default: `"6"`  
flags: `0x2`  
min value: `1`  
max value: `12`  
</details>
<details>
<summary><code>zipline_subdiv_slices_lod</code></summary>

Zipline subdivision amount around the rope when lod is active, affects roundedness.

default: `"4"`  
flags: `0x2`  
min value: `0`  
max value: `12`  
</details>
<details>
<summary><code>zipline_subdiv_stacks</code></summary>

Zipline subdivision amount between each zipline node.

default: `"6"`  
flags: `0x2`  
min value: `1`  
max value: `24`  
</details>

### Addresses

```
r5apex.exe!0x01e0c560 ConVar 
r5apex.exe!0x01177fd0 ConVar Allow_auto_Party
r5apex.exe!0x010347f0 ConVar BlendBonesMode
r5apex.exe!0x01d968a0 ConVar DoorSoundPrefixDouble
r5apex.exe!0x01dcb720 ConVar DoorSoundPrefixSingle
r5apex.exe!0x01dd3130 ConVar ScriptDisallowedToUsePersistenceOnSP
r5apex.exe!0x01ddb9a0 ConVar ScriptSaveAllowed
r5apex.exe!0x0104c410 ConVar StreamMicDisabled
r5apex.exe!0x0104cc30 ConVar TalkIsStream
r5apex.exe!0x0104ca50 ConVar VoiceNeedsReset
r5apex.exe!0x01dd7790 ConVar When set to 0, player always returns false when asked if it has a vehicle
r5apex.exe!0x01e00c70 ConVar ai_titan_grapple_max_len
r5apex.exe!0x01dce5b0 ConVar airslowmo_enabled
r5apex.exe!0x01dc5040 ConVar airslowmo_enter_time
r5apex.exe!0x01dc96b0 ConVar airslowmo_ground_immediate_end
r5apex.exe!0x01d98cc0 ConVar airslowmo_leave_time
r5apex.exe!0x01dca400 ConVar airslowmo_scripted_speed
r5apex.exe!0x01dce410 ConVar airslowmo_when_hovering
r5apex.exe!0x0182a9c0 ConVar animEvent_debug
r5apex.exe!0x01834500 ConVar animEvent_debugEnt
r5apex.exe!0x0183e930 ConVar animEvent_debug_cl
r5apex.exe!0x01df0430 ConVar anim_estimateVelocity
r5apex.exe!0x01df0bf0 ConVar anim_playerMovementAngleMargin
r5apex.exe!0x01df0570 ConVar anim_player_ragdoll_fix
r5apex.exe!0x01ddd380 ConVar anim_print_transition_overflow
r5apex.exe!0x01df0b50 ConVar anim_runGestureAnimEventsToCompletionOnReset_client
r5apex.exe!0x0181ba40 ConVar anim_showPoseParamErrors
r5apex.exe!0x01df02f0 ConVar anim_showstate
r5apex.exe!0x01df06b0 ConVar anim_showstatelog
r5apex.exe!0x01ded140 ConVar anim_transitionsequences
r5apex.exe!0x01ded0a0 ConVar anim_view_entity_third_person_camera_use_move_parent
r5apex.exe!0x011772d0 ConVar announcement
r5apex.exe!0x01177870 ConVar announcementImage
r5apex.exe!0x011774b0 ConVar announcementVersion
r5apex.exe!0x0114bb60 ConVar assetdownloads_desiredState
r5apex.exe!0x0114bc00 ConVar assetdownloads_enabled
r5apex.exe!0x0114bac0 ConVar assetdownloads_hostname
r5apex.exe!0x0114bca0 ConVar assetdownloads_useProdPreview
r5apex.exe!0x01049400 ConVar async_serialize
r5apex.exe!0x01dc8be0 ConVar automantle_backoff_anim_maxfrac
r5apex.exe!0x01d969e0 ConVar automantle_cooldown
r5apex.exe!0x01dc6f00 ConVar automantle_dangle_required_space
r5apex.exe!0x01d98d60 ConVar automantle_debug
r5apex.exe!0x01dc8310 ConVar automantle_enable
r5apex.exe!0x01dca4a0 ConVar automantle_forwarddist
r5apex.exe!0x01dc9ec0 ConVar automantle_gun_enable_height
r5apex.exe!0x01db44d0 ConVar automantle_height_above
r5apex.exe!0x01dcb820 ConVar automantle_height_below
r5apex.exe!0x01d9b3d0 ConVar automantle_height_level
r5apex.exe!0x01d9b9f0 ConVar automantle_jumpoff_anim_maxfrac
r5apex.exe!0x01dc8270 ConVar automantle_jumpoff_duration
r5apex.exe!0x01dcd390 ConVar automantle_max_frac
r5apex.exe!0x01dc7270 ConVar automantle_maxangle_push
r5apex.exe!0x01dca120 ConVar automantle_maxangle_view
r5apex.exe!0x01d99880 ConVar automantle_min_frac
r5apex.exe!0x01d9bb30 ConVar automantle_mindist
r5apex.exe!0x01dc9e20 ConVar automantle_rest_frac
r5apex.exe!0x01d9ba90 ConVar automantle_rest_frac_below
r5apex.exe!0x01dcf9c0 ConVar automantle_searchdist
r5apex.exe!0x0182b9c0 ConVar automantle_view_correction_speed
r5apex.exe!0x0182a0c0 ConVar automantle_view_high_yaw_max
r5apex.exe!0x0182ebf0 ConVar automantle_view_pitch_max
r5apex.exe!0x01bc20e0 ConVar automantle_view_pitch_min
r5apex.exe!0x01825e10 ConVar automantle_view_yaw_max
r5apex.exe!0x01dc9930 ConVar automantle_wallrun_maxangle_view
r5apex.exe!0x01d960b0 ConVar baseanimatingoverlay_playbackRateThreshold
r5apex.exe!0x0114b790 ConVar baselines_print
r5apex.exe!0x01dfae20 ConVar bhit_enable
r5apex.exe!0x01dffcf0 ConVar bhit_reliable
r5apex.exe!0x0104dd10 ConVar bink_materials_enabled
r5apex.exe!0x01d91930 ConVar bink_preload_videopanel_movies
r5apex.exe!0x01bcba50 ConVar boost_jetwash_prediction_factor
r5apex.exe!0x01148290 ConVar bot_lagOut
r5apex.exe!0x01039750 ConVar budget_animatingEntities
r5apex.exe!0x01037680 ConVar budget_animationOverlayEntities
r5apex.exe!0x01038b80 ConVar budget_combatCharEntities
r5apex.exe!0x01037430 ConVar budget_weaponEntities
r5apex.exe!0x01039020 ConVar budget_ziplineEntities
r5apex.exe!0x01dd2fd0 ConVar bug_reproNum
r5apex.exe!0x0103a5b0 ConVar buildcubemaps_async
r5apex.exe!0x01039ce0 ConVar buildcubemaps_index
r5apex.exe!0x01037c10 ConVar buildcubemaps_pvs_start_early
r5apex.exe!0x01039fe0 ConVar buildcubemaps_single_step
r5apex.exe!0x010390c0 ConVar building_cubemaps
r5apex.exe!0x01e046b0 ConVar bulletPredictionDebug
r5apex.exe!0x01ddbb10 ConVar bullet_trace_test_debug
r5apex.exe!0x01de1fe0 ConVar bullet_trace_test_enable
r5apex.exe!0x01bcb7f0 ConVar c_dropship_ground_fx_dist_interval
r5apex.exe!0x018324d0 ConVar c_dropship_ground_fx_time_interval
r5apex.exe!0x0182bb00 ConVar c_dropship_rope_debug
r5apex.exe!0x0183bc90 ConVar c_dropship_rope_events
r5apex.exe!0x01bbf4f0 ConVar c_dropship_rope_magnitude
r5apex.exe!0x01828ed0 ConVar c_dropship_rope_range
r5apex.exe!0x01bd08d0 ConVar c_maxdistance
r5apex.exe!0x01bcdb50 ConVar c_maxpitch
r5apex.exe!0x01bde7f0 ConVar c_maxyaw
r5apex.exe!0x01bd27c0 ConVar c_mindistance
r5apex.exe!0x01bd57a0 ConVar c_minpitch
r5apex.exe!0x01bd7640 ConVar c_minyaw
r5apex.exe!0x01bdb120 ConVar c_orthoheight
r5apex.exe!0x01bcd170 ConVar c_orthowidth
r5apex.exe!0x01be0270 ConVar c_thirdpersonshoulderaimdist_110
r5apex.exe!0x01be0310 ConVar c_thirdpersonshoulderaimdist_70
r5apex.exe!0x01be01d0 ConVar c_thirdpersonshoulderaimdist_90
r5apex.exe!0x01bdd970 ConVar c_thirdpersonshoulderdist
r5apex.exe!0x01be0130 ConVar c_thirdpersonshouldergetsviewpunch
r5apex.exe!0x01be0450 ConVar c_thirdpersonshoulderheight
r5apex.exe!0x01be03b0 ConVar c_thirdpersonshoulderoffset
r5apex.exe!0x01830e70 ConVar c_threadedAnimPostData
r5apex.exe!0x01bd1ab0 ConVar cam_collision
r5apex.exe!0x01bd8930 ConVar cam_idealdelta
r5apex.exe!0x01bcf4d0 ConVar cam_idealdist
r5apex.exe!0x01bdb000 ConVar cam_ideallag
r5apex.exe!0x01bdeb70 ConVar cam_idealpitch
r5apex.exe!0x01bdaca0 ConVar cam_idealyaw
r5apex.exe!0x01bcd710 ConVar cam_pitchLock_feetRelative
r5apex.exe!0x01bd2f60 ConVar cam_pitchlock_on
r5apex.exe!0x01bdca00 ConVar cam_pitchlock_period
r5apex.exe!0x01bcf730 ConVar cam_pitchlock_phase
r5apex.exe!0x01bdd300 ConVar cam_pitchlock_pitchBase
r5apex.exe!0x01bd00e0 ConVar cam_pitchlock_pitchRange
r5apex.exe!0x01bccf30 ConVar cam_pitchlock_pitchWiggleRoom
r5apex.exe!0x01bdff50 ConVar cam_player_viewheight_scale
r5apex.exe!0x01bd0790 ConVar cam_showangles
r5apex.exe!0x01bdcaa0 ConVar cc_captiontrace
r5apex.exe!0x01bd6770 ConVar cc_global_norepeat
r5apex.exe!0x01bdf6f0 ConVar cc_linger_time
r5apex.exe!0x01bd4410 ConVar cc_max_duration
r5apex.exe!0x01bd4970 ConVar cc_minvisibleitems
r5apex.exe!0x01bd86d0 ConVar cc_predisplay_time
r5apex.exe!0x01bdccc0 ConVar cc_rui
r5apex.exe!0x01bd32c0 ConVar cc_text_size
r5apex.exe!0x01bd1410 ConVar cc_timeshift_norepeat
r5apex.exe!0x0181b670 ConVar chasecam_distanceMax_override
r5apex.exe!0x0104cd70 ConVar chatroom_console_ptt
r5apex.exe!0x01178770 ConVar chatroom_debug
r5apex.exe!0x0117b7f0 ConVar chatroom_doRealNameLookups
r5apex.exe!0x0117bbf0 ConVar chatroom_min_status_send_interval
r5apex.exe!0x01df1860 ConVar chatroom_nameLength
r5apex.exe!0x01df2660 ConVar chatroom_namePaddingX
r5apex.exe!0x01df25c0 ConVar chatroom_nameWidth
r5apex.exe!0x0114c240 ConVar chatroom_onlyWhenActive
r5apex.exe!0x01df2200 ConVar chatroom_useSlopSpace
r5apex.exe!0x0117b890 ConVar chatroom_voiceMode
r5apex.exe!0x01df2840 ConVar chatroom_voiceMode
r5apex.exe!0x01d92f40 ConVar cheap_captions_fadetime
r5apex.exe!0x01d92660 ConVar cheap_captions_test
r5apex.exe!0x0104de50 ConVar chroma_enable
r5apex.exe!0x01825620 ConVar cl_NotifyAllLevelAssetsLoaded_endframe
r5apex.exe!0x01bc1f00 ConVar cl_RunClientConnectScripts_Before_ProcessOnDataChangedEvents
r5apex.exe!0x01831cf0 ConVar cl_SetupAllBones
r5apex.exe!0x0182b320 ConVar cl_ShowBoneSetupEnts
r5apex.exe!0x01146d90 ConVar cl_adjustTimeEntsPerJob
r5apex.exe!0x01d947d0 ConVar cl_aggregate_particles
r5apex.exe!0x01830b50 ConVar cl_allowABSCalculationDuringSnapshotScriptCalls
r5apex.exe!0x018248a0 ConVar cl_allowABSDuringSnapshotScriptCalls
r5apex.exe!0x0182aa60 ConVar cl_allowAnimsToInterpolateBackward
r5apex.exe!0x01d95b10 ConVar cl_always_draw_3p_player
r5apex.exe!0x0183ac40 ConVar cl_always_ragdoll_radius
r5apex.exe!0x01bdd260 ConVar cl_anglespeedkey
r5apex.exe!0x01bc2e60 ConVar cl_anim_blend_transition_dist
r5apex.exe!0x0182e490 ConVar cl_anim_detail_dist
r5apex.exe!0x0182df00 ConVar cl_anim_face_dist
r5apex.exe!0x0182f710 ConVar cl_anim_sequence_transition_full_weight_optimization
r5apex.exe!0x01bcbb90 ConVar cl_anim_sounds_seek
r5apex.exe!0x0181b550 ConVar cl_approx_footstep_origin
r5apex.exe!0x01bdeed0 ConVar cl_approx_tracer_origin
r5apex.exe!0x01bbfc30 ConVar cl_async_bone_setup
r5apex.exe!0x01830850 ConVar cl_base_entity_effect_lock
r5apex.exe!0x018258a0 ConVar cl_bones_incremental_blend
r5apex.exe!0x01828c50 ConVar cl_bones_incremental_transform
r5apex.exe!0x0183eff0 ConVar cl_bones_oldhack
r5apex.exe!0x01dcef80 ConVar cl_bounds_show_errors
r5apex.exe!0x01dd6a80 ConVar cl_burninggibs
r5apex.exe!0x0103a750 ConVar cl_clock_correction
r5apex.exe!0x01038c20 ConVar cl_clock_correction_ahead_correct_interval
r5apex.exe!0x01037a00 ConVar cl_clock_correction_behind_correct_interval
r5apex.exe!0x010392a0 ConVar cl_clock_correction_force_server_tick
r5apex.exe!0x0110f6d0 ConVar cl_cmdbackup
r5apex.exe!0x0110e840 ConVar cl_cmdrate
r5apex.exe!0x01040fb0 ConVar cl_configversion
r5apex.exe!0x0103edc0 ConVar cl_configversion_dummy
r5apex.exe!0x01dfa4b0 ConVar cl_cull_weapon_fx
r5apex.exe!0x0104a840 ConVar cl_dataBlockFragmentPL
r5apex.exe!0x01de1410 ConVar cl_deathhints_enabled
r5apex.exe!0x01829990 ConVar cl_debugClientEntities
r5apex.exe!0x01ddba70 ConVar cl_debug_deferred_trace
r5apex.exe!0x01de1c90 ConVar cl_debug_deferred_trace_overlay
r5apex.exe!0x01bdd0c0 ConVar cl_debug_model_fx_sounds
r5apex.exe!0x01d994c0 ConVar cl_decal_alwayswhite
r5apex.exe!0x01dcf2f0 ConVar cl_decal_backoff
r5apex.exe!0x01d95c50 ConVar cl_deferred_effects
r5apex.exe!0x01ddabd0 ConVar cl_deferred_trace_normal_priority
r5apex.exe!0x0183aba0 ConVar cl_demoviewoverride
r5apex.exe!0x01bc2180 ConVar cl_disable_ragdolls
r5apex.exe!0x01824800 ConVar cl_disable_splitscreen_cpu_level_cfgs_in_pip
r5apex.exe!0x0104e0b0 ConVar cl_disconnectOnTooManySnapshotFrames
r5apex.exe!0x01e05ae0 ConVar cl_doNetworkAsserts
r5apex.exe!0x0104e010 ConVar cl_doRecreateEnts
r5apex.exe!0x018240a0 ConVar cl_draw_player_model
r5apex.exe!0x01bc2360 ConVar cl_drawhud
r5apex.exe!0x01be2cc0 ConVar cl_drawmonitors
r5apex.exe!0x01839270 ConVar cl_ejectbrass
r5apex.exe!0x01be16c0 ConVar cl_enable_remote_splitscreen
r5apex.exe!0x0104e3d0 ConVar cl_entCreateDeleteDebug
r5apex.exe!0x0182dfa0 ConVar cl_events_ignore_invalidate
r5apex.exe!0x01049980 ConVar cl_failremoteconnections
r5apex.exe!0x01bde890 ConVar cl_fasttempentcollision
r5apex.exe!0x01829ad0 ConVar cl_flip_vis_bits
r5apex.exe!0x0104e1f0 ConVar cl_flushentitypacket
r5apex.exe!0x01bc02e0 ConVar cl_footstep_event_max_dist
r5apex.exe!0x01833790 ConVar cl_footstep_event_max_dist_titan
r5apex.exe!0x01145e90 ConVar cl_forceAdjustTime
r5apex.exe!0x0117cc70 ConVar cl_fovScale
r5apex.exe!0x0117cbd0 ConVar cl_gib_allow
r5apex.exe!0x01bc05e0 ConVar cl_gib_attack_dir_scale
r5apex.exe!0x0181abc0 ConVar cl_gib_lifetime
r5apex.exe!0x01d8da20 ConVar cl_idealpitchscale
r5apex.exe!0x0110fbe0 ConVar cl_ignorepackets
r5apex.exe!0x01834640 ConVar cl_interp_all
r5apex.exe!0x01145d50 ConVar cl_interpolate
r5apex.exe!0x0182cae0 ConVar cl_interpolate
r5apex.exe!0x018295f0 ConVar cl_interpolateSoAllAnimsLoop
r5apex.exe!0x01bcb6b0 ConVar cl_interpolation_before_prediction
r5apex.exe!0x0104bf10 ConVar cl_isUnderAge
r5apex.exe!0x0110fd70 ConVar cl_is_softened_locale
r5apex.exe!0x01dee3b0 ConVar cl_jiggle_bone_debug
r5apex.exe!0x01dee270 ConVar cl_jiggle_bone_debug_pitch_constraints
r5apex.exe!0x01dee530 ConVar cl_jiggle_bone_debug_yaw_constraints
r5apex.exe!0x01dee310 ConVar cl_jiggle_bone_invert
r5apex.exe!0x01dee450 ConVar cl_jiggle_bone_sanity
r5apex.exe!0x01145b70 ConVar cl_keepPersistentDataOnDisconnect
r5apex.exe!0x01d8fd90 ConVar cl_lagcompensation
r5apex.exe!0x01110980 ConVar cl_language
r5apex.exe!0x01bcbf70 ConVar cl_leafsystemvis
r5apex.exe!0x01bc1400 ConVar cl_lerpIfChildrenLerp
r5apex.exe!0x0104aac0 ConVar cl_loadBspFromServerInfo
r5apex.exe!0x01047a90 ConVar cl_loadPostProcessShadersEarly
r5apex.exe!0x01046e70 ConVar cl_loadStaticPropsInJob
r5apex.exe!0x01146bb0 ConVar cl_matchmaking_timeout
r5apex.exe!0x0183aec0 ConVar cl_minimal_rtt_shadows
r5apex.exe!0x01bda860 ConVar cl_model_fx_gib_cull_front_dist
r5apex.exe!0x01bda3e0 ConVar cl_model_fx_gib_cull_radius
r5apex.exe!0x01bdd600 ConVar cl_mouseenable
r5apex.exe!0x01110490 ConVar cl_move_use_dt
r5apex.exe!0x0110ebc0 ConVar cl_noTimeoutLocalHost
r5apex.exe!0x01110b60 ConVar cl_overrideEventTimes
r5apex.exe!0x01d91570 ConVar cl_parallelParticlePreDrawWork
r5apex.exe!0x0182ab00 ConVar cl_parallel_clientside_animations
r5apex.exe!0x01ded970 ConVar cl_particle_batch_mode
r5apex.exe!0x0117c9f0 ConVar cl_particle_fallback_base
r5apex.exe!0x0117cd10 ConVar cl_particle_fallback_multiplier
r5apex.exe!0x01d936d0 ConVar cl_particle_limiter_display_killed
r5apex.exe!0x01e086b0 ConVar cl_particle_limiter_hide_killable
r5apex.exe!0x01be29a0 ConVar cl_particle_limiter_max_particle_count
r5apex.exe!0x01be2f10 ConVar cl_particle_limiter_max_system_count
r5apex.exe!0x01d8db40 ConVar cl_particle_limiter_min_kill_distance
r5apex.exe!0x01d93250 ConVar cl_particle_limiter_overlay
r5apex.exe!0x01be3e30 ConVar cl_particle_max_count
r5apex.exe!0x01d94730 ConVar cl_particle_sim_fallback_base_multiplier
r5apex.exe!0x01be1100 ConVar cl_particle_sim_fallback_threshold_ms
r5apex.exe!0x01d91af0 ConVar cl_particle_snoozetime
r5apex.exe!0x01d8e080 ConVar cl_particles_show_bbox
r5apex.exe!0x01d8f240 ConVar cl_particles_show_controlpoints
r5apex.exe!0x01d95240 ConVar cl_pclass
r5apex.exe!0x01d92150 ConVar cl_pdump
r5apex.exe!0x01be1240 ConVar cl_phys_maxticks
r5apex.exe!0x01d8fc50 ConVar cl_phys_show_active
r5apex.exe!0x01d90450 ConVar cl_phys_timescale
r5apex.exe!0x01d93cf0 ConVar cl_physics_invalidate_ents
r5apex.exe!0x01d93f50 ConVar cl_physics_maxvelocity
r5apex.exe!0x01dd71e0 ConVar cl_physicsshadowupdate_render
r5apex.exe!0x01bdda10 ConVar cl_pitchspeed
r5apex.exe!0x011102d0 ConVar cl_playback_screenshots
r5apex.exe!0x0182fdb0 ConVar cl_player_fullupdate_predicted_origin_fix
r5apex.exe!0x01dca1c0 ConVar cl_player_touch_triggers
r5apex.exe!0x0104e290 ConVar cl_postSnapshotTransitionBlockCount
r5apex.exe!0x0183afe0 ConVar cl_preSnapshotTransitionBlockCount
r5apex.exe!0x01dedab0 ConVar cl_pred_error_verbose
r5apex.exe!0x01d8e6b0 ConVar cl_pred_optimize
r5apex.exe!0x011100a0 ConVar cl_predict
r5apex.exe!0x0181a200 ConVar cl_predict_basetoggles
r5apex.exe!0x01be1460 ConVar cl_predict_cmdlimit
r5apex.exe!0x01d945f0 ConVar cl_predict_error_icon_duration
r5apex.exe!0x01be3f10 ConVar cl_predict_error_icon_show
r5apex.exe!0x01d929a0 ConVar cl_predict_error_icon_threshold_angle
r5apex.exe!0x01be2200 ConVar cl_predict_error_icon_threshold_dist
r5apex.exe!0x01dd34d0 ConVar cl_predict_motioncontrol
r5apex.exe!0x01d95bb0 ConVar cl_predict_viewangles
r5apex.exe!0x01be2160 ConVar cl_prediction_error_timestamps
r5apex.exe!0x01d92070 ConVar cl_predictionlist
r5apex.exe!0x01d8e790 ConVar cl_predictweapons
r5apex.exe!0x01df6110 ConVar cl_prevent_weapon_text_hints
r5apex.exe!0x01bc1520 ConVar cl_ragdoll_force_fade_time
r5apex.exe!0x01bbfb90 ConVar cl_ragdoll_force_fade_time_local_view_player
r5apex.exe!0x01ded280 ConVar cl_ragdoll_force_fade_time_on_moving_geo
r5apex.exe!0x01839570 ConVar cl_ragdoll_force_fade_time_titan
r5apex.exe!0x0117cb30 ConVar cl_ragdoll_maxcount
r5apex.exe!0x0117cdb0 ConVar cl_ragdoll_self_collision
r5apex.exe!0x011468b0 ConVar cl_replayDelayTolerance
r5apex.exe!0x01bcc410 ConVar cl_requireAnimForAnimEventsHdr
r5apex.exe!0x011465b0 ConVar cl_resend
r5apex.exe!0x011469f0 ConVar cl_resend_timeout
r5apex.exe!0x0110f470 ConVar cl_retire_low_priority_lights
r5apex.exe!0x018345a0 ConVar cl_runWeaponCloneThinkWhenHidden
r5apex.exe!0x01de18d0 ConVar cl_safearea
r5apex.exe!0x0110f7e0 ConVar cl_screenshotname
r5apex.exe!0x01d95d90 ConVar cl_scriptCompileAsync
r5apex.exe!0x01d8f950 ConVar cl_script_perf_dump_on_shutdown
r5apex.exe!0x01ddbc50 ConVar cl_shadowupdatespacing
r5apex.exe!0x01839730 ConVar cl_showClanTags
r5apex.exe!0x01bda500 ConVar cl_show_splashes
r5apex.exe!0x01d95a70 ConVar cl_showerror
r5apex.exe!0x01be22b0 ConVar cl_showerror_watchfield
r5apex.exe!0x01e04f90 ConVar cl_showfiredbullets
r5apex.exe!0x01d92700 ConVar cl_showfps
r5apex.exe!0x01be4380 ConVar cl_showfps_altframetime
r5apex.exe!0x01d90a30 ConVar cl_showpausedimage
r5apex.exe!0x01be0d20 ConVar cl_showpos
r5apex.exe!0x01048fa0 ConVar cl_showsounds
r5apex.exe!0x01d8e610 ConVar cl_showtime
r5apex.exe!0x01830310 ConVar cl_simulateAllModelsRegardless
r5apex.exe!0x01bc2400 ConVar cl_simulationtimefix
r5apex.exe!0x0182a280 ConVar cl_skipAnimEventsOnProps
r5apex.exe!0x01d93450 ConVar cl_skipfastpath
r5apex.exe!0x01bcc550 ConVar cl_smooth
r5apex.exe!0x018369b0 ConVar cl_smooth_debug
r5apex.exe!0x01bc1ac0 ConVar cl_smoothtime
r5apex.exe!0x01824140 ConVar cl_threaded_bone_setup
r5apex.exe!0x01db4ef0 ConVar cl_updatedirty_async
r5apex.exe!0x0182e0e0 ConVar cl_updatedirty_early
r5apex.exe!0x01110a20 ConVar cl_updaterate_mp
r5apex.exe!0x01bdd8d0 ConVar cl_upspeed
r5apex.exe!0x01110ac0 ConVar cl_useFutureSnapForEvents
r5apex.exe!0x01146410 ConVar cl_useLobbyTypeForChatroom
r5apex.exe!0x0183b080 ConVar cl_view_cone
r5apex.exe!0x01832430 ConVar cl_view_cone_debug
r5apex.exe!0x0183bd70 ConVar cl_viewmodel_pre_animate
r5apex.exe!0x01bdbc40 ConVar cl_warnAboutSoundsOnInvalidEntities
r5apex.exe!0x01bcd050 ConVar cl_yawspeed
r5apex.exe!0x01041af0 ConVar clampHostFrameTimeToOneTick_enable
r5apex.exe!0x0182e220 ConVar clearOnAnimChange
r5apex.exe!0x01d95cf0 ConVar client_deferredSnapshotScriptCalls
r5apex.exe!0x010405b0 ConVar clientport
r5apex.exe!0x01186f70 ConVar cloak_enabled
r5apex.exe!0x0180f280 ConVar cloak_pilotNoiseFactor
r5apex.exe!0x0180f3c0 ConVar cloak_pilotTint1
r5apex.exe!0x0180ef60 ConVar cloak_pilotTint2
r5apex.exe!0x0180ece0 ConVar cloak_pilotTint3
r5apex.exe!0x01039570 ConVar clock_bias_mp
r5apex.exe!0x0103a420 ConVar clock_bias_sp
r5apex.exe!0x01039ad0 ConVar clock_showcorrections
r5apex.exe!0x0103a320 ConVar clock_showdebuginfo
r5apex.exe!0x1fe00fc0 ConVar closecaption
r5apex.exe!0x01dd8410 ConVar cockpitDrift_scalePitch
r5apex.exe!0x01ddb5a0 ConVar cockpitDrift_scaleYaw
r5apex.exe!0x01de1f40 ConVar cockpitDrift_speedPitch
r5apex.exe!0x01dd7430 ConVar cockpitDrift_speedYaw
r5apex.exe!0x01df4ae0 ConVar cockpitShake_sourceRollRange
r5apex.exe!0x01df5ac0 ConVar cockpitShake_translateRange
r5apex.exe!0x01df3730 ConVar cockpit_damage_chroma_scale
r5apex.exe!0x01df3bb0 ConVar cockpit_hit_chroma_max_time
r5apex.exe!0x01df5550 ConVar cockpit_hit_chroma_scale
r5apex.exe!0x01df4ce0 ConVar cockpit_pitch_down_frac
r5apex.exe!0x01df62f0 ConVar cockpit_pitch_up_frac
r5apex.exe!0x01df50c0 ConVar cockpit_screen_boot_chroma_scale
r5apex.exe!0x01df4720 ConVar cockpit_screen_boot_delay_bottom
r5apex.exe!0x01df6390 ConVar cockpit_screen_boot_delay_left
r5apex.exe!0x01df34c0 ConVar cockpit_screen_boot_delay_mid
r5apex.exe!0x01df4ba0 ConVar cockpit_screen_boot_delay_right
r5apex.exe!0x01df6760 ConVar cockpit_screen_boot_delay_top
r5apex.exe!0x01147470 ConVar coll_spatial_entry_limit_client
r5apex.exe!0x01147510 ConVar coll_spatial_optimize_prefetch
r5apex.exe!0x01034890 ConVar coll_use_bolt_size
r5apex.exe!0x01d8e970 ConVar colorblind_mode
r5apex.exe!0x0114bde0 ConVar communities_doRealNameLookupsForCommunityCreators
r5apex.exe!0x0114c740 ConVar communities_enabled
r5apex.exe!0x01177190 ConVar communities_hostname
r5apex.exe!0x0114bf20 ConVar community
r5apex.exe!0x0114c060 ConVar community_abortCommunitySettingsTime
r5apex.exe!0x0114c380 ConVar community_abortUserInfoTime
r5apex.exe!0x01176970 ConVar community_browse_excludeMine
r5apex.exe!0x0114b330 ConVar community_clantags
r5apex.exe!0x01176550 ConVar community_doRealNameLookupsForInbox
r5apex.exe!0x0114c6a0 ConVar community_frame_run
r5apex.exe!0x01179150 ConVar community_queryServerWhenOrphaned
r5apex.exe!0x011764b0 ConVar community_replaceInboxTokens
r5apex.exe!0x01176cd0 ConVar community_replaceInboxTokens
r5apex.exe!0x01176390 ConVar community_resolveNames
r5apex.exe!0x01176690 ConVar community_resolveNames
r5apex.exe!0x0114c2e0 ConVar community_send_server_voice
r5apex.exe!0x01176730 ConVar community_spam
r5apex.exe!0x0114c7e0 ConVar community_staleCommunitySettingsTime
r5apex.exe!0x0114c100 ConVar community_staleUserInfoTime
r5apex.exe!0x0117b360 ConVar con_logfile
r5apex.exe!0x010393e0 ConVar con_timestamp
r5apex.exe!0x01189c90 ConVar cpu_level
r5apex.exe!0x01839090 ConVar cpu_level
r5apex.exe!0x0183bb70 ConVar createentitydecals
r5apex.exe!0x01bbf590 ConVar csm0_on_worker
r5apex.exe!0x01189dd0 ConVar csm_cascade_res
r5apex.exe!0x018394d0 ConVar csm_cascade_res
r5apex.exe!0x011870b0 ConVar csm_coverage
r5apex.exe!0x01bc1fa0 ConVar csm_culling_use_base_planes
r5apex.exe!0x01834d00 ConVar csm_culling_use_exclusion_planes
r5apex.exe!0x0181a160 ConVar csm_culling_use_inclusion_planes
r5apex.exe!0x01bcc4b0 ConVar csm_culling_use_planes
r5apex.exe!0x0182a3a0 ConVar csm_debug_2d
r5apex.exe!0x018304d0 ConVar csm_debug_culling
r5apex.exe!0x01bbf890 ConVar csm_debug_vis_hi_range
r5apex.exe!0x018300b0 ConVar csm_debug_vis_lo_range
r5apex.exe!0x01830270 ConVar csm_depth_bias
r5apex.exe!0x01bc1700 ConVar csm_dropsequence_adjusted_coverage
r5apex.exe!0x0181a3e0 ConVar csm_dropsequence_adjustment
r5apex.exe!0x01189a10 ConVar csm_enabled
r5apex.exe!0x01826cb0 ConVar csm_fadeModels
r5apex.exe!0x0182a1e0 ConVar csm_force_no_csm_in_reflections
r5apex.exe!0x01819f70 ConVar csm_frustum_draw
r5apex.exe!0x01819ed0 ConVar csm_frustum_draw_lock
r5apex.exe!0x01bbf450 ConVar csm_ignore_cascade12
r5apex.exe!0x01825a80 ConVar csm_ignore_edge_planes
r5apex.exe!0x01831b30 ConVar csm_ignore_face_planes
r5apex.exe!0x01bc1360 ConVar csm_max_z_offset
r5apex.exe!0x0182e670 ConVar csm_min_z_offset
r5apex.exe!0x0181a700 ConVar csm_renderable_shadows
r5apex.exe!0x01bc15c0 ConVar csm_rope_shadows
r5apex.exe!0x0182de60 ConVar csm_rot_override
r5apex.exe!0x01824f60 ConVar csm_rot_x
r5apex.exe!0x01828bb0 ConVar csm_rot_y
r5apex.exe!0x01bc1dc0 ConVar csm_shadow_split_lerp_factor_range
r5apex.exe!0x0182a920 ConVar csm_texel_size_cascade_0
r5apex.exe!0x01830a10 ConVar csm_texel_size_cascade_1
r5apex.exe!0x01825d70 ConVar csm_texel_size_cascade_2
r5apex.exe!0x0183b430 ConVar csm_texel_size_cascade_onecascade
r5apex.exe!0x01824ec0 ConVar csm_use_env_light_direction
r5apex.exe!0x01834c60 ConVar csm_world_shadow_meshes
r5apex.exe!0x0183ba50 ConVar csm_world_shadows
r5apex.exe!0x0183b120 ConVar csm_z_cover_world
r5apex.exe!0x01177af0 ConVar curl_allowHTTPS
r5apex.exe!0x01177cd0 ConVar curl_preloadDlls
r5apex.exe!0x01177c30 ConVar curl_spamAllQueryStates
r5apex.exe!0x238b3570 ConVar cursorWide
r5apex.exe!0x01df5fa0 ConVar damageIndicatorReplayTimeOffset
r5apex.exe!0x01de1ab0 ConVar damage_indicator_style_pilot
r5apex.exe!0x01ddb770 ConVar damage_indicator_style_titan
r5apex.exe!0x01d96010 ConVar damageinfo_defendInvalidValues
r5apex.exe!0x0183b1c0 ConVar debugFootstepEffects
r5apex.exe!0x01037570 ConVar debug_debug_overlay
r5apex.exe!0x0114c600 ConVar debug_force_textRestriction
r5apex.exe!0x0114bfc0 ConVar debug_force_ugcRestriction
r5apex.exe!0x0114c420 ConVar debug_force_voiceRestriction
r5apex.exe!0x010397f0 ConVar debug_map_crc
r5apex.exe!0x0128e230 ConVar decal_clip_debug_draw
r5apex.exe!0x0128e150 ConVar decal_clip_debug_groups
r5apex.exe!0x01dfad60 ConVar defer_weapon_effects
r5apex.exe!0x0181a940 ConVar delayPostSnapshotNotificationsToAfterInterpolation
r5apex.exe!0x01146790 ConVar demo_autoRecord
r5apex.exe!0x011462f0 ConVar demo_autoRecordName
r5apex.exe!0x01df1400 ConVar demo_connect_string
r5apex.exe!0x01df10e0 ConVar demo_ui_enable
r5apex.exe!0x01dd8e00 ConVar devStats
r5apex.exe!0x01044510 ConVar developer
r5apex.exe!0x01df52a0 ConVar disable_player_use_prompts
r5apex.exe!0x011471f0 ConVar discord_largeImage
r5apex.exe!0x01147150 ConVar discord_smallImage
r5apex.exe!0x01147290 ConVar discord_updatePresence
r5apex.exe!0x01037b70 ConVar dlight_default_falloff
r5apex.exe!0x0110ff00 ConVar dlight_enable
r5apex.exe!0x0110ee20 ConVar dlight_overlay
r5apex.exe!0x01d99560 ConVar dodge_cockpitHack
r5apex.exe!0x01dcaa80 ConVar dodge_cockpitOffsetMax
r5apex.exe!0x01db49d0 ConVar dodge_cockpitTiltMax
r5apex.exe!0x01d96f80 ConVar dodge_vertical_enable
r5apex.exe!0x01d9a340 ConVar dodge_vertical_horzspeedscale
r5apex.exe!0x01dcd600 ConVar dodge_vertical_in_air
r5apex.exe!0x01dce130 ConVar dodge_vertical_threshold
r5apex.exe!0x01d96c60 ConVar dodge_viewTiltDecreaseSpeed
r5apex.exe!0x01dc8820 ConVar dodge_viewTiltFalloffTime
r5apex.exe!0x01d96760 ConVar dodge_viewTiltIncreaseSpeed
r5apex.exe!0x01dc8730 ConVar dodge_viewTiltMax
r5apex.exe!0x01d914b0 ConVar dof_enable
r5apex.exe!0x0180e380 ConVar dof_farDepthEnd
r5apex.exe!0x0180e420 ConVar dof_farDepthStart
r5apex.exe!0x0180e560 ConVar dof_monitorFarDepthEnd
r5apex.exe!0x0180e240 ConVar dof_monitorFarDepthStart
r5apex.exe!0x0180e100 ConVar dof_monitorNearDepthEnd
r5apex.exe!0x0180e2e0 ConVar dof_monitorNearDepthStart
r5apex.exe!0x0180e4c0 ConVar dof_nearDepthEnd
r5apex.exe!0x0180e1a0 ConVar dof_nearDepthStart
r5apex.exe!0x0180e600 ConVar dof_overrideParams
r5apex.exe!0x01be3d50 ConVar dof_variable_blur
r5apex.exe!0x01826c10 ConVar dormant_debug
r5apex.exe!0x01df3a70 ConVar draw_target_info_offscreen
r5apex.exe!0x01037960 ConVar dtwatchclass
r5apex.exe!0x0103a990 ConVar dtwatchdecode
r5apex.exe!0x01037270 ConVar dtwatchencode
r5apex.exe!0x0103a870 ConVar dtwatchent
r5apex.exe!0x010377a0 ConVar dtwatchvar
r5apex.exe!0x01dd2910 ConVar dump_varsights_calculations
r5apex.exe!0x01829730 ConVar durango_voice_chat_team_only
r5apex.exe!0x01189bf0 ConVar dvs_enable
r5apex.exe!0x0118ab70 ConVar dvs_gpuframetime_max
r5apex.exe!0x0118acb0 ConVar dvs_gpuframetime_min
r5apex.exe!0x0118ac10 ConVar dvs_scale_min
r5apex.exe!0x0180e920 ConVar edge_override_depth
r5apex.exe!0x01810360 ConVar edge_override_depth
r5apex.exe!0x0180e9c0 ConVar edge_override_silhouette
r5apex.exe!0x01810400 ConVar edge_override_silhouette
r5apex.exe!0x0104b060 ConVar enable_KVFileOverrides
r5apex.exe!0x01038490 ConVar enable_debug_overlays
r5apex.exe!0x01dcbbb0 ConVar enable_height_based_land_anims
r5apex.exe!0x01dc9d80 ConVar enable_height_based_land_anims_titans
r5apex.exe!0x0182f5d0 ConVar enable_skeleton_draw
r5apex.exe!0x01046220 ConVar encrypt_multiKey
r5apex.exe!0x01834bc0 ConVar ent_lightweightEnts
r5apex.exe!0x0181a5c0 ConVar ent_repack_almostFull
r5apex.exe!0x01827d10 ConVar ent_repack_threshhold
r5apex.exe!0x01db4430 ConVar entity_skipRedundantAddEffects
r5apex.exe!0x01145a30 ConVar entity_useNetworkFieldBuffer
r5apex.exe!0x01829a30 ConVar error_if_non_standard_ent_create
r5apex.exe!0x01d90d30 ConVar eula_version
r5apex.exe!0x01d8eec0 ConVar eula_version_accepted
r5apex.exe!0x0182f350 ConVar eventseq_debug
r5apex.exe!0x01dd29b0 ConVar everything_unlocked
r5apex.exe!0x010428d0 ConVar fakelag_debug
r5apex.exe!0x01817bf0 ConVar fast_poly_convert
r5apex.exe!0x01e09410 ConVar fatal_script_error_prompt
r5apex.exe!0x01e092d0 ConVar fatal_script_errors
r5apex.exe!0x01e09370 ConVar fatal_script_errors_client
r5apex.exe!0x01e094b0 ConVar fatal_script_errors_server
r5apex.exe!0x01828e30 ConVar fd_playlist_bits
r5apex.exe!0x0117d7f0 ConVar filesystem_buffer_size
r5apex.exe!0x0117daf0 ConVar filesystem_max_stdio_read
r5apex.exe!0x0117db90 ConVar filesystem_native
r5apex.exe!0x0117d9b0 ConVar filesystem_report_buffered_io
r5apex.exe!0x0117d910 ConVar filesystem_unbuffered_io
r5apex.exe!0x0117da50 ConVar filesystem_use_overlapped_io
r5apex.exe!0x01dd12c0 ConVar fire_animevents_overlay_not_active
r5apex.exe!0x01e01c60 ConVar first_person_bullet_delay
r5apex.exe!0x01bcc010 ConVar first_person_proxy_blend_distance
r5apex.exe!0x01dfe050 ConVar first_person_proxy_debug
r5apex.exe!0x01d8ec40 ConVar firsttime_mp_message
r5apex.exe!0x01d959d0 ConVar fog_enable
r5apex.exe!0x01045a00 ConVar fog_enable_water_fog
r5apex.exe!0x01d91ee0 ConVar fog_enableskybox
r5apex.exe!0x01e04ef0 ConVar force3PLaserAttachment
r5apex.exe!0x0114c560 ConVar force_EAAccess
r5apex.exe!0x0104b7a0 ConVar fps_max
r5apex.exe!0x0104a240 ConVar fps_max_use_refresh
r5apex.exe!0x0104a8e0 ConVar fps_max_vsync
r5apex.exe!0x018259e0 ConVar freecam_swallowButtonInput
r5apex.exe!0x01d96330 ConVar freefall_sound_autoplay_time
r5apex.exe!0x01dc6be0 ConVar freefall_sound_height
r5apex.exe!0x0117aab0 ConVar friends_onlineUpdateInterval
r5apex.exe!0x0117cf50 ConVar fs_intralevel_reads
r5apex.exe!0x0117d410 ConVar fs_monitor_read_from_pack
r5apex.exe!0x0117d090 ConVar fs_report_intra_level_readopens
r5apex.exe!0x0117d520 ConVar fs_report_long_reads
r5apex.exe!0x0117cff0 ConVar fs_report_sync_opens
r5apex.exe!0x0117d250 ConVar fs_report_sync_opens_callstack
r5apex.exe!0x0117d370 ConVar fs_report_sync_opens_fatal
r5apex.exe!0x0117d1b0 ConVar fs_showAllReads
r5apex.exe!0x0117dc60 ConVar fs_vpk_file_open
r5apex.exe!0x0117d6c0 ConVar fs_warning_mode
r5apex.exe!0x01bd8770 ConVar func_break_max_pieces
r5apex.exe!0x0183a2a0 ConVar fx_debug
r5apex.exe!0x01e06e50 ConVar fx_deferWorldTraceConstraint
r5apex.exe!0x01bd1cf0 ConVar fx_glass_velocity_cap
r5apex.exe!0x01d943d0 ConVar fx_impact_ally
r5apex.exe!0x01be0890 ConVar fx_impact_enemy
r5apex.exe!0x01d8ece0 ConVar fx_impact_neutral
r5apex.exe!0x01e08570 ConVar fx_screenspacepass
r5apex.exe!0x01dedbf0 ConVar g_debug_ragdoll_removal
r5apex.exe!0x01bc0d40 ConVar g_ragdoll_fadespeed
r5apex.exe!0x01ded320 ConVar g_ragdoll_important_maxcount
r5apex.exe!0x01833e30 ConVar g_ragdoll_lvfadespeed
r5apex.exe!0x01bd58c0 ConVar gameCursor_ModeActive
r5apex.exe!0x01bd66d0 ConVar gameCursor_Velocity
r5apex.exe!0x0182ae20 ConVar gamepad_ads_advanced_sensitivity_scalar_0
r5apex.exe!0x0182aec0 ConVar gamepad_ads_advanced_sensitivity_scalar_1
r5apex.exe!0x0182af60 ConVar gamepad_ads_advanced_sensitivity_scalar_2
r5apex.exe!0x0182b000 ConVar gamepad_ads_advanced_sensitivity_scalar_3
r5apex.exe!0x0182b0a0 ConVar gamepad_ads_advanced_sensitivity_scalar_4
r5apex.exe!0x0182b140 ConVar gamepad_ads_advanced_sensitivity_scalar_5
r5apex.exe!0x0182b1e0 ConVar gamepad_ads_advanced_sensitivity_scalar_6
r5apex.exe!0x0182b280 ConVar gamepad_ads_advanced_sensitivity_scalar_7
r5apex.exe!0x0181b710 ConVar gamepad_aim_assist_hip_high_power_scopes
r5apex.exe!0x01831c50 ConVar gamepad_aim_assist_hip_low_power_scopes
r5apex.exe!0x0183ef50 ConVar gamepad_aim_assist_melee
r5apex.exe!0x01832ad0 ConVar gamepad_aim_assist_scope_0
r5apex.exe!0x01832b70 ConVar gamepad_aim_assist_scope_1
r5apex.exe!0x01832c10 ConVar gamepad_aim_assist_scope_2
r5apex.exe!0x01832cb0 ConVar gamepad_aim_assist_scope_3
r5apex.exe!0x01832d50 ConVar gamepad_aim_assist_scope_4
r5apex.exe!0x01832df0 ConVar gamepad_aim_assist_scope_5
r5apex.exe!0x01832e90 ConVar gamepad_aim_assist_scope_6
r5apex.exe!0x01832f30 ConVar gamepad_aim_assist_scope_7
r5apex.exe!0x01831510 ConVar gamepad_aim_assist_sniper_scopes
r5apex.exe!0x018393b0 ConVar gamepad_aim_speed
r5apex.exe!0x01832fd0 ConVar gamepad_aim_speed_ads_0
r5apex.exe!0x01833070 ConVar gamepad_aim_speed_ads_1
r5apex.exe!0x01833110 ConVar gamepad_aim_speed_ads_2
r5apex.exe!0x018331b0 ConVar gamepad_aim_speed_ads_3
r5apex.exe!0x01833250 ConVar gamepad_aim_speed_ads_4
r5apex.exe!0x018332f0 ConVar gamepad_aim_speed_ads_5
r5apex.exe!0x01833390 ConVar gamepad_aim_speed_ads_6
r5apex.exe!0x01833430 ConVar gamepad_aim_speed_ads_7
r5apex.exe!0x01d8ee20 ConVar gamepad_button_layout
r5apex.exe!0x01d8e550 ConVar gamepad_buttons_are_southpaw
r5apex.exe!0x0183ace0 ConVar gamepad_custom_ads_pitch
r5apex.exe!0x0182f490 ConVar gamepad_custom_ads_turn_delay
r5apex.exe!0x0181b1f0 ConVar gamepad_custom_ads_turn_pitch
r5apex.exe!0x01bc2220 ConVar gamepad_custom_ads_turn_time
r5apex.exe!0x01825eb0 ConVar gamepad_custom_ads_turn_yaw
r5apex.exe!0x01bc0420 ConVar gamepad_custom_ads_yaw
r5apex.exe!0x01824940 ConVar gamepad_custom_assist_on
r5apex.exe!0x01836fe0 ConVar gamepad_custom_curve
r5apex.exe!0x018391d0 ConVar gamepad_custom_deadzone_in
r5apex.exe!0x0181ada0 ConVar gamepad_custom_deadzone_out
r5apex.exe!0x01833830 ConVar gamepad_custom_enabled
r5apex.exe!0x01828cf0 ConVar gamepad_custom_hip_pitch
r5apex.exe!0x01bbf630 ConVar gamepad_custom_hip_turn_delay
r5apex.exe!0x01bc1a20 ConVar gamepad_custom_hip_turn_pitch
r5apex.exe!0x01bc07c0 ConVar gamepad_custom_hip_turn_time
r5apex.exe!0x01839990 ConVar gamepad_custom_hip_turn_yaw
r5apex.exe!0x01834140 ConVar gamepad_custom_hip_yaw
r5apex.exe!0x01be18e0 ConVar gamepad_custom_pilot
r5apex.exe!0x01be0930 ConVar gamepad_custom_titan
r5apex.exe!0x018336f0 ConVar gamepad_deadzone_index_look
r5apex.exe!0x0182aba0 ConVar gamepad_deadzone_index_move
r5apex.exe!0x01bdfff0 ConVar gamepad_enabled
r5apex.exe!0x0183aa60 ConVar gamepad_look_curve
r5apex.exe!0x01d910b0 ConVar gamepad_stick_layout
r5apex.exe!0x01bccd70 ConVar gamepad_toggle_ads
r5apex.exe!0x01bd73c0 ConVar gamepad_togglecrouch_hold
r5apex.exe!0x01181000 ConVar gamepad_trigger_threshold
r5apex.exe!0x0183a940 ConVar gamepad_use_per_scope_ads_settings
r5apex.exe!0x01834300 ConVar gamepad_use_per_scope_sensitivity_scalars
r5apex.exe!0x01d93630 ConVar gamepad_use_type
r5apex.exe!0x01047fb0 ConVar gameui_xbox
r5apex.exe!0x01d90230 ConVar gamma_adjusted
r5apex.exe!0x018272b0 ConVar gatherprops_no_wait
r5apex.exe!0x011813c0 ConVar gfx_desaturate_force
r5apex.exe!0x01d957f0 ConVar gl_clear_color_buffer
r5apex.exe!0x01d8f2e0 ConVar gl_clear_fogcolor
r5apex.exe!0x01d90370 ConVar gl_clear_randomcolor
r5apex.exe!0x01830150 ConVar glass_break_required_speed
r5apex.exe!0x01bdec10 ConVar glass_shatter_direction_force_scale
r5apex.exe!0x01bd3680 ConVar glass_shatter_force_scale
r5apex.exe!0x01bd8890 ConVar glass_shatter_size_scale
r5apex.exe!0x01bd2420 ConVar glass_shatter_use_real_direction
r5apex.exe!0x0180fc80 ConVar glitch_aberrationScale
r5apex.exe!0x0180c6f0 ConVar global_lighting_partial_update
r5apex.exe!0x01e0a000 ConVar gpu_count
r5apex.exe!0x0128dd90 ConVar gpu_driven_tex_stream
r5apex.exe!0x0128e2d0 ConVar gpu_driven_tex_stream_single_thread
r5apex.exe!0x0118ad50 ConVar gpu_level
r5apex.exe!0x01bcb090 ConVar gpu_level
r5apex.exe!0x01181850 ConVar gpu_mem_level
r5apex.exe!0x0181b150 ConVar gpu_mem_level
r5apex.exe!0x0118aa30 ConVar gpu_vram_size_mb
r5apex.exe!0x01dce090 ConVar grapple_accel_human
r5apex.exe!0x01dcab80 ConVar grapple_accel_titan
r5apex.exe!0x01dcd860 ConVar grapple_around_obstacle_accel
r5apex.exe!0x01dcc270 ConVar grapple_autoMantle
r5apex.exe!0x01dcf5d0 ConVar grapple_autoMeleeConvergeTime
r5apex.exe!0x01d9b710 ConVar grapple_autoMeleeOnDetach
r5apex.exe!0x01dd0050 ConVar grapple_autoMeleePredict
r5apex.exe!0x01dd2730 ConVar grapple_autoMeleePredictTime
r5apex.exe!0x01dd2690 ConVar grapple_autoMeleeViewRotateSpeedFar
r5apex.exe!0x01dd25f0 ConVar grapple_autoMeleeViewRotateSpeedNear
r5apex.exe!0x01dcdf20 ConVar grapple_debug
r5apex.exe!0x01d96a80 ConVar grapple_decelMeleeStrength
r5apex.exe!0x01dc7040 ConVar grapple_decel_human
r5apex.exe!0x01d98e00 ConVar grapple_decel_titan
r5apex.exe!0x01d99920 ConVar grapple_detachExtraAllowedLength
r5apex.exe!0x01d9a660 ConVar grapple_disableMeleeWhenActive
r5apex.exe!0x01dc66b0 ConVar grapple_dontFightGravity
r5apex.exe!0x01dc79d0 ConVar grapple_fallSpeed
r5apex.exe!0x01dcd180 ConVar grapple_forcedRetractVel
r5apex.exe!0x01d9b260 ConVar grapple_gracePeriod
r5apex.exe!0x01d9a870 ConVar grapple_gravityPushUnderContribution
r5apex.exe!0x01dcb9c0 ConVar grapple_initialImpulseOffGround_human
r5apex.exe!0x01dcf0f0 ConVar grapple_initialImpulseOffGround_human_npc
r5apex.exe!0x01db4a70 ConVar grapple_initialImpulseOffGround_titan
r5apex.exe!0x01dcaee0 ConVar grapple_initialImpulse_human
r5apex.exe!0x01d9a0c0 ConVar grapple_initialImpulse_titan
r5apex.exe!0x01d9a730 ConVar grapple_initialSlowFracVert_human
r5apex.exe!0x01db4e50 ConVar grapple_initialSlowFracVert_titan
r5apex.exe!0x01dc73b0 ConVar grapple_initialSlowFrac_human
r5apex.exe!0x01dc70e0 ConVar grapple_initialSlowFrac_titan
r5apex.exe!0x01dcc340 ConVar grapple_initialSpeedMin_human
r5apex.exe!0x01dcf670 ConVar grapple_initialSpeedMin_titan
r5apex.exe!0x01dc67f0 ConVar grapple_jumpFrac
r5apex.exe!0x01dc6a70 ConVar grapple_letGravityHelpCosAngle
r5apex.exe!0x01dc6b10 ConVar grapple_lift
r5apex.exe!0x01dd0ea0 ConVar grapple_pullDelay_human
r5apex.exe!0x01dd0e00 ConVar grapple_pullDelay_titan
r5apex.exe!0x01dcf920 ConVar grapple_retractVel
r5apex.exe!0x01dcee40 ConVar grapple_rodeoVerticalImpulse
r5apex.exe!0x01dd27d0 ConVar grapple_shootVel
r5apex.exe!0x01dc6930 ConVar grapple_speedRampMax_human
r5apex.exe!0x01dcfdd0 ConVar grapple_speedRampMax_titan
r5apex.exe!0x01d9a020 ConVar grapple_speedRampMin_human
r5apex.exe!0x01dc7450 ConVar grapple_speedRampMin_titan
r5apex.exe!0x01dcae40 ConVar grapple_speedRampTime_human
r5apex.exe!0x01dca080 ConVar grapple_speedRampTime_titan
r5apex.exe!0x01dc6890 ConVar grapple_swingAngle
r5apex.exe!0x01d9aaf0 ConVar grapple_swingPullAngle
r5apex.exe!0x01d9a520 ConVar grapple_swingPullSpeedLength
r5apex.exe!0x01d96ee0 ConVar grapple_swingPullSpeedScale
r5apex.exe!0x01dcd7c0 ConVar grapple_titanEmbarkDist
r5apex.exe!0x01dc69d0 ConVar grapple_windowCheckDist
r5apex.exe!0x01ddd0c0 ConVar gravity_grenade_decel
r5apex.exe!0x01dd6270 ConVar gravity_grenade_projectile_min_speed
r5apex.exe!0x01d96d00 ConVar ground_debug
r5apex.exe!0x01dd17e0 ConVar ground_trace_hull_radius
r5apex.exe!0x0104dc00 ConVar grx_hasUnknownItems
r5apex.exe!0x0128f430 ConVar gtao_angle_bias
r5apex.exe!0x0128ecb0 ConVar gtao_intensity
r5apex.exe!0x0128f6b0 ConVar gtao_intensity_in_lobby
r5apex.exe!0x0128f750 ConVar gtao_thickness_heuristic
r5apex.exe!0x01146110 ConVar hasAnyAssetsWithDiscardedStreamableData
r5apex.exe!0x01146b10 ConVar hasMic
r5apex.exe!0x01146070 ConVar hasPartialInstall
r5apex.exe!0x0128f4d0 ConVar hbao_angle_bias
r5apex.exe!0x0128efd0 ConVar hbao_intensity
r5apex.exe!0x0128f1b0 ConVar hbao_stepsize_random
r5apex.exe!0x0128ee90 ConVar hbaobasic_tangent_bias
r5apex.exe!0x01bddbd0 ConVar hidehud
r5apex.exe!0x0182ff70 ConVar highlight_deferred_update
r5apex.exe!0x0128e370 ConVar highlight_draw
r5apex.exe!0x0128e530 ConVar highlight_lazy_clear_buffers
r5apex.exe!0x0128e490 ConVar highlight_object_max_count
r5apex.exe!0x01035210 ConVar hitbox_bodygroup_check
r5apex.exe!0x01d8fcf0 ConVar hitch_alert_active
r5apex.exe!0x01d8f080 ConVar hitch_alert_color
r5apex.exe!0x01be2d60 ConVar hitch_alert_show_large_snapshots
r5apex.exe!0x010465b0 ConVar host_RunFrameServerAlways
r5apex.exe!0x01043d40 ConVar host_ShowIPCCallCount
r5apex.exe!0x01148a10 ConVar host_flush_threshold
r5apex.exe!0x01044790 ConVar host_framerate
r5apex.exe!0x01044c30 ConVar host_hasIrreversibleShutdown
r5apex.exe!0x01041410 ConVar host_limitlocal
r5apex.exe!0x0103bcb0 ConVar host_map
r5apex.exe!0x01040650 ConVar host_preload_shaders
r5apex.exe!0x01046830 ConVar host_print_frame_times
r5apex.exe!0x0103f840 ConVar host_profile
r5apex.exe!0x010435b0 ConVar host_runframe_input_parcelremainder
r5apex.exe!0x01041a50 ConVar host_server_thread_min_ticks
r5apex.exe!0x0103b8f0 ConVar host_sleep
r5apex.exe!0x0103e040 ConVar host_speeds
r5apex.exe!0x0110f990 ConVar host_syncfps
r5apex.exe!0x0103f150 ConVar host_thread_join_fast
r5apex.exe!0x01046360 ConVar host_thread_mode
r5apex.exe!0x0103e980 ConVar host_threaded_sound
r5apex.exe!0x01047b30 ConVar host_timescale
r5apex.exe!0x0103f260 ConVar hostip
r5apex.exe!0x0103ff70 ConVar hostname
r5apex.exe!0x0103bc10 ConVar hostport
r5apex.exe!0x01177910 ConVar http_StryderKey
r5apex.exe!0x01177370 ConVar http_debug
r5apex.exe!0x011777d0 ConVar http_debug_forceFailRate
r5apex.exe!0x011775f0 ConVar http_debug_forceFailStatus
r5apex.exe!0x01177410 ConVar http_failuresAsErrors
r5apex.exe!0x01177730 ConVar http_maxAllocateAttempts
r5apex.exe!0x01177b90 ConVar http_recv_fail_realloc
r5apex.exe!0x01177690 ConVar http_sandbox
r5apex.exe!0x01177550 ConVar http_showQueries
r5apex.exe!0x01e05bc0 ConVar hud_autoreloadscript
r5apex.exe!0x01d92a40 ConVar hud_setting_accessibleChat
r5apex.exe!0x01be24e0 ConVar hud_setting_adsDof
r5apex.exe!0x01df3870 ConVar hud_setting_compactOverHeadNames
r5apex.exe!0x01d93910 ConVar hud_setting_damageIndicatorStyle
r5apex.exe!0x01d8e830 ConVar hud_setting_damageTextStyle
r5apex.exe!0x01be0bb0 ConVar hud_setting_enableModWheel
r5apex.exe!0x01be1e40 ConVar hud_setting_healthUseOnHold
r5apex.exe!0x01be28e0 ConVar hud_setting_healthWheelToggle
r5apex.exe!0x01be3170 ConVar hud_setting_healthWheelUseOnRelease
r5apex.exe!0x01d8dda0 ConVar hud_setting_lootPromptStyle
r5apex.exe!0x01d8e1c0 ConVar hud_setting_minimapRotate
r5apex.exe!0x01be15e0 ConVar hud_setting_ordnanceUseOnHold
r5apex.exe!0x01d91750 ConVar hud_setting_ordnanceWheelToggle
r5apex.exe!0x01be1a00 ConVar hud_setting_ordnanceWheelUseOnRelease
r5apex.exe!0x01d937f0 ConVar hud_setting_pingAlpha
r5apex.exe!0x01d8f500 ConVar hud_setting_pingDoubleTapEnemy
r5apex.exe!0x01be3980 ConVar hud_setting_pingWheelToggle
r5apex.exe!0x01be11a0 ConVar hud_setting_showButtonHints
r5apex.exe!0x01d931b0 ConVar hud_setting_showCallsigns
r5apex.exe!0x01d91e40 ConVar hud_setting_showLevelUp
r5apex.exe!0x01d90510 ConVar hud_setting_showMedals
r5apex.exe!0x01d94690 ConVar hud_setting_showMeter
r5apex.exe!0x01d90790 ConVar hud_setting_showObituary
r5apex.exe!0x01be2fb0 ConVar hud_setting_showTips
r5apex.exe!0x01d94130 ConVar hud_setting_showWeaponFlyouts
r5apex.exe!0x01d941f0 ConVar hud_setting_streamerMode
r5apex.exe!0x01bd1810 ConVar hudchat_new_message_fade_duration
r5apex.exe!0x01bd89d0 ConVar hudchat_new_message_shown_duration
r5apex.exe!0x01bd2560 ConVar hudchat_play_text_to_speech
r5apex.exe!0x01bd1650 ConVar hudchat_transition_message_mode_fade_duration
r5apex.exe!0x01bd0830 ConVar hudchat_visibility
r5apex.exe!0x01815b40 ConVar hudwarp_chopsize
r5apex.exe!0x01815820 ConVar hudwarp_override
r5apex.exe!0x018156e0 ConVar hudwarp_viewDist
r5apex.exe!0x01815aa0 ConVar hudwarp_xScale
r5apex.exe!0x01814840 ConVar hudwarp_xWarp
r5apex.exe!0x018148e0 ConVar hudwarp_yScale
r5apex.exe!0x01815780 ConVar hudwarp_yWarp
r5apex.exe!0x01be2350 ConVar idcolor_ally
r5apex.exe!0x01d94910 ConVar idcolor_ally_cb1
r5apex.exe!0x01d91fd0 ConVar idcolor_ally_cb2
r5apex.exe!0x01d927a0 ConVar idcolor_ally_cb3
r5apex.exe!0x01d95580 ConVar idcolor_enemy
r5apex.exe!0x01d91270 ConVar idcolor_enemy_cb1
r5apex.exe!0x01d908f0 ConVar idcolor_enemy_cb2
r5apex.exe!0x01be0c80 ConVar idcolor_enemy_cb3
r5apex.exe!0x01be2840 ConVar idcolor_neutral
r5apex.exe!0x01dd15b0 ConVar ik_debug
r5apex.exe!0x01d95ed0 ConVar ik_debug_chain
r5apex.exe!0x01dcc4b0 ConVar ik_debug_ent
r5apex.exe!0x01dd0370 ConVar ik_debug_text
r5apex.exe!0x01d99740 ConVar ik_enable
r5apex.exe!0x01dc6610 ConVar ik_enable_client
r5apex.exe!0x01dcda70 ConVar ik_height_adjust
r5apex.exe!0x01d99ba0 ConVar ik_height_adjust_debug
r5apex.exe!0x01d986a0 ConVar ik_height_adjust_move_speed
r5apex.exe!0x01d96940 ConVar ik_height_adjust_sine
r5apex.exe!0x01dcf050 ConVar ik_height_adjust_speed
r5apex.exe!0x01dc9cb0 ConVar ik_latch
r5apex.exe!0x01db4d30 ConVar ik_normal_lerp_rate
r5apex.exe!0x01dc8a10 ConVar ik_unlatch_max_rate
r5apex.exe!0x0117f510 ConVar ime_enabled
r5apex.exe!0x01813b00 ConVar imgui_buildmode
r5apex.exe!0x01813bd0 ConVar imgui_buildmode
r5apex.exe!0x01bdbd60 ConVar impact_allow
r5apex.exe!0x01d90830 ConVar impact_debug_info
r5apex.exe!0x01bddb30 ConVar impact_victim_offset_dist
r5apex.exe!0x01dd3270 ConVar impulse_low_decel_duration_scalar
r5apex.exe!0x01146c50 ConVar inPartyChat
r5apex.exe!0x01042410 ConVar in_forceuser
r5apex.exe!0x0104adc0 ConVar in_syncRT
r5apex.exe!0x01bcc8b0 ConVar in_usekeyboardsampletime
r5apex.exe!0x0114c4c0 ConVar inbox_enabled
r5apex.exe!0x0114c1a0 ConVar infoblock_requestInterval
r5apex.exe!0x01be3050 ConVar intro_viewed
r5apex.exe!0x01043830 ConVar ip
r5apex.exe!0x01bdc860 ConVar joy_advaxisr
r5apex.exe!0x01bcd830 ConVar joy_advaxisu
r5apex.exe!0x01bdee30 ConVar joy_advaxisv
r5apex.exe!0x01bd7810 ConVar joy_advaxisx
r5apex.exe!0x01bd9f80 ConVar joy_advaxisy
r5apex.exe!0x01bd7460 ConVar joy_advaxisz
r5apex.exe!0x01bde5d0 ConVar joy_inverty
r5apex.exe!0x01be0590 ConVar joy_legacy
r5apex.exe!0x01bd2860 ConVar joy_movement_stick
r5apex.exe!0x01bccad0 ConVar joy_requireFocus
r5apex.exe!0x01bdaac0 ConVar joy_rumble
r5apex.exe!0x01bdc570 ConVar joy_xcontroller_cfg_loaded
r5apex.exe!0x0110f000 ConVar jpeg_quality
r5apex.exe!0x01810c20 ConVar jt_help_with_anything_ignore_preference
r5apex.exe!0x01dcdff0 ConVar jump_graceperiod
r5apex.exe!0x01dcb120 ConVar jump_keyboardgrace_max
r5apex.exe!0x01db42f0 ConVar jump_keyboardgrace_strength
r5apex.exe!0x01d9a7d0 ConVar jump_keyboardgraceperiodmax
r5apex.exe!0x01dcb570 ConVar jump_keyboardgraceperiodmin
r5apex.exe!0x01147bd0 ConVar killReplay_lagCompensate
r5apex.exe!0x01df5e20 ConVar killReplay_playNonReplayRemoteCallsOnLocalClientPlayer
r5apex.exe!0x01839b00 ConVar leaf_threadedRecompute
r5apex.exe!0x018334d0 ConVar leaf_threadedRecompute_batchSize
r5apex.exe!0x01dfa550 ConVar leech_npc_angle_cos
r5apex.exe!0x01836890 ConVar lerp_careAboutAttachmentBonePosition
r5apex.exe!0x01d96b20 ConVar lerp_debugEnt
r5apex.exe!0x0183e770 ConVar lerp_opt
r5apex.exe!0x0181b290 ConVar lerp_threaded
r5apex.exe!0x01835c40 ConVar lerp_threaded_numEntsPerTask
r5apex.exe!0x0103cb40 ConVar light_maxcone
r5apex.exe!0x011871f0 ConVar lightmap_realtimelight
r5apex.exe!0x0118a7d0 ConVar lightmap_realtimeshadows
r5apex.exe!0x01043c10 ConVar load_during_video
r5apex.exe!0x01df22a0 ConVar loaderrorsCount
r5apex.exe!0x01df1b80 ConVar loaderrorsNeedShown
r5apex.exe!0x01df27a0 ConVar localClientPlayerCachedLevel
r5apex.exe!0x01110870 ConVar locationInfo
r5apex.exe!0x011107d0 ConVar locationInfo_nucleus
r5apex.exe!0x01bd0970 ConVar locator_background_border_color
r5apex.exe!0x01bcc790 ConVar locator_background_border_thickness
r5apex.exe!0x01bd3120 ConVar locator_background_color
r5apex.exe!0x01bd3360 ConVar locator_background_shift_x
r5apex.exe!0x01bd3c90 ConVar locator_background_shift_y
r5apex.exe!0x01bd75a0 ConVar locator_background_style
r5apex.exe!0x01bd3b70 ConVar locator_background_thickness_x
r5apex.exe!0x01bcd450 ConVar locator_background_thickness_y
r5apex.exe!0x01bd1bd0 ConVar locator_fade_time
r5apex.exe!0x01bde4b0 ConVar locator_icon_max_size_non_ss
r5apex.exe!0x01bd7e10 ConVar locator_icon_min_size_non_ss
r5apex.exe!0x01bd1d90 ConVar locator_lerp_rest
r5apex.exe!0x01bd4530 ConVar locator_lerp_speed
r5apex.exe!0x01bd6630 ConVar locator_lerp_time
r5apex.exe!0x01bda6a0 ConVar locator_pulse_time
r5apex.exe!0x01bcce90 ConVar locator_split_len
r5apex.exe!0x01bd4790 ConVar locator_split_maxwide_percent
r5apex.exe!0x01bda020 ConVar locator_start_at_crosshair
r5apex.exe!0x01bd85b0 ConVar locator_target_offset_x
r5apex.exe!0x01bd0040 ConVar locator_target_offset_y
r5apex.exe!0x01bdcd60 ConVar locator_topdown_style
r5apex.exe!0x01bd8b80 ConVar lookspring
r5apex.exe!0x01bdd4c0 ConVar lookstrafe
r5apex.exe!0x01bd37a0 ConVar m_acceleration
r5apex.exe!0x01bdd3a0 ConVar m_forward
r5apex.exe!0x01bd24c0 ConVar m_invert_pitch
r5apex.exe!0x01bd7770 ConVar m_side
r5apex.exe!0x01df1ea0 ConVar mainmenu_background_movie
r5apex.exe!0x01bdc790 ConVar map_settings_override
r5apex.exe!0x01be1520 ConVar mat_autoexposure_compensation
r5apex.exe!0x0180c970 ConVar mat_autoexposure_force_value
r5apex.exe!0x01d92ae0 ConVar mat_autoexposure_max
r5apex.exe!0x01d8f5a0 ConVar mat_autoexposure_max_multiplier
r5apex.exe!0x01d94330 ConVar mat_autoexposure_min
r5apex.exe!0x01d902d0 ConVar mat_autoexposure_min_multiplier
r5apex.exe!0x01be3fb0 ConVar mat_autoexposure_speed
r5apex.exe!0x01d951a0 ConVar mat_autoexposure_uncap
r5apex.exe!0x01d93eb0 ConVar mat_bloom_cutoff
r5apex.exe!0x0180cab0 ConVar mat_bloom_max_lighting_value
r5apex.exe!0x01be2660 ConVar mat_bloom_scalefactor_scalar
r5apex.exe!0x0180eb00 ConVar mat_bloom_streak_amount
r5apex.exe!0x01d91610 ConVar mat_bloom_streak_cutoff
r5apex.exe!0x01d90650 ConVar mat_bloom_streak_cutoff_exposure_adapt
r5apex.exe!0x0180fb40 ConVar mat_bloom_streak_exponent_post
r5apex.exe!0x01d8fbb0 ConVar mat_bloom_streak_exponent_pre
r5apex.exe!0x0180f640 ConVar mat_bloom_wide_amount
r5apex.exe!0x01d91410 ConVar mat_bloom_wide_exponent_pre
r5apex.exe!0x01d93b10 ConVar mat_bloomamount_rate
r5apex.exe!0x01d95930 ConVar mat_bloomscale
r5apex.exe!0x0128fe30 ConVar mat_checkStalls
r5apex.exe!0x011874f0 ConVar mat_cloudmask
r5apex.exe!0x01039990 ConVar mat_colcorrection_disableentities
r5apex.exe!0x01147330 ConVar mat_colcorrection_disableentities
r5apex.exe!0x01bbf3b0 ConVar mat_colcorrection_disableentities
r5apex.exe!0x01038350 ConVar mat_colcorrection_editor
r5apex.exe!0x0182e5d0 ConVar mat_colcorrection_editor
r5apex.exe!0x0182f210 ConVar mat_colcorrection_forceentitiesclientside
r5apex.exe!0x01038a70 ConVar mat_colorcorrection
r5apex.exe!0x0180fbe0 ConVar mat_debug_postprocess_allowed
r5apex.exe!0x01d91c10 ConVar mat_debug_postprocessing_effects
r5apex.exe!0x0180f960 ConVar mat_debug_tonemapping
r5apex.exe!0x0180f460 ConVar mat_debug_tonemapping_disable
r5apex.exe!0x0180f320 ConVar mat_debug_tonemapping_mid1
r5apex.exe!0x0180f000 ConVar mat_debug_tonemapping_mid2
r5apex.exe!0x0180f1e0 ConVar mat_debug_tonemapping_shoulder
r5apex.exe!0x0180f140 ConVar mat_debug_tonemapping_toe
r5apex.exe!0x0118aad0 ConVar mat_debugalttab
r5apex.exe!0x0180d5a0 ConVar mat_depthbias_decal
r5apex.exe!0x0180d780 ConVar mat_depthbias_normal
r5apex.exe!0x0180d460 ConVar mat_depthbias_shadowmap
r5apex.exe!0x0180d640 ConVar mat_depthbias_tightshadowmap
r5apex.exe!0x0180d140 ConVar mat_depthbias_ui
r5apex.exe!0x0180d6e0 ConVar mat_depthbias_zfill
r5apex.exe!0x0180d000 ConVar mat_depthbiasclamp_decal
r5apex.exe!0x0180da00 ConVar mat_depthbiasclamp_normal
r5apex.exe!0x0180d1e0 ConVar mat_depthbiasclamp_shadowmap
r5apex.exe!0x0180d3c0 ConVar mat_depthbiasclamp_ui
r5apex.exe!0x0180d320 ConVar mat_depthbiasclamp_zfill
r5apex.exe!0x0180d0a0 ConVar mat_depthtest_force_disabled
r5apex.exe!0x0118a5f0 ConVar mat_detail_tex
r5apex.exe!0x01181670 ConVar mat_diffuse
r5apex.exe!0x01d91010 ConVar mat_disable_bloom
r5apex.exe!0x0180c8d0 ConVar mat_disable_lightmap_ambient
r5apex.exe!0x01189830 ConVar mat_disable_lightmaps
r5apex.exe!0x01184c70 ConVar mat_disable_model_ambient
r5apex.exe!0x01049180 ConVar mat_drawMenuGrid
r5apex.exe!0x0104b840 ConVar mat_drawTitleSafe
r5apex.exe!0x011818f0 ConVar mat_drawflat
r5apex.exe!0x01187450 ConVar mat_dxlevel
r5apex.exe!0x010383f0 ConVar mat_dynamic_tonemapping
r5apex.exe!0x0118ae90 ConVar mat_dynamic_tonemapping
r5apex.exe!0x01181460 ConVar mat_enable_ssr
r5apex.exe!0x0180cb50 ConVar mat_envmap_scale
r5apex.exe!0x010374d0 ConVar mat_envmaptgasize
r5apex.exe!0x0118a050 ConVar mat_fastnobump
r5apex.exe!0x01043e80 ConVar mat_fastspecular
r5apex.exe!0x0118a4b0 ConVar mat_filterlightmaps
r5apex.exe!0x01184ab0 ConVar mat_filtertextures
r5apex.exe!0x01be2e50 ConVar mat_force_bloom
r5apex.exe!0x01189fb0 ConVar mat_forceaniso
r5apex.exe!0x01be38e0 ConVar mat_frame_color_bias
r5apex.exe!0x01d8ef80 ConVar mat_frame_color_enabled
r5apex.exe!0x01d952e0 ConVar mat_frame_color_scale
r5apex.exe!0x01be41f0 ConVar mat_frame_color_spot_metering_screen_ratio
r5apex.exe!0x01041eb0 ConVar mat_fullbright
r5apex.exe!0x0180f780 ConVar mat_fxaa_enable
r5apex.exe!0x011815d0 ConVar mat_global_lighting
r5apex.exe!0x01290130 ConVar mat_global_lighting
r5apex.exe!0x01be0a70 ConVar mat_global_lighting
r5apex.exe!0x01041550 ConVar mat_hdr_level
r5apex.exe!0x011473d0 ConVar mat_hdrcolcorrection_editor
r5apex.exe!0x01181290 ConVar mat_hdrcolorcorrection
r5apex.exe!0x01290280 ConVar mat_hide_sun_in_last_cascade
r5apex.exe!0x012901e0 ConVar mat_instancing
r5apex.exe!0x0128fd90 ConVar mat_letterbox_aspect_goal
r5apex.exe!0x0128fcf0 ConVar mat_letterbox_aspect_threshold
r5apex.exe!0x01be1d80 ConVar mat_lightcull_subview
r5apex.exe!0x01d8de40 ConVar mat_lightcull_subviews
r5apex.exe!0x0180fa00 ConVar mat_local_contrast_edge_scale_override
r5apex.exe!0x0180ed80 ConVar mat_local_contrast_midtone_mask_override
r5apex.exe!0x0180ec40 ConVar mat_local_contrast_scale_override
r5apex.exe!0x0180ee20 ConVar mat_local_contrast_vignette_end_override
r5apex.exe!0x0180eec0 ConVar mat_local_contrast_vignette_start_override
r5apex.exe!0x0118be60 ConVar mat_materialmip_character_0
r5apex.exe!0x0118b1b0 ConVar mat_materialmip_character_1
r5apex.exe!0x0118b570 ConVar mat_materialmip_character_2
r5apex.exe!0x0118b930 ConVar mat_materialmip_character_3
r5apex.exe!0x0118b890 ConVar mat_materialmip_character_4
r5apex.exe!0x0118bbe0 ConVar mat_materialmip_cockpit_0
r5apex.exe!0x0118b390 ConVar mat_materialmip_cockpit_1
r5apex.exe!0x0118afd0 ConVar mat_materialmip_cockpit_2
r5apex.exe!0x0118b110 ConVar mat_materialmip_cockpit_3
r5apex.exe!0x0118bd20 ConVar mat_materialmip_cockpit_4
r5apex.exe!0x0118b070 ConVar mat_materialmip_model_0
r5apex.exe!0x0118b6b0 ConVar mat_materialmip_model_1
r5apex.exe!0x0118bdc0 ConVar mat_materialmip_model_2
r5apex.exe!0x0118c0c0 ConVar mat_materialmip_model_3
r5apex.exe!0x0118b7f0 ConVar mat_materialmip_model_4
r5apex.exe!0x0118bc80 ConVar mat_materialmip_other_0
r5apex.exe!0x0118b4d0 ConVar mat_materialmip_other_1
r5apex.exe!0x0118c160 ConVar mat_materialmip_other_2
r5apex.exe!0x0118b250 ConVar mat_materialmip_other_3
r5apex.exe!0x0118bfa0 ConVar mat_materialmip_other_4
r5apex.exe!0x0118b750 ConVar mat_materialmip_world_0
r5apex.exe!0x0118af30 ConVar mat_materialmip_world_1
r5apex.exe!0x0118b2f0 ConVar mat_materialmip_world_2
r5apex.exe!0x0118bf00 ConVar mat_materialmip_world_3
r5apex.exe!0x0118b430 ConVar mat_materialmip_world_4
r5apex.exe!0x0103dd40 ConVar mat_maxframelatency
r5apex.exe!0x01187310 ConVar mat_mip_linear
r5apex.exe!0x01187590 ConVar mat_mipmaptextures
r5apex.exe!0x0103b4b0 ConVar mat_norendering
r5apex.exe!0x01181710 ConVar mat_norendering
r5apex.exe!0x01189b50 ConVar mat_phong
r5apex.exe!0x01184d10 ConVar mat_picmip
r5apex.exe!0x0180f6e0 ConVar mat_postprocess_enable
r5apex.exe!0x01d92ca0 ConVar mat_postprocess_enable
r5apex.exe!0x0128c2f0 ConVar mat_processtoolvars
r5apex.exe!0x0118a230 ConVar mat_proxy
r5apex.exe!0x0118a2d0 ConVar mat_reducefillrate
r5apex.exe!0x0118c200 ConVar mat_reduceparticles
r5apex.exe!0x0180cf60 ConVar mat_remoteshadercompile
r5apex.exe!0x0118adf0 ConVar mat_report_queue_status
r5apex.exe!0x011896f0 ConVar mat_reversedepth
r5apex.exe!0x01be3750 ConVar mat_screen_blur_enabled
r5apex.exe!0x0180f0a0 ConVar mat_screen_blur_override
r5apex.exe!0x0103d840 ConVar mat_shadowstate
r5apex.exe!0x0180ea60 ConVar mat_sharpen_amount
r5apex.exe!0x0180f5a0 ConVar mat_sharpen_threshold
r5apex.exe!0x0180f8c0 ConVar mat_sharpen_width
r5apex.exe!0x01145820 ConVar mat_show_texture_memory_usage
r5apex.exe!0x0128c640 ConVar mat_showenvmapmask
r5apex.exe!0x01181530 ConVar mat_showlowresimage
r5apex.exe!0x01189ab0 ConVar mat_showmiplevels
r5apex.exe!0x010406f0 ConVar mat_skipid
r5apex.exe!0x01043ad0 ConVar mat_sky_color
r5apex.exe!0x01040290 ConVar mat_sky_scale
r5apex.exe!0x0180d960 ConVar mat_slopescaledepthbias_decal
r5apex.exe!0x0180d280 ConVar mat_slopescaledepthbias_normal
r5apex.exe!0x0180d8c0 ConVar mat_slopescaledepthbias_shadowmap
r5apex.exe!0x0180d500 ConVar mat_slopescaledepthbias_ui
r5apex.exe!0x0180d820 ConVar mat_slopescaledepthbias_zfill
r5apex.exe!0x01043970 ConVar mat_sun_color
r5apex.exe!0x0103e620 ConVar mat_sun_scale
r5apex.exe!0x01040c90 ConVar mat_surfacefilter
r5apex.exe!0x0103fcf0 ConVar mat_surfaceid
r5apex.exe!0x01043fc0 ConVar mat_surfacemat
r5apex.exe!0x0128fff0 ConVar mat_syncGPU
r5apex.exe!0x0128ff50 ConVar mat_syncInterval
r5apex.exe!0x01189d30 ConVar mat_sync_rt
r5apex.exe!0x0128c4d0 ConVar mat_sync_rt_flushes_gpu
r5apex.exe!0x011456e0 ConVar mat_texture_list
r5apex.exe!0x01145780 ConVar mat_texture_list_view
r5apex.exe!0x0128c570 ConVar mat_translucency_errors
r5apex.exe!0x0180fd20 ConVar mat_use_compressed_hdr_textures
r5apex.exe!0x0180faa0 ConVar mat_vignette_enable
r5apex.exe!0x0118b610 ConVar mat_warn_texture_convert
r5apex.exe!0x0103cd00 ConVar match_backingOutMaxTimeToWait
r5apex.exe!0x01043650 ConVar match_backoutslow
r5apex.exe!0x0103bf70 ConVar match_connect
r5apex.exe!0x0103e360 ConVar match_defaultMap_party
r5apex.exe!0x01044240 ConVar match_dir
r5apex.exe!0x0103f040 ConVar match_dumpSearchResults
r5apex.exe!0x0103b850 ConVar match_emptyUpdateRate
r5apex.exe!0x01045130 ConVar match_enabled
r5apex.exe!0x0103e220 ConVar match_fakePort
r5apex.exe!0x0103ba30 ConVar match_fakeS2SPort
r5apex.exe!0x01046970 ConVar match_forceVerboseSearches
r5apex.exe!0x010410f0 ConVar match_goodReputation
r5apex.exe!0x01041cd0 ConVar match_maxPingsSent
r5apex.exe!0x01046040 ConVar match_mixtape_unchecked
r5apex.exe!0x0103de50 ConVar match_mixtape_unchecked_version
r5apex.exe!0x01046d30 ConVar match_mixtape_version
r5apex.exe!0x0103ef00 ConVar match_mixtape_warnOnPlay
r5apex.exe!0x01044100 ConVar match_myBestDatacenter
r5apex.exe!0x01044060 ConVar match_myDatacenter
r5apex.exe!0x010443f0 ConVar match_myRankedDatacenter
r5apex.exe!0x0103c5c0 ConVar match_myTeam
r5apex.exe!0x01043170 ConVar match_partyChangeNum
r5apex.exe!0x0103c400 ConVar match_partySize
r5apex.exe!0x0103bb70 ConVar match_partySub
r5apex.exe!0x0103b090 ConVar match_pingWaveInterval
r5apex.exe!0x01044ff0 ConVar match_playlist
r5apex.exe!0x0103d270 ConVar match_precachemap
r5apex.exe!0x0103e8e0 ConVar match_privateMatchListWithStryder
r5apex.exe!0x0103fbb0 ConVar match_rankedMaxPing
r5apex.exe!0x0103ce40 ConVar match_rankedSwitchETA
r5apex.exe!0x01044af0 ConVar match_resetPlaylistBetweenMatches
r5apex.exe!0x01043790 ConVar match_roleToken
r5apex.exe!0x0103fe30 ConVar match_searchInterval
r5apex.exe!0x01045590 ConVar match_searching
r5apex.exe!0x01041f50 ConVar match_teamNoFill
r5apex.exe!0x010430d0 ConVar match_updateNotableRate
r5apex.exe!0x01043350 ConVar match_updateRate
r5apex.exe!0x010415f0 ConVar match_useMatchmaking
r5apex.exe!0x01044a50 ConVar match_verbosePrintsInterval
r5apex.exe!0x0103ed20 ConVar match_visiblePlaylists
r5apex.exe!0x01040b50 ConVar matchmaking_hostname
r5apex.exe!0x01dd7140 ConVar max_explosive_damage_mass
r5apex.exe!0x01de1b50 ConVar max_explosive_damage_velocity
r5apex.exe!0x0181b330 ConVar max_tweak_shadow_updates
r5apex.exe!0x01dfa390 ConVar melee_aim_assist_can_lock_pitch
r5apex.exe!0x01e009d0 ConVar melee_aim_assist_use_target_velocity
r5apex.exe!0x01df6d00 ConVar melee_attack_trace_can_use_lunge_distance
r5apex.exe!0x01df93f0 ConVar melee_cone_trace_box_check
r5apex.exe!0x01dc9750 ConVar melee_lunge_abort_distance
r5apex.exe!0x01d99b00 ConVar melee_lunge_abort_if_blocked
r5apex.exe!0x01dfe190 ConVar melee_lunge_adjust_trace_distance
r5apex.exe!0x01dfec50 ConVar melee_lunge_align_eye_position
r5apex.exe!0x01df9df0 ConVar melee_lunge_dot_check
r5apex.exe!0x01dce370 ConVar melee_lunge_force_enable_flying
r5apex.exe!0x01dd0780 ConVar melee_lunge_lag_compensate_target
r5apex.exe!0x01e01fe0 ConVar melee_lunge_scale_by_speed
r5apex.exe!0x01d9b8b0 ConVar melee_lunge_slide
r5apex.exe!0x01d96bc0 ConVar melee_lunge_use_closest_distance_between_cylinders
r5apex.exe!0x01e01770 ConVar melee_lunge_use_command_time
r5apex.exe!0x01dd7000 ConVar melee_queue_attack_anim_event
r5apex.exe!0x010479f0 ConVar mem_dumpstats
r5apex.exe!0x01036c90 ConVar mem_force_flush
r5apex.exe!0x01036bf0 ConVar mem_force_flush_section
r5apex.exe!0x0103d980 ConVar mem_incremental_compact_rate
r5apex.exe!0x01187010 ConVar mem_level
r5apex.exe!0x01828770 ConVar mem_level
r5apex.exe!0x0104a5e0 ConVar mem_max_heapsize
r5apex.exe!0x01048b00 ConVar mem_max_heapsize_dedicated
r5apex.exe!0x010483b0 ConVar mem_min_heapsize
r5apex.exe!0x018114f0 ConVar mem_runheapchecks
r5apex.exe!0x01040830 ConVar mem_test_each_frame
r5apex.exe!0x01042c70 ConVar mem_test_every_n_seconds
r5apex.exe!0x0103d550 ConVar mem_test_quiet
r5apex.exe!0x01d8e280 ConVar menu_faq_community_version
r5apex.exe!0x01be2c20 ConVar menu_faq_patchnotes_version
r5apex.exe!0x01be0b10 ConVar menu_faq_viewed
r5apex.exe!0x01be3b80 ConVar menu_was_multiplayer_played_last
r5apex.exe!0x011789f0 ConVar migrate_attempt_interval
r5apex.exe!0x01df4d80 ConVar miles_actor_occlusion_radius
r5apex.exe!0x01df5860 ConVar miles_channels
r5apex.exe!0x0182f530 ConVar miles_flip_active_window_logic
r5apex.exe!0x01df3420 ConVar miles_force_emitter_environment
r5apex.exe!0x01df4c40 ConVar miles_force_listener_environment
r5apex.exe!0x01df6620 ConVar miles_freeze
r5apex.exe!0x01df5ce0 ConVar miles_initial_occlusion_delay
r5apex.exe!0x01df5690 ConVar miles_language
r5apex.exe!0x01df3600 ConVar miles_listener_freeze
r5apex.exe!0x01df4ee0 ConVar miles_nonactor_occlusion
r5apex.exe!0x01df5f00 ConVar miles_nonactor_occlusion_radius
r5apex.exe!0x01df5020 ConVar miles_nopandist
r5apex.exe!0x01df5d80 ConVar miles_occlusion
r5apex.exe!0x01df5370 ConVar miles_occlusion_force
r5apex.exe!0x01df3b10 ConVar miles_occlusion_partial
r5apex.exe!0x01df4860 ConVar miles_occlusion_use_reset_after_deferred_initial
r5apex.exe!0x01df64e0 ConVar miles_samplerate
r5apex.exe!0x0104d940 ConVar miles_server_sounds_debug
r5apex.exe!0x0104d9e0 ConVar miles_server_sounds_print
r5apex.exe!0x01df3560 ConVar miles_solo_ents
r5apex.exe!0x01bcf570 ConVar miles_soundscape_imgui
r5apex.exe!0x01df45c0 ConVar miles_spatialize_front_degrees
r5apex.exe!0x01df6040 ConVar miles_spatialize_offplane_strength
r5apex.exe!0x01df47c0 ConVar miles_spatialize_on
r5apex.exe!0x01df5160 ConVar miles_spatialize_rear_degrees
r5apex.exe!0x01df6250 ConVar miles_suffixes
r5apex.exe!0x01dd9180 ConVar min_explosive_damage_mass
r5apex.exe!0x01e00b10 ConVar missile_default_speed
r5apex.exe!0x01e00290 ConVar missile_homing_speed
r5apex.exe!0x01034f50 ConVar mod_check_vcollide
r5apex.exe!0x01035170 ConVar mod_trace_load
r5apex.exe!0x0104bfb0 ConVar model_defaultFadeDistMin
r5apex.exe!0x01bcbc50 ConVar model_defaultFadeDistMin
r5apex.exe!0x01048dc0 ConVar model_defaultFadeDistScale
r5apex.exe!0x018297d0 ConVar model_defaultFadeDistScale
r5apex.exe!0x01bcc650 ConVar model_fadeRangeFraction
r5apex.exe!0x01bcc6f0 ConVar model_fadeRangeFractionNear
r5apex.exe!0x01be1aa0 ConVar monitor_cc
r5apex.exe!0x0180eba0 ConVar monitor_mat_sharpen_amount
r5apex.exe!0x01be3a20 ConVar monitor_postfx
r5apex.exe!0x01d906f0 ConVar monitor_rui_world_enabled
r5apex.exe!0x0181b010 ConVar monitor_snapshot_frame_delay
r5apex.exe!0x01825f50 ConVar monitor_zfar_default
r5apex.exe!0x01d8fe50 ConVar monitor_zfar_override
r5apex.exe!0x01be09d0 ConVar monitor_zfar_override_enabled
r5apex.exe!0x01040010 ConVar motd
r5apex.exe!0x01bdce80 ConVar mouse_sensitivity
r5apex.exe!0x01bd1e80 ConVar mouse_use_per_scope_sensitivity_scalars
r5apex.exe!0x01bd1f20 ConVar mouse_zoomed_sensitivity_scalar_0
r5apex.exe!0x01bd1fc0 ConVar mouse_zoomed_sensitivity_scalar_1
r5apex.exe!0x01bd2060 ConVar mouse_zoomed_sensitivity_scalar_2
r5apex.exe!0x01bd2100 ConVar mouse_zoomed_sensitivity_scalar_3
r5apex.exe!0x01bd21a0 ConVar mouse_zoomed_sensitivity_scalar_4
r5apex.exe!0x01bd2240 ConVar mouse_zoomed_sensitivity_scalar_5
r5apex.exe!0x01bd22e0 ConVar mouse_zoomed_sensitivity_scalar_6
r5apex.exe!0x01bd2380 ConVar mouse_zoomed_sensitivity_scalar_7
r5apex.exe!0x01047bf0 ConVar move_one_cmd_per_client_frame
r5apex.exe!0x01df0c90 ConVar movement_anim_downed_playback_maxrate
r5apex.exe!0x01df04d0 ConVar movement_anim_playback_maxrate
r5apex.exe!0x01df0390 ConVar movement_anim_playback_minrate
r5apex.exe!0x01df0890 ConVar movement_anim_sprint_playback_maxrate
r5apex.exe!0x01045310 ConVar mp_accountLink_requestInterval
r5apex.exe!0x0103c260 ConVar mp_allowed
r5apex.exe!0x01df09d0 ConVar mp_bodyyawrate
r5apex.exe!0x01dcaf80 ConVar mp_class_max_dronecontroller
r5apex.exe!0x01dd0410 ConVar mp_class_max_fireteam
r5apex.exe!0x01dc6430 ConVar mp_class_max_pilot
r5apex.exe!0x01dcc3e0 ConVar mp_class_max_titan
r5apex.exe!0x011466f0 ConVar mp_countRRNobodyAsLobby
r5apex.exe!0x01dcb920 ConVar mp_enablematchending
r5apex.exe!0x01d997e0 ConVar mp_enabletimelimit
r5apex.exe!0x01dd2870 ConVar mp_gamemode
r5apex.exe!0x0114b290 ConVar mp_huge_threshhold
r5apex.exe!0x0117ac90 ConVar mp_linkingAccountTime
r5apex.exe!0x0117abf0 ConVar mp_linkingAccountWindow
r5apex.exe!0x01df0610 ConVar mp_maxbodyyaw
r5apex.exe!0x01046470 ConVar mp_permission_requestInterval
r5apex.exe!0x01044830 ConVar mp_permission_rerequestInterval
r5apex.exe!0x01d9b670 ConVar mp_player_level
r5apex.exe!0x01df07f0 ConVar mp_scaleAnimationSpeeds
r5apex.exe!0x01df0930 ConVar mp_showgestureslots
r5apex.exe!0x0104da80 ConVar mtx_svEdition
r5apex.exe!0x01e01ee0 ConVar muteWeaponSounds
r5apex.exe!0x01147010 ConVar name
r5apex.exe!0x01830ab0 ConVar net_RunInvalidatePhysics
r5apex.exe!0x010425d0 ConVar net_async_sendto
r5apex.exe!0x01040bf0 ConVar net_autoUnthrottle
r5apex.exe!0x010478b0 ConVar net_bandwidthPrintThreshold
r5apex.exe!0x01045f00 ConVar net_bindToSpecificAddress
r5apex.exe!0x0103bdd0 ConVar net_blockmsg
r5apex.exe!0x0117bb50 ConVar net_chatThroughChatserver
r5apex.exe!0x01047050 ConVar net_chokeloop
r5apex.exe!0x01040ab0 ConVar net_clearReliableDataOnReset
r5apex.exe!0x01e01b80 ConVar net_client_side_weapon_animations
r5apex.exe!0x01047590 ConVar net_compressDataBlock
r5apex.exe!0x01042790 ConVar net_compressLZValue
r5apex.exe!0x0103fed0 ConVar net_compresspackets
r5apex.exe!0x0103d3b0 ConVar net_compresspackets_minsize
r5apex.exe!0x011461b0 ConVar net_connectPacketWarningThreshhold
r5apex.exe!0x011498b0 ConVar net_connectingDataRate
r5apex.exe!0x010396b0 ConVar net_createUndoDeltas
r5apex.exe!0x0114b8d0 ConVar net_data_block_enabled
r5apex.exe!0x01044eb0 ConVar net_datablockPrintSummaries
r5apex.exe!0x01147e50 ConVar net_datablock_fastRate
r5apex.exe!0x01040150 ConVar net_datablock_longSendTime
r5apex.exe!0x01045b40 ConVar net_datablock_minResendInterval
r5apex.exe!0x01149bb0 ConVar net_datablock_networkLossForSlowSpeed
r5apex.exe!0x011483d0 ConVar net_datablock_resendRateForSlowSpeed
r5apex.exe!0x0114ab10 ConVar net_datablock_slowRate
r5apex.exe!0x0103b710 ConVar net_debugDataBlockReceiver
r5apex.exe!0x01045090 ConVar net_debugDataBlockSender
r5apex.exe!0x0104e150 ConVar net_debugLerping
r5apex.exe!0x01146cf0 ConVar net_deltaFieldEntityBlockSize
r5apex.exe!0x01145ad0 ConVar net_disconnectIfDeltaBufferIsFull
r5apex.exe!0x0103b130 ConVar net_drawslider
r5apex.exe!0x01042db0 ConVar net_droppackets
r5apex.exe!0x01039a30 ConVar net_dumpChangesPrecise
r5apex.exe!0x0103d8e0 ConVar net_encrypt_copyCtx
r5apex.exe!0x01040330 ConVar net_encryptionDebug
r5apex.exe!0x01040d30 ConVar net_fakelag
r5apex.exe!0x01046c90 ConVar net_fakelag_clientOnly
r5apex.exe!0x01047810 ConVar net_fakelagjitter
r5apex.exe!0x01047770 ConVar net_fakeloss
r5apex.exe!0x01145990 ConVar net_forceDeltaBufferToOverflow
r5apex.exe!0x010388c0 ConVar net_forceUnnecessaryUndoDeltas
r5apex.exe!0x01044cd0 ConVar net_forcetimeout
r5apex.exe!0x01047370 ConVar net_fullyConnectedDataRate
r5apex.exe!0x0110fad0 ConVar net_highPacketLatencyThreshold
r5apex.exe!0x0110ea80 ConVar net_highPacketLossThreshold
r5apex.exe!0x0110e700 ConVar net_ignoreAllSnapshots
r5apex.exe!0x01145fd0 ConVar net_largeSnapshotThreshold
r5apex.exe!0x0104e330 ConVar net_lerpFields
r5apex.exe!0x01149770 ConVar net_lowBandwidthConnect
r5apex.exe!0x0103b7b0 ConVar net_maxAccumulatedClearTimeBalance
r5apex.exe!0x0103c660 ConVar net_maxcleartime
r5apex.exe!0x0103d700 ConVar net_maxfilesize
r5apex.exe!0x0103c820 ConVar net_maxfragments
r5apex.exe!0x0103bad0 ConVar net_maxroutable
r5apex.exe!0x0103c010 ConVar net_minConnectionTimeForSpam
r5apex.exe!0x01044350 ConVar net_minQueuedPacketsForPrint
r5apex.exe!0x01d8f640 ConVar net_minResetIdleTimerInterval
r5apex.exe!0x0103caa0 ConVar net_minimumPacketLossDC
r5apex.exe!0x0103c8c0 ConVar net_minroutable
r5apex.exe!0x01146e30 ConVar net_noPostDataForDeletedEnts
r5apex.exe!0x01dd70a0 ConVar net_old_seed_generation
r5apex.exe!0x011493b0 ConVar net_optimize_persistent_data
r5apex.exe!0x0114b650 ConVar net_optimize_playlists
r5apex.exe!0x01e04cd0 ConVar net_optimize_weapons
r5apex.exe!0x0182b3c0 ConVar net_predictParentEntities
r5apex.exe!0x01146510 ConVar net_predictedEntsUseFirstAvailableSnapshot
r5apex.exe!0x01145f30 ConVar net_predictionDebug
r5apex.exe!0x0104e470 ConVar net_pretendSnapshotArrayFull
r5apex.exe!0x0103e730 ConVar net_printCompression
r5apex.exe!0x01146650 ConVar net_printOutOfSnapshots
r5apex.exe!0x010385d0 ConVar net_printUnnecessaryDeltas
r5apex.exe!0x0103a650 ConVar net_propSkipPrintThreshold
r5apex.exe!0x01041ff0 ConVar net_public_adr
r5apex.exe!0x0103e4a0 ConVar net_queue_trace
r5apex.exe!0x0103acb0 ConVar net_queuedPackets_PrintOversleeps
r5apex.exe!0x01040970 ConVar net_queuedPackets_SkipSmallSleeps
r5apex.exe!0x0103ebe0 ConVar net_queued_packet_sender_nopacket_sleep
r5apex.exe!0x0103dac0 ConVar net_queued_packet_thread
r5apex.exe!0x0103f700 ConVar net_recentNetworkGapWindow
r5apex.exe!0x01041870 ConVar net_recentNetworkGapsNeeded
r5apex.exe!0x01145cb0 ConVar net_recreateScriptInstanceOnReplayTransition
r5apex.exe!0x0104e5b0 ConVar net_recv_dumpChanges
r5apex.exe!0x01038f80 ConVar net_recv_dumpNetworkedChangesOnEntCreate
r5apex.exe!0x01039340 ConVar net_recv_watchEnt
r5apex.exe!0x01038960 ConVar net_recv_watchField1
r5apex.exe!0x01039f40 ConVar net_recv_watchField2
r5apex.exe!0x010436f0 ConVar net_resourcePrintMinimum
r5apex.exe!0x010381d0 ConVar net_sendFloatDeltas
r5apex.exe!0x0114a890 ConVar net_sendProfileTotals
r5apex.exe!0x01042970 ConVar net_sendtoInJob
r5apex.exe!0x01046ab0 ConVar net_showFailedAuth
r5apex.exe!0x01146250 ConVar net_showLargeSnapshot
r5apex.exe!0x0103ee60 ConVar net_showQueued
r5apex.exe!0x010378c0 ConVar net_showUndoDeltas
r5apex.exe!0x0110eec0 ConVar net_showUserWarnings
r5apex.exe!0x01046f10 ConVar net_showchoke
r5apex.exe!0x0103b670 ConVar net_showchokeInterval
r5apex.exe!0x0103fd90 ConVar net_showdrop
r5apex.exe!0x0103b990 ConVar net_showfragments
r5apex.exe!0x01041910 ConVar net_showmsg
r5apex.exe!0x0103dc00 ConVar net_showpeaks
r5apex.exe!0x0103f5f0 ConVar net_showsendrecv
r5apex.exe!0x0103dca0 ConVar net_showsplits
r5apex.exe!0x0103fc50 ConVar net_showudp
r5apex.exe!0x0103b1d0 ConVar net_showudp_oob
r5apex.exe!0x010476d0 ConVar net_showudp_remoteonly
r5apex.exe!0x01de14b0 ConVar net_showusercmd
r5apex.exe!0x010371d0 ConVar net_skipUnnecessaryDeltas
r5apex.exe!0x01043b70 ConVar net_splitrate
r5apex.exe!0x01042530 ConVar net_splitrateDefaultMP
r5apex.exe!0x0103ec80 ConVar net_splitrateDefaultSP
r5apex.exe!0x0103ae70 ConVar net_tamperPackets
r5apex.exe!0x01145df0 ConVar net_threadedEntityDeltas
r5apex.exe!0x01146ed0 ConVar net_threadedProcessPacket
r5apex.exe!0x0103c4a0 ConVar net_timeoutUsesLastReadTime
r5apex.exe!0x01042f90 ConVar net_trackerWarningInterval
r5apex.exe!0x01045d20 ConVar net_usesocketsforloopback
r5apex.exe!0x010445b0 ConVar net_verifyEncryption
r5apex.exe!0x0117ba30 ConVar net_voiceEchoFromChatServer
r5apex.exe!0x0103c1c0 ConVar net_warnAboutSocketReadGaps
r5apex.exe!0x01044e10 ConVar net_warnGapTime
r5apex.exe!0x01045fa0 ConVar net_wifi
r5apex.exe!0x01145c10 ConVar net_worldHitchSlopTime
r5apex.exe!0x01042e50 ConVar next
r5apex.exe!0x01dd90e0 ConVar noReloadAfterUse
r5apex.exe!0x0180f820 ConVar noise_filter_scale
r5apex.exe!0x01048c40 ConVar not_focus_sleep
r5apex.exe!0x011765f0 ConVar notification_displayTime
r5apex.exe!0x01049ca0 ConVar nucleus_id
r5apex.exe!0x010492c0 ConVar nucleus_pid
r5apex.exe!0x01be1820 ConVar number_shortenToMillionsAfter
r5apex.exe!0x01e05770 ConVar offhandTossOverheadPitchThreshold
r5apex.exe!0x01d99600 ConVar offhand_alignEndAnim1p3p
r5apex.exe!0x0104b240 ConVar old_culling
r5apex.exe!0x010495e0 ConVar old_gather_props
r5apex.exe!0x01dd05b0 ConVar one_handed_change_rate
r5apex.exe!0x01176d70 ConVar openInvite_spam
r5apex.exe!0x011767d0 ConVar openInvites_filterByLanguage
r5apex.exe!0x01176a90 ConVar openInvites_filterByRegion
r5apex.exe!0x01178f70 ConVar openinvite_duration_default
r5apex.exe!0x01bcd3b0 ConVar ordnanceSwapSelectCooldown
r5apex.exe!0x011783d0 ConVar origin_Errorlevel_OldBehaviour
r5apex.exe!0x01178650 ConVar origin_Errorlevel_Telementry
r5apex.exe!0x011785b0 ConVar origin_authCodeFailureMaxBackoffSeconds
r5apex.exe!0x01178470 ConVar origin_autoRefreshToken
r5apex.exe!0x01178290 ConVar origin_debug
r5apex.exe!0x01177e90 ConVar origin_disconnectWhenOffline
r5apex.exe!0x01177f30 ConVar origin_ignoreInvitesOnLoadScreen
r5apex.exe!0x01177d70 ConVar origin_igo_mutes_sound_enabled
r5apex.exe!0x0182e040 ConVar origin_igo_muting_sound
r5apex.exe!0x01178330 ConVar origin_presense_updateRate
r5apex.exe!0x01178510 ConVar origin_tokenFailureMaxBackoffSeconds
r5apex.exe!0x018141b0 ConVar panel_showVisChanges
r5apex.exe!0x01813e70 ConVar panel_test_title_safe
r5apex.exe!0x0181a660 ConVar parenting_debug
r5apex.exe!0x01dedb50 ConVar particleEffect_checkShouldStillPlay
r5apex.exe!0x01e08750 ConVar particle_alwayswakeonstop
r5apex.exe!0x0117ca90 ConVar particle_cpu_level
r5apex.exe!0x01e08b10 ConVar particle_delete_all_except
r5apex.exe!0x01bc2040 ConVar particle_dlights_enable
r5apex.exe!0x01bcbed0 ConVar particle_dlights_spew
r5apex.exe!0x01d954c0 ConVar particle_gpu_level
r5apex.exe!0x0128e670 ConVar particle_lighting_clear_enable
r5apex.exe!0x0128e5d0 ConVar particle_lighting_size
r5apex.exe!0x01e08350 ConVar particle_lighting_viewmodel_enable
r5apex.exe!0x01be40d0 ConVar particle_overlay
r5apex.exe!0x01d8ea10 ConVar particle_overlay_detail_attributes
r5apex.exe!0x01d94470 ConVar particle_overlay_detail_filter
r5apex.exe!0x01d94090 ConVar particle_overlay_detail_list_particles
r5apex.exe!0x01d90110 ConVar particle_overlay_detail_scroll
r5apex.exe!0x01d924c0 ConVar particle_overlay_hide_sleeping
r5apex.exe!0x01d92e00 ConVar particle_overlay_list_filter
r5apex.exe!0x01be3c20 ConVar particle_overlay_list_tally
r5apex.exe!0x01d8e8d0 ConVar particle_overlay_list_tally_collapse_children
r5apex.exe!0x01d90c90 ConVar particle_overlay_old
r5apex.exe!0x01d916b0 ConVar particle_overlay_scroll
r5apex.exe!0x01e07d10 ConVar particle_remap_vol2cp_debug
r5apex.exe!0x01e06c40 ConVar particle_script_dump
r5apex.exe!0x01e06ce0 ConVar particle_script_list
r5apex.exe!0x01e06d80 ConVar particle_script_log
r5apex.exe!0x01d8d980 ConVar particle_scrub_debug
r5apex.exe!0x01e08430 ConVar particle_scrub_debug_effect
r5apex.exe!0x01e08c50 ConVar particle_scrub_is_using_time_scrub
r5apex.exe!0x01e087f0 ConVar particle_scrub_max_dt
r5apex.exe!0x01e08890 ConVar particle_scrub_play_speed
r5apex.exe!0x01e08a70 ConVar particle_scrub_quality
r5apex.exe!0x01e08930 ConVar particle_scrub_time
r5apex.exe!0x01d92b80 ConVar particle_simulateoverflow
r5apex.exe!0x01e08bb0 ConVar particles_cull_dlights
r5apex.exe!0x01e089d0 ConVar particles_max_passes
r5apex.exe!0x01e084d0 ConVar particles_spawncull
r5apex.exe!0x01e08610 ConVar particles_spawncull_report
r5apex.exe!0x01179010 ConVar parties_alwaysReadSubs
r5apex.exe!0x01178810 ConVar party_autoCreatePartyAlways
r5apex.exe!0x01179310 ConVar party_autoCreatePartyDelay
r5apex.exe!0x01de1790 ConVar party_color_enabled
r5apex.exe!0x01178a90 ConVar party_doRealNameLookups
r5apex.exe!0x01178950 ConVar party_doRealNameLookupsForOwner
r5apex.exe!0x01177050 ConVar party_hostname
r5apex.exe!0x011788b0 ConVar party_httpHandleTimeout
r5apex.exe!0x01176b30 ConVar party_keepAliveTime
r5apex.exe!0x01178e30 ConVar party_keepAliveTime
r5apex.exe!0x01178c70 ConVar party_leaderAlwaysDetectsChanges
r5apex.exe!0x01178ed0 ConVar party_leaveMatchOnJoin
r5apex.exe!0x01178d90 ConVar party_lookupRealNamesForOpenInvites
r5apex.exe!0x011790b0 ConVar party_lookupRealNamesForOpenInvitesForOwner
r5apex.exe!0x01178bd0 ConVar party_minSize
r5apex.exe!0x01179270 ConVar party_privacy
r5apex.exe!0x011793b0 ConVar party_readyToSearch
r5apex.exe!0x0114be80 ConVar party_relyOnPartyForMemberUserInfo
r5apex.exe!0x01179450 ConVar party_requireConsensusForSearch
r5apex.exe!0x0104c190 ConVar perTriangleCollisionForced
r5apex.exe!0x01047190 ConVar persistenceDef_hostname
r5apex.exe!0x0117bf50 ConVar persistenceDef_queryMaxHttpRetries
r5apex.exe!0x0117beb0 ConVar persistenceDef_readMaxHttpRetries
r5apex.exe!0x0117bff0 ConVar persistenceDef_retryReadAfterErrorTime
r5apex.exe!0x0117c090 ConVar persistenceDef_writeMaxHttpRetries
r5apex.exe!0x0117c1d0 ConVar persistence_clForceNew
r5apex.exe!0x0117c270 ConVar persistence_disableForBuildProcess
r5apex.exe!0x0117c310 ConVar persistence_enforce_manifest
r5apex.exe!0x01045e60 ConVar persistence_hostname
r5apex.exe!0x0117c450 ConVar persistence_new_player_if_upgrade_fails
r5apex.exe!0x0117c130 ConVar persistence_upload_def
r5apex.exe!0x0117c3b0 ConVar persistence_upload_failure_is_error
r5apex.exe!0x01146950 ConVar persistent_warningRate
r5apex.exe!0x01048630 ConVar pertrianglecollision
r5apex.exe!0x01e067e0 ConVar phys_bounce
r5apex.exe!0x01e068c0 ConVar phys_cfm
r5apex.exe!0x01e06600 ConVar phys_cfm_anglejointstop
r5apex.exe!0x01e066a0 ConVar phys_drawContacts
r5apex.exe!0x01e06740 ConVar phys_drawContactsDuration
r5apex.exe!0x01e062e0 ConVar phys_drawGeoms
r5apex.exe!0x01e06420 ConVar phys_drawTunnelChecks
r5apex.exe!0x01e06060 ConVar phys_enableObjectPairCollidePrototype
r5apex.exe!0x01e05f20 ConVar phys_erp
r5apex.exe!0x01e06240 ConVar phys_erp_anglejointstop
r5apex.exe!0x01e06560 ConVar phys_frictionDefault
r5apex.exe!0x01d93590 ConVar phys_showObjectCount
r5apex.exe!0x01e06100 ConVar phys_threadGoWide
r5apex.exe!0x01e06ba0 ConVar physics_async_cl
r5apex.exe!0x01e06a60 ConVar physics_autoSleepAngularThreshold
r5apex.exe!0x01e064c0 ConVar physics_autoSleepDebug
r5apex.exe!0x01e06380 ConVar physics_autoSleepGroundHysteresis
r5apex.exe!0x01e06b00 ConVar physics_autoSleepSpeedThreshold
r5apex.exe!0x01e05fc0 ConVar physics_collideWithMovingGeo
r5apex.exe!0x01db4250 ConVar physics_defaultMaxAngularSpeed
r5apex.exe!0x01dcff10 ConVar physics_defaultMaxSpeed
r5apex.exe!0x01817c90 ConVar physics_scaled_mem
r5apex.exe!0x01e061a0 ConVar physics_tunnelChecks
r5apex.exe!0x01e069c0 ConVar physics_tunnelChecksForceAlways
r5apex.exe!0x0104b5c0 ConVar pin_opt_in
r5apex.exe!0x01146f70 ConVar pin_plat_id
r5apex.exe!0x0104b8e0 ConVar pin_sid
r5apex.exe!0x01048310 ConVar pin_telemetry_actually_send
r5apex.exe!0x0104bb60 ConVar pin_telemetry_debug_code
r5apex.exe!0x010480f0 ConVar pin_telemetry_debug_payload
r5apex.exe!0x01de2a20 ConVar pin_telemetry_debug_script
r5apex.exe!0x01048590 ConVar pin_telemetry_dont_send_events
r5apex.exe!0x010497a0 ConVar pin_telemetry_hostname
r5apex.exe!0x01049360 ConVar pin_telemetry_inactivity_send_time
r5apex.exe!0x01049540 ConVar pin_telemetry_max_payload_size
r5apex.exe!0x0104a980 ConVar pin_telemetry_send_debug
r5apex.exe!0x01041690 ConVar ping_debug
r5apex.exe!0x01df20c0 ConVar ping_max_green
r5apex.exe!0x01df1f40 ConVar ping_max_red
r5apex.exe!0x01df2520 ConVar ping_max_yellow
r5apex.exe!0x0103fb10 ConVar ping_minSentForChoice
r5apex.exe!0x01041c30 ConVar ping_qos_units
r5apex.exe!0x01046510 ConVar ping_usePacketLoss
r5apex.exe!0x0181ad00 ConVar pixvis_enable
r5apex.exe!0x0180cd10 ConVar pixvis_maxquads
r5apex.exe!0x0181aa80 ConVar pixvis_spew
r5apex.exe!0x01177a50 ConVar plat_environment
r5apex.exe!0x011762f0 ConVar plat_retryNameLookups
r5apex.exe!0x011470b0 ConVar platform_user_id
r5apex.exe!0x01df1680 ConVar playerListPartyColorB
r5apex.exe!0x01df2480 ConVar playerListPartyColorG
r5apex.exe!0x01df1360 ConVar playerListPartyColorR
r5apex.exe!0x01df2700 ConVar playerListUseFriendColor
r5apex.exe!0x01dd8570 ConVar player_ADS_buffer_time_seconds
r5apex.exe!0x01bbf310 ConVar player_debugPredictedPosition
r5apex.exe!0x0181a480 ConVar player_deltaAnimsMakeMeUnpredicted
r5apex.exe!0x01835e30 ConVar player_doJetwashEffects
r5apex.exe!0x01dc7a70 ConVar player_extraairaccelleration
r5apex.exe!0x01bbf1d0 ConVar player_highFrequencyThinkDistance
r5apex.exe!0x01dd0820 ConVar player_movementBounds_predictionShare
r5apex.exe!0x01df0a70 ConVar player_movingDeathThreshold
r5apex.exe!0x01839610 ConVar player_respawnInputDebounceDuration
r5apex.exe!0x01bd8e40 ConVar player_setting_autosprint
r5apex.exe!0x01d90e10 ConVar player_setting_damage_closes_deathbox_menu
r5apex.exe!0x0182f670 ConVar player_showEyePosition
r5apex.exe!0x01d96640 ConVar player_useMovementBounds
r5apex.exe!0x018397d0 ConVar player_viewchange_debug_pitch
r5apex.exe!0x01bc12c0 ConVar player_viewchange_debug_roll
r5apex.exe!0x01834420 ConVar player_viewchange_debug_yaw
r5apex.exe!0x0104b100 ConVar playlist_changeGamemodeAutomatically
r5apex.exe!0x0104a1a0 ConVar playlist_debug
r5apex.exe!0x01048050 ConVar playlist_debug_getvar
r5apex.exe!0x01049700 ConVar playlist_debug_localization
r5apex.exe!0x01dc6570 ConVar playlist_variableErrorsChecks
r5apex.exe!0x01df32e0 ConVar portal_pointpush_debug
r5apex.exe!0x01df3380 ConVar portal_pointpush_think_rate
r5apex.exe!0x01ddb8b0 ConVar portal_use_player_avoidance
r5apex.exe!0x01830c90 ConVar postdataupdate_threaded
r5apex.exe!0x01825760 ConVar postdataupdate_threaded_chunksize
r5apex.exe!0x01042a10 ConVar printConnectTimings
r5apex.exe!0x0183e6d0 ConVar print_timeprefix
r5apex.exe!0x01dfe0f0 ConVar process_pending_vm_effects
r5apex.exe!0x01df14a0 ConVar progressbar_allow_wrap
r5apex.exe!0x01df1900 ConVar progressbar_high_precision
r5apex.exe!0x01df1720 ConVar progressbar_single_bar
r5apex.exe!0x01e03cf0 ConVar projectile_fake_prediction_in_kill_replay
r5apex.exe!0x01dfedb0 ConVar projectile_faketrails
r5apex.exe!0x01e00a70 ConVar projectile_filltrails
r5apex.exe!0x01df6b00 ConVar projectile_lagCompensationDebug
r5apex.exe!0x01df9350 ConVar projectile_lagCompensationDebugDrawTime
r5apex.exe!0x01dff5b0 ConVar projectile_lagCompensationDebugExtra
r5apex.exe!0x01df8db0 ConVar projectile_lagCompensationDebugServerOffset
r5apex.exe!0x01df8ce0 ConVar projectile_lagCompensationMissileTimeStepScalar
r5apex.exe!0x01dff510 ConVar projectile_muzzleOffsetFirstPersonDecayDist
r5apex.exe!0x01dfaad0 ConVar projectile_muzzleOffsetFirstPersonDecayMaxTime
r5apex.exe!0x01df9c10 ConVar projectile_muzzleOffsetThirdPersonDecayDist
r5apex.exe!0x01dff650 ConVar projectile_muzzleOffsetThirdPersonDecayMaxTime
r5apex.exe!0x01dfef50 ConVar projectile_prediction
r5apex.exe!0x01e011f0 ConVar projectile_predictionErrorCorrectTime
r5apex.exe!0x01bbf7f0 ConVar prop_lightweightPropsSkipAnimData
r5apex.exe!0x0181a8a0 ConVar prop_survivalSkipsAnimData
r5apex.exe!0x01dd8770 ConVar props_break_burst_rotation
r5apex.exe!0x01dd8610 ConVar props_break_max_pieces
r5apex.exe!0x01deb2c0 ConVar props_break_max_pieces_perframe
r5apex.exe!0x011770f0 ConVar publication_hostname
r5apex.exe!0x01bcbd90 ConVar push_cl
r5apex.exe!0x0182ca40 ConVar push_cl_always_update_prev_matrix
r5apex.exe!0x01de2b60 ConVar push_debug
r5apex.exe!0x01ddd2e0 ConVar push_debug_ent
r5apex.exe!0x01dd7570 ConVar push_ragdolls
r5apex.exe!0x01dd84d0 ConVar pve_debug
r5apex.exe!0x01def0d0 ConVar pvs_addWorkItemsAccum
r5apex.exe!0x01def490 ConVar pvs_addWorkItemsThreshold_edges
r5apex.exe!0x01def670 ConVar pvs_addWorkItemsThreshold_leaves
r5apex.exe!0x01def710 ConVar pvs_cullBoxes
r5apex.exe!0x01def2b0 ConVar pvs_debug
r5apex.exe!0x01def530 ConVar pvs_drawPortals
r5apex.exe!0x01def350 ConVar pvs_frustumCullOnly
r5apex.exe!0x01bcc0b0 ConVar pvs_start_early
r5apex.exe!0x01dcbfd0 ConVar r_AirboatViewDampenDamp
r5apex.exe!0x01dcf880 ConVar r_AirboatViewDampenFreq
r5apex.exe!0x01dcd0e0 ConVar r_AirboatViewZHeight
r5apex.exe!0x01dcffb0 ConVar r_JeepViewDampenDamp
r5apex.exe!0x01dcd4c0 ConVar r_JeepViewDampenFreq
r5apex.exe!0x01d9a200 ConVar r_VehicleViewDampen
r5apex.exe!0x01def3f0 ConVar r_WaterDrawReflection
r5apex.exe!0x01189790 ConVar r_WaterDrawRefraction
r5apex.exe!0x0103cbe0 ConVar r_aspectratio
r5apex.exe!0x0180e6a0 ConVar r_bloomtintb
r5apex.exe!0x0180e880 ConVar r_bloomtintexponent
r5apex.exe!0x0180e7e0 ConVar r_bloomtintg
r5apex.exe!0x0180e740 ConVar r_bloomtintr
r5apex.exe!0x01be27a0 ConVar r_blurmenubg
r5apex.exe!0x0180c650 ConVar r_bone_matrix_bulk_update_threshold
r5apex.exe!0x0104a2e0 ConVar r_brush_queue_mode
r5apex.exe!0x0104ad20 ConVar r_createmodeldecals
r5apex.exe!0x010408d0 ConVar r_cullshadowworldmeshes
r5apex.exe!0x01d96da0 ConVar r_debug_draw_box_depth_test
r5apex.exe!0x0104b380 ConVar r_decal_cover_count
r5apex.exe!0x01039e20 ConVar r_decal_cull_stretch_limit
r5apex.exe!0x010490e0 ConVar r_decal_draw_basis
r5apex.exe!0x01039200 ConVar r_decal_drawclipped
r5apex.exe!0x0104b980 ConVar r_decal_overlap_area
r5apex.exe!0x0104ac80 ConVar r_decal_overlap_count
r5apex.exe!0x01049220 ConVar r_decal_test_scale
r5apex.exe!0x01041b90 ConVar r_decals
r5apex.exe!0x0128d260 ConVar r_delay_texture_destroy
r5apex.exe!0x0180dfc0 ConVar r_ditherAlpha
r5apex.exe!0x01189970 ConVar r_ditherFade
r5apex.exe!0x01835d80 ConVar r_ditherFade
r5apex.exe!0x0180e060 ConVar r_ditherFadeShadows
r5apex.exe!0x01830010 ConVar r_ditherFadeShadows
r5apex.exe!0x01bcbaf0 ConVar r_drawallrenderables
r5apex.exe!0x01be2b80 ConVar r_drawalphasort
r5apex.exe!0x01046bf0 ConVar r_drawbrushmodels
r5apex.exe!0x01d95030 ConVar r_drawbrushmodels
r5apex.exe!0x0104aa20 ConVar r_drawdecals
r5apex.exe!0x01d939b0 ConVar r_drawdepth_of_blend2transparent
r5apex.exe!0x01038020 ConVar r_drawdlights
r5apex.exe!0x0103da20 ConVar r_drawentities
r5apex.exe!0x01039d80 ConVar r_drawlightdist
r5apex.exe!0x010394d0 ConVar r_drawlightinfo
r5apex.exe!0x01830970 ConVar r_drawmodelsinzfill
r5apex.exe!0x0103ca00 ConVar r_drawmodelstatsoverlay
r5apex.exe!0x01824280 ConVar r_drawmodelstatsoverlay
r5apex.exe!0x010449b0 ConVar r_drawmodelstatsoverlaydistance
r5apex.exe!0x010419b0 ConVar r_drawmodelstatsoverlayfilter
r5apex.exe!0x0103e400 ConVar r_drawmodelstatsoverlaymax
r5apex.exe!0x01040470 ConVar r_drawmodelstatsoverlaymin
r5apex.exe!0x01d95420 ConVar r_drawopaquerenderables
r5apex.exe!0x0183b310 ConVar r_drawothermodels
r5apex.exe!0x01d95380 ConVar r_drawparticles
r5apex.exe!0x01828d90 ConVar r_drawrenderboxes
r5apex.exe!0x01be2420 ConVar r_drawscreenspaceparticles
r5apex.exe!0x01d8fa90 ConVar r_drawsky
r5apex.exe!0x01d905b0 ConVar r_drawskybox_deprecated
r5apex.exe!0x01039c10 ConVar r_drawstaticlight
r5apex.exe!0x01d934f0 ConVar r_drawstaticprops
r5apex.exe!0x01bd5a90 ConVar r_drawtracers
r5apex.exe!0x010494a0 ConVar r_drawvgui
r5apex.exe!0x01d8f9f0 ConVar r_drawviewmodel
r5apex.exe!0x01044b90 ConVar r_drawworld
r5apex.exe!0x0103b270 ConVar r_dynamic
r5apex.exe!0x01d919d0 ConVar r_earlyRenderables
r5apex.exe!0x0182fd10 ConVar r_enableOriginSort
r5apex.exe!0x01be0dc0 ConVar r_fadeincode
r5apex.exe!0x01bcc270 ConVar r_farz
r5apex.exe!0x01043f20 ConVar r_fastzreject
r5apex.exe!0x01def170 ConVar r_forcecheapwater
r5apex.exe!0x01839c40 ConVar r_jiggle_bones
r5apex.exe!0x010454f0 ConVar r_lightmap
r5apex.exe!0x01044f50 ConVar r_lightprobe_force_trans_dist
r5apex.exe!0x01045810 ConVar r_lightstyle
r5apex.exe!0x01042830 ConVar r_lod
r5apex.exe!0x01d90b50 ConVar r_lod
r5apex.exe!0x01df5c40 ConVar r_lod_switch_scale
r5apex.exe!0x018315b0 ConVar r_mapextents
r5apex.exe!0x01810890 ConVar r_modeldecal_maxtotal
r5apex.exe!0x0181ab20 ConVar r_nearz
r5apex.exe!0x01290090 ConVar r_no_stalls
r5apex.exe!0x0180c830 ConVar r_no_stalls
r5apex.exe!0x0180cdb0 ConVar r_no_stalls
r5apex.exe!0x0103d310 ConVar r_norefresh
r5apex.exe!0x01d8e4b0 ConVar r_particle_lighting_debug
r5apex.exe!0x01be0770 ConVar r_particle_lighting_enable
r5apex.exe!0x01e08010 ConVar r_particle_lighting_enable
r5apex.exe!0x01e073d0 ConVar r_particle_lighting_force
r5apex.exe!0x01e08290 ConVar r_particle_lighting_force
r5apex.exe!0x01be0fa0 ConVar r_particle_low_res_debug
r5apex.exe!0x0128e7b0 ConVar r_particle_low_res_draw_weight_tex
r5apex.exe!0x01e080e0 ConVar r_particle_low_res_enable
r5apex.exe!0x01e08180 ConVar r_particle_low_res_force
r5apex.exe!0x0128e710 ConVar r_particle_low_res_tiled_composite
r5apex.exe!0x01d8dfe0 ConVar r_particle_sim_spike_increment_ms
r5apex.exe!0x01d93a50 ConVar r_particle_sim_spike_threshold_ms
r5apex.exe!0x01be06b0 ConVar r_particle_timescale
r5apex.exe!0x01839ba0 ConVar r_pos_debug
r5apex.exe!0x01830710 ConVar r_render_pos_debug
r5apex.exe!0x01189e70 ConVar r_rimlight
r5apex.exe!0x01035030 ConVar r_rootlod
r5apex.exe!0x01045dc0 ConVar r_rootlod
r5apex.exe!0x01bd2600 ConVar r_ropetranslucent
r5apex.exe!0x01d917f0 ConVar r_setupBoneWorkerThreadhold
r5apex.exe!0x01189f10 ConVar r_shadowrendertotexture
r5apex.exe!0x01be2ae0 ConVar r_sky_ignoreAngles
r5apex.exe!0x01bc22c0 ConVar r_sort_trans_debug
r5apex.exe!0x01830570 ConVar r_sort_trans_debug_dist
r5apex.exe!0x01be3ac0 ConVar r_threaded_particles
r5apex.exe!0x01be1ee0 ConVar r_updaterefracttexture
r5apex.exe!0x01d94870 ConVar r_updaterefracttexture_allowmultiple
r5apex.exe!0x01037aa0 ConVar r_visambient
r5apex.exe!0x01039610 ConVar r_visambient_orig
r5apex.exe!0x01039b70 ConVar r_visambient_point
r5apex.exe!0x0103c780 ConVar r_vislighting_sphereradius
r5apex.exe!0x0103efa0 ConVar r_vismodellighting
r5apex.exe!0x0104b520 ConVar r_vismodellighting_lightpos
r5apex.exe!0x0103b410 ConVar r_vismodellighting_maxdist
r5apex.exe!0x01be13c0 ConVar r_vismodellighting_maxdist
r5apex.exe!0x01042670 ConVar r_vismodellighting_mindist
r5apex.exe!0x01d90990 ConVar r_vismodellighting_mindist
r5apex.exe!0x0104a4a0 ConVar r_vismodellighting_offset_x
r5apex.exe!0x01048f00 ConVar r_vismodellighting_offset_y
r5apex.exe!0x0104a100 ConVar r_vismodellighting_offset_z
r5apex.exe!0x0183e590 ConVar r_visualizeproplightcaching
r5apex.exe!0x01ded000 ConVar r_visualizetraces
r5apex.exe!0x01deca80 ConVar r_visualizetraces_duration
r5apex.exe!0x0128fb10 ConVar r_volumetric_lighting_blur_count
r5apex.exe!0x0128fa70 ConVar r_volumetric_lighting_blur_type
r5apex.exe!0x0128f9d0 ConVar r_volumetric_lighting_enabled
r5apex.exe!0x0128fc50 ConVar r_volumetric_lighting_numSteps
r5apex.exe!0x0128fbb0 ConVar r_volumetric_lighting_rotate_dither
r5apex.exe!0x01def210 ConVar r_waterforceexpensive
r5apex.exe!0x01def5d0 ConVar r_waterforcereflectentities
r5apex.exe!0x01d94f10 ConVar r_zfill
r5apex.exe!0x0182f170 ConVar ragdoll_debug
r5apex.exe!0x01d93d90 ConVar ragdoll_sleepaftertime
r5apex.exe!0x01be37f0 ConVar rankedplay_display_enabled
r5apex.exe!0x01be1060 ConVar rankedplay_voice_enabled
r5apex.exe!0x011103f0 ConVar rate
r5apex.exe!0x01bd14b0 ConVar real_time_update_dt
r5apex.exe!0x0104e510 ConVar recalculateOrigin_threaded_chunksize
r5apex.exe!0x01df4a40 ConVar remoteCalls_requireConnectionScriptsForViewPlayer
r5apex.exe!0x01047e70 ConVar remoteMatchInfo_print
r5apex.exe!0x0104c2d0 ConVar replay_enable
r5apex.exe!0x01049c00 ConVar replay_prediction_smooth
r5apex.exe!0x01bc04c0 ConVar report_cliententitysim
r5apex.exe!0x01830430 ConVar report_clientthinklist
r5apex.exe!0x01bcb890 ConVar rodeo_camera_smooth_blend_out_time
r5apex.exe!0x01833570 ConVar rodeo_camera_smooth_enable
r5apex.exe!0x01df0750 ConVar rodeoed_anims_enabled
r5apex.exe!0x01bdead0 ConVar rope_collide
r5apex.exe!0x01bdd560 ConVar rope_debug_shake
r5apex.exe!0x01bcff20 ConVar rope_parallelMeshBuilder
r5apex.exe!0x01bde390 ConVar rope_regenMeshEachDraw
r5apex.exe!0x01bd1770 ConVar rope_shake
r5apex.exe!0x01bddd70 ConVar rope_texels_per_world_unit
r5apex.exe!0x01bcd570 ConVar rope_wiggle_harmonic_falloff
r5apex.exe!0x01bdf550 ConVar rope_wiggle_magnitude_loose
r5apex.exe!0x01bd26a0 ConVar rope_wiggle_magnitude_tight
r5apex.exe!0x01bdadc0 ConVar rope_wiggle_oscillate_speed
r5apex.exe!0x01bcd290 ConVar rope_wiggle_rotate_speed
r5apex.exe!0x01bd7500 ConVar rope_wiggle_zipline_min_points
r5apex.exe!0x01bcccd0 ConVar rope_wind_dist
r5apex.exe!0x01831eb0 ConVar rotate_ents
r5apex.exe!0x0103c0b0 ConVar rspn_motd
r5apex.exe!0x0128c390 ConVar rt_sync_message_pump
r5apex.exe!0x0128c430 ConVar rt_worker
r5apex.exe!0x01df2160 ConVar ruiPanel_resArgName
r5apex.exe!0x01d92ea0 ConVar rui_asyncTracks
r5apex.exe!0x01815640 ConVar rui_defaultDebugFontFace
r5apex.exe!0x01815960 ConVar rui_defaultFontFace
r5apex.exe!0x018147a0 ConVar rui_defaultFontHeight
r5apex.exe!0x01813f70 ConVar rui_overrideVguiTextRendering
r5apex.exe!0x01147650 ConVar rui_padDist
r5apex.exe!0x011475b0 ConVar rui_safeAreaFrac
r5apex.exe!0x01048770 ConVar rui_standardTextHeight
r5apex.exe!0x0103cda0 ConVar s2sPort
r5apex.exe!0x0114b510 ConVar save_enable
r5apex.exe!0x01814260 ConVar scheme_manager_font_debug
r5apex.exe!0x01d8f440 ConVar scr_centertime
r5apex.exe!0x01826b70 ConVar screen_indicator_back_range
r5apex.exe!0x0183b9b0 ConVar screen_indicator_ellipse_height
r5apex.exe!0x0181ae50 ConVar screen_indicator_ellipse_width
r5apex.exe!0x01828a90 ConVar screen_indicator_pitch_limit
r5apex.exe!0x01828ff0 ConVar screen_indicator_pitch_scale
r5apex.exe!0x01d92900 ConVar screenfade_debug
r5apex.exe!0x01dd2c30 ConVar script_compile_all_levels
r5apex.exe!0x01d90070 ConVar script_debugger_connect_client_on_mapspawn
r5apex.exe!0x01d94e70 ConVar script_debugger_connect_ui_auto
r5apex.exe!0x01e09190 ConVar script_debugger_host
r5apex.exe!0x01e09230 ConVar script_debugger_port_client
r5apex.exe!0x01e090f0 ConVar script_debugger_port_server
r5apex.exe!0x01e08fb0 ConVar script_debugger_port_ui
r5apex.exe!0x01e095c0 ConVar script_disallow_newslot_on_globals
r5apex.exe!0x01e09050 ConVar script_dump_simple
r5apex.exe!0x01ddadb0 ConVar script_error_on_midgame_load
r5apex.exe!0x01e08f10 ConVar script_infinite_loop_ms
r5apex.exe!0x01ddc9f0 ConVar script_parallel_trace_LOS_multiple
r5apex.exe!0x01dcd9d0 ConVar script_precache_errors
r5apex.exe!0x01be0f00 ConVar script_printDeferredCalls
r5apex.exe!0x01deb5c0 ConVar script_retry_after_compile_errors
r5apex.exe!0x01dde0b0 ConVar script_seasonNameQueryInterval
r5apex.exe!0x01df1540 ConVar script_showErrorDialogs
r5apex.exe!0x0103b550 ConVar script_slopTimeBeforeBudgetEnforcement
r5apex.exe!0x01dd2d70 ConVar sequence_transitioner_enable
r5apex.exe!0x010468d0 ConVar serverFilter
r5apex.exe!0x01176f10 ConVar serverReports_hostname
r5apex.exe!0x01037310 ConVar server_concommands_allways_network
r5apex.exe!0x01178b30 ConVar server_query_interval
r5apex.exe!0x01bcc1d0 ConVar sfm_record_hz
r5apex.exe!0x0182f2b0 ConVar shadow_always_update
r5apex.exe!0x0180ca10 ConVar shadow_bleedfudge
r5apex.exe!0x0103d1d0 ConVar shadow_capable
r5apex.exe!0x01d90eb0 ConVar shadow_clear_dist
r5apex.exe!0x0182ad80 ConVar shadow_dbg_draw
r5apex.exe!0x0182ac40 ConVar shadow_default_filter_size
r5apex.exe!0x01bc0380 ConVar shadow_depth_dimen_min
r5apex.exe!0x0182e180 ConVar shadow_depth_upres_factor_max
r5apex.exe!0x01839130 ConVar shadow_drawfrustum
r5apex.exe!0x018254e0 ConVar shadow_dynamic_blendfactor
r5apex.exe!0x01044650 ConVar shadow_enable
r5apex.exe!0x018307b0 ConVar shadow_esm_enable
r5apex.exe!0x01823b20 ConVar shadow_filter_maxstep
r5apex.exe!0x01bcb130 ConVar shadow_info
r5apex.exe!0x01bbf750 ConVar shadow_lobby_mode_allowed
r5apex.exe!0x0183e890 ConVar shadow_max_dynamic_lobby
r5apex.exe!0x01828810 ConVar shadow_max_old_dynamic
r5apex.exe!0x01833610 ConVar shadow_max_spot_updates
r5apex.exe!0x01184a10 ConVar shadow_maxdynamic
r5apex.exe!0x018241e0 ConVar shadow_maxdynamic
r5apex.exe!0x0182f3f0 ConVar shadow_min_count_smallest
r5apex.exe!0x0180cbf0 ConVar shadow_minvariance
r5apex.exe!0x01bcbcf0 ConVar shadow_multisampled
r5apex.exe!0x01826ad0 ConVar shadow_noLOD
r5apex.exe!0x01bbfaf0 ConVar shadow_show_spot_udpate_infos
r5apex.exe!0x01839870 ConVar shadow_tools_depth_dimen_min
r5apex.exe!0x0182fed0 ConVar shadow_tools_depth_upres_factor_max
r5apex.exe!0x01836f30 ConVar shadow_tools_min_count_smallest
r5apex.exe!0x01bbf930 ConVar shadow_tools_mode
r5apex.exe!0x0183ad80 ConVar shadow_update_culling
r5apex.exe!0x01bbf270 ConVar shake_angleFactor_human
r5apex.exe!0x01829690 ConVar shake_angleFactor_titan
r5apex.exe!0x01be3290 ConVar shake_basicPitchFactor
r5apex.exe!0x01d8f380 ConVar shake_basicRandomRollFactor
r5apex.exe!0x01835ce0 ConVar shake_offsetFactor_human
r5apex.exe!0x01825800 ConVar shake_offsetFactor_titan
r5apex.exe!0x01bc0680 ConVar shake_viewmodelFactor_ads_human
r5apex.exe!0x01bbfa50 ConVar shake_viewmodelFactor_ads_titan
r5apex.exe!0x01826a30 ConVar shake_viewmodelFactor_human
r5apex.exe!0x01bc0720 ConVar shake_viewmodelFactor_titan
r5apex.exe!0x0118a190 ConVar showfps_enabled
r5apex.exe!0x01187150 ConVar showfps_heightpercent
r5apex.exe!0x0117f5e0 ConVar showfps_mouse_latency
r5apex.exe!0x011898d0 ConVar showfps_smoothtime
r5apex.exe!0x0118a910 ConVar showfps_spinner
r5apex.exe!0x011873b0 ConVar showmem_enabled
r5apex.exe!0x0118a370 ConVar showmem_mode_bottom
r5apex.exe!0x0118a870 ConVar showmem_mode_top
r5apex.exe!0x0118a550 ConVar shownet_enabled
r5apex.exe!0x0118a0f0 ConVar showsnapshot_enabled
r5apex.exe!0x01bd0b90 ConVar sidearmSwapSelectCooldown
r5apex.exe!0x01bdaa20 ConVar sidearmSwapSelectDoubleTapTime
r5apex.exe!0x0103e180 ConVar single_frame_shutdown_for_reload
r5apex.exe!0x01045770 ConVar singlestep
r5apex.exe!0x01049f20 ConVar skill_arena
r5apex.exe!0x01049a20 ConVar skill_dediOnly
r5apex.exe!0x0104ae60 ConVar skill_enabled
r5apex.exe!0x0103e2c0 ConVar skill_hostname
r5apex.exe!0x01d9b330 ConVar skip_jump_height_fraction
r5apex.exe!0x01dcde20 ConVar skip_jump_height_speed
r5apex.exe!0x01dcf1c0 ConVar skip_replenish_double_jump
r5apex.exe!0x01d99420 ConVar skip_sounds
r5apex.exe!0x01dceca0 ConVar skip_speed_reduce
r5apex.exe!0x01dcbc80 ConVar skip_speed_retain
r5apex.exe!0x01dcfd00 ConVar skip_time
r5apex.exe!0x01048810 ConVar sleep_when_meeting_framerate
r5apex.exe!0x0104b660 ConVar sleep_when_meeting_framerate_headroom_ms
r5apex.exe!0x01dcad70 ConVar slide_auto_stand
r5apex.exe!0x01d985c0 ConVar slide_max_angle_dot
r5apex.exe!0x01d9b530 ConVar slide_step_velocity_reduction
r5apex.exe!0x01ddd9a0 ConVar slide_viewTiltDecreaseSpeed
r5apex.exe!0x01de1830 ConVar slide_viewTiltIncreaseSpeed
r5apex.exe!0x01dd2f30 ConVar slide_viewTiltPlayerSpeed
r5apex.exe!0x01ddb810 ConVar slide_viewTiltSide
r5apex.exe!0x01dc50e0 ConVar slide_whileInAir
r5apex.exe!0x01039160 ConVar slowconsolelog_old_logic
r5apex.exe!0x01de12d0 ConVar smoothstairs_lunge
r5apex.exe!0x01038530 ConVar sort_opaque_meshes
r5apex.exe!0x0182ba60 ConVar sound_classic_music
r5apex.exe!0x01be04f0 ConVar sound_entity_seek_snap
r5apex.exe!0x01dd74d0 ConVar sound_musicReduced
r5apex.exe!0x01bcc310 ConVar sound_num_speakers
r5apex.exe!0x01df3910 ConVar sound_only_warn_on_missing_sound_events_in_client_script
r5apex.exe!0x01110630 ConVar sound_printloaderrors
r5apex.exe!0x018256c0 ConVar sound_volume
r5apex.exe!0x01830dd0 ConVar sound_volume_dialogue
r5apex.exe!0x01830d30 ConVar sound_volume_dialogue_sp
r5apex.exe!0x01834260 ConVar sound_volume_music_game
r5apex.exe!0x01831470 ConVar sound_volume_music_game_sp
r5apex.exe!0x01830610 ConVar sound_volume_music_lobby
r5apex.exe!0x0181a2a0 ConVar sound_volume_sfx
r5apex.exe!0x01bc1c00 ConVar sound_volume_sfx_sp
r5apex.exe!0x01bc1e60 ConVar sound_volume_voice
r5apex.exe!0x0183ae20 ConVar sound_without_focus
r5apex.exe!0x01bda900 ConVar soundscape_fadetime
r5apex.exe!0x01bd8da0 ConVar soundscape_message
r5apex.exe!0x01bdbe00 ConVar soundscape_radius_debug
r5apex.exe!0x01831e10 ConVar soundtrigger_repeat_interval
r5apex.exe!0x01047f10 ConVar sp_not_focus_pause
r5apex.exe!0x0180c5b0 ConVar spam_skinning_matrices_used
r5apex.exe!0x0180c790 ConVar spam_skinning_matrices_used_detailed
r5apex.exe!0x01d99f80 ConVar spatial_partition_deadlock_assert
r5apex.exe!0x01df5b80 ConVar speech_queue_bytes
r5apex.exe!0x0117a750 ConVar speechtotext_audioenabled
r5apex.exe!0x0117a610 ConVar speechtotext_enabled
r5apex.exe!0x0117a570 ConVar speechtotext_forcedisabled
r5apex.exe!0x0117a390 ConVar speechtotext_hostname
r5apex.exe!0x0117a4d0 ConVar speechtotext_msg_droptimeout
r5apex.exe!0x011794f0 ConVar speechtotext_path
r5apex.exe!0x0117a6b0 ConVar speechtotext_quiettime
r5apex.exe!0x0117a110 ConVar speechtotext_stats_errorspermin
r5apex.exe!0x0117a430 ConVar speechtotext_stats_interval
r5apex.exe!0x0117a250 ConVar speechtotext_stats_senderrors
r5apex.exe!0x0117a1b0 ConVar speechtotext_stats_sendrequests
r5apex.exe!0x0117a2f0 ConVar speechtotext_stats_sendsuccess
r5apex.exe!0x010401f0 ConVar speechtotexttoken_hostname
r5apex.exe!0x0104d310 ConVar speex_audio_recording
r5apex.exe!0x0104c550 ConVar speex_audio_value
r5apex.exe!0x0117c950 ConVar speex_preprocess_agc_max_gain
r5apex.exe!0x0117c590 ConVar speex_preprocess_noise_suppress
r5apex.exe!0x0117c8b0 ConVar speex_preprocess_set_agc_decrenment
r5apex.exe!0x0117c4f0 ConVar speex_preprocess_set_agc_increment
r5apex.exe!0x0117c770 ConVar speex_preprocess_set_agc_target
r5apex.exe!0x0104c730 ConVar speex_quiet_threshold
r5apex.exe!0x0104c870 ConVar speex_quiet_window
r5apex.exe!0x0117c630 ConVar speex_set_enh
r5apex.exe!0x0117c6d0 ConVar speex_use_highpass
r5apex.exe!0x0117c810 ConVar speex_use_preproser
r5apex.exe!0x01ded1e0 ConVar spinner_debug_info
r5apex.exe!0x01dd86b0 ConVar sprint_powerdrain
r5apex.exe!0x01bbfd50 ConVar sprint_view_shake_style
r5apex.exe!0x01d96150 ConVar sprinttilt_accel
r5apex.exe!0x01dcc550 ConVar sprinttilt_maxvel
r5apex.exe!0x01d98ea0 ConVar sprinttilt_turnrange
r5apex.exe!0x01d8dce0 ConVar ss_enable
r5apex.exe!0x01be1300 ConVar ss_force_primary_fullscreen
r5apex.exe!0x01bd8f60 ConVar ss_mimic
r5apex.exe!0x01d8e120 ConVar ss_splitmode
r5apex.exe!0x01d921f0 ConVar ss_verticalsplit
r5apex.exe!0x01bcb750 ConVar ss_viewmodelfov
r5apex.exe!0x011496d0 ConVar ss_voice_hearpartner
r5apex.exe!0x0128eb70 ConVar ssao_allow_partial
r5apex.exe!0x0128edf0 ConVar ssao_blur
r5apex.exe!0x0128f250 ConVar ssao_blur_edge_sharpness
r5apex.exe!0x0128f070 ConVar ssao_depth_max
r5apex.exe!0x0128ea30 ConVar ssao_downsample
r5apex.exe!0x0128ed50 ConVar ssao_enabled
r5apex.exe!0x0128f110 ConVar ssao_exponent
r5apex.exe!0x0128e990 ConVar ssao_jitter_scale
r5apex.exe!0x0128f390 ConVar ssao_max_res
r5apex.exe!0x0128ec10 ConVar ssao_max_res_threshold
r5apex.exe!0x0128e8f0 ConVar ssao_num_directions
r5apex.exe!0x0128f7f0 ConVar ssao_num_steps
r5apex.exe!0x0128f610 ConVar ssao_on_everything
r5apex.exe!0x0128e850 ConVar ssao_radius
r5apex.exe!0x0128ef30 ConVar ssao_radius_in_lobby
r5apex.exe!0x0128f570 ConVar ssao_show
r5apex.exe!0x0128f930 ConVar ssao_show
r5apex.exe!0x0180f500 ConVar ssao_show
r5apex.exe!0x0128ead0 ConVar ssao_snap_uv
r5apex.exe!0x0128f2f0 ConVar ssao_tech
r5apex.exe!0x01d911d0 ConVar ssao_tech
r5apex.exe!0x0128f890 ConVar ssao_upsample_ranged
r5apex.exe!0x01049de0 ConVar startButtonCommand
r5apex.exe!0x01048a60 ConVar staticProp_budget
r5apex.exe!0x01825580 ConVar staticProp_buildlists_on_worker
r5apex.exe!0x01049fc0 ConVar staticProp_debug_draw
r5apex.exe!0x010486d0 ConVar staticProp_earlyDepthPrepass
r5apex.exe!0x0104c0f0 ConVar staticProp_earlyDepthPrepassDist
r5apex.exe!0x01049d40 ConVar staticProp_earlyDepthPrepassIncludeOpaques
r5apex.exe!0x0104ba20 ConVar staticProp_earlyDepthPrepassIncludeOpaquesDist
r5apex.exe!0x0104b2e0 ConVar staticProp_gather_size_weight
r5apex.exe!0x01048ba0 ConVar staticProp_max_scaled_dist
r5apex.exe!0x0104bac0 ConVar staticProp_no_fade_scalar
r5apex.exe!0x01d94550 ConVar staticProp_refineDrawOnWorker
r5apex.exe!0x011817b0 ConVar static_shadow
r5apex.exe!0x0181af70 ConVar static_shadow
r5apex.exe!0x01830bf0 ConVar static_shadow_bounds_per_env
r5apex.exe!0x01d8f1a0 ConVar static_shadow_debug_2d
r5apex.exe!0x0182e530 ConVar static_shadow_debug_dirty_rects
r5apex.exe!0x01bcb9b0 ConVar static_shadow_depth_bias_scale
r5apex.exe!0x018289f0 ConVar static_shadow_expand_z
r5apex.exe!0x0182e3f0 ConVar static_shadow_good_merge_ratio
r5apex.exe!0x0183a200 ConVar static_shadow_good_merge_score
r5apex.exe!0x01bc1ca0 ConVar static_shadow_prop_min_size
r5apex.exe!0x0118a410 ConVar static_shadow_res
r5apex.exe!0x0182ace0 ConVar static_shadow_shrink_culler
r5apex.exe!0x0118a690 ConVar static_shadow_use_d16
r5apex.exe!0x01825b20 ConVar static_shadow_uses_shadow_lod
r5apex.exe!0x0103f950 ConVar staticfile_hostname
r5apex.exe!0x01041730 ConVar stats_hostname
r5apex.exe!0x01dd7280 ConVar status_effect_warning_level
r5apex.exe!0x0117a870 ConVar steam_id
r5apex.exe!0x0117aa10 ConVar steam_name
r5apex.exe!0x0117ab50 ConVar steamlink_hostname
r5apex.exe!0x0128cb20 ConVar stream_addnoise
r5apex.exe!0x0128c780 ConVar stream_bsp_bucket_bias
r5apex.exe!0x0128cf80 ConVar stream_bsp_dist_scale
r5apex.exe!0x010353f0 ConVar stream_cache_capacity
r5apex.exe!0x01035530 ConVar stream_cache_capacity_while_loading
r5apex.exe!0x010350d0 ConVar stream_cache_high_priority_static_models
r5apex.exe!0x01034cd0 ConVar stream_cache_multithreaded
r5apex.exe!0x01034eb0 ConVar stream_cache_preload_from_rpak
r5apex.exe!0x01035350 ConVar stream_cache_read_buffer_cap
r5apex.exe!0x01035490 ConVar stream_cache_read_count_cap
r5apex.exe!0x010352b0 ConVar stream_cache_speculative_add_level
r5apex.exe!0x010355d0 ConVar stream_cache_speculative_drop
r5apex.exe!0x0128ca00 ConVar stream_drop_unused
r5apex.exe!0x0128c820 ConVar stream_enable
r5apex.exe!0x01184b50 ConVar stream_freeze_camera
r5apex.exe!0x0128c6e0 ConVar stream_load_after_drop
r5apex.exe!0x0128d0a0 ConVar stream_memory
r5apex.exe!0x0128cd00 ConVar stream_memory_ignore
r5apex.exe!0x0128cc60 ConVar stream_memory_ignore_vram
r5apex.exe!0x0128cbc0 ConVar stream_memory_while_loading
r5apex.exe!0x0128cda0 ConVar stream_mode
r5apex.exe!0x0128ce40 ConVar stream_never_high_priority_frac
r5apex.exe!0x0128d140 ConVar stream_overlay
r5apex.exe!0x0128cee0 ConVar stream_overlay_mode
r5apex.exe!0x0128c960 ConVar stream_pause
r5apex.exe!0x0128c8c0 ConVar stream_picmip
r5apex.exe!0x0128df70 ConVar stream_resource_max_commits_per_frame
r5apex.exe!0x0128e010 ConVar stream_resource_thread
r5apex.exe!0x0128ded0 ConVar stream_resource_wait_copy_to_commit
r5apex.exe!0x0128e0b0 ConVar stream_resource_wait_creation_to_copy
r5apex.exe!0x0128de30 ConVar stream_resource_wait_for_additional_gpus
r5apex.exe!0x01047230 ConVar stringtable_alwaysrebuilddictionaries
r5apex.exe!0x01041e10 ConVar stringtable_compress
r5apex.exe!0x01042ef0 ConVar stringtable_showsizes
r5apex.exe!0x01178070 ConVar stryder_forceOriginUsersInvisible
r5apex.exe!0x01177230 ConVar stryder_security
r5apex.exe!0x01d9aa50 ConVar stuck_debugging
r5apex.exe!0x01dcb4a0 ConVar stuck_debugging_world_only
r5apex.exe!0x01034750 ConVar studiobonecache_unlimited
r5apex.exe!0x01176fb0 ConVar subscription_hostname
r5apex.exe!0x01dc85d0 ConVar superjump_disabled_from_water
r5apex.exe!0x01dcfbd0 ConVar superjump_drain_power_onfail
r5apex.exe!0x01dc9a60 ConVar superjump_fail_sound_when_jump_limit
r5apex.exe!0x01dcbdf0 ConVar superjump_limit
r5apex.exe!0x01dc8130 ConVar superjump_limitreset_onwallrun
r5apex.exe!0x01db4890 ConVar superjump_max_power_use
r5apex.exe!0x01d9b1c0 ConVar superjump_min_height_fraction
r5apex.exe!0x01dcaca0 ConVar superjump_min_power_use
r5apex.exe!0x01dc6c80 ConVar superjump_powerreset_onground
r5apex.exe!0x01dca330 ConVar sv_airaccelerate
r5apex.exe!0x01040790 ConVar sv_allTicksFinal
r5apex.exe!0x01148510 ConVar sv_allowSendTableTransmitToClients
r5apex.exe!0x0114a310 ConVar sv_allowSpectatorClients
r5apex.exe!0x011494f0 ConVar sv_asyncSendSnapshot
r5apex.exe!0x01d99cc0 ConVar sv_backspeed
r5apex.exe!0x01149810 ConVar sv_balanceTeams
r5apex.exe!0x01db4710 ConVar sv_bounce
r5apex.exe!0x0114ba20 ConVar sv_cheats
r5apex.exe!0x01148bf0 ConVar sv_checkPropBudgets
r5apex.exe!0x01148f90 ConVar sv_compressPlaylists
r5apex.exe!0x01038130 ConVar sv_compressTimeValEpsilon
r5apex.exe!0x010382a0 ConVar sv_compressTimeVals
r5apex.exe!0x01148790 ConVar sv_connectingClientDelay
r5apex.exe!0x01149310 ConVar sv_debug_prop_send
r5apex.exe!0x0114b150 ConVar sv_debugmanualmode
r5apex.exe!0x0114acf0 ConVar sv_disconnectOnScriptError
r5apex.exe!0x0114b5b0 ConVar sv_disconnectOnTooManySnapshotFrames
r5apex.exe!0x0103d7a0 ConVar sv_dumpstringtables
r5apex.exe!0x0114b3d0 ConVar sv_earlyPersistenceRead
r5apex.exe!0x010432b0 ConVar sv_everyThirdTick
r5apex.exe!0x0114a930 ConVar sv_extra_client_connect_time
r5apex.exe!0x0114a7f0 ConVar sv_fakeClientBaseId
r5apex.exe!0x01dd1120 ConVar sv_footsteps
r5apex.exe!0x01d99ee0 ConVar sv_friction
r5apex.exe!0x01dd16b0 ConVar sv_gravity
r5apex.exe!0x011480b0 ConVar sv_hibernate_ms
r5apex.exe!0x01148db0 ConVar sv_hibernate_ms_vgui
r5apex.exe!0x0114b470 ConVar sv_hibernate_postgame_delay
r5apex.exe!0x0114ad90 ConVar sv_hibernate_when_empty
r5apex.exe!0x0114a4f0 ConVar sv_instancebaselines
r5apex.exe!0x0103ad50 ConVar sv_loadMapModelEarly
r5apex.exe!0x010446f0 ConVar sv_lobbyType
r5apex.exe!0x01148d10 ConVar sv_max_prop_data_dwords_huge_lobby
r5apex.exe!0x0114a750 ConVar sv_max_prop_data_dwords_huge_multiplayer
r5apex.exe!0x01148b50 ConVar sv_max_prop_data_dwords_lobby
r5apex.exe!0x01149590 ConVar sv_max_prop_data_dwords_multiplayer
r5apex.exe!0x011488d0 ConVar sv_max_prop_data_dwords_singleplayer
r5apex.exe!0x0114b830 ConVar sv_max_props_huge_lobby
r5apex.exe!0x01148150 ConVar sv_max_props_huge_multiplayer
r5apex.exe!0x01148ef0 ConVar sv_max_props_lobby
r5apex.exe!0x01148830 ConVar sv_max_props_multiplayer
r5apex.exe!0x01148650 ConVar sv_max_props_singleplayer
r5apex.exe!0x0114a3b0 ConVar sv_max_snapshots_lobby
r5apex.exe!0x01147a10 ConVar sv_max_snapshots_multiplayer
r5apex.exe!0x0114b6f0 ConVar sv_max_snapshots_singleplayer
r5apex.exe!0x01149150 ConVar sv_maxclientframes
r5apex.exe!0x0114ac50 ConVar sv_maxrate
r5apex.exe!0x01041190 ConVar sv_maxroutable
r5apex.exe!0x01dcdb40 ConVar sv_maxspeed
r5apex.exe!0x0114a1d0 ConVar sv_maxupdaterate
r5apex.exe!0x01dcdd80 ConVar sv_maxvelocity
r5apex.exe!0x01149030 ConVar sv_minrate
r5apex.exe!0x01149450 ConVar sv_minupdaterate
r5apex.exe!0x01db45d0 ConVar sv_noclipaccelerate
r5apex.exe!0x01dc8090 ConVar sv_noclipaccelerate_fast
r5apex.exe!0x01dd06e0 ConVar sv_noclipaccelerate_slow
r5apex.exe!0x01dcbd50 ConVar sv_noclipspeed
r5apex.exe!0x01db4390 ConVar sv_noclipspeed_fast
r5apex.exe!0x01d9b470 ConVar sv_noclipspeed_slow
r5apex.exe!0x01dc9f90 ConVar sv_optimizedmovement
r5apex.exe!0x01148330 ConVar sv_parallel_sendsnapshot
r5apex.exe!0x01148010 ConVar sv_pausable
r5apex.exe!0x01147830 ConVar sv_playerNameAppendCheater
r5apex.exe!0x01dcf7e0 ConVar sv_players
r5apex.exe!0x0114a630 ConVar sv_printHighWaterMark
r5apex.exe!0x01dc6dc0 ConVar sv_pushaway_accel
r5apex.exe!0x01d95f70 ConVar sv_pushaway_clientside
r5apex.exe!0x01dd7d30 ConVar sv_pushaway_clientside_size
r5apex.exe!0x01dce200 ConVar sv_pushaway_debug
r5apex.exe!0x01d96800 ConVar sv_pushaway_dist
r5apex.exe!0x01dc9b00 ConVar sv_pushaway_min_player_speed
r5apex.exe!0x01dc9c10 ConVar sv_pushaway_player_accel
r5apex.exe!0x01d9a3e0 ConVar sv_pushaway_player_dist
r5apex.exe!0x01149a70 ConVar sv_rejectClientConnects
r5apex.exe!0x01147db0 ConVar sv_rejectConnections
r5apex.exe!0x01148970 ConVar sv_requireOriginToken
r5apex.exe!0x01149b10 ConVar sv_resendSignonData
r5apex.exe!0x01dcec00 ConVar sv_rollangle
r5apex.exe!0x01dc8b10 ConVar sv_rollspeed
r5apex.exe!0x01147d10 ConVar sv_runSpatialOptimizeInJob
r5apex.exe!0x0114a270 ConVar sv_scarySnapDeltaPrints
r5apex.exe!0x0114af70 ConVar sv_sendEarlyServerInfo
r5apex.exe!0x01148e50 ConVar sv_sendReplayNetMessagesOnNoDeltaSnaps
r5apex.exe!0x01149c50 ConVar sv_separate_freq_change_prop_send
r5apex.exe!0x011481f0 ConVar sv_showClientTickCmds
r5apex.exe!0x01147ab0 ConVar sv_showLargeSnapshotSize
r5apex.exe!0x01147790 ConVar sv_showSnapshots
r5apex.exe!0x01147970 ConVar sv_showUserCmds
r5apex.exe!0x0114abb0 ConVar sv_single_core_dedi
r5apex.exe!0x01147c70 ConVar sv_skipSendingUnnecessaryPersistence
r5apex.exe!0x01dca260 ConVar sv_skyname
r5apex.exe!0x0114a130 ConVar sv_snapshot_uniform_interval
r5apex.exe!0x01dc9530 ConVar sv_specaccelerate
r5apex.exe!0x01d9b810 ConVar sv_specnoclip
r5apex.exe!0x01d99a60 ConVar sv_specspeed
r5apex.exe!0x011476f0 ConVar sv_stats
r5apex.exe!0x01d99e40 ConVar sv_stopspeed
r5apex.exe!0x0114aed0 ConVar sv_stressbots
r5apex.exe!0x0114b1f0 ConVar sv_struggleCheck
r5apex.exe!0x01148470 ConVar sv_struggleSpam
r5apex.exe!0x0114aa70 ConVar sv_struggleSpamInterval
r5apex.exe!0x01149630 ConVar sv_tempents_send_from_delta
r5apex.exe!0x011478d0 ConVar sv_tempents_send_from_last_sent
r5apex.exe!0x0117be10 ConVar sv_testLargeDatablock
r5apex.exe!0x0182ddc0 ConVar sv_teststepsimulation
r5apex.exe!0x0114ae30 ConVar sv_transmitToAllPlayersMask_allBitsSet
r5apex.exe!0x011499d0 ConVar sv_unnecessaryConnectDelay
r5apex.exe!0x011485b0 ConVar sv_unreliableSnapMaxSize
r5apex.exe!0x01147f70 ConVar sv_updaterate_mp
r5apex.exe!0x0114a450 ConVar sv_updaterate_sp
r5apex.exe!0x01ddac90 ConVar sv_useRK4forprojectiles
r5apex.exe!0x011486f0 ConVar sv_useReputation
r5apex.exe!0x0114a590 ConVar sv_useThreadsForSnapshots
r5apex.exe!0x011491f0 ConVar sv_voiceDebug
r5apex.exe!0x0114b010 ConVar sv_voiceEcho
r5apex.exe!0x0114a9d0 ConVar sv_voiceenable
r5apex.exe!0x01148ab0 ConVar sv_warnAboutCmdNumJumps
r5apex.exe!0x01047950 ConVar sv_watchdogTimer
r5apex.exe!0x01d95e30 ConVar sv_wateraccelerate
r5apex.exe!0x01dd0240 ConVar sv_waterdist
r5apex.exe!0x010456d0 ConVar sv_writePersistenceOnShutdown
r5apex.exe!0x01df2020 ConVar sys_attract_mode_timeout
r5apex.exe!0x0104a400 ConVar system_alt_f4_closes_window
r5apex.exe!0x01dcb3d0 ConVar teams_unassigned_are_friendly
r5apex.exe!0x01046180 ConVar telemetry_client_debug
r5apex.exe!0x0103e840 ConVar telemetry_client_enable
r5apex.exe!0x01043210 ConVar telemetry_client_sendInterval
r5apex.exe!0x01dd3070 ConVar test_fakeTimeDays
r5apex.exe!0x01dccb00 ConVar tether_damageScale
r5apex.exe!0x01d965a0 ConVar tether_dodge_damage
r5apex.exe!0x01d961f0 ConVar tether_healthDrain
r5apex.exe!0x01d9a160 ConVar tether_healthDrainNPC
r5apex.exe!0x01dcfb30 ConVar tether_maxvel
r5apex.exe!0x01dcf390 ConVar tether_radius
r5apex.exe!0x01dce2a0 ConVar tether_strength
r5apex.exe!0x01deb4d0 ConVar think_limit
r5apex.exe!0x01bd4830 ConVar thirdperson_mayamode
r5apex.exe!0x01bcbe30 ConVar thirdperson_override
r5apex.exe!0x01bdcf20 ConVar thirdperson_screenspace
r5apex.exe!0x0103a1e0 ConVar timeout
r5apex.exe!0x0103a280 ConVar timeout_during_load
r5apex.exe!0x01de2980 ConVar titan_sprint_sound
r5apex.exe!0x0114bd40 ConVar tracehull_height_error_check
r5apex.exe!0x01bd7f30 ConVar tracer_debug
r5apex.exe!0x01bcf610 ConVar tracer_extra
r5apex.exe!0x01dfabf0 ConVar trail_optimizedRemove
r5apex.exe!0x01d96290 ConVar traversal_anim
r5apex.exe!0x01d983a0 ConVar traversal_cooldown
r5apex.exe!0x01dc9260 ConVar traversal_enable
r5apex.exe!0x01d9a5c0 ConVar traversal_hand_debug
r5apex.exe!0x01dc97f0 ConVar traversal_hand_required_width
r5apex.exe!0x01dfacc0 ConVar traversal_viewLerpInDuration
r5apex.exe!0x01dfdfb0 ConVar traversal_viewLerpOut
r5apex.exe!0x01e00330 ConVar traversal_viewLerpOutAngle
r5apex.exe!0x01df6c20 ConVar traversal_viewLerpOutDebug
r5apex.exe!0x01e00930 ConVar traversal_viewLerpOutPos
r5apex.exe!0x01d996a0 ConVar traversal_window_duration
r5apex.exe!0x01d98440 ConVar traversal_window_enable
r5apex.exe!0x01d9a480 ConVar traversal_window_finish_angle
r5apex.exe!0x01dc83b0 ConVar traversal_window_forward_offset
r5apex.exe!0x01dcf500 ConVar traversal_window_hand_vertical_offset
r5apex.exe!0x01dcc1a0 ConVar traversal_window_sideways_offset
r5apex.exe!0x0182e2d0 ConVar traversal_window_view_pitch_max
r5apex.exe!0x018288b0 ConVar traversal_window_view_pitch_min
r5apex.exe!0x0183e630 ConVar traversal_window_yaw_max
r5apex.exe!0x01dcd560 ConVar trigger_ignore_nonsolids
r5apex.exe!0x0180ff00 ConVar tsaa_blendfactorincreaseatmaxvelocity
r5apex.exe!0x0180ffa0 ConVar tsaa_blendfactorincreasewhenunoccluded
r5apex.exe!0x018100e0 ConVar tsaa_blendfactormaxesoutatvelocity
r5apex.exe!0x018102c0 ConVar tsaa_blendfactormodulationonsparklesandunocclusion
r5apex.exe!0x01810180 ConVar tsaa_blendfactoroverride
r5apex.exe!0x01810040 ConVar tsaa_curframeblendamount
r5apex.exe!0x01810220 ConVar tsaa_debugresponsiveflag
r5apex.exe!0x0180fdc0 ConVar tsaa_neighborhoodclamping
r5apex.exe!0x0180fe60 ConVar tsaa_neighborhoodclampingsoftened
r5apex.exe!0x01d925a0 ConVar tsaa_numsamples
r5apex.exe!0x01bdde10 ConVar tweak_light_shadows_every_frame
r5apex.exe!0x0110e7a0 ConVar twitch_check_interval
r5apex.exe!0x0110f8f0 ConVar twitch_prime_rewards
r5apex.exe!0x0110ed80 ConVar twitch_shouldQuery
r5apex.exe!0x01df1e00 ConVar ui_fadecloud_time
r5apex.exe!0x01df1d60 ConVar ui_fadexui_time
r5apex.exe!0x01df1040 ConVar ui_gameui_ctrlr_title
r5apex.exe!0x01df17c0 ConVar ui_gameui_modal
r5apex.exe!0x01df12c0 ConVar ui_loadingscreen_autotransition_time
r5apex.exe!0x01df19a0 ConVar ui_loadingscreen_fadein_time
r5apex.exe!0x01038d80 ConVar ui_loadingscreen_fadeout_time
r5apex.exe!0x01df1a40 ConVar ui_loadingscreen_fadeout_time
r5apex.exe!0x01df23e0 ConVar ui_loadingscreen_mintransition_time
r5apex.exe!0x01df15e0 ConVar ui_loadingscreen_transition_time
r5apex.exe!0x01df1220 ConVar ui_lobby_jointimeout
r5apex.exe!0x01df2340 ConVar ui_lobby_noautostart
r5apex.exe!0x01df1cc0 ConVar ui_lobby_noresults_create_msg_time
r5apex.exe!0x01dee670 ConVar ui_posedebug_fade_in_time
r5apex.exe!0x01dee5d0 ConVar ui_posedebug_fade_out_time
r5apex.exe!0x01df1180 ConVar ui_virtualnav_render
r5apex.exe!0x01d96e40 ConVar unique_entity_names
r5apex.exe!0x01df3c50 ConVar usePromptBaseColor
r5apex.exe!0x01df5980 ConVar usePromptButtonTextColor
r5apex.exe!0x01df39b0 ConVar usePromptImageScale
r5apex.exe!0x01df54b0 ConVar usePromptImageYOffset
r5apex.exe!0x01df6580 ConVar usePromptTextColor
r5apex.exe!0x01d90bf0 ConVar use_monitors
r5apex.exe!0x0104c370 ConVar use_valve_auto_gain
r5apex.exe!0x011779b0 ConVar user_tracking_enabled
r5apex.exe!0x0103d660 ConVar users_hostname
r5apex.exe!0x0181a0c0 ConVar v_centermove
r5apex.exe!0x01828950 ConVar v_centerspeed
r5apex.exe!0x01dda650 ConVar variable_sights_gravity_scale_override
r5apex.exe!0x01bdc4d0 ConVar vehicle_predictViaPlayer
r5apex.exe!0x01814040 ConVar vgui_EnableFixedAspectScaling
r5apex.exe!0x01df1ae0 ConVar vgui_drawPolyShapes
r5apex.exe!0x010488b0 ConVar vgui_drawfocus
r5apex.exe!0x01813c90 ConVar vgui_drawfocus
r5apex.exe!0x0104a700 ConVar vgui_drawkeyfocus
r5apex.exe!0x01049ac0 ConVar vgui_drawtree
r5apex.exe!0x0104b1a0 ConVar vgui_drawtree_bounds
r5apex.exe!0x0104c050 ConVar vgui_drawtree_draw_selected
r5apex.exe!0x0104a540 ConVar vgui_drawtree_freeze
r5apex.exe!0x01047c90 ConVar vgui_drawtree_hidden
r5apex.exe!0x0104ab60 ConVar vgui_drawtree_panelalpha
r5apex.exe!0x01049040 ConVar vgui_drawtree_panelptr
r5apex.exe!0x01048d20 ConVar vgui_drawtree_popupsonly
r5apex.exe!0x01047d30 ConVar vgui_drawtree_render_order
r5apex.exe!0x0104b700 ConVar vgui_drawtree_scheme
r5apex.exe!0x0104a060 ConVar vgui_drawtree_visible
r5apex.exe!0x0181a520 ConVar vgui_interactive
r5apex.exe!0x01815a00 ConVar vgui_noquads
r5apex.exe!0x018158c0 ConVar vgui_notext
r5apex.exe!0x01813d30 ConVar vgui_resize_on_resolution_change
r5apex.exe!0x01814640 ConVar vgui_show_glyph_miss
r5apex.exe!0x010451d0 ConVar vgui_simulate_during_bone_setup
r5apex.exe!0x01df49a0 ConVar video_menu_uiscript_reset
r5apex.exe!0x01dda450 ConVar viewDrift
r5apex.exe!0x01ddd420 ConVar viewDrift_ads_delay_debounce_time
r5apex.exe!0x01dde1c0 ConVar viewDrift_pitch_base1_amp
r5apex.exe!0x01dddaa0 ConVar viewDrift_pitch_base1_freq
r5apex.exe!0x01dd8d60 ConVar viewDrift_pitch_base1_phase
r5apex.exe!0x01ddbd20 ConVar viewDrift_pitch_base2_amp
r5apex.exe!0x01de2720 ConVar viewDrift_pitch_base2_freq
r5apex.exe!0x01dda3b0 ConVar viewDrift_pitch_base2_phase
r5apex.exe!0x01dd7dd0 ConVar viewDrift_pitch_scaler_amp
r5apex.exe!0x01deb220 ConVar viewDrift_pitch_scaler_base
r5apex.exe!0x01dd76b0 ConVar viewDrift_pitch_scaler_freq
r5apex.exe!0x01dd7320 ConVar viewDrift_pitch_scaler_phase
r5apex.exe!0x01ddd1d0 ConVar viewDrift_pitch_shifter_amp
r5apex.exe!0x01ddd020 ConVar viewDrift_pitch_shifter_freq
r5apex.exe!0x01de1bf0 ConVar viewDrift_pitch_shifter_phase
r5apex.exe!0x01deb180 ConVar viewDrift_yaw_base1_amp
r5apex.exe!0x01de1610 ConVar viewDrift_yaw_base1_freq
r5apex.exe!0x01dd2a70 ConVar viewDrift_yaw_base1_phase
r5apex.exe!0x01dda4f0 ConVar viewDrift_yaw_base2_amp
r5apex.exe!0x01ddc470 ConVar viewDrift_yaw_base2_freq
r5apex.exe!0x01ddc3d0 ConVar viewDrift_yaw_base2_phase
r5apex.exe!0x01dd9040 ConVar viewDrift_yaw_scaler_amp
r5apex.exe!0x01de16b0 ConVar viewDrift_yaw_scaler_base
r5apex.exe!0x01de2080 ConVar viewDrift_yaw_scaler_freq
r5apex.exe!0x01de1990 ConVar viewDrift_yaw_scaler_phase
r5apex.exe!0x01dd2b90 ConVar viewDrift_yaw_shifter_amp
r5apex.exe!0x01ddc2a0 ConVar viewDrift_yaw_shifter_freq
r5apex.exe!0x01deb360 ConVar viewDrift_yaw_shifter_phase
r5apex.exe!0x01debb40 ConVar view_offset_entity_enable
r5apex.exe!0x01be0090 ConVar viewangle_debug
r5apex.exe!0x01829870 ConVar viewangles_simpler
r5apex.exe!0x0181a9e0 ConVar viewmodelShake
r5apex.exe!0x0183ab00 ConVar viewmodelShake_sourceRollRange
r5apex.exe!0x01dd0110 ConVar viewmodel_attachment_fov_fix
r5apex.exe!0x01d91890 ConVar viewmodel_bounds_draw
r5apex.exe!0x01be2a40 ConVar viewmodel_bounds_draw_lock
r5apex.exe!0x0118a730 ConVar viewmodel_selfshadow
r5apex.exe!0x01d90f50 ConVar viewmodel_selfshadow_debug_2d
r5apex.exe!0x01be1780 ConVar viewmodel_selfshadow_tightbounds
r5apex.exe!0x01825940 ConVar viewportscale
r5apex.exe!0x01dc6750 ConVar viewpunch_base_springConstantX
r5apex.exe!0x01dce4e0 ConVar viewpunch_base_springConstantY
r5apex.exe!0x01dcd900 ConVar viewpunch_base_springConstantZ
r5apex.exe!0x01db4930 ConVar viewpunch_base_springDampingX
r5apex.exe!0x01dcdc40 ConVar viewpunch_base_springDampingY
r5apex.exe!0x01d9a910 ConVar viewpunch_base_springDampingZ
r5apex.exe!0x0103c960 ConVar violence_ablood
r5apex.exe!0x01dd6380 ConVar violence_ablood
r5apex.exe!0x0103f370 ConVar violence_agibs
r5apex.exe!0x01dd8f80 ConVar violence_agibs
r5apex.exe!0x01046650 ConVar violence_hblood
r5apex.exe!0x01de27c0 ConVar violence_hblood
r5apex.exe!0x01041050 ConVar violence_hgibs
r5apex.exe!0x01ddbbb0 ConVar violence_hgibs
r5apex.exe!0x01de28a0 ConVar visible_ent_cone_debug_duration_client
r5apex.exe!0x0104d6c0 ConVar voice_absTriggerAmount
r5apex.exe!0x01df69c0 ConVar voice_allow_mute_self
r5apex.exe!0x0104c7d0 ConVar voice_avggain
r5apex.exe!0x01ded3c0 ConVar voice_clientdebug
r5apex.exe!0x0104cb90 ConVar voice_debugAddSecondTalker
r5apex.exe!0x0104d270 ConVar voice_debugThresholds
r5apex.exe!0x01048950 ConVar voice_debugfeedback
r5apex.exe!0x01df5200 ConVar voice_decimate_at_bytes
r5apex.exe!0x01df6880 ConVar voice_decimate_rate
r5apex.exe!0x0104cff0 ConVar voice_enabled
r5apex.exe!0x0104ccd0 ConVar voice_energyPerZeroThreshold
r5apex.exe!0x0104d1d0 ConVar voice_energyThreshold
r5apex.exe!0x0104d090 ConVar voice_forcemicrecord
r5apex.exe!0x0103e0e0 ConVar voice_inputfromfile
r5apex.exe!0x0181a020 ConVar voice_late_update
r5apex.exe!0x0104d450 ConVar voice_loopback
r5apex.exe!0x0104d620 ConVar voice_maxgain
r5apex.exe!0x0104ce10 ConVar voice_minEnergyPerZeroThreshold
r5apex.exe!0x0104d760 ConVar voice_mixer_boost
r5apex.exe!0x0104d800 ConVar voice_mixer_mute
r5apex.exe!0x0104d8a0 ConVar voice_mixer_volume
r5apex.exe!0x01deda10 ConVar voice_modenable
r5apex.exe!0x0114b0b0 ConVar voice_noxplat
r5apex.exe!0x0104d580 ConVar voice_profile
r5apex.exe!0x010403d0 ConVar voice_recordtofile
r5apex.exe!0x0104c910 ConVar voice_scale
r5apex.exe!0x0104cf50 ConVar voice_showchannels
r5apex.exe!0x0104c5f0 ConVar voice_showincoming
r5apex.exe!0x0104c690 ConVar voice_threshold_delay
r5apex.exe!0x0104d3b0 ConVar voice_triggerCrossingRate
r5apex.exe!0x0104caf0 ConVar voice_triggerRate
r5apex.exe!0x0104d130 ConVar voice_vox
r5apex.exe!0x0104c4b0 ConVar voice_writevoices
r5apex.exe!0x0103db60 ConVar voice_xsend_debug
r5apex.exe!0x0104c9b0 ConVar voice_zeroCrossingThreshold
r5apex.exe!0x01df9530 ConVar vortex_damageimpulsescale
r5apex.exe!0x01046a10 ConVar vprof_server_spike_threshold
r5apex.exe!0x0103fa70 ConVar vprof_server_thread
r5apex.exe!0x01d92840 ConVar vscript_ui_do_delay_init
r5apex.exe!0x01839310 ConVar vsm_culling
r5apex.exe!0x01bc1660 ConVar vsm_ignore_edge_planes
r5apex.exe!0x0181b0b0 ConVar vsm_ignore_face_planes
r5apex.exe!0x01048450 ConVar vx_do_not_throttle_events
r5apex.exe!0x01dda5b0 ConVar wall_climb_pose_paramteter_hands_enabled
r5apex.exe!0x01dcd2f0 ConVar wallclimb_vertical_gain_reduction
r5apex.exe!0x01dd04e0 ConVar wallrun_angleChangeMinCos
r5apex.exe!0x01dcfa60 ConVar wallrun_avoid_wall_top_decel
r5apex.exe!0x01dfaf40 ConVar wallrun_curveDebug
r5apex.exe!0x01df9cb0 ConVar wallrun_curveEnable
r5apex.exe!0x01dcb650 ConVar wallrun_debug
r5apex.exe!0x01d9b5d0 ConVar wallrun_enable
r5apex.exe!0x01dcc070 ConVar wallrun_fallAwaySpeed
r5apex.exe!0x01dcba90 ConVar wallrun_hangStopTime
r5apex.exe!0x01d98520 ConVar wallrun_hangslipduration
r5apex.exe!0x01dc9390 ConVar wallrun_hangslipstarttime
r5apex.exe!0x01dc7b10 ConVar wallrun_hangslipvel
r5apex.exe!0x01dc6390 ConVar wallrun_maxViewTilt
r5apex.exe!0x01db4670 ConVar wallrun_minAngle_air
r5apex.exe!0x01dcbec0 ConVar wallrun_noInputSlipFrac
r5apex.exe!0x01d99da0 ConVar wallrun_pushAwayFallOffTime
r5apex.exe!0x01dc4fa0 ConVar wallrun_repelEnable
r5apex.exe!0x01dc8530 ConVar wallrun_repelSoftness
r5apex.exe!0x01dceee0 ConVar wallrun_repelTimeMax
r5apex.exe!0x01dc9490 ConVar wallrun_repelTimeMin
r5apex.exe!0x01dc6e60 ConVar wallrun_retry_interval
r5apex.exe!0x01dc7310 ConVar wallrun_rotateMaxRate
r5apex.exe!0x01dc64d0 ConVar wallrun_sameWallDist
r5apex.exe!0x01dc6d20 ConVar wallrun_sameWallDot
r5apex.exe!0x01d96500 ConVar wallrun_sameWallSlope
r5apex.exe!0x01dced70 ConVar wallrun_slipduration
r5apex.exe!0x01dc71d0 ConVar wallrun_slipslowdown
r5apex.exe!0x01dcb1c0 ConVar wallrun_slipstarttime
r5apex.exe!0x01dcd6a0 ConVar wallrun_slipvel
r5apex.exe!0x01d9a2a0 ConVar wallrun_strengthLossEnd
r5apex.exe!0x01d999c0 ConVar wallrun_strengthLossStart
r5apex.exe!0x01dc9890 ConVar wallrun_upwardAutoPush
r5apex.exe!0x01dcdce0 ConVar wallrun_viewTiltPredictTime
r5apex.exe!0x01d9a9b0 ConVar wallrun_viewTiltSpeed
r5apex.exe!0x01dc8670 ConVar was_loaded
r5apex.exe!0x01e019d0 ConVar weaponFastHolsterScale
r5apex.exe!0x01dcf740 ConVar weaponSwitch3p_checkNewWeapon
r5apex.exe!0x01e051f0 ConVar weaponSwitch3p_onHolster
r5apex.exe!0x01ddb6d0 ConVar weapon_auto_swap_ordnance_no_ammo
r5apex.exe!0x01e04d70 ConVar weapon_debugScript
r5apex.exe!0x01e04750 ConVar weapon_doIdleForSurvivalMelee
r5apex.exe!0x0183e9d0 ConVar weapon_friendly_fire_prevent_ui
r5apex.exe!0x01de1550 ConVar weapon_meleeButtonPressProtection
r5apex.exe!0x01bc1b60 ConVar weapon_parentingFixLerp
r5apex.exe!0x01de21a0 ConVar weapon_pickup_allow_dupes
r5apex.exe!0x0181ac60 ConVar weapon_poseParamMaxDistance
r5apex.exe!0x01e050f0 ConVar weapon_render_with_fastpath
r5apex.exe!0x01bda740 ConVar weapon_setting_autocycle_on_empty
r5apex.exe!0x01e04e50 ConVar weapon_sprint_raise_delay
r5apex.exe!0x01e05810 ConVar weaponx_predicting_client_only_optimization
r5apex.exe!0x01e058d0 ConVar weaponx_smartammo_data_optimization
r5apex.exe!0x01e00bd0 ConVar window_hint_debug
r5apex.exe!0x01db4c90 ConVar window_hint_fov_down
r5apex.exe!0x01dcd220 ConVar window_hint_fov_horz
r5apex.exe!0x01dc6fa0 ConVar window_hint_fov_up
r5apex.exe!0x01dcb330 ConVar window_hint_keyboard_fov_horz
r5apex.exe!0x01d98740 ConVar window_hint_lookahead_time
r5apex.exe!0x01dcf460 ConVar window_hint_max_horz_vel_change_dot
r5apex.exe!0x01dc81d0 ConVar window_hint_max_vel_change_down
r5apex.exe!0x01dceb60 ConVar window_hint_max_vel_change_up
r5apex.exe!0x01d9b950 ConVar window_hint_min_horz_vel
r5apex.exe!0x01dcfe70 ConVar window_hint_permissive_max_horz_vel_change_dot
r5apex.exe!0x01dcb260 ConVar window_hint_permissive_max_vel_change_down
r5apex.exe!0x01dc62f0 ConVar window_hint_permissive_max_vel_change_up
r5apex.exe!0x0183a8a0 ConVar z_ragdoll_impact_strength
r5apex.exe!0x01df9490 ConVar zipline_fade_dist
r5apex.exe!0x01df9d50 ConVar zipline_subdiv_lod_dist_base
r5apex.exe!0x01bdea30 ConVar zipline_subdiv_slices
r5apex.exe!0x01dffc50 ConVar zipline_subdiv_slices_lod
r5apex.exe!0x01bd6dc0 ConVar zipline_subdiv_stacks
```

## ConCommands

<details>
<summary><code>+ability</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>+ability_held</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>+attack</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+backward</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>+break</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+camdistance</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>+camin</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>+cammousemove</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>+camout</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>+campitchdown</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>+campitchup</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>+camyawleft</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>+camyawright</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>+commandermousemove</code></summary>



flags: `0x80000`  
</details>
<details>
<summary><code>+csm_rot_x_neg</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>+csm_rot_x_plus</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>+csm_rot_y_neg</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>+csm_rot_y_plus</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>+displayFullscreenMap</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+dodge</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>+duck</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>+forward</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>+graph</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+jump</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>+klook</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+left</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+lookdown</code></summary>



flags: `0x80000`  
</details>
<details>
<summary><code>+lookup</code></summary>



flags: `0x80000`  
</details>
<details>
<summary><code>+mat_texture_list</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>+melee</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+movedown</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+moveleft</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>+moveright</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>+moveup</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+offhand0</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+offhand1</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+offhand2</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+offhand3</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+offhand4</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+pause_menu</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+ping</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>+posedebug</code></summary>

Turn on pose debugger or add ents to pose debugger UI

flags: `0x4000`  
</details>
<details>
<summary><code>+pushtotalk</code></summary>



flags: `0x40000000`  
</details>
<details>
<summary><code>+reload</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+right</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+score</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+scriptCommand1</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+scriptCommand2</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+scriptCommand3</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+scriptCommand4</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+scriptCommand5</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+scriptCommand6</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+scriptCommand7</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+scriptCommand8</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+scriptCommand9</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+showscores</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+speed</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+strafe</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+toggle_duck</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>+toggle_zoom</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+use</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+useAndReload</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+use_alt</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+use_long</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+variableScopeToggle</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>+vgui_drawtree</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>+voicerecord</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>+walk</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+weaponCycle</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+weapon_discard</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>+zoom</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-ability</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>-ability_held</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>-attack</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-backward</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>-break</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-camdistance</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>-camin</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>-cammousemove</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>-camout</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>-campitchdown</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>-campitchup</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>-camyawleft</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>-camyawright</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>-commandermousemove</code></summary>



flags: `0x80000`  
</details>
<details>
<summary><code>-csm_rot_x_neg</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>-csm_rot_x_plus</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>-csm_rot_y_neg</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>-csm_rot_y_plus</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>-displayFullscreenMap</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-dodge</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>-duck</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>-forward</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>-graph</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-jump</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>-klook</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-left</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-lookdown</code></summary>



flags: `0x80000`  
</details>
<details>
<summary><code>-lookup</code></summary>



flags: `0x80000`  
</details>
<details>
<summary><code>-mat_texture_list</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>-melee</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-movedown</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-moveleft</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>-moveright</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>-moveup</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-offhand0</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-offhand1</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-offhand2</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-offhand3</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-offhand4</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-pause_menu</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-ping</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>-posedebug</code></summary>

Turn off pose debugger or hide ents from pose debugger UI

flags: `0x4000`  
</details>
<details>
<summary><code>-pushtotalk</code></summary>



flags: `0x40000000`  
</details>
<details>
<summary><code>-reload</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-right</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-score</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-scriptCommand1</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-scriptCommand2</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-scriptCommand3</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-scriptCommand4</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-scriptCommand5</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-scriptCommand6</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-scriptCommand7</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-scriptCommand8</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-scriptCommand9</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-showscores</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-speed</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-strafe</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-toggle_duck</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>-toggle_zoom</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-use</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-useAndReload</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-use_alt</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-use_long</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-variableScopeToggle</code></summary>



flags: `0x400a0000`  
</details>
<details>
<summary><code>-vgui_drawtree</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>-voicerecord</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>-walk</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-weaponCycle</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-weapon_discard</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>-zoom</code></summary>



flags: `0x40000000`  
</details>
<details>
<summary><code>BindToggle</code></summary>

Performs a bind <key> "increment var <cvar> 0 1 1"

flags: `0x2`  
</details>
<details>
<summary><code>CMaterialSystem_clear_loading</code></summary>



flags: `0x20002`  
</details>
<details>
<summary><code>CMaterialSystem_set_loading</code></summary>



flags: `0x20002`  
</details>
<details>
<summary><code>DebugPrintUsedTextures</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>DumpClientDataBlockReceiver</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>MemTrackDeltaSnapshot</code></summary>

Debug command compares two snapshots. Takes indices into the snapshot array, negative means from end

flags: `0x2`  
</details>
<details>
<summary><code>MemTrackPrintStats</code></summary>

Debug command prints current mem stats & creates a named snapshot - first param is snapshot name

flags: `0x2`  
</details>
<details>
<summary><code>ReloadAimAssistSettings</code></summary>

Reloads aimassist config files.

flags: `0xa`  
</details>
<details>
<summary><code>adminmsg</code></summary>

Send text to the current community (if you are an admin)

flags: `0x2`  
</details>
<details>
<summary><code>aisettings_reparse_client</code></summary>

Reloads the AI settings files

flags: `0xa`  
</details>
<details>
<summary><code>alias</code></summary>

Alias a command.

flags: `0x2`  
</details>
<details>
<summary><code>applyVideoChangesDeferred</code></summary>

Workaround for applying video changes using controller buttons shortcuts.

flags: `0x40000008`  
</details>
<details>
<summary><code>bind</code></summary>

Bind a key to TAPPED.

flags: `0x40000000`  
</details>
<details>
<summary><code>bind_US_standard</code></summary>

Bind a key to TAPPED. Given a key on a standard US keyboard, this function will translate that key to the appropriate key on the user's current keyboard.

flags: `0x40000000`  
</details>
<details>
<summary><code>bind_held</code></summary>

Bind a key to HELD.

flags: `0x40000000`  
</details>
<details>
<summary><code>bind_held_US_standard</code></summary>

Bind a key to HELD.. Given a key on a standard US keyboard, this function will translate that key to the appropriate key on the user's current keyboard.

flags: `0x40000000`  
</details>
<details>
<summary><code>bind_list</code></summary>

List all current bindings.

flags: `0x2`  
</details>
<details>
<summary><code>bind_list_abilities</code></summary>

List all ability bindings and what commands they resolve to

flags: `0x2`  
</details>
<details>
<summary><code>bink_dump_precached_movies</code></summary>

Dumps information about all precached Bink movies

flags: `0x2`  
</details>
<details>
<summary><code>bot_loadout</code></summary>

Override bot loadout

flags: `0x4008`  
</details>
<details>
<summary><code>box</code></summary>

Draw a debug box.

flags: `0x4000`  
</details>
<details>
<summary><code>buildcubemaps</code></summary>

Rebuild cubemaps.

flags: `0x2`  
</details>
<details>
<summary><code>cache_print</code></summary>

cache_print [section]
Print out contents of cache memory.

flags: `0x2`  
</details>
<details>
<summary><code>cache_print_lru</code></summary>

cache_print_lru [section]
Print out contents of cache memory.

flags: `0x2`  
</details>
<details>
<summary><code>cache_print_summary</code></summary>

cache_print_summary [section]
Print out a summary contents of cache memory.

flags: `0x2`  
</details>
<details>
<summary><code>cam_command</code></summary>

Tells camera to change modes

flags: `0x4008`  
</details>
<details>
<summary><code>cancelselect</code></summary>



flags: `0x10000002`  
</details>
<details>
<summary><code>cc_emit</code></summary>

Emits a closed caption

flags: `0xa`  
</details>
<details>
<summary><code>centerview</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>changelevel</code></summary>

Change server to the specified map

flags: `0x20002`  
</details>
<details>
<summary><code>chat</code></summary>

Send text to the current chatroom

flags: `0x2`  
</details>
<details>
<summary><code>chat_wheel</code></summary>

Opens the chat wheel

flags: `0x40000008`  
</details>
<details>
<summary><code>chatroom_adminsOnly</code></summary>

Set the chatroom to be admins-only

flags: `0x2`  
</details>
<details>
<summary><code>chatroom_away</code></summary>

Tell the chatserver you are away from the room

flags: `0x2`  
</details>
<details>
<summary><code>chatroom_freetalk</code></summary>

Set the chatroom to be free talk

flags: `0x2`  
</details>
<details>
<summary><code>chatroom_present</code></summary>

Tell the chatserver you are present in the room

flags: `0x2`  
</details>
<details>
<summary><code>chatserver</code></summary>

Connect to a chatserver

flags: `0x40000000`  
</details>
<details>
<summary><code>chroma_base</code></summary>

Transitions to a new base layer for chroma hardware

flags: `0x2`  
</details>
<details>
<summary><code>chroma_layer</code></summary>

Adds an overlay layer for chroma hardware

flags: `0x2`  
</details>
<details>
<summary><code>cl_dump_particle_stats</code></summary>

dump particle profiling info to particle_profile.csv

flags: `0x2`  
</details>
<details>
<summary><code>cl_ent_absbox</code></summary>

Displays the client's absbox for the entity under the crosshair.

flags: `0x4008`  
</details>
<details>
<summary><code>cl_ent_bbox</code></summary>

Displays the client's bounding box for the entity under the crosshair.

flags: `0x4008`  
</details>
<details>
<summary><code>cl_ent_rbox</code></summary>

Displays the client's render box for the entity under the crosshair.

flags: `0x4008`  
</details>
<details>
<summary><code>cl_find_ent</code></summary>

Find and list all client entities with classnames that contain the specified substring.
Format: cl_find_ent <substring>


flags: `0x4000`  
</details>
<details>
<summary><code>cl_find_ent_index</code></summary>

Display data for clientside entity matching specified index.
Format: cl_find_ent_index <index>


flags: `0x4000`  
</details>
<details>
<summary><code>cl_flip_visibility</code></summary>

Flips the visibilityBits of all Entities

flags: `0x4000`  
</details>
<details>
<summary><code>cl_fullupdate</code></summary>

Forces the server to send a full update packet

flags: `0x4000`  
</details>
<details>
<summary><code>cl_interpolation_report</code></summary>

Prints all entities being interpolated on the next frame

flags: `0xa`  
</details>
<details>
<summary><code>cl_panelanimation</code></summary>

Shows panel animation variables: <panelname | blank for all panels>.

flags: `0xa`  
</details>
<details>
<summary><code>cl_particles_dump_effects</code></summary>



flags: `0xa`  
</details>
<details>
<summary><code>cl_particles_dumplist</code></summary>

Dump all new particles, optional name substring.

flags: `0xa`  
</details>
<details>
<summary><code>cl_precacheinfo</code></summary>

Show precache info (client).

flags: `0x2`  
</details>
<details>
<summary><code>cl_removedecals</code></summary>

Remove the decals from the entity under the crosshair.

flags: `0x4000`  
</details>
<details>
<summary><code>cl_showents</code></summary>

Dump entity list to console.

flags: `0x4000`  
</details>
<details>
<summary><code>cl_soundscape_flush</code></summary>

Flushes the client side soundscapes

flags: `0x10004008`  
</details>
<details>
<summary><code>cl_trace_start_solid</code></summary>

Trace with given parameters and return start solid result

flags: `0xa`  
</details>
<details>
<summary><code>cl_trace_test_hitbox_with_non_zero_start_offset</code></summary>



flags: `0xa`  
</details>
<details>
<summary><code>cl_updatevisibility</code></summary>

Updates visibility bits.

flags: `0xa`  
</details>
<details>
<summary><code>clear_loading_progress_detente</code></summary>

Clears the detente for the load screen.

flags: `0x20002`  
</details>
<details>
<summary><code>clear_loading_progress_sp_text</code></summary>

Clears the sp text for the load screen.

flags: `0x20002`  
</details>
<details>
<summary><code>cm_query_log_record</code></summary>

Start recording a log of all queries

flags: `0x2`  
</details>
<details>
<summary><code>cm_query_log_replay</code></summary>

Play back a query log for performance testing

flags: `0x2`  
</details>
<details>
<summary><code>cmd</code></summary>

Forward command to server.

flags: `0x2`  
</details>
<details>
<summary><code>cmd1</code></summary>

sets userinfo string for split screen player in slot 1

flags: `0x40000000`  
</details>
<details>
<summary><code>cmd2</code></summary>

sets userinfo string for split screen player in slot 2

flags: `0x2`  
</details>
<details>
<summary><code>cmd3</code></summary>

sets userinfo string for split screen player in slot 3

flags: `0x2`  
</details>
<details>
<summary><code>cmd4</code></summary>

sets userinfo string for split screen player in slot 4

flags: `0x2`  
</details>
<details>
<summary><code>collision_debug</code></summary>

Sends a collision ray from player and gathers info.

flags: `0x4008`  
</details>
<details>
<summary><code>colorcorrectionui</code></summary>

Show/hide the color correction tools UI.

flags: `0x4000`  
</details>
<details>
<summary><code>community_browse</code></summary>

Browse available communities

flags: `0x2`  
</details>
<details>
<summary><code>community_getPendingJoinRequest</code></summary>

Get a random pending join request to answer

flags: `0x2`  
</details>
<details>
<summary><code>community_join</code></summary>

Join a community

flags: `0x2`  
</details>
<details>
<summary><code>community_leave</code></summary>

Leave a community

flags: `0x2`  
</details>
<details>
<summary><code>community_list</code></summary>

list my communities

flags: `0x2`  
</details>
<details>
<summary><code>community_report</code></summary>

Report a community

flags: `0x2`  
</details>
<details>
<summary><code>connect</code></summary>

Connect to specified server.

flags: `0x20002`  
</details>
<details>
<summary><code>connectAsSpectator</code></summary>

Connect to specified server as a spectator

flags: `0x20002`  
</details>
<details>
<summary><code>connectWithKey</code></summary>

Connect to specified server with an explicit encryption key.

flags: `0x20002`  
</details>
<details>
<summary><code>connectwithtoken</code></summary>

Connect to specified server with a reservation token.

flags: `0xa0000`  
</details>
<details>
<summary><code>convar_differences</code></summary>

Show all convars which are not at their default values.

flags: `0x2`  
</details>
<details>
<summary><code>convar_findByFlags</code></summary>

Find concommands by flags.

flags: `0x2`  
</details>
<details>
<summary><code>convar_list</code></summary>

Show the list of convars/concommands.

flags: `0x2`  
</details>
<details>
<summary><code>createparty</code></summary>

Create a party

flags: `0x2`  
</details>
<details>
<summary><code>createpartyifnotinone</code></summary>

Create a party if we aren't in one

flags: `0x2`  
</details>
<details>
<summary><code>csm_status</code></summary>

Usage:
   csm_status


flags: `0x2`  
</details>
<details>
<summary><code>damagedefs_reparse_client</code></summary>

Reloads the damage defs

flags: `0xa`  
</details>
<details>
<summary><code>debugModelPurge</code></summary>

Debug command to purge unused models...

flags: `0x2`  
</details>
<details>
<summary><code>devshots_nextmap</code></summary>

Used by the devshots system to go to the next map in the devshots maplist.

flags: `0x2`  
</details>
<details>
<summary><code>devshots_screenshot</code></summary>

Used by the -makedevshots system to take a screenshot. For taking your own screenshots, use the 'screenshot' command instead.

flags: `0x20002`  
</details>
<details>
<summary><code>dfs_print_flag_states</code></summary>

Prints all dfs flag states to console

flags: `0x2`  
</details>
<details>
<summary><code>disconnect</code></summary>

Disconnect game from server.

flags: `0x48000000`  
</details>
<details>
<summary><code>display_elapsedtime</code></summary>

Displays how much time has elapsed since the game started

flags: `0x4000`  
</details>
<details>
<summary><code>dlight_debug</code></summary>

Creates a dlight in front of the player

flags: `0x4008`  
</details>
<details>
<summary><code>do_InvitePeople_test</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>do_Invite_friend_test</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>do_joinPeople_test</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>do_origin_test_presence</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>downloadPlaylists</code></summary>

Re-download the playlists


flags: `0x2`  
</details>
<details>
<summary><code>dumpClientStringTable</code></summary>

Dump the contents of the client's game string table to the console.

flags: `0x4000`  
</details>
<details>
<summary><code>dumpstringtables</code></summary>

Print string tables to console.

flags: `0x2`  
</details>
<details>
<summary><code>echo</code></summary>

Echo text to console.

flags: `0x10000002`  
</details>
<details>
<summary><code>echo_error</code></summary>

Echo error text to console.

flags: `0x10000002`  
</details>
<details>
<summary><code>editor_toggle</code></summary>

Disables the simulation and returns focus to the editor

flags: `0x4000`  
</details>
<details>
<summary><code>endmovie</code></summary>

Stop recording movie frames.

flags: `0x20002`  
</details>
<details>
<summary><code>entitlements_print</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>entitlements_send</code></summary>

Send client's entitlements to the server

flags: `0x2`  
</details>
<details>
<summary><code>entitlements_set_bits</code></summary>

Set entitlement bitflags on client. Needs to be set before connecting to server to have effect on server

flags: `0x2`  
</details>
<details>
<summary><code>envmap</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>escape</code></summary>

Escape key pressed.

flags: `0x40000000`  
</details>
<details>
<summary><code>exec</code></summary>

Execute script file.

flags: `0x40000000`  
</details>
<details>
<summary><code>execPlayerConfig</code></summary>

Load player settings.

flags: `0x80000`  
</details>
<details>
<summary><code>execifexists</code></summary>

Execute script file if file exists.

flags: `0x2`  
</details>
<details>
<summary><code>exit</code></summary>

Exit the engine.

flags: `0x2`  
</details>
<details>
<summary><code>eyeInfo</code></summary>

gets info about the current view


flags: `0x2`  
</details>
<details>
<summary><code>firstperson</code></summary>

Switch to firstperson camera.

flags: `0x2`  
</details>
<details>
<summary><code>flush</code></summary>

Flush unlocked cache memory.

flags: `0x4000`  
</details>
<details>
<summary><code>flush_locked</code></summary>

Flush unlocked and locked cache memory.

flags: `0x4000`  
</details>
<details>
<summary><code>force_centerview</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>fps_stats_dump</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>fps_stats_reset</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>fps_stats_start</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>fps_stats_stop</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>friends_update</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>fs_clear_open_duplicate_times</code></summary>

Clear the list of files that have been opened.

flags: `0x2`  
</details>
<details>
<summary><code>fs_dump_open_duplicate_times</code></summary>

Set fs_report_long_reads 1 before loading to use this. Prints a list of files that were opened more than once and ~how long was spent reading from them.

flags: `0x2`  
</details>
<details>
<summary><code>fs_fios_cancel_prefetches</code></summary>

Cancels all the prefetches in progress.

flags: `0x2`  
</details>
<details>
<summary><code>fs_fios_flush_cache</code></summary>

Flushes the FIOS HDD cache.

flags: `0x2`  
</details>
<details>
<summary><code>fs_fios_prefetch_file</code></summary>

Prefetches a file: </PS3_GAME/USRDIR/filename.bin>.
The preftech is medium priority and persistent.

flags: `0x2`  
</details>
<details>
<summary><code>fs_fios_prefetch_file_in_pack</code></summary>

Prefetches a file in a pack: <portal2/models/container_ride/fineDebris_part5.ani>.
The preftech is medium priority and non-persistent.

flags: `0x2`  
</details>
<details>
<summary><code>fs_fios_print_prefetches</code></summary>

Displays all the prefetches currently in progress.

flags: `0x2`  
</details>
<details>
<summary><code>fs_printopenfiles</code></summary>

Show all files currently opened by the engine.

flags: `0x2`  
</details>
<details>
<summary><code>fs_warning_level</code></summary>

Set the filesystem warning level.

flags: `0x2`  
</details>
<details>
<summary><code>fx_impact_reparse</code></summary>

Reloads the weapon impact effect table files

flags: `0x4008`  
</details>
<details>
<summary><code>gameui_activate</code></summary>

Shows the game UI

flags: `0x40000000`  
</details>
<details>
<summary><code>gameui_allowescape</code></summary>

Escape key allowed to hide game UI

flags: `0x40000000`  
</details>
<details>
<summary><code>gameui_allowescapetoshow</code></summary>

Escape key allowed to show game UI

flags: `0x2`  
</details>
<details>
<summary><code>gameui_hide</code></summary>

Hides the game UI

flags: `0x40000000`  
</details>
<details>
<summary><code>gameui_preventescape</code></summary>

Escape key doesn't hide game UI

flags: `0x40000000`  
</details>
<details>
<summary><code>gameui_preventescapetoshow</code></summary>

Escape key doesn't show game UI

flags: `0x2`  
</details>
<details>
<summary><code>getNewAuthToken</code></summary>

Ask for a new auth token.

flags: `0x2`  
</details>
<details>
<summary><code>getfov</code></summary>

Gets info about the current FOV

flags: `0x2`  
</details>
<details>
<summary><code>gethttpdatacenterlist</code></summary>

Gets the list of datacenters

flags: `0x2`  
</details>
<details>
<summary><code>getpos</code></summary>

dump position and angles to the console

flags: `0xa`  
</details>
<details>
<summary><code>getpos_bind</code></summary>

Binds the given key to a setpos/setang command of your current position.

flags: `0xa`  
</details>
<details>
<summary><code>getposvec</code></summary>

dump position and angles to the console in 'Vector( x, y, z ), Vector( pitch, yaw, roll )' format

flags: `0xa`  
</details>
<details>
<summary><code>give</code></summary>

Give weapon to player.

flags: `0x4008`  
</details>
<details>
<summary><code>gpu_profile_snapshot</code></summary>

Print a snapshot of a GPU frame (time stamp list).

flags: `0x2`  
</details>
<details>
<summary><code>help</code></summary>

Find help about a convar/concommand.

flags: `0x2`  
</details>
<details>
<summary><code>hidepanel</code></summary>

Hides a viewport panel <name>

flags: `0xa`  
</details>
<details>
<summary><code>hidevideos</code></summary>

Hides video panels playing to the screen

flags: `0xa`  
</details>
<details>
<summary><code>highlight_log</code></summary>

Log Highlight

flags: `0x2`  
</details>
<details>
<summary><code>host_runofftime</code></summary>

Run off some time without rendering/updating sounds


flags: `0x2`  
</details>
<details>
<summary><code>hud_subtitles</code></summary>

Plays the Subtitles: <filename>

flags: `0xa`  
</details>
<details>
<summary><code>huffman_readProps</code></summary>

Read the huffman file and regenerate huffman trees

flags: `0x2`  
</details>
<details>
<summary><code>impulse</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>inboxmessage_report</code></summary>

Report an inbox message as abusive

flags: `0x2`  
</details>
<details>
<summary><code>incrementvar</code></summary>

Increment specified convar value.

flags: `0x20002`  
</details>
<details>
<summary><code>ingamemenu_activate</code></summary>

Shows the in-game menu

flags: `0x40080008`  
</details>
<details>
<summary><code>initMatchmaking</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>invnext</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>is_considered_sony_multiplayer</code></summary>

Checks the value for whether the game is currently telling sony it's in multiplayer

flags: `0xa`  
</details>
<details>
<summary><code>joinopeninvite</code></summary>

Join the active open invite in the chat room

flags: `0x2`  
</details>
<details>
<summary><code>joystick_initialize</code></summary>



flags: `0x40000000`  
</details>
<details>
<summary><code>jpeg</code></summary>

Take a jpeg screenshot:  jpeg <filename> <quality 1-100>.

flags: `0x80000`  
</details>
<details>
<summary><code>key_listboundkeys</code></summary>

(DEPRECATED. Prefer bind_list)List bound keys with bindings.

flags: `0x2`  
</details>
<details>
<summary><code>key_updatelayout</code></summary>

Updates game keyboard layout to current windows keyboard setting.

flags: `0x2`  
</details>
<details>
<summary><code>launchplaylist</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>leaveopeninvite</code></summary>

Leave the active open invite in the chat room

flags: `0x2`  
</details>
<details>
<summary><code>listClientFXScriptHandles</code></summary>

Lists all active effects tracked by script.

flags: `0xa`  
</details>
<details>
<summary><code>listmodels</code></summary>

List loaded models.

flags: `0x2`  
</details>
<details>
<summary><code>loadPlaylists</code></summary>

Reload the playlists


flags: `0x2`  
</details>
<details>
<summary><code>map</code></summary>

Start playing on specified map.

flags: `0x20002`  
</details>
<details>
<summary><code>map_background</code></summary>

Runs a map as the background to the main menu.

flags: `0x20002`  
</details>
<details>
<summary><code>maps</code></summary>

Displays list of maps.

flags: `0x2`  
</details>
<details>
<summary><code>mat_antialias_mode</code></summary>

Set antialias mode

flags: `0x2`  
</details>
<details>
<summary><code>mat_configcurrent</code></summary>

show the current video control panel config for the material system

flags: `0x2`  
</details>
<details>
<summary><code>mat_crosshair</code></summary>

Display the name of the material under the crosshair

flags: `0x4000`  
</details>
<details>
<summary><code>mat_crosshair_edit</code></summary>

open the material under the crosshair in the editor defined by mat_crosshair_edit_editor

flags: `0x4000`  
</details>
<details>
<summary><code>mat_crosshair_explorer</code></summary>

open the material under the crosshair in explorer and highlight the vmt file

flags: `0x4000`  
</details>
<details>
<summary><code>mat_crosshair_printmaterial</code></summary>

print the material under the crosshair

flags: `0x4000`  
</details>
<details>
<summary><code>mat_crosshair_reloadmaterial</code></summary>

reload the material under the crosshair

flags: `0x4000`  
</details>
<details>
<summary><code>mat_gamma</code></summary>

Set gamma ramp

flags: `0x40000000`  
</details>
<details>
<summary><code>mat_hdr_enabled</code></summary>

Report if HDR is enabled for debugging

flags: `0x2`  
</details>
<details>
<summary><code>mat_printLiveTex</code></summary>

Print stats of all known live textures.

flags: `0x2`  
</details>
<details>
<summary><code>mat_savechanges</code></summary>

saves current video configuration

flags: `0x40000000`  
</details>
<details>
<summary><code>mat_setvideomode</code></summary>

sets the width, height, windowed state of the material system, as well as borderless state

flags: `0x40000000`  
</details>
<details>
<summary><code>mat_shadercount</code></summary>

display count of all shaders and reset that count

flags: `0x2`  
</details>
<details>
<summary><code>mat_spewvertexandpixelshaders</code></summary>

Print all vertex and pixel shaders currently loaded to the console

flags: `0x2`  
</details>
<details>
<summary><code>mat_vsync</code></summary>

Set vsync enabled

flags: `0x2`  
</details>
<details>
<summary><code>match_abortAllSearches</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>match_showAllSearches</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>matchmake</code></summary>



flags: `0x40000000`  
</details>
<details>
<summary><code>matchmake_cancel</code></summary>



flags: `0x40000000`  
</details>
<details>
<summary><code>matchmake_cleanupforparty</code></summary>



flags: `0x40000000`  
</details>
<details>
<summary><code>maxplayers</code></summary>

Change the maximum number of players allowed on this server.

flags: `0x2`  
</details>
<details>
<summary><code>mem_compact</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>mem_dump</code></summary>

Dump memory stats to text file.

flags: `0x2`  
</details>
<details>
<summary><code>mem_dump_vm</code></summary>

Dump vm allocations to console.

flags: `0x2`  
</details>
<details>
<summary><code>mem_eat</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>mem_incremental_compact</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>mem_leak_vm</code></summary>

Leak specified amount of virtual memory (in MB or 'oom' to deliberately run out.)

flags: `0x2`  
</details>
<details>
<summary><code>mem_test</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>mem_textures</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>mem_verify</code></summary>

Verify the validity of the heap

flags: `0x2`  
</details>
<details>
<summary><code>mem_vram</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>memory</code></summary>

Print memory stats.

flags: `0x2`  
</details>
<details>
<summary><code>migrateme</code></summary>

Ask your server to migrate you over to another server

flags: `0x10080000`  
</details>
<details>
<summary><code>miles_dump</code></summary>

Writes out milesdump file and perf CSV for current session (when CSOM_MILESDUMP_PASSIVELY_FOR_DEBUGGING is enabled)

flags: `0xa`  
</details>
<details>
<summary><code>miles_event_info</code></summary>

Shows information about a particular event.

flags: `0x2`  
</details>
<details>
<summary><code>miles_pauseui_byname</code></summary>

Pauses any sound played on the listener for this client with the given name.

flags: `0xa`  
</details>
<details>
<summary><code>miles_play</code></summary>

Plays a given alias at an optional given position

flags: `0x2`  
</details>
<details>
<summary><code>miles_reboot</code></summary>

restarts the audio engine

flags: `0x40000008`  
</details>
<details>
<summary><code>miles_record</code></summary>

Enable or disable continuous recording (including previous buffer if available) of audio output to WAV file.

flags: `0xa`  
</details>
<details>
<summary><code>miles_record_that</code></summary>

Writes audio output from the last minute or so to WAV file. (Only useful when CSOM_MILESDUMP_PASSIVE_SAMPLES is enabled)

flags: `0xa`  
</details>
<details>
<summary><code>miles_stop_all</code></summary>

stops all playing sounds

flags: `0x40000008`  
</details>
<details>
<summary><code>miles_unpauseui_byname</code></summary>

Resumes any paused sound played on the listener for this client with the given name.

flags: `0xa`  
</details>
<details>
<summary><code>miles_write_passive_dumpfile</code></summary>

Writes out milesdump file for current session (Only when CSOM_MILESDUMP_PASSIVELY_FOR_DEBUGGING is enabled)

flags: `0xa`  
</details>
<details>
<summary><code>mmdevinit</code></summary>



flags: `0x20002`  
</details>
<details>
<summary><code>multvar</code></summary>

Multiply specified convar value.

flags: `0x20002`  
</details>
<details>
<summary><code>muteroom</code></summary>

Mute the chatroom

flags: `0x2`  
</details>
<details>
<summary><code>net_channels</code></summary>

Shows net channel info

flags: `0x2`  
</details>
<details>
<summary><code>net_dumpIncomingStats</code></summary>

Dump incoming traffic stats

flags: `0x2`  
</details>
<details>
<summary><code>net_dumpOutgoingStats</code></summary>

Dump outgoing traffic stats

flags: `0x2`  
</details>
<details>
<summary><code>net_dumpStats</code></summary>

Dump all traffic stats

flags: `0x2`  
</details>
<details>
<summary><code>net_start</code></summary>

Inits multiplayer network sockets

flags: `0x2`  
</details>
<details>
<summary><code>net_status</code></summary>

Shows current network status

flags: `0x2`  
</details>
<details>
<summary><code>net_writeStatsFile</code></summary>

Write out networking info to a file


flags: `0x2`  
</details>
<details>
<summary><code>openinvite</code></summary>

Send an open invite to the chat room

flags: `0x2`  
</details>
<details>
<summary><code>openinvitecomplete</code></summary>

Open Invite is complete (we have our search results and reservation is done)

flags: `0x2`  
</details>
<details>
<summary><code>openinvitelaunch</code></summary>

Open Invite should launch

flags: `0x2`  
</details>
<details>
<summary><code>origin_friendlist_dump</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>particle_create</code></summary>

Creates the named particle effect at the location under the crosshair.

flags: `0x4000`  
</details>
<details>
<summary><code>particle_create_on_me</code></summary>

Creates a particle effect on my location

flags: `0x4000`  
</details>
<details>
<summary><code>particle_create_ss</code></summary>

Creates a screen space particle effect

flags: `0x4000`  
</details>
<details>
<summary><code>particle_dump</code></summary>

dumps particles matching provided filter (id or defname substring or *)

flags: `0xa`  
</details>
<details>
<summary><code>particle_kill</code></summary>

Destroys the particle effect created with the particle_create console command.

flags: `0x4000`  
</details>
<details>
<summary><code>particle_list</code></summary>

lists particles all, or matching optional filter (id or defname substring)

flags: `0xa`  
</details>
<details>
<summary><code>particle_recreate</code></summary>

Replays the last particle effect created with the particle_create console command.

flags: `0x4000`  
</details>
<details>
<summary><code>particle_remove_all</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>particle_scrub_bake</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>particle_scrub_play</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>particle_scrub_stop</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>party_leave</code></summary>

quit the current party

flags: `0x40000000`  
</details>
<details>
<summary><code>party_serverChange</code></summary>

update the party with new server info

flags: `0x40000000`  
</details>
<details>
<summary><code>path</code></summary>

Show the engine filesystem path.

flags: `0x2`  
</details>
<details>
<summary><code>pause</code></summary>

Toggle the server pause state.

flags: `0x2`  
</details>
<details>
<summary><code>pausevideos</code></summary>

Pauses all videos playing to the screen

flags: `0xa`  
</details>
<details>
<summary><code>phys_objectDump</code></summary>

Dump a list of the active objects on the client.

flags: `0x2000a`  
</details>
<details>
<summary><code>phys_throw_client</code></summary>

Throws an entity of the given model where the player is looking. Model must already be loaded.

flags: `0x4008`  
</details>
<details>
<summary><code>ping</code></summary>

Display ping to server.

flags: `0x6`  
</details>
<details>
<summary><code>ping_specific_type</code></summary>

Pings a specific ping

flags: `0x40000008`  
</details>
<details>
<summary><code>pingdatacenters</code></summary>

Re-pings the datacenters

flags: `0x2`  
</details>
<details>
<summary><code>pixelvis_debug</code></summary>

Dump debug info

flags: `0xa`  
</details>
<details>
<summary><code>playerSettings_reparse</code></summary>

Reload player class settings from .set files

flags: `0x40004002`  
</details>
<details>
<summary><code>playsoundscape</code></summary>

Forces a soundscape to play

flags: `0x4008`  
</details>
<details>
<summary><code>playvideo</code></summary>

Plays a video: <filename> [width height]

flags: `0xa`  
</details>
<details>
<summary><code>playvideo_end_level_transition</code></summary>

Plays a video fullscreen without ability to skip (unless dev 1) and fades in: <filename> <time>

flags: `0xa`  
</details>
<details>
<summary><code>playvideo_exitcommand</code></summary>

Plays a video and fires and exit command when it is stopped or finishes: <filename> <exit command>

flags: `0xa`  
</details>
<details>
<summary><code>playvideo_exitcommand_nointerrupt</code></summary>

Plays a video (without interruption) and fires and exit command when it is stopped or finishes: <filename> <exit command>

flags: `0xa`  
</details>
<details>
<summary><code>playvideo_nointerrupt</code></summary>

Plays a video without ability to skip: <filename> [width height]

flags: `0xa`  
</details>
<details>
<summary><code>playvideo_scaled</code></summary>

Plays a video at position using coordinates scaled relative to the base screen resolution: <filename> [pinPos posX posY width height]

flags: `0xa`  
</details>
<details>
<summary><code>print_colorcorrection</code></summary>

Display the color correction layer information.

flags: `0x4000`  
</details>
<details>
<summary><code>progress_enable</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>quit</code></summary>

Exit the engine.

flags: `0x40000000`  
</details>
<details>
<summary><code>r_cheapwaterend</code></summary>



flags: `0xa`  
</details>
<details>
<summary><code>r_cheapwaterstart</code></summary>



flags: `0xa`  
</details>
<details>
<summary><code>r_cleardecals</code></summary>

Usage r_cleardecals <permanent>.

flags: `0x40000000`  
</details>
<details>
<summary><code>r_dxgi_max_frame_latency</code></summary>

Set the max number of command buffers in flight. 0 will set it to the DXGI default of 3. Make sure you are not forcing "Maximum pre-rendered frames" in the driver settings, but leave it application controlled.

flags: `0x2`  
</details>
<details>
<summary><code>r_printdecalinfo</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>readMsgs</code></summary>

Read your messages

flags: `0x2`  
</details>
<details>
<summary><code>recheck</code></summary>

Ask your server to recheck your community

flags: `0x10000002`  
</details>
<details>
<summary><code>recompute_speed</code></summary>

Recomputes clock speed (for debugging purposes).

flags: `0x4000`  
</details>
<details>
<summary><code>reload</code></summary>

Reload the game (add setpos to jump to current view position on reload).

flags: `0x2`  
</details>
<details>
<summary><code>reload_localization</code></summary>

Reloads all the localization data files

flags: `0x2`  
</details>
<details>
<summary><code>reload_script_callbacks</code></summary>

Reloads script callback function pointers for client and server.

flags: `0x4008`  
</details>
<details>
<summary><code>reset_cam_ideal_angles</code></summary>

Resets camera ideal angles to its default

flags: `0x2`  
</details>
<details>
<summary><code>restart</code></summary>

Restart the game on the same level, to the beginning of the level (add setpos to jump to current view position on restart).

flags: `0x2`  
</details>
<details>
<summary><code>restart_checkpoint</code></summary>

Restart the game on the same level, to the last checkpoint (add setpos to jump to current view position on restart).

flags: `0x2`  
</details>
<details>
<summary><code>rumble_print</code></summary>

Print current list of active rumbles

flags: `0xa`  
</details>
<details>
<summary><code>savePlayerConfig</code></summary>

Store player settings.

flags: `0x40000000`  
</details>
<details>
<summary><code>scoreboard_down</code></summary>

Select next scoreboard player

flags: `0x40000000`  
</details>
<details>
<summary><code>scoreboard_focus</code></summary>

Focus on scoreboard

flags: `0x2`  
</details>
<details>
<summary><code>scoreboard_mute</code></summary>

Toggle the scoreboard player's muted status

flags: `0x40000000`  
</details>
<details>
<summary><code>scoreboard_profile</code></summary>

Show the scoreboard player's profile

flags: `0x40000000`  
</details>
<details>
<summary><code>scoreboard_toggle_focus</code></summary>

Toggle scoreboard focus

flags: `0x40000000`  
</details>
<details>
<summary><code>scoreboard_up</code></summary>

Select previous scoreboard player

flags: `0x40000000`  
</details>
<details>
<summary><code>screenshot</code></summary>

Take a screenshot.

flags: `0x40000000`  
</details>
<details>
<summary><code>server_single_frame</code></summary>

Single step a frame for server

flags: `0x6`  
</details>
<details>
<summary><code>serverinfo</code></summary>

Request serverinfo from a remote ip and port

flags: `0x2`  
</details>
<details>
<summary><code>set</code></summary>

Change a variable in the class settings (does not save out to disk)


flags: `0x40004002`  
</details>
<details>
<summary><code>set_loading_progress_background</code></summary>

Sets the background for load screen. This is cleared to the default after each load.

flags: `0x20002`  
</details>
<details>
<summary><code>set_loading_progress_detente</code></summary>

Set the keyboard and controller strings for the detentes. This is cleared to the default after each load.

flags: `0x20002`  
</details>
<details>
<summary><code>set_loading_progress_fadeout_enabled</code></summary>

Sets whether or not to fade out of loading. This is cleared to the default after each load. (Default is <fadeoutEnabled> = true )

flags: `0x20002`  
</details>
<details>
<summary><code>set_loading_progress_sp_text</code></summary>

Set the sp text for the load sreen. This is cleared to the default after each load.

flags: `0x20002`  
</details>
<details>
<summary><code>setinfo</code></summary>

Adds a new user info value

flags: `0x40000000`  
</details>
<details>
<summary><code>settype</code></summary>

Sets a type for a Convar/ConCommand. This affects UI rendering for the convar. Examples: 'text', 'bool', 'int 0 10', 'float 0.0 100.0', 'enum apple orange banana'. Move these to code eventually.

flags: `0x2`  
</details>
<details>
<summary><code>shake_stop</code></summary>

Stops all active screen shakes.


flags: `0x4000`  
</details>
<details>
<summary><code>shake_testpunch</code></summary>

Test a punch-style screen shake.


flags: `0x4000`  
</details>
<details>
<summary><code>show_loading_progress</code></summary>

Prints all debug information regarding the state of the loading progress.

flags: `0x2`  
</details>
<details>
<summary><code>showpanel</code></summary>

Shows a viewport panel <name>

flags: `0xa`  
</details>
<details>
<summary><code>showvideos</code></summary>

Makes video panels playing to the screen visible (if they were hidden)

flags: `0xa`  
</details>
<details>
<summary><code>silentconnect</code></summary>

Silently connect to specified server, without disconnecting from our current server unless it succeeds.

flags: `0xa0000`  
</details>
<details>
<summary><code>skill_writeTrainingData</code></summary>

Write training gauntlet skill data

flags: `0x6`  
</details>
<details>
<summary><code>soundscape_dumpclient</code></summary>

Dumps the client's soundscape data.


flags: `0x4000`  
</details>
<details>
<summary><code>spawn_as_pilot</code></summary>

Spawn as Pilot

flags: `0x2`  
</details>
<details>
<summary><code>spawn_as_titan</code></summary>

Spawn as Titan

flags: `0x2`  
</details>
<details>
<summary><code>ss_map</code></summary>

Start playing on specified map with max allowed splitscreen players.

flags: `0x20002`  
</details>
<details>
<summary><code>ss_reloadletterbox</code></summary>

ss_reloadletterbox

flags: `0xa`  
</details>
<details>
<summary><code>sssss_enable</code></summary>

Enable screen-space subsurface scattering. 0 - off, 1 - enabled in lobby, 2 - always enabled

flags: `0xa`  
</details>
<details>
<summary><code>star_memory</code></summary>

Dump memory stats

flags: `0x2`  
</details>
<details>
<summary><code>startmovie</code></summary>

Start recording movie frames.

flags: `0x20002`  
</details>
<details>
<summary><code>status</code></summary>

Display map and connection status.

flags: `0x2`  
</details>
<details>
<summary><code>steamlink</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>steamunlink</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>stop_transition_videos_fadeout</code></summary>

Fades out all transition videos playing to the screen: <time>

flags: `0x40000008`  
</details>
<details>
<summary><code>stopsoundscape</code></summary>

Stops all soundscape processing and fades current looping sounds

flags: `0x4008`  
</details>
<details>
<summary><code>stopvideos</code></summary>

Stops all videos playing to the screen

flags: `0xa`  
</details>
<details>
<summary><code>stopvideos_fadeout</code></summary>

Fades out all videos playing to the screen: <time>

flags: `0xa`  
</details>
<details>
<summary><code>sv_precacheinfo</code></summary>

Show precache info.

flags: `0x2`  
</details>
<details>
<summary><code>sv_showents</code></summary>

Prints the server entity list

flags: `0x2`  
</details>
<details>
<summary><code>sv_shutdown</code></summary>

Sets the server to shutdown when all games have completed

flags: `0x4004`  
</details>
<details>
<summary><code>sv_writeSendTableStreamFile</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>testCockpitJoltAngles</code></summary>



flags: `0xa`  
</details>
<details>
<summary><code>testCockpitJoltOrigin</code></summary>



flags: `0xa`  
</details>
<details>
<summary><code>test_freezeframe</code></summary>

Test the freeze frame code.

flags: `0x4000`  
</details>
<details>
<summary><code>testhudanim</code></summary>

Test a hud element animation.
	Arguments: <anim name>


flags: `0x4008`  
</details>
<details>
<summary><code>thread_test_tslist</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>thread_test_tsqueue</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>titan_loadout_select</code></summary>

Titan loadout select

flags: `0x2`  
</details>
<details>
<summary><code>toggle</code></summary>

Toggles a convar on or off, or cycles through a set of values.

flags: `0x80000`  
</details>
<details>
<summary><code>toggle_inventory</code></summary>

Toggle the inventory menu

flags: `0x40080008`  
</details>
<details>
<summary><code>toggle_map</code></summary>

Toggle the big map

flags: `0x40080008`  
</details>
<details>
<summary><code>ui_reloadscheme</code></summary>

Reloads the resource files for the active UI window

flags: `0xa`  
</details>
<details>
<summary><code>uiscript_reset</code></summary>

Resets all UI script state

flags: `0x40000000`  
</details>
<details>
<summary><code>uiscript_resolutionchanged</code></summary>

Notifies UI script that the resolution has changed

flags: `0x40000000`  
</details>
<details>
<summary><code>unbind</code></summary>

Unbind a key's TAPPED binding.

flags: `0x40000000`  
</details>
<details>
<summary><code>unbind_US_standard</code></summary>

Unbind a key's TAPPED binding. Given a key on a standard US keyboard, this function will translate that key to the appropriate key on the user's current keyboard and unbind that key.

flags: `0x40000000`  
</details>
<details>
<summary><code>unbind_all_gamepad</code></summary>

Unbinds all gamepad binds

flags: `0x40000000`  
</details>
<details>
<summary><code>unbind_batch</code></summary>

Unbind all bindings (tapped/held) from all specified keys (no keyboard layout translation).

flags: `0x40000000`  
</details>
<details>
<summary><code>unbind_held</code></summary>

Unbind a key's HELD binding.

flags: `0x40000000`  
</details>
<details>
<summary><code>unbind_held_US_standard</code></summary>

Unbind a key's HELD binding. Given a key on a standard US keyboard, this function will translate that key to the appropriate key on the user's current keyboard and unbind that key.

flags: `0x40000000`  
</details>
<details>
<summary><code>unbindall</code></summary>

Unbind all keys.

flags: `0x40000000`  
</details>
<details>
<summary><code>unbindall_ignoreGamepad</code></summary>

Unbind all keys, skip gamepad binds.

flags: `0x40000000`  
</details>
<details>
<summary><code>unload_level_loadscreen</code></summary>

Unloads the loadscreen for the current level

flags: `0x40000000`  
</details>
<details>
<summary><code>unmuteroom</code></summary>

Unmute the chatroom

flags: `0x2`  
</details>
<details>
<summary><code>unpausevideos</code></summary>

Unpauses all videos playing to the screen

flags: `0xa`  
</details>
<details>
<summary><code>use_consumable</code></summary>

Uses a specific consumable

flags: `0x80008`  
</details>
<details>
<summary><code>user</code></summary>

Show user data.

flags: `0x2`  
</details>
<details>
<summary><code>users</code></summary>

Show user info for players on server.

flags: `0x2`  
</details>
<details>
<summary><code>version</code></summary>

Print version info string.

flags: `0x2`  
</details>
<details>
<summary><code>vgui_drawtree_clear</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>vgui_spew_fonts</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>vgui_togglepanel</code></summary>

show/hide vgui panel by name.

flags: `0x2`  
</details>
<details>
<summary><code>voicerecord_toggle</code></summary>



flags: `0x80000`  
</details>
<details>
<summary><code>vx_datacache_list</code></summary>

vx_datacache_list

flags: `0x2`  
</details>
<details>
<summary><code>vx_model_list</code></summary>

Dump models to VXConsole

flags: `0x2`  
</details>
<details>
<summary><code>weaponSelectOrdnance</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>weaponSelectPrimary0</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>weaponSelectPrimary1</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>weaponSelectPrimary2</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>weapon_activity</code></summary>

Play a custom animation activity on the active weapon.

flags: `0x40080000`  
</details>
<details>
<summary><code>weapon_inspect</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>weapon_list</code></summary>

Lists all weapons owned by the local player

flags: `0x2`  
</details>
<details>
<summary><code>weapon_reparse</code></summary>

Reloads the weapon script files

flags: `0x4008`  
</details>
<details>
<summary><code>xlog_list</code></summary>

List all xlogs, and various stats

flags: `0x2`  
</details>
<details>
<summary><code>xlog_record</code></summary>

Start writing log file to disk (including any previously buffered data)

flags: `0x2`  
</details>
<details>
<summary><code>xlog_record_that</code></summary>

Write buffered data (if any)

flags: `0x2`  
</details>
<details>
<summary><code>xlog_stop</code></summary>

Stop writing it to disk. (resume buffering if buffering enabled on log)

flags: `0x2`  
</details>
<details>
<summary><code>xlook</code></summary>



flags: `0x40080000`  
</details>
<details>
<summary><code>xmove</code></summary>



flags: `0x40080000`  
</details>

### Addresses

```
r5apex.exe!0x01bcd0f0 ConCommand +ability
r5apex.exe!0x01bd45d0 ConCommand +ability_held
r5apex.exe!0x01bd8d20 ConCommand +attack
r5apex.exe!0x01bdcbc0 ConCommand +backward
r5apex.exe!0x01bd5990 ConCommand +break
r5apex.exe!0x01bd8ee0 ConCommand +camdistance
r5apex.exe!0x01bd4650 ConCommand +camin
r5apex.exe!0x01bd1550 ConCommand +cammousemove
r5apex.exe!0x01bd1a30 ConCommand +camout
r5apex.exe!0x01bdcb40 ConCommand +campitchdown
r5apex.exe!0x01bcce10 ConCommand +campitchup
r5apex.exe!0x01bd5a10 ConCommand +camyawleft
r5apex.exe!0x01bd3720 ConCommand +camyawright
r5apex.exe!0x01bd7eb0 ConCommand +commandermousemove
r5apex.exe!0x0181a7a0 ConCommand +csm_rot_x_neg
r5apex.exe!0x01831bd0 ConCommand +csm_rot_x_plus
r5apex.exe!0x0182b940 ConCommand +csm_rot_y_neg
r5apex.exe!0x018313f0 ConCommand +csm_rot_y_plus
r5apex.exe!0x01bdfd80 ConCommand +displayFullscreenMap
r5apex.exe!0x01bd3000 ConCommand +dodge
r5apex.exe!0x01bdd6a0 ConCommand +duck
r5apex.exe!0x01bcd330 ConCommand +forward
r5apex.exe!0x01bda140 ConCommand +graph
r5apex.exe!0x01bde430 ConCommand +jump
r5apex.exe!0x01bd8050 ConCommand +klook
r5apex.exe!0x01bdc450 ConCommand +left
r5apex.exe!0x01bda2c0 ConCommand +lookdown
r5apex.exe!0x01bdaf00 ConCommand +lookup
r5apex.exe!0x011458c0 ConCommand +mat_texture_list
r5apex.exe!0x01bddc70 ConCommand +melee
r5apex.exe!0x01bda0c0 ConCommand +movedown
r5apex.exe!0x01bd4390 ConCommand +moveleft
r5apex.exe!0x01bcd950 ConCommand +moveright
r5apex.exe!0x01bd3240 ConCommand +moveup
r5apex.exe!0x01bde770 ConCommand +offhand0
r5apex.exe!0x01bcfcb0 ConCommand +offhand1
r5apex.exe!0x01bdd1e0 ConCommand +offhand2
r5apex.exe!0x01bd8c20 ConCommand +offhand3
r5apex.exe!0x01bdcc40 ConCommand +offhand4
r5apex.exe!0x01bdfe00 ConCommand +pause_menu
r5apex.exe!0x01bd6d40 ConCommand +ping
r5apex.exe!0x01deefa0 ConCommand +posedebug
r5apex.exe!0x0104d4f0 ConCommand +pushtotalk
r5apex.exe!0x01bd18b0 ConCommand +reload
r5apex.exe!0x01bd0db0 ConCommand +right
r5apex.exe!0x01bdef70 ConCommand +score
r5apex.exe!0x01bdb6e0 ConCommand +scriptCommand1
r5apex.exe!0x01bddcf0 ConCommand +scriptCommand2
r5apex.exe!0x01bd0cb0 ConCommand +scriptCommand3
r5apex.exe!0x01bd3500 ConCommand +scriptCommand4
r5apex.exe!0x01bdac20 ConCommand +scriptCommand5
r5apex.exe!0x01bdaf80 ConCommand +scriptCommand6
r5apex.exe!0x01bde670 ConCommand +scriptCommand7
r5apex.exe!0x01bd1310 ConCommand +scriptCommand8
r5apex.exe!0x01bda9a0 ConCommand +scriptCommand9
r5apex.exe!0x01bcdbf0 ConCommand +showscores
r5apex.exe!0x01bcc9d0 ConCommand +speed
r5apex.exe!0x01bdfd00 ConCommand +strafe
r5apex.exe!0x01bd16f0 ConCommand +toggle_duck
r5apex.exe!0x01bcda50 ConCommand +toggle_zoom
r5apex.exe!0x01bdc980 ConCommand +use
r5apex.exe!0x01bdad40 ConCommand +useAndReload
r5apex.exe!0x01bd3af0 ConCommand +use_alt
r5apex.exe!0x01bdd720 ConCommand +use_long
r5apex.exe!0x01bd1c70 ConCommand +variableScopeToggle
r5apex.exe!0x0104b4a0 ConCommand +vgui_drawtree
r5apex.exe!0x0103ea90 ConCommand +voicerecord
r5apex.exe!0x01bdf670 ConCommand +walk
r5apex.exe!0x01bdecb0 ConCommand +weaponCycle
r5apex.exe!0x01bdb0a0 ConCommand +weapon_discard
r5apex.exe!0x01bd1930 ConCommand +zoom
r5apex.exe!0x01bd7fd0 ConCommand -ability
r5apex.exe!0x01bd2900 ConCommand -ability_held
r5apex.exe!0x01bd4a10 ConCommand -attack
r5apex.exe!0x01bccc50 ConCommand -backward
r5apex.exe!0x01bd6e60 ConCommand -break
r5apex.exe!0x01bdf4d0 ConCommand -camdistance
r5apex.exe!0x01bdbce0 ConCommand -camin
r5apex.exe!0x01bd1390 ConCommand -cammousemove
r5apex.exe!0x01bd65b0 ConCommand -camout
r5apex.exe!0x01bda620 ConCommand -campitchdown
r5apex.exe!0x01bd5b30 ConCommand -campitchup
r5apex.exe!0x01bd44b0 ConCommand -camyawleft
r5apex.exe!0x01bd31c0 ConCommand -camyawright
r5apex.exe!0x01bdd850 ConCommand -commandermousemove
r5apex.exe!0x01833db0 ConCommand -csm_rot_x_neg
r5apex.exe!0x0181aef0 ConCommand -csm_rot_x_plus
r5apex.exe!0x01bc0560 ConCommand -csm_rot_y_neg
r5apex.exe!0x018308f0 ConCommand -csm_rot_y_plus
r5apex.exe!0x01bcd210 ConCommand -displayFullscreenMap
r5apex.exe!0x01bde9b0 ConCommand -dodge
r5apex.exe!0x01bda240 ConCommand -duck
r5apex.exe!0x01bd3600 ConCommand -forward
r5apex.exe!0x01bd3840 ConCommand -graph
r5apex.exe!0x01bd3db0 ConCommand -jump
r5apex.exe!0x01bd2980 ConCommand -klook
r5apex.exe!0x01bcd9d0 ConCommand -left
r5apex.exe!0x01bd3400 ConCommand -lookdown
r5apex.exe!0x01bccfd0 ConCommand -lookup
r5apex.exe!0x01145660 ConCommand -mat_texture_list
r5apex.exe!0x01bd78b0 ConCommand -melee
r5apex.exe!0x01bd15d0 ConCommand -movedown
r5apex.exe!0x01bdc610 ConCommand -moveleft
r5apex.exe!0x01bd3e30 ConCommand -moveright
r5apex.exe!0x01bda360 ConCommand -moveup
r5apex.exe!0x01bdae60 ConCommand -offhand0
r5apex.exe!0x01bdce00 ConCommand -offhand1
r5apex.exe!0x01bdbea0 ConCommand -offhand2
r5apex.exe!0x01bd2a00 ConCommand -offhand3
r5apex.exe!0x01bcd690 ConCommand -offhand4
r5apex.exe!0x01bdc710 ConCommand -pause_menu
r5apex.exe!0x01bcdad0 ConCommand -ping
r5apex.exe!0x01def020 ConCommand -posedebug
r5apex.exe!0x0104ceb0 ConCommand -pushtotalk
r5apex.exe!0x01bcd7b0 ConCommand -reload
r5apex.exe!0x01bda1c0 ConCommand -right
r5apex.exe!0x01bdcfc0 ConCommand -score
r5apex.exe!0x01bcd8d0 ConCommand -scriptCommand1
r5apex.exe!0x01bda7e0 ConCommand -scriptCommand2
r5apex.exe!0x01bde6f0 ConCommand -scriptCommand3
r5apex.exe!0x01bde550 ConCommand -scriptCommand4
r5apex.exe!0x01bcca50 ConCommand -scriptCommand5
r5apex.exe!0x01bcc830 ConCommand -scriptCommand6
r5apex.exe!0x01bde930 ConCommand -scriptCommand7
r5apex.exe!0x01bd0a10 ConCommand -scriptCommand8
r5apex.exe!0x01bd9e80 ConCommand -scriptCommand9
r5apex.exe!0x01bd0c30 ConCommand -showscores
r5apex.exe!0x01bd3d30 ConCommand -speed
r5apex.exe!0x01bdd160 ConCommand -strafe
r5apex.exe!0x01bd3580 ConCommand -toggle_duck
r5apex.exe!0x01bdab60 ConCommand -toggle_zoom
r5apex.exe!0x01bd0a90 ConCommand -use
r5apex.exe!0x01bcc950 ConCommand -useAndReload
r5apex.exe!0x01bdc900 ConCommand -use_alt
r5apex.exe!0x01bd5720 ConCommand -use_long
r5apex.exe!0x01bcd610 ConCommand -variableScopeToggle
r5apex.exe!0x0104af00 ConCommand -vgui_drawtree
r5apex.exe!0x0103aa30 ConCommand -voicerecord
r5apex.exe!0x01bded30 ConCommand -walk
r5apex.exe!0x01bd30a0 ConCommand -weaponCycle
r5apex.exe!0x01bdf790 ConCommand -weapon_discard
r5apex.exe!0x01bd6810 ConCommand -zoom
r5apex.exe!0x01037160 ConCommand BindToggle
r5apex.exe!0x0128d020 ConCommand CMaterialSystem_clear_loading
r5apex.exe!0x0128caa0 ConCommand CMaterialSystem_set_loading
r5apex.exe!0x0128d1e0 ConCommand DebugPrintUsedTextures
r5apex.exe!0x01146390 ConCommand DumpClientDataBlockReceiver
r5apex.exe!0x0110f580 ConCommand MemTrackDeltaSnapshot
r5apex.exe!0x0110f0a0 ConCommand MemTrackPrintStats
r5apex.exe!0x0183b3b0 ConCommand ReloadAimAssistSettings
r5apex.exe!0x0110eb20 ConCommand adminmsg
r5apex.exe!0x01df6ba0 ConCommand aisettings_reparse_client
r5apex.exe!0x0103a7f0 ConCommand alias
r5apex.exe!0x01df55f0 ConCommand applyVideoChangesDeferred
r5apex.exe!0x010424b0 ConCommand bind
r5apex.exe!0x0103f0e0 ConCommand bind_US_standard
r5apex.exe!0x010442e0 ConCommand bind_held
r5apex.exe!0x01043a10 ConCommand bind_held_US_standard
r5apex.exe!0x0103f690 ConCommand bind_list
r5apex.exe!0x0103df60 ConCommand bind_list_abilities
r5apex.exe!0x0104ddb0 ConCommand bink_dump_precached_movies
r5apex.exe!0x01dd31d0 ConCommand bot_loadout
r5apex.exe!0x0110f140 ConCommand box
r5apex.exe!0x01037f10 ConCommand buildcubemaps
r5apex.exe!0x01040f10 ConCommand cache_print
r5apex.exe!0x01040510 ConCommand cache_print_lru
r5apex.exe!0x0103af10 ConCommand cache_print_summary
r5apex.exe!0x01bd8810 ConCommand cam_command
r5apex.exe!0x01bd0b10 ConCommand cancelselect
r5apex.exe!0x01bcffc0 ConCommand cc_emit
r5apex.exe!0x0181b450 ConCommand centerview
r5apex.exe!0x01045270 ConCommand changelevel
r5apex.exe!0x01110530 ConCommand chat
r5apex.exe!0x01d93770 ConCommand chat_wheel
r5apex.exe!0x0117bad0 ConCommand chatroom_adminsOnly
r5apex.exe!0x0110ea00 ConCommand chatroom_away
r5apex.exe!0x0117b930 ConCommand chatroom_freetalk
r5apex.exe!0x0110fc80 ConCommand chatroom_present
r5apex.exe!0x0110f1c0 ConCommand chatserver
r5apex.exe!0x0104df70 ConCommand chroma_base
r5apex.exe!0x0104def0 ConCommand chroma_layer
r5apex.exe!0x01d94fb0 ConCommand cl_dump_particle_stats
r5apex.exe!0x01bc14a0 ConCommand cl_ent_absbox
r5apex.exe!0x01823aa0 ConCommand cl_ent_bbox
r5apex.exe!0x01833ed0 ConCommand cl_ent_rbox
r5apex.exe!0x018340c0 ConCommand cl_find_ent
r5apex.exe!0x01839910 ConCommand cl_find_ent_index
r5apex.exe!0x01834040 ConCommand cl_flip_visibility
r5apex.exe!0x0110f5f0 ConCommand cl_fullupdate
r5apex.exe!0x0183b290 ConCommand cl_interpolation_report
r5apex.exe!0x01be3210 ConCommand cl_panelanimation
r5apex.exe!0x01d8dc60 ConCommand cl_particles_dump_effects
r5apex.exe!0x01d8e410 ConCommand cl_particles_dumplist
r5apex.exe!0x0110e960 ConCommand cl_precacheinfo
r5apex.exe!0x0183af60 ConCommand cl_removedecals
r5apex.exe!0x0110f400 ConCommand cl_showents
r5apex.exe!0x01bddab0 ConCommand cl_soundscape_flush
r5apex.exe!0x01d963d0 ConCommand cl_trace_start_solid
r5apex.exe!0x0183bc10 ConCommand cl_trace_test_hitbox_with_non_zero_start_offset
r5apex.exe!0x01bc1920 ConCommand cl_updatevisibility
r5apex.exe!0x01045980 ConCommand clear_loading_progress_detente
r5apex.exe!0x0103be70 ConCommand clear_loading_progress_sp_text
r5apex.exe!0x01037cb0 ConCommand cm_query_log_record
r5apex.exe!0x0103a080 ConCommand cm_query_log_replay
r5apex.exe!0x01037db0 ConCommand cmd
r5apex.exe!0x01037840 ConCommand cmd1
r5apex.exe!0x01037ea0 ConCommand cmd2
r5apex.exe!0x010373b0 ConCommand cmd3
r5apex.exe!0x01037fb0 ConCommand cmd4
r5apex.exe!0x0182d860 ConCommand collision_debug
r5apex.exe!0x0103a910 ConCommand colorcorrectionui
r5apex.exe!0x01176a10 ConCommand community_browse
r5apex.exe!0x01176bd0 ConCommand community_getPendingJoinRequest
r5apex.exe!0x01176870 ConCommand community_join
r5apex.exe!0x01176e90 ConCommand community_leave
r5apex.exe!0x011768f0 ConCommand community_list
r5apex.exe!0x01176e10 ConCommand community_report
r5apex.exe!0x0110fb70 ConCommand connect
r5apex.exe!0x01110020 ConCommand connectAsSpectator
r5apex.exe!0x0110e8e0 ConCommand connectWithKey
r5apex.exe!0x01110750 ConCommand connectwithtoken
r5apex.exe!0x010386e0 ConCommand convar_differences
r5apex.exe!0x01038670 ConCommand convar_findByFlags
r5apex.exe!0x0103a0f0 ConCommand convar_list
r5apex.exe!0x0110f380 ConCommand createparty
r5apex.exe!0x0110fe10 ConCommand createpartyifnotinone
r5apex.exe!0x01bc18a0 ConCommand csm_status
r5apex.exe!0x01de1370 ConCommand damagedefs_reparse_client
r5apex.exe!0x0110f300 ConCommand debugModelPurge
r5apex.exe!0x01038850 ConCommand devshots_nextmap
r5apex.exe!0x0110f770 ConCommand devshots_screenshot
r5apex.exe!0x0103e6c0 ConCommand dfs_print_flag_states
r5apex.exe!0x0103abb0 ConCommand disconnect
r5apex.exe!0x01045630 ConCommand display_elapsedtime
r5apex.exe!0x018303b0 ConCommand dlight_debug
r5apex.exe!0x011786f0 ConCommand do_InvitePeople_test
r5apex.exe!0x01178210 ConCommand do_Invite_friend_test
r5apex.exe!0x01177e10 ConCommand do_joinPeople_test
r5apex.exe!0x01178110 ConCommand do_origin_test_presence
r5apex.exe!0x0104a680 ConCommand downloadPlaylists
r5apex.exe!0x01d99c40 ConCommand dumpClientStringTable
r5apex.exe!0x010462c0 ConCommand dumpstringtables
r5apex.exe!0x010380c0 ConCommand echo
r5apex.exe!0x010387d0 ConCommand echo_error
r5apex.exe!0x0117b400 ConCommand editor_toggle
r5apex.exe!0x011106d0 ConCommand endmovie
r5apex.exe!0x0103b5f0 ConCommand entitlements_print
r5apex.exe!0x01146830 ConCommand entitlements_send
r5apex.exe!0x01042710 ConCommand entitlements_set_bits
r5apex.exe!0x01037720 ConCommand envmap
r5apex.exe!0x0103bef0 ConCommand escape
r5apex.exe!0x0103a4c0 ConCommand exec
r5apex.exe!0x01045be0 ConCommand execPlayerConfig
r5apex.exe!0x01038760 ConCommand execifexists
r5apex.exe!0x0103c540 ConCommand exit
r5apex.exe!0x01831d90 ConCommand eyeInfo
r5apex.exe!0x01bd38c0 ConCommand firstperson
r5apex.exe!0x0103b310 ConCommand flush
r5apex.exe!0x0103f8e0 ConCommand flush_locked
r5apex.exe!0x01bdd040 ConCommand force_centerview
r5apex.exe!0x01184db0 ConCommand fps_stats_dump
r5apex.exe!0x01186ef0 ConCommand fps_stats_reset
r5apex.exe!0x01184bf0 ConCommand fps_stats_start
r5apex.exe!0x01187290 ConCommand fps_stats_stop
r5apex.exe!0x0117a990 ConCommand friends_update
r5apex.exe!0x0117ced0 ConCommand fs_clear_open_duplicate_times
r5apex.exe!0x0117d130 ConCommand fs_dump_open_duplicate_times
r5apex.exe!0x0117d890 ConCommand fs_fios_cancel_prefetches
r5apex.exe!0x0117ce50 ConCommand fs_fios_flush_cache
r5apex.exe!0x0117d2f0 ConCommand fs_fios_prefetch_file
r5apex.exe!0x0117d640 ConCommand fs_fios_prefetch_file_in_pack
r5apex.exe!0x0117d5c0 ConCommand fs_fios_print_prefetches
r5apex.exe!0x01038b10 ConCommand fs_printopenfiles
r5apex.exe!0x01038f10 ConCommand fs_warning_level
r5apex.exe!0x01bd2740 ConCommand fx_impact_reparse
r5apex.exe!0x010484f0 ConCommand gameui_activate
r5apex.exe!0x0104a7a0 ConCommand gameui_allowescape
r5apex.exe!0x0104c230 ConCommand gameui_allowescapetoshow
r5apex.exe!0x01048270 ConCommand gameui_hide
r5apex.exe!0x01047dd0 ConCommand gameui_preventescape
r5apex.exe!0x01049680 ConCommand gameui_preventescapetoshow
r5apex.exe!0x0117bd10 ConCommand getNewAuthToken
r5apex.exe!0x0182a160 ConCommand getfov
r5apex.exe!0x0103def0 ConCommand gethttpdatacenterlist
r5apex.exe!0x0181b3d0 ConCommand getpos
r5apex.exe!0x01839a80 ConCommand getpos_bind
r5apex.exe!0x01bbfcd0 ConCommand getposvec
r5apex.exe!0x01de2ac0 ConCommand give
r5apex.exe!0x018104f0 ConCommand gpu_profile_snapshot
r5apex.exe!0x01038ea0 ConCommand help
r5apex.exe!0x01e05ce0 ConCommand hidepanel
r5apex.exe!0x01be1b40 ConCommand hidevideos
r5apex.exe!0x0128e410 ConCommand highlight_log
r5apex.exe!0x010433f0 ConCommand host_runofftime
r5apex.exe!0x01bd4f70 ConCommand hud_subtitles
r5apex.exe!0x0117bd90 ConCommand huffman_readProps
r5apex.exe!0x01bd9f00 ConCommand impulse
r5apex.exe!0x01176c50 ConCommand inboxmessage_report
r5apex.exe!0x010460e0 ConCommand incrementvar
r5apex.exe!0x01d8dac0 ConCommand ingamemenu_activate
r5apex.exe!0x0103bd50 ConCommand initMatchmaking
r5apex.exe!0x01bd3480 ConCommand invnext
r5apex.exe!0x0183a820 ConCommand is_considered_sony_multiplayer
r5apex.exe!0x0110f260 ConCommand joinopeninvite
r5apex.exe!0x01bda5a0 ConCommand joystick_initialize
r5apex.exe!0x0110f880 ConCommand jpeg
r5apex.exe!0x01045aa0 ConCommand key_listboundkeys
r5apex.exe!0x01180f80 ConCommand key_updatelayout
r5apex.exe!0x0104a380 ConCommand launchplaylist
r5apex.exe!0x01110370 ConCommand leaveopeninvite
r5apex.exe!0x01d96450 ConCommand listClientFXScriptHandles
r5apex.exe!0x010414b0 ConCommand listmodels
r5apex.exe!0x01048e60 ConCommand loadPlaylists
r5apex.exe!0x0103f300 ConCommand map
r5apex.exe!0x0103cee0 ConCommand map_background
r5apex.exe!0x0103f7a0 ConCommand maps
r5apex.exe!0x0103e5b0 ConCommand mat_antialias_mode
r5apex.exe!0x010474b0 ConCommand mat_configcurrent
r5apex.exe!0x01038cc0 ConCommand mat_crosshair
r5apex.exe!0x01038a00 ConCommand mat_crosshair_edit
r5apex.exe!0x01037610 ConCommand mat_crosshair_explorer
r5apex.exe!0x01038e20 ConCommand mat_crosshair_printmaterial
r5apex.exe!0x0103a160 ConCommand mat_crosshair_reloadmaterial
r5apex.exe!0x010400b0 ConCommand mat_gamma
r5apex.exe!0x0118a9b0 ConCommand mat_hdr_enabled
r5apex.exe!0x0180cc90 ConCommand mat_printLiveTex
r5apex.exe!0x0103f480 ConCommand mat_savechanges
r5apex.exe!0x01043470 ConCommand mat_setvideomode
r5apex.exe!0x0180cee0 ConCommand mat_shadercount
r5apex.exe!0x0180ce60 ConCommand mat_spewvertexandpixelshaders
r5apex.exe!0x010472d0 ConCommand mat_vsync
r5apex.exe!0x01043510 ConCommand match_abortAllSearches
r5apex.exe!0x0103dde0 ConCommand match_showAllSearches
r5apex.exe!0x01046790 ConCommand matchmake
r5apex.exe!0x01046b50 ConCommand matchmake_cancel
r5apex.exe!0x01043cb0 ConCommand matchmake_cleanupforparty
r5apex.exe!0x01147ef0 ConCommand maxplayers
r5apex.exe!0x01045c80 ConCommand mem_compact
r5apex.exe!0x0103f4f0 ConCommand mem_dump
r5apex.exe!0x0103eb00 ConCommand mem_dump_vm
r5apex.exe!0x0103c700 ConCommand mem_eat
r5apex.exe!0x01044d70 ConCommand mem_incremental_compact
r5apex.exe!0x010458b0 ConCommand mem_leak_vm
r5apex.exe!0x0103f9f0 ConCommand mem_test
r5apex.exe!0x0118bb60 ConCommand mem_textures
r5apex.exe!0x01046dd0 ConCommand mem_verify
r5apex.exe!0x0118c040 ConCommand mem_vram
r5apex.exe!0x01041d70 ConCommand memory
r5apex.exe!0x010448d0 ConCommand migrateme
r5apex.exe!0x01df4e20 ConCommand miles_dump
r5apex.exe!0x01df4fa0 ConCommand miles_event_info
r5apex.exe!0x01df5a20 ConCommand miles_pauseui_byname
r5apex.exe!0x01df66c0 ConCommand miles_play
r5apex.exe!0x01df6920 ConCommand miles_reboot
r5apex.exe!0x01df37d0 ConCommand miles_record
r5apex.exe!0x01df4660 ConCommand miles_record_that
r5apex.exe!0x01df4270 ConCommand miles_stop_all
r5apex.exe!0x01df3cf0 ConCommand miles_unpauseui_byname
r5apex.exe!0x01df4900 ConCommand miles_write_passive_dumpfile
r5apex.exe!0x0110ece0 ConCommand mmdevinit
r5apex.exe!0x0103aab0 ConCommand multvar
r5apex.exe!0x0117bc90 ConCommand muteroom
r5apex.exe!0x0103b390 ConCommand net_channels
r5apex.exe!0x0110fe80 ConCommand net_dumpIncomingStats
r5apex.exe!0x0110fa30 ConCommand net_dumpOutgoingStats
r5apex.exe!0x0110fcf0 ConCommand net_dumpStats
r5apex.exe!0x01043de0 ConCommand net_start
r5apex.exe!0x01040dd0 ConCommand net_status
r5apex.exe!0x01041230 ConCommand net_writeStatsFile
r5apex.exe!0x01110140 ConCommand openinvite
r5apex.exe!0x0110f660 ConCommand openinvitecomplete
r5apex.exe!0x0110f510 ConCommand openinvitelaunch
r5apex.exe!0x01178190 ConCommand origin_friendlist_dump
r5apex.exe!0x01829910 ConCommand particle_create
r5apex.exe!0x0182fe50 ConCommand particle_create_on_me
r5apex.exe!0x01bc19a0 ConCommand particle_create_ss
r5apex.exe!0x01be4420 ConCommand particle_dump
r5apex.exe!0x0181bae0 ConCommand particle_kill
r5apex.exe!0x01d932f0 ConCommand particle_list
r5apex.exe!0x01829570 ConCommand particle_recreate
r5apex.exe!0x01be25c0 ConCommand particle_remove_all
r5apex.exe!0x01826d50 ConCommand particle_scrub_bake
r5apex.exe!0x01bcc150 ConCommand particle_scrub_play
r5apex.exe!0x01bbf9d0 ConCommand particle_scrub_stop
r5apex.exe!0x011791f0 ConCommand party_leave
r5apex.exe!0x01178d10 ConCommand party_serverChange
r5apex.exe!0x0103a530 ConCommand path
r5apex.exe!0x0103c380 ConCommand pause
r5apex.exe!0x01d8ffd0 ConCommand pausevideos
r5apex.exe!0x01d901b0 ConCommand phys_objectDump
r5apex.exe!0x01823540 ConCommand phys_throw_client
r5apex.exe!0x0103d4d0 ConCommand ping
r5apex.exe!0x01d8fb30 ConCommand ping_specific_type
r5apex.exe!0x010466f0 ConCommand pingdatacenters
r5apex.exe!0x0181b4d0 ConCommand pixelvis_debug
r5apex.exe!0x01dd2e10 ConCommand playerSettings_reparse
r5apex.exe!0x01bd19b0 ConCommand playsoundscape
r5apex.exe!0x01d91150 ConCommand playvideo
r5apex.exe!0x01d8dee0 ConCommand playvideo_end_level_transition
r5apex.exe!0x01d93bb0 ConCommand playvideo_exitcommand
r5apex.exe!0x01d93ff0 ConCommand playvideo_exitcommand_nointerrupt
r5apex.exe!0x01d949b0 ConCommand playvideo_nointerrupt
r5apex.exe!0x01d90ad0 ConCommand playvideo_scaled
r5apex.exe!0x01037e30 ConCommand print_colorcorrection
r5apex.exe!0x0104afa0 ConCommand progress_enable
r5apex.exe!0x01041370 ConCommand quit
r5apex.exe!0x01be30f0 ConCommand r_cheapwaterend
r5apex.exe!0x01d930b0 ConCommand r_cheapwaterstart
r5apex.exe!0x0103ea20 ConCommand r_cleardecals
r5apex.exe!0x0128fed0 ConCommand r_dxgi_max_frame_latency
r5apex.exe!0x01049b60 ConCommand r_printdecalinfo
r5apex.exe!0x01176430 ConCommand readMsgs
r5apex.exe!0x01046fb0 ConCommand recheck
r5apex.exe!0x0103f410 ConCommand recompute_speed
r5apex.exe!0x010438d0 ConCommand reload
r5apex.exe!0x01146a90 ConCommand reload_localization
r5apex.exe!0x01dd33b0 ConCommand reload_script_callbacks
r5apex.exe!0x01bd5840 ConCommand reset_cam_ideal_angles
r5apex.exe!0x010441a0 ConCommand restart
r5apex.exe!0x010412d0 ConCommand restart_checkpoint
r5apex.exe!0x01df5760 ConCommand rumble_print
r5apex.exe!0x0103af90 ConCommand savePlayerConfig
r5apex.exe!0x01d92d80 ConCommand scoreboard_down
r5apex.exe!0x01d8ed80 ConCommand scoreboard_focus
r5apex.exe!0x01d93c50 ConCommand scoreboard_mute
r5apex.exe!0x01d95890 ConCommand scoreboard_profile
r5apex.exe!0x01d92c20 ConCommand scoreboard_toggle_focus
r5apex.exe!0x01d91a70 ConCommand scoreboard_up
r5apex.exe!0x0110ec60 ConCommand screenshot
r5apex.exe!0x01040e70 ConCommand server_single_frame
r5apex.exe!0x010498e0 ConCommand serverinfo
r5apex.exe!0x01ddb500 ConCommand set
r5apex.exe!0x0103e7d0 ConCommand set_loading_progress_background
r5apex.exe!0x01047410 ConCommand set_loading_progress_detente
r5apex.exe!0x0103b010 ConCommand set_loading_progress_fadeout_enabled
r5apex.exe!0x0103adf0 ConCommand set_loading_progress_sp_text
r5apex.exe!0x0110ffa0 ConCommand setinfo
r5apex.exe!0x01039890 ConCommand settype
r5apex.exe!0x01d93e30 ConCommand shake_stop
r5apex.exe!0x01d942b0 ConCommand shake_testpunch
r5apex.exe!0x01047630 ConCommand show_loading_progress
r5apex.exe!0x01e05c60 ConCommand showpanel
r5apex.exe!0x01d93130 ConCommand showvideos
r5apex.exe!0x0110ef60 ConCommand silentconnect
r5apex.exe!0x0104ac00 ConCommand skill_writeTrainingData
r5apex.exe!0x018396b0 ConCommand soundscape_dumpclient
r5apex.exe!0x01be2700 ConCommand spawn_as_pilot
r5apex.exe!0x01d93370 ConCommand spawn_as_titan
r5apex.exe!0x01040a10 ConCommand ss_map
r5apex.exe!0x01be0e60 ConCommand ss_reloadletterbox
r5apex.exe!0x01d8dbe0 ConCommand sssss_enable
r5apex.exe!0x01049840 ConCommand star_memory
r5apex.exe!0x011105b0 ConCommand startmovie
r5apex.exe!0x01046400 ConCommand status
r5apex.exe!0x0117a7f0 ConCommand steamlink
r5apex.exe!0x0117a910 ConCommand steamunlink
r5apex.exe!0x01d8f120 ConCommand stop_transition_videos_fadeout
r5apex.exe!0x01bdd440 ConCommand stopsoundscape
r5apex.exe!0x01be4050 ConCommand stopvideos
r5apex.exe!0x01be0630 ConCommand stopvideos_fadeout
r5apex.exe!0x01147b50 ConCommand sv_precacheinfo
r5apex.exe!0x01149290 ConCommand sv_showents
r5apex.exe!0x01148c90 ConCommand sv_shutdown
r5apex.exe!0x0114a6d0 ConCommand sv_writeSendTableStreamFile
r5apex.exe!0x0181a820 ConCommand testCockpitJoltAngles
r5apex.exe!0x0183f090 ConCommand testCockpitJoltOrigin
r5apex.exe!0x01be0810 ConCommand test_freezeframe
r5apex.exe!0x01bdedb0 ConCommand testhudanim
r5apex.exe!0x01044490 ConCommand thread_test_tslist
r5apex.exe!0x0103ab30 ConCommand thread_test_tsqueue
r5apex.exe!0x01be1980 ConCommand titan_loadout_select
r5apex.exe!0x01039ec0 ConCommand toggle
r5apex.exe!0x01d93890 ConCommand toggle_inventory
r5apex.exe!0x01be4170 ConCommand toggle_map
r5apex.exe!0x01df1c20 ConCommand ui_reloadscheme
r5apex.exe!0x01d8df60 ConCommand uiscript_reset
r5apex.exe!0x01d91390 ConCommand uiscript_resolutionchanged
r5apex.exe!0x0103d450 ConCommand unbind
r5apex.exe!0x0103f1f0 ConCommand unbind_US_standard
r5apex.exe!0x01043030 ConCommand unbind_all_gamepad
r5apex.exe!0x010417d0 ConCommand unbind_batch
r5apex.exe!0x01042bd0 ConCommand unbind_held
r5apex.exe!0x0103c300 ConCommand unbind_held_US_standard
r5apex.exe!0x0103f570 ConCommand unbindall
r5apex.exe!0x0103e540 ConCommand unbindall_ignoreGamepad
r5apex.exe!0x01042ab0 ConCommand unload_level_loadscreen
r5apex.exe!0x0117b9b0 ConCommand unmuteroom
r5apex.exe!0x01d91b90 ConCommand unpausevideos
r5apex.exe!0x01d91310 ConCommand use_consumable
r5apex.exe!0x01149950 ConCommand user
r5apex.exe!0x011490d0 ConCommand users
r5apex.exe!0x01042d10 ConCommand version
r5apex.exe!0x01049e80 ConCommand vgui_drawtree_clear
r5apex.exe!0x018140e0 ConCommand vgui_spew_fonts
r5apex.exe!0x0104b420 ConCommand vgui_togglepanel
r5apex.exe!0x0103cc80 ConCommand voicerecord_toggle
r5apex.exe!0x0103ac30 ConCommand vx_datacache_list
r5apex.exe!0x0103d5f0 ConCommand vx_model_list
r5apex.exe!0x01bd48d0 ConCommand weaponSelectOrdnance
r5apex.exe!0x01bdc690 ConCommand weaponSelectPrimary0
r5apex.exe!0x01bd1b50 ConCommand weaponSelectPrimary1
r5apex.exe!0x01bd0d30 ConCommand weaponSelectPrimary2
r5apex.exe!0x01bda480 ConCommand weapon_activity
r5apex.exe!0x01bccbd0 ConCommand weapon_inspect
r5apex.exe!0x0182a320 ConCommand weapon_list
r5apex.exe!0x01ddd160 ConCommand weapon_reparse
r5apex.exe!0x0103eb70 ConCommand xlog_list
r5apex.exe!0x010470f0 ConCommand xlog_record
r5apex.exe!0x01042b30 ConCommand xlog_record_that
r5apex.exe!0x0103dfd0 ConCommand xlog_stop
r5apex.exe!0x01bd8ca0 ConCommand xlook
r5apex.exe!0x01bcd4f0 ConCommand xmove
```

## Globals

List of global variables with an associated vtable and their type name.

```
r5apex.exe!0x01e09a18 .?AUSQArray@@
r5apex.exe!0x01e09950 .?AUSQClass@@
r5apex.exe!0x01e09900 .?AUSQClosure@@
r5apex.exe!0x01e097e8 .?AUSQFunctionProto@@
r5apex.exe!0x01e09680 .?AUSQInstance@@
r5apex.exe!0x01e09a40 .?AUSQNativeClosure@@
r5apex.exe!0x01e09748 .?AUSQString@@
r5apex.exe!0x01e099c8 .?AUSQStructDef@@
r5apex.exe!0x01e09798 .?AUSQStructInstance@@
r5apex.exe!0x01e097c0 .?AUSQTable@@
r5apex.exe!0x01e096a8 .?AUSQUserData@@
r5apex.exe!0x01e09a68 .?AUSQVM@@
r5apex.exe!0x01e09a90 .?AUSQWeakRef@@
r5apex.exe!0x22d11a90 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22d11af8 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22d11b00 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22d11b30 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22d11b98 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22d11ba0 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22d11bd0 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22d11c38 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22d11c40 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22d11c70 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22d11cd8 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22d11ce0 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22d11d10 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22d11d78 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22d11d80 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22d11db0 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22d11e18 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22d11e20 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x01036a88 .?AV?$CDataManager@UDataCacheItem_t@@UDataCacheItemData_t@@PEAU1@VCThreadFastMutex@@@@
r5apex.exe!0x0cbcebf0 .?AV?$CDataManager@VCBoneCache@@Ubonecacheparams_t@@PEAV1@VCThreadFastMutex@@@@
r5apex.exe!0x0cbceca0 .?AV?$CDataManager@VCBoneCache@@Ubonecacheparams_t@@PEAV1@VCThreadFastMutex@@@@
r5apex.exe!0x0110fbd8 .?AV?$CPanelFactory@VCMovieDisplayScreen@@UVGuiScreenInitData_t@@@@
r5apex.exe!0x0110f468 .?AV?$CPanelFactory@VCVGuiScreenPanel@@UVGuiScreenInitData_t@@@@
r5apex.exe!0x01e07280 .?AV?$CParticleOperatorDefinition@VC_INIT_AgeNoise@@@@
r5apex.exe!0x01e07160 .?AV?$CParticleOperatorDefinition@VC_INIT_ChaoticAttractor@@@@
r5apex.exe!0x01e072c8 .?AV?$CParticleOperatorDefinition@VC_INIT_ColorLitPerParticle@@@@
r5apex.exe!0x01e070b8 .?AV?$CParticleOperatorDefinition@VC_INIT_CreateAlongPath@@@@
r5apex.exe!0x01e06fc8 .?AV?$CParticleOperatorDefinition@VC_INIT_CreateFromParentParticles@@@@
r5apex.exe!0x01e072e0 .?AV?$CParticleOperatorDefinition@VC_INIT_CreateFromPlaneCache@@@@
r5apex.exe!0x01e07358 .?AV?$CParticleOperatorDefinition@VC_INIT_CreateInEpitrochoid@@@@
r5apex.exe!0x01e071f0 .?AV?$CParticleOperatorDefinition@VC_INIT_CreateInHierarchy@@@@
r5apex.exe!0x01e06f80 .?AV?$CParticleOperatorDefinition@VC_INIT_CreateOnModel@@@@
r5apex.exe!0x01e072f8 .?AV?$CParticleOperatorDefinition@VC_INIT_CreateSequentialPath@@@@
r5apex.exe!0x01e07238 .?AV?$CParticleOperatorDefinition@VC_INIT_CreateWithinBox@@@@
r5apex.exe!0x01e07100 .?AV?$CParticleOperatorDefinition@VC_INIT_CreateWithinControlPointBox@@@@
r5apex.exe!0x01e074d0 .?AV?$CParticleOperatorDefinition@VC_INIT_CreateWithinSphere@@@@
r5apex.exe!0x01e07058 .?AV?$CParticleOperatorDefinition@VC_INIT_CreationNoise@@@@
r5apex.exe!0x01e073b8 .?AV?$CParticleOperatorDefinition@VC_INIT_DistanceToCPInit@@@@
r5apex.exe!0x01e07088 .?AV?$CParticleOperatorDefinition@VC_INIT_InheritFromParentParticles@@@@
r5apex.exe!0x01e07220 .?AV?$CParticleOperatorDefinition@VC_INIT_InheritVelocity@@@@
r5apex.exe!0x01e07178 .?AV?$CParticleOperatorDefinition@VC_INIT_InitFromParentKilled@@@@
r5apex.exe!0x01e07208 .?AV?$CParticleOperatorDefinition@VC_INIT_InitialRepulsionVelocity@@@@
r5apex.exe!0x01e074e8 .?AV?$CParticleOperatorDefinition@VC_INIT_InitialVelocityNoise@@@@
r5apex.exe!0x01e07028 .?AV?$CParticleOperatorDefinition@VC_INIT_LifespanFromVelocity@@@@
r5apex.exe!0x01e07340 .?AV?$CParticleOperatorDefinition@VC_INIT_ModelCull@@@@
r5apex.exe!0x01e071c0 .?AV?$CParticleOperatorDefinition@VC_INIT_MoveBetweenPoints@@@@
r5apex.exe!0x01e071d8 .?AV?$CParticleOperatorDefinition@VC_INIT_NormalAlignToCP@@@@
r5apex.exe!0x01e07488 .?AV?$CParticleOperatorDefinition@VC_INIT_NormalOffset@@@@
r5apex.exe!0x01e07070 .?AV?$CParticleOperatorDefinition@VC_INIT_OffsetVectorToVector@@@@
r5apex.exe!0x01e07250 .?AV?$CParticleOperatorDefinition@VC_INIT_PositionOffset@@@@
r5apex.exe!0x01e06f68 .?AV?$CParticleOperatorDefinition@VC_INIT_PositionPlaceOnGround@@@@
r5apex.exe!0x01e07268 .?AV?$CParticleOperatorDefinition@VC_INIT_PositionWarp@@@@
r5apex.exe!0x01e072b0 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomAlpha@@@@
r5apex.exe!0x01e07010 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomColor@@@@
r5apex.exe!0x01e07118 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomLifeTime@@@@
r5apex.exe!0x01e06f08 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomRadius@@@@
r5apex.exe!0x01e06fe0 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomRotation@@@@
r5apex.exe!0x01e05d48 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomRotationSpeed@@@@
r5apex.exe!0x01e070d0 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomScalar@@@@
r5apex.exe!0x01e07130 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomSecondSequence@@@@
r5apex.exe!0x01e074b8 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomSequence@@@@
r5apex.exe!0x01e06f38 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomTrailLength@@@@
r5apex.exe!0x01e07500 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomVector@@@@
r5apex.exe!0x01e07328 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomVectorComponent@@@@
r5apex.exe!0x01e07470 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomYaw@@@@
r5apex.exe!0x01e06f20 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomYawFlip@@@@
r5apex.exe!0x01e07040 .?AV?$CParticleOperatorDefinition@VC_INIT_RemapCPtoScalar@@@@
r5apex.exe!0x01e06f98 .?AV?$CParticleOperatorDefinition@VC_INIT_RemapCPtoVector@@@@
r5apex.exe!0x01e070a0 .?AV?$CParticleOperatorDefinition@VC_INIT_RemapInitialCPDirectionToRotation@@@@
r5apex.exe!0x01e06fb0 .?AV?$CParticleOperatorDefinition@VC_INIT_RemapInitialDirectionToCPToVector@@@@
r5apex.exe!0x01e07148 .?AV?$CParticleOperatorDefinition@VC_INIT_RemapParticleCountToScalar@@@@
r5apex.exe!0x01e06f50 .?AV?$CParticleOperatorDefinition@VC_INIT_RemapScalar@@@@
r5apex.exe!0x01e07370 .?AV?$CParticleOperatorDefinition@VC_INIT_RemapScalarToVector@@@@
r5apex.exe!0x01e06ef0 .?AV?$CParticleOperatorDefinition@VC_INIT_RemapSpeedToScalar@@@@
r5apex.exe!0x01e073a0 .?AV?$CParticleOperatorDefinition@VC_INIT_RemapWorldCPtoScreen@@@@
r5apex.exe!0x01e074a0 .?AV?$CParticleOperatorDefinition@VC_INIT_RingWave@@@@
r5apex.exe!0x01e07310 .?AV?$CParticleOperatorDefinition@VC_INIT_SequenceFromCP@@@@
r5apex.exe!0x01e06ff8 .?AV?$CParticleOperatorDefinition@VC_INIT_SequenceLifeTime@@@@
r5apex.exe!0x01e07298 .?AV?$CParticleOperatorDefinition@VC_INIT_SetCPPosition@@@@
r5apex.exe!0x01e070e8 .?AV?$CParticleOperatorDefinition@VC_INIT_SetHitboxToClosest@@@@
r5apex.exe!0x01e071a8 .?AV?$CParticleOperatorDefinition@VC_INIT_SetHitboxToModel@@@@
r5apex.exe!0x01e07190 .?AV?$CParticleOperatorDefinition@VC_INIT_VelocityFromCP@@@@
r5apex.exe!0x01e07388 .?AV?$CParticleOperatorDefinition@VC_INIT_VelocityRandom@@@@
r5apex.exe!0x01e07cb0 .?AV?$CParticleOperatorDefinition@VC_OP_AlphaDecay@@@@
r5apex.exe!0x01e075a8 .?AV?$CParticleOperatorDefinition@VC_OP_AttractToControlPoint@@@@
r5apex.exe!0x01e07dc8 .?AV?$CParticleOperatorDefinition@VC_OP_AxisSpin@@@@
r5apex.exe!0x01e07740 .?AV?$CParticleOperatorDefinition@VC_OP_BasicMovement@@@@
r5apex.exe!0x01dfe278 .?AV?$CParticleOperatorDefinition@VC_OP_BoxConstraint@@@@
r5apex.exe!0x01e07650 .?AV?$CParticleOperatorDefinition@VC_OP_CPOffsetToPercentageBetweenCPs@@@@
r5apex.exe!0x01e07fe0 .?AV?$CParticleOperatorDefinition@VC_OP_ClampScalar@@@@
r5apex.exe!0x01e07a88 .?AV?$CParticleOperatorDefinition@VC_OP_ClampVector@@@@
r5apex.exe!0x01e07980 .?AV?$CParticleOperatorDefinition@VC_OP_ColorInterpolate@@@@
r5apex.exe!0x01e03cd8 .?AV?$CParticleOperatorDefinition@VC_OP_ConstrainDistance@@@@
r5apex.exe!0x01e051d8 .?AV?$CParticleOperatorDefinition@VC_OP_ConstrainDistanceToPath@@@@
r5apex.exe!0x01e07578 .?AV?$CParticleOperatorDefinition@VC_OP_ContinuousEmitter@@@@
r5apex.exe!0x01e07df8 .?AV?$CParticleOperatorDefinition@VC_OP_ControlpointLight@@@@
r5apex.exe!0x01e07788 .?AV?$CParticleOperatorDefinition@VC_OP_Cull@@@@
r5apex.exe!0x01e07fc8 .?AV?$CParticleOperatorDefinition@VC_OP_DampenToCP@@@@
r5apex.exe!0x01e076e0 .?AV?$CParticleOperatorDefinition@VC_OP_Decay@@@@
r5apex.exe!0x01e07ce0 .?AV?$CParticleOperatorDefinition@VC_OP_DecayMaintainCount@@@@
r5apex.exe!0x01e07e68 .?AV?$CParticleOperatorDefinition@VC_OP_DifferencePreviousParticle@@@@
r5apex.exe!0x01e07b18 .?AV?$CParticleOperatorDefinition@VC_OP_DistanceBetweenCPs@@@@
r5apex.exe!0x01e07a40 .?AV?$CParticleOperatorDefinition@VC_OP_DistanceBetweenCPsToCP@@@@
r5apex.exe!0x01e07f38 .?AV?$CParticleOperatorDefinition@VC_OP_DistanceCull@@@@
r5apex.exe!0x01e07518 .?AV?$CParticleOperatorDefinition@VC_OP_DistanceEmitter@@@@
r5apex.exe!0x01e079e0 .?AV?$CParticleOperatorDefinition@VC_OP_DistanceToCP@@@@
r5apex.exe!0x01e07710 .?AV?$CParticleOperatorDefinition@VC_OP_FadeAndKill@@@@
r5apex.exe!0x01e07e50 .?AV?$CParticleOperatorDefinition@VC_OP_FadeAndKillForTracers@@@@
r5apex.exe!0x01e079c8 .?AV?$CParticleOperatorDefinition@VC_OP_FadeIn@@@@
r5apex.exe!0x01e078a8 .?AV?$CParticleOperatorDefinition@VC_OP_FadeInSimple@@@@
r5apex.exe!0x01e07ae8 .?AV?$CParticleOperatorDefinition@VC_OP_FadeOut@@@@
r5apex.exe!0x01e07bf0 .?AV?$CParticleOperatorDefinition@VC_OP_FadeOutSimple@@@@
r5apex.exe!0x01e075c0 .?AV?$CParticleOperatorDefinition@VC_OP_ForceBasedOnDistanceToPlane@@@@
r5apex.exe!0x01e07c50 .?AV?$CParticleOperatorDefinition@VC_OP_GraphScalar@@@@
r5apex.exe!0x01e07ee0 .?AV?$CParticleOperatorDefinition@VC_OP_GraphVector@@@@
r5apex.exe!0x01e07830 .?AV?$CParticleOperatorDefinition@VC_OP_InheritFromParentParticles@@@@
r5apex.exe!0x01e07590 .?AV?$CParticleOperatorDefinition@VC_OP_InstantaneousDistanceEmitter@@@@
r5apex.exe!0x01e07530 .?AV?$CParticleOperatorDefinition@VC_OP_InstantaneousEmitter@@@@
r5apex.exe!0x01e07f50 .?AV?$CParticleOperatorDefinition@VC_OP_InterpolateRadius@@@@
r5apex.exe!0x01e07758 .?AV?$CParticleOperatorDefinition@VC_OP_LagCompensation@@@@
r5apex.exe!0x01e07698 .?AV?$CParticleOperatorDefinition@VC_OP_LerpEndCapScalar@@@@
r5apex.exe!0x01e07b30 .?AV?$CParticleOperatorDefinition@VC_OP_LerpEndCapVector@@@@
r5apex.exe!0x01e078d8 .?AV?$CParticleOperatorDefinition@VC_OP_LerpScalar@@@@
r5apex.exe!0x01e07b00 .?AV?$CParticleOperatorDefinition@VC_OP_LerpVector@@@@
r5apex.exe!0x01e07950 .?AV?$CParticleOperatorDefinition@VC_OP_LockToBone@@@@
r5apex.exe!0x01e07b60 .?AV?$CParticleOperatorDefinition@VC_OP_LockToSavedSequentialPath@@@@
r5apex.exe!0x01e07548 .?AV?$CParticleOperatorDefinition@VC_OP_MaintainEmitter@@@@
r5apex.exe!0x01e07f80 .?AV?$CParticleOperatorDefinition@VC_OP_MaintainSequentialPath@@@@
r5apex.exe!0x01e07ad0 .?AV?$CParticleOperatorDefinition@VC_OP_MaxVelocity@@@@
r5apex.exe!0x01e07968 .?AV?$CParticleOperatorDefinition@VC_OP_ModelCull@@@@
r5apex.exe!0x01e07e38 .?AV?$CParticleOperatorDefinition@VC_OP_MoveToHitbox@@@@
r5apex.exe!0x01e07770 .?AV?$CParticleOperatorDefinition@VC_OP_MovementMaintainOffset@@@@
r5apex.exe!0x01e07e98 .?AV?$CParticleOperatorDefinition@VC_OP_MovementPlaceOnGround@@@@
r5apex.exe!0x01e07cf8 .?AV?$CParticleOperatorDefinition@VC_OP_MovementRotateParticleAroundAxis@@@@
r5apex.exe!0x01e079b0 .?AV?$CParticleOperatorDefinition@VC_OP_Noise@@@@
r5apex.exe!0x01e07560 .?AV?$CParticleOperatorDefinition@VC_OP_NoiseEmitter@@@@
r5apex.exe!0x01e07cc8 .?AV?$CParticleOperatorDefinition@VC_OP_NormalLock@@@@
r5apex.exe!0x01e07a28 .?AV?$CParticleOperatorDefinition@VC_OP_NormalizeVector@@@@
r5apex.exe!0x01e07e80 .?AV?$CParticleOperatorDefinition@VC_OP_Orient2DRelToCP@@@@
r5apex.exe!0x01e07eb0 .?AV?$CParticleOperatorDefinition@VC_OP_OrientTo2dDirection@@@@
r5apex.exe!0x01e07b90 .?AV?$CParticleOperatorDefinition@VC_OP_OrientTowardPlayer@@@@
r5apex.exe!0x01e07818 .?AV?$CParticleOperatorDefinition@VC_OP_OscillateScalar@@@@
r5apex.exe!0x01e077d0 .?AV?$CParticleOperatorDefinition@VC_OP_OscillateScalarSimple@@@@
r5apex.exe!0x01e07f08 .?AV?$CParticleOperatorDefinition@VC_OP_OscillateVector@@@@
r5apex.exe!0x01e07728 .?AV?$CParticleOperatorDefinition@VC_OP_OscillateVectorSimple@@@@
r5apex.exe!0x01e07608 .?AV?$CParticleOperatorDefinition@VC_OP_ParentVortices@@@@
r5apex.exe!0x01e07c38 .?AV?$CParticleOperatorDefinition@VC_OP_PercentageBetweenCPs@@@@
r5apex.exe!0x01e07860 .?AV?$CParticleOperatorDefinition@VC_OP_PercentageBetweenCPsVector@@@@
r5apex.exe!0x01dfed98 .?AV?$CParticleOperatorDefinition@VC_OP_PlanarConstraint@@@@
r5apex.exe!0x01e07c08 .?AV?$CParticleOperatorDefinition@VC_OP_PlaneCull@@@@
r5apex.exe!0x01e07aa0 .?AV?$CParticleOperatorDefinition@VC_OP_PositionBetweenCPs@@@@
r5apex.exe!0x01e076b0 .?AV?$CParticleOperatorDefinition@VC_OP_PositionLock@@@@
r5apex.exe!0x01e07e10 .?AV?$CParticleOperatorDefinition@VC_OP_ProjectileArc@@@@
r5apex.exe!0x01e077a0 .?AV?$CParticleOperatorDefinition@VC_OP_RadiusDecay@@@@
r5apex.exe!0x01e07c98 .?AV?$CParticleOperatorDefinition@VC_OP_RampScalarLinear@@@@
r5apex.exe!0x01e07f20 .?AV?$CParticleOperatorDefinition@VC_OP_RampScalarLinearSimple@@@@
r5apex.exe!0x01e076f8 .?AV?$CParticleOperatorDefinition@VC_OP_RampScalarSpline@@@@
r5apex.exe!0x01e078f0 .?AV?$CParticleOperatorDefinition@VC_OP_RampScalarSplineSimple@@@@
r5apex.exe!0x01e07638 .?AV?$CParticleOperatorDefinition@VC_OP_RandomForce@@@@
r5apex.exe!0x01e07ec8 .?AV?$CParticleOperatorDefinition@VC_OP_RemapAverageScalarValuetoCP@@@@
r5apex.exe!0x01e07680 .?AV?$CParticleOperatorDefinition@VC_OP_RemapBoundingVolumetoCP@@@@
r5apex.exe!0x01e07938 .?AV?$CParticleOperatorDefinition@VC_OP_RemapCPVelocityToVector@@@@
r5apex.exe!0x01e07f68 .?AV?$CParticleOperatorDefinition@VC_OP_RemapCPtoScalar@@@@
r5apex.exe!0x01e07a58 .?AV?$CParticleOperatorDefinition@VC_OP_RemapCPtoVector@@@@
r5apex.exe!0x01e07ab8 .?AV?$CParticleOperatorDefinition@VC_OP_RemapControlPointDirectionToVector@@@@
r5apex.exe!0x01e07bc0 .?AV?$CParticleOperatorDefinition@VC_OP_RemapDirectionToCPToVector@@@@
r5apex.exe!0x01e07800 .?AV?$CParticleOperatorDefinition@VC_OP_RemapDotProductToScalar@@@@
r5apex.exe!0x01e07c68 .?AV?$CParticleOperatorDefinition@VC_OP_RemapModelVolumetoCP@@@@
r5apex.exe!0x01e07c20 .?AV?$CParticleOperatorDefinition@VC_OP_RemapScalar@@@@
r5apex.exe!0x01e079f8 .?AV?$CParticleOperatorDefinition@VC_OP_RemapSpeed@@@@
r5apex.exe!0x01e07b48 .?AV?$CParticleOperatorDefinition@VC_OP_RemapSpeedtoCP@@@@
r5apex.exe!0x01e076c8 .?AV?$CParticleOperatorDefinition@VC_OP_RemapVelocityToVector@@@@
r5apex.exe!0x01e078c0 .?AV?$CParticleOperatorDefinition@VC_OP_RemapWorldCPToScreen@@@@
r5apex.exe!0x01e07ff8 .?AV?$CParticleOperatorDefinition@VC_OP_RenderDecal@@@@
r5apex.exe!0x01e08260 .?AV?$CParticleOperatorDefinition@VC_OP_RenderLightSource@@@@
r5apex.exe!0x01e080c8 .?AV?$CParticleOperatorDefinition@VC_OP_RenderModels@@@@
r5apex.exe!0x00d68320 .?AV?$CParticleOperatorDefinition@VC_OP_RenderPoints@@@@
r5apex.exe!0x01e08330 .?AV?$CParticleOperatorDefinition@VC_OP_RenderRope@@@@
r5apex.exe!0x01e08248 .?AV?$CParticleOperatorDefinition@VC_OP_RenderScreenVelocityRotate@@@@
r5apex.exe!0x01e080b0 .?AV?$CParticleOperatorDefinition@VC_OP_RenderScripts@@@@
r5apex.exe!0x01e08278 .?AV?$CParticleOperatorDefinition@VC_OP_RenderSprites@@@@
r5apex.exe!0x01e08230 .?AV?$CParticleOperatorDefinition@VC_OP_RenderSpritesTrail@@@@
r5apex.exe!0x01e07bd8 .?AV?$CParticleOperatorDefinition@VC_OP_RestartAfterDuration@@@@
r5apex.exe!0x01e077b8 .?AV?$CParticleOperatorDefinition@VC_OP_RotateVector@@@@
r5apex.exe!0x01e07a70 .?AV?$CParticleOperatorDefinition@VC_OP_SetCPOrientationToDirection@@@@
r5apex.exe!0x01e07878 .?AV?$CParticleOperatorDefinition@VC_OP_SetChildControlPoints@@@@
r5apex.exe!0x01e07998 .?AV?$CParticleOperatorDefinition@VC_OP_SetControlPointPositions@@@@
r5apex.exe!0x01e07f98 .?AV?$CParticleOperatorDefinition@VC_OP_SetControlPointRotation@@@@
r5apex.exe!0x01e07c80 .?AV?$CParticleOperatorDefinition@VC_OP_SetControlPointToCenter@@@@
r5apex.exe!0x01e077e8 .?AV?$CParticleOperatorDefinition@VC_OP_SetControlPointToImpactPoint@@@@
r5apex.exe!0x01e07920 .?AV?$CParticleOperatorDefinition@VC_OP_SetControlPointToPlayer@@@@
r5apex.exe!0x01e07a10 .?AV?$CParticleOperatorDefinition@VC_OP_SetControlPointsToParticle@@@@
r5apex.exe!0x01e07890 .?AV?$CParticleOperatorDefinition@VC_OP_SetPerChildControlPoint@@@@
r5apex.exe!0x01e07de0 .?AV?$CParticleOperatorDefinition@VC_OP_SoundMeterScalar@@@@
r5apex.exe!0x01e07668 .?AV?$CParticleOperatorDefinition@VC_OP_Spin@@@@
r5apex.exe!0x01e07908 .?AV?$CParticleOperatorDefinition@VC_OP_SpinUpdate@@@@
r5apex.exe!0x01e07ba8 .?AV?$CParticleOperatorDefinition@VC_OP_SpinYaw@@@@
r5apex.exe!0x01e07fb0 .?AV?$CParticleOperatorDefinition@VC_OP_StopAfterCPDuration@@@@
r5apex.exe!0x01e075f0 .?AV?$CParticleOperatorDefinition@VC_OP_TimeVaryingForce@@@@
r5apex.exe!0x01e075d8 .?AV?$CParticleOperatorDefinition@VC_OP_TurbulenceForce@@@@
r5apex.exe!0x01e07620 .?AV?$CParticleOperatorDefinition@VC_OP_TwistAroundAxis@@@@
r5apex.exe!0x01e07848 .?AV?$CParticleOperatorDefinition@VC_OP_VectorNoise@@@@
r5apex.exe!0x01e07b78 .?AV?$CParticleOperatorDefinition@VC_OP_VelocityDecay@@@@
r5apex.exe!0x01e07db0 .?AV?$CParticleOperatorDefinition@VC_OP_VelocityMatchingForce@@@@
r5apex.exe!0x01df9698 .?AV?$CParticleOperatorDefinition@VC_OP_WorldCollideConstraint@@@@
r5apex.exe!0x01e05cc8 .?AV?$CParticleOperatorDefinition@VC_OP_WorldTraceConstraint@@@@
r5apex.exe!0x0182a098 .?AV?$C_EntityClassList@VC_PointCamera@@@@
r5apex.exe!0x01bd56f8 .?AV?$C_EntityClassList@VC_TriggerPlayerMovement@@@@
r5apex.exe!0x01e408f0 .?AV?$_Ref_count_obj_alloc@V__ExceptionPtr@@U?$_StaticAllocator@H@@@std@@
r5apex.exe!0x01839a30 .?AVCAimAssistTargets@@
r5apex.exe!0x01839a38 .?AVCAimAssistTargets@@
r5apex.exe!0x0104dca0 .?AVCAvi@@
r5apex.exe!0x00d67588 .?AVCBSPPack@@
r5apex.exe!0x01044348 .?AVCBaseClientRenderTargets@@
r5apex.exe!0x01e0a5a8 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e0a5c8 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e0a798 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e0adc8 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e0af98 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e0afb8 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e0b1a8 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e0b1c8 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e0b1e8 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e0b208 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e0b228 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e0b358 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e0b3e8 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e0b408 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e0b428 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e0b448 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e0fe88 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e10068 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e10168 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e101f8 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e10758 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e108c8 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e10be8 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e11248 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e113b8 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e114b8 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e114d8 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e118e8 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e13138 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x01e131f8 .?AVCBaseResourcePrecacher@@
r5apex.exe!0x1fea0b10 .?AVCBik@@
r5apex.exe!0x0103e838 .?AVCBoolProperty@@
r5apex.exe!0x01bc1d40 .?AVCCascadeLightManager@@
r5apex.exe!0x01d95190 .?AVCCenterPrint@@
r5apex.exe!0x01e0a830 .?AVCClassMap@@
r5apex.exe!0x01be42d0 .?AVCClientCollisionEvent@@
r5apex.exe!0x01043d18 .?AVCClientDLLSharedAppSystems@@
r5apex.exe!0x0183f110 .?AVCClientEntityList@@
r5apex.exe!0x01abf168 .?AVCClientEntityList@@
r5apex.exe!0x23a3e140 .?AVCClientLeafSystem@@
r5apex.exe!0x239f1430 .?AVCClientShadowMgr@@
r5apex.exe!0x01038f08 .?AVCClientSound@@
r5apex.exe!0x01110c10 .?AVCClientState@@
r5apex.exe!0x01110c18 .?AVCClientState@@
r5apex.exe!0x01110c20 .?AVCClientState@@
r5apex.exe!0x01110c28 .?AVCClientState@@
r5apex.exe!0x01bcc5f0 .?AVCClientThinkList@@
r5apex.exe!0x01034cc0 .?AVCCmdLibFileLoggingListener@@
r5apex.exe!0x01034c80 .?AVCCmdLibStandardLoggingListener@@
r5apex.exe!0x0181b7b0 .?AVCColorCorrectionMgr@@
r5apex.exe!0x01181240 .?AVCColorCorrectionSystem@@
r5apex.exe!0x0103e618 .?AVCColorProperty@@
r5apex.exe!0x01810cc0 .?AVCCommandLine@@
r5apex.exe!0x22d0f3e0 .?AVCCountedStringPool@@
r5apex.exe!0x22d0f430 .?AVCCountedStringPool@@
r5apex.exe!0x018182c0 .?AVCCvar@@
r5apex.exe!0x01038340 .?AVCCvarQuery@@
r5apex.exe!0x01036a80 .?AVCDataCache@@
r5apex.exe!0x01be0c70 .?AVCDebugOverlayPanel@@
r5apex.exe!0x0103a528 .?AVCDebugTextureInfoDX11@@
r5apex.exe!0x0103dfc8 .?AVCDefaultAccessor@@
r5apex.exe!0x0103f948 .?AVCDefaultCvarQuery@@
r5apex.exe!0x01837080 .?AVCEffectsList@@
r5apex.exe!0x01813a00 .?AVCEmptyConVar@@
r5apex.exe!0x01813a40 .?AVCEmptyConVar@@
r5apex.exe!0x01df0fa0 .?AVCEmptyGameUIConVar@@
r5apex.exe!0x01df0fe0 .?AVCEmptyGameUIConVar@@
r5apex.exe!0x010489f0 .?AVCEngine@@
r5apex.exe!0x0117b4a0 .?AVCEngineAPI@@
r5apex.exe!0x010371c8 .?AVCEngineClient@@
r5apex.exe!0x0103a0e8 .?AVCEngineConsoleLoggingListener@@
r5apex.exe!0x01036be8 .?AVCEngineTraceClient@@
r5apex.exe!0x01034ea8 .?AVCEngineTraceClientDecals@@
r5apex.exe!0x010388b8 .?AVCEngineUniformRandomStream@@
r5apex.exe!0x01048190 .?AVCEngineVGui@@
r5apex.exe!0x011464b0 .?AVCEntityReadInfo@@
r5apex.exe!0x01818430 .?AVCEventSystem@@
r5apex.exe!0x01bdd798 .?AVCExampleEffect@@
r5apex.exe!0x01be2410 .?AVCFPS@@
r5apex.exe!0x22d0f1f0 .?AVCFileSystem_Stdio@@
r5apex.exe!0x22d0f1f8 .?AVCFileSystem_Stdio@@
r5apex.exe!0x0103eb68 .?AVCFloatProperty@@
r5apex.exe!0x01046468 .?AVCGameClientExports@@
r5apex.exe!0x01df0e00 .?AVCGameUI@@
r5apex.exe!0x01039938 .?AVCGameUIFuncs@@
r5apex.exe!0x0103ebd8 .?AVCHFontProperty@@
r5apex.exe!0x0181a340 .?AVCHLClient@@
r5apex.exe!0x0110f8e8 .?AVCHudTextMessage@@
r5apex.exe!0x0110f578 .?AVCHudTextureHandleProperty@@
r5apex.exe!0x01036d30 .?AVCIVDebugOverlay@@
r5apex.exe!0x01036d38 .?AVCIVDebugOverlay@@
r5apex.exe!0x238ccda0 .?AVCIVPMaterialManager@@
r5apex.exe!0x01bd9000 .?AVCInput@@
r5apex.exe!0x0117f5b0 .?AVCInputStackSystem@@
r5apex.exe!0x0117f680 .?AVCInputSystem@@
r5apex.exe!0x238b3aa0 .?AVCInputWin32@@
r5apex.exe!0x0103eaf8 .?AVCIntProperty@@
r5apex.exe!0x0183be90 .?AVCKeyBindingListenerMgr@@
r5apex.exe!0x01819c20 .?AVCKeyValuesSystem@@
r5apex.exe!0x0103d658 .?AVCLauncherLoggingListener@@
r5apex.exe!0x0103df58 .?AVCListOps@TSListTests@@
r5apex.exe!0x01d8e5f0 .?AVCLoadingDisc@@
r5apex.exe!0x011810a0 .?AVCLocalize@@
r5apex.exe!0x01034d70 .?AVCMDLCache@@
r5apex.exe!0x22d11f18 .?AVCMatQueuedRenderContext@@
r5apex.exe!0x22d11f20 .?AVCMatQueuedRenderContext@@
r5apex.exe!0x22d120e8 .?AVCMatRenderContext@@
r5apex.exe!0x22d120f0 .?AVCMatRenderContext@@
r5apex.exe!0x238bc940 .?AVCMatSystemSurface@@
r5apex.exe!0x238bc948 .?AVCMatSystemSurface@@
r5apex.exe!0x238bc950 .?AVCMatSystemSurface@@
r5apex.exe!0x01e0b470 .?AVCMaterialProxyDict@@
r5apex.exe!0x01037e98 .?AVCMaterialProxyFactory@@
r5apex.exe!0x22d11a80 .?AVCMaterialSystem@@
r5apex.exe!0x22d11a88 .?AVCMaterialSystem@@
r5apex.exe!0x01be22a0 .?AVCMessageChars@@
r5apex.exe!0x00d67ef8 .?AVCMessageListener@vgui@@
r5apex.exe!0x01037f08 .?AVCModelInfoClient@@
r5apex.exe!0x01038018 .?AVCModelInfoServer@@
r5apex.exe!0x01042090 .?AVCModelLoader@@
r5apex.exe!0x01047550 .?AVCModelRender@@
r5apex.exe!0x01d950d0 .?AVCModelRenderSystem@@
r5apex.exe!0x01d950e8 .?AVCModelRenderSystem@@
r5apex.exe!0x01d91fa0 .?AVCMoveHelperClient@@
r5apex.exe!0x01037d78 .?AVCNetworkStringTableContainer@@
r5apex.exe!0x01045930 .?AVCNetworkStringTableContainer@@
r5apex.exe!0x01e0bd10 .?AVCPanelMetaClassMgrImp@@
r5apex.exe!0x01e0c2b0 .?AVCParticleMgr@@
r5apex.exe!0x0103f808 .?AVCPhysicsCollision@@
r5apex.exe!0x01817d30 .?AVCPhysicsInterface@@
r5apex.exe!0x238cccf0 .?AVCPhysicsSurfaceProps@@
r5apex.exe!0x0110fa98 .?AVCPhysicsSystem@@
r5apex.exe!0x01d9b7b0 .?AVCPickupList@@
r5apex.exe!0x01045918 .?AVCPixelVisibilitySystem@@
r5apex.exe!0x018104b0 .?AVCPolyhedron_TempMemory@@
r5apex.exe!0x01dee710 .?AVCPoseDebuggerImpl@@
r5apex.exe!0x01038d28 .?AVCPrecacheSystem@@
r5apex.exe!0x01defe00 .?AVCPrediction@@
r5apex.exe!0x01819d90 .?AVCProcessUtils@@
r5apex.exe!0x0103e038 .?AVCProportionalFloatProperty@@
r5apex.exe!0x0103f258 .?AVCProportionalIntProperty@@
r5apex.exe!0x0103f478 .?AVCProportionalXPosProperty@@
r5apex.exe!0x0103e5a8 .?AVCProportionalYPosProperty@@
r5apex.exe!0x0103de48 .?AVCQueueOps@TSListTests@@
r5apex.exe!0x010453b0 .?AVCQueuedPacketSender@@
r5apex.exe!0x01dee1b0 .?AVCRagdollLRURetirement@@
r5apex.exe!0x010394b8 .?AVCRegistry@@
r5apex.exe!0x01810730 .?AVCResListGenerator@@
r5apex.exe!0x0104b048 .?AVCRopeInitializer@@
r5apex.exe!0x01def0c8 .?AVCRunGameEngine@@
r5apex.exe!0x01038128 .?AVCSaveRestoreFileSystemPassthrough@@
r5apex.exe!0x01814330 .?AVCSchemeManager@@
r5apex.exe!0x0110f5e8 .?AVCScreenSpaceEffectManager@@
r5apex.exe!0x01034c88 .?AVCScriptLib@@
r5apex.exe!0x1fea6480 .?AVCServer@@
r5apex.exe!0x01ddda40 .?AVCServerRandomStream@@
r5apex.exe!0x0103a598 .?AVCShader@Basic@@
r5apex.exe!0x0103a738 .?AVCShader@BasicForceWireframe@Basic@@
r5apex.exe!0x0103a858 .?AVCShader@Bik@@
r5apex.exe!0x0103a978 .?AVCShader@Black@@
r5apex.exe!0x0180daa0 .?AVCShader@BlurFilter@@
r5apex.exe!0x0103aa98 .?AVCShader@BoxFilterCompute@@
r5apex.exe!0x0103ab18 .?AVCShader@DebugDrawEnvmapMask@@
r5apex.exe!0x0103ab98 .?AVCShader@DecalModulate@@
r5apex.exe!0x0103ac18 .?AVCShader@DepthWrite@@
r5apex.exe!0x0103ac98 .?AVCShader@DoFBlurFilterCompute@@
r5apex.exe!0x0103ae58 .?AVCShader@Downsample4x4@@
r5apex.exe!0x0103aff8 .?AVCShader@Downsample@@
r5apex.exe!0x0103af78 .?AVCShader@Downsample_bloom@@
r5apex.exe!0x0103b078 .?AVCShader@Edge@@
r5apex.exe!0x0103b378 .?AVCShader@Engine_Post@@
r5apex.exe!0x0103b3f8 .?AVCShader@ExposureAdaptation@@
r5apex.exe!0x0103b658 .?AVCShader@FrameColorCompute@@
r5apex.exe!0x0103bdb8 .?AVCShader@Logluminance@@
r5apex.exe!0x0103bed8 .?AVCShader@Modulate@@
r5apex.exe!0x0103bf58 .?AVCShader@Occlusion@@
r5apex.exe!0x0103c1a8 .?AVCShader@Refract@@
r5apex.exe!0x0103c3e8 .?AVCShader@Sky@@
r5apex.exe!0x0103c5a8 .?AVCShader@Sprite@@
r5apex.exe!0x0103c768 .?AVCShader@TSAA@@
r5apex.exe!0x0103cce8 .?AVCShader@UnlitTwoTexture@@
r5apex.exe!0x0103cf48 .?AVCShader@VisQuery@@
r5apex.exe!0x0103d4b8 .?AVCShader@Water@@
r5apex.exe!0x0103d538 .?AVCShader@WriteZ@@
r5apex.exe!0x0103c368 .?AVCShader@screenspace_general@@
r5apex.exe!0x00d67ad8 .?AVCShaderLibConVarAccessor@@
r5apex.exe!0x0128c610 .?AVCShaderSystem@@
r5apex.exe!0x0128c618 .?AVCShaderSystem@@
r5apex.exe!0x010346b8 .?AVCSimpleLoggingListener@@
r5apex.exe!0x28c640e8 .?AVCSimpleLoggingListener@@
r5apex.exe!0x010346c8 .?AVCSimpleWindowsLoggingListener@@
r5apex.exe!0x01dd25e8 .?AVCSolidSetDefaults@@
r5apex.exe!0x01110c00 .?AVCSplitScreen@@
r5apex.exe!0x0104bc90 .?AVCStaticPropMgr@@
r5apex.exe!0x0104bc98 .?AVCStaticPropMgr@@
r5apex.exe!0x26b9c880 .?AVCStdMemAlloc@@
r5apex.exe!0x0103ea88 .?AVCStringProperty@@
r5apex.exe!0x01810ab0 .?AVCStudioRenderContext@@
r5apex.exe!0x0103f5d8 .?AVCSurfaceDragDropTarget@@
r5apex.exe!0x01814390 .?AVCSystem@@
r5apex.exe!0x01bcfd30 .?AVCTempEnts@@
r5apex.exe!0x018146e0 .?AVCTextureDictionary@@
r5apex.exe!0x0103f148 .?AVCTextureIdProperty@@
r5apex.exe!0x0c00d398 .?AVCTraceFilterHitAll@@
r5apex.exe!0x0c015b98 .?AVCTraceFilterHitAll@@
r5apex.exe!0x0c01e398 .?AVCTraceFilterHitAll@@
r5apex.exe!0x0c026b98 .?AVCTraceFilterHitAll@@
r5apex.exe!0x0c02f398 .?AVCTraceFilterHitAll@@
r5apex.exe!0x0c037b98 .?AVCTraceFilterHitAll@@
r5apex.exe!0x0c040398 .?AVCTraceFilterHitAll@@
r5apex.exe!0x0c048b98 .?AVCTraceFilterHitAll@@
r5apex.exe!0x01037f78 .?AVCUniformRandomStream@@
r5apex.exe!0x01040118 .?AVCUniformRandomStream@@
r5apex.exe!0x01ddda48 .?AVCUniformRandomStream@@
r5apex.exe!0x018116c0 .?AVCUtlCStringConversion@@
r5apex.exe!0x018128e0 .?AVCUtlNoEscConversion@@
r5apex.exe!0x010387c8 .?AVCVEfx@@
r5apex.exe!0x01038f78 .?AVCVEngineServer@@
r5apex.exe!0x238b3910 .?AVCVGui@@
r5apex.exe!0x010386d8 .?AVCVRenderView@@
r5apex.exe!0x01d908d0 .?AVCVScriptGameSystem@@
r5apex.exe!0x01be3330 .?AVCViewEffects@@
r5apex.exe!0x01d94a50 .?AVCViewEffects@@
r5apex.exe!0x01be44c0 .?AVCViewRender@@
r5apex.exe!0x0110f658 .?AVCViewportClientSystem@@
r5apex.exe!0x01038a68 .?AVCVoiceServer@@
r5apex.exe!0x238b3710 .?AVCWin32Surface@@
r5apex.exe!0x238b3718 .?AVCWin32Surface@@
r5apex.exe!0x238b3720 .?AVCWin32Surface@@
r5apex.exe!0x01042b18 .?AVC_BaseAnimatingGameSystem@@
r5apex.exe!0x01dd5c70 .?AVC_DataObjectAccessSystem@@
r5apex.exe!0x01e08348 .?AVC_DefaultParticleSystemQuery@@
r5apex.exe!0x0114ba08 .?AVC_DirtySpatialPartitionEntityList@@
r5apex.exe!0x01d9ab90 .?AVC_GameMovement@@
r5apex.exe!0x01db47d0 .?AVC_GameRules@@
r5apex.exe!0x01dc9190 .?AVC_GameStringPool@@
r5apex.exe!0x01be2768 .?AVC_GameTimescale@@
r5apex.exe!0x01dc8450 .?AVC_ParticleSystemQuery@@
r5apex.exe!0x01ddd1c8 .?AVC_PrecacheHandler@@
r5apex.exe!0x01dd5d88 .?AVC_PrecacheRegister@@
r5apex.exe!0x01deb570 .?AVC_PropData@@
r5apex.exe!0x01ddc398 .?AVC_PropSurvivalList@@
r5apex.exe!0x23d45b48 .?AVC_SequenceTransitionerLayer@@
r5apex.exe!0x01bd8a70 .?AVC_SoundscapeSystem@@
r5apex.exe!0x01bd3a80 .?AVC_TEBreakModel@@
r5apex.exe!0x01bd3a90 .?AVC_TEBreakModel@@
r5apex.exe!0x01bcfe50 .?AVC_TEEffectDispatch@@
r5apex.exe!0x01bcfe60 .?AVC_TEEffectDispatch@@
r5apex.exe!0x01bcfdb0 .?AVC_TEExplosion@@
r5apex.exe!0x01bcfdc0 .?AVC_TEExplosion@@
r5apex.exe!0x01048558 .?AVC_TEGibEvent@@
r5apex.exe!0x01048568 .?AVC_TEGibEvent@@
r5apex.exe!0x01bccb70 .?AVC_TEPhysicsProp@@
r5apex.exe!0x01bccb80 .?AVC_TEPhysicsProp@@
r5apex.exe!0x01df9640 .?AVC_TEProjectileTrail@@
r5apex.exe!0x01df9650 .?AVC_TEProjectileTrail@@
r5apex.exe!0x01dc7180 .?AVC_TEScriptParticleSystem@@
r5apex.exe!0x01dc7190 .?AVC_TEScriptParticleSystem@@
r5apex.exe!0x01d99d60 .?AVC_TEScriptParticleSystemOnEntity@@
r5apex.exe!0x01d99d70 .?AVC_TEScriptParticleSystemOnEntity@@
r5apex.exe!0x01dcbf60 .?AVC_TEScriptParticleSystemOnEntityWithPos@@
r5apex.exe!0x01dcbf70 .?AVC_TEScriptParticleSystemOnEntityWithPos@@
r5apex.exe!0x01bd76e0 .?AVC_TEShatterSurface@@
r5apex.exe!0x01bd76f0 .?AVC_TEShatterSurface@@
r5apex.exe!0x01bd4730 .?AVC_TESoundDispatch@@
r5apex.exe!0x01bd4740 .?AVC_TESoundDispatch@@
r5apex.exe!0x0104d558 .?AVC_TempEntsSystem@@
r5apex.exe!0x01db4b10 .?AVC_TraceFilterSkipTwoEntities@@
r5apex.exe!0x01db4b40 .?AVC_TraceFilterSkipTwoEntities@@
r5apex.exe!0x01db4b70 .?AVC_TraceFilterSkipTwoEntities@@
r5apex.exe!0x01db4ba0 .?AVC_TraceFilterSkipTwoEntities@@
r5apex.exe!0x01db4bd0 .?AVC_TraceFilterSkipTwoEntities@@
r5apex.exe!0x01db4c00 .?AVC_TraceFilterSkipTwoEntities@@
r5apex.exe!0x01db4c30 .?AVC_TraceFilterSkipTwoEntities@@
r5apex.exe!0x01db4c60 .?AVC_TraceFilterSkipTwoEntities@@
r5apex.exe!0x01dfe230 .?AVC_TurretList@@
r5apex.exe!0x01e05190 .?AVC_WeaponXList@@
r5apex.exe!0x01129380 .?AVClientDataBlockReceiver@@
r5apex.exe!0x018344c0 .?AVClientModeFullscreen@@
r5apex.exe!0x00d39ec0 .?AVDNameStatusNode@@
r5apex.exe!0x00d39ed0 .?AVDNameStatusNode@@
r5apex.exe!0x00d39ee0 .?AVDNameStatusNode@@
r5apex.exe!0x00d39ef0 .?AVDNameStatusNode@@
r5apex.exe!0x0101f400 .?AVDenuvoTrialV2@@
r5apex.exe!0x0180ce50 .?AVHardwareConfigDX11@@
r5apex.exe!0x0104dbd8 .?AVIPredictionSystem_Client@@
r5apex.exe!0x01815be0 .?AVIVP_BetterDebugmanager@@
r5apex.exe!0x0117dd00 .?AVImeTextStore@@
r5apex.exe!0x0117dd08 .?AVImeTextStore@@
r5apex.exe!0x0117dd10 .?AVImeTextStore@@
r5apex.exe!0x0117dd18 .?AVImeTextStore@@
r5apex.exe!0x0117dd20 .?AVImeTextStore@@
r5apex.exe!0x0117dd28 .?AVImeTextStore@@
r5apex.exe!0x0117dd30 .?AVImeTextStore@@
r5apex.exe!0x0117dd38 .?AVImeTextStore@@
r5apex.exe!0x0110ecc8 .?AVMapSettingsReseter@@
r5apex.exe!0x01e09f08 .?AVMonitorDefaultChanges@@
r5apex.exe!0x0114a070 .?AVSVC_UserMessage@@
r5apex.exe!0x0103f6f8 .?AVVPanelWrapper@@
r5apex.exe!0x01e409a0 .?AVbad_alloc@std@@
r5apex.exe!0x1f11fbf8 .?AVstl_critical_section_win7@details@Concurrency@@
r5apex.exe!0x1f120ce8 .?AVstl_critical_section_win7@details@Concurrency@@
```

