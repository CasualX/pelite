## Interfaces

```
r5apex.exe!0x017533b8 ClientRenderTargets001
r5apex.exe!0x01744758 EngineTraceClient004
r5apex.exe!0x01744738 EngineTraceClientDecals004
r5apex.exe!0x01f728e0 EventSystem001
r5apex.exe!0x023833a0 GameUI011
r5apex.exe!0x01747d28 ISoundC002
r5apex.exe!0x01f670b8 RunGameEngine005
r5apex.exe!0x023f37d8 ServerGameClients004
r5apex.exe!0x023feaf0 ServerGameDLL005
r5apex.exe!0x023f56b8 ServerGameEnts002
r5apex.exe!0x019ea448 ShaderSystem002
r5apex.exe!0x01f747f0 VClient018
r5apex.exe!0x02217f08 VClientEntityList003
r5apex.exe!0x02382d10 VClientPrediction001
r5apex.exe!0x01747e00 VCvarQuery001
r5apex.exe!0x017469a0 VDebugOverlay004
r5apex.exe!0x01748288 VENGINE_GAMEUIFUNCS_VERSION005
r5apex.exe!0x018df7f0 VENGINE_LAUNCHER_API_VERSION004
r5apex.exe!0x01756e40 VEngineModel016
r5apex.exe!0x01747968 VEngineRandom001
r5apex.exe!0x01747078 VEngineRenderView013
r5apex.exe!0x01f6e780 VGUI_System010
r5apex.exe!0x22ab3b08 VMaterialSystemConfig004
r5apex.exe!0x017469a8 VPhysicsDebugOverlay001
r5apex.exe!0x023f5f28 VServerDllSharedAppSystems001
```

## Miscellaneous

```
TimeDateStamp = 0x5d099c3a
CheckSum = 0x2a456ec
GameVersion = "v3.0.9.10"
NUM_ENT_ENTRIES = 0x10000
r5apex.exe!0x1f97eb8 cl_entitylist
r5apex.exe!0x17452ec LocalEntityHandle
r5apex.exe!0x18204e0 GlobalVars
r5apex.exe!0xc5f1860 PlayerResource
r5apex.exe!0xc5f0a10 ViewRender
```

## Buttons

These are addresses to global instances of the [`kbutton_t`](https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/game/client/kbutton.h#L14-L20) struct.

```
r5apex.exe!0x0c5f2210 kbutton_t in_attack
r5apex.exe!0x0c5f21a0 kbutton_t in_backward
r5apex.exe!0x2771d8a0 kbutton_t in_break
r5apex.exe!0x27af7c30 kbutton_t in_camin
r5apex.exe!0x2771dbd0 kbutton_t in_camout
r5apex.exe!0x2771db80 kbutton_t in_campitchdown
r5apex.exe!0x27af7cb8 kbutton_t in_campitchup
r5apex.exe!0x2771dc10 kbutton_t in_camyawleft
r5apex.exe!0x2771dbb0 kbutton_t in_camyawright
r5apex.exe!0x0c5f21c8 kbutton_t in_commandermousemove
r5apex.exe!0x0c5f22a8 kbutton_t in_dodge
r5apex.exe!0x2771d8c0 kbutton_t in_duck
r5apex.exe!0x0c5f2178 kbutton_t in_forward
r5apex.exe!0x0c5f2220 kbutton_t in_graph
r5apex.exe!0x0c5f2290 kbutton_t in_jump
r5apex.exe!0x27af7c98 kbutton_t in_klook
r5apex.exe!0x2771dbf8 kbutton_t in_left
r5apex.exe!0x2771dba0 kbutton_t in_lookdown
r5apex.exe!0x2771d8b0 kbutton_t in_lookup
r5apex.exe!0x27af7ca8 kbutton_t in_melee
r5apex.exe!0x2771dc20 kbutton_t in_movedown
r5apex.exe!0x0c5f2168 kbutton_t in_moveleft
r5apex.exe!0x0c5f2190 kbutton_t in_moveright
r5apex.exe!0x27af7c78 kbutton_t in_moveup
r5apex.exe!0x27af7c88 kbutton_t in_offhand0
r5apex.exe!0x2771dbe8 kbutton_t in_offhand1
r5apex.exe!0x2771d9b0 kbutton_t in_offhand2
r5apex.exe!0x2771d990 kbutton_t in_offhand3
r5apex.exe!0x2771d950 kbutton_t in_offhand4
r5apex.exe!0x0c5f2278 kbutton_t in_pause_menu
r5apex.exe!0x0c5f21e0 kbutton_t in_ping
r5apex.exe!0x0c5f2230 kbutton_t in_reload
r5apex.exe!0x2771dbc0 kbutton_t in_right
r5apex.exe!0x2771d9c0 kbutton_t in_score
r5apex.exe!0x2771d9c0 kbutton_t in_showscores
r5apex.exe!0x0c5f21b0 kbutton_t in_speed
r5apex.exe!0x0c5f2248 kbutton_t in_strafe
r5apex.exe!0x27af7c50 kbutton_t in_toggle_duck
r5apex.exe!0x27af7c40 kbutton_t in_toggle_zoom
r5apex.exe!0x2771d9a0 kbutton_t in_use
r5apex.exe!0x2771d940 kbutton_t in_useAndReload
r5apex.exe!0x27af7c60 kbutton_t in_use_alt
r5apex.exe!0x2771d930 kbutton_t in_use_long
r5apex.exe!0x0c5f2258 kbutton_t in_variableScopeToggle
r5apex.exe!0x2771d970 kbutton_t in_walk
r5apex.exe!0x2771db90 kbutton_t in_weaponCycle
r5apex.exe!0x2771d960 kbutton_t in_weapon_discard
r5apex.exe!0x2771d980 kbutton_t in_zoom
```

## ClientClasses

<details>
<summary><code>client_class CAI_BaseNPC</code></summary>

class_id: `0`  
sizeof: `6528`  
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
sizeof: `5632`  
</details>
<details>
<summary><code>client_class CBaseButton</code></summary>

class_id: `0`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CBaseCombatCharacter</code></summary>

class_id: `4`  
sizeof: `6080`  
</details>
<details>
<summary><code>client_class CBaseEntity</code></summary>

class_id: `5`  
sizeof: `2560`  
</details>
<details>
<summary><code>client_class CBaseGrenade</code></summary>

class_id: `6`  
sizeof: `10240`  
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
sizeof: `19392`  
</details>
<details>
<summary><code>client_class CBeam</code></summary>

class_id: `12`  
sizeof: `2752`  
</details>
<details>
<summary><code>client_class CBoneFollower</code></summary>

class_id: `13`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CBreakableProp</code></summary>

class_id: `14`  
sizeof: `4928`  
</details>
<details>
<summary><code>client_class CBreakableSurface</code></summary>

class_id: `15`  
sizeof: `3840`  
</details>
<details>
<summary><code>client_class CCascadeLight</code></summary>

class_id: `16`  
sizeof: `2944`  
</details>
<details>
<summary><code>client_class CColorCorrection</code></summary>

class_id: `17`  
sizeof: `2944`  
</details>
<details>
<summary><code>client_class CCrossbowBolt</code></summary>

class_id: `18`  
sizeof: `10112`  
</details>
<details>
<summary><code>client_class CDeathBoxProp</code></summary>

class_id: `19`  
sizeof: `5056`  
</details>
<details>
<summary><code>client_class CDynamicLight</code></summary>

class_id: `20`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CDynamicProp</code></summary>

class_id: `21`  
sizeof: `4992`  
</details>
<details>
<summary><code>client_class CDynamicPropLightweight</code></summary>

class_id: `22`  
sizeof: `4992`  
</details>
<details>
<summary><code>client_class CEntityBlocker</code></summary>

class_id: `23`  
sizeof: `2560`  
</details>
<details>
<summary><code>client_class CEntityDissolve</code></summary>

class_id: `24`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CEntityLinkPage</code></summary>

class_id: `25`  
sizeof: `4672`  
</details>
<details>
<summary><code>client_class CEnvTonemapController</code></summary>

class_id: `26`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CEnvWind</code></summary>

class_id: `27`  
sizeof: `2944`  
</details>
<details>
<summary><code>client_class CFirstPersonProxy</code></summary>

class_id: `28`  
sizeof: `4992`  
</details>
<details>
<summary><code>client_class CFogController</code></summary>

class_id: `29`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CFuncBrush</code></summary>

class_id: `30`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CFuncBrushLightweight</code></summary>

class_id: `31`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CFuncMoveLinear</code></summary>

class_id: `32`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CGameRulesProxy</code></summary>

class_id: `33`  
sizeof: `2560`  
</details>
<details>
<summary><code>client_class CGlobalNonRewinding</code></summary>

class_id: `34`  
sizeof: `3648`  
</details>
<details>
<summary><code>client_class CGrappleHook</code></summary>

class_id: `35`  
sizeof: `4928`  
</details>
<details>
<summary><code>client_class CHardPointEntity</code></summary>

class_id: `36`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CHardPointFrontierEntity</code></summary>

class_id: `37`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CHealthKit</code></summary>

class_id: `38`  
sizeof: `4864`  
</details>
<details>
<summary><code>client_class CImportantOnEntSound</code></summary>

class_id: `39`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CInfoPlacementHelper</code></summary>

class_id: `40`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CInfoTarget</code></summary>

class_id: `41`  
sizeof: `2560`  
</details>
<details>
<summary><code>client_class CInfoTargetGravity</code></summary>

class_id: `42`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CInfoTargetMinimap</code></summary>

class_id: `43`  
sizeof: `2560`  
</details>
<details>
<summary><code>client_class CMissile</code></summary>

class_id: `44`  
sizeof: `10304`  
</details>
<details>
<summary><code>client_class CMovieDisplay</code></summary>

class_id: `45`  
sizeof: `2944`  
</details>
<details>
<summary><code>client_class CNPC_Drone</code></summary>

class_id: `46`  
sizeof: `6592`  
</details>
<details>
<summary><code>client_class CNPC_Dropship</code></summary>

class_id: `47`  
sizeof: `6656`  
</details>
<details>
<summary><code>client_class CNPC_SentryTurret</code></summary>

class_id: `48`  
sizeof: `6592`  
</details>
<details>
<summary><code>client_class CNPC_Titan</code></summary>

class_id: `49`  
sizeof: `6720`  
</details>
<details>
<summary><code>client_class CParticleSystem</code></summary>

class_id: `50`  
sizeof: `2752`  
</details>
<details>
<summary><code>client_class CPhysBox</code></summary>

class_id: `51`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CPhysicsProp</code></summary>

class_id: `52`  
sizeof: `5056`  
</details>
<details>
<summary><code>client_class CPlayer</code></summary>

class_id: `53`  
sizeof: `17344`  
</details>
<details>
<summary><code>client_class CPlayerDecoy</code></summary>

class_id: `54`  
sizeof: `4992`  
</details>
<details>
<summary><code>client_class CPlayerResource</code></summary>

class_id: `55`  
sizeof: `12416`  
</details>
<details>
<summary><code>client_class CPlayerTasklist</code></summary>

class_id: `56`  
sizeof: `3968`  
</details>
<details>
<summary><code>client_class CPlayerVehicle</code></summary>

class_id: `57`  
sizeof: `4928`  
</details>
<details>
<summary><code>client_class CPlayerWaypoint</code></summary>

class_id: `58`  
sizeof: `3328`  
</details>
<details>
<summary><code>client_class CPointCamera</code></summary>

class_id: `59`  
sizeof: `2752`  
</details>
<details>
<summary><code>client_class CPortal_PointPush</code></summary>

class_id: `60`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CPostProcessController</code></summary>

class_id: `61`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CPredictedFirstPersonProxy</code></summary>

class_id: `62`  
sizeof: `5056`  
</details>
<details>
<summary><code>client_class CProjectile</code></summary>

class_id: `63`  
sizeof: `10048`  
</details>
<details>
<summary><code>client_class CPropDoor</code></summary>

class_id: `64`  
sizeof: `5184`  
</details>
<details>
<summary><code>client_class CPropSurvival</code></summary>

class_id: `65`  
sizeof: `4928`  
</details>
<details>
<summary><code>client_class CRopeKeyframe</code></summary>

class_id: `66`  
sizeof: `3840`  
</details>
<details>
<summary><code>client_class CScriptMover</code></summary>

class_id: `67`  
sizeof: `5440`  
</details>
<details>
<summary><code>client_class CScriptMoverWaypoint</code></summary>

class_id: `68`  
sizeof: `2560`  
</details>
<details>
<summary><code>client_class CScriptNetData</code></summary>

class_id: `69`  
sizeof: `3136`  
</details>
<details>
<summary><code>client_class CScriptNetDataGlobal</code></summary>

class_id: `75`  
sizeof: `3392`  
</details>
<details>
<summary><code>client_class CScriptNetData_SNDC_DEATH_BOX</code></summary>

class_id: `70`  
sizeof: `3200`  
</details>
<details>
<summary><code>client_class CScriptNetData_SNDC_GLOBAL</code></summary>

class_id: `71`  
sizeof: `3392`  
</details>
<details>
<summary><code>client_class CScriptNetData_SNDC_PLAYER_EXCLUSIVE</code></summary>

class_id: `72`  
sizeof: `3328`  
</details>
<details>
<summary><code>client_class CScriptNetData_SNDC_PLAYER_GLOBAL</code></summary>

class_id: `73`  
sizeof: `3328`  
</details>
<details>
<summary><code>client_class CScriptNetData_SNDC_TITAN_SOUL</code></summary>

class_id: `74`  
sizeof: `3264`  
</details>
<details>
<summary><code>client_class CScriptProp</code></summary>

class_id: `76`  
sizeof: `5184`  
</details>
<details>
<summary><code>client_class CScriptTraceVolume</code></summary>

class_id: `77`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CShieldProp</code></summary>

class_id: `78`  
sizeof: `5056`  
</details>
<details>
<summary><code>client_class CSkyCamera</code></summary>

class_id: `79`  
sizeof: `2560`  
</details>
<details>
<summary><code>client_class CSpotlightEnd</code></summary>

class_id: `80`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CSprite</code></summary>

class_id: `81`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CSpriteOriented</code></summary>

class_id: `82`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CStatueProp</code></summary>

class_id: `0`  
sizeof: `5120`  
</details>
<details>
<summary><code>client_class CStatusEffectPlugin</code></summary>

class_id: `83`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CTEBaseBeam</code></summary>

class_id: `85`  
sizeof: `104`  
</details>
<details>
<summary><code>client_class CTEBeamEntPoint</code></summary>

class_id: `86`  
sizeof: `136`  
</details>
<details>
<summary><code>client_class CTEBeamEnts</code></summary>

class_id: `87`  
sizeof: `112`  
</details>
<details>
<summary><code>client_class CTEBeamFollow</code></summary>

class_id: `88`  
sizeof: `112`  
</details>
<details>
<summary><code>client_class CTEBeamLaser</code></summary>

class_id: `89`  
sizeof: `112`  
</details>
<details>
<summary><code>client_class CTEBeamPoints</code></summary>

class_id: `90`  
sizeof: `128`  
</details>
<details>
<summary><code>client_class CTEBeamRing</code></summary>

class_id: `91`  
sizeof: `112`  
</details>
<details>
<summary><code>client_class CTEBeamRingPoint</code></summary>

class_id: `92`  
sizeof: `128`  
</details>
<details>
<summary><code>client_class CTEBeamSpline</code></summary>

class_id: `93`  
sizeof: `240`  
</details>
<details>
<summary><code>client_class CTEBreakModel</code></summary>

class_id: `94`  
sizeof: `112`  
</details>
<details>
<summary><code>client_class CTEEffectDispatch</code></summary>

class_id: `95`  
sizeof: `208`  
</details>
<details>
<summary><code>client_class CTEExplosion</code></summary>

class_id: `96`  
sizeof: `136`  
</details>
<details>
<summary><code>client_class CTEGibEvent</code></summary>

class_id: `97`  
sizeof: `56`  
</details>
<details>
<summary><code>client_class CTEParticleSystem</code></summary>

class_id: `98`  
sizeof: `56`  
</details>
<details>
<summary><code>client_class CTEPhysicsProp</code></summary>

class_id: `99`  
sizeof: `96`  
</details>
<details>
<summary><code>client_class CTEProjectileTrail</code></summary>

class_id: `100`  
sizeof: `88`  
</details>
<details>
<summary><code>client_class CTEScriptParticleSystem</code></summary>

class_id: `101`  
sizeof: `80`  
</details>
<details>
<summary><code>client_class CTEScriptParticleSystemOnEntity</code></summary>

class_id: `102`  
sizeof: `64`  
</details>
<details>
<summary><code>client_class CTEScriptParticleSystemOnEntityWithPos</code></summary>

class_id: `103`  
sizeof: `88`  
</details>
<details>
<summary><code>client_class CTEShatterSurface</code></summary>

class_id: `104`  
sizeof: `120`  
</details>
<details>
<summary><code>client_class CTESoundDispatch</code></summary>

class_id: `105`  
sizeof: `72`  
</details>
<details>
<summary><code>client_class CTeam</code></summary>

class_id: `84`  
sizeof: `2944`  
</details>
<details>
<summary><code>client_class CTitanSoul</code></summary>

class_id: `106`  
sizeof: `3456`  
</details>
<details>
<summary><code>client_class CTriggerCylinderHeavy</code></summary>

class_id: `107`  
sizeof: `2816`  
</details>
<details>
<summary><code>client_class CTriggerNoGrapple</code></summary>

class_id: `108`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CTriggerNoZipline</code></summary>

class_id: `109`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CTriggerPlayerMovement</code></summary>

class_id: `110`  
sizeof: `2752`  
</details>
<details>
<summary><code>client_class CTriggerPointGravity</code></summary>

class_id: `111`  
sizeof: `2752`  
</details>
<details>
<summary><code>client_class CTriggerSlip</code></summary>

class_id: `112`  
sizeof: `2752`  
</details>
<details>
<summary><code>client_class CTurret</code></summary>

class_id: `113`  
sizeof: `6208`  
</details>
<details>
<summary><code>client_class CVGuiScreen</code></summary>

class_id: `114`  
sizeof: `2816`  
</details>
<details>
<summary><code>client_class CVortexSphere</code></summary>

class_id: `115`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CWaterLODControl</code></summary>

class_id: `116`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CWeaponX</code></summary>

class_id: `117`  
sizeof: `24576`  
</details>
<details>
<summary><code>client_class CWorld</code></summary>

class_id: `118`  
sizeof: `2944`  
</details>
<details>
<summary><code>client_class CZipline</code></summary>

class_id: `119`  
sizeof: `4160`  
</details>
<details>
<summary><code>client_class CZiplineEnd</code></summary>

class_id: `120`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class DoorMover</code></summary>

class_id: `121`  
sizeof: `5504`  
</details>
<details>
<summary><code>client_class ScriptMoverLightweight</code></summary>

class_id: `122`  
sizeof: `5504`  
</details>
<details>
<summary><code>client_class Titan_Cockpit</code></summary>

class_id: `0`  
sizeof: `5504`  
</details>

### Addresses

```
r5apex.exe!0x01757c98 ClientClass CAI_BaseNPC
r5apex.exe!0x01753c68 ClientClass CAmbientGeneric
r5apex.exe!0x01756d68 ClientClass CBaseAnimating
r5apex.exe!0x01754e08 ClientClass CBaseAnimatingOverlay
r5apex.exe!0x017577c8 ClientClass CBaseButton
r5apex.exe!0x01750db8 ClientClass CBaseCombatCharacter
r5apex.exe!0x01756e08 ClientClass CBaseEntity
r5apex.exe!0x018da9a8 ClientClass CBaseGrenade
r5apex.exe!0x019eb668 ClientClass CBaseParticleEntity
r5apex.exe!0x01756f88 ClientClass CBaseTempEntity
r5apex.exe!0x017555c8 ClientClass CBaseToggle
r5apex.exe!0x0181edf8 ClientClass CBaseTrigger
r5apex.exe!0x01758858 ClientClass CBaseVPhysicsTrigger
r5apex.exe!0x018dae08 ClientClass CBaseViewModel
r5apex.exe!0x018dab88 ClientClass CBeam
r5apex.exe!0x0174f1b8 ClientClass CBoneFollower
r5apex.exe!0x01750978 ClientClass CBreakableProp
r5apex.exe!0x01752f58 ClientClass CBreakableSurface
r5apex.exe!0x017508d8 ClientClass CCascadeLight
r5apex.exe!0x01756188 ClientClass CColorCorrection
r5apex.exe!0x01f7ded8 ClientClass CCrossbowBolt
r5apex.exe!0x0174f078 ClientClass CDeathBoxProp
r5apex.exe!0x01755668 ClientClass CDynamicLight
r5apex.exe!0x01756c28 ClientClass CDynamicProp
r5apex.exe!0x0174f438 ClientClass CDynamicPropLightweight
r5apex.exe!0x018dc7a8 ClientClass CEntityBlocker
r5apex.exe!0x017542e8 ClientClass CEntityDissolve
r5apex.exe!0x0181e8f8 ClientClass CEntityLinkPage
r5apex.exe!0x01752608 ClientClass CEnvTonemapController
r5apex.exe!0x01750838 ClientClass CEnvWind
r5apex.exe!0x01f75448 ClientClass CFirstPersonProxy
r5apex.exe!0x01755e68 ClientClass CFogController
r5apex.exe!0x01754b88 ClientClass CFuncBrush
r5apex.exe!0x01752a68 ClientClass CFuncBrushLightweight
r5apex.exe!0x01752568 ClientClass CFuncMoveLinear
r5apex.exe!0x018deb48 ClientClass CGameRulesProxy
r5apex.exe!0x018dc208 ClientClass CGlobalNonRewinding
r5apex.exe!0x01f7e458 ClientClass CGrappleHook
r5apex.exe!0x01759d98 ClientClass CHardPointEntity
r5apex.exe!0x0175ae58 ClientClass CHardPointFrontierEntity
r5apex.exe!0x019ecb08 ClientClass CHealthKit
r5apex.exe!0x0181f178 ClientClass CImportantOnEntSound
r5apex.exe!0x01f6ba58 ClientClass CInfoPlacementHelper
r5apex.exe!0x018dcde8 ClientClass CInfoTarget
r5apex.exe!0x018da868 ClientClass CInfoTargetGravity
r5apex.exe!0x018daea8 ClientClass CInfoTargetMinimap
r5apex.exe!0x01f74228 ClientClass CMissile
r5apex.exe!0x01750c98 ClientClass CMovieDisplay
r5apex.exe!0x017524c8 ClientClass CNPC_Drone
r5apex.exe!0x01752108 ClientClass CNPC_Dropship
r5apex.exe!0x0174fb18 ClientClass CNPC_SentryTurret
r5apex.exe!0x01753b28 ClientClass CNPC_Titan
r5apex.exe!0x01753e48 ClientClass CParticleSystem
r5apex.exe!0x01753278 ClientClass CPhysBox
r5apex.exe!0x017552d8 ClientClass CPhysicsProp
r5apex.exe!0x01750478 ClientClass CPlayer
r5apex.exe!0x019eaf98 ClientClass CPlayerDecoy
r5apex.exe!0x01755268 ClientClass CPlayerResource
r5apex.exe!0x018e1528 ClientClass CPlayerTasklist
r5apex.exe!0x0181e418 ClientClass CPlayerVehicle
r5apex.exe!0x018e1038 ClientClass CPlayerWaypoint
r5apex.exe!0x01753808 ClientClass CPointCamera
r5apex.exe!0x01f6e568 ClientClass CPortal_PointPush
r5apex.exe!0x0174ff78 ClientClass CPostProcessController
r5apex.exe!0x01f74cb8 ClientClass CPredictedFirstPersonProxy
r5apex.exe!0x01f728a8 ClientClass CProjectile
r5apex.exe!0x018dac28 ClientClass CPropDoor
r5apex.exe!0x01752e28 ClientClass CPropSurvival
r5apex.exe!0x018df7b8 ClientClass CRopeKeyframe
r5apex.exe!0x019ea458 ClientClass CScriptMover
r5apex.exe!0x018e1838 ClientClass CScriptMoverWaypoint
r5apex.exe!0x018e4ea8 ClientClass CScriptNetData
r5apex.exe!0x018e0068 ClientClass CScriptNetDataGlobal
r5apex.exe!0x018df5d8 ClientClass CScriptNetData_SNDC_DEATH_BOX
r5apex.exe!0x018e1218 ClientClass CScriptNetData_SNDC_GLOBAL
r5apex.exe!0x018e0d18 ClientClass CScriptNetData_SNDC_PLAYER_EXCLUSIVE
r5apex.exe!0x018e0138 ClientClass CScriptNetData_SNDC_PLAYER_GLOBAL
r5apex.exe!0x018e13e8 ClientClass CScriptNetData_SNDC_TITAN_SOUL
r5apex.exe!0x01752748 ClientClass CScriptProp
r5apex.exe!0x01f6f9f8 ClientClass CScriptTraceVolume
r5apex.exe!0x0174f2f8 ClientClass CShieldProp
r5apex.exe!0x01755488 ClientClass CSkyCamera
r5apex.exe!0x01759258 ClientClass CSpotlightEnd
r5apex.exe!0x018dfd78 ClientClass CSprite
r5apex.exe!0x018e4c58 ClientClass CSpriteOriented
r5apex.exe!0x01755d28 ClientClass CStatueProp
r5apex.exe!0x018e16f8 ClientClass CStatusEffectPlugin
r5apex.exe!0x0175a0b8 ClientClass CTEBaseBeam
r5apex.exe!0x0181e728 ClientClass CTEBeamEntPoint
r5apex.exe!0x0175aef8 ClientClass CTEBeamEnts
r5apex.exe!0x018560e8 ClientClass CTEBeamFollow
r5apex.exe!0x01855218 ClientClass CTEBeamLaser
r5apex.exe!0x018566f8 ClientClass CTEBeamPoints
r5apex.exe!0x0181f218 ClientClass CTEBeamRing
r5apex.exe!0x0175cfa8 ClientClass CTEBeamRingPoint
r5apex.exe!0x0181eb78 ClientClass CTEBeamSpline
r5apex.exe!0x0181e558 ClientClass CTEBreakModel
r5apex.exe!0x01759898 ClientClass CTEEffectDispatch
r5apex.exe!0x01759578 ClientClass CTEExplosion
r5apex.exe!0x01754fe8 ClientClass CTEGibEvent
r5apex.exe!0x0181ef18 ClientClass CTEParticleSystem
r5apex.exe!0x0175a9f8 ClientClass CTEPhysicsProp
r5apex.exe!0x01f75a68 ClientClass CTEProjectileTrail
r5apex.exe!0x018dc168 ClientClass CTEScriptParticleSystem
r5apex.exe!0x018dbe48 ClientClass CTEScriptParticleSystemOnEntity
r5apex.exe!0x018dd388 ClientClass CTEScriptParticleSystemOnEntityWithPos
r5apex.exe!0x0181ed58 ClientClass CTEShatterSurface
r5apex.exe!0x01759078 ClientClass CTESoundDispatch
r5apex.exe!0x0181f038 ClientClass CTeam
r5apex.exe!0x01751e88 ClientClass CTitanSoul
r5apex.exe!0x018e01d8 ClientClass CTriggerCylinderHeavy
r5apex.exe!0x017592f8 ClientClass CTriggerNoGrapple
r5apex.exe!0x0181ecb8 ClientClass CTriggerNoZipline
r5apex.exe!0x019ea0e8 ClientClass CTriggerPlayerMovement
r5apex.exe!0x018e15c8 ClientClass CTriggerPointGravity
r5apex.exe!0x018e0db8 ClientClass CTriggerSlip
r5apex.exe!0x01f759c8 ClientClass CTurret
r5apex.exe!0x0181f478 ClientClass CVGuiScreen
r5apex.exe!0x01f847c8 ClientClass CVortexSphere
r5apex.exe!0x0175d868 ClientClass CWaterLODControl
r5apex.exe!0x01f75f78 ClientClass CWeaponX
r5apex.exe!0x0181f338 ClientClass CWorld
r5apex.exe!0x01f74d58 ClientClass CZipline
r5apex.exe!0x01f742c8 ClientClass CZiplineEnd
r5apex.exe!0x018dfcd8 ClientClass DoorMover
r5apex.exe!0x018e9e98 ClientClass ScriptMoverLightweight
r5apex.exe!0x01f6ea18 ClientClass Titan_Cockpit
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
DT_AI_BaseNPC!0x0424 m_localAngles
DT_AI_BaseNPC!0x0500 m_iMaxHealth
DT_AI_BaseNPC!0x0720 m_lifeState
DT_AI_BaseNPC!0x1630 m_inventory
DT_AI_BaseNPC!0x17c0 m_fireteamSlotIndex
DT_AI_BaseNPC!0x192a m_aiSprinting
DT_AI_BaseNPC!0x194c m_aiNetworkFlags
DT_AI_BaseNPC!0x1950 m_isHologram
DT_AI_BaseNPC!0x1951 m_title
DT_AI_BaseNPC!0x1974 m_aiSettingsIndex
DT_AI_BaseNPC!0x1978 m_subclass
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
DT_BaseAnimating!0x074c m_passDamageToParent
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
DT_BaseAnimating!0x0e4c m_nBody
DT_BaseAnimating!0x0e50 m_camoIndex
DT_BaseAnimating!0x0e88 m_nForceBone
DT_BaseAnimating!0x0ef8 m_bSequenceFinished
DT_BaseAnimating!0x0efc m_lockedAnimDeltaYaw
DT_BaseAnimating!0x0f04 m_flModelScale
```
</details>
<details>
<summary><code>class DT_BaseBeam</code></summary>

```
{
	m_nModelIndex: Int,
	m_nHaloIndex: Int,
	m_nStartFrame: Int,
	m_nFrameRate: Int,
	m_fLife: Float,
	m_fWidth: Float,
	m_fEndWidth: Float,
	m_nFadeLength: Int,
	m_fAmplitude: Float,
	r: Int,
	g: Int,
	b: Int,
	a: Int,
	m_nSpeed: Int,
	m_nFlags: Int,
}
```

### Offsets

```
DT_BaseBeam!0x0028 m_nModelIndex
DT_BaseBeam!0x002c m_nHaloIndex
DT_BaseBeam!0x0030 m_nStartFrame
DT_BaseBeam!0x0034 m_nFrameRate
DT_BaseBeam!0x0038 m_fLife
DT_BaseBeam!0x003c m_fWidth
DT_BaseBeam!0x0040 m_fEndWidth
DT_BaseBeam!0x0044 m_nFadeLength
DT_BaseBeam!0x0048 m_fAmplitude
DT_BaseBeam!0x004c r
DT_BaseBeam!0x0050 g
DT_BaseBeam!0x0054 b
DT_BaseBeam!0x0058 a
DT_BaseBeam!0x005c m_nSpeed
DT_BaseBeam!0x0060 m_nFlags
```
</details>
<details>
<summary><code>class DT_BaseCombatCharacter extends DT_BaseAnimatingOverlay</code></summary>

```
{
	bcc_localdata: DT_BCCLocalPlayerExclusive,
	m_weaponGettingSwitchedOut: DataTable,
	m_showActiveWeapon3p: DataTable,
	m_rodeoRiders: DataTable,
	m_numRodeoSlots: Int,
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
	m_latestNonOffhandWeapons: DataTable,
	m_lastCycleSlot: Int,
	m_weaponPermission: Int,
	m_weaponDelayEnableTime: Time,
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
DT_BaseCombatCharacter!0x0014 m_rodeoRiders
DT_BaseCombatCharacter!0x0028 m_numRodeoSlots
DT_BaseCombatCharacter!0x0030 m_vecViewOffset.x
DT_BaseCombatCharacter!0x0034 m_vecViewOffset.y
DT_BaseCombatCharacter!0x0038 m_vecViewOffset.z
DT_BaseCombatCharacter!0x017c m_cloakEndTime
DT_BaseCombatCharacter!0x0180 m_cloakFadeInEndTime
DT_BaseCombatCharacter!0x0184 m_cloakFadeOutStartTime
DT_BaseCombatCharacter!0x0188 m_cloakFadeInDuration
DT_BaseCombatCharacter!0x018c m_cloakFlickerAmount
DT_BaseCombatCharacter!0x0190 m_cloakFlickerEndTime
DT_BaseCombatCharacter!0x0394 m_networkedFlags
DT_BaseCombatCharacter!0x040c m_deathVelocity
DT_BaseCombatCharacter!0x0908 m_minimapData
DT_BaseCombatCharacter!0x0958 m_nameVisibilityFlags
DT_BaseCombatCharacter!0x1604 m_lastFiredTime
DT_BaseCombatCharacter!0x1608 m_lastFiredWeapon
DT_BaseCombatCharacter!0x160c m_raiseFromMeleeEndTime
DT_BaseCombatCharacter!0x1610 m_sharedEnergyCount
DT_BaseCombatCharacter!0x1614 m_sharedEnergyTotal
DT_BaseCombatCharacter!0x1618 m_sharedEnergyLockoutThreshold
DT_BaseCombatCharacter!0x161c m_lastSharedEnergyRegenTime
DT_BaseCombatCharacter!0x1620 m_sharedEnergyRegenRate
DT_BaseCombatCharacter!0x1624 m_sharedEnergyRegenDelay
DT_BaseCombatCharacter!0x1628 m_lastSharedEnergyTakeTime
DT_BaseCombatCharacter!0x1680 m_selectedWeapons
DT_BaseCombatCharacter!0x1684 m_latestPrimaryWeapons
DT_BaseCombatCharacter!0x168c m_latestNonOffhandWeapons
DT_BaseCombatCharacter!0x1694 m_lastCycleSlot
DT_BaseCombatCharacter!0x169c m_weaponPermission
DT_BaseCombatCharacter!0x16a0 m_weaponDelayEnableTime
DT_BaseCombatCharacter!0x16c5 m_weaponDisabledFlags
DT_BaseCombatCharacter!0x16c6 m_hudInfo_visibilityTestAlwaysPasses
DT_BaseCombatCharacter!0x16d8 m_contextAction
DT_BaseCombatCharacter!0x1704 m_phaseShiftTimeStart
DT_BaseCombatCharacter!0x1708 m_phaseShiftTimeEnd
DT_BaseCombatCharacter!0x1754 m_targetInfoIconName
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
	m_originRelativeToPusher: Vector,
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
	m_parentAttachmentIndex: Int,
	m_parentAttachmentModel: Int,
	m_fadeDist: Float,
	m_dissolveEffectEntityHandle: Int,
	m_usablePriority: Int,
	m_usableDistanceOverride: Float,
	m_usableFOV: Float,
	m_usePromptSize: Float,
	m_spottedByTeams: DataTable,
	m_pusher: Int,
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
DT_BaseEntity!0x0024 m_originRelativeToPusher
DT_BaseEntity!0x003c m_fEffects
DT_BaseEntity!0x0040 m_usableType
DT_BaseEntity!0x0044 m_cellX
DT_BaseEntity!0x0048 m_cellY
DT_BaseEntity!0x0048 m_clrRender
DT_BaseEntity!0x004c m_cellZ
DT_BaseEntity!0x004c m_clIntensity
DT_BaseEntity!0x0050 m_localOrigin
DT_BaseEntity!0x005c m_nModelIndex
DT_BaseEntity!0x0104 m_bossPlayer
DT_BaseEntity!0x0150 m_shieldHealth
DT_BaseEntity!0x0154 m_shieldHealthMax
DT_BaseEntity!0x038c m_bIsSoundCodeControllerValueSet
DT_BaseEntity!0x0390 m_flSoundCodeControllerValue
DT_BaseEntity!0x0394 m_networkedFlags
DT_BaseEntity!0x03e8 m_visibilityFlags
DT_BaseEntity!0x03f0 m_iTeamNum
DT_BaseEntity!0x03f4 m_teamMemberIndex
DT_BaseEntity!0x03f8 m_squadID
DT_BaseEntity!0x03fc m_grade
DT_BaseEntity!0x0400 m_passThroughFlags
DT_BaseEntity!0x0404 m_passThroughThickness
DT_BaseEntity!0x0408 m_passThroughDirection
DT_BaseEntity!0x0424 m_localAngles
DT_BaseEntity!0x0438 m_hOwnerEntity
DT_BaseEntity!0x043c m_bRenderWithViewModels
DT_BaseEntity!0x043d m_nRenderFX
DT_BaseEntity!0x0449 m_nRenderMode
DT_BaseEntity!0x0450 m_Collision
DT_BaseEntity!0x04c8 m_CollisionGroup
DT_BaseEntity!0x04cc m_contents
DT_BaseEntity!0x04d0 m_collideWithOwner
DT_BaseEntity!0x0508 m_iSignifierName
DT_BaseEntity!0x0511 m_iName
DT_BaseEntity!0x0618 m_scriptNameIndex
DT_BaseEntity!0x061c m_instanceNameIndex
DT_BaseEntity!0x06a0 m_holdUsePrompt
DT_BaseEntity!0x06a8 m_pressUsePrompt
DT_BaseEntity!0x0740 m_phaseShiftFlags
DT_BaseEntity!0x0744 m_baseTakeDamage
DT_BaseEntity!0x0748 m_invulnerableToDamageCount
DT_BaseEntity!0x07c0 m_attachmentLerpStartTime
DT_BaseEntity!0x07c4 m_attachmentLerpEndTime
DT_BaseEntity!0x07c8 m_attachmentLerpStartOrigin
DT_BaseEntity!0x07d4 m_attachmentLerpStartAngles
DT_BaseEntity!0x07e4 m_parentAttachmentIndex
DT_BaseEntity!0x07ec m_parentAttachmentModel
DT_BaseEntity!0x07f8 m_fadeDist
DT_BaseEntity!0x08b8 m_dissolveEffectEntityHandle
DT_BaseEntity!0x08c8 m_usablePriority
DT_BaseEntity!0x08cc m_usableDistanceOverride
DT_BaseEntity!0x08d0 m_usableFOV
DT_BaseEntity!0x08d4 m_usePromptSize
DT_BaseEntity!0x08e8 m_spottedByTeams
DT_BaseEntity!0x09c4 m_pusher
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
	m_cloakEndTime: Float,
	m_cloakFadeInEndTime: Time,
	m_cloakFadeOutStartTime: Float,
	m_cloakFadeInDuration: Float,
	m_cloakFlickerAmount: Float,
	m_cloakFlickerEndTime: Time,
	m_baseTakeDamage: Int,
	m_invulnerableToDamageCount: Int,
	m_parentAttachmentIndex: Int,
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
DT_BaseGrenade!0x017c m_cloakEndTime
DT_BaseGrenade!0x0180 m_cloakFadeInEndTime
DT_BaseGrenade!0x0184 m_cloakFadeOutStartTime
DT_BaseGrenade!0x0188 m_cloakFadeInDuration
DT_BaseGrenade!0x018c m_cloakFlickerAmount
DT_BaseGrenade!0x0190 m_cloakFlickerEndTime
DT_BaseGrenade!0x0744 m_baseTakeDamage
DT_BaseGrenade!0x0748 m_invulnerableToDamageCount
DT_BaseGrenade!0x07e4 m_parentAttachmentIndex
DT_BaseGrenade!0x07e8 m_parentAttachmentHitbox
DT_BaseGrenade!0x07ec m_parentAttachmentModel
DT_BaseGrenade!0x2741 m_doesExplode
DT_BaseGrenade!0x2744 m_DmgRadius
DT_BaseGrenade!0x27d0 m_ziplineGrenadeExpectedEndPosition
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
	m_animOverlayStartTime: DataTable,
	m_animFrozen: Int,
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
DT_BaseViewModel!0x0010 m_animOverlayStartTime
DT_BaseViewModel!0x0014 m_animFrozen
DT_BaseViewModel!0x0018 m_animModelIndex
DT_BaseViewModel!0x001c m_animSequence
DT_BaseViewModel!0x0020 m_nNewSequenceParity
DT_BaseViewModel!0x0030 m_animOverlayStartCycle
DT_BaseViewModel!0x003c m_fEffects
DT_BaseViewModel!0x0048 m_clrRender
DT_BaseViewModel!0x0050 m_animOverlayPlaybackRate
DT_BaseViewModel!0x005c m_nModelIndex
DT_BaseViewModel!0x0070 m_animOverlayModelIndex
DT_BaseViewModel!0x0090 m_animOverlaySequence
DT_BaseViewModel!0x00b0 m_animOverlayWeight
DT_BaseViewModel!0x00d0 m_animOverlayOrder
DT_BaseViewModel!0x00f0 m_animOverlayAnimTime
DT_BaseViewModel!0x0110 m_animOverlayFadeInDuration
DT_BaseViewModel!0x0130 m_animOverlayFadeOutDuration
DT_BaseViewModel!0x0449 m_nRenderMode
DT_BaseViewModel!0x0e4c m_nBody
DT_BaseViewModel!0x0e58 m_nResetEventsParity
DT_BaseViewModel!0x0ef8 m_bSequenceFinished
DT_BaseViewModel!0x0f04 m_flModelScale
DT_BaseViewModel!0x13f4 m_overlayEventParity
DT_BaseViewModel!0x1658 m_viewModelOwner
DT_BaseViewModel!0x165c m_projectileIsVisible
DT_BaseViewModel!0x1a40 m_bBlockEventLayer
DT_BaseViewModel!0x1a41 m_isAdsTransition
DT_BaseViewModel!0x1a44 m_hWeapon
DT_BaseViewModel!0x1a48 m_tracerAttachments
DT_BaseViewModel!0x1a50 m_tracerAttachmentsScoped
```
</details>
<details>
<summary><code>class DT_Beam</code></summary>

```
{
	beampredictable_id: DT_BeamPredictableId,
	m_localOrigin: Vector,
	moveparent: Int,
	m_parentAttachmentType: Int,
	m_clrRender: Int,
	m_nModelIndex: Int,
	m_visibilityFlags: Int,
	m_iTeamNum: Int,
	m_hOwnerEntity: Int,
	m_nRenderFX: Int,
	m_nRenderMode: Int,
	m_parentAttachmentIndex: Int,
	m_parentAttachmentHitbox: Int,
	m_parentAttachmentModel: Int,
	m_flFrameRate: Float,
	m_flHDRColorScale: Float,
	m_clrRenderFriendly: Int,
	m_nNumBeamEnts: Int,
	m_nHaloIndex: Int,
	m_nBeamType: Int,
	m_nBeamFlags: Int,
	m_hAttachEntity: DataTable,
	m_nAttachIndex: DataTable,
	m_fWidth: Float,
	m_fEndWidth: Float,
	m_fFadeLength: Float,
	m_fHaloScale: Float,
	m_fAmplitude: Float,
	m_fStartFrame: Float,
	m_fSpeed: Float,
	m_flFrame: Float,
	m_nClipStyle: Int,
	m_vecEndPos: Vector,
}
```

### Offsets

```
DT_Beam!0x0000 beampredictable_id
DT_Beam!0x0004 m_localOrigin
DT_Beam!0x001c moveparent
DT_Beam!0x0020 m_parentAttachmentType
DT_Beam!0x0048 m_clrRender
DT_Beam!0x005c m_nModelIndex
DT_Beam!0x03e8 m_visibilityFlags
DT_Beam!0x03f0 m_iTeamNum
DT_Beam!0x0438 m_hOwnerEntity
DT_Beam!0x043d m_nRenderFX
DT_Beam!0x0449 m_nRenderMode
DT_Beam!0x07e4 m_parentAttachmentIndex
DT_Beam!0x07e8 m_parentAttachmentHitbox
DT_Beam!0x07ec m_parentAttachmentModel
DT_Beam!0x0a00 m_flFrameRate
DT_Beam!0x0a04 m_flHDRColorScale
DT_Beam!0x0a08 m_clrRenderFriendly
DT_Beam!0x0a14 m_nNumBeamEnts
DT_Beam!0x0a1c m_nHaloIndex
DT_Beam!0x0a20 m_nBeamType
DT_Beam!0x0a24 m_nBeamFlags
DT_Beam!0x0a28 m_hAttachEntity
DT_Beam!0x0a50 m_nAttachIndex
DT_Beam!0x0a78 m_fWidth
DT_Beam!0x0a7c m_fEndWidth
DT_Beam!0x0a80 m_fFadeLength
DT_Beam!0x0a84 m_fHaloScale
DT_Beam!0x0a88 m_fAmplitude
DT_Beam!0x0a8c m_fStartFrame
DT_Beam!0x0a90 m_fSpeed
DT_Beam!0x0a94 m_flFrame
DT_Beam!0x0a98 m_nClipStyle
DT_Beam!0x0a9c m_vecEndPos
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
DT_BoneFollower!0x0044 m_cellX
DT_BoneFollower!0x0048 m_cellY
DT_BoneFollower!0x004c m_cellZ
DT_BoneFollower!0x0050 m_localOrigin
DT_BoneFollower!0x005c m_nModelIndex
DT_BoneFollower!0x0394 m_networkedFlags
DT_BoneFollower!0x0424 m_localAngles
DT_BoneFollower!0x0438 m_hOwnerEntity
DT_BoneFollower!0x0450 m_Collision
DT_BoneFollower!0x04c8 m_CollisionGroup
DT_BoneFollower!0x0a00 m_modelIndex
DT_BoneFollower!0x0a04 m_boneIndex
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
DT_CPropDoor!0x003c m_fEffects
DT_CPropDoor!0x0040 m_usableType
DT_CPropDoor!0x0044 m_cellX
DT_CPropDoor!0x0048 m_cellY
DT_CPropDoor!0x004c m_cellZ
DT_CPropDoor!0x0050 m_localOrigin
DT_CPropDoor!0x005c m_nModelIndex
DT_CPropDoor!0x0394 m_networkedFlags
DT_CPropDoor!0x0424 m_localAngles
DT_CPropDoor!0x13b0 m_closedAngle
DT_CPropDoor!0x13b4 m_angle
DT_CPropDoor!0x13b8 m_startAngle
DT_CPropDoor!0x13bc m_startAngleVel
DT_CPropDoor!0x13c0 m_startMoveTime
DT_CPropDoor!0x13c4 m_isLocked
DT_CPropDoor!0x13c8 m_oppositeDoor
DT_CPropDoor!0x1418 m_interactingPlayer
DT_CPropDoor!0x141c m_interactingPlayerWantsOpen
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
DT_CollisionProperty!0x0039 m_nSurroundType
DT_CollisionProperty!0x0040 m_vecSpecifiedSurroundingMins
DT_CollisionProperty!0x004c m_vecSpecifiedSurroundingMaxs
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
DT_ColorCorrection!0x0438 m_hOwnerEntity
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
	m_vecPunchBase_Angle: Vector,
	m_vecPunchBase_AngleVel: Vector,
	m_vecPunchWeapon_Angle: Vector,
	m_vecPunchWeapon_AngleVel: Vector,
	m_localGravityRotation: Rotation,
}
```

### Offsets

```
DT_CurrentData_LocalPlayer!0x0000 m_viewConeAngleMin
DT_CurrentData_LocalPlayer!0x000c m_viewConeAngleMax
DT_CurrentData_LocalPlayer!0x0018 m_stepSmoothingOffset
DT_CurrentData_LocalPlayer!0x0024 m_vecPunchBase_Angle
DT_CurrentData_LocalPlayer!0x0030 m_vecPunchBase_AngleVel
DT_CurrentData_LocalPlayer!0x003c m_vecPunchWeapon_Angle
DT_CurrentData_LocalPlayer!0x0048 m_vecPunchWeapon_AngleVel
DT_CurrentData_LocalPlayer!0x0054 m_localGravityRotation
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
DT_DoorMover!0x003c m_fEffects
DT_DoorMover!0x0040 m_usableType
DT_DoorMover!0x0044 m_cellX
DT_DoorMover!0x0048 m_cellY
DT_DoorMover!0x004c m_cellZ
DT_DoorMover!0x0050 m_localOrigin
DT_DoorMover!0x005c m_nModelIndex
DT_DoorMover!0x0108 m_vecAngVelocity
DT_DoorMover!0x0394 m_networkedFlags
DT_DoorMover!0x0418 m_vecVelocity
DT_DoorMover!0x0424 m_localAngles
DT_DoorMover!0x0450 m_Collision
DT_DoorMover!0x04c8 m_CollisionGroup
DT_DoorMover!0x0508 m_iSignifierName
DT_DoorMover!0x0618 m_scriptNameIndex
DT_DoorMover!0x06a0 m_holdUsePrompt
DT_DoorMover!0x06a8 m_pressUsePrompt
DT_DoorMover!0x07e4 m_parentAttachmentIndex
DT_DoorMover!0x07e8 m_parentAttachmentHitbox
DT_DoorMover!0x07ec m_parentAttachmentModel
DT_DoorMover!0x07f8 m_fadeDist
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
DT_DynamicProp!0x0720 m_lifeState
DT_DynamicProp!0x1341 m_bUseHitboxesForRenderBox
DT_DynamicProp!0x1342 m_bAnimateInStaticShadow
DT_DynamicProp!0x1343 m_wantsScopeHighlight
```
</details>
<details>
<summary><code>class DT_DynamicPropLightweight</code></summary>

```
{
	moveparent: Int,
	m_parentAttachmentType: Int,
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
	m_parentAttachmentIndex: Int,
	m_parentAttachmentModel: Int,
	m_fadeDist: Float,
	m_nSkin: Int,
}
```

### Offsets

```
DT_DynamicPropLightweight!0x001c moveparent
DT_DynamicPropLightweight!0x0020 m_parentAttachmentType
DT_DynamicPropLightweight!0x003c m_fEffects
DT_DynamicPropLightweight!0x0044 m_cellX
DT_DynamicPropLightweight!0x0048 m_cellY
DT_DynamicPropLightweight!0x004c m_cellZ
DT_DynamicPropLightweight!0x0050 m_localOrigin
DT_DynamicPropLightweight!0x005c m_nModelIndex
DT_DynamicPropLightweight!0x0394 m_networkedFlags
DT_DynamicPropLightweight!0x03e8 m_visibilityFlags
DT_DynamicPropLightweight!0x0424 m_localAngles
DT_DynamicPropLightweight!0x0450 m_Collision
DT_DynamicPropLightweight!0x04c8 m_CollisionGroup
DT_DynamicPropLightweight!0x07e4 m_parentAttachmentIndex
DT_DynamicPropLightweight!0x07ec m_parentAttachmentModel
DT_DynamicPropLightweight!0x07f8 m_fadeDist
DT_DynamicPropLightweight!0x0e48 m_nSkin
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
<summary><code>class DT_EnvTonemapController extends DT_BaseEntity</code></summary>

```
{
	m_bUseCustomAutoExposureMin: Int,
	m_bUseCustomAutoExposureMax: Int,
	m_bUseCustomAutoExposureRate: Int,
	m_bUseCustomBloomScale: Int,
	m_flCustomAutoExposureMin: Float,
	m_flCustomAutoExposureMax: Float,
	m_flCustomAutoExposureRate: Float,
	m_flCustomBloomScale: Float,
}
```

### Offsets

```
DT_EnvTonemapController!0x0a00 m_bUseCustomAutoExposureMin
DT_EnvTonemapController!0x0a01 m_bUseCustomAutoExposureMax
DT_EnvTonemapController!0x0a02 m_bUseCustomAutoExposureRate
DT_EnvTonemapController!0x0a03 m_bUseCustomBloomScale
DT_EnvTonemapController!0x0a04 m_flCustomAutoExposureMin
DT_EnvTonemapController!0x0a08 m_flCustomAutoExposureMax
DT_EnvTonemapController!0x0a0c m_flCustomAutoExposureRate
DT_EnvTonemapController!0x0a10 m_flCustomBloomScale
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
<summary><code>class DT_FogController</code></summary>

```
{
	m_fog.botAlt: Float,
	m_fog.topAlt: Float,
	m_fog.halfDistBot: Float,
	m_fog.halfDistTop: Float,
	m_fog.distColorStr: Float,
	m_fog.dirColorStr: Float,
	m_fog.distOffset: Float,
	m_fog.densityScale: Float,
	m_fog.halfAngleDeg: Float,
	m_fog.HDRColorScale: Float,
	m_fog.distColor: Int,
	m_fog.dirColor: Int,
	m_fog.direction: Vector,
	m_fog.minFadeTime: Float,
	m_fog.forceOntoSky: Int,
	m_fog.enable: Int,
	m_fog.id: Int,
}
```

### Offsets

```
DT_FogController!0x0a00 m_fog.botAlt
DT_FogController!0x0a04 m_fog.topAlt
DT_FogController!0x0a08 m_fog.halfDistBot
DT_FogController!0x0a0c m_fog.halfDistTop
DT_FogController!0x0a10 m_fog.distColorStr
DT_FogController!0x0a14 m_fog.dirColorStr
DT_FogController!0x0a18 m_fog.distOffset
DT_FogController!0x0a1c m_fog.densityScale
DT_FogController!0x0a20 m_fog.halfAngleDeg
DT_FogController!0x0a24 m_fog.HDRColorScale
DT_FogController!0x0a28 m_fog.distColor
DT_FogController!0x0a2c m_fog.dirColor
DT_FogController!0x0a30 m_fog.direction
DT_FogController!0x0a3c m_fog.minFadeTime
DT_FogController!0x0a40 m_fog.forceOntoSky
DT_FogController!0x0a41 m_fog.enable
DT_FogController!0x0a44 m_fog.id
```
</details>
<details>
<summary><code>class DT_FuncBrushLightweight</code></summary>

```
{
	moveparent: Int,
	m_parentAttachmentType: Int,
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
	m_parentAttachmentIndex: Int,
	m_parentAttachmentHitbox: Int,
	m_parentAttachmentModel: Int,
}
```

### Offsets

```
DT_FuncBrushLightweight!0x001c moveparent
DT_FuncBrushLightweight!0x0020 m_parentAttachmentType
DT_FuncBrushLightweight!0x0044 m_cellX
DT_FuncBrushLightweight!0x0048 m_cellY
DT_FuncBrushLightweight!0x004c m_cellZ
DT_FuncBrushLightweight!0x0050 m_localOrigin
DT_FuncBrushLightweight!0x005c m_nModelIndex
DT_FuncBrushLightweight!0x0394 m_networkedFlags
DT_FuncBrushLightweight!0x03e8 m_visibilityFlags
DT_FuncBrushLightweight!0x0424 m_localAngles
DT_FuncBrushLightweight!0x0450 m_Collision
DT_FuncBrushLightweight!0x04c8 m_CollisionGroup
DT_FuncBrushLightweight!0x07e4 m_parentAttachmentIndex
DT_FuncBrushLightweight!0x07e8 m_parentAttachmentHitbox
DT_FuncBrushLightweight!0x07ec m_parentAttachmentModel
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
	m_cellX: Int,
	m_cellY: Int,
	m_cellZ: Int,
	m_localOrigin: Vector,
	m_nModelIndex: Int,
	m_visibilityFlags: Int,
	m_localAngles: Vector,
	m_hOwnerEntity: Int,
	m_parentAttachmentIndex: Int,
	m_parentAttachmentHitbox: Int,
	m_realmsBitMask: BitMask,
	m_grappleZipline: Int,
}
```

### Offsets

```
DT_GrappleHook!0x001c moveparent
DT_GrappleHook!0x0020 m_parentAttachmentType
DT_GrappleHook!0x0044 m_cellX
DT_GrappleHook!0x0048 m_cellY
DT_GrappleHook!0x004c m_cellZ
DT_GrappleHook!0x0050 m_localOrigin
DT_GrappleHook!0x005c m_nModelIndex
DT_GrappleHook!0x03e8 m_visibilityFlags
DT_GrappleHook!0x0424 m_localAngles
DT_GrappleHook!0x0438 m_hOwnerEntity
DT_GrappleHook!0x07e4 m_parentAttachmentIndex
DT_GrappleHook!0x07e8 m_parentAttachmentHitbox
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
DT_HighlightSettings!0x0198 m_highlightParams
DT_HighlightSettings!0x0258 m_highlightFunctionBits
DT_HighlightSettings!0x0298 m_highlightServerFadeBases
DT_HighlightSettings!0x02a0 m_highlightServerFadeStartTimes
DT_HighlightSettings!0x02a8 m_highlightServerFadeEndTimes
DT_HighlightSettings!0x02e8 m_highlightServerContextID
DT_HighlightSettings!0x02f4 m_highlightTeamBits
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
	m_localAngles: Vector,
	m_parentAttachmentType: Int,
	m_parentAttachmentIndex: Int,
	m_parentAttachmentHitbox: Int,
	m_parentAttachmentModel: Int,
}
```

### Offsets

```
DT_InfoPlacementHelper!0x0004 m_localOrigin
DT_InfoPlacementHelper!0x001c moveparent
DT_InfoPlacementHelper!0x0424 m_localAngles
DT_InfoPlacementHelper!0x07e0 m_parentAttachmentType
DT_InfoPlacementHelper!0x07e4 m_parentAttachmentIndex
DT_InfoPlacementHelper!0x07e8 m_parentAttachmentHitbox
DT_InfoPlacementHelper!0x07ec m_parentAttachmentModel
```
</details>
<details>
<summary><code>class DT_InfoTarget</code></summary>

```
{
	moveparent: Int,
	m_cellX: Int,
	m_cellY: Int,
	m_cellZ: Int,
	m_localOrigin: Vector,
	m_iTeamNum: Int,
	m_localAngles: Vector,
	m_hOwnerEntity: Int,
	m_iSignifierName: String,
	m_iName: String,
	m_scriptNameIndex: Int,
	m_instanceNameIndex: Int,
	m_parentAttachmentType: Int,
	m_parentAttachmentIndex: Int,
	m_parentAttachmentHitbox: Int,
	m_parentAttachmentModel: Int,
	m_firstChildEntityLink: Int,
	m_firstParentEntityLink: Int,
}
```

### Offsets

```
DT_InfoTarget!0x001c moveparent
DT_InfoTarget!0x0044 m_cellX
DT_InfoTarget!0x0048 m_cellY
DT_InfoTarget!0x004c m_cellZ
DT_InfoTarget!0x0050 m_localOrigin
DT_InfoTarget!0x03f0 m_iTeamNum
DT_InfoTarget!0x0424 m_localAngles
DT_InfoTarget!0x0438 m_hOwnerEntity
DT_InfoTarget!0x0508 m_iSignifierName
DT_InfoTarget!0x0511 m_iName
DT_InfoTarget!0x0618 m_scriptNameIndex
DT_InfoTarget!0x061c m_instanceNameIndex
DT_InfoTarget!0x07e0 m_parentAttachmentType
DT_InfoTarget!0x07e4 m_parentAttachmentIndex
DT_InfoTarget!0x07e8 m_parentAttachmentHitbox
DT_InfoTarget!0x07ec m_parentAttachmentModel
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
	m_nDuckTransitionTimeMsecs: Int,
	m_superJumpsUsed: Int,
	m_jumpedOffRodeo: Int,
	m_jumpPressTime: Time,
	m_jetpackActivateTime: Time,
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
DT_Local!0x0014 m_nDuckTransitionTimeMsecs
DT_Local!0x0018 m_superJumpsUsed
DT_Local!0x001c m_jumpedOffRodeo
DT_Local!0x0020 m_jumpPressTime
DT_Local!0x0024 m_jetpackActivateTime
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
DT_Local!0x024c m_playerLocalGravityBlendStartRotation
DT_Local!0x025c m_playerLocalGravityBlendEndRotation
DT_Local!0x026c m_playerLocalGravityBlendEndDirection
DT_Local!0x0278 m_playerLocalGravityBlendStartTime
DT_Local!0x027c m_playerLocalGravityBlendEndTime
DT_Local!0x0280 m_playerLocalGravityBlendStrength
DT_Local!0x0284 m_playerLocalGravityStrength
DT_Local!0x0288 m_playerLocalGravityType
DT_Local!0x028c m_playerLocalGravityPoint
DT_Local!0x0298 m_playerLocalGravityLineStart
DT_Local!0x02a4 m_playerLocalGravityLineEnd
DT_Local!0x02b0 m_playerLocalGravityEntity
DT_Local!0x02b4 m_playerLocalGravityLineStartEntity
DT_Local!0x02b8 m_playerLocalGravityLineEndEntity
DT_Local!0x02bc m_playerFloatLookStartTime
DT_Local!0x02c0 m_playerFloatLookEndTime
DT_Local!0x02c4 m_wallrunLatestFloorHeight
DT_Local!0x02c8 m_groundNormal
DT_Local!0x02d4 m_continuousUseBlocked
DT_Local!0x02d8 m_useEnt
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
	m_hTonemapController: Int,
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
	m_repeatedBoost: Int,
	m_boostMeter: Float,
	m_jetpack: Int,
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
	m_pushedFixedPointOffset: DataTable,
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
	m_skydiveIsDiving: Int,
	m_skydiveFreelookEnabled: Int,
	m_skydiveFreelookLockedAngle: Vector,
	m_skydiveFollowing: Int,
	m_skydiveUnfollowVelocity: Vector,
	m_skydiveIsNearLeviathan: Int,
	m_skydiveLeviathanHitPosition: Vector,
	m_skydiveLeviathanHitNormal: Vector,
	m_skydiveSlipVelocity: Vector,
	m_playerKnockBacks: DataTable,
	m_armsModelIndex: Int,
}
```

### Offsets

```
DT_LocalPlayerExclusive!0x0000 NearbyPushers
DT_LocalPlayerExclusive!0x0004 m_localOrigin
DT_LocalPlayerExclusive!0x000c m_localOrigin.z
DT_LocalPlayerExclusive!0x0120 m_vecAbsVelocity
DT_LocalPlayerExclusive!0x03d0 m_vecBaseVelocity
DT_LocalPlayerExclusive!0x0418 m_vecVelocity.x
DT_LocalPlayerExclusive!0x041c m_vecVelocity.y
DT_LocalPlayerExclusive!0x0420 m_vecVelocity.z
DT_LocalPlayerExclusive!0x0430 m_flFriction
DT_LocalPlayerExclusive!0x1794 m_tethers
DT_LocalPlayerExclusive!0x1850 m_lastUCmdSimulationTicks
DT_LocalPlayerExclusive!0x1854 m_lastUCmdSimulationRemainderTime
DT_LocalPlayerExclusive!0x1a10 m_Local
DT_LocalPlayerExclusive!0x1ec0 m_currentFrameLocalPlayer
DT_LocalPlayerExclusive!0x20c0 m_hTonemapController
DT_LocalPlayerExclusive!0x2304 m_modInventory
DT_LocalPlayerExclusive!0x2384 m_consumableInventory
DT_LocalPlayerExclusive!0x2668 m_fStickySprintMinTime
DT_LocalPlayerExclusive!0x2678 m_sprintStartedTime
DT_LocalPlayerExclusive!0x267c m_sprintStartedFrac
DT_LocalPlayerExclusive!0x2680 m_sprintEndedTime
DT_LocalPlayerExclusive!0x2684 m_sprintEndedFrac
DT_LocalPlayerExclusive!0x2688 m_stickySprintStartTime
DT_LocalPlayerExclusive!0x26ec m_upDirPredicted
DT_LocalPlayerExclusive!0x26f8 m_lastWallRunStartPos
DT_LocalPlayerExclusive!0x271c m_wallrunFrictionScale
DT_LocalPlayerExclusive!0x2720 m_groundFrictionScale
DT_LocalPlayerExclusive!0x2764 m_traversalBegin
DT_LocalPlayerExclusive!0x2770 m_traversalMid
DT_LocalPlayerExclusive!0x277c m_traversalEnd
DT_LocalPlayerExclusive!0x2788 m_traversalMidFrac
DT_LocalPlayerExclusive!0x278c m_traversalForwardDir
DT_LocalPlayerExclusive!0x27a4 m_traversalProgress
DT_LocalPlayerExclusive!0x27a8 m_traversalStartTime
DT_LocalPlayerExclusive!0x27ac m_traversalHandAppearTime
DT_LocalPlayerExclusive!0x27b0 m_traversalReleaseTime
DT_LocalPlayerExclusive!0x27b4 m_traversalBlendOutStartTime
DT_LocalPlayerExclusive!0x27b8 m_traversalBlendOutStartOffset
DT_LocalPlayerExclusive!0x27d0 m_wallDangleJumpOffTime
DT_LocalPlayerExclusive!0x27d4 m_wallDangleMayHangHere
DT_LocalPlayerExclusive!0x27d5 m_wallDangleForceFallOff
DT_LocalPlayerExclusive!0x27d6 m_wallDangleLastPushedForward
DT_LocalPlayerExclusive!0x27d8 m_wallDangleDisableWeapon
DT_LocalPlayerExclusive!0x288c m_slowMoEnabled
DT_LocalPlayerExclusive!0x288d m_sliding
DT_LocalPlayerExclusive!0x288e m_slideLongJumpAllowed
DT_LocalPlayerExclusive!0x289c m_bIsStickySprinting
DT_LocalPlayerExclusive!0x28a0 m_prevMoveYaw
DT_LocalPlayerExclusive!0x28a4 m_sprintTiltVel
DT_LocalPlayerExclusive!0x28a8 m_sprintTiltPoseParameter
DT_LocalPlayerExclusive!0x28ac m_sprintFracPoseParameter
DT_LocalPlayerExclusive!0x2a14 m_ziplineAllowed
DT_LocalPlayerExclusive!0x2a1c m_lastZipline
DT_LocalPlayerExclusive!0x2a20 m_lastZiplineDetachTime
DT_LocalPlayerExclusive!0x2a30 m_zipline
DT_LocalPlayerExclusive!0x2aa0 m_ziplineViewOffsetPosition
DT_LocalPlayerExclusive!0x2aac m_ziplineViewOffsetVelocity
DT_LocalPlayerExclusive!0x2ab8 m_ziplineGrenadeEntity
DT_LocalPlayerExclusive!0x2ac8 m_highSpeedViewmodelAnims
DT_LocalPlayerExclusive!0x2acc m_playAnimationType
DT_LocalPlayerExclusive!0x2ad0 m_detachGrappleOnPlayAnimationEnd
DT_LocalPlayerExclusive!0x2ad4 m_playAnimationNext
DT_LocalPlayerExclusive!0x2adc m_playAnimationEntityBlocker
DT_LocalPlayerExclusive!0x2ae0 m_playAnimationEntityBlockerDucking
DT_LocalPlayerExclusive!0x2ae8 m_boosting
DT_LocalPlayerExclusive!0x2ae9 m_repeatedBoost
DT_LocalPlayerExclusive!0x2aec m_boostMeter
DT_LocalPlayerExclusive!0x2af0 m_jetpack
DT_LocalPlayerExclusive!0x2af1 m_jetpackAfterburner
DT_LocalPlayerExclusive!0x2af2 m_gliding
DT_LocalPlayerExclusive!0x2af4 m_glideMeter
DT_LocalPlayerExclusive!0x2af8 m_glideRechargeDelayAccumulator
DT_LocalPlayerExclusive!0x2afc m_hovering
DT_LocalPlayerExclusive!0x2b00 m_lastJumpHeight
DT_LocalPlayerExclusive!0x2b50 m_slipAirRestrictDirection
DT_LocalPlayerExclusive!0x2b5c m_slipAirRestrictTime
DT_LocalPlayerExclusive!0x2c98 m_replayImportantSounds_networkTableSoundID
DT_LocalPlayerExclusive!0x2ca8 m_replayImportantSounds_beginTime
DT_LocalPlayerExclusive!0x2ce5 m_viewConeActive
DT_LocalPlayerExclusive!0x2ce6 m_viewConeParented
DT_LocalPlayerExclusive!0x2ce8 m_viewConeParity
DT_LocalPlayerExclusive!0x2fcc m_hConstraintEntity
DT_LocalPlayerExclusive!0x2fd0 m_vecConstraintCenter
DT_LocalPlayerExclusive!0x2fdc m_flConstraintRadius
DT_LocalPlayerExclusive!0x2fe0 m_flConstraintWidth
DT_LocalPlayerExclusive!0x2fe4 m_flConstraintSpeedFactor
DT_LocalPlayerExclusive!0x2fe8 m_bConstraintPastRadius
DT_LocalPlayerExclusive!0x304c m_observerModeStaticPosition
DT_LocalPlayerExclusive!0x3058 m_observerModeStaticAngles
DT_LocalPlayerExclusive!0x30cc m_lastKillTime
DT_LocalPlayerExclusive!0x30f4 m_wallRunStartTime
DT_LocalPlayerExclusive!0x30f8 m_wallRunClearTime
DT_LocalPlayerExclusive!0x310c m_dodging
DT_LocalPlayerExclusive!0x3166 m_dodgingInAir
DT_LocalPlayerExclusive!0x3180 m_airSpeed
DT_LocalPlayerExclusive!0x3184 m_airAcceleration
DT_LocalPlayerExclusive!0x31b0 m_firstPersonProxy
DT_LocalPlayerExclusive!0x31b4 m_predictedFirstPersonProxy
DT_LocalPlayerExclusive!0x31c4 m_hardpointEntity
DT_LocalPlayerExclusive!0x3214 m_petTitanMode
DT_LocalPlayerExclusive!0x321c m_hThirdPersonEnt
DT_LocalPlayerExclusive!0x3220 m_thirdPersonShoulderView
DT_LocalPlayerExclusive!0x327c m_thirdPerson
DT_LocalPlayerExclusive!0x334c m_viewConeLerpTime
DT_LocalPlayerExclusive!0x3604 m_flLaggedMovementValue
DT_LocalPlayerExclusive!0x3608 m_lastMoveInputTime
DT_LocalPlayerExclusive!0x360c m_ignoreEntityForMovementUntilNotTouching
DT_LocalPlayerExclusive!0x3c8c m_lungeTargetEntity
DT_LocalPlayerExclusive!0x3c90 m_isLungingToPosition
DT_LocalPlayerExclusive!0x3c94 m_lungeTargetPosition
DT_LocalPlayerExclusive!0x3ca0 m_lungeStartPositionOffset
DT_LocalPlayerExclusive!0x3cac m_lungeEndPositionOffset
DT_LocalPlayerExclusive!0x3cb8 m_lungeStartTime
DT_LocalPlayerExclusive!0x3cbc m_lungeEndTime
DT_LocalPlayerExclusive!0x3cc0 m_lungeCanFly
DT_LocalPlayerExclusive!0x3cc1 m_lungeLockPitch
DT_LocalPlayerExclusive!0x3cc4 m_lungeStartPitch
DT_LocalPlayerExclusive!0x3cc8 m_lungeSmoothTime
DT_LocalPlayerExclusive!0x3ccc m_lungeMaxTime
DT_LocalPlayerExclusive!0x3cd0 m_lungeMaxEndSpeed
DT_LocalPlayerExclusive!0x41fc m_nearbyPusherCount
DT_LocalPlayerExclusive!0x420c m_pushedFixedPointOffset
DT_LocalPlayerExclusive!0x4224 m_pushAwayFromTopAcceleration
DT_LocalPlayerExclusive!0x4234 m_minimapTargetZoomScale
DT_LocalPlayerExclusive!0x4238 m_minimapTargetLerpTime
DT_LocalPlayerExclusive!0x4240 m_playerScriptNetDataExclusive
DT_LocalPlayerExclusive!0x4270 m_skydiveForwardPoseValueVelocity
DT_LocalPlayerExclusive!0x4278 m_skydiveForwardPoseValueCurrent
DT_LocalPlayerExclusive!0x427c m_skydiveSidePoseValueVelocity
DT_LocalPlayerExclusive!0x4284 m_skydiveSidePoseValueCurrent
DT_LocalPlayerExclusive!0x4288 m_skydiveYawVelocity
DT_LocalPlayerExclusive!0x42a8 m_freefallStartTime
DT_LocalPlayerExclusive!0x42ac m_freefallEndTime
DT_LocalPlayerExclusive!0x42b0 m_freefallAnticipateStartTime
DT_LocalPlayerExclusive!0x42b4 m_freefallAnticipateEndTime
DT_LocalPlayerExclusive!0x42b8 m_freefallDistanceToLand
DT_LocalPlayerExclusive!0x42c0 m_skydiveIsDiving
DT_LocalPlayerExclusive!0x42cc m_skydiveFreelookEnabled
DT_LocalPlayerExclusive!0x42d0 m_skydiveFreelookLockedAngle
DT_LocalPlayerExclusive!0x42e4 m_skydiveFollowing
DT_LocalPlayerExclusive!0x42e8 m_skydiveUnfollowVelocity
DT_LocalPlayerExclusive!0x42f5 m_skydiveIsNearLeviathan
DT_LocalPlayerExclusive!0x42f8 m_skydiveLeviathanHitPosition
DT_LocalPlayerExclusive!0x4304 m_skydiveLeviathanHitNormal
DT_LocalPlayerExclusive!0x4310 m_skydiveSlipVelocity
DT_LocalPlayerExclusive!0x4320 m_playerKnockBacks
DT_LocalPlayerExclusive!0x43a0 m_armsModelIndex
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
DT_NPC_SentryTurret!0x1980 m_turretState
DT_NPC_SentryTurret!0x1984 m_killCount
DT_NPC_SentryTurret!0x1988 m_titanKillCount
DT_NPC_SentryTurret!0x198c m_eyeAttach
DT_NPC_SentryTurret!0x1990 m_controlPanel
```
</details>
<details>
<summary><code>class DT_ParticleSystem</code></summary>

```
{
	m_localOrigin: Vector,
	moveparent: Int,
	m_parentAttachmentType: Int,
	m_fEffects: Int,
	m_visibilityFlags: Int,
	m_iTeamNum: Int,
	m_localAngles: Vector,
	m_hOwnerEntity: Int,
	m_parentAttachmentIndex: Int,
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
DT_ParticleSystem!0x003c m_fEffects
DT_ParticleSystem!0x03e8 m_visibilityFlags
DT_ParticleSystem!0x03f0 m_iTeamNum
DT_ParticleSystem!0x0424 m_localAngles
DT_ParticleSystem!0x0438 m_hOwnerEntity
DT_ParticleSystem!0x07e4 m_parentAttachmentIndex
DT_ParticleSystem!0x07e8 m_parentAttachmentHitbox
DT_ParticleSystem!0x07ec m_parentAttachmentModel
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
	m_iPhysicsMode: Int,
	m_fMass: Float,
	m_collisionMins: Vector,
	m_collisionMaxs: Vector,
}
```

### Offsets

```
DT_PhysicsProp!0x079c m_spawnflags
DT_PhysicsProp!0x1348 m_bAwake
DT_PhysicsProp!0x1349 m_ignoresCollisionWithPlayers
DT_PhysicsProp!0x1364 m_iPhysicsMode
DT_PhysicsProp!0x1368 m_fMass
DT_PhysicsProp!0x136c m_collisionMins
DT_PhysicsProp!0x1378 m_collisionMaxs
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
	m_rodeo: DT_Rodeo_PlayerData,
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
	m_titanSoulBeingRodeoed: Int,
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
	m_remoteTurret: Int,
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
	m_activeBurnCardIndex: Int,
	m_flDeathTime: Time,
	m_lastDodgeTime: Time,
	m_timeJetpackHeightActivateCheckPassed: Time,
	m_grappleHook: Int,
	m_petTitan: Int,
	m_xp: Int,
	m_generation: Int,
	m_rank: Int,
	m_serverForceIncreasePlayerListGenerationParity: Int,
	m_isPlayingRanked: Int,
	m_skill_mu: Float,
	m_nextTitanRespawnAvailable: Float,
	m_ubEFNoInterpParity: Int,
	m_hColorCorrectionCtrl: Int,
	m_PlayerFog.m_hCtrl: Int,
	m_title: String,
	m_smartAmmoHighestLocksOnMeFractionValues: DataTable,
	m_smartAmmoHighestLocksOnMeEntities: DataTable,
	m_smartAmmoPreviousHighestLockOnMeFractionValue: Float,
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
DT_Player!0x008c m_fFlags
DT_Player!0x03dc m_hGroundEntity
DT_Player!0x03e0 m_iHealth
DT_Player!0x03e4 m_flMaxspeed
DT_Player!0x0500 m_iMaxHealth
DT_Player!0x0720 m_lifeState
DT_Player!0x0e54 m_decalIndex
DT_Player!0x1630 m_inventory
DT_Player!0x168e m_selectedOffhands
DT_Player!0x1691 m_selectedOffhandsPendingHybridAction
DT_Player!0x16fc m_titanSoul
DT_Player!0x17c1 m_bZooming
DT_Player!0x17c4 m_zoomToggleOnStartTime
DT_Player!0x17c8 m_zoomBaseFrac
DT_Player!0x17cc m_zoomBaseTime
DT_Player!0x17d0 m_zoomFullStartTime
DT_Player!0x1cf0 m_currentFramePlayer
DT_Player!0x20c8 pl
DT_Player!0x2148 m_rodeo
DT_Player!0x223c m_ammoPoolCapacity
DT_Player!0x2240 m_hasBadReputation
DT_Player!0x2241 m_happyHourActive
DT_Player!0x2249 m_communityName
DT_Player!0x2289 m_communityClanTag
DT_Player!0x2299 m_factionName
DT_Player!0x22a9 m_hardwareIcon
DT_Player!0x22b9 m_hardware
DT_Player!0x22c0 m_platformUserId
DT_Player!0x22d0 m_classModsActive
DT_Player!0x23c8 m_passives[ 0 ]
DT_Player!0x23e8 m_bleedoutState
DT_Player!0x23f0 m_statusEffectsTimedPlayerNV
DT_Player!0x24e0 m_statusEffectsEndlessPlayerNV
DT_Player!0x2594 m_damageComboLatestUpdateTime
DT_Player!0x2598 m_damageComboStartHealth
DT_Player!0x259c m_gestureSequences
DT_Player!0x25b4 m_gestureStartTimes
DT_Player!0x25cc m_gestureBlendInDuration
DT_Player!0x25e4 m_gestureBlendOutDuration
DT_Player!0x25fc m_gestureFadeOutStartTime
DT_Player!0x2614 m_gestureFadeOutDuration
DT_Player!0x262c m_gestureAutoKillBitfield
DT_Player!0x2670 m_autoSprintForced
DT_Player!0x2674 m_fIsSprinting
DT_Player!0x268c m_damageImpulseNoDecelEndTime
DT_Player!0x2694 m_playerVehicle
DT_Player!0x2698 m_titanSoulBeingRodeoed
DT_Player!0x26a0 m_duckState
DT_Player!0x26a4 m_leanState
DT_Player!0x26a9 m_canStand
DT_Player!0x26ac m_StandHullMin
DT_Player!0x26b8 m_StandHullMax
DT_Player!0x26c4 m_DuckHullMin
DT_Player!0x26d0 m_DuckHullMax
DT_Player!0x26dc m_entitySyncingWithMe
DT_Player!0x26e0 m_upDir
DT_Player!0x275c m_traversalState
DT_Player!0x2760 m_traversalType
DT_Player!0x278c m_traversalForwardDir
DT_Player!0x2798 m_traversalRefPos
DT_Player!0x27c4 m_traversalYawDelta
DT_Player!0x27c8 m_traversalYawPoseParameter
DT_Player!0x27e0 m_wallClimbSetUp
DT_Player!0x27e1 m_wallHanging
DT_Player!0x27e8 m_grapple
DT_Player!0x2878 m_grappleActive
DT_Player!0x28c0 m_remoteTurret
DT_Player!0x28c4 m_hViewModels
DT_Player!0x28d8 m_viewOffsetEntity
DT_Player!0x2918 m_animViewEntity
DT_Player!0x2a18 m_activeZipline
DT_Player!0x2a24 m_ziplineValid3pWeaponLayerAnim
DT_Player!0x2a28 m_ziplineState
DT_Player!0x2abc m_ziplineGrenadeBeginStationEntity
DT_Player!0x2ac0 m_ziplineGrenadeBeginStationAttachmentIndex
DT_Player!0x2afd m_isPerformingBoostAction
DT_Player!0x2bb8 m_lastJumpPadTouched
DT_Player!0x2bc0 m_launchCount
DT_Player!0x2cf0 m_melee
DT_Player!0x2d20 m_useCredit
DT_Player!0x2d24 m_playerFlags
DT_Player!0x2d28 m_hasMic
DT_Player!0x2d29 m_inPartyChat
DT_Player!0x2d2c m_playerMoveSpeedScale
DT_Player!0x2fc8 m_bShouldDrawPlayerWhileUsingViewEntity
DT_Player!0x3034 m_iSpawnParity
DT_Player!0x303c m_iObserverMode
DT_Player!0x3040 m_hObserverTarget
DT_Player!0x3044 m_activeBurnCardIndex
DT_Player!0x30d0 m_flDeathTime
DT_Player!0x3110 m_lastDodgeTime
DT_Player!0x3130 m_timeJetpackHeightActivateCheckPassed
DT_Player!0x31b8 m_grappleHook
DT_Player!0x31bc m_petTitan
DT_Player!0x31e4 m_xp
DT_Player!0x31ec m_generation
DT_Player!0x31f0 m_rank
DT_Player!0x31f4 m_serverForceIncreasePlayerListGenerationParity
DT_Player!0x31f8 m_isPlayingRanked
DT_Player!0x31fc m_skill_mu
DT_Player!0x3200 m_nextTitanRespawnAvailable
DT_Player!0x3b08 m_ubEFNoInterpParity
DT_Player!0x3b0c m_hColorCorrectionCtrl
DT_Player!0x3b10 m_PlayerFog.m_hCtrl
DT_Player!0x3c68 m_title
DT_Player!0x3d10 m_smartAmmoHighestLocksOnMeFractionValues
DT_Player!0x3d20 m_smartAmmoHighestLocksOnMeEntities
DT_Player!0x3d30 m_smartAmmoPreviousHighestLockOnMeFractionValue
DT_Player!0x4060 m_Shared
DT_Player!0x40a0 m_pilotClassIndex
DT_Player!0x40a4 m_pilotClassActivityModifier
DT_Player!0x423c m_playerScriptNetDataGlobal
DT_Player!0x4244 m_helmetType
DT_Player!0x4248 m_armorType
DT_Player!0x424c m_controllerModeActive
DT_Player!0x4274 m_skydiveForwardPoseValueTarget
DT_Player!0x4280 m_skydiveSidePoseValueTarget
DT_Player!0x42a4 m_freefallState
DT_Player!0x42bc m_skydiveDiveAngle
DT_Player!0x42c4 m_skydiveSpeed
DT_Player!0x42c8 m_skydiveStrafeAngle
DT_Player!0x42dc m_skydivePlayerPitch
DT_Player!0x42e0 m_skydivePlayerYaw
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
DT_PlayerDecoy!0x017c m_cloakEndTime
DT_PlayerDecoy!0x0180 m_cloakFadeInEndTime
DT_PlayerDecoy!0x0184 m_cloakFadeOutStartTime
DT_PlayerDecoy!0x0188 m_cloakFadeInDuration
DT_PlayerDecoy!0x018c m_cloakFlickerAmount
DT_PlayerDecoy!0x0190 m_cloakFlickerEndTime
DT_PlayerDecoy!0x03e0 m_iHealth
DT_PlayerDecoy!0x0500 m_iMaxHealth
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
	attackActive: Int,
	attackRecoveryShouldBeQuick: Int,
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
DT_PlayerMelee_PlayerData!0x0008 attackActive
DT_PlayerMelee_PlayerData!0x0009 attackRecoveryShouldBeQuick
DT_PlayerMelee_PlayerData!0x000c attackStartTime
DT_PlayerMelee_PlayerData!0x0010 attackHitEntity
DT_PlayerMelee_PlayerData!0x0014 attackHitEntityTime
DT_PlayerMelee_PlayerData!0x0018 attackLastHitNonWorldEntity
DT_PlayerMelee_PlayerData!0x001c scriptedState
DT_PlayerMelee_PlayerData!0x0020 pendingMeleePress
DT_PlayerMelee_PlayerData!0x0024 lungeBoost
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
DT_PlayerTeamShared!0x2244 m_healResources_healthTarget
DT_PlayerTeamShared!0x2b60 m_lastTimeDamagedByOtherPlayer
DT_PlayerTeamShared!0x2b64 m_lastTimeDamagedByNPC
DT_PlayerTeamShared!0x2b68 m_lastTimeDidDamageToOtherPlayer
DT_PlayerTeamShared!0x2b6c m_lastTimeDidDamageToNPC
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
	m_vehicleFlags: Int,
	m_vehicleType: Int,
	m_vehicleLaunchTime: Float,
	m_vehicleVelocity: Vector,
}
```

### Offsets

```
DT_PlayerVehicle!0x0000 vehicledriverdata
DT_PlayerVehicle!0x0000 vehiclenondriverdata
DT_PlayerVehicle!0x0000 m_vehiclePlayers
DT_PlayerVehicle!0x0030 m_vecViewOffset.x
DT_PlayerVehicle!0x0034 m_vecViewOffset.y
DT_PlayerVehicle!0x0038 m_vecViewOffset.z
DT_PlayerVehicle!0x0424 m_localAngles
DT_PlayerVehicle!0x1304 m_vehicleDriver
DT_PlayerVehicle!0x130c m_vehiclePlayers[0]
DT_PlayerVehicle!0x131c m_vehiclePlayerCount
DT_PlayerVehicle!0x1320 m_vehicleActivated
DT_PlayerVehicle!0x1324 m_vehicleFlags
DT_PlayerVehicle!0x1328 m_vehicleType
DT_PlayerVehicle!0x132c m_vehicleLaunchTime
DT_PlayerVehicle!0x1334 m_vehicleVelocity
```
</details>
<details>
<summary><code>class DT_PlayerWaypoint</code></summary>

```
{
	moveparent: Int,
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
	m_parentAttachmentIndex: Int,
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
DT_PlayerWaypoint!0x0044 m_cellX
DT_PlayerWaypoint!0x0048 m_cellY
DT_PlayerWaypoint!0x004c m_cellZ
DT_PlayerWaypoint!0x0050 m_localOrigin
DT_PlayerWaypoint!0x0394 m_networkedFlags
DT_PlayerWaypoint!0x03e8 m_visibilityFlags
DT_PlayerWaypoint!0x03f0 m_iTeamNum
DT_PlayerWaypoint!0x03f4 m_teamMemberIndex
DT_PlayerWaypoint!0x0438 m_hOwnerEntity
DT_PlayerWaypoint!0x0508 m_iSignifierName
DT_PlayerWaypoint!0x07e4 m_parentAttachmentIndex
DT_PlayerWaypoint!0x07ec m_parentAttachmentModel
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
	m_cellX: Int,
	m_cellY: Int,
	m_cellZ: Int,
	m_localOrigin: VectorXY,
	m_localOrigin.z: Float,
}
```

### Offsets

```
DT_PortalNonLocalPlayerExclusive!0x0044 m_cellX
DT_PortalNonLocalPlayerExclusive!0x0048 m_cellY
DT_PortalNonLocalPlayerExclusive!0x004c m_cellZ
DT_PortalNonLocalPlayerExclusive!0x0050 m_localOrigin
DT_PortalNonLocalPlayerExclusive!0x0058 m_localOrigin.z
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
DT_Projectile!0x0044 m_cellX
DT_Projectile!0x0048 m_cellY
DT_Projectile!0x004c m_cellZ
DT_Projectile!0x0050 m_localOrigin
DT_Projectile!0x005c m_nModelIndex
DT_Projectile!0x0394 m_networkedFlags
DT_Projectile!0x03f0 m_iTeamNum
DT_Projectile!0x0418 m_vecVelocity
DT_Projectile!0x0424 m_localAngles
DT_Projectile!0x0438 m_hOwnerEntity
DT_Projectile!0x04c8 m_CollisionGroup
DT_Projectile!0x0754 m_PredictableID
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
	m_parentAttachmentIndex: Int,
	m_parentAttachmentModel: Int,
	m_fadeDist: Float,
	m_usablePriority: Int,
	m_usableDistanceOverride: Float,
	m_usableFOV: Float,
	m_usePromptSize: Float,
	m_realmsBitMask: BitMask,
	m_nSkin: Int,
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
DT_PropSurvival!0x003c m_fEffects
DT_PropSurvival!0x0040 m_usableType
DT_PropSurvival!0x0044 m_cellX
DT_PropSurvival!0x0048 m_cellY
DT_PropSurvival!0x004c m_cellZ
DT_PropSurvival!0x0050 m_localOrigin
DT_PropSurvival!0x005c m_nModelIndex
DT_PropSurvival!0x0394 m_networkedFlags
DT_PropSurvival!0x03e8 m_visibilityFlags
DT_PropSurvival!0x0424 m_localAngles
DT_PropSurvival!0x0450 m_Collision
DT_PropSurvival!0x04c8 m_CollisionGroup
DT_PropSurvival!0x0508 m_iSignifierName
DT_PropSurvival!0x07e4 m_parentAttachmentIndex
DT_PropSurvival!0x07ec m_parentAttachmentModel
DT_PropSurvival!0x07f8 m_fadeDist
DT_PropSurvival!0x08c8 m_usablePriority
DT_PropSurvival!0x08cc m_usableDistanceOverride
DT_PropSurvival!0x08d0 m_usableFOV
DT_PropSurvival!0x08d4 m_usePromptSize
DT_PropSurvival!0x09e8 m_realmsBitMask
DT_PropSurvival!0x0e48 m_nSkin
DT_PropSurvival!0x0e4c m_nBody
DT_PropSurvival!0x0e50 m_camoIndex
DT_PropSurvival!0x1304 m_ammoInClip
DT_PropSurvival!0x1308 m_customScriptInt
DT_PropSurvival!0x130c m_survivalProperty
DT_PropSurvival!0x1310 m_weaponNameIndex
DT_PropSurvival!0x1314 m_modBitField
```
</details>
<details>
<summary><code>class DT_Rodeo_PlayerData</code></summary>

```
{
	stage: Int,
	canRodeo: Int,
	rodeoCountParity: Int,
	startTime: Time,
	endTime: Time,
	targetEnt: Int,
	prevEnt: Int,
	prevEntCooldown: Time,
	pilot1pSequenceIndex: Int,
	pilot3pSequenceIndex: Int,
	targetAttachmentIndex: Int,
}
```

### Offsets

```
DT_Rodeo_PlayerData!0x0008 stage
DT_Rodeo_PlayerData!0x000c canRodeo
DT_Rodeo_PlayerData!0x0010 rodeoCountParity
DT_Rodeo_PlayerData!0x0014 startTime
DT_Rodeo_PlayerData!0x0018 endTime
DT_Rodeo_PlayerData!0x001c targetEnt
DT_Rodeo_PlayerData!0x0020 prevEnt
DT_Rodeo_PlayerData!0x0024 prevEntCooldown
DT_Rodeo_PlayerData!0x0028 pilot1pSequenceIndex
DT_Rodeo_PlayerData!0x002c pilot3pSequenceIndex
DT_Rodeo_PlayerData!0x0030 targetAttachmentIndex
```
</details>
<details>
<summary><code>class DT_RopeKeyframe</code></summary>

```
{
	m_localOrigin: Vector,
	moveparent: Int,
	m_parentAttachmentType: Int,
	m_visibilityFlags: Int,
	m_hOwnerEntity: Int,
	m_parentAttachmentIndex: Int,
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
DT_RopeKeyframe!0x03e8 m_visibilityFlags
DT_RopeKeyframe!0x0438 m_hOwnerEntity
DT_RopeKeyframe!0x07e4 m_parentAttachmentIndex
DT_RopeKeyframe!0x07e8 m_parentAttachmentHitbox
DT_RopeKeyframe!0x07ec m_parentAttachmentModel
DT_RopeKeyframe!0x07f8 m_fadeDist
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
DT_RopeKeyframe!0x0ed0 m_bConstrainBetweenEndpoints
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
DT_ScriptMover!0x0108 m_vecAngVelocity
DT_ScriptMover!0x0418 m_vecVelocity
DT_ScriptMover!0x0424 m_localAngles
DT_ScriptMover!0x07e8 m_parentAttachmentHitbox
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
	m_moveToWaypointStartTime: Time,
	m_moveToWaypointStartPos: Vector,
	m_moveToWaypointEnd: Int,
	m_moveToWaypointSpeed: Float,
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
DT_ScriptMoverLightweight!0x003c m_fEffects
DT_ScriptMoverLightweight!0x0044 m_moverNetworkCellX
DT_ScriptMoverLightweight!0x0048 m_moverNetworkCellY
DT_ScriptMoverLightweight!0x004c m_moverNetworkCellZ
DT_ScriptMoverLightweight!0x0050 m_moverNetworkLocalOrigin
DT_ScriptMoverLightweight!0x005c m_nModelIndex
DT_ScriptMoverLightweight!0x0108 m_moverNetworkAngularVelocity
DT_ScriptMoverLightweight!0x0394 m_networkedFlags
DT_ScriptMoverLightweight!0x0418 m_moverNetworkLinearVelocity
DT_ScriptMoverLightweight!0x0424 m_moverNetworkLocalAngles
DT_ScriptMoverLightweight!0x07e4 m_parentAttachmentIndex
DT_ScriptMoverLightweight!0x07e8 m_parentAttachmentHitbox
DT_ScriptMoverLightweight!0x07ec m_parentAttachmentModel
DT_ScriptMoverLightweight!0x07f8 m_fadeDist
DT_ScriptMoverLightweight!0x1484 m_moveModeNonPhysics
DT_ScriptMoverLightweight!0x1488 m_moveModeIsLocal
DT_ScriptMoverLightweight!0x148c m_moveToStartPos
DT_ScriptMoverLightweight!0x1498 m_moveToEndPos
DT_ScriptMoverLightweight!0x14a4 m_moveToTimeStart
DT_ScriptMoverLightweight!0x14a8 m_moveToTimeEnd
DT_ScriptMoverLightweight!0x14ac m_moveToTimeEaseIn
DT_ScriptMoverLightweight!0x14b0 m_moveToTimeEaseOut
DT_ScriptMoverLightweight!0x14b4 m_moveVelocity
DT_ScriptMoverLightweight!0x14c0 m_moveGravity
DT_ScriptMoverLightweight!0x14cc m_moveToWaypointStartTime
DT_ScriptMoverLightweight!0x14d0 m_moveToWaypointStartPos
DT_ScriptMoverLightweight!0x14dc m_moveToWaypointEnd
DT_ScriptMoverLightweight!0x14e0 m_moveToWaypointSpeed
DT_ScriptMoverLightweight!0x14e4 m_rotateModeNonPhysics
DT_ScriptMoverLightweight!0x14e8 m_rotateModeIsLocal
DT_ScriptMoverLightweight!0x14ec m_RotateToAnglesStart
DT_ScriptMoverLightweight!0x14f8 m_RotateToAnglesEnd
DT_ScriptMoverLightweight!0x1504 m_rotateToTimeStart
DT_ScriptMoverLightweight!0x1508 m_rotateToTimeEnd
DT_ScriptMoverLightweight!0x150c m_rotateToTimeEaseIn
DT_ScriptMoverLightweight!0x1510 m_rotateToTimeEaseOut
DT_ScriptMoverLightweight!0x1514 m_rotateAxis
DT_ScriptMoverLightweight!0x1520 m_rotateSpeed
DT_ScriptMoverLightweight!0x1541 m_useNonPhysicsMoveInterpolation
```
</details>
<details>
<summary><code>class DT_ScriptMoverWaypoint</code></summary>

```
{
	m_cellX: Int,
	m_cellY: Int,
	m_cellZ: Int,
	m_localOrigin: Vector,
	m_scriptNameIndex: Int,
}
```

### Offsets

```
DT_ScriptMoverWaypoint!0x0044 m_cellX
DT_ScriptMoverWaypoint!0x0048 m_cellY
DT_ScriptMoverWaypoint!0x004c m_cellZ
DT_ScriptMoverWaypoint!0x0050 m_localOrigin
DT_ScriptMoverWaypoint!0x0618 m_scriptNameIndex
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
DT_ScriptNetData_SNDC_GLOBAL!0x0cf0 m_entities[0]
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
DT_ScriptNetData_SNDC_PLAYER_GLOBAL!0x0ca8 m_times[0]
DT_ScriptNetData_SNDC_PLAYER_GLOBAL!0x0cc8 m_entities[0]
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
DT_ScriptProp!0x0500 m_iMaxHealth
DT_ScriptProp!0x0908 m_minimapData
DT_ScriptProp!0x0958 m_nameVisibilityFlags
DT_ScriptProp!0x13a0 m_title
DT_ScriptProp!0x13c0 m_footstepType
DT_ScriptProp!0x1400 m_renderColorFriendlyIsValid
DT_ScriptProp!0x1401 m_renderColorFriendly
DT_ScriptProp!0x1408 m_armorType
DT_ScriptProp!0x140c m_scriptPropFlags
DT_ScriptProp!0x1410 m_scriptPropSmartAmmoLockType
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
<summary><code>class DT_SmartAmmo_WeaponData</code></summary>

```
{
	numTargetEntities: Int,
	fractions: DataTable,
	targetEntities: DataTable,
	visiblePoints: DataTable,
	lastVisibleTimes: DataTable,
	lastFullLockTimes: DataTable,
	storedTargets: DataTable,
	lastNewTargetTime: Time,
	trackerCount: Int,
	trackerEntities: DataTable,
	trackerLocks: DataTable,
	trackerTimes: DataTable,
}
```

### Offsets

```
DT_SmartAmmo_WeaponData!0x0008 numTargetEntities
DT_SmartAmmo_WeaponData!0x0008 fractions
DT_SmartAmmo_WeaponData!0x000c targetEntities
DT_SmartAmmo_WeaponData!0x008c visiblePoints
DT_SmartAmmo_WeaponData!0x00ac lastVisibleTimes
DT_SmartAmmo_WeaponData!0x00cc lastFullLockTimes
DT_SmartAmmo_WeaponData!0x00ec storedTargets
DT_SmartAmmo_WeaponData!0x010c lastNewTargetTime
DT_SmartAmmo_WeaponData!0x0110 trackerCount
DT_SmartAmmo_WeaponData!0x0114 trackerEntities
DT_SmartAmmo_WeaponData!0x0134 trackerLocks
DT_SmartAmmo_WeaponData!0x0154 trackerTimes
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
<summary><code>class DT_Sprite extends DT_BaseEntity</code></summary>

```
{
	m_iTeamNum: Int,
	m_hAttachedToEntity: Int,
	m_nAttachment: Int,
	m_flSpriteFramerate: Float,
	m_flFrame: Float,
	m_clrRenderFriendly: Int,
	m_nBrightness: Int,
	m_flBrightnessDuration: Float,
	m_flSpriteScale: Float,
	m_flScaleDuration: Float,
	m_bWorldSpaceScale: Int,
	m_flGlowProxySize: Float,
	m_flHDRColorScale: Float,
}
```

### Offsets

```
DT_Sprite!0x03f0 m_iTeamNum
DT_Sprite!0x0a18 m_hAttachedToEntity
DT_Sprite!0x0a1c m_nAttachment
DT_Sprite!0x0a20 m_flSpriteFramerate
DT_Sprite!0x0a24 m_flFrame
DT_Sprite!0x0a2c m_clrRenderFriendly
DT_Sprite!0x0a30 m_nBrightness
DT_Sprite!0x0a34 m_flBrightnessDuration
DT_Sprite!0x0a38 m_flSpriteScale
DT_Sprite!0x0a3c m_flScaleDuration
DT_Sprite!0x0a40 m_bWorldSpaceScale
DT_Sprite!0x0a44 m_flGlowProxySize
DT_Sprite!0x0a48 m_flHDRColorScale
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
DT_StatueProp!0x13c0 m_hInitBaseAnimating
DT_StatueProp!0x13c4 m_bShatter
DT_StatueProp!0x13c8 m_nShatterFlags
DT_StatueProp!0x13cc m_vShatterPosition
DT_StatueProp!0x13d8 m_vShatterForce
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
DT_StatusEffectPlugin!0x0438 m_hOwnerEntity
DT_StatusEffectPlugin!0x0a00 m_statusEffectsTimedPluginNV
DT_StatusEffectPlugin!0x0a18 m_statusEffectsEndlessPluginNV
```
</details>
<details>
<summary><code>class DT_TEBeamEntPoint extends DT_BaseBeam</code></summary>

```
{
	m_nStartEntity: Int,
	m_nEndEntity: Int,
	m_vecStartPoint: Vector,
	m_vecEndPoint: Vector,
}
```

### Offsets

```
DT_TEBeamEntPoint!0x0068 m_nStartEntity
DT_TEBeamEntPoint!0x006c m_nEndEntity
DT_TEBeamEntPoint!0x0070 m_vecStartPoint
DT_TEBeamEntPoint!0x007c m_vecEndPoint
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
	m_thirdPersonEntBlendTotalDuration: Float,
	m_thirdPersonEntBlendEaseInDuration: Float,
	m_thirdPersonEntBlendEaseOutDuration: Float,
	m_thirdPersonEntFixedPitch: Float,
	m_thirdPersonEntFixedYaw: Float,
	m_thirdPersonEntFixedDist: Float,
	m_thirdPersonEntFixedHeight: Float,
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
DT_ThirdPersonView!0x0014 m_thirdPersonEntBlendTotalDuration
DT_ThirdPersonView!0x0018 m_thirdPersonEntBlendEaseInDuration
DT_ThirdPersonView!0x001c m_thirdPersonEntBlendEaseOutDuration
DT_ThirdPersonView!0x0020 m_thirdPersonEntFixedPitch
DT_ThirdPersonView!0x0024 m_thirdPersonEntFixedYaw
DT_ThirdPersonView!0x0028 m_thirdPersonEntFixedDist
DT_ThirdPersonView!0x002c m_thirdPersonEntFixedHeight
DT_ThirdPersonView!0x0040 m_thirdPersonEntMinYaw
DT_ThirdPersonView!0x0044 m_thirdPersonEntMaxYaw
DT_ThirdPersonView!0x0048 m_thirdPersonEntMinPitch
DT_ThirdPersonView!0x004c m_thirdPersonEntMaxPitch
DT_ThirdPersonView!0x0050 m_thirdPersonEntSpringToCenterRate
DT_ThirdPersonView!0x0054 m_thirdPersonEntLookaheadLowerEntSpeed
DT_ThirdPersonView!0x0058 m_thirdPersonEntLookaheadUpperEntSpeed
DT_ThirdPersonView!0x005c m_thirdPersonEntLookaheadMaxAngle
DT_ThirdPersonView!0x0060 m_thirdPersonEntLookaheadLerpAheadRate
DT_ThirdPersonView!0x0064 m_thirdPersonEntLookaheadLerpToCenterRate
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
DT_TitanSoul!0x0104 m_bossPlayer
DT_TitanSoul!0x0150 m_shieldHealth
DT_TitanSoul!0x0154 m_shieldHealthMax
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
<summary><code>class DT_Turret extends DT_BaseCombatCharacter</code></summary>

```
{
	m_iHealth: Int,
	m_iMaxHealth: Int,
	m_inventory: DT_WeaponInventory,
	m_settingsIndex: Int,
	m_driver: Int,
	m_forceAimPitch: Float,
	m_forceAimYaw: Float,
	m_driverTimeStart: Time,
	m_title: String,
}
```

### Offsets

```
DT_Turret!0x03e0 m_iHealth
DT_Turret!0x0500 m_iMaxHealth
DT_Turret!0x1630 m_inventory
DT_Turret!0x17c8 m_settingsIndex
DT_Turret!0x17e0 m_driver
DT_Turret!0x17f0 m_forceAimPitch
DT_Turret!0x17f4 m_forceAimYaw
DT_Turret!0x17f8 m_driverTimeStart
DT_Turret!0x17fc m_title
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
DT_VehicleNonDriverExclusive!0x0044 m_cellX
DT_VehicleNonDriverExclusive!0x0048 m_cellY
DT_VehicleNonDriverExclusive!0x004c m_cellZ
DT_VehicleNonDriverExclusive!0x0050 m_localOrigin
DT_VehicleNonDriverExclusive!0x0058 m_localOrigin.z
```
</details>
<details>
<summary><code>class DT_VortexSphere extends DT_BaseEntity</code></summary>

```
{
	m_iHealth: Int,
	m_spawnflags: Int,
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
DT_VortexSphere!0x03e0 m_iHealth
DT_VortexSphere!0x079c m_spawnflags
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
DT_WeaponInventory!0x0060 activeWeapons
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
	m_smartAmmoEnable: Int,
	m_smartAmmo: DT_SmartAmmo_WeaponData,
	m_needsReloadCheck: Int,
	m_needsEmptyCycleCheck: Int,
	m_skinOverride: Int,
	m_skinOverrideIsValid: Int,
	m_chargeStartTime: Time,
	m_chargeEndTime: Time,
	m_lastChargeFrac: Float,
	m_sustainedDischargeEndTime: Time,
	m_sustainedDischargeIsInPrimaryAttack: Int,
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
}
```

### Offsets

```
DT_WeaponX!0x0000 LocalWeaponData
DT_WeaponX!0x0000 predictingClientOnly
DT_WeaponX!0x0394 m_networkedFlags
DT_WeaponX!0x0fe8 m_bClientSideAnimation
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
DT_WeaponX!0x1438 m_smartAmmoEnable
DT_WeaponX!0x1440 m_smartAmmo
DT_WeaponX!0x1630 m_needsReloadCheck
DT_WeaponX!0x1631 m_needsEmptyCycleCheck
DT_WeaponX!0x1634 m_skinOverride
DT_WeaponX!0x1638 m_skinOverrideIsValid
DT_WeaponX!0x163c m_chargeStartTime
DT_WeaponX!0x1640 m_chargeEndTime
DT_WeaponX!0x1644 m_lastChargeFrac
DT_WeaponX!0x1668 m_sustainedDischargeEndTime
DT_WeaponX!0x166c m_sustainedDischargeIsInPrimaryAttack
DT_WeaponX!0x1670 m_modBitfieldFromPlayer
DT_WeaponX!0x1674 m_modBitfieldInternal
DT_WeaponX!0x1678 m_modBitfieldCurrent
DT_WeaponX!0x167c m_curSharedEnergyCost
DT_WeaponX!0x1680 m_grappleWeaponNeedsDryfire
DT_WeaponX!0x1681 m_scriptActivated
DT_WeaponX!0x1682 m_isLoadoutPickup
DT_WeaponX!0x1684 m_utilityEnt
DT_WeaponX!0x168c m_weaponNameIndex
DT_WeaponX!0x1698 m_shouldPlayIdleAnims
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
DT_WeaponX_LocalWeaponData!0x04fc m_nNextThinkTick
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
DT_WeaponX_PredictingClientOnly!0x1648 m_lastRegenTime
DT_WeaponX_PredictingClientOnly!0x164c m_cooldownEndTime
DT_WeaponX_PredictingClientOnly!0x1650 m_stockPileWasDraining
DT_WeaponX_PredictingClientOnly!0x1654 m_lastChargeLevel
DT_WeaponX_PredictingClientOnly!0x1658 m_chargeEnergyDepleteStepCounter
DT_WeaponX_PredictingClientOnly!0x165c m_burstFireCount
DT_WeaponX_PredictingClientOnly!0x1660 m_burstFireIndex
DT_WeaponX_PredictingClientOnly!0x1664 m_shotCount
DT_WeaponX_PredictingClientOnly!0x1690 m_animModelIndexPredictingClientOnly
DT_WeaponX_PredictingClientOnly!0x1694 m_animSequencePredictingClientOnly
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
```
</details>
<details>
<summary><code>class DT_ZiplinePhysics</code></summary>

```
{
	m_isInit: Int,
	m_ziplineType: Int,
	m_ziplineStart: Vector,
	m_ziplineEnd: Vector,
	m_springDistance: Float,
	m_springDistanceScale: Float,
	m_remainingUnsimulatedTime: Float,
	m_outerZiplineEntity: Int,
	m_attachedEntities: DataTable,
	m_numAttachedEntities: Int,
	m_ziplineOwner: Int,
}
```

### Offsets

```
DT_ZiplinePhysics!0x0008 m_isInit
DT_ZiplinePhysics!0x000c m_ziplineType
DT_ZiplinePhysics!0x0010 m_ziplineStart
DT_ZiplinePhysics!0x001c m_ziplineEnd
DT_ZiplinePhysics!0x022c m_springDistance
DT_ZiplinePhysics!0x0230 m_springDistanceScale
DT_ZiplinePhysics!0x0234 m_remainingUnsimulatedTime
DT_ZiplinePhysics!0x0238 m_outerZiplineEntity
DT_ZiplinePhysics!0x0240 m_attachedEntities
DT_ZiplinePhysics!0x0340 m_numAttachedEntities
DT_ZiplinePhysics!0x0344 m_ziplineOwner
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
CBaseGrenade!0x0418 m_vecVelocity
CBaseGrenade!0x2741 m_doesExplode
CBaseGrenade!0x2744 m_DmgRadius
CBaseGrenade!0x2754 m_grenadeCreationTime
CBaseGrenade!0x2758 m_grenadeCreationOrigin
CBaseGrenade!0x27e8 m_flDamage
CBaseGrenade!0x27ec m_hThrower
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
CBaseViewModel!0x009c m_currentFrame.modelIndex
CBaseViewModel!0x00b8 m_currentFrame.animCycle
CBaseViewModel!0x0114 m_angAbsRotation
CBaseViewModel!0x012c m_vecAbsOrigin
CBaseViewModel!0x0138 m_localOrigin
CBaseViewModel!0x0144 m_localAngles
CBaseViewModel!0x03ec m_fEffects
CBaseViewModel!0x0424 m_angNetworkAngles
CBaseViewModel!0x0e4c m_nBody
CBaseViewModel!0x0e58 m_nResetEventsParity
CBaseViewModel!0x0ef8 m_bSequenceFinished
CBaseViewModel!0x0f10 m_currentFrameBaseAnimating.animStartTime
CBaseViewModel!0x0f14 m_currentFrameBaseAnimating.animStartCycle
CBaseViewModel!0x0f18 m_currentFrameBaseAnimating.animPlaybackRate
CBaseViewModel!0x0f20 m_currentFrameBaseAnimating.animModelIndex
CBaseViewModel!0x0f24 m_currentFrameBaseAnimating.animSequence
CBaseViewModel!0x0f28 m_currentFrameBaseAnimating.animSequenceParity
CBaseViewModel!0x0f2c m_currentFrameBaseAnimating.m_flPoseParameters
CBaseViewModel!0x1404 m_currentFrameAnimatingOverlay.animOverlayIsActive
CBaseViewModel!0x140c m_currentFrameAnimatingOverlay.animOverlayStartTime
CBaseViewModel!0x142c m_currentFrameAnimatingOverlay.animOverlayStartCycle
CBaseViewModel!0x144c m_currentFrameAnimatingOverlay.animOverlayPlaybackRate
CBaseViewModel!0x146c m_currentFrameAnimatingOverlay.animOverlayModelIndex
CBaseViewModel!0x148c m_currentFrameAnimatingOverlay.animOverlaySequence
CBaseViewModel!0x14ac m_currentFrameAnimatingOverlay.animOverlayWeight
CBaseViewModel!0x14ec m_currentFrameAnimatingOverlay.animOverlayAnimTime
CBaseViewModel!0x150c m_currentFrameAnimatingOverlay.animOverlayFadeInDuration
CBaseViewModel!0x152c m_currentFrameAnimatingOverlay.animOverlayFadeOutDuration
CBaseViewModel!0x154c m_currentFrameAnimatingOverlay.animOverlayCycle
CBaseViewModel!0x1658 m_viewModelOwner
CBaseViewModel!0x165c m_projectileIsVisible
CBaseViewModel!0x1a40 m_bBlockEventLayer
CBaseViewModel!0x1a41 m_isAdsTransition
CBaseViewModel!0x1a44 m_hWeapon
CBaseViewModel!0x1a48 m_tracerAttachments
CBaseViewModel!0x1a48 m_tracerAttachments
CBaseViewModel!0x1a50 m_tracerAttachmentsScoped
CBaseViewModel!0x1a50 m_tracerAttachmentsScoped
```
</details>
<details>
<summary><code>class CBeam extends C_BaseEntity</code></summary>

```
{
	m_clrRender: Int,
	m_currentFrame.modelIndex: Int,
	m_localOrigin: Vector,
	m_nRenderFX: Int,
	m_nRenderMode: Int,
	m_flFrameRate: Float,
	m_nNumBeamEnts: Int,
	m_nHaloIndex: Int,
	m_nBeamType: Int,
	m_hAttachEntity: EHANDLE,
	m_nAttachIndex: Int,
	m_fWidth: Float,
	m_fEndWidth: Float,
	m_fFadeLength: Float,
	m_fHaloScale: Float,
	m_fAmplitude: Float,
	m_fStartFrame: Float,
	m_fSpeed: Float,
	m_flFrame: Float,
	m_vecEndPos: Vector,
}
```

### Offsets

```
CBeam!0x0048 m_clrRender
CBeam!0x009c m_currentFrame.modelIndex
CBeam!0x0138 m_localOrigin
CBeam!0x043d m_nRenderFX
CBeam!0x0449 m_nRenderMode
CBeam!0x0a00 m_flFrameRate
CBeam!0x0a14 m_nNumBeamEnts
CBeam!0x0a1c m_nHaloIndex
CBeam!0x0a20 m_nBeamType
CBeam!0x0a28 m_hAttachEntity
CBeam!0x0a50 m_nAttachIndex
CBeam!0x0a78 m_fWidth
CBeam!0x0a7c m_fEndWidth
CBeam!0x0a80 m_fFadeLength
CBeam!0x0a84 m_fHaloScale
CBeam!0x0a88 m_fAmplitude
CBeam!0x0a8c m_fStartFrame
CBeam!0x0a90 m_fSpeed
CBeam!0x0a94 m_flFrame
CBeam!0x0a9c m_vecEndPos
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
CGrappleHook!0x00f8 m_pMoveParent
CGrappleHook!0x0138 m_localOrigin
CGrappleHook!0x0144 m_localAngles
CGrappleHook!0x03e8 m_visibilityFlags
CGrappleHook!0x07e0 m_parentAttachmentType
CGrappleHook!0x07e4 m_parentAttachmentIndex
CGrappleHook!0x07e8 m_parentAttachmentHitbox
CGrappleHook!0x1300 m_grappleZipline
```
</details>
<details>
<summary><code>class CMovementSpeedMod extends CBaseEntity</code></summary>

```
{
	InputSpeedMod: Float,
}
```

### Offsets

```
CMovementSpeedMod!0x0000 InputSpeedMod
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
<summary><code>class CPointBroadcastClientCommand extends CBaseEntity</code></summary>

```
{
	InputCommand: String,
}
```

### Offsets

```
CPointBroadcastClientCommand!0x0000 InputCommand
```
</details>
<details>
<summary><code>class CPointClientCommand extends CBaseEntity</code></summary>

```
{
	InputCommand: String,
}
```

### Offsets

```
CPointClientCommand!0x0000 InputCommand
```
</details>
<details>
<summary><code>class CPointServerCommand extends CBaseEntity</code></summary>

```
{
	InputCommand: String,
}
```

### Offsets

```
CPointServerCommand!0x0000 InputCommand
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
}
```

### Offsets

```
CPredictedFirstPersonProxy!0x0138 m_localOrigin
CPredictedFirstPersonProxy!0x0144 m_localAngles
CPredictedFirstPersonProxy!0x0418 m_vecVelocity
CPredictedFirstPersonProxy!0x0424 m_angNetworkAngles
CPredictedFirstPersonProxy!0x0bc0 m_SequenceTransitioner
```
</details>
<details>
<summary><code>class CSprite extends C_BaseEntity</code></summary>

```
{
	m_hAttachedToEntity: EHANDLE,
	m_nAttachment: Int,
	m_flSpriteFramerate: Float,
	m_flFrame: Float,
	m_flDieTime: Float,
	m_nBrightness: Int,
	m_flBrightnessDuration: Float,
	m_flSpriteScale: Float,
	m_flScaleDuration: Float,
	m_flLastTime: Float,
	m_flMaxFrame: Float,
}
```

### Offsets

```
CSprite!0x0a18 m_hAttachedToEntity
CSprite!0x0a1c m_nAttachment
CSprite!0x0a20 m_flSpriteFramerate
CSprite!0x0a24 m_flFrame
CSprite!0x0a28 m_flDieTime
CSprite!0x0a30 m_nBrightness
CSprite!0x0a34 m_flBrightnessDuration
CSprite!0x0a38 m_flSpriteScale
CSprite!0x0a3c m_flScaleDuration
CSprite!0x0a4c m_flLastTime
CSprite!0x0a50 m_flMaxFrame
```
</details>
<details>
<summary><code>class CTurret extends C_BaseCombatCharacter</code></summary>

```
{
	m_aimAngle: Float,
}
```

### Offsets

```
CTurret!0x17e4 m_aimAngle
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
	m_smartAmmoEnable: Bool,
	m_smartAmmo: SmartAmmo_WeaponData,
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
	m_lastChargeLevel: Int,
	m_chargeEnergyDepleteStepCounter: Int,
	m_burstFireCount: Int,
	m_burstFireIndex: Int,
	m_shotCount: Int,
	m_sustainedDischargeEndTime: Time,
	m_sustainedDischargeIsInPrimaryAttack: Bool,
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
CWeaponX!0x0138 m_localOrigin
CWeaponX!0x04fc m_nNextThinkTick
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
CWeaponX!0x1438 m_smartAmmoEnable
CWeaponX!0x1440 m_smartAmmo
CWeaponX!0x1630 m_needsReloadCheck
CWeaponX!0x1631 m_needsEmptyCycleCheck
CWeaponX!0x1634 m_skinOverride
CWeaponX!0x1638 m_skinOverrideIsValid
CWeaponX!0x163c m_chargeStartTime
CWeaponX!0x1640 m_chargeEndTime
CWeaponX!0x1644 m_lastChargeFrac
CWeaponX!0x1648 m_lastRegenTime
CWeaponX!0x164c m_cooldownEndTime
CWeaponX!0x1650 m_stockPileWasDraining
CWeaponX!0x1654 m_lastChargeLevel
CWeaponX!0x1658 m_chargeEnergyDepleteStepCounter
CWeaponX!0x165c m_burstFireCount
CWeaponX!0x1660 m_burstFireIndex
CWeaponX!0x1664 m_shotCount
CWeaponX!0x1668 m_sustainedDischargeEndTime
CWeaponX!0x166c m_sustainedDischargeIsInPrimaryAttack
CWeaponX!0x1670 m_modBitfieldFromPlayer
CWeaponX!0x1674 m_modBitfieldInternal
CWeaponX!0x1678 m_modBitfieldCurrent
CWeaponX!0x167c m_curSharedEnergyCost
CWeaponX!0x1680 m_grappleWeaponNeedsDryfire
CWeaponX!0x1681 m_scriptActivated
CWeaponX!0x28b8 m_flNextEmptySoundTime
CWeaponX!0x28de m_bRemoveable
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
C_BaseAnimating!0x00b8 m_currentFrame.animCycle
C_BaseAnimating!0x0a28 m_animNetworkFlags
C_BaseAnimating!0x0a2c m_networkAnimActive
C_BaseAnimating!0x0a2e m_animActive
C_BaseAnimating!0x0a2f m_animCollisionEnabled
C_BaseAnimating!0x0a30 m_animPlantingEnabled
C_BaseAnimating!0x0b28 m_predictedAnimEventData
C_BaseAnimating!0x0bc0 m_SequenceTransitioner
C_BaseAnimating!0x0e48 m_nSkin
C_BaseAnimating!0x0e4c m_nBody
C_BaseAnimating!0x0e58 m_nResetEventsParity
C_BaseAnimating!0x0ef8 m_bSequenceFinished
C_BaseAnimating!0x0f00 m_bSequenceLooped
C_BaseAnimating!0x0f01 m_bSequenceLoops
C_BaseAnimating!0x0f04 m_flModelScale
C_BaseAnimating!0x0f10 m_currentFrameBaseAnimating.animStartTime
C_BaseAnimating!0x0f14 m_currentFrameBaseAnimating.animStartCycle
C_BaseAnimating!0x0f18 m_currentFrameBaseAnimating.animPlaybackRate
C_BaseAnimating!0x0f20 m_currentFrameBaseAnimating.animModelIndex
C_BaseAnimating!0x0f24 m_currentFrameBaseAnimating.animSequence
C_BaseAnimating!0x0f28 m_currentFrameBaseAnimating.animSequenceParity
C_BaseAnimating!0x0f2c m_currentFrameBaseAnimating.m_flPoseParameters
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
C_BaseAnimatingOverlay!0x13c8 m_AnimOverlayCount
C_BaseAnimatingOverlay!0x1404 m_currentFrameAnimatingOverlay.animOverlayIsActive
C_BaseAnimatingOverlay!0x140c m_currentFrameAnimatingOverlay.animOverlayStartTime
C_BaseAnimatingOverlay!0x142c m_currentFrameAnimatingOverlay.animOverlayStartCycle
C_BaseAnimatingOverlay!0x144c m_currentFrameAnimatingOverlay.animOverlayPlaybackRate
C_BaseAnimatingOverlay!0x146c m_currentFrameAnimatingOverlay.animOverlayModelIndex
C_BaseAnimatingOverlay!0x148c m_currentFrameAnimatingOverlay.animOverlaySequence
C_BaseAnimatingOverlay!0x14ac m_currentFrameAnimatingOverlay.animOverlayWeight
C_BaseAnimatingOverlay!0x14cc m_currentFrameAnimatingOverlay.animOverlayOrder
C_BaseAnimatingOverlay!0x14ec m_currentFrameAnimatingOverlay.animOverlayAnimTime
C_BaseAnimatingOverlay!0x150c m_currentFrameAnimatingOverlay.animOverlayFadeInDuration
C_BaseAnimatingOverlay!0x152c m_currentFrameAnimatingOverlay.animOverlayFadeOutDuration
C_BaseAnimatingOverlay!0x154c m_currentFrameAnimatingOverlay.animOverlayCycle
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
	m_latestNonOffhandWeapons: Char,
	m_selectedOffhands: Char,
	m_selectedOffhandsPendingHybridAction: Char,
	m_lastCycleSlot: Char,
	m_latestMeleeWeapon: EHANDLE,
	m_weaponPermission: Int,
	m_weaponDelayEnableTime: Time,
	m_weaponDisabledFlags: Char,
	m_hudInfo_visibilityTestAlwaysPasses: Bool,
	m_contextAction: Int,
	m_phaseShiftTimeStart: Time,
	m_phaseShiftTimeEnd: Time,
}
```

### Offsets

```
C_BaseCombatCharacter!0x00bc m_currentFrame.weaponGettingSwitchedOut
C_BaseCombatCharacter!0x00c4 m_currentFrame.showActiveWeapon3p
C_BaseCombatCharacter!0x040c m_deathVelocity
C_BaseCombatCharacter!0x0740 m_phaseShiftFlags
C_BaseCombatCharacter!0x1600 m_flNextAttack
C_BaseCombatCharacter!0x1604 m_lastFiredTime
C_BaseCombatCharacter!0x1608 m_lastFiredWeapon
C_BaseCombatCharacter!0x160c m_raiseFromMeleeEndTime
C_BaseCombatCharacter!0x1610 m_sharedEnergyCount
C_BaseCombatCharacter!0x1614 m_sharedEnergyTotal
C_BaseCombatCharacter!0x1618 m_sharedEnergyLockoutThreshold
C_BaseCombatCharacter!0x161c m_lastSharedEnergyRegenTime
C_BaseCombatCharacter!0x1620 m_sharedEnergyRegenRate
C_BaseCombatCharacter!0x1624 m_sharedEnergyRegenDelay
C_BaseCombatCharacter!0x1628 m_lastSharedEnergyTakeTime
C_BaseCombatCharacter!0x1630 m_inventory
C_BaseCombatCharacter!0x1680 m_selectedWeapons
C_BaseCombatCharacter!0x1684 m_latestPrimaryWeapons
C_BaseCombatCharacter!0x168c m_latestNonOffhandWeapons
C_BaseCombatCharacter!0x168e m_selectedOffhands
C_BaseCombatCharacter!0x1691 m_selectedOffhandsPendingHybridAction
C_BaseCombatCharacter!0x1694 m_lastCycleSlot
C_BaseCombatCharacter!0x1698 m_latestMeleeWeapon
C_BaseCombatCharacter!0x169c m_weaponPermission
C_BaseCombatCharacter!0x16a0 m_weaponDelayEnableTime
C_BaseCombatCharacter!0x16c5 m_weaponDisabledFlags
C_BaseCombatCharacter!0x16c6 m_hudInfo_visibilityTestAlwaysPasses
C_BaseCombatCharacter!0x16d8 m_contextAction
C_BaseCombatCharacter!0x1704 m_phaseShiftTimeStart
C_BaseCombatCharacter!0x1708 m_phaseShiftTimeEnd
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
	m_vecBaseVelocity: Vector,
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
C_BaseEntity!0x0050 m_iEFlags
C_BaseEntity!0x008c m_fFlags
C_BaseEntity!0x009c m_currentFrame.modelIndex
C_BaseEntity!0x00ac m_currentFrame.viewOffset
C_BaseEntity!0x0108 m_vecAngVelocity
C_BaseEntity!0x0114 m_angAbsRotation
C_BaseEntity!0x0120 m_vecAbsVelocity
C_BaseEntity!0x012c m_vecAbsOrigin
C_BaseEntity!0x0138 m_localOrigin
C_BaseEntity!0x0144 m_localAngles
C_BaseEntity!0x03c8 m_flGravity
C_BaseEntity!0x03cc m_flProxyRandomValue
C_BaseEntity!0x03d0 m_vecBaseVelocity
C_BaseEntity!0x03dc m_hGroundEntity
C_BaseEntity!0x03e4 m_flMaxspeed
C_BaseEntity!0x03e8 m_visibilityFlags
C_BaseEntity!0x03ec m_fEffects
C_BaseEntity!0x03f0 m_iTeamNum
C_BaseEntity!0x0400 m_passThroughFlags
C_BaseEntity!0x0404 m_passThroughThickness
C_BaseEntity!0x0408 m_passThroughDirection
C_BaseEntity!0x040c m_deathVelocity
C_BaseEntity!0x0418 m_vecVelocity
C_BaseEntity!0x0424 m_angNetworkAngles
C_BaseEntity!0x0430 m_flFriction
C_BaseEntity!0x0438 m_hOwnerEntity
C_BaseEntity!0x043c m_bRenderWithViewModels
C_BaseEntity!0x043d m_nRenderFX
C_BaseEntity!0x0449 m_nRenderMode
C_BaseEntity!0x044a m_MoveType
C_BaseEntity!0x044b m_MoveCollide
C_BaseEntity!0x0450 m_Collision
```
</details>
<details>
<summary><code>class C_BaseEntity</code></summary>

```
{
	m_fFlags: Int,
	m_angAbsRotation: Vector,
	m_vecAbsOrigin: PositionVector,
	m_vecPrevAbsOrigin: PositionVector,
	m_flGravity: Float,
	m_ModelName: String,
	m_rgflCoordinateFrame: Float,
}
```

### Offsets

```
C_BaseEntity!0x008c m_fFlags
C_BaseEntity!0x0114 m_angAbsRotation
C_BaseEntity!0x012c m_vecAbsOrigin
C_BaseEntity!0x03bc m_vecPrevAbsOrigin
C_BaseEntity!0x03c8 m_flGravity
C_BaseEntity!0x0818 m_ModelName
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
C_ClientRagdoll!0x0048 m_clrRender
C_ClientRagdoll!0x043d m_nRenderFX
C_ClientRagdoll!0x0449 m_nRenderMode
C_ClientRagdoll!0x0b88 m_pRagdoll
C_ClientRagdoll!0x0e48 m_nSkin
C_ClientRagdoll!0x0e4c m_nBody
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
C_CrossbowBolt!0x2740 m_bounceCount
C_CrossbowBolt!0x2744 m_maxBounceCount
C_CrossbowBolt!0x2748 m_doesGrow
C_CrossbowBolt!0x274c m_growStartSize
C_CrossbowBolt!0x2750 m_growStage1Tick
C_CrossbowBolt!0x2754 m_growStage1Size
C_CrossbowBolt!0x2758 m_growStage2Tick
C_CrossbowBolt!0x275c m_growStage2Size
C_CrossbowBolt!0x2760 m_growStageFinalTick
C_CrossbowBolt!0x2764 m_growStageFinalSize
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
C_DynamicProp!0x1340 m_bClientSide
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
C_Missile!0x2740 m_hasPlayedWhizby
C_Missile!0x2744 m_whizByStart
C_Missile!0x2750 m_whizBySoundName
C_Missile!0x2790 m_homingSpeed
C_Missile!0x2794 m_homingSpeedDodgingPlayer
C_Missile!0x2798 m_launchDir
C_Missile!0x27a4 m_hSpecificTarget
C_Missile!0x27a8 m_targetOffset
C_Missile!0x27b4 m_targetPosition
C_Missile!0x27c0 m_useTargetPosition
C_Missile!0x27c4 m_postIgnitionSpeed
C_Missile!0x27c8 m_flGracePeriodEndsAt
C_Missile!0x27cc m_pathSettingsInitialized
C_Missile!0x27cd m_expandContractMissile
C_Missile!0x27ce m_spiralMissile
C_Missile!0x27d0 m_spiralSettings
C_Missile!0x27f4 m_expandContractSettings
C_Missile!0x282c m_lastThinkTime
C_Missile!0x2830 m_explosionIgnoreEntity
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
C_NPC_SentryTurret!0x1984 m_killCount
C_NPC_SentryTurret!0x1988 m_titanKillCount
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
	m_vecBaseVelocity: Vector,
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
	m_currentFrameLocalPlayer.m_vecPunchBase_Angle: Vector,
	m_currentFrameLocalPlayer.m_vecPunchBase_AngleVel: Vector,
	m_currentFrameLocalPlayer.m_vecPunchWeapon_Angle: Vector,
	m_currentFrameLocalPlayer.m_vecPunchWeapon_AngleVel.x: Float,
	m_currentFrameLocalPlayer.m_vecPunchWeapon_AngleVel.y: Float,
	m_currentFrameLocalPlayer.m_vecPunchWeapon_AngleVel.z: Float,
	m_currentFrameLocalPlayer.m_localGravityRotation: Quaternion,
	pl: CPlayerState,
	m_rodeo: Rodeo_PlayerData,
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
	m_remoteTurret: EHANDLE,
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
	m_repeatedBoost: Bool,
	m_boostMeter: Float,
	m_jetpack: Bool,
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
}
```

### Offsets

```
C_Player!0x008c m_fFlags
C_Player!0x00f8 m_pMoveParent
C_Player!0x0120 m_vecAbsVelocity
C_Player!0x03d0 m_vecBaseVelocity
C_Player!0x03dc m_hGroundEntity
C_Player!0x03e4 m_flMaxspeed
C_Player!0x0418 m_vecVelocity
C_Player!0x0430 m_flFriction
C_Player!0x04fc m_nNextThinkTick
C_Player!0x0bc0 m_SequenceTransitioner
C_Player!0x17c1 m_bZooming
C_Player!0x17c4 m_zoomToggleOnStartTime
C_Player!0x17c8 m_zoomBaseFrac
C_Player!0x17cc m_zoomBaseTime
C_Player!0x17d0 m_zoomFullStartTime
C_Player!0x1850 m_lastUCmdSimulationTicks
C_Player!0x1854 m_lastUCmdSimulationRemainderTime
C_Player!0x1a10 m_Local
C_Player!0x1cf0 m_currentFramePlayer.timeBase
C_Player!0x1cf8 m_currentFramePlayer.statusEffectsTimedPlayerCUR
C_Player!0x1de8 m_currentFramePlayer.statusEffectsEndlessPlayerCUR
C_Player!0x1e88 m_currentFramePlayer.m_flHullHeight
C_Player!0x1e8c m_currentFramePlayer.m_traversalAnimProgress
C_Player!0x1e90 m_currentFramePlayer.m_sprintTiltFrac
C_Player!0x1ea0 m_currentFramePlayer.m_ammoPoolCount
C_Player!0x2070 m_currentFrameLocalPlayer.m_stepSmoothingOffset
C_Player!0x207c m_currentFrameLocalPlayer.m_vecPunchBase_Angle
C_Player!0x2088 m_currentFrameLocalPlayer.m_vecPunchBase_AngleVel
C_Player!0x2094 m_currentFrameLocalPlayer.m_vecPunchWeapon_Angle
C_Player!0x20a0 m_currentFrameLocalPlayer.m_vecPunchWeapon_AngleVel.x
C_Player!0x20a4 m_currentFrameLocalPlayer.m_vecPunchWeapon_AngleVel.y
C_Player!0x20a8 m_currentFrameLocalPlayer.m_vecPunchWeapon_AngleVel.z
C_Player!0x20ac m_currentFrameLocalPlayer.m_localGravityRotation
C_Player!0x20c8 pl
C_Player!0x2148 m_rodeo
C_Player!0x223c m_ammoPoolCapacity
C_Player!0x259c m_gestureSequences
C_Player!0x25b4 m_gestureStartTimes
C_Player!0x25cc m_gestureBlendInDuration
C_Player!0x25e4 m_gestureBlendOutDuration
C_Player!0x25fc m_gestureFadeOutStartTime
C_Player!0x2614 m_gestureFadeOutDuration
C_Player!0x262c m_gestureAutoKillBitfield
C_Player!0x2648 m_afButtonLast
C_Player!0x264c m_afButtonPressed
C_Player!0x2650 m_afButtonReleased
C_Player!0x2654 m_nButtons
C_Player!0x2658 m_nImpulse
C_Player!0x265c m_flPhysics
C_Player!0x2660 m_flStepSoundTime
C_Player!0x2664 m_flTimeAllSuitDevicesOff
C_Player!0x2668 m_fStickySprintMinTime
C_Player!0x266c m_bPlayedSprintStartEffects
C_Player!0x2674 m_fIsSprinting
C_Player!0x2675 m_fIsWalking
C_Player!0x2678 m_sprintStartedTime
C_Player!0x267c m_sprintStartedFrac
C_Player!0x2680 m_sprintEndedTime
C_Player!0x2684 m_sprintEndedFrac
C_Player!0x2688 m_stickySprintStartTime
C_Player!0x268c m_damageImpulseNoDecelEndTime
C_Player!0x26a0 m_duckState
C_Player!0x26a4 m_leanState
C_Player!0x26a8 m_doingHalfDuck
C_Player!0x26a9 m_canStand
C_Player!0x26ac m_StandHullMin
C_Player!0x26b8 m_StandHullMax
C_Player!0x26c4 m_DuckHullMin
C_Player!0x26d0 m_DuckHullMax
C_Player!0x26e0 m_upDir
C_Player!0x26ec m_upDirPredicted
C_Player!0x26f8 m_lastWallRunStartPos
C_Player!0x2704 m_wallRunCount
C_Player!0x2708 m_wallRunWeak
C_Player!0x2709 m_shouldBeOneHanded
C_Player!0x270c m_oneHandFraction
C_Player!0x2710 m_animAimPitch
C_Player!0x2714 m_animAimYaw
C_Player!0x2718 m_wallRunPushAwayTime
C_Player!0x2724 m_wallrunRetryTime
C_Player!0x2728 m_wallrunRetryPos
C_Player!0x2734 m_wallrunRetryNormal
C_Player!0x2758 m_wallHangTime
C_Player!0x275c m_traversalState
C_Player!0x2760 m_traversalType
C_Player!0x2764 m_traversalBegin
C_Player!0x2770 m_traversalMid
C_Player!0x277c m_traversalEnd
C_Player!0x2788 m_traversalMidFrac
C_Player!0x278c m_traversalForwardDir
C_Player!0x2798 m_traversalRefPos
C_Player!0x27a4 m_traversalProgress
C_Player!0x27a8 m_traversalStartTime
C_Player!0x27ac m_traversalHandAppearTime
C_Player!0x27b0 m_traversalReleaseTime
C_Player!0x27b4 m_traversalBlendOutStartTime
C_Player!0x27b8 m_traversalBlendOutStartOffset
C_Player!0x27c4 m_traversalYawDelta
C_Player!0x27d0 m_wallDangleJumpOffTime
C_Player!0x27d4 m_wallDangleMayHangHere
C_Player!0x27d5 m_wallDangleForceFallOff
C_Player!0x27d6 m_wallDangleLastPushedForward
C_Player!0x27d8 m_wallDangleDisableWeapon
C_Player!0x27dc m_wallDangleClimbProgressFloor
C_Player!0x27e0 m_wallClimbSetUp
C_Player!0x27e1 m_wallHanging
C_Player!0x27e8 m_grapple
C_Player!0x27e8 m_grapple
C_Player!0x2878 m_grappleActive
C_Player!0x2878 m_grappleActive
C_Player!0x2879 m_grappleNeedWindowCheck
C_Player!0x287c m_grappleNextWindowHint
C_Player!0x288c m_slowMoEnabled
C_Player!0x288d m_sliding
C_Player!0x288e m_slideLongJumpAllowed
C_Player!0x2890 m_lastSlideTime
C_Player!0x2894 m_lastSlideBoost
C_Player!0x2898 m_gravityGrenadeStatusEffect
C_Player!0x289c m_bIsStickySprinting
C_Player!0x28a0 m_prevMoveYaw
C_Player!0x28a4 m_sprintTiltVel
C_Player!0x28c0 m_remoteTurret
C_Player!0x28c4 m_hViewModels
C_Player!0x28d8 m_viewOffsetEntity
C_Player!0x2a18 m_activeZipline
C_Player!0x2a1c m_lastZipline
C_Player!0x2a20 m_lastZiplineDetachTime
C_Player!0x2a24 m_ziplineValid3pWeaponLayerAnim
C_Player!0x2a28 m_ziplineState
C_Player!0x2a30 m_zipline
C_Player!0x2aa0 m_ziplineViewOffsetPosition
C_Player!0x2aac m_ziplineViewOffsetVelocity
C_Player!0x2ab8 m_ziplineGrenadeEntity
C_Player!0x2abc m_ziplineGrenadeBeginStationEntity
C_Player!0x2ac0 m_ziplineGrenadeBeginStationAttachmentIndex
C_Player!0x2acc m_playAnimationType
C_Player!0x2ad0 m_detachGrappleOnPlayAnimationEnd
C_Player!0x2ad4 m_playAnimationNext
C_Player!0x2ae8 m_boosting
C_Player!0x2ae9 m_repeatedBoost
C_Player!0x2aec m_boostMeter
C_Player!0x2af0 m_jetpack
C_Player!0x2af1 m_jetpackAfterburner
C_Player!0x2af2 m_gliding
C_Player!0x2af4 m_glideMeter
C_Player!0x2af8 m_glideRechargeDelayAccumulator
C_Player!0x2afc m_hovering
C_Player!0x2afd m_isPerformingBoostAction
C_Player!0x2b00 m_lastJumpHeight
C_Player!0x2b50 m_slipAirRestrictDirection
C_Player!0x2b5c m_slipAirRestrictTime
C_Player!0x2cf0 m_melee
C_Player!0x2d20 m_useCredit
C_Player!0x30f4 m_wallRunStartTime
C_Player!0x30f8 m_wallRunClearTime
C_Player!0x30fc m_onSlopeTime
C_Player!0x3100 m_lastWallNormal
C_Player!0x310c m_dodging
C_Player!0x3110 m_lastDodgeTime
C_Player!0x3114 m_vecPreviouslyPredictedOrigin
C_Player!0x312c m_flTimeLastTouchedWall
C_Player!0x3130 m_timeJetpackHeightActivateCheckPassed
C_Player!0x3134 m_flTimeLastTouchedGround
C_Player!0x3138 m_flTimeLastJumped
C_Player!0x313c m_flTimeLastLanded
C_Player!0x3140 m_flLastLandFromHeight
C_Player!0x3144 m_usePressedTime
C_Player!0x3148 m_lastUseTime
C_Player!0x3158 m_lastFakeFloorPos
C_Player!0x3164 m_bHasJumpedSinceTouchedGround
C_Player!0x3165 m_bDoMultiJumpPenalty
C_Player!0x3166 m_dodgingInAir
C_Player!0x3328 m_activeViewmodelModifiers
C_Player!0x3608 m_lastMoveInputTime
C_Player!0x360c m_ignoreEntityForMovementUntilNotTouching
C_Player!0x3ad8 m_gameMovementUtil.m_surfaceFriction
C_Player!0x3c8c m_lungeTargetEntity
C_Player!0x3c90 m_isLungingToPosition
C_Player!0x3c94 m_lungeTargetPosition
C_Player!0x3ca0 m_lungeStartPositionOffset
C_Player!0x3cac m_lungeEndPositionOffset
C_Player!0x3cb8 m_lungeStartTime
C_Player!0x3cbc m_lungeEndTime
C_Player!0x3cc0 m_lungeCanFly
C_Player!0x3cc1 m_lungeLockPitch
C_Player!0x3cc4 m_lungeStartPitch
C_Player!0x3cc8 m_lungeSmoothTime
C_Player!0x3ccc m_lungeMaxTime
C_Player!0x3cd0 m_lungeMaxEndSpeed
C_Player!0x4054 m_vPrevGroundNormal
C_Player!0x4224 m_pushAwayFromTopAcceleration
C_Player!0x424c m_controllerModeActive
C_Player!0x4270 m_skydiveForwardPoseValueVelocity
C_Player!0x4274 m_skydiveForwardPoseValueTarget
C_Player!0x4278 m_skydiveForwardPoseValueCurrent
C_Player!0x427c m_skydiveSidePoseValueVelocity
C_Player!0x4280 m_skydiveSidePoseValueTarget
C_Player!0x4284 m_skydiveSidePoseValueCurrent
C_Player!0x4288 m_skydiveYawVelocity
C_Player!0x42a4 m_freefallState
C_Player!0x42a8 m_freefallStartTime
C_Player!0x42ac m_freefallEndTime
C_Player!0x42b0 m_freefallAnticipateStartTime
C_Player!0x42b4 m_freefallAnticipateEndTime
C_Player!0x42b8 m_freefallDistanceToLand
C_Player!0x42bc m_skydiveDiveAngle
C_Player!0x42c0 m_skydiveIsDiving
C_Player!0x42c4 m_skydiveSpeed
C_Player!0x42c8 m_skydiveStrafeAngle
C_Player!0x42cc m_skydiveFreelookEnabled
C_Player!0x42d0 m_skydiveFreelookLockedAngle
C_Player!0x42dc m_skydivePlayerPitch
C_Player!0x42e0 m_skydivePlayerYaw
C_Player!0x42e4 m_skydiveFollowing
C_Player!0x42e8 m_skydiveUnfollowVelocity
C_Player!0x42f5 m_skydiveIsNearLeviathan
C_Player!0x42f8 m_skydiveLeviathanHitPosition
C_Player!0x4304 m_skydiveLeviathanHitNormal
C_Player!0x4310 m_skydiveSlipVelocity
C_Player!0x4320 m_playerKnockBacks
```
</details>
<details>
<summary><code>class C_PlayerLocalData</code></summary>

```
{
	m_nStepside: Int,
	m_nOldButtons: Int,
	m_iHideHUD: Int,
	m_nDuckTransitionTimeMsecs: Int,
	m_superJumpsUsed: Int,
	m_jumpedOffRodeo: Bool,
	m_jumpPressTime: Time,
	m_jetpackActivateTime: Time,
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
C_PlayerLocalData!0x0014 m_nDuckTransitionTimeMsecs
C_PlayerLocalData!0x0018 m_superJumpsUsed
C_PlayerLocalData!0x001c m_jumpedOffRodeo
C_PlayerLocalData!0x0020 m_jumpPressTime
C_PlayerLocalData!0x0024 m_jetpackActivateTime
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
C_PlayerLocalData!0x021c m_playerLocalGravityToWorldTransform
C_PlayerLocalData!0x024c m_playerLocalGravityBlendStartRotation
C_PlayerLocalData!0x025c m_playerLocalGravityBlendEndRotation
C_PlayerLocalData!0x026c m_playerLocalGravityBlendEndDirection
C_PlayerLocalData!0x0278 m_playerLocalGravityBlendStartTime
C_PlayerLocalData!0x027c m_playerLocalGravityBlendEndTime
C_PlayerLocalData!0x0280 m_playerLocalGravityBlendStrength
C_PlayerLocalData!0x0284 m_playerLocalGravityStrength
C_PlayerLocalData!0x0288 m_playerLocalGravityType
C_PlayerLocalData!0x028c m_playerLocalGravityPoint
C_PlayerLocalData!0x0298 m_playerLocalGravityLineStart
C_PlayerLocalData!0x02a4 m_playerLocalGravityLineEnd
C_PlayerLocalData!0x02b0 m_playerLocalGravityEntity
C_PlayerLocalData!0x02b4 m_playerLocalGravityLineStartEntity
C_PlayerLocalData!0x02b8 m_playerLocalGravityLineEndEntity
C_PlayerLocalData!0x02bc m_playerFloatLookStartTime
C_PlayerLocalData!0x02c0 m_playerFloatLookEndTime
C_PlayerLocalData!0x02c4 m_wallrunLatestFloorHeight
C_PlayerLocalData!0x02c8 m_groundNormal
C_PlayerLocalData!0x02d4 m_continuousUseBlocked
C_PlayerLocalData!0x02d8 m_useEnt
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
	m_vehicleLaunchTime: Float,
	m_vehicleVelocity: Vector,
}
```

### Offsets

```
C_PlayerVehicle!0x0138 m_localOrigin
C_PlayerVehicle!0x1304 m_vehicleDriver
C_PlayerVehicle!0x1320 m_vehicleActivated
C_PlayerVehicle!0x132c m_vehicleLaunchTime
C_PlayerVehicle!0x1334 m_vehicleVelocity
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
C_Projectile!0x23e0 m_modVarsAreValid
C_Projectile!0x23e4 m_launchOrigin
C_Projectile!0x23f0 m_scriptCB
C_Projectile!0x2418 m_hasPlayedTrailEffect
C_Projectile!0x241c m_projectileLifeTimeEndTick
C_Projectile!0x2420 m_projectileCreationTime
C_Projectile!0x2424 m_isVortexRefired
C_Projectile!0x2425 m_damageAliveOnly
C_Projectile!0x2426 m_usesPositionFunction
C_Projectile!0x2428 m_lastCollisionNormal
C_Projectile!0x2434 m_bounceIndex
C_Projectile!0x2438 m_randomInt
C_Projectile!0x243c m_thrownByAI
C_Projectile!0x2440 m_perPolyRadius
C_Projectile!0x2448 m_posBeforePhysicsSimulate
C_Projectile!0x2454 m_hasIgnited
C_Projectile!0x2455 m_inLagCompensation
C_Projectile!0x2458 m_passEntities
C_Projectile!0x24c0 m_projectileSpeed
C_Projectile!0x24c4 m_wantStartTrailEffect
C_Projectile!0x24c6 m_hasCalledPostDataUpdate
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
C_PropDoor!0x0138 m_localOrigin
C_PropDoor!0x0144 m_localAngles
C_PropDoor!0x04fc m_nNextThinkTick
C_PropDoor!0x13b4 m_angle
C_PropDoor!0x13b8 m_startAngle
C_PropDoor!0x13bc m_startAngleVel
C_PropDoor!0x13c0 m_startMoveTime
C_PropDoor!0x13cc m_nextHitSoundTime
C_PropDoor!0x13d0 m_lastThinkTime
C_PropDoor!0x1418 m_interactingPlayer
C_PropDoor!0x141c m_interactingPlayerWantsOpen
C_PropDoor!0x1420 m_useDebounceEndTime
C_PropDoor!0x1428 m_prevAngle
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
<summary><code>class PlayerMelee_PlayerData</code></summary>

```
{
	attackActive: Bool,
	attackRecoveryShouldBeQuick: Bool,
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
PlayerMelee_PlayerData!0x0008 attackActive
PlayerMelee_PlayerData!0x0009 attackRecoveryShouldBeQuick
PlayerMelee_PlayerData!0x000c attackStartTime
PlayerMelee_PlayerData!0x0010 attackHitEntity
PlayerMelee_PlayerData!0x0014 attackHitEntityTime
PlayerMelee_PlayerData!0x0018 attackLastHitNonWorldEntity
PlayerMelee_PlayerData!0x001c scriptedState
PlayerMelee_PlayerData!0x0020 pendingMeleePress
PlayerMelee_PlayerData!0x0024 lungeBoost
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
<summary><code>class Rodeo_PlayerData</code></summary>

```
{
	stage: Int,
	canRodeo: Bool,
	rodeoCountParity: Int,
	startTime: Time,
	endTime: Time,
	targetEnt: EHANDLE,
	prevEnt: EHANDLE,
	prevEntCooldown: Time,
	pilot1pSequenceIndex: Int,
	pilot3pSequenceIndex: Int,
	targetAttachmentIndex: Int,
}
```

### Offsets

```
Rodeo_PlayerData!0x0008 stage
Rodeo_PlayerData!0x000c canRodeo
Rodeo_PlayerData!0x0010 rodeoCountParity
Rodeo_PlayerData!0x0014 startTime
Rodeo_PlayerData!0x0018 endTime
Rodeo_PlayerData!0x001c targetEnt
Rodeo_PlayerData!0x0020 prevEnt
Rodeo_PlayerData!0x0024 prevEntCooldown
Rodeo_PlayerData!0x0028 pilot1pSequenceIndex
Rodeo_PlayerData!0x002c pilot3pSequenceIndex
Rodeo_PlayerData!0x0030 targetAttachmentIndex
```
</details>
<details>
<summary><code>class SmartAmmo_WeaponData</code></summary>

```
{
	numTargetEntities: Int,
	targetEntities: EHANDLE,
	currentFrameSmartAmmoFractions: Float,
	visiblePoints: Int,
	lastVisibleTimes: Time,
	lastFullLockTimes: Time,
	storedTargets: EHANDLE,
	lastNewTargetTime: Time,
	trackerCount: Int,
	trackerEntities: EHANDLE,
	trackerLocks: Int,
	trackerTimes: Time,
}
```

### Offsets

```
SmartAmmo_WeaponData!0x0008 numTargetEntities
SmartAmmo_WeaponData!0x000c targetEntities
SmartAmmo_WeaponData!0x002c currentFrameSmartAmmoFractions
SmartAmmo_WeaponData!0x008c visiblePoints
SmartAmmo_WeaponData!0x00ac lastVisibleTimes
SmartAmmo_WeaponData!0x00cc lastFullLockTimes
SmartAmmo_WeaponData!0x00ec storedTargets
SmartAmmo_WeaponData!0x010c lastNewTargetTime
SmartAmmo_WeaponData!0x0110 trackerCount
SmartAmmo_WeaponData!0x0114 trackerEntities
SmartAmmo_WeaponData!0x0134 trackerLocks
SmartAmmo_WeaponData!0x0154 trackerTimes
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
```
</details>

## ConVars

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
<summary><code>CTeam_DontSave</code></summary>



default: `"1"`  
flags: `0x2`  
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
<summary><code>When set to 0, player always returns false when asked if it has a vehicle</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>ai_ainRebuildOnMapStart</code></summary>

Values >= 1 means the game will build the .ain file when the map loads. Value 1 means this ConVar will reset to 0 after building. Value 2 means this ConVar will NOT reset after building

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_ain_crc_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_anim_overlay_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_auto_contact_solver</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_choose_new_enemy_max_time</code></summary>

The maximum amount of seconds that can pass before an AI must attempt to choose a new enemy

default: `"1.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_cluster_select</code></summary>



default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_collide_other_ai</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>ai_current_enemy_bonus</code></summary>

The AI's current enemy is given a bonus distance heuristic so that he is likely to pick them again

default: `"15"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_debug_corpse</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_debug_directnavprobe</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_debug_doors</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_debug_draw_depth_test</code></summary>

Toggle depth test for some AI debug draw functionality

default: `"1"`  
flags: `0x82`  
</details>
<details>
<summary><code>ai_debug_draw_nav_dist</code></summary>



default: `"1024"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_debug_dyninteractions</code></summary>

Debug the NPC dynamic interaction system.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_debug_efficiency</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_debug_enemies</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_debug_enemy_memory</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_debug_engagement_dist</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_debug_follow</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_debug_info_node_spectre</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_debug_los</code></summary>

NPC Line-Of-Sight debug mode. If 1, solid entities that block NPC LOC will be highlighted with white bounding boxes. If 2, it'll show non-solid entities that would do it if they were solid.

default: `"0"`  
flags: `0x4004`  
</details>
<details>
<summary><code>ai_debug_move_script</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_debug_move_transitions</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_debug_nodes</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_debug_obstacle_avoid</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_debug_pieoff</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_debug_poseparameters</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_debug_savePosition</code></summary>



default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_debug_search_paths</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_debug_shoot_positions</code></summary>



default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_debug_squads</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_debug_stats</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_debug_test_anim_path</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_debug_think_ticks</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_default_efficient</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_disable_task_announce_attack</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_draw_motor_movement</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_efficiency_override</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_enable_corpse_manager</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_excluded_clusters</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_follow_use_points</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_follow_use_points_when_moving</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_frametime_limit</code></summary>

frametime limit for min efficiency AIE_NORMAL (in sec's).

default: `"50"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_grenade_default_weapon</code></summary>



default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_grenade_enabled</code></summary>

Allow AI to throw grenades

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_grenade_forced_weapon</code></summary>



default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_grenade_fuse_time</code></summary>



default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_grenade_initial_contact_delay</code></summary>

When an AI first sees an enemy it must wait this long before possibly throwing a grenade at them

default: `"10"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_grenade_max_throw_speed</code></summary>



default: `"5000"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_grenade_target_debounce_default</code></summary>

Whenever a grenade is thrown at an entity, how long before another grenade can be thrown at them again

default: `"22"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_grenade_target_horizontal_offset</code></summary>



default: `"15"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_grenade_target_variance_dist_scalar</code></summary>



default: `"0.03"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_grenade_target_variance_min</code></summary>



default: `"8"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_grenade_throw_debounce</code></summary>

After an AI throws a grenade, how long until it can throw another one

default: `"15"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_local_step_size</code></summary>



default: `"1024"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_max_corpse_detect_dist</code></summary>



default: `"1024"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_max_look_at_friendly_dist</code></summary>



default: `"512"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_max_node_drop</code></summary>



default: `"2048"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_max_triangulation_attempts</code></summary>



default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_max_triangulation_dist</code></summary>



default: `"384"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_melee_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_melee_kill_sound_radius</code></summary>



default: `"250"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_min_signal_dist</code></summary>



default: `"512"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_missFastPlayer_sideWindowYMax</code></summary>



default: `"80"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_missFastPlayer_sideWindowYMin</code></summary>



default: `"30"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_missFastPlayer_sideWindowZMax</code></summary>



default: `"80"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_missFastPlayer_sideWindowZMin</code></summary>



default: `"-20"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_missFastPlayer_topWindowYMax</code></summary>



default: `"50"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_missFastPlayer_topWindowYMin</code></summary>



default: `"-50"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_missFastPlayer_topWindowZMax</code></summary>



default: `"40"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_missFastPlayer_topWindowZMin</code></summary>



default: `"20"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_move_do_short_probe</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_move_probe_delay</code></summary>



default: `"0.25"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_move_sanity_check</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_moveprobe_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_moveprobe_jump_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_near_node_for_hull_box_extent</code></summary>



default: `"800"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_no_local_ground_paths</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_no_local_paths</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_no_node_cache</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_no_select_box</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_no_steer</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_node_draw_safety</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_node_select</code></summary>



default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_pain_death_sound_radius</code></summary>



default: `"250"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_pain_on_repeat_damage_threshold</code></summary>



default: `"0.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_pain_on_repeated_damage</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_path_adjust_speed_on_immediate_turns</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_path_dangerous_cluster_cost_scalar</code></summary>

When cluster path finding the given cost of a cluster will be incremented by: its absolute danger time remaining multiplied by this value. Larger values = More avoidance of dangerous clusters

default: `"70"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_path_dangerous_cluster_death_time_inc</code></summary>

When an AI dies it increments danger expiration time on the cluster it is near by X. More deaths = longer time

default: `"35.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_path_dangerous_cluster_exclude_dist</code></summary>

Maximum distance threshold between AI's start cluster and candidate cluster X, for cluster X to use any danger related logic

default: `"2200"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_path_dangerous_cluster_look_ahead</code></summary>

How many clusters an AI should examine ahead in its current path to look for dangerous clusters. AI will attempt to avoid one if it is found.

default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_path_dangerous_cluster_min_time</code></summary>

The minimum amount of danger time left for a cluster to be considered dangerous

default: `"40"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_path_insert_pause_at_est_end</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_path_insert_pause_at_obstruction</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_physics_shadow</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_pos_debug</code></summary>



default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_radial_max_link_dist</code></summary>



default: `"512"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_range_attack_twitch_debounce</code></summary>



default: `"6"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_react_far_dist</code></summary>



default: `"2000"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_reasonable_facing_min_dist</code></summary>



default: `"120"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_rebalance_thinks</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_recent_enemy_damage_dist_bonus</code></summary>

AI will given distance bonus priority to enemies who have attacked them recently when they are picking a best enemy

default: `"1000"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_recent_enemy_damage_expire_time</code></summary>

AI will given distance bonus priority to enemies who have attacked them recently when they are picking a best enemy

default: `"3.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_require_pvs</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>ai_route_simplify_interval</code></summary>



default: `"0.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_run_from_enemy_try_shoot_chance</code></summary>

Chance of AI that needs to run away taking a few shots at enemy before running

default: `"60"`  
flags: `0x4000`  
</details>
<details>
<summary><code>ai_schedule_reset_conditions_on_gather</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_schedule_selector_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_script_assault_points_validation_debug</code></summary>



default: `"30"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_script_nodes_draw</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_shot_bias</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_shot_stats_term</code></summary>



default: `"100"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_show_hull_attacks</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_show_path_search_nodes</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_show_think_tolerance</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_sight_npc_search_time</code></summary>

How often AI look for other NPC enemies

default: `"0.45"`  
flags: `0x4000`  
</details>
<details>
<summary><code>ai_simplify_path_dist</code></summary>



default: `"1024"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_simplify_path_quick_dist</code></summary>



default: `"1024"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_solid_spawn_script_error</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_sprint_min_enemy_dist</code></summary>

Don't sprint if enemy is within this distance

default: `"256"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_squad_cache_path_max_dest_diff</code></summary>



default: `"1500"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_squad_cache_path_max_start_diff</code></summary>



default: `"1000"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_squad_clear_assigned_node_time</code></summary>



default: `"5"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_squad_enemy_notify_delay</code></summary>



default: `"0.7"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_squad_keep_dist_increment</code></summary>



default: `"800"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_squad_keep_dist_start</code></summary>



default: `"1200"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_squad_min_cache_path_length</code></summary>



default: `"1500"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_squad_min_salute_interval</code></summary>

AI squad will not salute friendly player more frequently than this

default: `"20"`  
flags: `0x4000`  
</details>
<details>
<summary><code>ai_squad_move_spread_factor</code></summary>

Allow this times hull width from squad centroid before slowing down or speeding up members

default: `"10"`  
flags: `0x4000`  
</details>
<details>
<summary><code>ai_squad_num_LOFs</code></summary>



default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_squad_num_chasers</code></summary>



default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_squad_stay_close_radius</code></summary>



default: `"500"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_stepsize</code></summary>



default: `"18"`  
flags: `0x2102`  
</details>
<details>
<summary><code>ai_strong_optimizations</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_strong_optimizations_no_checkstand</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_team_enemy_notify_interval</code></summary>

Interval for notifying teammates of my current enemy. Should be more than ai_squad_enemy_notify_delay

default: `"3.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_team_enemy_notify_max_dist</code></summary>

maximum distance for notifying teammates about enemies

default: `"3000"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_test_hull_model_name</code></summary>



default: `"mdl/robots/drone_frag/drone_frag.rmdl"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_threaded_post_process_is_delayed</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_titan_grapple_max_len</code></summary>



default: `"3000"`  
flags: `0x2002`  
</details>
<details>
<summary><code>ai_use_cached_squad_paths</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_use_clipped_paths</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_use_cluster_path</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_use_efficiency</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_use_frame_think_limits</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>ai_use_think_optimizations</code></summary>



default: `"1"`  
flags: `0x2`  
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
<summary><code>automantle_debug</code></summary>

Debugs player auto-mantle behavior

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_duration_above</code></summary>

Duration of "above" mantle animation

default: `"0.5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_duration_above</code></summary>

Duration of "above" mantle animation

default: `"0.5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_duration_below</code></summary>

Duration of "below" mantle animation

default: `"0.65"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_duration_below</code></summary>

Duration of "below" mantle animation

default: `"0.65"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_duration_high</code></summary>

Duration of "high" mantle animation

default: `"0.35"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_duration_high</code></summary>

Duration of "high" mantle animation

default: `"0.35"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_duration_level</code></summary>

Duration of "level" mantle animation

default: `"1.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_duration_level</code></summary>

Duration of "level" mantle animation

default: `"1.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>automantle_enable</code></summary>

Enables player auto-mantle behavior

default: `"1"`  
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
<summary><code>automantle_wallrun_maxangle_view</code></summary>

Max angle the player can be facing from the wall to auto-mantle while wall running

default: `"45"`  
flags: `0x2002`  
</details>
<details>
<summary><code>autosprint_type</code></summary>

Automatically sprint when walking forward.  0: off, 1: on, 2: on (pilots only), 3: on (titans only)

default: `"0"`  
flags: `0x1000200`  
</details>
<details>
<summary><code>base_tickinterval_mp</code></summary>

The tick interval used by the MP games.

default: `"0.05"`  
flags: `0x2`  
min value: `0.01`  
max value: `0.5`  
</details>
<details>
<summary><code>base_tickinterval_sp</code></summary>

The tick interval used by the SP games.

default: `"0.05"`  
flags: `0x2`  
min value: `0.01`  
max value: `0.5`  
</details>
<details>
<summary><code>baseanimatingoverlay_playbackRateThreshold</code></summary>



default: `"0.05"`  
flags: `0x2`  
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
<summary><code>bbox_draw_vphysics</code></summary>

Draws physics data when showing ent_bbox

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
<summary><code>breakable_disable_gib_limit</code></summary>



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
<summary><code>c_thirdpersonshoulderaimdist</code></summary>



default: `"120.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_thirdpersonshoulderdist</code></summary>



default: `"40.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_thirdpersonshouldergetsviewpunch</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_thirdpersonshoulderheight</code></summary>



default: `"5.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>c_thirdpersonshoulderheightaffectsangles</code></summary>



default: `"0"`  
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
<summary><code>checkstuck_nonworld</code></summary>

Checks for the player being stuck in non-world entities and disallows movement when they are stuck.

default: `"0"`  
flags: `0x2002`  
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

default: `"300"`  
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
<summary><code>cl_showLoadMovies</code></summary>



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
<summary><code>cl_simdbones_slerp</code></summary>

Use SIMD bone slerping.

default: `"0"`  
flags: `0x2002`  
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
<summary><code>cl_sticksCountAgainstIdle</code></summary>



default: `"1"`  
flags: `0x2`  
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
<summary><code>cl_use_calculate_local_player</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>cl_use_simd_bones</code></summary>

1 use SIMD bones 0 use scalar bones.

default: `"1"`  
flags: `0x2002`  
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
<summary><code>coll_spatial_entry_limit_server</code></summary>

How many entries are used in the spatial acceleration structure for dynamic entities on the server.

default: `"3000"`  
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



default: `"0"`  
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
<summary><code>csm_auto_entity</code></summary>



default: `"1"`  
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

Expands CSM Depth coverage. 1 - csm_z_coverage_sea_level to csm_z_coverage_jump_height, 2 - Static shadow's depth range

default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_z_coverage_jump_height</code></summary>



default: `"25000"`  
flags: `0x2`  
</details>
<details>
<summary><code>csm_z_coverage_sea_level</code></summary>



default: `"0"`  
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
<summary><code>damage_debug</code></summary>



default: `"0"`  
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
<summary><code>data_map_do_display</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>data_map_do_validate</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>death_velocityScale</code></summary>

Scales the death velocity per point of damage

default: `"100"`  
flags: `0x2`  
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
<summary><code>debug_draw_all_entity_links</code></summary>

Draw all entity links

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>debug_draw_box_depth_test</code></summary>

Toggle depth test for debug draw box functionality

default: `"1"`  
flags: `0x2`  
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
<summary><code>debug_overlay_fullposition</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>debug_physimpact</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>debug_touchlinks</code></summary>

Spew touch link activity

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>decal_normal_eps</code></summary>



default: `"0.45"`  
flags: `0x2`  
</details>
<details>
<summary><code>decalfrequency</code></summary>



default: `"10"`  
flags: `0x102`  
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
<summary><code>drawBeams</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>draw_target_info_offscreen</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>dropped_weapon_limit</code></summary>



default: `"8"`  
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
<summary><code>ent_create_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ent_debugkeys</code></summary>



default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>ent_lightweightEnts</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>ent_messages_draw</code></summary>

Visualizes all entity input/output activity.

default: `"0"`  
flags: `0x4004`  
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
<summary><code>ent_text_mode</code></summary>

0: Full,  1: Brief

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>ent_text_no_player_ents</code></summary>

If true then filter out all entities that are players, parented to players or owned by players


default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ent_text_only_transmitted_ents</code></summary>

If true then only show entities that are transmitted from server to client. Eg. server only entities are filtered out


default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ent_text_pick_type</code></summary>

0: All, 1: NPCs and Players

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>ent_text_radius_default</code></summary>

The default radius to use when finding entities to toggle ent_text on


default: `"100"`  
flags: `0x2`  
</details>
<details>
<summary><code>entity_skipRedundantAddEffects</code></summary>



default: `"1"`  
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
<summary><code>everything_unlocked</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>explosion_orientation_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>fakelag_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>fast_iteration</code></summary>



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
<summary><code>fireteam_catchup_max_speed_scale</code></summary>

Maximum speed scale on top of animation speed change

default: `"1.2"`  
flags: `0x4000`  
</details>
<details>
<summary><code>fireteam_catchup_sprint_dist</code></summary>

Sprint if more than this distance from goal

default: `"250"`  
flags: `0x2`  
</details>
<details>
<summary><code>fireteam_cover_search_tolerance</code></summary>

Distance leader moves for triggering new cover search

default: `"100"`  
flags: `0x2`  
</details>
<details>
<summary><code>fireteam_leader_cover_max_speed_threshold</code></summary>

If speed is above max run speed * this, fireteam doesn't search for cover

default: `"0.9"`  
flags: `0x2`  
</details>
<details>
<summary><code>fireteam_leader_runtime_tolerance</code></summary>

If the leader is running for this amount of time, start moving with the leader

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>fireteam_member0_angle</code></summary>



default: `"180"`  
flags: `0x2`  
</details>
<details>
<summary><code>fireteam_member0_offset_x</code></summary>



default: `"-145"`  
flags: `0x2`  
</details>
<details>
<summary><code>fireteam_member0_offset_y</code></summary>



default: `"40"`  
flags: `0x2`  
</details>
<details>
<summary><code>fireteam_member1_angle</code></summary>



default: `"60"`  
flags: `0x2`  
</details>
<details>
<summary><code>fireteam_member1_offset_x</code></summary>



default: `"180"`  
flags: `0x2`  
</details>
<details>
<summary><code>fireteam_member1_offset_y</code></summary>



default: `"80"`  
flags: `0x2`  
</details>
<details>
<summary><code>fireteam_member2_angle</code></summary>



default: `"-60"`  
flags: `0x2`  
</details>
<details>
<summary><code>fireteam_member2_offset_x</code></summary>



default: `"180"`  
flags: `0x2`  
</details>
<details>
<summary><code>fireteam_member2_offset_y</code></summary>



default: `"-80"`  
flags: `0x2`  
</details>
<details>
<summary><code>fireteam_move_delay</code></summary>

Amount of delay between each fireteam member starting to move

default: `"0.4"`  
flags: `0x2`  
</details>
<details>
<summary><code>fireteam_move_tolerance</code></summary>

Distance target must move to reset formation

default: `"20"`  
flags: `0x2`  
</details>
<details>
<summary><code>fireteam_use_cover_hints</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>fireteam_use_offsets</code></summary>

Use offsets for fireteam formation. Requires map restart

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>first_person_bullet_delay</code></summary>

Set the amount of additional delay for first person bullets fired with net_optimize_weapons in seconds. Required so bullets match animations with cl_predict 0 and in kill replay

default: `"0.1f"`  
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
<summary><code>fog_volume_debug</code></summary>

If enabled, prints diagnostic information about the current fog volume

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>force3PLaserAttachment</code></summary>



default: `"HEADFOCUS"`  
flags: `0x2002`  
</details>
<details>
<summary><code>force3PLaserAttachment</code></summary>



default: `"HEADFOCUS"`  
flags: `0x2002`  
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
<summary><code>func_break_reduction_factor</code></summary>



default: `".5"`  
flags: `0x2`  
</details>
<details>
<summary><code>func_breakdmg_bullet</code></summary>



default: `"0.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>func_breakdmg_club</code></summary>



default: `"1.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>func_breakdmg_explosive</code></summary>



default: `"1.25"`  
flags: `0x2`  
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
<summary><code>fx_screenspacepass</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>g_debug_doors</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>g_debug_flying_ai</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>g_debug_ragdoll_removal</code></summary>



default: `"0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>g_debug_trackpather</code></summary>



default: `"0"`  
flags: `0x4000`  
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
flags: `0x2`  
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
<summary><code>glass_shatter_attack_speed_scale</code></summary>



default: `"300"`  
flags: `0x2`  
</details>
<details>
<summary><code>glass_shatter_direction_force_scale</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>glass_shatter_drop_speed</code></summary>



default: `"100"`  
flags: `0x2`  
</details>
<details>
<summary><code>glass_shatter_explosive_scale</code></summary>



default: `"1"`  
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
<summary><code>globalNonRewindingObject_DontSave</code></summary>



default: `"1"`  
flags: `0x2`  
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

default: `"3000"`  
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
<summary><code>hibernation_assumed_max_player_speed</code></summary>

Assumed max player speed for estimating how long until we need to reevaluate hibernation

default: `"1400"`  
flags: `0x2`  
</details>
<details>
<summary><code>hibernation_debounce_dist</code></summary>

Entities that stop hibernating must go this far outside of the required distance to start hibernating again

default: `"256"`  
flags: `0x8000002`  
max value: `340282350000000000000000000000000000000`  
</details>
<details>
<summary><code>hibernation_enable</code></summary>

Enables entity hibernation

default: `"1"`  
flags: `0x8000002`  
max value: `340282350000000000000000000000000000000`  
</details>
<details>
<summary><code>hibernation_far_dist</code></summary>

Far distance for entity hibernation

default: `"8000"`  
flags: `0x8000002`  
max value: `340282350000000000000000000000000000000`  
</details>
<details>
<summary><code>hibernation_medium_dist</code></summary>

Medium distance for entity hibernation

default: `"3000"`  
flags: `0x8000002`  
max value: `340282350000000000000000000000000000000`  
</details>
<details>
<summary><code>hibernation_min_reevaluate_time</code></summary>

Entities always wait at least this long before their next reevaluation of whether they should be hibernating

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>hibernation_near_dist</code></summary>

Near distance for entity hibernation

default: `"1000"`  
flags: `0x8000002`  
max value: `340282350000000000000000000000000000000`  
</details>
<details>
<summary><code>hidehud</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>high_perf_dev_server</code></summary>



default: `"0"`  
flags: `0x2`  
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
<summary><code>hudchat_dead_can_only_talk_to_other_dead</code></summary>



default: `"0"`  
flags: `0x2`  
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
<summary><code>idleKickTime_min_alive_seconds</code></summary>

Dead players' idle timers will never go below this many seconds, so they have a chance to recover after spawning

default: `"40"`  
flags: `0x22`  
</details>
<details>
<summary><code>idleKickTime_minutes</code></summary>

Time after which idle players in a game are kicked

default: `"2"`  
flags: `0x22`  
</details>
<details>
<summary><code>idleKickTime_party_minutes</code></summary>

Time after which idle players in a party are kicked (all party members must be idle)

default: `"10"`  
flags: `0x22`  
</details>
<details>
<summary><code>idleKickTime_privatematch_game_minutes</code></summary>

Time after which idle players in a private match game are kicked

default: `"10"`  
flags: `0x22`  
</details>
<details>
<summary><code>idleKickTime_privatematch_lobby_minutes</code></summary>

Time after which idle players in a private match lobby are kicked (all players must be idle)

default: `"10"`  
flags: `0x22`  
</details>
<details>
<summary><code>idleKickTime_training_minutes</code></summary>

Time after which idle players in a training game are kicked

default: `"3"`  
flags: `0x22`  
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
<summary><code>ik_enable_server</code></summary>

Enables IK on the server

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
<summary><code>info_spawnpoint_human_classname</code></summary>

Class settings used to get size of info_spawnpoint_human

default: `"settings/player/mp/spectator.rpak"`  
flags: `0x6`  
</details>
<details>
<summary><code>info_spawnpoint_titan_classname</code></summary>

Class settings used to get size of info_spawnpoint_titan

default: `"settings/player/mp/spectator.rpak"`  
flags: `0x6`  
</details>
<details>
<summary><code>infoblock_requestInterval</code></summary>

Time between info block requests

default: `"300"`  
flags: `0x2`  
</details>
<details>
<summary><code>interpolate_on_parent_change</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>intro_viewed</code></summary>

Whether the introduction video has been viewed by this player

default: `"0"`  
flags: `0x1000000`  
</details>
<details>
<summary><code>invalidate_opt</code></summary>

Toggles the optimized InvalidatePhysicsRecursive codepath to early out if there's nothing to do

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>invalidate_opt</code></summary>

Toggles the optimized InvalidatePhysicsRecursive codepath to early out if there's nothing to do

default: `"1"`  
flags: `0x2`  
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
<summary><code>jump_graceperiod</code></summary>

Extra time during which a player can jump after falling off a ledge

default: `"0.2"`  
flags: `0x2002`  
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
<summary><code>lagcompensation_debug_ent</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>lagcompensation_ignore_friendlies</code></summary>



default: `"0"`  
flags: `0x4000`  
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
<summary><code>leech_lagcompensate</code></summary>



default: `"0"`  
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
<summary><code>lerp_debugEnt_server</code></summary>



default: `"1"`  
flags: `0x2`  
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
<summary><code>map_wants_save_disable</code></summary>



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
<summary><code>mat_autoexposure_override_min_max</code></summary>

When this is not zero, mat_autoexposure_min and mat_autoexposure_max override any existing setting in the map.

default: `"0"`  
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



default: `"0.7"`  
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

default: `"-8"`  
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
<summary><code>mat_depthfeather_enable</code></summary>



default: `"1"`  
flags: `0x40000002`  
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



default: `"-2"`  
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

default: `"west us"`  
flags: `0x80080200`  
</details>
<details>
<summary><code>match_myDatacenter</code></summary>

Which datacenter we prefer (same as match_myBestDatacenter unless user changes it)

default: `"west us"`  
flags: `0x80080200`  
</details>
<details>
<summary><code>match_myRankedDatacenter</code></summary>

Which datacenter we prefer for Ranked play (same as match_myBestDatacenter unless user changes it)

default: `"west us"`  
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



default: `"0"`  
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
<summary><code>matchresults_write_enabled</code></summary>



default: `"0"`  
flags: `0x22`  
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
<summary><code>melee_cone_trace_box_check</code></summary>



default: `"0.5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>melee_cone_trace_lag_compensate_user_command_target</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>melee_lunge_abort_distance</code></summary>

Abort the lunge if the distance moved in one frame is less than this much of the expected lunge distance.

default: `"0.25"`  
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
<summary><code>melee_titan_execution_attacker_can_be_ref</code></summary>



default: `"0"`  
flags: `0x2`  
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

Language to use for audio (requires a restart to change.)

default: `""`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_listener_freeze</code></summary>

When 1, stop updating listener position.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>miles_max_sound_commands_per_server_frame</code></summary>

Drop sounds commands past this. -1 to disable.

default: `"-1"`  
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
<summary><code>miles_server_disable_sounds</code></summary>

Don't send server sounds over the network.

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
<summary><code>miles_server_useSoundIDTable</code></summary>

Use soundId table optimization (2 = also send original ID over network.)

default: `"1"`  
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
<summary><code>missile_debug_draw</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>missile_default_speed</code></summary>



default: `"2500"`  
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
<summary><code>modeldecals_forceAllowed</code></summary>

If 0 don't generate decal data for any models. If 1 generate decal data for all models. Otherwise do according to the qc.

default: `"-1"`  
flags: `0x2`  
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
<summary><code>mp_allowNPCs</code></summary>



default: `"1"`  
flags: `0x102`  
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
<summary><code>mp_defaultteam</code></summary>



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
<summary><code>mp_fraglimit</code></summary>



default: `"20000"`  
flags: `0x102`  
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
<summary><code>mp_teamlist</code></summary>



default: `"hgrunt;scientist"`  
flags: `0x102`  
</details>
<details>
<summary><code>mp_teamoverride</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>mp_weaponstay</code></summary>



default: `"0"`  
flags: `0x102`  
</details>
<details>
<summary><code>mtx_svEdition</code></summary>



default: `"25"`  
flags: `0x2002`  
</details>
<details>
<summary><code>multiplayer_animstate_once_per_frame_on_server</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>muteWeaponSounds</code></summary>



default: `"0"`  
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
<summary><code>navmesh_move_along_surface_asserts</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>navmesh_normal_links_only</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>navmesh_test_zone_connectivity_traverse_anim_type</code></summary>



default: `"human"`  
flags: `0x2`  
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
<summary><code>net_sv_showusercmd</code></summary>

Show user command decoding

default: `"0"`  
flags: `0x2`  
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
<summary><code>noclip_fixup</code></summary>



default: `"1"`  
flags: `0x4000`  
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
<summary><code>npc_chancetohit_forcedOn</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>npc_sight_mode</code></summary>

Each mode represents different visual additions to npc_sight. (0: barebones) (1: near->intersection corner lines) (2: curved intersection lines)

default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>npc_titan_always_block_projectile_health</code></summary>

At this health and below always try to vortex projectiles

default: `"50"`  
flags: `0x4000`  
</details>
<details>
<summary><code>npc_titan_block_projectile_chance</code></summary>

Chance to use vortex/particle wall on projectile before taking damage at full health. Scales up to 100 npc_titan_always_block_projectile_health

default: `"10"`  
flags: `0x4000`  
</details>
<details>
<summary><code>npc_titan_footstep_sound_radius</code></summary>

How far other enemies will hear this titan sound

default: `"700"`  
flags: `0x4000`  
</details>
<details>
<summary><code>npc_titan_light_pain_threshold</code></summary>

Amount of damage a titan must take on a single damage to do light pain

default: `"80"`  
flags: `0x4000`  
</details>
<details>
<summary><code>npc_titan_phys_ignore_mass</code></summary>

Minimum mass of physics objects that the titan will collide with

default: `"300"`  
flags: `0x4000`  
</details>
<details>
<summary><code>npc_titan_phys_knock_damage</code></summary>

Damage applied to objects the titan knocks away

default: `"50"`  
flags: `0x4000`  
</details>
<details>
<summary><code>npc_titan_phys_knock_mass</code></summary>

Maximum mass of physics objects that the titan will knock away

default: `"2000"`  
flags: `0x4000`  
</details>
<details>
<summary><code>npc_titan_phys_knock_radius</code></summary>

Maximum distance of physics objects a titan can knock away

default: `"100"`  
flags: `0x4000`  
</details>
<details>
<summary><code>npc_titan_phys_knock_speed</code></summary>

Speed at which the titan knocks physics objects away

default: `"500"`  
flags: `0x4000`  
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
<summary><code>offhandTossOverheadPitchThreshold</code></summary>



default: `"-1.0"`  
flags: `0x2002`  
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
<summary><code>ordnancePickupSound</code></summary>



default: `"player_ordinance_pickup"`  
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
<summary><code>parenting_clearParentOriginFix</code></summary>



default: `"1"`  
flags: `0x2`  
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
<summary><code>particle_test_attach_attachment</code></summary>

Attachment index for attachment mode

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>particle_test_attach_mode</code></summary>

Possible Values: 'start_at_attachment', 'follow_attachment', 'start_at_origin', 'follow_origin'

default: `"follow_attachment"`  
flags: `0x4000`  
</details>
<details>
<summary><code>particle_test_file</code></summary>

Name of the particle system to dynamically spawn

default: `""`  
flags: `0x4000`  
</details>
<details>
<summary><code>particles_cull_dlights</code></summary>



default: `"1"`  
flags: `0x2002`  
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
<summary><code>particles_spawncull_report</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>particles_try_reloading_sheets</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>particles_try_reloading_sheets</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>parties_alwaysReadSubs</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>party_alwaysGoToLobbyOnSwitch</code></summary>



default: `"0"`  
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
<summary><code>persistence_clForceNew</code></summary>



default: `"0"`  
flags: `0x200`  
</details>
<details>
<summary><code>persistence_hostname</code></summary>



default: `""`  
flags: `0x80000`  
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
<summary><code>phys_cfm_anglejointstop</code></summary>



default: `"0.0001"`  
flags: `0x2002`  
</details>
<details>
<summary><code>phys_damage_players</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>phys_drawContacts</code></summary>



default: `"0"`  
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
<summary><code>phys_impactforcescale</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>phys_showObjectCount</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>phys_show_active</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>phys_speeds</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>phys_stressbodyweights</code></summary>



default: `"5.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>phys_timescale</code></summary>

Scale time for physics

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>phys_updateDummyGeomsThreaded</code></summary>

Go wide across threads with Physics_UpdateDummyGeoms()

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>phys_upimpactforcescale</code></summary>



default: `"0.375"`  
flags: `0x2`  
</details>
<details>
<summary><code>physics_async_cl</code></summary>

Run physics simulation asynchronously from the main thread.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>physics_async_sv</code></summary>

Run physics simulation asynchronously from the main server thread.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>physics_autoSleepAngularThreshold</code></summary>

Angular speed below which a physic object goes to sleep. (in degrees / second)

default: `"120"`  
flags: `0x2002`  
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
<summary><code>physics_tunnelChecksForceAlways</code></summary>

Require objects to do tunnel checks every frame.

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>physicsshadowupdate_render</code></summary>



default: `"0"`  
flags: `0x2`  
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

default: `"4"`  
flags: `0x2`  
</details>
<details>
<summary><code>ping_qos_units</code></summary>

Divisor to use for pings, so we don't think a 3 ping is wildly better than a 4 ping, but we do think a 33 ping is worse than a 31 ping (at 60fps, that's another frame of latency)

default: `"32"`  
flags: `0x2`  
</details>
<details>
<summary><code>ping_show_measured</code></summary>

Use only the measured value for ping reporting.

default: `"1"`  
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
<summary><code>playerDeathAnimMaxFrames</code></summary>

Max length of a death animation, in server frames.

default: `"300"`  
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
<summary><code>player_ADS_buffer_time_seconds</code></summary>

How long (in seconds) will the game buffer a Toggle Zoom attempt if the player cannot ADS when they press the button.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>player_adjustTimersWithTimeBase</code></summary>

Adjusts player and weapon time fields when player time base changes.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>player_charDataMinInterval</code></summary>

Minimum interval for script to use between calls to SetPlayerCharData()

default: `"2"`  
flags: `0x2`  
</details>
<details>
<summary><code>player_debugPredictedPosition</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>player_debug_print_damage</code></summary>

When true, print amount and type of all damage received by player to console.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>player_deltaAnimsMakeMeUnpredicted</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>player_disallow_negative_frametime</code></summary>

Ignores negative frametime values from client commands

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>player_dispatch_anim_events_per_frame</code></summary>

When enabled, animation events for players are handled every frame rather than only when there are move commands to process.

default: `"1"`  
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
<summary><code>player_extraairaccelleration</code></summary>

Extra air acceleration given to players, even if they're already at max speed. Helps to start wall running

default: `"2.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>player_find_rodeo_target_per_cmd</code></summary>

Run FindRodeoTarget callback

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>player_highFrequencyThinkDistance</code></summary>



default: `"6000"`  
flags: `0x2`  
</details>
<details>
<summary><code>player_maxTimerAdjust</code></summary>

If player time base changes more than this, don't adjust player and weapon time fields.

default: `"0.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>player_max_command_contexts</code></summary>



default: `"1000"`  
flags: `0x2`  
</details>
<details>
<summary><code>player_melee_cone_from_user_command_only</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>player_movementBounds_predictionShare</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>player_movement_debug</code></summary>

Draws player origin changes from player movement code

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>player_movingDeathThreshold</code></summary>



default: `"50"`  
flags: `0x6000`  
</details>
<details>
<summary><code>player_old_armor</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>player_respawnInputDebounceDuration</code></summary>

How long after respawning will certain player inputs be debounced for

default: `"0.5"`  
flags: `0x2`  
</details>
<details>
<summary><code>player_restore_use_SetLocalAngles</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>player_restore_use_UpdateCurrentPlayerClass</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>player_share_squad_info</code></summary>

Player shares enemy information with AI squad

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>player_showEyePosition</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>player_showpredictedposition</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>player_showpredictedposition_timestep</code></summary>



default: `"1.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>player_useMovementBounds</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>player_useMovementBounds</code></summary>



default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>player_userCmdsQueueWarning</code></summary>



default: `"300"`  
flags: `0x2`  
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
<summary><code>playerframetimekick_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>playerframetimekick_decayrate</code></summary>

rate at which the player's extra accumulated time falls off

default: `"0.032"`  
flags: `0x2`  
</details>
<details>
<summary><code>playerframetimekick_includerealtime</code></summary>

Whether to include passage of real time (as opposed to game time) in the check for players too far in the future

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>playerframetimekick_margin</code></summary>

Maximum amount of extra time the player can accumulate before being kicked

default: `"3.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>playerlist_showGen</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>players_updatePingTickInterval</code></summary>



default: `"20"`  
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
<summary><code>playlist_errorOnDeprecated</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>playlist_errorOnDeprecated</code></summary>



default: `"0"`  
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
<summary><code>prevent_ammo_suck</code></summary>

stops weapons from sucking up ammo

default: `"0"`  
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
<summary><code>prop_active_gib_limit</code></summary>



default: `"64"`  
flags: `0x2`  
</details>
<details>
<summary><code>prop_active_gib_max_fade_time</code></summary>



default: `"12"`  
flags: `0x2`  
</details>
<details>
<summary><code>prop_break_disable_float</code></summary>



default: `"1"`  
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
<summary><code>push_debug_pause_always</code></summary>

Always pause for squished entities, even if they are not a player (requires push_debug or push_debug_ent)

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>push_player_nearby_dist</code></summary>

Player distance offset for looking for nearby safe locations when they're about to be squished

default: `"12"`  
flags: `0x2`  
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
<summary><code>pve_modedetect_substring</code></summary>

Sets script VMs' "#if MP_PVEMODE" true on level load if playlist name begins with this substring.

default: `"freelance"`  
flags: `0x4000`  
</details>
<details>
<summary><code>pvs_addWorkItemsAccum</code></summary>

Accumulate this many work items from the main PVS job before adding them to the worker thread array, which can be slow

default: `"100"`  
flags: `0x2`  
</details>
<details>
<summary><code>pvs_addWorkItemsThreshold</code></summary>

load balancing threshold; if a node has more than this many leaves, it will spread the work across threads

default: `"10000"`  
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

default: `"1"`  
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
<summary><code>r_DrawBeams</code></summary>

0=Off, 1=Normal, 2=Wireframe

default: `"1"`  
flags: `0x4000`  
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
<summary><code>r_drawsprites</code></summary>



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
flags: `0x40000010`  
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
<summary><code>r_randomflex</code></summary>



default: `"0"`  
flags: `0x4000`  
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
<summary><code>r_volumetric_lighting_distFalloff</code></summary>



default: `"1500"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_volumetric_lighting_enabled</code></summary>

Toggle Volumetric Light

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>r_volumetric_lighting_intensity</code></summary>



default: `"0.15"`  
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
<summary><code>r_volumetric_lighting_scatter</code></summary>



default: `"0.8"`  
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
<summary><code>ragdoll_debug</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>ragdoll_skipDeathAcceleration</code></summary>



default: `"1000000"`  
flags: `0x2`  
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
<summary><code>reduced_trigger_checks</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>reliable_effects_enable</code></summary>



default: `"0"`  
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
<summary><code>requestBestObserverTargetFromScript</code></summary>

If set to false, code will no longer call CodeCallback_GetBestObserverTarget to get the best observer target.

default: `"1"`  
flags: `0x2`  
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
<summary><code>rodeoed_anim_weight</code></summary>



default: `"0.88"`  
flags: `0x2002`  
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
<summary><code>rope_default_segment_length</code></summary>

Length that causes an additional rope segment to be created after 500 units

default: `"300"`  
flags: `0x2`  
</details>
<details>
<summary><code>rope_min_pixel_diameter</code></summary>



default: `"2.0"`  
flags: `0x4000`  
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
<summary><code>save_client_entity</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>save_enable</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>save_thread_entities</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>scene_clamplookat</code></summary>

Clamp head turns to a MAX of 20 degrees per think.

default: `"1"`  
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
<summary><code>script_debugger_connect_server_on_mapspawn</code></summary>



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
flags: `0x2002`  
</details>
<details>
<summary><code>script_debugger_port_server</code></summary>



default: `"15100"`  
flags: `0x2002`  
</details>
<details>
<summary><code>script_debugger_port_ui</code></summary>



default: `"15102"`  
flags: `0x2002`  
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
<summary><code>script_server_fps</code></summary>



default: `"10"`  
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
<summary><code>send_data_to_all_players</code></summary>

If true update the local player for all players.

default: `"0"`  
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
<summary><code>server_helicopter_rope_events</code></summary>

Controls weather the helicopter/rope integration is controlled by the server.

default: `"0"`  
flags: `0x4000`  
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

Size of the blur filter applied to spot shadows that don't request a different size.

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

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>shadow_max_dynamic</code></summary>

Maximum number of shadows that should update every frame.

default: `"4"`  
flags: `0x40000000`  
</details>
<details>
<summary><code>shadow_max_old_dynamic</code></summary>

Maximum number of old shadows that should update every frame. It's a part of shadow_max_dynamic

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
<summary><code>shadow_min_count_smallest</code></summary>

Represents the minimum number of min resolution spot shadows to allocate in the shadow atlas.This will be adjusted upward to a multiple of the max sized spot shadow to find legal sized atlas dimensions.

default: `"576"`  
flags: `0x4000`  
</details>
<details>
<summary><code>shadow_minvariance</code></summary>

Minimum variance for shadow maps (controls edge softness)

default: `"0.01"`  
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
<summary><code>showhitlocation</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>showmem_enabled</code></summary>



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
<summary><code>showtriggers</code></summary>

Shows/draws trigger brushes. Must be set while loading the map

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>showtriggers_distance</code></summary>

Sets the distance from the player that triggers can be shown at.

default: `"1000"`  
flags: `0x4000`  
</details>
<details>
<summary><code>showtriggers_entindex</code></summary>

If set to a non-zero value, only shows the entity of the given entity index

default: `"0"`  
flags: `0x4000`  
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
<summary><code>sk_bullseye_health</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sk_healthcharger</code></summary>



default: `"0"`  
flags: `0x2`  
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
<summary><code>smart_ammo_debug</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>smart_ammo_interp_entity_fields</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>smoothstairs_lunge</code></summary>



default: `"0"`  
flags: `0x2002`  
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
<summary><code>soundscape_debug</code></summary>

1 - console prints when entering and leaving soundscapes | 2 - draws lines to all env_soundscape entities. Green lines show the active soundscape, red lines show soundscapes that aren't in range, and white lines show soundscapes that are in range, but not the active soundscape.

default: `"0"`  
flags: `0x4000`  
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
<summary><code>spawnpoint_avoid_npc_titan_sight</code></summary>

Avoids sightlines to NPC titans

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>spawnpoint_enemy_ai_far_dist</code></summary>

Distance beyond which enemies do not count for a spawnpoint for AI

default: `"1024"`  
flags: `0x2`  
</details>
<details>
<summary><code>spawnpoint_enemy_ai_near_dist</code></summary>

Distance within which enemies fully count for a spawnpoint for AI

default: `"384"`  
flags: `0x2`  
</details>
<details>
<summary><code>spawnpoint_enemy_titan_far_dist</code></summary>

Distance beyond which enemies do not count for a spawnpoint for titans

default: `"2048"`  
flags: `0x2`  
</details>
<details>
<summary><code>spawnpoint_enemy_titan_near_dist</code></summary>

Distance within which enemies fully count for a spawnpoint for titans

default: `"512"`  
flags: `0x2`  
</details>
<details>
<summary><code>spawnpoint_enemy_wallrun_far_dist</code></summary>

Distance beyond which enemies do not count for a spawnpoint for wall runners

default: `"2048"`  
flags: `0x2`  
</details>
<details>
<summary><code>spawnpoint_enemy_wallrun_near_dist</code></summary>

Distance within which enemies fully count for a spawnpoint for wall runners

default: `"512"`  
flags: `0x2`  
</details>
<details>
<summary><code>spawnpoint_friendly_ai_far_dist</code></summary>

Distance beyond which allies do not count for a spawnpoint for AI

default: `"1024"`  
flags: `0x2`  
</details>
<details>
<summary><code>spawnpoint_friendly_ai_near_dist</code></summary>

Distance within which allies fully count for a spawnpoint for AI

default: `"384"`  
flags: `0x2`  
</details>
<details>
<summary><code>spawnpoint_friendly_titan_far_dist</code></summary>

Distance beyond which allies do not count for a spawnpoint for titans

default: `"1536"`  
flags: `0x2`  
</details>
<details>
<summary><code>spawnpoint_friendly_titan_near_dist</code></summary>

Distance within which allies fully count for a spawnpoint for titans

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>spawnpoint_friendly_wallrun_far_dist</code></summary>

Distance beyond which allies do not count for a spawnpoint for wall runners

default: `"2048"`  
flags: `0x2`  
</details>
<details>
<summary><code>spawnpoint_friendly_wallrun_near_dist</code></summary>

Distance within which allies fully count for a spawnpoint for wall runners

default: `"384"`  
flags: `0x2`  
</details>
<details>
<summary><code>spawnpoint_last_spawn_rating</code></summary>

Penalty given to spawnpoint that was last chosen

default: `"-4"`  
flags: `0x2`  
</details>
<details>
<summary><code>spawnpoint_pet_titan_far_dist</code></summary>

Distance beyond which pet titan does not count for a spawnpoint

default: `"2048"`  
flags: `0x2`  
</details>
<details>
<summary><code>spawnpoint_pet_titan_near_dist</code></summary>

Distance within which pet titan fully counts for a spawnpoint

default: `"768"`  
flags: `0x2`  
</details>
<details>
<summary><code>spawnpoint_show_all</code></summary>

Draws/debugs spawn point entities

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>spawnpoint_show_class</code></summary>

Draw spawnpoint data for class: -1(automatic based on spawnpoint type), 0(titan), 1(wallrun), 2(ai)

default: `"-1"`  
flags: `0x2`  
</details>
<details>
<summary><code>spawnpoint_show_dist</code></summary>

Draws spawnpoint_*_near_dist and spawnpoint_*_far_dist around spawnpoints

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>spawnpoint_show_sight</code></summary>

Shows successful sight checks on spawnpoints that are drawing

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>spawnpoint_text_dist</code></summary>

Distance to draw spawnpoint debug text

default: `"500"`  
flags: `0x2`  
</details>
<details>
<summary><code>spawnpoint_text_dynamic</code></summary>

Shows dynamic spawnpoint text for current gamestate instead of last spawn

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>spawnpoint_text_team</code></summary>

Team used when drawing ent_text for spawnpoints. -1 for all, -2 for just TEAM_IMC (2) and TEAM_MILITIA (3)

default: `"-2"`  
flags: `0x2`  
</details>
<details>
<summary><code>spawnpoint_velocity_predict_time</code></summary>

Time to predict player movement when calculating spawn weights

default: `"2.0"`  
flags: `0x2`  
</details>
<details>
<summary><code>spec_chasecam_wait_on_dead_player_duration</code></summary>

How long chase camera should wait on a dead player before forcing camera to the next observer target

default: `"6"`  
flags: `0x2`  
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
<summary><code>status_effect_warning_level</code></summary>

Set to 0 for nothing, 1 for warnings, 2 for script errors

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>steam_id</code></summary>



default: `""`  
flags: `0x4000`  
</details>
<details>
<summary><code>steam_name</code></summary>



default: `""`  
flags: `0x4000`  
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

default: `"500"`  
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

0 = Never preload; 1 = Preload static models; 2 = Always Preload

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

default: `"400000"`  
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
<summary><code>sv_alltalk</code></summary>

Players can hear all other players, no team restrictions

default: `"0"`  
flags: `0x80100`  
</details>
<details>
<summary><code>sv_asyncAIInit</code></summary>



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
<summary><code>sv_bounds_show_errors</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_calcOriginsAnglesForSnapshotPacking</code></summary>



default: `"1"`  
flags: `0x2`  
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
<summary><code>sv_checkTransmitEntitiesPerJob</code></summary>



default: `"300"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_clampPlayerFrameTime</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_clockcorrection</code></summary>

Whether to try to correct client's tickcount if they are ahead of the server

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_clockcorrection_msecs</code></summary>

The server tries to keep each player's m_nTickBase within this many msecs of the server absolute tickcount

default: `"100"`  
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
<summary><code>sv_crossbowBoltAutoCull</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_debug_deferred_trace</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_debug_deferred_trace_overlay</code></summary>



default: `"0"`  
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
<summary><code>sv_disconnectOnTooManySnapshotFrames</code></summary>

Disconnect client when the server has sent 128 snapshot messages to client without the server getting any message from the client.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_dispatchSpawnsForBaseline</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_distanceCull</code></summary>

FL_EDICT_DISTANCECULL entities won't be transmitted to clients who are more than this distance from them

default: `"6000"`  
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
<summary><code>sv_footsteps</code></summary>

Play footstep sound for players

default: `"1"`  
flags: `0x2102`  
</details>
<details>
<summary><code>sv_forceChatToTeamOnly</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_forceGrapplesToFail</code></summary>

Force all grapples to fail on the server (cause clients to mispredict them)

default: `"0"`  
flags: `0x2`  
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
<summary><code>sv_interpolateAnimatedEntitiesPerJob</code></summary>



default: `"10"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_kickPlayersTooFarInFuture</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_lagpushticks</code></summary>

Push computed lag compensation amount by this many ticks.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_lerpAnims</code></summary>



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
<summary><code>sv_massreport</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_maxUserCmdsPerPlayerPerFrame</code></summary>



default: `"10"`  
flags: `0x2`  
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
<summary><code>sv_maxunlag</code></summary>

Maximum lag compensation in seconds

default: `"0.5"`  
flags: `0x2`  
min value: `0`  
max value: `0.5`  
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
<summary><code>sv_netvisdist</code></summary>

Test networking visibility distance

default: `"10000"`  
flags: `0x4006`  
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
<summary><code>sv_normalSimulationCommandThreshold</code></summary>



default: `"3"`  
flags: `0x2`  
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
<summary><code>sv_physics_maxvelocity</code></summary>

Max velocity of a vphysics object on the server

default: `"4000.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>sv_playerNameAppendCheater</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_playerSimTimeBuffer</code></summary>

On first sim, how close to curtime do we set the player's m_lastUCmdSimulationTime

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_players</code></summary>



default: `"1"`  
flags: `0x2012`  
</details>
<details>
<summary><code>sv_printClockCorrections</code></summary>

Should we print when we are adjust player's simulation time?

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_printClockTiming</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_printHighWaterMark</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_printNetReports</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_printSnapshotDeltaStats</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_props_funnel_into_portals</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>sv_props_funnel_into_portals_deceleration</code></summary>

When a funneling prop is leaving a portal, decelerate any velocity that is in opposition to funneling by this amount per second

default: `"2.0f"`  
flags: `0x4000`  
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
<summary><code>sv_recalcOrigins_enabled</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_recalcOrigins_entsPerJob</code></summary>



default: `"100"`  
flags: `0x2`  
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
<summary><code>sv_screenShake_debug</code></summary>



default: `"0"`  
flags: `0x4002`  
</details>
<details>
<summary><code>sv_screenShake_enabled</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>sv_screenShake_maxAmplitude</code></summary>



default: `"16"`  
flags: `0x4002`  
</details>
<details>
<summary><code>sv_scriptCompileAsync</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_script_perf_dump_on_shutdown</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_script_think_interval</code></summary>



default: `"0.1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_sendEarlyServerInfo</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_sendPlayerDamageMsg</code></summary>



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
<summary><code>sv_shiftPlayerSimTimeBackwards</code></summary>

If a player is sending us commands that would get ahead of the server, do we rewind time to allow it

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
<summary><code>sv_showWeirdDeltas</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_show_placement_help_in_preview</code></summary>

Forces the placement preview to show any help in placement given from info_placement_helper entities.


default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_showfiredbullets</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_showhitboxes</code></summary>

Send server-side hitboxes for specified entity to client (NOTE:  this uses lots of bandwidth, use on listen server only).

default: `"-1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>sv_showlagcompensation</code></summary>

Show lag compensated hitboxes whenever a player is lag compensated.

default: `"0"`  
flags: `0x4000`  
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
<summary><code>sv_spawnAIHintsInMP</code></summary>



default: `"0"`  
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
<summary><code>sv_thinktimecheck</code></summary>

Check for thinktimes all on same timestamp.

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_threaded_post_process_ai</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_threaded_post_process_players</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_threaded_pre_process_ents</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_turbophysics</code></summary>

Turns on turbo physics

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_turbophysics_player</code></summary>

Turns on turbo physics on players only

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_unlag</code></summary>

Enables player lag compensation

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_unlag_debug</code></summary>



default: `"1"`  
flags: `0x6`  
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
<summary><code>sv_usercmd_before_entities</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_usercmd_fairness</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_usercmd_fairness_dediOnly</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_usercmd_max_queued</code></summary>



default: `"40"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_usercmd_num_per_iteration</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_usercmd_shuffle_players</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_visualizetraces</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>sv_visualizetraces_duration</code></summary>



default: `"0.5"`  
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
<summary><code>sv_weapon_despawn_time</code></summary>



default: `"90.0"`  
flags: `0x4000`  
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
<summary><code>sys_minidumpexpandedspew</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sys_minidumpspewlines</code></summary>

Lines of crash dump console spew to keep.

default: `"500"`  
flags: `0x80000`  
</details>
<details>
<summary><code>system_alt_f4_closes_window</code></summary>

If set to true, alt+f4 will close the window

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>teamSpot_costLimitPerFrame</code></summary>

How much work the teamspot system can do before it tries to wait until next frame to continue. Use "teamSpot_debug 2" to debug costs.

default: `"10"`  
flags: `0x4004`  
</details>
<details>
<summary><code>teamSpot_enabled</code></summary>

Enables teamspotting including letting AI know about what players see. CANNOT BE CHANGED DURING GAMEPLAY

default: `"0"`  
flags: `0x6`  
</details>
<details>
<summary><code>teamSpot_lockOffTime</code></summary>

In seconds, how long a team must go without seeing an entity before they are no longer conisdered "spotted".

default: `"0.1"`  
flags: `0x4004`  
</details>
<details>
<summary><code>teamSpot_lockOnTime</code></summary>

Consecutive seconds that a team must see an entity before they are considered "spotted".

default: `"0.75"`  
flags: `0x4004`  
</details>
<details>
<summary><code>teamSpot_lockOnTimeForgiveness</code></summary>

During lockon, how many seconds a team can go without seeing an entity before the lockon process must start over.

default: `"0.45"`  
flags: `0x4004`  
</details>
<details>
<summary><code>teamSpot_minimap_enabled</code></summary>

Enables teamspotting on minimap.

default: `"0"`  
flags: `0x6004`  
</details>
<details>
<summary><code>teamSpot_threaded</code></summary>



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
<summary><code>template_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>test_massive_dmg</code></summary>



default: `"30"`  
flags: `0x2`  
</details>
<details>
<summary><code>test_massive_dmg_clip</code></summary>



default: `"0.5"`  
flags: `0x2`  
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
<summary><code>tether_npc_strength</code></summary>

Strength with which tether pulls NPCs (velocity per unit of distance)

default: `"5"`  
flags: `0x2`  
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
<summary><code>threat_detection_in_job</code></summary>



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
<summary><code>titanSoul_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>titan_hideEnts</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>titan_hidePlayer</code></summary>

Whether to send the player entity while they are in a titan

default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>titan_sprint_sound</code></summary>



default: `"titan_eject_servos_3p"`  
flags: `0x2`  
</details>
<details>
<summary><code>titan_step_damage_can_push_down</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>titan_step_damage_debug</code></summary>

Draw debug box of titan step damage volume

default: `"0"`  
flags: `0x6`  
</details>
<details>
<summary><code>titan_step_damage_rodeo_immunity_time</code></summary>

How long players are immune from step damage after jumping off from a rodeo

default: `"0.1"`  
flags: `0x6`  
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
<summary><code>trigger_ignore_nonsolids</code></summary>

If set to false, non solid objects will activate triggers.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>trigger_touch_on_spawn</code></summary>

Causes triggers to detect entities the moment they spawn.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>trigger_use_new_filters</code></summary>

Always use new trigger filter settings

default: `"0"`  
flags: `0x6`  
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
<summary><code>twitch_prime_linked</code></summary>

true if this user has a linked twitch prime account

default: `"0"`  
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
<summary><code>vgui_paintEnabled</code></summary>



default: `"1"`  
flags: `0x2`  
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
<summary><code>viewpunch_predictable_scalar</code></summary>

Scales view punch from damage (predictable server events only).

default: `"1.0"`  
flags: `0x2`  
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
<summary><code>visible_ent_cone_debug_duration_server</code></summary>



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
<summary><code>voice_serverdebug</code></summary>



default: `"0"`  
flags: `0x2`  
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
<summary><code>vprof_scope_entity_gamephys</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>vprof_scope_entity_thinks</code></summary>



default: `"0"`  
flags: `0x2`  
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
<summary><code>vprof_think_limit</code></summary>



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
<summary><code>weaponAmmoPickupSound</code></summary>



default: `"player.ammoPickup"`  
flags: `0x2`  
</details>
<details>
<summary><code>weaponFastHolsterScale</code></summary>

Scales holster animations if swapping to a weapon with "fast_swap_to" enabled.

default: `"0.25"`  
flags: `0x2002`  
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
<summary><code>weapon_showproficiency</code></summary>



default: `"0"`  
flags: `0x2`  
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
<summary><code>xc_crouch_debounce</code></summary>



default: `"0"`  
flags: `0x2`  
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
r5apex.exe!0x018dbfc0 ConVar Allow_auto_Party
r5apex.exe!0x01744390 ConVar BlendBonesMode
r5apex.exe!0x02803350 ConVar CTeam_DontSave
r5apex.exe!0x023d2940 ConVar DoorSoundPrefixDouble
r5apex.exe!0x023d5290 ConVar DoorSoundPrefixSingle
r5apex.exe!0x023ecb60 ConVar ScriptDisallowedToUsePersistenceOnSP
r5apex.exe!0x023df980 ConVar ScriptSaveAllowed
r5apex.exe!0x0175be80 ConVar StreamMicDisabled
r5apex.exe!0x0175c6a0 ConVar TalkIsStream
r5apex.exe!0x0175c4c0 ConVar VoiceNeedsReset
r5apex.exe!0x02377bb0 ConVar When set to 0, player always returns false when asked if it has a vehicle
r5apex.exe!0x023dc350 ConVar When set to 0, player always returns false when asked if it has a vehicle
r5apex.exe!0x023f6270 ConVar ai_ainRebuildOnMapStart
r5apex.exe!0x023fa310 ConVar ai_ain_crc_debug
r5apex.exe!0x023fdb30 ConVar ai_anim_overlay_debug
r5apex.exe!0x023f7d50 ConVar ai_auto_contact_solver
r5apex.exe!0x023f8a30 ConVar ai_choose_new_enemy_max_time
r5apex.exe!0x023fbb80 ConVar ai_cluster_select
r5apex.exe!0x023fd6f0 ConVar ai_collide_other_ai
r5apex.exe!0x023f88f0 ConVar ai_current_enemy_bonus
r5apex.exe!0x023f2a30 ConVar ai_debug_corpse
r5apex.exe!0x023f8be0 ConVar ai_debug_directnavprobe
r5apex.exe!0x023f7e70 ConVar ai_debug_doors
r5apex.exe!0x023f6430 ConVar ai_debug_draw_depth_test
r5apex.exe!0x023f4960 ConVar ai_debug_draw_nav_dist
r5apex.exe!0x023f8990 ConVar ai_debug_dyninteractions
r5apex.exe!0x023fca20 ConVar ai_debug_efficiency
r5apex.exe!0x023f2850 ConVar ai_debug_enemies
r5apex.exe!0x023f6570 ConVar ai_debug_enemy_memory
r5apex.exe!0x023fa710 ConVar ai_debug_engagement_dist
r5apex.exe!0x023f5e10 ConVar ai_debug_follow
r5apex.exe!0x023f3eb0 ConVar ai_debug_info_node_spectre
r5apex.exe!0x023fa5d0 ConVar ai_debug_los
r5apex.exe!0x023f9860 ConVar ai_debug_move_script
r5apex.exe!0x023f87b0 ConVar ai_debug_move_transitions
r5apex.exe!0x023fd390 ConVar ai_debug_nodes
r5apex.exe!0x023f79e0 ConVar ai_debug_obstacle_avoid
r5apex.exe!0x023f8f90 ConVar ai_debug_pieoff
r5apex.exe!0x023ef2d0 ConVar ai_debug_poseparameters
r5apex.exe!0x023f76c0 ConVar ai_debug_savePosition
r5apex.exe!0x023f8670 ConVar ai_debug_search_paths
r5apex.exe!0x02826470 ConVar ai_debug_shoot_positions
r5apex.exe!0x023f9680 ConVar ai_debug_squads
r5apex.exe!0x023f4270 ConVar ai_debug_stats
r5apex.exe!0x023f8e30 ConVar ai_debug_test_anim_path
r5apex.exe!0x023f82b0 ConVar ai_debug_think_ticks
r5apex.exe!0x023f4a00 ConVar ai_default_efficient
r5apex.exe!0x023fc240 ConVar ai_disable_task_announce_attack
r5apex.exe!0x023fa270 ConVar ai_draw_motor_movement
r5apex.exe!0x023ef960 ConVar ai_efficiency_override
r5apex.exe!0x023f2370 ConVar ai_enable_corpse_manager
r5apex.exe!0x023f3ff0 ConVar ai_excluded_clusters
r5apex.exe!0x023f2710 ConVar ai_follow_use_points
r5apex.exe!0x023f8cf0 ConVar ai_follow_use_points_when_moving
r5apex.exe!0x023fa850 ConVar ai_frametime_limit
r5apex.exe!0x023f3f50 ConVar ai_grenade_default_weapon
r5apex.exe!0x023f9720 ConVar ai_grenade_enabled
r5apex.exe!0x023f5790 ConVar ai_grenade_forced_weapon
r5apex.exe!0x023f2010 ConVar ai_grenade_fuse_time
r5apex.exe!0x023f61d0 ConVar ai_grenade_initial_contact_delay
r5apex.exe!0x023f3880 ConVar ai_grenade_max_throw_speed
r5apex.exe!0x023efa00 ConVar ai_grenade_target_debounce_default
r5apex.exe!0x023f92b0 ConVar ai_grenade_target_horizontal_offset
r5apex.exe!0x023f4cf0 ConVar ai_grenade_target_variance_dist_scalar
r5apex.exe!0x023f3120 ConVar ai_grenade_target_variance_min
r5apex.exe!0x023f22d0 ConVar ai_grenade_throw_debounce
r5apex.exe!0x023fbc20 ConVar ai_local_step_size
r5apex.exe!0x023f97c0 ConVar ai_max_corpse_detect_dist
r5apex.exe!0x023f9350 ConVar ai_max_look_at_friendly_dist
r5apex.exe!0x023faba0 ConVar ai_max_node_drop
r5apex.exe!0x023f7fb0 ConVar ai_max_triangulation_attempts
r5apex.exe!0x023fd2f0 ConVar ai_max_triangulation_dist
r5apex.exe!0x023fbeb0 ConVar ai_melee_debug
r5apex.exe!0x023fa3b0 ConVar ai_melee_kill_sound_radius
r5apex.exe!0x023f8850 ConVar ai_min_signal_dist
r5apex.exe!0x023efdd0 ConVar ai_missFastPlayer_sideWindowYMax
r5apex.exe!0x023faef0 ConVar ai_missFastPlayer_sideWindowYMin
r5apex.exe!0x023f74e0 ConVar ai_missFastPlayer_sideWindowZMax
r5apex.exe!0x023f8170 ConVar ai_missFastPlayer_sideWindowZMin
r5apex.exe!0x023f56f0 ConVar ai_missFastPlayer_topWindowYMax
r5apex.exe!0x023f5f50 ConVar ai_missFastPlayer_topWindowYMin
r5apex.exe!0x023eff10 ConVar ai_missFastPlayer_topWindowZMax
r5apex.exe!0x023fb3d0 ConVar ai_missFastPlayer_topWindowZMin
r5apex.exe!0x023ef4d0 ConVar ai_move_do_short_probe
r5apex.exe!0x023f4c50 ConVar ai_move_probe_delay
r5apex.exe!0x023f93f0 ConVar ai_move_sanity_check
r5apex.exe!0x023f2990 ConVar ai_moveprobe_debug
r5apex.exe!0x023faa30 ConVar ai_moveprobe_jump_debug
r5apex.exe!0x023f4b40 ConVar ai_near_node_for_hull_box_extent
r5apex.exe!0x023f7f10 ConVar ai_no_local_ground_paths
r5apex.exe!0x023f2670 ConVar ai_no_local_paths
r5apex.exe!0x023ef710 ConVar ai_no_node_cache
r5apex.exe!0x023fc130 ConVar ai_no_select_box
r5apex.exe!0x023f3b20 ConVar ai_no_steer
r5apex.exe!0x023f90d0 ConVar ai_node_draw_safety
r5apex.exe!0x023f03c0 ConVar ai_node_select
r5apex.exe!0x023fb170 ConVar ai_pain_death_sound_radius
r5apex.exe!0x023f4aa0 ConVar ai_pain_on_repeat_damage_threshold
r5apex.exe!0x023f6cb0 ConVar ai_pain_on_repeated_damage
r5apex.exe!0x023f0460 ConVar ai_path_adjust_speed_on_immediate_turns
r5apex.exe!0x023f37e0 ConVar ai_path_dangerous_cluster_cost_scalar
r5apex.exe!0x023f6c10 ConVar ai_path_dangerous_cluster_death_time_inc
r5apex.exe!0x023f19c0 ConVar ai_path_dangerous_cluster_exclude_dist
r5apex.exe!0x023f1f70 ConVar ai_path_dangerous_cluster_look_ahead
r5apex.exe!0x023f67e0 ConVar ai_path_dangerous_cluster_min_time
r5apex.exe!0x023f6880 ConVar ai_path_insert_pause_at_est_end
r5apex.exe!0x023fa450 ConVar ai_path_insert_pause_at_obstruction
r5apex.exe!0x023f80d0 ConVar ai_physics_shadow
r5apex.exe!0x023f4090 ConVar ai_pos_debug
r5apex.exe!0x023f6090 ConVar ai_radial_max_link_dist
r5apex.exe!0x023fd790 ConVar ai_range_attack_twitch_debounce
r5apex.exe!0x023f0020 ConVar ai_react_far_dist
r5apex.exe!0x023f6130 ConVar ai_reasonable_facing_min_dist
r5apex.exe!0x023f2fe0 ConVar ai_rebalance_thinks
r5apex.exe!0x023ef570 ConVar ai_recent_enemy_damage_dist_bonus
r5apex.exe!0x023f4d90 ConVar ai_recent_enemy_damage_expire_time
r5apex.exe!0x023ef0f0 ConVar ai_require_pvs
r5apex.exe!0x023face0 ConVar ai_route_simplify_interval
r5apex.exe!0x02828640 ConVar ai_run_from_enemy_try_shoot_chance
r5apex.exe!0x023fd8d0 ConVar ai_schedule_reset_conditions_on_gather
r5apex.exe!0x023f2490 ConVar ai_schedule_selector_debug
r5apex.exe!0x023f5ff0 ConVar ai_script_assault_points_validation_debug
r5apex.exe!0x023f27b0 ConVar ai_script_nodes_draw
r5apex.exe!0x023f3e10 ConVar ai_shot_bias
r5apex.exe!0x023fbff0 ConVar ai_shot_stats_term
r5apex.exe!0x023fba40 ConVar ai_show_hull_attacks
r5apex.exe!0x023f20b0 ConVar ai_show_path_search_nodes
r5apex.exe!0x023f95e0 ConVar ai_show_think_tolerance
r5apex.exe!0x023f7580 ConVar ai_sight_npc_search_time
r5apex.exe!0x023f9540 ConVar ai_simplify_path_dist
r5apex.exe!0x023fd1b0 ConVar ai_simplify_path_quick_dist
r5apex.exe!0x023f3920 ConVar ai_solid_spawn_script_error
r5apex.exe!0x023fad80 ConVar ai_sprint_min_enemy_dist
r5apex.exe!0x023fa530 ConVar ai_squad_cache_path_max_dest_diff
r5apex.exe!0x023f41d0 ConVar ai_squad_cache_path_max_start_diff
r5apex.exe!0x023f8210 ConVar ai_squad_clear_assigned_node_time
r5apex.exe!0x023faad0 ConVar ai_squad_enemy_notify_delay
r5apex.exe!0x023f64d0 ConVar ai_squad_keep_dist_increment
r5apex.exe!0x023fcb60 ConVar ai_squad_keep_dist_start
r5apex.exe!0x023faf90 ConVar ai_squad_min_cache_path_length
r5apex.exe!0x023f55a0 ConVar ai_squad_min_salute_interval
r5apex.exe!0x023fcac0 ConVar ai_squad_move_spread_factor
r5apex.exe!0x023efe70 ConVar ai_squad_num_LOFs
r5apex.exe!0x023fd250 ConVar ai_squad_num_chasers
r5apex.exe!0x023f8710 ConVar ai_squad_stay_close_radius
r5apex.exe!0x023ef850 ConVar ai_stepsize
r5apex.exe!0x023f9030 ConVar ai_strong_optimizations
r5apex.exe!0x023fbd70 ConVar ai_strong_optimizations_no_checkstand
r5apex.exe!0x023f5500 ConVar ai_team_enemy_notify_interval
r5apex.exe!0x023f78b0 ConVar ai_team_enemy_notify_max_dist
r5apex.exe!0x023f0160 ConVar ai_test_hull_model_name
r5apex.exe!0x023fda90 ConVar ai_threaded_post_process_is_delayed
r5apex.exe!0x028275f0 ConVar ai_titan_grapple_max_len
r5apex.exe!0x023fbcc0 ConVar ai_use_cached_squad_paths
r5apex.exe!0x023fbae0 ConVar ai_use_clipped_paths
r5apex.exe!0x023f9210 ConVar ai_use_cluster_path
r5apex.exe!0x023fae50 ConVar ai_use_efficiency
r5apex.exe!0x023f99a0 ConVar ai_use_frame_think_limits
r5apex.exe!0x023f39c0 ConVar ai_use_think_optimizations
r5apex.exe!0x023d3930 ConVar airslowmo_enabled
r5apex.exe!0x023d4350 ConVar airslowmo_enter_time
r5apex.exe!0x023cfec0 ConVar airslowmo_ground_immediate_end
r5apex.exe!0x023d21d0 ConVar airslowmo_leave_time
r5apex.exe!0x023d0fb0 ConVar airslowmo_scripted_speed
r5apex.exe!0x023d58a0 ConVar airslowmo_when_hovering
r5apex.exe!0x023f4820 ConVar animEvent_debug
r5apex.exe!0x01f8d860 ConVar animEvent_debugEnt
r5apex.exe!0x01f97770 ConVar animEvent_debug_cl
r5apex.exe!0x02817770 ConVar anim_estimateVelocity
r5apex.exe!0x02817fd0 ConVar anim_playerMovementAngleMargin
r5apex.exe!0x02817c10 ConVar anim_player_ragdoll_fix
r5apex.exe!0x023e1740 ConVar anim_print_transition_overflow
r5apex.exe!0x028178f0 ConVar anim_runGestureAnimEventsToCompletionOnReset_client
r5apex.exe!0x01f75e70 ConVar anim_showPoseParamErrors
r5apex.exe!0x02817ad0 ConVar anim_showstate
r5apex.exe!0x02817b70 ConVar anim_showstatelog
r5apex.exe!0x023ee620 ConVar anim_transitionsequences
r5apex.exe!0x023ee580 ConVar anim_view_entity_third_person_camera_use_move_parent
r5apex.exe!0x018db2a0 ConVar announcement
r5apex.exe!0x018db840 ConVar announcementImage
r5apex.exe!0x018db480 ConVar announcementVersion
r5apex.exe!0x01758d90 ConVar async_serialize
r5apex.exe!0x0236f260 ConVar automantle_backoff_anim_maxfrac
r5apex.exe!0x023b8e00 ConVar automantle_backoff_anim_maxfrac
r5apex.exe!0x023486e0 ConVar automantle_cooldown
r5apex.exe!0x02398de0 ConVar automantle_cooldown
r5apex.exe!0x0236db60 ConVar automantle_dangle_required_space
r5apex.exe!0x023b71a0 ConVar automantle_dangle_required_space
r5apex.exe!0x0234a4f0 ConVar automantle_debug
r5apex.exe!0x0239af40 ConVar automantle_debug
r5apex.exe!0x02372a90 ConVar automantle_duration_above
r5apex.exe!0x023cd4e0 ConVar automantle_duration_above
r5apex.exe!0x0234b170 ConVar automantle_duration_below
r5apex.exe!0x0239bfa0 ConVar automantle_duration_below
r5apex.exe!0x0234ad90 ConVar automantle_duration_high
r5apex.exe!0x0239b7a0 ConVar automantle_duration_high
r5apex.exe!0x02348510 ConVar automantle_duration_level
r5apex.exe!0x023989b0 ConVar automantle_duration_level
r5apex.exe!0x0236ee10 ConVar automantle_enable
r5apex.exe!0x023b8a40 ConVar automantle_enable
r5apex.exe!0x02370080 ConVar automantle_forwarddist
r5apex.exe!0x023ba370 ConVar automantle_forwarddist
r5apex.exe!0x0236fea0 ConVar automantle_gun_enable_height
r5apex.exe!0x023b9ab0 ConVar automantle_gun_enable_height
r5apex.exe!0x0235c130 ConVar automantle_height_above
r5apex.exe!0x023aca70 ConVar automantle_height_above
r5apex.exe!0x02370980 ConVar automantle_height_below
r5apex.exe!0x023bad20 ConVar automantle_height_below
r5apex.exe!0x0234bb00 ConVar automantle_height_level
r5apex.exe!0x0239c3d0 ConVar automantle_height_level
r5apex.exe!0x0234bde0 ConVar automantle_jumpoff_anim_maxfrac
r5apex.exe!0x0239c5e0 ConVar automantle_jumpoff_anim_maxfrac
r5apex.exe!0x0236ed70 ConVar automantle_jumpoff_duration
r5apex.exe!0x023b89a0 ConVar automantle_jumpoff_duration
r5apex.exe!0x02371810 ConVar automantle_max_frac
r5apex.exe!0x023bbf40 ConVar automantle_max_frac
r5apex.exe!0x0236dcf0 ConVar automantle_maxangle_push
r5apex.exe!0x023b7890 ConVar automantle_maxangle_push
r5apex.exe!0x0236ff40 ConVar automantle_maxangle_view
r5apex.exe!0x023b9b50 ConVar automantle_maxangle_view
r5apex.exe!0x0234ac50 ConVar automantle_min_frac
r5apex.exe!0x0239b660 ConVar automantle_min_frac
r5apex.exe!0x0234bf20 ConVar automantle_mindist
r5apex.exe!0x0239c7c0 ConVar automantle_mindist
r5apex.exe!0x0236fe00 ConVar automantle_rest_frac
r5apex.exe!0x023b9a10 ConVar automantle_rest_frac
r5apex.exe!0x0234be80 ConVar automantle_rest_frac_below
r5apex.exe!0x0239c720 ConVar automantle_rest_frac_below
r5apex.exe!0x02372bd0 ConVar automantle_searchdist
r5apex.exe!0x023cd760 ConVar automantle_searchdist
r5apex.exe!0x01f85b60 ConVar automantle_view_correction_speed
r5apex.exe!0x01f84800 ConVar automantle_view_high_yaw_max
r5apex.exe!0x01f88590 ConVar automantle_view_pitch_max
r5apex.exe!0x0231ade0 ConVar automantle_view_pitch_min
r5apex.exe!0x01f80490 ConVar automantle_view_yaw_max
r5apex.exe!0x0236fcc0 ConVar automantle_wallrun_maxangle_view
r5apex.exe!0x023b9810 ConVar automantle_wallrun_maxangle_view
r5apex.exe!0x02333f00 ConVar autosprint_type
r5apex.exe!0x023fe7d0 ConVar base_tickinterval_mp
r5apex.exe!0x02475b50 ConVar base_tickinterval_sp
r5apex.exe!0x023482d0 ConVar baseanimatingoverlay_playbackRateThreshold
r5apex.exe!0x023987d0 ConVar baseanimatingoverlay_playbackRateThreshold
r5apex.exe!0x0185b1a0 ConVar baselines_print
r5apex.exe!0x023f8ad0 ConVar bbox_draw_vphysics
r5apex.exe!0x0238db70 ConVar bhit_enable
r5apex.exe!0x0281e010 ConVar bhit_enable
r5apex.exe!0x023933e0 ConVar bhit_reliable
r5apex.exe!0x028239e0 ConVar bhit_reliable
r5apex.exe!0x0175d760 ConVar bink_materials_enabled
r5apex.exe!0x02343d80 ConVar bink_preload_videopanel_movies
r5apex.exe!0x02324770 ConVar boost_jetwash_prediction_factor
r5apex.exe!0x01857f60 ConVar bot_lagOut
r5apex.exe!0x02814e80 ConVar breakable_disable_gib_limit
r5apex.exe!0x017491b0 ConVar budget_animatingEntities
r5apex.exe!0x017472e0 ConVar budget_animationOverlayEntities
r5apex.exe!0x017485e0 ConVar budget_combatCharEntities
r5apex.exe!0x01747080 ConVar budget_weaponEntities
r5apex.exe!0x01748a80 ConVar budget_ziplineEntities
r5apex.exe!0x023ec070 ConVar bug_reproNum
r5apex.exe!0x0174a050 ConVar buildcubemaps_async
r5apex.exe!0x01749750 ConVar buildcubemaps_index
r5apex.exe!0x01747860 ConVar buildcubemaps_pvs_start_early
r5apex.exe!0x01749a50 ConVar buildcubemaps_single_step
r5apex.exe!0x01748b20 ConVar building_cubemaps
r5apex.exe!0x02822030 ConVar bulletPredictionDebug
r5apex.exe!0x023e0000 ConVar bullet_trace_test_debug
r5apex.exe!0x023e2a50 ConVar bullet_trace_test_enable
r5apex.exe!0x023244f0 ConVar c_dropship_ground_fx_dist_interval
r5apex.exe!0x01f8c230 ConVar c_dropship_ground_fx_time_interval
r5apex.exe!0x01f85ca0 ConVar c_dropship_rope_debug
r5apex.exe!0x01f94bb0 ConVar c_dropship_rope_events
r5apex.exe!0x02318290 ConVar c_dropship_rope_magnitude
r5apex.exe!0x01f83550 ConVar c_dropship_rope_range
r5apex.exe!0x0232b830 ConVar c_maxdistance
r5apex.exe!0x02326930 ConVar c_maxpitch
r5apex.exe!0x0233a200 ConVar c_maxyaw
r5apex.exe!0x0232d400 ConVar c_mindistance
r5apex.exe!0x02330a30 ConVar c_minpitch
r5apex.exe!0x02332470 ConVar c_minyaw
r5apex.exe!0x023364c0 ConVar c_orthoheight
r5apex.exe!0x02325eb0 ConVar c_orthowidth
r5apex.exe!0x0233bbb0 ConVar c_thirdpersonshoulderaimdist
r5apex.exe!0x02338f40 ConVar c_thirdpersonshoulderdist
r5apex.exe!0x0233bcf0 ConVar c_thirdpersonshouldergetsviewpunch
r5apex.exe!0x0233be30 ConVar c_thirdpersonshoulderheight
r5apex.exe!0x0233bc50 ConVar c_thirdpersonshoulderheightaffectsangles
r5apex.exe!0x0233bd90 ConVar c_thirdpersonshoulderoffset
r5apex.exe!0x01f8ac70 ConVar c_threadedAnimPostData
r5apex.exe!0x0232cb50 ConVar cam_collision
r5apex.exe!0x02333cb0 ConVar cam_idealdelta
r5apex.exe!0x02329b10 ConVar cam_idealdist
r5apex.exe!0x023363a0 ConVar cam_ideallag
r5apex.exe!0x0233a600 ConVar cam_idealpitch
r5apex.exe!0x02336060 ConVar cam_idealyaw
r5apex.exe!0x023264f0 ConVar cam_pitchLock_feetRelative
r5apex.exe!0x0232dc00 ConVar cam_pitchlock_on
r5apex.exe!0x02337fb0 ConVar cam_pitchlock_period
r5apex.exe!0x0232a250 ConVar cam_pitchlock_phase
r5apex.exe!0x02338950 ConVar cam_pitchlock_pitchBase
r5apex.exe!0x0232b040 ConVar cam_pitchlock_pitchRange
r5apex.exe!0x02325c70 ConVar cam_pitchlock_pitchWiggleRoom
r5apex.exe!0x0233b9d0 ConVar cam_player_viewheight_scale
r5apex.exe!0x0232b6f0 ConVar cam_showangles
r5apex.exe!0x02338050 ConVar cc_captiontrace
r5apex.exe!0x02331a00 ConVar cc_global_norepeat
r5apex.exe!0x0233b170 ConVar cc_linger_time
r5apex.exe!0x0232f570 ConVar cc_max_duration
r5apex.exe!0x0232fb90 ConVar cc_minvisibleitems
r5apex.exe!0x023339e0 ConVar cc_predisplay_time
r5apex.exe!0x02338270 ConVar cc_rui
r5apex.exe!0x0232df40 ConVar cc_text_size
r5apex.exe!0x0232c4b0 ConVar cc_timeshift_norepeat
r5apex.exe!0x0175c7e0 ConVar chatroom_console_ptt
r5apex.exe!0x018dc7e0 ConVar chatroom_debug
r5apex.exe!0x018dfb30 ConVar chatroom_doRealNameLookups
r5apex.exe!0x018dff60 ConVar chatroom_min_status_send_interval
r5apex.exe!0x02383ea0 ConVar chatroom_nameLength
r5apex.exe!0x02384be0 ConVar chatroom_namePaddingX
r5apex.exe!0x02384b40 ConVar chatroom_nameWidth
r5apex.exe!0x0185b9d0 ConVar chatroom_onlyWhenActive
r5apex.exe!0x02384780 ConVar chatroom_useSlopSpace
r5apex.exe!0x018dfbd0 ConVar chatroom_voiceMode
r5apex.exe!0x02384dc0 ConVar chatroom_voiceMode
r5apex.exe!0x02345370 ConVar cheap_captions_fadetime
r5apex.exe!0x02344a90 ConVar cheap_captions_test
r5apex.exe!0x0239b8e0 ConVar checkstuck_nonworld
r5apex.exe!0x0175d8a0 ConVar chroma_enable
r5apex.exe!0x01f7fe50 ConVar cl_NotifyAllLevelAssetsLoaded_endframe
r5apex.exe!0x0231ac00 ConVar cl_RunClientConnectScripts_Before_ProcessOnDataChangedEvents
r5apex.exe!0x01f8b9b0 ConVar cl_SetupAllBones
r5apex.exe!0x01f854c0 ConVar cl_ShowBoneSetupEnts
r5apex.exe!0x018569b0 ConVar cl_adjustTimeEntsPerJob
r5apex.exe!0x02346b30 ConVar cl_aggregate_particles
r5apex.exe!0x01f8a950 ConVar cl_allowABSCalculationDuringSnapshotScriptCalls
r5apex.exe!0x01f7f170 ConVar cl_allowABSDuringSnapshotScriptCalls
r5apex.exe!0x01f85100 ConVar cl_allowAnimsToInterpolateBackward
r5apex.exe!0x02347e70 ConVar cl_always_draw_3p_player
r5apex.exe!0x01f93a20 ConVar cl_always_ragdoll_radius
r5apex.exe!0x023388b0 ConVar cl_anglespeedkey
r5apex.exe!0x0231bb60 ConVar cl_anim_blend_transition_dist
r5apex.exe!0x01f87e30 ConVar cl_anim_detail_dist
r5apex.exe!0x01f878a0 ConVar cl_anim_face_dist
r5apex.exe!0x01f89150 ConVar cl_anim_sequence_transition_full_weight_optimization
r5apex.exe!0x023248b0 ConVar cl_anim_sounds_seek
r5apex.exe!0x01f75aa0 ConVar cl_approx_footstep_origin
r5apex.exe!0x0233a960 ConVar cl_approx_tracer_origin
r5apex.exe!0x02318930 ConVar cl_async_bone_setup
r5apex.exe!0x01f8a650 ConVar cl_base_entity_effect_lock
r5apex.exe!0x01f800d0 ConVar cl_bones_incremental_blend
r5apex.exe!0x01f832d0 ConVar cl_bones_incremental_transform
r5apex.exe!0x01f97d90 ConVar cl_bones_oldhack
r5apex.exe!0x02372810 ConVar cl_bounds_show_errors
r5apex.exe!0x02377300 ConVar cl_burninggibs
r5apex.exe!0x0174a1f0 ConVar cl_clock_correction
r5apex.exe!0x01748680 ConVar cl_clock_correction_ahead_correct_interval
r5apex.exe!0x01747650 ConVar cl_clock_correction_behind_correct_interval
r5apex.exe!0x01748d00 ConVar cl_clock_correction_force_server_tick
r5apex.exe!0x0181f370 ConVar cl_cmdbackup
r5apex.exe!0x0181e310 ConVar cl_cmdrate
r5apex.exe!0x017509b0 ConVar cl_configversion
r5apex.exe!0x0174e6d0 ConVar cl_configversion_dummy
r5apex.exe!0x0238b9d0 ConVar cl_cull_weapon_fx
r5apex.exe!0x0175a210 ConVar cl_dataBlockFragmentPL
r5apex.exe!0x0237d1d0 ConVar cl_deathhints_enabled
r5apex.exe!0x01f84010 ConVar cl_debugClientEntities
r5apex.exe!0x0237ae30 ConVar cl_debug_deferred_trace
r5apex.exe!0x0237d5d0 ConVar cl_debug_deferred_trace_overlay
r5apex.exe!0x02338670 ConVar cl_debug_model_fx_sounds
r5apex.exe!0x0234ab10 ConVar cl_decal_alwayswhite
r5apex.exe!0x02372950 ConVar cl_decal_backoff
r5apex.exe!0x02347fb0 ConVar cl_deferred_effects
r5apex.exe!0x02379f50 ConVar cl_deferred_trace_normal_priority
r5apex.exe!0x01f93980 ConVar cl_demoviewoverride
r5apex.exe!0x0231ae80 ConVar cl_disable_ragdolls
r5apex.exe!0x01f7f0d0 ConVar cl_disable_splitscreen_cpu_level_cfgs_in_pip
r5apex.exe!0x0175dae0 ConVar cl_disconnectOnTooManySnapshotFrames
r5apex.exe!0x023954c0 ConVar cl_doNetworkAsserts
r5apex.exe!0x0175da40 ConVar cl_doRecreateEnts
r5apex.exe!0x01f7ea10 ConVar cl_draw_player_model
r5apex.exe!0x0231b060 ConVar cl_drawhud
r5apex.exe!0x0233e690 ConVar cl_drawmonitors
r5apex.exe!0x01f925d0 ConVar cl_ejectbrass
r5apex.exe!0x0233d000 ConVar cl_enable_remote_splitscreen
r5apex.exe!0x0175dea0 ConVar cl_entCreateDeleteDebug
r5apex.exe!0x01f87940 ConVar cl_events_ignore_invalidate
r5apex.exe!0x01759330 ConVar cl_failremoteconnections
r5apex.exe!0x0233a2a0 ConVar cl_fasttempentcollision
r5apex.exe!0x01f841f0 ConVar cl_flip_vis_bits
r5apex.exe!0x0175dc20 ConVar cl_flushentitypacket
r5apex.exe!0x02318fe0 ConVar cl_footstep_event_max_dist
r5apex.exe!0x01f8caf0 ConVar cl_footstep_event_max_dist_titan
r5apex.exe!0x01855a80 ConVar cl_forceAdjustTime
r5apex.exe!0x018e0ad0 ConVar cl_fovScale
r5apex.exe!0x018e0a30 ConVar cl_gib_allow
r5apex.exe!0x023192e0 ConVar cl_gib_attack_dir_scale
r5apex.exe!0x01f750b0 ConVar cl_gib_lifetime
r5apex.exe!0x0233ffb0 ConVar cl_idealpitchscale
r5apex.exe!0x0181f810 ConVar cl_ignorepackets
r5apex.exe!0x01f8d9a0 ConVar cl_interp_all
r5apex.exe!0x01855940 ConVar cl_interpolate
r5apex.exe!0x01f86d20 ConVar cl_interpolate
r5apex.exe!0x01f83c70 ConVar cl_interpolateSoAllAnimsLoop
r5apex.exe!0x023243b0 ConVar cl_interpolation_before_prediction
r5apex.exe!0x0175b900 ConVar cl_isUnderAge
r5apex.exe!0x0181f9b0 ConVar cl_is_softened_locale
r5apex.exe!0x02381330 ConVar cl_jiggle_bone_debug
r5apex.exe!0x023811f0 ConVar cl_jiggle_bone_debug_pitch_constraints
r5apex.exe!0x023814b0 ConVar cl_jiggle_bone_debug_yaw_constraints
r5apex.exe!0x02381290 ConVar cl_jiggle_bone_invert
r5apex.exe!0x023813d0 ConVar cl_jiggle_bone_sanity
r5apex.exe!0x01855760 ConVar cl_keepPersistentDataOnDisconnect
r5apex.exe!0x02342300 ConVar cl_lagcompensation
r5apex.exe!0x01820550 ConVar cl_language
r5apex.exe!0x02324d10 ConVar cl_leafsystemvis
r5apex.exe!0x0231a100 ConVar cl_lerpIfChildrenLerp
r5apex.exe!0x0175a490 ConVar cl_loadBspFromServerInfo
r5apex.exe!0x01757380 ConVar cl_loadPostProcessShadersEarly
r5apex.exe!0x01756840 ConVar cl_loadStaticPropsInJob
r5apex.exe!0x018567d0 ConVar cl_matchmaking_timeout
r5apex.exe!0x01f93ca0 ConVar cl_minimal_rtt_shadows
r5apex.exe!0x02335c60 ConVar cl_model_fx_gib_cull_front_dist
r5apex.exe!0x02335880 ConVar cl_model_fx_gib_cull_radius
r5apex.exe!0x02338c50 ConVar cl_mouseenable
r5apex.exe!0x01820060 ConVar cl_move_use_dt
r5apex.exe!0x0181e760 ConVar cl_noTimeoutLocalHost
r5apex.exe!0x01820730 ConVar cl_overrideEventTimes
r5apex.exe!0x02343a60 ConVar cl_parallelParticlePreDrawWork
r5apex.exe!0x01f851a0 ConVar cl_parallel_clientside_animations
r5apex.exe!0x02380910 ConVar cl_particle_batch_mode
r5apex.exe!0x018e0850 ConVar cl_particle_fallback_base
r5apex.exe!0x018e0b70 ConVar cl_particle_fallback_multiplier
r5apex.exe!0x02345ab0 ConVar cl_particle_limiter_display_killed
r5apex.exe!0x02398080 ConVar cl_particle_limiter_hide_killable
r5apex.exe!0x028161b0 ConVar cl_particle_limiter_hide_killable
r5apex.exe!0x0233e370 ConVar cl_particle_limiter_max_particle_count
r5apex.exe!0x0233e8e0 ConVar cl_particle_limiter_max_system_count
r5apex.exe!0x023400d0 ConVar cl_particle_limiter_min_kill_distance
r5apex.exe!0x02345630 ConVar cl_particle_limiter_overlay
r5apex.exe!0x0233f8a0 ConVar cl_particle_max_count
r5apex.exe!0x02346a90 ConVar cl_particle_sim_fallback_base_multiplier
r5apex.exe!0x0233cae0 ConVar cl_particle_sim_fallback_threshold_ms
r5apex.exe!0x02343f40 ConVar cl_particle_snoozetime
r5apex.exe!0x02340630 ConVar cl_particles_show_bbox
r5apex.exe!0x023417f0 ConVar cl_particles_show_controlpoints
r5apex.exe!0x023475a0 ConVar cl_pclass
r5apex.exe!0x02344580 ConVar cl_pdump
r5apex.exe!0x0233cc20 ConVar cl_phys_maxticks
r5apex.exe!0x023421c0 ConVar cl_phys_show_active
r5apex.exe!0x023429c0 ConVar cl_phys_timescale
r5apex.exe!0x02346050 ConVar cl_physics_invalidate_ents
r5apex.exe!0x023462b0 ConVar cl_physics_maxvelocity
r5apex.exe!0x02377880 ConVar cl_physicsshadowupdate_render
r5apex.exe!0x02338fe0 ConVar cl_pitchspeed
r5apex.exe!0x0181fea0 ConVar cl_playback_screenshots
r5apex.exe!0x01f897f0 ConVar cl_player_fullupdate_predicted_origin_fix
r5apex.exe!0x0236ffe0 ConVar cl_player_touch_triggers
r5apex.exe!0x0175dcc0 ConVar cl_postSnapshotTransitionBlockCount
r5apex.exe!0x01f93e60 ConVar cl_preSnapshotTransitionBlockCount
r5apex.exe!0x02380a50 ConVar cl_pred_error_verbose
r5apex.exe!0x02340c60 ConVar cl_pred_optimize
r5apex.exe!0x0181fc70 ConVar cl_predict
r5apex.exe!0x01f746b0 ConVar cl_predict_basetoggles
r5apex.exe!0x0233ce40 ConVar cl_predict_cmdlimit
r5apex.exe!0x02346950 ConVar cl_predict_error_icon_duration
r5apex.exe!0x0233f980 ConVar cl_predict_error_icon_show
r5apex.exe!0x02344dd0 ConVar cl_predict_error_icon_threshold_angle
r5apex.exe!0x0233db40 ConVar cl_predict_error_icon_threshold_dist
r5apex.exe!0x023743c0 ConVar cl_predict_motioncontrol
r5apex.exe!0x02347f10 ConVar cl_predict_viewangles
r5apex.exe!0x0233daa0 ConVar cl_prediction_error_timestamps
r5apex.exe!0x023444a0 ConVar cl_predictionlist
r5apex.exe!0x02340d40 ConVar cl_predictweapons
r5apex.exe!0x023882c0 ConVar cl_prevent_weapon_text_hints
r5apex.exe!0x0231a220 ConVar cl_ragdoll_force_fade_time
r5apex.exe!0x02318890 ConVar cl_ragdoll_force_fade_time_local_view_player
r5apex.exe!0x02380220 ConVar cl_ragdoll_force_fade_time_on_moving_geo
r5apex.exe!0x01f928d0 ConVar cl_ragdoll_force_fade_time_titan
r5apex.exe!0x018e0990 ConVar cl_ragdoll_maxcount
r5apex.exe!0x018e0c10 ConVar cl_ragdoll_self_collision
r5apex.exe!0x018564b0 ConVar cl_replayDelayTolerance
r5apex.exe!0x02325150 ConVar cl_requireAnimForAnimEventsHdr
r5apex.exe!0x018561c0 ConVar cl_resend
r5apex.exe!0x018565f0 ConVar cl_resend_timeout
r5apex.exe!0x0181f070 ConVar cl_retire_low_priority_lights
r5apex.exe!0x01f8d900 ConVar cl_runWeaponCloneThinkWhenHidden
r5apex.exe!0x0237d490 ConVar cl_safearea
r5apex.exe!0x0181f4b0 ConVar cl_screenshotname
r5apex.exe!0x023480f0 ConVar cl_scriptCompileAsync
r5apex.exe!0x02341e80 ConVar cl_script_perf_dump_on_shutdown
r5apex.exe!0x0237af70 ConVar cl_shadowupdatespacing
r5apex.exe!0x01f92a90 ConVar cl_showClanTags
r5apex.exe!0x02348190 ConVar cl_showLoadMovies
r5apex.exe!0x023359a0 ConVar cl_show_splashes
r5apex.exe!0x02347dd0 ConVar cl_showerror
r5apex.exe!0x0233dca0 ConVar cl_showerror_watchfield
r5apex.exe!0x02392d40 ConVar cl_showfiredbullets
r5apex.exe!0x02344b30 ConVar cl_showfps
r5apex.exe!0x0233fdf0 ConVar cl_showfps_altframetime
r5apex.exe!0x02342fa0 ConVar cl_showpausedimage
r5apex.exe!0x0233c700 ConVar cl_showpos
r5apex.exe!0x01758930 ConVar cl_showsounds
r5apex.exe!0x02340bc0 ConVar cl_showtime
r5apex.exe!0x01744510 ConVar cl_simdbones_slerp
r5apex.exe!0x01f8a110 ConVar cl_simulateAllModelsRegardless
r5apex.exe!0x0231b100 ConVar cl_simulationtimefix
r5apex.exe!0x01f849c0 ConVar cl_skipAnimEventsOnProps
r5apex.exe!0x02345830 ConVar cl_skipfastpath
r5apex.exe!0x02325290 ConVar cl_smooth
r5apex.exe!0x01f8fd10 ConVar cl_smooth_debug
r5apex.exe!0x0231a7c0 ConVar cl_smoothtime
r5apex.exe!0x023b7240 ConVar cl_sticksCountAgainstIdle
r5apex.exe!0x01f7eab0 ConVar cl_threaded_bone_setup
r5apex.exe!0x0235c580 ConVar cl_updatedirty_async
r5apex.exe!0x01f87a80 ConVar cl_updatedirty_early
r5apex.exe!0x018205f0 ConVar cl_updaterate_mp
r5apex.exe!0x02338ea0 ConVar cl_upspeed
r5apex.exe!0x01820690 ConVar cl_useFutureSnapForEvents
r5apex.exe!0x01856000 ConVar cl_useLobbyTypeForChatroom
r5apex.exe!0x0175de00 ConVar cl_use_calculate_local_player
r5apex.exe!0x017442f0 ConVar cl_use_simd_bones
r5apex.exe!0x01f93f00 ConVar cl_view_cone
r5apex.exe!0x01f8c190 ConVar cl_view_cone_debug
r5apex.exe!0x01f94c90 ConVar cl_viewmodel_pre_animate
r5apex.exe!0x02336fe0 ConVar cl_warnAboutSoundsOnInvalidEntities
r5apex.exe!0x02325d90 ConVar cl_yawspeed
r5apex.exe!0x01751490 ConVar clampHostFrameTimeToOneTick_enable
r5apex.exe!0x01f87bc0 ConVar clearOnAnimChange
r5apex.exe!0x02348050 ConVar client_deferredSnapshotScriptCalls
r5apex.exe!0x0174ffb0 ConVar clientport
r5apex.exe!0x018e4ee0 ConVar cloak_enabled
r5apex.exe!0x01f69460 ConVar cloak_pilotNoiseFactor
r5apex.exe!0x01f695a0 ConVar cloak_pilotTint1
r5apex.exe!0x01f69140 ConVar cloak_pilotTint2
r5apex.exe!0x01f68ec0 ConVar cloak_pilotTint3
r5apex.exe!0x01748fd0 ConVar clock_bias_mp
r5apex.exe!0x01749eb0 ConVar clock_bias_sp
r5apex.exe!0x01749540 ConVar clock_showcorrections
r5apex.exe!0x01749db0 ConVar clock_showdebuginfo
r5apex.exe!0x1f86db70 ConVar closecaption
r5apex.exe!0x02378610 ConVar cockpitDrift_scalePitch
r5apex.exe!0x0237a740 ConVar cockpitDrift_scaleYaw
r5apex.exe!0x0237d880 ConVar cockpitDrift_speedPitch
r5apex.exe!0x023779c0 ConVar cockpitDrift_speedYaw
r5apex.exe!0x02386db0 ConVar cockpitShake_sourceRollRange
r5apex.exe!0x02387d10 ConVar cockpitShake_translateRange
r5apex.exe!0x02385ae0 ConVar cockpit_damage_chroma_scale
r5apex.exe!0x02385f20 ConVar cockpit_hit_chroma_max_time
r5apex.exe!0x02387800 ConVar cockpit_hit_chroma_scale
r5apex.exe!0x02386fb0 ConVar cockpit_pitch_down_frac
r5apex.exe!0x023884a0 ConVar cockpit_pitch_up_frac
r5apex.exe!0x02387370 ConVar cockpit_screen_boot_chroma_scale
r5apex.exe!0x02386a10 ConVar cockpit_screen_boot_delay_bottom
r5apex.exe!0x02388540 ConVar cockpit_screen_boot_delay_left
r5apex.exe!0x02385900 ConVar cockpit_screen_boot_delay_mid
r5apex.exe!0x02386e70 ConVar cockpit_screen_boot_delay_right
r5apex.exe!0x023888f0 ConVar cockpit_screen_boot_delay_top
r5apex.exe!0x01857130 ConVar coll_spatial_entry_limit_client
r5apex.exe!0x01857090 ConVar coll_spatial_entry_limit_server
r5apex.exe!0x018571d0 ConVar coll_spatial_optimize_prefetch
r5apex.exe!0x01744430 ConVar coll_use_bolt_size
r5apex.exe!0x02340f20 ConVar colorblind_mode
r5apex.exe!0x0185b570 ConVar communities_doRealNameLookupsForCommunityCreators
r5apex.exe!0x0185be30 ConVar communities_enabled
r5apex.exe!0x018db160 ConVar communities_hostname
r5apex.exe!0x0185b6b0 ConVar community
r5apex.exe!0x0185b7f0 ConVar community_abortCommunitySettingsTime
r5apex.exe!0x0185bb10 ConVar community_abortUserInfoTime
r5apex.exe!0x018da8a0 ConVar community_browse_excludeMine
r5apex.exe!0x0185ad40 ConVar community_clantags
r5apex.exe!0x018da460 ConVar community_doRealNameLookupsForInbox
r5apex.exe!0x0185bd90 ConVar community_frame_run
r5apex.exe!0x018dd280 ConVar community_queryServerWhenOrphaned
r5apex.exe!0x018da3c0 ConVar community_replaceInboxTokens
r5apex.exe!0x018dac60 ConVar community_replaceInboxTokens
r5apex.exe!0x018da2b0 ConVar community_resolveNames
r5apex.exe!0x018da5a0 ConVar community_resolveNames
r5apex.exe!0x0185ba70 ConVar community_send_server_voice
r5apex.exe!0x018da640 ConVar community_spam
r5apex.exe!0x0185bed0 ConVar community_staleCommunitySettingsTime
r5apex.exe!0x0185b890 ConVar community_staleUserInfoTime
r5apex.exe!0x018df6b0 ConVar con_logfile
r5apex.exe!0x01748e40 ConVar con_timestamp
r5apex.exe!0x018e7be0 ConVar cpu_level
r5apex.exe!0x01f923f0 ConVar cpu_level
r5apex.exe!0x01f94a90 ConVar createentitydecals
r5apex.exe!0x02476260 ConVar csm_auto_entity
r5apex.exe!0x018e7d20 ConVar csm_cascade_res
r5apex.exe!0x01f92830 ConVar csm_cascade_res
r5apex.exe!0x018e5020 ConVar csm_coverage
r5apex.exe!0x0231aca0 ConVar csm_culling_use_base_planes
r5apex.exe!0x01f8e060 ConVar csm_culling_use_exclusion_planes
r5apex.exe!0x01f74610 ConVar csm_culling_use_inclusion_planes
r5apex.exe!0x023251f0 ConVar csm_culling_use_planes
r5apex.exe!0x01f84ae0 ConVar csm_debug_2d
r5apex.exe!0x01f8a2d0 ConVar csm_debug_culling
r5apex.exe!0x02318590 ConVar csm_debug_vis_hi_range
r5apex.exe!0x01f89f50 ConVar csm_debug_vis_lo_range
r5apex.exe!0x01f8a070 ConVar csm_depth_bias
r5apex.exe!0x0231a400 ConVar csm_dropsequence_adjusted_coverage
r5apex.exe!0x01f74890 ConVar csm_dropsequence_adjustment
r5apex.exe!0x018e7960 ConVar csm_enabled
r5apex.exe!0x01f81330 ConVar csm_fadeModels
r5apex.exe!0x01f84920 ConVar csm_force_no_csm_in_reflections
r5apex.exe!0x01f74420 ConVar csm_frustum_draw
r5apex.exe!0x01f74380 ConVar csm_frustum_draw_lock
r5apex.exe!0x023181f0 ConVar csm_ignore_cascade12
r5apex.exe!0x01f802b0 ConVar csm_ignore_edge_planes
r5apex.exe!0x01f8b890 ConVar csm_ignore_face_planes
r5apex.exe!0x0231a060 ConVar csm_max_z_offset
r5apex.exe!0x01f88010 ConVar csm_min_z_offset
r5apex.exe!0x01f74bb0 ConVar csm_renderable_shadows
r5apex.exe!0x0231a2c0 ConVar csm_rope_shadows
r5apex.exe!0x01f87800 ConVar csm_rot_override
r5apex.exe!0x01f7f830 ConVar csm_rot_x
r5apex.exe!0x01f83230 ConVar csm_rot_y
r5apex.exe!0x0231aac0 ConVar csm_shadow_split_lerp_factor_range
r5apex.exe!0x01f85060 ConVar csm_texel_size_cascade_0
r5apex.exe!0x01f8a810 ConVar csm_texel_size_cascade_1
r5apex.exe!0x01f803f0 ConVar csm_texel_size_cascade_2
r5apex.exe!0x01f942b0 ConVar csm_texel_size_cascade_onecascade
r5apex.exe!0x01f7f790 ConVar csm_use_env_light_direction
r5apex.exe!0x01f8dfc0 ConVar csm_world_shadow_meshes
r5apex.exe!0x01f94970 ConVar csm_world_shadows
r5apex.exe!0x01f93fa0 ConVar csm_z_cover_world
r5apex.exe!0x01f8c0f0 ConVar csm_z_coverage_jump_height
r5apex.exe!0x02324a90 ConVar csm_z_coverage_sea_level
r5apex.exe!0x018dbac0 ConVar curl_allowHTTPS
r5apex.exe!0x018dbca0 ConVar curl_preloadDlls
r5apex.exe!0x018dbc00 ConVar curl_spamAllQueryStates
r5apex.exe!0x23477c30 ConVar cursorWide
r5apex.exe!0x02388150 ConVar damageIndicatorReplayTimeOffset
r5apex.exe!0x023fd970 ConVar damage_debug
r5apex.exe!0x0237d530 ConVar damage_indicator_style_pilot
r5apex.exe!0x0237a7e0 ConVar damage_indicator_style_titan
r5apex.exe!0x023d72a0 ConVar data_map_do_display
r5apex.exe!0x023dc9c0 ConVar data_map_do_validate
r5apex.exe!0x023f7300 ConVar death_velocityScale
r5apex.exe!0x01f94040 ConVar debugFootstepEffects
r5apex.exe!0x017471c0 ConVar debug_debug_overlay
r5apex.exe!0x02477d90 ConVar debug_draw_all_entity_links
r5apex.exe!0x0239a4a0 ConVar debug_draw_box_depth_test
r5apex.exe!0x0185bcf0 ConVar debug_force_textRestriction
r5apex.exe!0x0185b750 ConVar debug_force_ugcRestriction
r5apex.exe!0x0185bbb0 ConVar debug_force_voiceRestriction
r5apex.exe!0x01749250 ConVar debug_map_crc
r5apex.exe!0x023f9c20 ConVar debug_overlay_fullposition
r5apex.exe!0x0280b7b0 ConVar debug_physimpact
r5apex.exe!0x023dba80 ConVar debug_touchlinks
r5apex.exe!0x01f6abd0 ConVar decal_normal_eps
r5apex.exe!0x02477040 ConVar decalfrequency
r5apex.exe!0x01f74e30 ConVar delayPostSnapshotNotificationsToAfterInterpolation
r5apex.exe!0x018563a0 ConVar demo_autoRecord
r5apex.exe!0x01855ee0 ConVar demo_autoRecordName
r5apex.exe!0x023839a0 ConVar demo_connect_string
r5apex.exe!0x02383680 ConVar demo_ui_enable
r5apex.exe!0x023dd7a0 ConVar devStats
r5apex.exe!0x01753e80 ConVar developer
r5apex.exe!0x02387550 ConVar disable_player_use_prompts
r5apex.exe!0x01856e10 ConVar discord_largeImage
r5apex.exe!0x01856d70 ConVar discord_smallImage
r5apex.exe!0x01856eb0 ConVar discord_updatePresence
r5apex.exe!0x017477c0 ConVar dlight_default_falloff
r5apex.exe!0x0181fb50 ConVar dlight_enable
r5apex.exe!0x0181e9d0 ConVar dlight_overlay
r5apex.exe!0x023cf560 ConVar dodge_cockpitHack
r5apex.exe!0x023cf760 ConVar dodge_cockpitOffsetMax
r5apex.exe!0x023cf690 ConVar dodge_cockpitTiltMax
r5apex.exe!0x023d2ef0 ConVar dodge_vertical_enable
r5apex.exe!0x023d3760 ConVar dodge_vertical_horzspeedscale
r5apex.exe!0x023d19e0 ConVar dodge_vertical_in_air
r5apex.exe!0x023d15a0 ConVar dodge_vertical_threshold
r5apex.exe!0x023cf9e0 ConVar dodge_viewTiltDecreaseSpeed
r5apex.exe!0x023d0d50 ConVar dodge_viewTiltFalloffTime
r5apex.exe!0x023d01b0 ConVar dodge_viewTiltIncreaseSpeed
r5apex.exe!0x023d07b0 ConVar dodge_viewTiltMax
r5apex.exe!0x023439a0 ConVar dof_enable
r5apex.exe!0x01f68560 ConVar dof_farDepthEnd
r5apex.exe!0x01f68600 ConVar dof_farDepthStart
r5apex.exe!0x01f68740 ConVar dof_monitorFarDepthEnd
r5apex.exe!0x01f68420 ConVar dof_monitorFarDepthStart
r5apex.exe!0x01f682e0 ConVar dof_monitorNearDepthEnd
r5apex.exe!0x01f684c0 ConVar dof_monitorNearDepthStart
r5apex.exe!0x01f686a0 ConVar dof_nearDepthEnd
r5apex.exe!0x01f68380 ConVar dof_nearDepthStart
r5apex.exe!0x01f687e0 ConVar dof_overrideParams
r5apex.exe!0x0233f7c0 ConVar dof_variable_blur
r5apex.exe!0x01f81290 ConVar dormant_debug
r5apex.exe!0x0234aed0 ConVar drawBeams
r5apex.exe!0x02385de0 ConVar draw_target_info_offscreen
r5apex.exe!0x02818f80 ConVar dropped_weapon_limit
r5apex.exe!0x017475b0 ConVar dtwatchclass
r5apex.exe!0x0174a430 ConVar dtwatchdecode
r5apex.exe!0x01746ed0 ConVar dtwatchencode
r5apex.exe!0x0174a310 ConVar dtwatchent
r5apex.exe!0x017473f0 ConVar dtwatchvar
r5apex.exe!0x02373e50 ConVar dump_varsights_calculations
r5apex.exe!0x01f83db0 ConVar durango_voice_chat_team_only
r5apex.exe!0x018e7b40 ConVar dvs_enable
r5apex.exe!0x018e8970 ConVar dvs_gpuframetime_max
r5apex.exe!0x018e8ab0 ConVar dvs_gpuframetime_min
r5apex.exe!0x018e8a10 ConVar dvs_scale_min
r5apex.exe!0x01f68b00 ConVar edge_override_depth
r5apex.exe!0x01f6a680 ConVar edge_override_depth
r5apex.exe!0x01f68ba0 ConVar edge_override_silhouette
r5apex.exe!0x01f6a720 ConVar edge_override_silhouette
r5apex.exe!0x0175aa30 ConVar enable_KVFileOverrides
r5apex.exe!0x01747f50 ConVar enable_debug_overlays
r5apex.exe!0x023d5400 ConVar enable_height_based_land_anims
r5apex.exe!0x023d2630 ConVar enable_height_based_land_anims_titans
r5apex.exe!0x01f89010 ConVar enable_skeleton_draw
r5apex.exe!0x01755c20 ConVar encrypt_multiKey
r5apex.exe!0x023f3d70 ConVar ent_create_debug
r5apex.exe!0x0239bf00 ConVar ent_debugkeys
r5apex.exe!0x01f8df20 ConVar ent_lightweightEnts
r5apex.exe!0x023fa990 ConVar ent_messages_draw
r5apex.exe!0x01f74a70 ConVar ent_repack_almostFull
r5apex.exe!0x01f822f0 ConVar ent_repack_threshhold
r5apex.exe!0x023fd830 ConVar ent_text_mode
r5apex.exe!0x023feba0 ConVar ent_text_no_player_ents
r5apex.exe!0x023fea50 ConVar ent_text_only_transmitted_ents
r5apex.exe!0x023fc350 ConVar ent_text_pick_type
r5apex.exe!0x023f36d0 ConVar ent_text_radius_default
r5apex.exe!0x0235c090 ConVar entity_skipRedundantAddEffects
r5apex.exe!0x023ac9d0 ConVar entity_skipRedundantAddEffects
r5apex.exe!0x01855620 ConVar entity_useNetworkFieldBuffer
r5apex.exe!0x01f840b0 ConVar error_if_non_standard_ent_create
r5apex.exe!0x023432a0 ConVar eula_version
r5apex.exe!0x02341470 ConVar eula_version_accepted
r5apex.exe!0x01f88cf0 ConVar eventseq_debug
r5apex.exe!0x02373ef0 ConVar everything_unlocked
r5apex.exe!0x023d70e0 ConVar everything_unlocked
r5apex.exe!0x02476fa0 ConVar explosion_orientation_debug
r5apex.exe!0x01752280 ConVar fakelag_debug
r5apex.exe!0x023e17e0 ConVar fast_iteration
r5apex.exe!0x01f72080 ConVar fast_poly_convert
r5apex.exe!0x0282acd0 ConVar fatal_script_error_prompt
r5apex.exe!0x0282ab90 ConVar fatal_script_errors
r5apex.exe!0x0282ac30 ConVar fatal_script_errors_client
r5apex.exe!0x0282ad70 ConVar fatal_script_errors_server
r5apex.exe!0x01f834b0 ConVar fd_playlist_bits
r5apex.exe!0x018e1730 ConVar filesystem_buffer_size
r5apex.exe!0x018e1a50 ConVar filesystem_max_stdio_read
r5apex.exe!0x018e1af0 ConVar filesystem_native
r5apex.exe!0x018e1910 ConVar filesystem_report_buffered_io
r5apex.exe!0x018e1870 ConVar filesystem_unbuffered_io
r5apex.exe!0x018e19b0 ConVar filesystem_use_overlapped_io
r5apex.exe!0x023d63c0 ConVar fire_animevents_overlay_not_active
r5apex.exe!0x023f6a40 ConVar fireteam_catchup_max_speed_scale
r5apex.exe!0x023fd5b0 ConVar fireteam_catchup_sprint_dist
r5apex.exe!0x023fc3f0 ConVar fireteam_cover_search_tolerance
r5apex.exe!0x023f9cc0 ConVar fireteam_leader_cover_max_speed_threshold
r5apex.exe!0x023f7620 ConVar fireteam_leader_runtime_tolerance
r5apex.exe!0x023fbe10 ConVar fireteam_member0_angle
r5apex.exe!0x023f2530 ConVar fireteam_member0_offset_x
r5apex.exe!0x023fb030 ConVar fireteam_member0_offset_y
r5apex.exe!0x023f6ae0 ConVar fireteam_member1_angle
r5apex.exe!0x023fa8f0 ConVar fireteam_member1_offset_x
r5apex.exe!0x023fb470 ConVar fireteam_member1_offset_y
r5apex.exe!0x023f28f0 ConVar fireteam_member2_angle
r5apex.exe!0x023f3cd0 ConVar fireteam_member2_offset_x
r5apex.exe!0x023fbf50 ConVar fireteam_member2_offset_y
r5apex.exe!0x023fb0d0 ConVar fireteam_move_delay
r5apex.exe!0x023f9490 ConVar fireteam_move_tolerance
r5apex.exe!0x023ef7b0 ConVar fireteam_use_cover_hints
r5apex.exe!0x023f0200 ConVar fireteam_use_offsets
r5apex.exe!0x0238ab80 ConVar first_person_bullet_delay
r5apex.exe!0x0281b080 ConVar first_person_bullet_delay
r5apex.exe!0x02324db0 ConVar first_person_proxy_blend_distance
r5apex.exe!0x02826a50 ConVar first_person_proxy_debug
r5apex.exe!0x023411f0 ConVar firsttime_mp_message
r5apex.exe!0x02347d30 ConVar fog_enable
r5apex.exe!0x01755380 ConVar fog_enable_water_fog
r5apex.exe!0x02344310 ConVar fog_enableskybox
r5apex.exe!0x02476630 ConVar fog_volume_debug
r5apex.exe!0x02392ca0 ConVar force3PLaserAttachment
r5apex.exe!0x02823310 ConVar force3PLaserAttachment
r5apex.exe!0x0175b1b0 ConVar fps_max
r5apex.exe!0x01759bf0 ConVar fps_max_use_refresh
r5apex.exe!0x0175a2b0 ConVar fps_max_vsync
r5apex.exe!0x01f80210 ConVar freecam_swallowButtonInput
r5apex.exe!0x02348370 ConVar freefall_sound_autoplay_time
r5apex.exe!0x02398910 ConVar freefall_sound_autoplay_time
r5apex.exe!0x0236da20 ConVar freefall_sound_height
r5apex.exe!0x023b6fa0 ConVar freefall_sound_height
r5apex.exe!0x018dec20 ConVar friends_onlineUpdateInterval
r5apex.exe!0x018e0df0 ConVar fs_intralevel_reads
r5apex.exe!0x018e12f0 ConVar fs_monitor_read_from_pack
r5apex.exe!0x018e0f30 ConVar fs_report_intra_level_readopens
r5apex.exe!0x018e1420 ConVar fs_report_long_reads
r5apex.exe!0x018e0e90 ConVar fs_report_sync_opens
r5apex.exe!0x018e1110 ConVar fs_report_sync_opens_callstack
r5apex.exe!0x018e1250 ConVar fs_report_sync_opens_fatal
r5apex.exe!0x018e1070 ConVar fs_showAllReads
r5apex.exe!0x018e1bc0 ConVar fs_vpk_file_open
r5apex.exe!0x018e1600 ConVar fs_warning_mode
r5apex.exe!0x027f80c0 ConVar func_break_max_pieces
r5apex.exe!0x02434e60 ConVar func_break_reduction_factor
r5apex.exe!0x028151a0 ConVar func_breakdmg_bullet
r5apex.exe!0x02815100 ConVar func_breakdmg_club
r5apex.exe!0x02814f20 ConVar func_breakdmg_explosive
r5apex.exe!0x027f8160 ConVar fx_debug
r5apex.exe!0x023977f0 ConVar fx_deferWorldTraceConstraint
r5apex.exe!0x0232cd90 ConVar fx_glass_velocity_cap
r5apex.exe!0x02346730 ConVar fx_impact_ally
r5apex.exe!0x0233c270 ConVar fx_impact_enemy
r5apex.exe!0x02341290 ConVar fx_impact_neutral
r5apex.exe!0x02397f40 ConVar fx_screenspacepass
r5apex.exe!0x02816070 ConVar fx_screenspacepass
r5apex.exe!0x02812ff0 ConVar g_debug_doors
r5apex.exe!0x02421d20 ConVar g_debug_flying_ai
r5apex.exe!0x02380b90 ConVar g_debug_ragdoll_removal
r5apex.exe!0x023f3080 ConVar g_debug_trackpather
r5apex.exe!0x02319a40 ConVar g_ragdoll_fadespeed
r5apex.exe!0x023802c0 ConVar g_ragdoll_important_maxcount
r5apex.exe!0x01f8d190 ConVar g_ragdoll_lvfadespeed
r5apex.exe!0x02330b50 ConVar gameCursor_ModeActive
r5apex.exe!0x02331960 ConVar gameCursor_Velocity
r5apex.exe!0x01f92710 ConVar gamepad_aim_speed
r5apex.exe!0x02324590 ConVar gamepad_aim_speed_ads_0
r5apex.exe!0x01f94350 ConVar gamepad_aim_speed_ads_1
r5apex.exe!0x01f82390 ConVar gamepad_aim_speed_ads_2
r5apex.exe!0x01f93d40 ConVar gamepad_aim_speed_ads_3
r5apex.exe!0x01f8d540 ConVar gamepad_aim_speed_ads_4
r5apex.exe!0x01f88ed0 ConVar gamepad_aim_speed_ads_5
r5apex.exe!0x023413d0 ConVar gamepad_button_layout
r5apex.exe!0x02340b00 ConVar gamepad_buttons_are_southpaw
r5apex.exe!0x01f93ac0 ConVar gamepad_custom_ads_pitch
r5apex.exe!0x01f88e30 ConVar gamepad_custom_ads_turn_delay
r5apex.exe!0x01f75700 ConVar gamepad_custom_ads_turn_pitch
r5apex.exe!0x0231af20 ConVar gamepad_custom_ads_turn_time
r5apex.exe!0x01f80530 ConVar gamepad_custom_ads_turn_yaw
r5apex.exe!0x02319120 ConVar gamepad_custom_ads_yaw
r5apex.exe!0x01f7f210 ConVar gamepad_custom_assist_on
r5apex.exe!0x01f90340 ConVar gamepad_custom_curve
r5apex.exe!0x01f92530 ConVar gamepad_custom_deadzone_in
r5apex.exe!0x01f75290 ConVar gamepad_custom_deadzone_out
r5apex.exe!0x01f8cb90 ConVar gamepad_custom_enabled
r5apex.exe!0x01f83370 ConVar gamepad_custom_hip_pitch
r5apex.exe!0x02318330 ConVar gamepad_custom_hip_turn_delay
r5apex.exe!0x0231a720 ConVar gamepad_custom_hip_turn_pitch
r5apex.exe!0x023194c0 ConVar gamepad_custom_hip_turn_time
r5apex.exe!0x01f92cf0 ConVar gamepad_custom_hip_turn_yaw
r5apex.exe!0x01f8d4a0 ConVar gamepad_custom_hip_yaw
r5apex.exe!0x0233d220 ConVar gamepad_custom_pilot
r5apex.exe!0x0233c310 ConVar gamepad_custom_titan
r5apex.exe!0x01f8ca50 ConVar gamepad_deadzone_index_look
r5apex.exe!0x01f85240 ConVar gamepad_deadzone_index_move
r5apex.exe!0x0233ba70 ConVar gamepad_enabled
r5apex.exe!0x01f93840 ConVar gamepad_look_curve
r5apex.exe!0x02343580 ConVar gamepad_stick_layout
r5apex.exe!0x02325ab0 ConVar gamepad_toggle_ads
r5apex.exe!0x023321f0 ConVar gamepad_togglecrouch_hold
r5apex.exe!0x018e4080 ConVar gamepad_trigger_threshold
r5apex.exe!0x01f93720 ConVar gamepad_use_per_scope_ads_settings
r5apex.exe!0x02345a10 ConVar gamepad_use_type
r5apex.exe!0x01757940 ConVar gameui_xbox
r5apex.exe!0x023427a0 ConVar gamma_adjusted
r5apex.exe!0x018e4440 ConVar gfx_desaturate_force
r5apex.exe!0x02347b50 ConVar gl_clear_color_buffer
r5apex.exe!0x02341890 ConVar gl_clear_fogcolor
r5apex.exe!0x023428e0 ConVar gl_clear_randomcolor
r5apex.exe!0x027f8200 ConVar glass_break_required_speed
r5apex.exe!0x024775c0 ConVar glass_shatter_attack_speed_scale
r5apex.exe!0x0233a6a0 ConVar glass_shatter_direction_force_scale
r5apex.exe!0x024761c0 ConVar glass_shatter_drop_speed
r5apex.exe!0x02475d30 ConVar glass_shatter_explosive_scale
r5apex.exe!0x0232e7e0 ConVar glass_shatter_force_scale
r5apex.exe!0x02333c10 ConVar glass_shatter_size_scale
r5apex.exe!0x0232cfc0 ConVar glass_shatter_use_real_direction
r5apex.exe!0x01f69e60 ConVar glitch_aberrationScale
r5apex.exe!0x023b8d60 ConVar globalNonRewindingObject_DontSave
r5apex.exe!0x01f668e0 ConVar global_lighting_partial_update
r5apex.exe!0x028af9a0 ConVar gpu_count
r5apex.exe!0x018e8b50 ConVar gpu_level
r5apex.exe!0x02323d90 ConVar gpu_level
r5apex.exe!0x018e48d0 ConVar gpu_mem_level
r5apex.exe!0x01f75660 ConVar gpu_mem_level
r5apex.exe!0x018e8830 ConVar gpu_vram_size_mb
r5apex.exe!0x023cf0e0 ConVar grapple_accel_human
r5apex.exe!0x023ce640 ConVar grapple_accel_titan
r5apex.exe!0x023d33f0 ConVar grapple_around_obstacle_accel
r5apex.exe!0x023d3f80 ConVar grapple_autoMantle
r5apex.exe!0x023cf220 ConVar grapple_autoMeleeConvergeTime
r5apex.exe!0x023cf400 ConVar grapple_autoMeleeOnDetach
r5apex.exe!0x023cefa0 ConVar grapple_autoMeleePredict
r5apex.exe!0x023d6e80 ConVar grapple_autoMeleePredictTime
r5apex.exe!0x023d6de0 ConVar grapple_autoMeleeViewRotateSpeedFar
r5apex.exe!0x023d6d40 ConVar grapple_autoMeleeViewRotateSpeedNear
r5apex.exe!0x023d2700 ConVar grapple_debug
r5apex.exe!0x023cdf60 ConVar grapple_decelMeleeStrength
r5apex.exe!0x023ced20 ConVar grapple_decel_human
r5apex.exe!0x023ce3c0 ConVar grapple_decel_titan
r5apex.exe!0x023ce500 ConVar grapple_detachExtraAllowedLength
r5apex.exe!0x023cfca0 ConVar grapple_disableMeleeWhenActive
r5apex.exe!0x023cea00 ConVar grapple_dontFightGravity
r5apex.exe!0x023cdec0 ConVar grapple_fallSpeed
r5apex.exe!0x023cf040 ConVar grapple_forcedRetractVel
r5apex.exe!0x023ce320 ConVar grapple_gracePeriod
r5apex.exe!0x0234b380 ConVar grapple_gravityPushUnderContribution
r5apex.exe!0x0239c180 ConVar grapple_gravityPushUnderContribution
r5apex.exe!0x023ce140 ConVar grapple_initialImpulseOffGround_human
r5apex.exe!0x023cf180 ConVar grapple_initialImpulseOffGround_human_npc
r5apex.exe!0x023ce8c0 ConVar grapple_initialImpulseOffGround_titan
r5apex.exe!0x023ce460 ConVar grapple_initialImpulse_human
r5apex.exe!0x023ce0a0 ConVar grapple_initialImpulse_titan
r5apex.exe!0x023ce780 ConVar grapple_initialSlowFracVert_human
r5apex.exe!0x023ce960 ConVar grapple_initialSlowFracVert_titan
r5apex.exe!0x023cee60 ConVar grapple_initialSlowFrac_human
r5apex.exe!0x023cedc0 ConVar grapple_initialSlowFrac_titan
r5apex.exe!0x023cec80 ConVar grapple_initialSpeedMin_human
r5apex.exe!0x023ce1e0 ConVar grapple_initialSpeedMin_titan
r5apex.exe!0x0236d8b0 ConVar grapple_jumpFrac
r5apex.exe!0x023b6920 ConVar grapple_jumpFrac
r5apex.exe!0x0236d950 ConVar grapple_letGravityHelpCosAngle
r5apex.exe!0x023b69c0 ConVar grapple_letGravityHelpCosAngle
r5apex.exe!0x023cebe0 ConVar grapple_lift
r5apex.exe!0x023d5fc0 ConVar grapple_pullDelay_human
r5apex.exe!0x023d5f20 ConVar grapple_pullDelay_titan
r5apex.exe!0x023cf2c0 ConVar grapple_retractVel
r5apex.exe!0x023cd1c0 ConVar grapple_rodeoVerticalImpulse
r5apex.exe!0x023d6f20 ConVar grapple_shootVel
r5apex.exe!0x023ceb40 ConVar grapple_speedRampMax_human
r5apex.exe!0x023cf360 ConVar grapple_speedRampMax_titan
r5apex.exe!0x023ce5a0 ConVar grapple_speedRampMin_human
r5apex.exe!0x023ce820 ConVar grapple_speedRampMin_titan
r5apex.exe!0x023cef00 ConVar grapple_speedRampTime_human
r5apex.exe!0x023ce280 ConVar grapple_speedRampTime_titan
r5apex.exe!0x023ceaa0 ConVar grapple_swingAngle
r5apex.exe!0x023d3b30 ConVar grapple_swingPullAngle
r5apex.exe!0x023ce6e0 ConVar grapple_swingPullSpeedLength
r5apex.exe!0x023ce000 ConVar grapple_swingPullSpeedScale
r5apex.exe!0x023bc1c0 ConVar grapple_titanEmbarkDist
r5apex.exe!0x023d4590 ConVar grapple_windowCheckDist
r5apex.exe!0x0237c410 ConVar gravity_grenade_decel
r5apex.exe!0x023e1600 ConVar gravity_grenade_decel
r5apex.exe!0x02376c90 ConVar gravity_grenade_projectile_min_speed
r5apex.exe!0x023dacf0 ConVar gravity_grenade_projectile_min_speed
r5apex.exe!0x02348820 ConVar ground_debug
r5apex.exe!0x0239a400 ConVar ground_debug
r5apex.exe!0x023d6910 ConVar ground_trace_hull_radius
r5apex.exe!0x0175d650 ConVar grx_hasUnknownItems
r5apex.exe!0x019ec500 ConVar gtao_angle_bias
r5apex.exe!0x019ebe20 ConVar gtao_intensity
r5apex.exe!0x019ec780 ConVar gtao_thickness_heuristic
r5apex.exe!0x01855d00 ConVar hasAnyAssetsWithDiscardedStreamableData
r5apex.exe!0x01856730 ConVar hasMic
r5apex.exe!0x01855c60 ConVar hasPartialInstall
r5apex.exe!0x019ec5a0 ConVar hbao_angle_bias
r5apex.exe!0x019ec0a0 ConVar hbao_intensity
r5apex.exe!0x019ec280 ConVar hbao_stepsize_random
r5apex.exe!0x019ec000 ConVar hbaobasic_tangent_bias
r5apex.exe!0x02423a10 ConVar hibernation_assumed_max_player_speed
r5apex.exe!0x024766d0 ConVar hibernation_debounce_dist
r5apex.exe!0x02400bd0 ConVar hibernation_enable
r5apex.exe!0x02476300 ConVar hibernation_far_dist
r5apex.exe!0x02475bf0 ConVar hibernation_medium_dist
r5apex.exe!0x024344f0 ConVar hibernation_min_reevaluate_time
r5apex.exe!0x02475eb0 ConVar hibernation_near_dist
r5apex.exe!0x023391a0 ConVar hidehud
r5apex.exe!0x028069c0 ConVar high_perf_dev_server
r5apex.exe!0x01f89e10 ConVar highlight_deferred_update
r5apex.exe!0x019eb560 ConVar highlight_draw
r5apex.exe!0x019eb740 ConVar highlight_lazy_clear_buffers
r5apex.exe!0x019eb6a0 ConVar highlight_object_max_count
r5apex.exe!0x01744ef0 ConVar hitbox_bodygroup_check
r5apex.exe!0x02342260 ConVar hitch_alert_active
r5apex.exe!0x02341630 ConVar hitch_alert_color
r5apex.exe!0x0233e730 ConVar hitch_alert_show_large_snapshots
r5apex.exe!0x01755fe0 ConVar host_RunFrameServerAlways
r5apex.exe!0x01753700 ConVar host_ShowIPCCallCount
r5apex.exe!0x018586e0 ConVar host_flush_threshold
r5apex.exe!0x01754100 ConVar host_framerate
r5apex.exe!0x01750df0 ConVar host_limitlocal
r5apex.exe!0x0174b750 ConVar host_map
r5apex.exe!0x01750050 ConVar host_preload_shaders
r5apex.exe!0x01756240 ConVar host_print_frame_times
r5apex.exe!0x0174f1f0 ConVar host_profile
r5apex.exe!0x01752f90 ConVar host_runframe_input_parcelremainder
r5apex.exe!0x017513f0 ConVar host_server_thread_min_ticks
r5apex.exe!0x0174b390 ConVar host_sleep
r5apex.exe!0x0174da50 ConVar host_speeds
r5apex.exe!0x0181f5d0 ConVar host_syncfps
r5apex.exe!0x0174ea70 ConVar host_thread_join_fast
r5apex.exe!0x01755d60 ConVar host_thread_mode
r5apex.exe!0x0174e320 ConVar host_threaded_sound
r5apex.exe!0x017574c0 ConVar host_timescale
r5apex.exe!0x0174eb80 ConVar hostip
r5apex.exe!0x0174f970 ConVar hostname
r5apex.exe!0x0174b6b0 ConVar hostport
r5apex.exe!0x018db8e0 ConVar http_StryderKey
r5apex.exe!0x018db340 ConVar http_debug
r5apex.exe!0x018db7a0 ConVar http_debug_forceFailRate
r5apex.exe!0x018db5c0 ConVar http_debug_forceFailStatus
r5apex.exe!0x018db3e0 ConVar http_failuresAsErrors
r5apex.exe!0x018db700 ConVar http_maxAllocateAttempts
r5apex.exe!0x018dbb60 ConVar http_recv_fail_realloc
r5apex.exe!0x018db660 ConVar http_sandbox
r5apex.exe!0x018db520 ConVar http_showQueries
r5apex.exe!0x023969c0 ConVar hud_autoreloadscript
r5apex.exe!0x02344e70 ConVar hud_setting_accessibleChat
r5apex.exe!0x0233ded0 ConVar hud_setting_adsDof
r5apex.exe!0x02385c00 ConVar hud_setting_compactOverHeadNames
r5apex.exe!0x02345c70 ConVar hud_setting_damageIndicatorStyle
r5apex.exe!0x02340de0 ConVar hud_setting_damageTextStyle
r5apex.exe!0x0233c590 ConVar hud_setting_enableModWheel
r5apex.exe!0x0233d780 ConVar hud_setting_healthUseOnHold
r5apex.exe!0x0233e2b0 ConVar hud_setting_healthWheelToggle
r5apex.exe!0x0233eb40 ConVar hud_setting_healthWheelUseOnRelease
r5apex.exe!0x02340350 ConVar hud_setting_lootPromptStyle
r5apex.exe!0x02340770 ConVar hud_setting_minimapRotate
r5apex.exe!0x0233cf20 ConVar hud_setting_ordnanceUseOnHold
r5apex.exe!0x02343c40 ConVar hud_setting_ordnanceWheelToggle
r5apex.exe!0x0233d340 ConVar hud_setting_ordnanceWheelUseOnRelease
r5apex.exe!0x02345b50 ConVar hud_setting_pingAlpha
r5apex.exe!0x02341ab0 ConVar hud_setting_pingDoubleTapEnemy
r5apex.exe!0x0233f3f0 ConVar hud_setting_pingWheelToggle
r5apex.exe!0x0233cb80 ConVar hud_setting_showButtonHints
r5apex.exe!0x02345590 ConVar hud_setting_showCallsigns
r5apex.exe!0x02344270 ConVar hud_setting_showLevelUp
r5apex.exe!0x02342a80 ConVar hud_setting_showMedals
r5apex.exe!0x023469f0 ConVar hud_setting_showMeter
r5apex.exe!0x02342d00 ConVar hud_setting_showObituary
r5apex.exe!0x0233e980 ConVar hud_setting_showTips
r5apex.exe!0x02346490 ConVar hud_setting_showWeaponFlyouts
r5apex.exe!0x02346550 ConVar hud_setting_streamerMode
r5apex.exe!0x0239a540 ConVar hudchat_dead_can_only_talk_to_other_dead
r5apex.exe!0x0232c8b0 ConVar hudchat_new_message_fade_duration
r5apex.exe!0x02333d50 ConVar hudchat_new_message_shown_duration
r5apex.exe!0x0232d1a0 ConVar hudchat_play_text_to_speech
r5apex.exe!0x0232c6f0 ConVar hudchat_transition_message_mode_fade_duration
r5apex.exe!0x0232b790 ConVar hudchat_visibility
r5apex.exe!0x01f6ffd0 ConVar hudwarp_chopsize
r5apex.exe!0x01f6fcb0 ConVar hudwarp_override
r5apex.exe!0x01f6fb70 ConVar hudwarp_viewDist
r5apex.exe!0x01f6ff30 ConVar hudwarp_xScale
r5apex.exe!0x01f6ec10 ConVar hudwarp_xWarp
r5apex.exe!0x01f6ecb0 ConVar hudwarp_yScale
r5apex.exe!0x01f6fc10 ConVar hudwarp_yWarp
r5apex.exe!0x0233dd40 ConVar idcolor_ally
r5apex.exe!0x02346c70 ConVar idcolor_ally_cb1
r5apex.exe!0x02344400 ConVar idcolor_ally_cb2
r5apex.exe!0x02344bd0 ConVar idcolor_ally_cb3
r5apex.exe!0x023478e0 ConVar idcolor_enemy
r5apex.exe!0x02343740 ConVar idcolor_enemy_cb1
r5apex.exe!0x02342e60 ConVar idcolor_enemy_cb2
r5apex.exe!0x0233c660 ConVar idcolor_enemy_cb3
r5apex.exe!0x0233e210 ConVar idcolor_neutral
r5apex.exe!0x02421e60 ConVar idleKickTime_min_alive_seconds
r5apex.exe!0x02400c70 ConVar idleKickTime_minutes
r5apex.exe!0x02476810 ConVar idleKickTime_party_minutes
r5apex.exe!0x024230b0 ConVar idleKickTime_privatematch_game_minutes
r5apex.exe!0x023fe870 ConVar idleKickTime_privatematch_lobby_minutes
r5apex.exe!0x023feb00 ConVar idleKickTime_training_minutes
r5apex.exe!0x023d66b0 ConVar ik_debug
r5apex.exe!0x023d1b80 ConVar ik_debug_chain
r5apex.exe!0x023d3890 ConVar ik_debug_ent
r5apex.exe!0x023d5c10 ConVar ik_debug_text
r5apex.exe!0x023d1c20 ConVar ik_enable
r5apex.exe!0x0236d810 ConVar ik_enable_client
r5apex.exe!0x023b54e0 ConVar ik_enable_server
r5apex.exe!0x023bc260 ConVar ik_height_adjust
r5apex.exe!0x0239b980 ConVar ik_height_adjust_debug
r5apex.exe!0x0239a990 ConVar ik_height_adjust_move_speed
r5apex.exe!0x02398d40 ConVar ik_height_adjust_sine
r5apex.exe!0x023cd300 ConVar ik_height_adjust_speed
r5apex.exe!0x023d4e10 ConVar ik_latch
r5apex.exe!0x023d1840 ConVar ik_normal_lerp_rate
r5apex.exe!0x023d2c40 ConVar ik_unlatch_max_rate
r5apex.exe!0x018e25a0 ConVar ime_enabled
r5apex.exe!0x01f6ded0 ConVar imgui_buildmode
r5apex.exe!0x01f6dfa0 ConVar imgui_buildmode
r5apex.exe!0x02337180 ConVar impact_allow
r5apex.exe!0x02342da0 ConVar impact_debug_info
r5apex.exe!0x02339100 ConVar impact_victim_offset_dist
r5apex.exe!0x023741b0 ConVar impulse_low_decel_duration_scalar
r5apex.exe!0x023d7340 ConVar impulse_low_decel_duration_scalar
r5apex.exe!0x01856870 ConVar inPartyChat
r5apex.exe!0x01751d80 ConVar in_forceuser
r5apex.exe!0x0175a790 ConVar in_syncRT
r5apex.exe!0x023255f0 ConVar in_usekeyboardsampletime
r5apex.exe!0x0185bc50 ConVar inbox_enabled
r5apex.exe!0x028033f0 ConVar info_spawnpoint_human_classname
r5apex.exe!0x02813970 ConVar info_spawnpoint_titan_classname
r5apex.exe!0x0185b930 ConVar infoblock_requestInterval
r5apex.exe!0x01f84150 ConVar interpolate_on_parent_change
r5apex.exe!0x0233ea20 ConVar intro_viewed
r5apex.exe!0x02370120 ConVar invalidate_opt
r5apex.exe!0x023ba410 ConVar invalidate_opt
r5apex.exe!0x01753170 ConVar ip
r5apex.exe!0x02337e10 ConVar joy_advaxisr
r5apex.exe!0x02326610 ConVar joy_advaxisu
r5apex.exe!0x0233a8c0 ConVar joy_advaxisv
r5apex.exe!0x02332640 ConVar joy_advaxisx
r5apex.exe!0x02335440 ConVar joy_advaxisy
r5apex.exe!0x02332290 ConVar joy_advaxisz
r5apex.exe!0x02339b00 ConVar joy_inverty
r5apex.exe!0x0233bf70 ConVar joy_legacy
r5apex.exe!0x0232d4a0 ConVar joy_movement_stick
r5apex.exe!0x02325810 ConVar joy_requireFocus
r5apex.exe!0x02335ec0 ConVar joy_rumble
r5apex.exe!0x023379e0 ConVar joy_xcontroller_cfg_loaded
r5apex.exe!0x0181ebb0 ConVar jpeg_quality
r5apex.exe!0x02371b30 ConVar jump_graceperiod
r5apex.exe!0x023bc4e0 ConVar jump_graceperiod
r5apex.exe!0x02370760 ConVar jump_keyboardgrace_max
r5apex.exe!0x023bab00 ConVar jump_keyboardgrace_max
r5apex.exe!0x0235bff0 ConVar jump_keyboardgrace_strength
r5apex.exe!0x023ac930 ConVar jump_keyboardgrace_strength
r5apex.exe!0x0234b2e0 ConVar jump_keyboardgraceperiodmax
r5apex.exe!0x0239c0e0 ConVar jump_keyboardgraceperiodmax
r5apex.exe!0x023708a0 ConVar jump_keyboardgraceperiodmin
r5apex.exe!0x023bac40 ConVar jump_keyboardgraceperiodmin
r5apex.exe!0x01857880 ConVar killReplay_lagCompensate
r5apex.exe!0x02387fd0 ConVar killReplay_playNonReplayRemoteCallsOnLocalClientPlayer
r5apex.exe!0x028107b0 ConVar lagcompensation_debug_ent
r5apex.exe!0x027f83e0 ConVar lagcompensation_ignore_friendlies
r5apex.exe!0x01f92e60 ConVar leaf_threadedRecompute
r5apex.exe!0x01f8c830 ConVar leaf_threadedRecompute_batchSize
r5apex.exe!0x0281ae30 ConVar leech_lagcompensate
r5apex.exe!0x02827040 ConVar leech_npc_angle_cos
r5apex.exe!0x01f8fbf0 ConVar lerp_careAboutAttachmentBonePosition
r5apex.exe!0x023f0320 ConVar lerp_debugEnt
r5apex.exe!0x023f8530 ConVar lerp_debugEnt_server
r5apex.exe!0x01f97650 ConVar lerp_opt
r5apex.exe!0x01f757a0 ConVar lerp_threaded
r5apex.exe!0x01f8efa0 ConVar lerp_threaded_numEntsPerTask
r5apex.exe!0x0174c5b0 ConVar light_maxcone
r5apex.exe!0x018e5160 ConVar lightmap_realtimelight
r5apex.exe!0x018e8680 ConVar lightmap_realtimeshadows
r5apex.exe!0x01753540 ConVar load_during_video
r5apex.exe!0x02384820 ConVar loaderrorsCount
r5apex.exe!0x023841c0 ConVar loaderrorsNeedShown
r5apex.exe!0x02384d20 ConVar localClientPlayerCachedLevel
r5apex.exe!0x01820440 ConVar locationInfo
r5apex.exe!0x018203a0 ConVar locationInfo_nucleus
r5apex.exe!0x0232b970 ConVar locator_background_border_color
r5apex.exe!0x023254d0 ConVar locator_background_border_thickness
r5apex.exe!0x0232dda0 ConVar locator_background_color
r5apex.exe!0x0232dfe0 ConVar locator_background_shift_x
r5apex.exe!0x0232edf0 ConVar locator_background_shift_y
r5apex.exe!0x023323d0 ConVar locator_background_style
r5apex.exe!0x0232ecd0 ConVar locator_background_thickness_x
r5apex.exe!0x02326230 ConVar locator_background_thickness_y
r5apex.exe!0x0232cc70 ConVar locator_fade_time
r5apex.exe!0x023399e0 ConVar locator_icon_max_size_non_ss
r5apex.exe!0x02332c40 ConVar locator_icon_min_size_non_ss
r5apex.exe!0x0232ce30 ConVar locator_lerp_rest
r5apex.exe!0x0232f690 ConVar locator_lerp_speed
r5apex.exe!0x023318c0 ConVar locator_lerp_time
r5apex.exe!0x02335b40 ConVar locator_pulse_time
r5apex.exe!0x02325bd0 ConVar locator_split_len
r5apex.exe!0x0232f960 ConVar locator_split_maxwide_percent
r5apex.exe!0x023354e0 ConVar locator_start_at_crosshair
r5apex.exe!0x023338c0 ConVar locator_target_offset_x
r5apex.exe!0x0232afa0 ConVar locator_target_offset_y
r5apex.exe!0x02338310 ConVar locator_topdown_style
r5apex.exe!0x02333fa0 ConVar lookspring
r5apex.exe!0x02338b10 ConVar lookstrafe
r5apex.exe!0x0232e900 ConVar m_acceleration
r5apex.exe!0x023389f0 ConVar m_forward
r5apex.exe!0x0232d100 ConVar m_invert_pitch
r5apex.exe!0x023325a0 ConVar m_side
r5apex.exe!0x023844c0 ConVar mainmenu_background_movie
r5apex.exe!0x02337ca0 ConVar map_settings_override
r5apex.exe!0x01754bc0 ConVar map_wants_save_disable
r5apex.exe!0x01f66b60 ConVar mat_autoexposure_force_value
r5apex.exe!0x02344f10 ConVar mat_autoexposure_max
r5apex.exe!0x02341b50 ConVar mat_autoexposure_max_multiplier
r5apex.exe!0x02346690 ConVar mat_autoexposure_min
r5apex.exe!0x02342840 ConVar mat_autoexposure_min_multiplier
r5apex.exe!0x0233f350 ConVar mat_autoexposure_override_min_max
r5apex.exe!0x0233fa20 ConVar mat_autoexposure_speed
r5apex.exe!0x02347500 ConVar mat_autoexposure_uncap
r5apex.exe!0x02346210 ConVar mat_bloom_cutoff
r5apex.exe!0x01f66ca0 ConVar mat_bloom_max_lighting_value
r5apex.exe!0x0233e050 ConVar mat_bloom_scalefactor_scalar
r5apex.exe!0x01f68ce0 ConVar mat_bloom_streak_amount
r5apex.exe!0x02343b00 ConVar mat_bloom_streak_cutoff
r5apex.exe!0x02342bc0 ConVar mat_bloom_streak_cutoff_exposure_adapt
r5apex.exe!0x01f69d20 ConVar mat_bloom_streak_exponent_post
r5apex.exe!0x02342120 ConVar mat_bloom_streak_exponent_pre
r5apex.exe!0x01f69820 ConVar mat_bloom_wide_amount
r5apex.exe!0x02343900 ConVar mat_bloom_wide_exponent_pre
r5apex.exe!0x02345e70 ConVar mat_bloomamount_rate
r5apex.exe!0x02347c90 ConVar mat_bloomscale
r5apex.exe!0x019ed0f0 ConVar mat_checkStalls
r5apex.exe!0x018e5480 ConVar mat_cloudmask
r5apex.exe!0x01749400 ConVar mat_colcorrection_disableentities
r5apex.exe!0x01856f50 ConVar mat_colcorrection_disableentities
r5apex.exe!0x02318150 ConVar mat_colcorrection_disableentities
r5apex.exe!0x01747e10 ConVar mat_colcorrection_editor
r5apex.exe!0x01f87f70 ConVar mat_colcorrection_editor
r5apex.exe!0x01f88bb0 ConVar mat_colcorrection_forceentitiesclientside
r5apex.exe!0x017484d0 ConVar mat_colorcorrection
r5apex.exe!0x01f69dc0 ConVar mat_debug_postprocess_allowed
r5apex.exe!0x02344060 ConVar mat_debug_postprocessing_effects
r5apex.exe!0x01f69b40 ConVar mat_debug_tonemapping
r5apex.exe!0x01f69640 ConVar mat_debug_tonemapping_disable
r5apex.exe!0x01f69500 ConVar mat_debug_tonemapping_mid1
r5apex.exe!0x01f691e0 ConVar mat_debug_tonemapping_mid2
r5apex.exe!0x01f693c0 ConVar mat_debug_tonemapping_shoulder
r5apex.exe!0x01f69320 ConVar mat_debug_tonemapping_toe
r5apex.exe!0x018e88d0 ConVar mat_debugalttab
r5apex.exe!0x01f67780 ConVar mat_depthbias_decal
r5apex.exe!0x01f67960 ConVar mat_depthbias_normal
r5apex.exe!0x01f67640 ConVar mat_depthbias_shadowmap
r5apex.exe!0x01f67820 ConVar mat_depthbias_tightshadowmap
r5apex.exe!0x01f67320 ConVar mat_depthbias_ui
r5apex.exe!0x01f678c0 ConVar mat_depthbias_zfill
r5apex.exe!0x01f671e0 ConVar mat_depthbiasclamp_decal
r5apex.exe!0x01f67be0 ConVar mat_depthbiasclamp_normal
r5apex.exe!0x01f673c0 ConVar mat_depthbiasclamp_shadowmap
r5apex.exe!0x01f675a0 ConVar mat_depthbiasclamp_ui
r5apex.exe!0x01f67500 ConVar mat_depthbiasclamp_zfill
r5apex.exe!0x01f6a040 ConVar mat_depthfeather_enable
r5apex.exe!0x01f67280 ConVar mat_depthtest_force_disabled
r5apex.exe!0x018e84a0 ConVar mat_detail_tex
r5apex.exe!0x018e46f0 ConVar mat_diffuse
r5apex.exe!0x023434e0 ConVar mat_disable_bloom
r5apex.exe!0x01f66ac0 ConVar mat_disable_lightmap_ambient
r5apex.exe!0x018e7780 ConVar mat_disable_lightmaps
r5apex.exe!0x018e4c90 ConVar mat_disable_model_ambient
r5apex.exe!0x01758b10 ConVar mat_drawMenuGrid
r5apex.exe!0x0175b250 ConVar mat_drawTitleSafe
r5apex.exe!0x018e4970 ConVar mat_drawflat
r5apex.exe!0x018e53e0 ConVar mat_dxlevel
r5apex.exe!0x01747eb0 ConVar mat_dynamic_tonemapping
r5apex.exe!0x018e8c90 ConVar mat_dynamic_tonemapping
r5apex.exe!0x018e44e0 ConVar mat_enable_ssr
r5apex.exe!0x01f66d40 ConVar mat_envmap_scale
r5apex.exe!0x028b1c60 ConVar mat_envmap_scale
r5apex.exe!0x01747120 ConVar mat_envmaptgasize
r5apex.exe!0x018e7fa0 ConVar mat_fastnobump
r5apex.exe!0x01753840 ConVar mat_fastspecular
r5apex.exe!0x018e8360 ConVar mat_filterlightmaps
r5apex.exe!0x018e4ab0 ConVar mat_filtertextures
r5apex.exe!0x0233e820 ConVar mat_force_bloom
r5apex.exe!0x018e7f00 ConVar mat_forceaniso
r5apex.exe!0x0233f2b0 ConVar mat_frame_color_bias
r5apex.exe!0x02341530 ConVar mat_frame_color_enabled
r5apex.exe!0x02347640 ConVar mat_frame_color_scale
r5apex.exe!0x0233fc60 ConVar mat_frame_color_spot_metering_screen_ratio
r5apex.exe!0x01751830 ConVar mat_fullbright
r5apex.exe!0x01f69960 ConVar mat_fxaa_enable
r5apex.exe!0x018e4650 ConVar mat_global_lighting
r5apex.exe!0x019ed3f0 ConVar mat_global_lighting
r5apex.exe!0x0233c450 ConVar mat_global_lighting
r5apex.exe!0x01750f10 ConVar mat_hdr_level
r5apex.exe!0x01856ff0 ConVar mat_hdrcolcorrection_editor
r5apex.exe!0x018e4310 ConVar mat_hdrcolorcorrection
r5apex.exe!0x019ed540 ConVar mat_hide_sun_in_last_cascade
r5apex.exe!0x019ed4a0 ConVar mat_instancing
r5apex.exe!0x019ed050 ConVar mat_letterbox_aspect_goal
r5apex.exe!0x019ecfb0 ConVar mat_letterbox_aspect_threshold
r5apex.exe!0x0233d6c0 ConVar mat_lightcull_subview
r5apex.exe!0x023403f0 ConVar mat_lightcull_subviews
r5apex.exe!0x01f69be0 ConVar mat_local_contrast_edge_scale_override
r5apex.exe!0x01f68f60 ConVar mat_local_contrast_midtone_mask_override
r5apex.exe!0x01f68e20 ConVar mat_local_contrast_scale_override
r5apex.exe!0x01f69000 ConVar mat_local_contrast_vignette_end_override
r5apex.exe!0x01f690a0 ConVar mat_local_contrast_vignette_start_override
r5apex.exe!0x018e9c50 ConVar mat_materialmip_character_0
r5apex.exe!0x018e8fb0 ConVar mat_materialmip_character_1
r5apex.exe!0x018e9370 ConVar mat_materialmip_character_2
r5apex.exe!0x018e9730 ConVar mat_materialmip_character_3
r5apex.exe!0x018e9690 ConVar mat_materialmip_character_4
r5apex.exe!0x018e99d0 ConVar mat_materialmip_cockpit_0
r5apex.exe!0x018e9190 ConVar mat_materialmip_cockpit_1
r5apex.exe!0x018e8dd0 ConVar mat_materialmip_cockpit_2
r5apex.exe!0x018e8f10 ConVar mat_materialmip_cockpit_3
r5apex.exe!0x018e9b10 ConVar mat_materialmip_cockpit_4
r5apex.exe!0x018e8e70 ConVar mat_materialmip_model_0
r5apex.exe!0x018e94b0 ConVar mat_materialmip_model_1
r5apex.exe!0x018e9bb0 ConVar mat_materialmip_model_2
r5apex.exe!0x018e9ed0 ConVar mat_materialmip_model_3
r5apex.exe!0x018e95f0 ConVar mat_materialmip_model_4
r5apex.exe!0x018e9a70 ConVar mat_materialmip_other_0
r5apex.exe!0x018e92d0 ConVar mat_materialmip_other_1
r5apex.exe!0x018e9f70 ConVar mat_materialmip_other_2
r5apex.exe!0x018e9050 ConVar mat_materialmip_other_3
r5apex.exe!0x018e9d90 ConVar mat_materialmip_other_4
r5apex.exe!0x018e9550 ConVar mat_materialmip_world_0
r5apex.exe!0x018e8d30 ConVar mat_materialmip_world_1
r5apex.exe!0x018e90f0 ConVar mat_materialmip_world_2
r5apex.exe!0x018e9cf0 ConVar mat_materialmip_world_3
r5apex.exe!0x018e9230 ConVar mat_materialmip_world_4
r5apex.exe!0x0174d750 ConVar mat_maxframelatency
r5apex.exe!0x018e52a0 ConVar mat_mip_linear
r5apex.exe!0x018e5520 ConVar mat_mipmaptextures
r5apex.exe!0x0174af50 ConVar mat_norendering
r5apex.exe!0x018e4790 ConVar mat_norendering
r5apex.exe!0x018e7aa0 ConVar mat_phong
r5apex.exe!0x018e4d30 ConVar mat_picmip
r5apex.exe!0x01f698c0 ConVar mat_postprocess_enable
r5apex.exe!0x023450d0 ConVar mat_postprocess_enable
r5apex.exe!0x019ea120 ConVar mat_processtoolvars
r5apex.exe!0x018e8180 ConVar mat_proxy
r5apex.exe!0x018e8220 ConVar mat_reducefillrate
r5apex.exe!0x018ea010 ConVar mat_reduceparticles
r5apex.exe!0x01f67140 ConVar mat_remoteshadercompile
r5apex.exe!0x018e8bf0 ConVar mat_report_queue_status
r5apex.exe!0x018e7640 ConVar mat_reversedepth
r5apex.exe!0x0233f120 ConVar mat_screen_blur_enabled
r5apex.exe!0x01f69280 ConVar mat_screen_blur_override
r5apex.exe!0x0174d250 ConVar mat_shadowstate
r5apex.exe!0x01f68c40 ConVar mat_sharpen_amount
r5apex.exe!0x01f69780 ConVar mat_sharpen_threshold
r5apex.exe!0x01f69aa0 ConVar mat_sharpen_width
r5apex.exe!0x01855410 ConVar mat_show_texture_memory_usage
r5apex.exe!0x019ea490 ConVar mat_showenvmapmask
r5apex.exe!0x018e45b0 ConVar mat_showlowresimage
r5apex.exe!0x018e7a00 ConVar mat_showmiplevels
r5apex.exe!0x017500f0 ConVar mat_skipid
r5apex.exe!0x01753400 ConVar mat_sky_color
r5apex.exe!0x0174fc90 ConVar mat_sky_scale
r5apex.exe!0x01f67b40 ConVar mat_slopescaledepthbias_decal
r5apex.exe!0x01f67460 ConVar mat_slopescaledepthbias_normal
r5apex.exe!0x01f67aa0 ConVar mat_slopescaledepthbias_shadowmap
r5apex.exe!0x01f676e0 ConVar mat_slopescaledepthbias_ui
r5apex.exe!0x01f67a00 ConVar mat_slopescaledepthbias_zfill
r5apex.exe!0x017532b0 ConVar mat_sun_color
r5apex.exe!0x0174e030 ConVar mat_sun_scale
r5apex.exe!0x01750690 ConVar mat_surfacefilter
r5apex.exe!0x0174f6f0 ConVar mat_surfaceid
r5apex.exe!0x01753980 ConVar mat_surfacemat
r5apex.exe!0x019ed2b0 ConVar mat_syncGPU
r5apex.exe!0x019ed210 ConVar mat_syncInterval
r5apex.exe!0x018e7c80 ConVar mat_sync_rt
r5apex.exe!0x019ea300 ConVar mat_sync_rt_flushes_gpu
r5apex.exe!0x018552d0 ConVar mat_texture_list
r5apex.exe!0x01855370 ConVar mat_texture_list_view
r5apex.exe!0x019ea3a0 ConVar mat_translucency_errors
r5apex.exe!0x01f69f00 ConVar mat_use_compressed_hdr_textures
r5apex.exe!0x01f69c80 ConVar mat_vignette_enable
r5apex.exe!0x018e9410 ConVar mat_warn_texture_convert
r5apex.exe!0x0174c760 ConVar match_backingOutMaxTimeToWait
r5apex.exe!0x01753030 ConVar match_backoutslow
r5apex.exe!0x0174ba10 ConVar match_connect
r5apex.exe!0x0174dd70 ConVar match_defaultMap_party
r5apex.exe!0x01753b60 ConVar match_dir
r5apex.exe!0x0174e950 ConVar match_dumpSearchResults
r5apex.exe!0x0174b2f0 ConVar match_emptyUpdateRate
r5apex.exe!0x017549e0 ConVar match_enabled
r5apex.exe!0x0174dc30 ConVar match_fakePort
r5apex.exe!0x0174b4d0 ConVar match_fakeS2SPort
r5apex.exe!0x01756380 ConVar match_forceVerboseSearches
r5apex.exe!0x01750af0 ConVar match_goodReputation
r5apex.exe!0x01751670 ConVar match_maxPingsSent
r5apex.exe!0x01755a60 ConVar match_mixtape_unchecked
r5apex.exe!0x0174d860 ConVar match_mixtape_unchecked_version
r5apex.exe!0x01756720 ConVar match_mixtape_version
r5apex.exe!0x0174e810 ConVar match_mixtape_warnOnPlay
r5apex.exe!0x01753a20 ConVar match_myBestDatacenter
r5apex.exe!0x01757420 ConVar match_myDatacenter
r5apex.exe!0x01753d40 ConVar match_myRankedDatacenter
r5apex.exe!0x0174c040 ConVar match_myTeam
r5apex.exe!0x01752b40 ConVar match_partyChangeNum
r5apex.exe!0x0174be90 ConVar match_partySize
r5apex.exe!0x0174b610 ConVar match_partySub
r5apex.exe!0x0174ab30 ConVar match_pingWaveInterval
r5apex.exe!0x017548a0 ConVar match_playlist
r5apex.exe!0x0174cca0 ConVar match_precachemap
r5apex.exe!0x0174e280 ConVar match_privateMatchListWithStryder
r5apex.exe!0x0174f5b0 ConVar match_rankedMaxPing
r5apex.exe!0x0174c8a0 ConVar match_rankedSwitchETA
r5apex.exe!0x01754460 ConVar match_resetPlaylistBetweenMatches
r5apex.exe!0x0174f830 ConVar match_searchInterval
r5apex.exe!0x01754ee0 ConVar match_searching
r5apex.exe!0x017518d0 ConVar match_teamNoFill
r5apex.exe!0x01752aa0 ConVar match_updateNotableRate
r5apex.exe!0x01752d20 ConVar match_updateRate
r5apex.exe!0x01750fb0 ConVar match_useMatchmaking
r5apex.exe!0x017543c0 ConVar match_verbosePrintsInterval
r5apex.exe!0x0174e630 ConVar match_visiblePlaylists
r5apex.exe!0x01750550 ConVar matchmaking_hostname
r5apex.exe!0x023b67e0 ConVar matchresults_write_enabled
r5apex.exe!0x023ec250 ConVar max_explosive_damage_mass
r5apex.exe!0x023edf40 ConVar max_explosive_damage_velocity
r5apex.exe!0x01f75840 ConVar max_tweak_shadow_updates
r5apex.exe!0x0238b8b0 ConVar melee_aim_assist_can_lock_pitch
r5apex.exe!0x0281bdf0 ConVar melee_aim_assist_can_lock_pitch
r5apex.exe!0x02394900 ConVar melee_aim_assist_use_target_velocity
r5apex.exe!0x02388e50 ConVar melee_attack_trace_can_use_lunge_distance
r5apex.exe!0x02819020 ConVar melee_attack_trace_can_use_lunge_distance
r5apex.exe!0x0238a9f0 ConVar melee_cone_trace_box_check
r5apex.exe!0x0281ad90 ConVar melee_cone_trace_box_check
r5apex.exe!0x0281ced0 ConVar melee_cone_trace_lag_compensate_user_command_target
r5apex.exe!0x0236fae0 ConVar melee_lunge_abort_distance
r5apex.exe!0x023b9630 ConVar melee_lunge_abort_distance
r5apex.exe!0x0234ae30 ConVar melee_lunge_abort_if_blocked
r5apex.exe!0x0239b840 ConVar melee_lunge_abort_if_blocked
r5apex.exe!0x02390e40 ConVar melee_lunge_adjust_trace_distance
r5apex.exe!0x02821f90 ConVar melee_lunge_adjust_trace_distance
r5apex.exe!0x02391e60 ConVar melee_lunge_align_eye_position
r5apex.exe!0x028226d0 ConVar melee_lunge_align_eye_position
r5apex.exe!0x0238b330 ConVar melee_lunge_dot_check
r5apex.exe!0x0281b7a0 ConVar melee_lunge_dot_check
r5apex.exe!0x023720b0 ConVar melee_lunge_force_enable_flying
r5apex.exe!0x023ccaa0 ConVar melee_lunge_force_enable_flying
r5apex.exe!0x023d5db0 ConVar melee_lunge_lag_compensate_target
r5apex.exe!0x02826b90 ConVar melee_lunge_scale_by_speed
r5apex.exe!0x0234bd40 ConVar melee_lunge_slide
r5apex.exe!0x0239c540 ConVar melee_lunge_slide
r5apex.exe!0x02348780 ConVar melee_lunge_use_closest_distance_between_cylinders
r5apex.exe!0x02398e80 ConVar melee_lunge_use_closest_distance_between_cylinders
r5apex.exe!0x02827550 ConVar melee_lunge_use_command_time
r5apex.exe!0x023ec1b0 ConVar melee_queue_attack_anim_event
r5apex.exe!0x0281ab10 ConVar melee_titan_execution_attacker_can_be_ref
r5apex.exe!0x017572e0 ConVar mem_dumpstats
r5apex.exe!0x01746900 ConVar mem_force_flush
r5apex.exe!0x01746860 ConVar mem_force_flush_section
r5apex.exe!0x0174d390 ConVar mem_incremental_compact_rate
r5apex.exe!0x018e4f80 ConVar mem_level
r5apex.exe!0x01f82df0 ConVar mem_level
r5apex.exe!0x01759fb0 ConVar mem_max_heapsize
r5apex.exe!0x01758490 ConVar mem_max_heapsize_dedicated
r5apex.exe!0x01757d70 ConVar mem_min_heapsize
r5apex.exe!0x01f6b8a0 ConVar mem_runheapchecks
r5apex.exe!0x01750230 ConVar mem_test_each_frame
r5apex.exe!0x01752640 ConVar mem_test_every_n_seconds
r5apex.exe!0x0174cf60 ConVar mem_test_quiet
r5apex.exe!0x02340830 ConVar menu_faq_community_version
r5apex.exe!0x0233e5f0 ConVar menu_faq_patchnotes_version
r5apex.exe!0x0233c4f0 ConVar menu_faq_viewed
r5apex.exe!0x0233f5f0 ConVar menu_was_multiplayer_played_last
r5apex.exe!0x018dca60 ConVar migrate_attempt_interval
r5apex.exe!0x02387050 ConVar miles_actor_occlusion_radius
r5apex.exe!0x02387ad0 ConVar miles_channels
r5apex.exe!0x01f88f70 ConVar miles_flip_active_window_logic
r5apex.exe!0x02385860 ConVar miles_force_emitter_environment
r5apex.exe!0x02386f10 ConVar miles_force_listener_environment
r5apex.exe!0x023887d0 ConVar miles_freeze
r5apex.exe!0x02387e90 ConVar miles_initial_occlusion_delay
r5apex.exe!0x02387920 ConVar miles_language
r5apex.exe!0x02385a40 ConVar miles_listener_freeze
r5apex.exe!0x0175bd40 ConVar miles_max_sound_commands_per_server_frame
r5apex.exe!0x02387190 ConVar miles_nonactor_occlusion
r5apex.exe!0x023880b0 ConVar miles_nonactor_occlusion_radius
r5apex.exe!0x023872d0 ConVar miles_nopandist
r5apex.exe!0x02387f30 ConVar miles_occlusion
r5apex.exe!0x02387620 ConVar miles_occlusion_force
r5apex.exe!0x02385e80 ConVar miles_occlusion_partial
r5apex.exe!0x02386b50 ConVar miles_occlusion_use_reset_after_deferred_initial
r5apex.exe!0x02388690 ConVar miles_samplerate
r5apex.exe!0x0282a340 ConVar miles_server_disable_sounds
r5apex.exe!0x0175d3a0 ConVar miles_server_sounds_debug
r5apex.exe!0x0175d440 ConVar miles_server_sounds_print
r5apex.exe!0x0282a3e0 ConVar miles_server_useSoundIDTable
r5apex.exe!0x023859a0 ConVar miles_solo_ents
r5apex.exe!0x02329bb0 ConVar miles_soundscape_imgui
r5apex.exe!0x023868f0 ConVar miles_spatialize_front_degrees
r5apex.exe!0x023881f0 ConVar miles_spatialize_offplane_strength
r5apex.exe!0x02386ab0 ConVar miles_spatialize_on
r5apex.exe!0x02387410 ConVar miles_spatialize_rear_degrees
r5apex.exe!0x02388400 ConVar miles_suffixes
r5apex.exe!0x023ed520 ConVar min_explosive_damage_mass
r5apex.exe!0x0281b660 ConVar missile_debug_draw
r5apex.exe!0x02394a40 ConVar missile_default_speed
r5apex.exe!0x02824e40 ConVar missile_default_speed
r5apex.exe!0x02393a00 ConVar missile_homing_speed
r5apex.exe!0x02823f90 ConVar missile_homing_speed
r5apex.exe!0x01744c30 ConVar mod_check_vcollide
r5apex.exe!0x01744e50 ConVar mod_trace_load
r5apex.exe!0x0175b9a0 ConVar model_defaultFadeDistMin
r5apex.exe!0x02324950 ConVar model_defaultFadeDistMin
r5apex.exe!0x01758750 ConVar model_defaultFadeDistScale
r5apex.exe!0x01f83e50 ConVar model_defaultFadeDistScale
r5apex.exe!0x02325390 ConVar model_fadeRangeFraction
r5apex.exe!0x02325430 ConVar model_fadeRangeFractionNear
r5apex.exe!0x01f6ad70 ConVar modeldecals_forceAllowed
r5apex.exe!0x0233d3e0 ConVar monitor_cc
r5apex.exe!0x01f68d80 ConVar monitor_mat_sharpen_amount
r5apex.exe!0x0233f490 ConVar monitor_postfx
r5apex.exe!0x02342c60 ConVar monitor_rui_world_enabled
r5apex.exe!0x01f75520 ConVar monitor_snapshot_frame_delay
r5apex.exe!0x01f805d0 ConVar monitor_zfar_default
r5apex.exe!0x023423c0 ConVar monitor_zfar_override
r5apex.exe!0x0233c3b0 ConVar monitor_zfar_override_enabled
r5apex.exe!0x0174fa10 ConVar motd
r5apex.exe!0x02338430 ConVar mouse_sensitivity
r5apex.exe!0x0232cf20 ConVar mouse_use_per_scope_sensitivity_scalars
r5apex.exe!0x02333a80 ConVar mouse_zoomed_sensitivity_scalar_0
r5apex.exe!0x023260f0 ConVar mouse_zoomed_sensitivity_scalar_1
r5apex.exe!0x0232b8d0 ConVar mouse_zoomed_sensitivity_scalar_2
r5apex.exe!0x02334420 ConVar mouse_zoomed_sensitivity_scalar_3
r5apex.exe!0x0232bb10 ConVar mouse_zoomed_sensitivity_scalar_4
r5apex.exe!0x02337c00 ConVar mouse_zoomed_sensitivity_scalar_5
r5apex.exe!0x01757580 ConVar move_one_cmd_per_client_frame
r5apex.exe!0x02817cb0 ConVar movement_anim_downed_playback_maxrate
r5apex.exe!0x02817df0 ConVar movement_anim_playback_maxrate
r5apex.exe!0x02817990 ConVar movement_anim_playback_minrate
r5apex.exe!0x02817a30 ConVar movement_anim_sprint_playback_maxrate
r5apex.exe!0x01754c60 ConVar mp_accountLink_requestInterval
r5apex.exe!0x02476590 ConVar mp_allowNPCs
r5apex.exe!0x0174bd00 ConVar mp_allowed
r5apex.exe!0x02817d50 ConVar mp_bodyyawrate
r5apex.exe!0x023baa60 ConVar mp_class_max_dronecontroller
r5apex.exe!0x023cdc40 ConVar mp_class_max_fireteam
r5apex.exe!0x023b6880 ConVar mp_class_max_pilot
r5apex.exe!0x023bb220 ConVar mp_class_max_titan
r5apex.exe!0x01856300 ConVar mp_countRRNobodyAsLobby
r5apex.exe!0x023fe910 ConVar mp_defaultteam
r5apex.exe!0x023d5330 ConVar mp_enablematchending
r5apex.exe!0x023d1e20 ConVar mp_enabletimelimit
r5apex.exe!0x02475f50 ConVar mp_fraglimit
r5apex.exe!0x023d6fc0 ConVar mp_gamemode
r5apex.exe!0x0185aca0 ConVar mp_huge_threshhold
r5apex.exe!0x018dee00 ConVar mp_linkingAccountTime
r5apex.exe!0x018ded60 ConVar mp_linkingAccountWindow
r5apex.exe!0x02817e90 ConVar mp_maxbodyyaw
r5apex.exe!0x01755ea0 ConVar mp_permission_requestInterval
r5apex.exe!0x017541a0 ConVar mp_permission_rerequestInterval
r5apex.exe!0x0234bc40 ConVar mp_player_level
r5apex.exe!0x02817f30 ConVar mp_scaleAnimationSpeeds
r5apex.exe!0x02818070 ConVar mp_showgestureslots
r5apex.exe!0x024773a0 ConVar mp_teamlist
r5apex.exe!0x02424360 ConVar mp_teamoverride
r5apex.exe!0x02477f70 ConVar mp_weaponstay
r5apex.exe!0x0175d4e0 ConVar mtx_svEdition
r5apex.exe!0x02815a20 ConVar multiplayer_animstate_once_per_frame_on_server
r5apex.exe!0x0238b290 ConVar muteWeaponSounds
r5apex.exe!0x0281b700 ConVar muteWeaponSounds
r5apex.exe!0x01856c30 ConVar name
r5apex.exe!0x023fd650 ConVar navmesh_move_along_surface_asserts
r5apex.exe!0x023fa7b0 ConVar navmesh_normal_links_only
r5apex.exe!0x023f85d0 ConVar navmesh_test_zone_connectivity_traverse_anim_type
r5apex.exe!0x01f8a8b0 ConVar net_RunInvalidatePhysics
r5apex.exe!0x01751f60 ConVar net_async_sendto
r5apex.exe!0x017505f0 ConVar net_autoUnthrottle
r5apex.exe!0x017571a0 ConVar net_bandwidthPrintThreshold
r5apex.exe!0x01755920 ConVar net_bindToSpecificAddress
r5apex.exe!0x0174b870 ConVar net_blockmsg
r5apex.exe!0x018dfec0 ConVar net_chatThroughChatserver
r5apex.exe!0x01756a00 ConVar net_chokeloop
r5apex.exe!0x017504b0 ConVar net_clearReliableDataOnReset
r5apex.exe!0x0238a950 ConVar net_client_side_weapon_animations
r5apex.exe!0x0281ac50 ConVar net_client_side_weapon_animations
r5apex.exe!0x01756e80 ConVar net_compressDataBlock
r5apex.exe!0x01752140 ConVar net_compressLZValue
r5apex.exe!0x0174f8d0 ConVar net_compresspackets
r5apex.exe!0x0174cde0 ConVar net_compresspackets_minsize
r5apex.exe!0x01855da0 ConVar net_connectPacketWarningThreshhold
r5apex.exe!0x01859550 ConVar net_connectingDataRate
r5apex.exe!0x01749110 ConVar net_createUndoDeltas
r5apex.exe!0x0185b2e0 ConVar net_data_block_enabled
r5apex.exe!0x01754760 ConVar net_datablockPrintSummaries
r5apex.exe!0x01857b00 ConVar net_datablock_fastRate
r5apex.exe!0x0174fb50 ConVar net_datablock_longSendTime
r5apex.exe!0x017554c0 ConVar net_datablock_minResendInterval
r5apex.exe!0x01859840 ConVar net_datablock_networkLossForSlowSpeed
r5apex.exe!0x018580a0 ConVar net_datablock_resendRateForSlowSpeed
r5apex.exe!0x0185a660 ConVar net_datablock_slowRate
r5apex.exe!0x0174b1b0 ConVar net_debugDataBlockReceiver
r5apex.exe!0x01754940 ConVar net_debugDataBlockSender
r5apex.exe!0x0175db80 ConVar net_debugLerping
r5apex.exe!0x01856910 ConVar net_deltaFieldEntityBlockSize
r5apex.exe!0x018556c0 ConVar net_disconnectIfDeltaBufferIsFull
r5apex.exe!0x0174abd0 ConVar net_drawslider
r5apex.exe!0x01752780 ConVar net_droppackets
r5apex.exe!0x017494a0 ConVar net_dumpChangesPrecise
r5apex.exe!0x0174d2f0 ConVar net_encrypt_copyCtx
r5apex.exe!0x0174fd30 ConVar net_encryptionDebug
r5apex.exe!0x01750730 ConVar net_fakelag
r5apex.exe!0x01756680 ConVar net_fakelag_clientOnly
r5apex.exe!0x01757100 ConVar net_fakelagjitter
r5apex.exe!0x01757060 ConVar net_fakeloss
r5apex.exe!0x01855580 ConVar net_forceDeltaBufferToOverflow
r5apex.exe!0x01748310 ConVar net_forceUnnecessaryUndoDeltas
r5apex.exe!0x017545a0 ConVar net_forcetimeout
r5apex.exe!0x01756c60 ConVar net_fullyConnectedDataRate
r5apex.exe!0x0181f6f0 ConVar net_highPacketLatencyThreshold
r5apex.exe!0x0181e620 ConVar net_highPacketLossThreshold
r5apex.exe!0x0181e1d0 ConVar net_ignoreAllSnapshots
r5apex.exe!0x01855bc0 ConVar net_largeSnapshotThreshold
r5apex.exe!0x0175dd60 ConVar net_lerpFields
r5apex.exe!0x01859410 ConVar net_lowBandwidthConnect
r5apex.exe!0x0174b250 ConVar net_maxAccumulatedClearTimeBalance
r5apex.exe!0x0174c0e0 ConVar net_maxcleartime
r5apex.exe!0x0174d110 ConVar net_maxfilesize
r5apex.exe!0x0174c290 ConVar net_maxfragments
r5apex.exe!0x0174b570 ConVar net_maxroutable
r5apex.exe!0x0174bab0 ConVar net_minConnectionTimeForSpam
r5apex.exe!0x01753ca0 ConVar net_minQueuedPacketsForPrint
r5apex.exe!0x02341bf0 ConVar net_minResetIdleTimerInterval
r5apex.exe!0x0174c510 ConVar net_minimumPacketLossDC
r5apex.exe!0x0174c330 ConVar net_minroutable
r5apex.exe!0x01856a50 ConVar net_noPostDataForDeletedEnts
r5apex.exe!0x023eca20 ConVar net_old_seed_generation
r5apex.exe!0x01859050 ConVar net_optimize_persistent_data
r5apex.exe!0x0185b060 ConVar net_optimize_playlists
r5apex.exe!0x02822c80 ConVar net_optimize_weapons
r5apex.exe!0x01f85560 ConVar net_predictParentEntities
r5apex.exe!0x01856120 ConVar net_predictedEntsUseFirstAvailableSnapshot
r5apex.exe!0x01855b20 ConVar net_predictionDebug
r5apex.exe!0x0175df40 ConVar net_pretendSnapshotArrayFull
r5apex.exe!0x0174e0d0 ConVar net_printCompression
r5apex.exe!0x01856260 ConVar net_printOutOfSnapshots
r5apex.exe!0x01748090 ConVar net_printUnnecessaryDeltas
r5apex.exe!0x0174a0f0 ConVar net_propSkipPrintThreshold
r5apex.exe!0x01751970 ConVar net_public_adr
r5apex.exe!0x0174deb0 ConVar net_queue_trace
r5apex.exe!0x0174a750 ConVar net_queuedPackets_PrintOversleeps
r5apex.exe!0x01750370 ConVar net_queuedPackets_SkipSmallSleeps
r5apex.exe!0x0174d4d0 ConVar net_queued_packet_thread
r5apex.exe!0x0174f0b0 ConVar net_recentNetworkGapWindow
r5apex.exe!0x01751210 ConVar net_recentNetworkGapsNeeded
r5apex.exe!0x018558a0 ConVar net_recreateScriptInstanceOnReplayTransition
r5apex.exe!0x0175e080 ConVar net_recv_dumpChanges
r5apex.exe!0x017489e0 ConVar net_recv_dumpNetworkedChangesOnEntCreate
r5apex.exe!0x01748da0 ConVar net_recv_watchEnt
r5apex.exe!0x017483b0 ConVar net_recv_watchField1
r5apex.exe!0x017499b0 ConVar net_recv_watchField2
r5apex.exe!0x017530d0 ConVar net_resourcePrintMinimum
r5apex.exe!0x01747d30 ConVar net_sendFloatDeltas
r5apex.exe!0x0185a3e0 ConVar net_sendProfileTotals
r5apex.exe!0x01752320 ConVar net_sendtoInJob
r5apex.exe!0x017564c0 ConVar net_showFailedAuth
r5apex.exe!0x01855e40 ConVar net_showLargeSnapshot
r5apex.exe!0x0174e770 ConVar net_showQueued
r5apex.exe!0x01747510 ConVar net_showUndoDeltas
r5apex.exe!0x0181ea70 ConVar net_showUserWarnings
r5apex.exe!0x017568e0 ConVar net_showchoke
r5apex.exe!0x0174b110 ConVar net_showchokeInterval
r5apex.exe!0x0174f790 ConVar net_showdrop
r5apex.exe!0x0174b430 ConVar net_showfragments
r5apex.exe!0x017512b0 ConVar net_showmsg
r5apex.exe!0x0174d610 ConVar net_showpeaks
r5apex.exe!0x0174ef70 ConVar net_showsendrecv
r5apex.exe!0x0174d6b0 ConVar net_showsplits
r5apex.exe!0x0174f650 ConVar net_showudp
r5apex.exe!0x0174ac70 ConVar net_showudp_oob
r5apex.exe!0x01756fc0 ConVar net_showudp_remoteonly
r5apex.exe!0x0237d270 ConVar net_showusercmd
r5apex.exe!0x01746e30 ConVar net_skipUnnecessaryDeltas
r5apex.exe!0x017534a0 ConVar net_splitrate
r5apex.exe!0x01751ec0 ConVar net_splitrateDefaultMP
r5apex.exe!0x0174e590 ConVar net_splitrateDefaultSP
r5apex.exe!0x023e3800 ConVar net_sv_showusercmd
r5apex.exe!0x0174a910 ConVar net_tamperPackets
r5apex.exe!0x018559e0 ConVar net_threadedEntityDeltas
r5apex.exe!0x01856af0 ConVar net_threadedProcessPacket
r5apex.exe!0x0174bf30 ConVar net_timeoutUsesLastReadTime
r5apex.exe!0x01752960 ConVar net_trackerWarningInterval
r5apex.exe!0x017556a0 ConVar net_usesocketsforloopback
r5apex.exe!0x01753f20 ConVar net_verifyEncryption
r5apex.exe!0x018dfdb0 ConVar net_voiceEchoFromChatServer
r5apex.exe!0x0174bc60 ConVar net_warnAboutSocketReadGaps
r5apex.exe!0x017546c0 ConVar net_warnGapTime
r5apex.exe!0x017559c0 ConVar net_wifi
r5apex.exe!0x01855800 ConVar net_worldHitchSlopTime
r5apex.exe!0x01752820 ConVar next
r5apex.exe!0x023ecd40 ConVar noReloadAfterUse
r5apex.exe!0x024769f0 ConVar noclip_fixup
r5apex.exe!0x01f69a00 ConVar noise_filter_scale
r5apex.exe!0x017585d0 ConVar not_focus_sleep
r5apex.exe!0x018da500 ConVar notification_displayTime
r5apex.exe!0x023f8380 ConVar npc_chancetohit_forcedOn
r5apex.exe!0x023f48c0 ConVar npc_sight_mode
r5apex.exe!0x02829af0 ConVar npc_titan_always_block_projectile_health
r5apex.exe!0x028287c0 ConVar npc_titan_block_projectile_chance
r5apex.exe!0x02829b90 ConVar npc_titan_footstep_sound_radius
r5apex.exe!0x02829970 ConVar npc_titan_light_pain_threshold
r5apex.exe!0x02828720 ConVar npc_titan_phys_ignore_mass
r5apex.exe!0x02829280 ConVar npc_titan_phys_knock_damage
r5apex.exe!0x02829a10 ConVar npc_titan_phys_knock_mass
r5apex.exe!0x028293c0 ConVar npc_titan_phys_knock_radius
r5apex.exe!0x02829320 ConVar npc_titan_phys_knock_speed
r5apex.exe!0x01759650 ConVar nucleus_id
r5apex.exe!0x01758c50 ConVar nucleus_pid
r5apex.exe!0x0233d160 ConVar number_shortenToMillionsAfter
r5apex.exe!0x02394600 ConVar offhandTossOverheadPitchThreshold
r5apex.exe!0x02824b90 ConVar offhandTossOverheadPitchThreshold
r5apex.exe!0x0175ac10 ConVar old_culling
r5apex.exe!0x01758f70 ConVar old_gather_props
r5apex.exe!0x02372ef0 ConVar one_handed_change_rate
r5apex.exe!0x023cde20 ConVar one_handed_change_rate
r5apex.exe!0x018dad00 ConVar openInvite_spam
r5apex.exe!0x018da6e0 ConVar openInvites_filterByLanguage
r5apex.exe!0x018da9e0 ConVar openInvites_filterByRegion
r5apex.exe!0x018dd0a0 ConVar openinvite_duration_default
r5apex.exe!0x02811650 ConVar ordnancePickupSound
r5apex.exe!0x02326190 ConVar ordnanceSwapSelectCooldown
r5apex.exe!0x018dc420 ConVar origin_Errorlevel_OldBehaviour
r5apex.exe!0x018dc6a0 ConVar origin_Errorlevel_Telementry
r5apex.exe!0x018dc600 ConVar origin_authCodeFailureMaxBackoffSeconds
r5apex.exe!0x018dc4c0 ConVar origin_autoRefreshToken
r5apex.exe!0x018dc2e0 ConVar origin_debug
r5apex.exe!0x018dbe80 ConVar origin_disconnectWhenOffline
r5apex.exe!0x018dbf20 ConVar origin_ignoreInvitesOnLoadScreen
r5apex.exe!0x018dbd40 ConVar origin_igo_mutes_sound_enabled
r5apex.exe!0x01f879e0 ConVar origin_igo_muting_sound
r5apex.exe!0x018dc380 ConVar origin_presense_updateRate
r5apex.exe!0x018dc560 ConVar origin_tokenFailureMaxBackoffSeconds
r5apex.exe!0x01f6e5a0 ConVar panel_showVisChanges
r5apex.exe!0x01f6e240 ConVar panel_test_title_safe
r5apex.exe!0x023f9170 ConVar parenting_clearParentOriginFix
r5apex.exe!0x01f74b10 ConVar parenting_debug
r5apex.exe!0x02380af0 ConVar particleEffect_checkShouldStillPlay
r5apex.exe!0x02398120 ConVar particle_alwayswakeonstop
r5apex.exe!0x02816250 ConVar particle_alwayswakeonstop
r5apex.exe!0x018e08f0 ConVar particle_cpu_level
r5apex.exe!0x023984e0 ConVar particle_delete_all_except
r5apex.exe!0x02816610 ConVar particle_delete_all_except
r5apex.exe!0x0231ad40 ConVar particle_dlights_enable
r5apex.exe!0x02324c70 ConVar particle_dlights_spew
r5apex.exe!0x02347820 ConVar particle_gpu_level
r5apex.exe!0x019eb880 ConVar particle_lighting_clear_enable
r5apex.exe!0x019eb7e0 ConVar particle_lighting_size
r5apex.exe!0x02397c80 ConVar particle_lighting_viewmodel_enable
r5apex.exe!0x0233fb40 ConVar particle_overlay
r5apex.exe!0x02340fc0 ConVar particle_overlay_detail_attributes
r5apex.exe!0x023467d0 ConVar particle_overlay_detail_filter
r5apex.exe!0x023463f0 ConVar particle_overlay_detail_list_particles
r5apex.exe!0x02342680 ConVar particle_overlay_detail_scroll
r5apex.exe!0x023448f0 ConVar particle_overlay_hide_sleeping
r5apex.exe!0x02345230 ConVar particle_overlay_list_filter
r5apex.exe!0x0233f690 ConVar particle_overlay_list_tally
r5apex.exe!0x02340e80 ConVar particle_overlay_list_tally_collapse_children
r5apex.exe!0x02343200 ConVar particle_overlay_old
r5apex.exe!0x02343ba0 ConVar particle_overlay_scroll
r5apex.exe!0x02397930 ConVar particle_remap_vol2cp_debug
r5apex.exe!0x023975e0 ConVar particle_script_dump
r5apex.exe!0x02815cb0 ConVar particle_script_dump
r5apex.exe!0x02397680 ConVar particle_script_list
r5apex.exe!0x02815d50 ConVar particle_script_list
r5apex.exe!0x02397720 ConVar particle_script_log
r5apex.exe!0x02815df0 ConVar particle_script_log
r5apex.exe!0x0233ff10 ConVar particle_scrub_debug
r5apex.exe!0x02397d60 ConVar particle_scrub_debug_effect
r5apex.exe!0x02815e90 ConVar particle_scrub_debug_effect
r5apex.exe!0x02816750 ConVar particle_scrub_is_using_time_scrub
r5apex.exe!0x023981c0 ConVar particle_scrub_max_dt
r5apex.exe!0x028162f0 ConVar particle_scrub_max_dt
r5apex.exe!0x02398260 ConVar particle_scrub_play_speed
r5apex.exe!0x02816390 ConVar particle_scrub_play_speed
r5apex.exe!0x02398440 ConVar particle_scrub_quality
r5apex.exe!0x02816570 ConVar particle_scrub_quality
r5apex.exe!0x02398300 ConVar particle_scrub_time
r5apex.exe!0x02816430 ConVar particle_scrub_time
r5apex.exe!0x02344fb0 ConVar particle_simulateoverflow
r5apex.exe!0x023cdd80 ConVar particle_test_attach_attachment
r5apex.exe!0x0239bbe0 ConVar particle_test_attach_mode
r5apex.exe!0x02398c20 ConVar particle_test_file
r5apex.exe!0x02398580 ConVar particles_cull_dlights
r5apex.exe!0x028166b0 ConVar particles_cull_dlights
r5apex.exe!0x023983a0 ConVar particles_max_passes
r5apex.exe!0x028164d0 ConVar particles_max_passes
r5apex.exe!0x02397e00 ConVar particles_spawncull
r5apex.exe!0x02815f30 ConVar particles_spawncull
r5apex.exe!0x02397fe0 ConVar particles_spawncull_report
r5apex.exe!0x02816110 ConVar particles_spawncull_report
r5apex.exe!0x02397ea0 ConVar particles_try_reloading_sheets
r5apex.exe!0x02815fd0 ConVar particles_try_reloading_sheets
r5apex.exe!0x018dd140 ConVar parties_alwaysReadSubs
r5apex.exe!0x018dcf60 ConVar party_alwaysGoToLobbyOnSwitch
r5apex.exe!0x018dc880 ConVar party_autoCreatePartyAlways
r5apex.exe!0x018dd460 ConVar party_autoCreatePartyDelay
r5apex.exe!0x0237d350 ConVar party_color_enabled
r5apex.exe!0x018dcb00 ConVar party_doRealNameLookups
r5apex.exe!0x018dc9c0 ConVar party_doRealNameLookupsForOwner
r5apex.exe!0x018db020 ConVar party_hostname
r5apex.exe!0x018dc920 ConVar party_httpHandleTimeout
r5apex.exe!0x018daa80 ConVar party_keepAliveTime
r5apex.exe!0x018dcec0 ConVar party_keepAliveTime
r5apex.exe!0x018dcce0 ConVar party_leaderAlwaysDetectsChanges
r5apex.exe!0x018dd000 ConVar party_leaveMatchOnJoin
r5apex.exe!0x018dce20 ConVar party_lookupRealNamesForOpenInvites
r5apex.exe!0x018dd1e0 ConVar party_lookupRealNamesForOpenInvitesForOwner
r5apex.exe!0x018dcc40 ConVar party_minSize
r5apex.exe!0x018dd3c0 ConVar party_privacy
r5apex.exe!0x018dd500 ConVar party_readyToSearch
r5apex.exe!0x0185b610 ConVar party_relyOnPartyForMemberUserInfo
r5apex.exe!0x018dd5a0 ConVar party_requireConsensusForSearch
r5apex.exe!0x0175bb80 ConVar perTriangleCollisionForced
r5apex.exe!0x018e02b0 ConVar persistence_clForceNew
r5apex.exe!0x017557e0 ConVar persistence_hostname
r5apex.exe!0x01856550 ConVar persistent_warningRate
r5apex.exe!0x01757ff0 ConVar pertrianglecollision
r5apex.exe!0x028176d0 ConVar phys_bounce
r5apex.exe!0x02397260 ConVar phys_cfm
r5apex.exe!0x02817170 ConVar phys_cfm
r5apex.exe!0x023970e0 ConVar phys_cfm_anglejointstop
r5apex.exe!0x02816eb0 ConVar phys_cfm_anglejointstop
r5apex.exe!0x02477520 ConVar phys_damage_players
r5apex.exe!0x02397180 ConVar phys_drawContacts
r5apex.exe!0x02816f50 ConVar phys_drawContacts
r5apex.exe!0x02817090 ConVar phys_drawContactsDuration
r5apex.exe!0x02816cd0 ConVar phys_drawGeoms
r5apex.exe!0x02817630 ConVar phys_drawTunnelChecks
r5apex.exe!0x02817590 ConVar phys_enableObjectPairCollidePrototype
r5apex.exe!0x02396d20 ConVar phys_erp
r5apex.exe!0x02816a50 ConVar phys_erp
r5apex.exe!0x02396f00 ConVar phys_erp_anglejointstop
r5apex.exe!0x02816c30 ConVar phys_erp_anglejointstop
r5apex.exe!0x028174f0 ConVar phys_frictionDefault
r5apex.exe!0x023fcc00 ConVar phys_impactforcescale
r5apex.exe!0x02345970 ConVar phys_showObjectCount
r5apex.exe!0x02476090 ConVar phys_show_active
r5apex.exe!0x02476950 ConVar phys_speeds
r5apex.exe!0x023ef190 ConVar phys_stressbodyweights
r5apex.exe!0x02477e30 ConVar phys_timescale
r5apex.exe!0x02817450 ConVar phys_updateDummyGeomsThreaded
r5apex.exe!0x023f6390 ConVar phys_upimpactforcescale
r5apex.exe!0x02397540 ConVar physics_async_cl
r5apex.exe!0x02816ff0 ConVar physics_async_sv
r5apex.exe!0x02397400 ConVar physics_autoSleepAngularThreshold
r5apex.exe!0x02817310 ConVar physics_autoSleepAngularThreshold
r5apex.exe!0x02397040 ConVar physics_autoSleepDebug
r5apex.exe!0x02816e10 ConVar physics_autoSleepDebug
r5apex.exe!0x02396fa0 ConVar physics_autoSleepGroundHysteresis
r5apex.exe!0x02816d70 ConVar physics_autoSleepGroundHysteresis
r5apex.exe!0x023974a0 ConVar physics_autoSleepSpeedThreshold
r5apex.exe!0x028173b0 ConVar physics_autoSleepSpeedThreshold
r5apex.exe!0x02396dc0 ConVar physics_collideWithMovingGeo
r5apex.exe!0x02816af0 ConVar physics_collideWithMovingGeo
r5apex.exe!0x023ac890 ConVar physics_defaultMaxAngularSpeed
r5apex.exe!0x023cd9e0 ConVar physics_defaultMaxSpeed
r5apex.exe!0x01f72120 ConVar physics_scaled_mem
r5apex.exe!0x02396e60 ConVar physics_tunnelChecks
r5apex.exe!0x02816b90 ConVar physics_tunnelChecks
r5apex.exe!0x02397360 ConVar physics_tunnelChecksForceAlways
r5apex.exe!0x02817270 ConVar physics_tunnelChecksForceAlways
r5apex.exe!0x023dc0e0 ConVar physicsshadowupdate_render
r5apex.exe!0x0175afd0 ConVar pin_opt_in
r5apex.exe!0x01856b90 ConVar pin_plat_id
r5apex.exe!0x0175b2f0 ConVar pin_sid
r5apex.exe!0x01757cd0 ConVar pin_telemetry_actually_send
r5apex.exe!0x0175b570 ConVar pin_telemetry_debug_code
r5apex.exe!0x01757a80 ConVar pin_telemetry_debug_payload
r5apex.exe!0x0237e5c0 ConVar pin_telemetry_debug_script
r5apex.exe!0x023e36c0 ConVar pin_telemetry_debug_script
r5apex.exe!0x01757f50 ConVar pin_telemetry_dont_send_events
r5apex.exe!0x01759150 ConVar pin_telemetry_hostname
r5apex.exe!0x01758cf0 ConVar pin_telemetry_inactivity_send_time
r5apex.exe!0x01758ed0 ConVar pin_telemetry_max_payload_size
r5apex.exe!0x0175a350 ConVar pin_telemetry_send_debug
r5apex.exe!0x01751050 ConVar ping_debug
r5apex.exe!0x023846e0 ConVar ping_max_green
r5apex.exe!0x02384560 ConVar ping_max_red
r5apex.exe!0x02384aa0 ConVar ping_max_yellow
r5apex.exe!0x0174f510 ConVar ping_minSentForChoice
r5apex.exe!0x017515d0 ConVar ping_qos_units
r5apex.exe!0x02813a10 ConVar ping_show_measured
r5apex.exe!0x01755f40 ConVar ping_usePacketLoss
r5apex.exe!0x01f751f0 ConVar pixvis_enable
r5apex.exe!0x01f66f00 ConVar pixvis_maxquads
r5apex.exe!0x01f74f70 ConVar pixvis_spew
r5apex.exe!0x018dba20 ConVar plat_environment
r5apex.exe!0x018da210 ConVar plat_retryNameLookups
r5apex.exe!0x01856cd0 ConVar platform_user_id
r5apex.exe!0x02814680 ConVar playerDeathAnimMaxFrames
r5apex.exe!0x02383c20 ConVar playerListPartyColorB
r5apex.exe!0x02384a00 ConVar playerListPartyColorG
r5apex.exe!0x02383900 ConVar playerListPartyColorR
r5apex.exe!0x02384c80 ConVar playerListUseFriendColor
r5apex.exe!0x023786b0 ConVar player_ADS_buffer_time_seconds
r5apex.exe!0x023dd120 ConVar player_ADS_buffer_time_seconds
r5apex.exe!0x02810b00 ConVar player_adjustTimersWithTimeBase
r5apex.exe!0x027f8610 ConVar player_charDataMinInterval
r5apex.exe!0x023180b0 ConVar player_debugPredictedPosition
r5apex.exe!0x027f8a50 ConVar player_debug_print_damage
r5apex.exe!0x01f74930 ConVar player_deltaAnimsMakeMeUnpredicted
r5apex.exe!0x0280fb20 ConVar player_disallow_negative_frametime
r5apex.exe!0x02815980 ConVar player_dispatch_anim_events_per_frame
r5apex.exe!0x01f8f190 ConVar player_doJetwashEffects
r5apex.exe!0x0236e750 ConVar player_extraairaccelleration
r5apex.exe!0x023b8350 ConVar player_extraairaccelleration
r5apex.exe!0x023da230 ConVar player_find_rodeo_target_per_cmd
r5apex.exe!0x02317f70 ConVar player_highFrequencyThinkDistance
r5apex.exe!0x02810670 ConVar player_maxTimerAdjust
r5apex.exe!0x02803e40 ConVar player_max_command_contexts
r5apex.exe!0x02824cd0 ConVar player_melee_cone_from_user_command_only
r5apex.exe!0x023d5e50 ConVar player_movementBounds_predictionShare
r5apex.exe!0x02803da0 ConVar player_movement_debug
r5apex.exe!0x02817810 ConVar player_movingDeathThreshold
r5apex.exe!0x0280ff80 ConVar player_old_armor
r5apex.exe!0x01f92970 ConVar player_respawnInputDebounceDuration
r5apex.exe!0x02812430 ConVar player_restore_use_SetLocalAngles
r5apex.exe!0x02802c40 ConVar player_restore_use_UpdateCurrentPlayerClass
r5apex.exe!0x02813280 ConVar player_share_squad_info
r5apex.exe!0x01f890b0 ConVar player_showEyePosition
r5apex.exe!0x023db4d0 ConVar player_showpredictedposition
r5apex.exe!0x023e2af0 ConVar player_showpredictedposition_timestep
r5apex.exe!0x023485c0 ConVar player_useMovementBounds
r5apex.exe!0x02398b80 ConVar player_useMovementBounds
r5apex.exe!0x02803bd0 ConVar player_userCmdsQueueWarning
r5apex.exe!0x01f92b30 ConVar player_viewchange_debug_pitch
r5apex.exe!0x02319fc0 ConVar player_viewchange_debug_roll
r5apex.exe!0x01f8d780 ConVar player_viewchange_debug_yaw
r5apex.exe!0x023fe5f0 ConVar playerframetimekick_debug
r5apex.exe!0x02477660 ConVar playerframetimekick_decayrate
r5apex.exe!0x02475dd0 ConVar playerframetimekick_includerealtime
r5apex.exe!0x02434c80 ConVar playerframetimekick_margin
r5apex.exe!0x02383e00 ConVar playerlist_showGen
r5apex.exe!0x028138d0 ConVar players_updatePingTickInterval
r5apex.exe!0x0175aad0 ConVar playlist_changeGamemodeAutomatically
r5apex.exe!0x01759b50 ConVar playlist_debug
r5apex.exe!0x017579e0 ConVar playlist_debug_getvar
r5apex.exe!0x017590b0 ConVar playlist_debug_localization
r5apex.exe!0x0236f9c0 ConVar playlist_errorOnDeprecated
r5apex.exe!0x023b9590 ConVar playlist_errorOnDeprecated
r5apex.exe!0x028181d0 ConVar portal_pointpush_debug
r5apex.exe!0x028182b0 ConVar portal_pointpush_think_rate
r5apex.exe!0x023ed9c0 ConVar portal_use_player_avoidance
r5apex.exe!0x01f8aa90 ConVar postdataupdate_threaded
r5apex.exe!0x01f7ff90 ConVar postdataupdate_threaded_chunksize
r5apex.exe!0x0280a060 ConVar prevent_ammo_suck
r5apex.exe!0x017523c0 ConVar printConnectTimings
r5apex.exe!0x01f975b0 ConVar print_timeprefix
r5apex.exe!0x02390da0 ConVar process_pending_vm_effects
r5apex.exe!0x02383a40 ConVar progressbar_allow_wrap
r5apex.exe!0x02383f40 ConVar progressbar_high_precision
r5apex.exe!0x02383cc0 ConVar progressbar_single_bar
r5apex.exe!0x0238dc10 ConVar projectile_fake_prediction_in_kill_replay
r5apex.exe!0x0281e0b0 ConVar projectile_fake_prediction_in_kill_replay
r5apex.exe!0x02392980 ConVar projectile_faketrails
r5apex.exe!0x023949a0 ConVar projectile_filltrails
r5apex.exe!0x02824ee0 ConVar projectile_lagCompensationDebug
r5apex.exe!0x0281acf0 ConVar projectile_lagCompensationDebugDrawTime
r5apex.exe!0x028233b0 ConVar projectile_lagCompensationDebugExtra
r5apex.exe!0x0281aa70 ConVar projectile_lagCompensationDebugServerOffset
r5apex.exe!0x02826de0 ConVar projectile_lagCompensationMissileTimeStepScalar
r5apex.exe!0x02392c00 ConVar projectile_muzzleOffsetFirstPersonDecayDist
r5apex.exe!0x0238bf50 ConVar projectile_muzzleOffsetFirstPersonDecayMaxTime
r5apex.exe!0x0238b1f0 ConVar projectile_muzzleOffsetThirdPersonDecayDist
r5apex.exe!0x02392de0 ConVar projectile_muzzleOffsetThirdPersonDecayMaxTime
r5apex.exe!0x02826af0 ConVar projectile_prediction
r5apex.exe!0x02395560 ConVar projectile_predictionErrorCorrectTime
r5apex.exe!0x02805ab0 ConVar prop_active_gib_limit
r5apex.exe!0x027f87f0 ConVar prop_active_gib_max_fade_time
r5apex.exe!0x0280b850 ConVar prop_break_disable_float
r5apex.exe!0x023184f0 ConVar prop_lightweightPropsSkipAnimData
r5apex.exe!0x01f74d90 ConVar prop_survivalSkipsAnimData
r5apex.exe!0x023ec7a0 ConVar props_break_burst_rotation
r5apex.exe!0x023ed340 ConVar props_break_max_pieces
r5apex.exe!0x023ecca0 ConVar props_break_max_pieces_perframe
r5apex.exe!0x018db0c0 ConVar publication_hostname
r5apex.exe!0x02324b30 ConVar push_cl
r5apex.exe!0x01f86be0 ConVar push_cl_always_update_prev_matrix
r5apex.exe!0x023e3760 ConVar push_debug
r5apex.exe!0x023e16a0 ConVar push_debug_ent
r5apex.exe!0x023debf0 ConVar push_debug_pause_always
r5apex.exe!0x023e2790 ConVar push_player_nearby_dist
r5apex.exe!0x02377b00 ConVar push_ragdolls
r5apex.exe!0x023dd010 ConVar pve_debug
r5apex.exe!0x01755880 ConVar pve_modedetect_substring
r5apex.exe!0x02382080 ConVar pvs_addWorkItemsAccum
r5apex.exe!0x02382620 ConVar pvs_addWorkItemsThreshold
r5apex.exe!0x02382580 ConVar pvs_cullBoxes
r5apex.exe!0x02382260 ConVar pvs_debug
r5apex.exe!0x02382440 ConVar pvs_drawPortals
r5apex.exe!0x02382300 ConVar pvs_frustumCullOnly
r5apex.exe!0x02324e50 ConVar pvs_start_early
r5apex.exe!0x023bb040 ConVar r_AirboatViewDampenDamp
r5apex.exe!0x023cd6c0 ConVar r_AirboatViewDampenFreq
r5apex.exe!0x023bbe00 ConVar r_AirboatViewZHeight
r5apex.exe!0x02348230 ConVar r_DrawBeams
r5apex.exe!0x023cda80 ConVar r_JeepViewDampenDamp
r5apex.exe!0x023bbfe0 ConVar r_JeepViewDampenFreq
r5apex.exe!0x0239bd20 ConVar r_VehicleViewDampen
r5apex.exe!0x023823a0 ConVar r_WaterDrawReflection
r5apex.exe!0x018e76e0 ConVar r_WaterDrawRefraction
r5apex.exe!0x0174c650 ConVar r_aspectratio
r5apex.exe!0x01f68880 ConVar r_bloomtintb
r5apex.exe!0x01f68a60 ConVar r_bloomtintexponent
r5apex.exe!0x01f689c0 ConVar r_bloomtintg
r5apex.exe!0x01f68920 ConVar r_bloomtintr
r5apex.exe!0x0233e170 ConVar r_blurmenubg
r5apex.exe!0x01759c90 ConVar r_brush_queue_mode
r5apex.exe!0x0175a6f0 ConVar r_createmodeldecals
r5apex.exe!0x017502d0 ConVar r_cullshadowworldmeshes
r5apex.exe!0x023488c0 ConVar r_debug_draw_box_depth_test
r5apex.exe!0x0175ad50 ConVar r_decal_cover_count
r5apex.exe!0x01749890 ConVar r_decal_cull_stretch_limit
r5apex.exe!0x01758a70 ConVar r_decal_draw_basis
r5apex.exe!0x01748c60 ConVar r_decal_drawclipped
r5apex.exe!0x0175b390 ConVar r_decal_overlap_area
r5apex.exe!0x0175a650 ConVar r_decal_overlap_count
r5apex.exe!0x01758bb0 ConVar r_decal_test_scale
r5apex.exe!0x01751530 ConVar r_decals
r5apex.exe!0x01f681a0 ConVar r_ditherAlpha
r5apex.exe!0x018e78c0 ConVar r_ditherFade
r5apex.exe!0x01f8f0e0 ConVar r_ditherFade
r5apex.exe!0x01f68240 ConVar r_ditherFadeShadows
r5apex.exe!0x01f89eb0 ConVar r_ditherFadeShadows
r5apex.exe!0x02324810 ConVar r_drawallrenderables
r5apex.exe!0x0233e550 ConVar r_drawalphasort
r5apex.exe!0x017565e0 ConVar r_drawbrushmodels
r5apex.exe!0x02347390 ConVar r_drawbrushmodels
r5apex.exe!0x0175a3f0 ConVar r_drawdecals
r5apex.exe!0x02345d10 ConVar r_drawdepth_of_blend2transparent
r5apex.exe!0x01747c20 ConVar r_drawdlights
r5apex.exe!0x0174d430 ConVar r_drawentities
r5apex.exe!0x017497f0 ConVar r_drawlightdist
r5apex.exe!0x01748f30 ConVar r_drawlightinfo
r5apex.exe!0x01f8a770 ConVar r_drawmodelsinzfill
r5apex.exe!0x0174c470 ConVar r_drawmodelstatsoverlay
r5apex.exe!0x01f7eb50 ConVar r_drawmodelstatsoverlay
r5apex.exe!0x01754320 ConVar r_drawmodelstatsoverlaydistance
r5apex.exe!0x01751350 ConVar r_drawmodelstatsoverlayfilter
r5apex.exe!0x0174de10 ConVar r_drawmodelstatsoverlaymax
r5apex.exe!0x0174fe70 ConVar r_drawmodelstatsoverlaymin
r5apex.exe!0x02347780 ConVar r_drawopaquerenderables
r5apex.exe!0x01f94190 ConVar r_drawothermodels
r5apex.exe!0x023476e0 ConVar r_drawparticles
r5apex.exe!0x01f83410 ConVar r_drawrenderboxes
r5apex.exe!0x0233de10 ConVar r_drawscreenspaceparticles
r5apex.exe!0x02342000 ConVar r_drawsky
r5apex.exe!0x02342b20 ConVar r_drawskybox_deprecated
r5apex.exe!0x02334260 ConVar r_drawsprites
r5apex.exe!0x01749680 ConVar r_drawstaticlight
r5apex.exe!0x023458d0 ConVar r_drawstaticprops
r5apex.exe!0x02330d20 ConVar r_drawtracers
r5apex.exe!0x01758e30 ConVar r_drawvgui
r5apex.exe!0x02341f20 ConVar r_drawviewmodel
r5apex.exe!0x01754500 ConVar r_drawworld
r5apex.exe!0x0174ad10 ConVar r_dynamic
r5apex.exe!0x02343e20 ConVar r_earlyRenderables
r5apex.exe!0x01f89750 ConVar r_enableOriginSort
r5apex.exe!0x0233c7a0 ConVar r_fadeincode
r5apex.exe!0x02325010 ConVar r_farz
r5apex.exe!0x017538e0 ConVar r_fastzreject
r5apex.exe!0x02382120 ConVar r_forcecheapwater
r5apex.exe!0x01f92fa0 ConVar r_jiggle_bones
r5apex.exe!0x01754e40 ConVar r_lightmap
r5apex.exe!0x01754800 ConVar r_lightprobe_force_trans_dist
r5apex.exe!0x01755160 ConVar r_lightstyle
r5apex.exe!0x017521e0 ConVar r_lod
r5apex.exe!0x023430c0 ConVar r_lod
r5apex.exe!0x01753660 ConVar r_lod_switch_scale
r5apex.exe!0x01f8b310 ConVar r_mapextents
r5apex.exe!0x01f6ab30 ConVar r_modeldecal_maxtotal
r5apex.exe!0x01f75010 ConVar r_nearz
r5apex.exe!0x019ed350 ConVar r_no_stalls
r5apex.exe!0x01f66a20 ConVar r_no_stalls
r5apex.exe!0x01f66fa0 ConVar r_no_stalls
r5apex.exe!0x0174cd40 ConVar r_norefresh
r5apex.exe!0x02340a60 ConVar r_particle_lighting_debug
r5apex.exe!0x0233c150 ConVar r_particle_lighting_enable
r5apex.exe!0x023979f0 ConVar r_particle_lighting_enable
r5apex.exe!0x02397890 ConVar r_particle_lighting_force
r5apex.exe!0x02397be0 ConVar r_particle_lighting_force
r5apex.exe!0x0233c980 ConVar r_particle_low_res_debug
r5apex.exe!0x02397a90 ConVar r_particle_low_res_enable
r5apex.exe!0x02397b30 ConVar r_particle_low_res_force
r5apex.exe!0x019eb920 ConVar r_particle_low_res_tiled_composite
r5apex.exe!0x02340590 ConVar r_particle_sim_spike_increment_ms
r5apex.exe!0x02345db0 ConVar r_particle_sim_spike_threshold_ms
r5apex.exe!0x0233c090 ConVar r_particle_timescale
r5apex.exe!0x01f92f00 ConVar r_pos_debug
r5apex.exe!0x01f6ae50 ConVar r_randomflex
r5apex.exe!0x01f8a510 ConVar r_render_pos_debug
r5apex.exe!0x018e7dc0 ConVar r_rimlight
r5apex.exe!0x01744d10 ConVar r_rootlod
r5apex.exe!0x01755740 ConVar r_rootlod
r5apex.exe!0x0232d240 ConVar r_ropetranslucent
r5apex.exe!0x018e7e60 ConVar r_shadowrendertotexture
r5apex.exe!0x0233e4b0 ConVar r_sky_ignoreAngles
r5apex.exe!0x0231afc0 ConVar r_sort_trans_debug
r5apex.exe!0x01f8a370 ConVar r_sort_trans_debug_dist
r5apex.exe!0x0233f530 ConVar r_threaded_particles
r5apex.exe!0x0233d820 ConVar r_updaterefracttexture
r5apex.exe!0x02346bd0 ConVar r_updaterefracttexture_allowmultiple
r5apex.exe!0x017476f0 ConVar r_visambient
r5apex.exe!0x01749070 ConVar r_visambient_orig
r5apex.exe!0x017495e0 ConVar r_visambient_point
r5apex.exe!0x0174c1f0 ConVar r_vislighting_sphereradius
r5apex.exe!0x0174e8b0 ConVar r_vismodellighting
r5apex.exe!0x0175af30 ConVar r_vismodellighting_lightpos
r5apex.exe!0x0174aeb0 ConVar r_vismodellighting_maxdist
r5apex.exe!0x0233cda0 ConVar r_vismodellighting_maxdist
r5apex.exe!0x01752000 ConVar r_vismodellighting_mindist
r5apex.exe!0x02342f00 ConVar r_vismodellighting_mindist
r5apex.exe!0x01759e70 ConVar r_vismodellighting_offset_x
r5apex.exe!0x01758890 ConVar r_vismodellighting_offset_y
r5apex.exe!0x01759ab0 ConVar r_vismodellighting_offset_z
r5apex.exe!0x01f97470 ConVar r_visualizeproplightcaching
r5apex.exe!0x023800e0 ConVar r_visualizetraces
r5apex.exe!0x0237fb60 ConVar r_visualizetraces_duration
r5apex.exe!0x019ecc80 ConVar r_volumetric_lighting_blur_count
r5apex.exe!0x019ecbe0 ConVar r_volumetric_lighting_blur_type
r5apex.exe!0x019ecf10 ConVar r_volumetric_lighting_distFalloff
r5apex.exe!0x019ecb40 ConVar r_volumetric_lighting_enabled
r5apex.exe!0x019eca00 ConVar r_volumetric_lighting_intensity
r5apex.exe!0x019ecdd0 ConVar r_volumetric_lighting_numSteps
r5apex.exe!0x019ecd20 ConVar r_volumetric_lighting_rotate_dither
r5apex.exe!0x019ece70 ConVar r_volumetric_lighting_scatter
r5apex.exe!0x023821c0 ConVar r_waterforceexpensive
r5apex.exe!0x023824e0 ConVar r_waterforcereflectentities
r5apex.exe!0x02347270 ConVar r_zfill
r5apex.exe!0x01f88b10 ConVar ragdoll_debug
r5apex.exe!0x023f5d70 ConVar ragdoll_debug
r5apex.exe!0x023f53d0 ConVar ragdoll_skipDeathAcceleration
r5apex.exe!0x023460f0 ConVar ragdoll_sleepaftertime
r5apex.exe!0x0233f1c0 ConVar rankedplay_display_enabled
r5apex.exe!0x0233ca40 ConVar rankedplay_voice_enabled
r5apex.exe!0x0181ffc0 ConVar rate
r5apex.exe!0x0232c550 ConVar real_time_update_dt
r5apex.exe!0x0175dfe0 ConVar recalculateOrigin_threaded_chunksize
r5apex.exe!0x02477ed0 ConVar reduced_trigger_checks
r5apex.exe!0x02806100 ConVar reliable_effects_enable
r5apex.exe!0x02386d10 ConVar remoteCalls_requireConnectionScriptsForViewPlayer
r5apex.exe!0x01757800 ConVar remoteMatchInfo_print
r5apex.exe!0x0175bca0 ConVar replay_enable
r5apex.exe!0x017595b0 ConVar replay_prediction_smooth
r5apex.exe!0x023191c0 ConVar report_cliententitysim
r5apex.exe!0x01f8a230 ConVar report_clientthinklist
r5apex.exe!0x02812710 ConVar requestBestObserverTargetFromScript
r5apex.exe!0x02324630 ConVar rodeo_camera_smooth_blend_out_time
r5apex.exe!0x01f8c8d0 ConVar rodeo_camera_smooth_enable
r5apex.exe!0x023ee4e0 ConVar rodeoed_anim_weight
r5apex.exe!0x02383240 ConVar rodeoed_anims_enabled
r5apex.exe!0x0233a560 ConVar rope_collide
r5apex.exe!0x02338bb0 ConVar rope_debug_shake
r5apex.exe!0x027f89b0 ConVar rope_default_segment_length
r5apex.exe!0x01f69fa0 ConVar rope_min_pixel_diameter
r5apex.exe!0x0232c810 ConVar rope_shake
r5apex.exe!0x02339340 ConVar rope_texels_per_world_unit
r5apex.exe!0x02326350 ConVar rope_wiggle_harmonic_falloff
r5apex.exe!0x0233afd0 ConVar rope_wiggle_magnitude_loose
r5apex.exe!0x0232d2e0 ConVar rope_wiggle_magnitude_tight
r5apex.exe!0x02336180 ConVar rope_wiggle_oscillate_speed
r5apex.exe!0x02325fd0 ConVar rope_wiggle_rotate_speed
r5apex.exe!0x02332330 ConVar rope_wiggle_zipline_min_points
r5apex.exe!0x02325a10 ConVar rope_wind_dist
r5apex.exe!0x01f8bb70 ConVar rotate_ents
r5apex.exe!0x0174bb50 ConVar rspn_motd
r5apex.exe!0x019ea1c0 ConVar rt_sync_message_pump
r5apex.exe!0x019ea260 ConVar rt_worker
r5apex.exe!0x023452d0 ConVar rui_asyncTracks
r5apex.exe!0x01f6fa30 ConVar rui_defaultDebugFontFace
r5apex.exe!0x01f6fdf0 ConVar rui_defaultFontFace
r5apex.exe!0x01f6eb70 ConVar rui_defaultFontHeight
r5apex.exe!0x01f6e340 ConVar rui_overrideVguiTextRendering
r5apex.exe!0x01857310 ConVar rui_padDist
r5apex.exe!0x01857270 ConVar rui_safeAreaFrac
r5apex.exe!0x01758130 ConVar rui_standardTextHeight
r5apex.exe!0x0174c800 ConVar s2sPort
r5apex.exe!0x023e26f0 ConVar save_client_entity
r5apex.exe!0x0185af20 ConVar save_enable
r5apex.exe!0x023dca60 ConVar save_thread_entities
r5apex.exe!0x023f9b80 ConVar scene_clamplookat
r5apex.exe!0x01f6e650 ConVar scheme_manager_font_debug
r5apex.exe!0x023419f0 ConVar scr_centertime
r5apex.exe!0x01f811f0 ConVar screen_indicator_back_range
r5apex.exe!0x01f948d0 ConVar screen_indicator_ellipse_height
r5apex.exe!0x01f75340 ConVar screen_indicator_ellipse_width
r5apex.exe!0x01f83110 ConVar screen_indicator_pitch_limit
r5apex.exe!0x01f83670 ConVar screen_indicator_pitch_scale
r5apex.exe!0x02344d30 ConVar screenfade_debug
r5apex.exe!0x023d7180 ConVar script_compile_all_levels
r5apex.exe!0x023425e0 ConVar script_debugger_connect_client_on_mapspawn
r5apex.exe!0x02800e90 ConVar script_debugger_connect_server_on_mapspawn
r5apex.exe!0x023471d0 ConVar script_debugger_connect_ui_auto
r5apex.exe!0x0282aa50 ConVar script_debugger_host
r5apex.exe!0x0282aaf0 ConVar script_debugger_port_client
r5apex.exe!0x0282a9b0 ConVar script_debugger_port_server
r5apex.exe!0x0282a870 ConVar script_debugger_port_ui
r5apex.exe!0x0282ae10 ConVar script_disallow_newslot_on_globals
r5apex.exe!0x0282a910 ConVar script_dump_simple
r5apex.exe!0x023ded30 ConVar script_error_on_midgame_load
r5apex.exe!0x0282a7d0 ConVar script_infinite_loop_ms
r5apex.exe!0x023edc10 ConVar script_parallel_trace_LOS_multiple
r5apex.exe!0x023d4630 ConVar script_precache_errors
r5apex.exe!0x0233c8e0 ConVar script_printDeferredCalls
r5apex.exe!0x023ee3a0 ConVar script_retry_after_compile_errors
r5apex.exe!0x02802ba0 ConVar script_server_fps
r5apex.exe!0x02383ae0 ConVar script_showErrorDialogs
r5apex.exe!0x0174aff0 ConVar script_slopTimeBeforeBudgetEnforcement
r5apex.exe!0x02800df0 ConVar send_data_to_all_players
r5apex.exe!0x023ec2f0 ConVar sequence_transitioner_enable
r5apex.exe!0x017562e0 ConVar serverFilter
r5apex.exe!0x018daee0 ConVar serverReports_hostname
r5apex.exe!0x01746f70 ConVar server_concommands_allways_network
r5apex.exe!0x02475ff0 ConVar server_helicopter_rope_events
r5apex.exe!0x018dcba0 ConVar server_query_interval
r5apex.exe!0x02324f70 ConVar sfm_record_hz
r5apex.exe!0x01f88c50 ConVar shadow_always_update
r5apex.exe!0x01f66c00 ConVar shadow_bleedfudge
r5apex.exe!0x0174cc00 ConVar shadow_capable
r5apex.exe!0x02343380 ConVar shadow_clear_dist
r5apex.exe!0x01f85420 ConVar shadow_dbg_draw
r5apex.exe!0x01f852e0 ConVar shadow_default_filter_size
r5apex.exe!0x02319080 ConVar shadow_depth_dimen_min
r5apex.exe!0x01f87b20 ConVar shadow_depth_upres_factor_max
r5apex.exe!0x01f92490 ConVar shadow_drawfrustum
r5apex.exe!0x01f7fdb0 ConVar shadow_dynamic_blendfactor
r5apex.exe!0x01753fc0 ConVar shadow_enable
r5apex.exe!0x01f8a5b0 ConVar shadow_esm_enable
r5apex.exe!0x01f7e490 ConVar shadow_filter_maxstep
r5apex.exe!0x02323e30 ConVar shadow_info
r5apex.exe!0x02318450 ConVar shadow_lobby_mode_allowed
r5apex.exe!0x01f86c80 ConVar shadow_max_dynamic
r5apex.exe!0x01f82e90 ConVar shadow_max_old_dynamic
r5apex.exe!0x01f8c970 ConVar shadow_max_spot_updates
r5apex.exe!0x018e4a10 ConVar shadow_maxdynamic
r5apex.exe!0x01f88d90 ConVar shadow_min_count_smallest
r5apex.exe!0x01f66de0 ConVar shadow_minvariance
r5apex.exe!0x023249f0 ConVar shadow_multisampled
r5apex.exe!0x01f81150 ConVar shadow_noLOD
r5apex.exe!0x023187f0 ConVar shadow_show_spot_udpate_infos
r5apex.exe!0x01f92bd0 ConVar shadow_tools_depth_dimen_min
r5apex.exe!0x01f89890 ConVar shadow_tools_depth_upres_factor_max
r5apex.exe!0x01f90290 ConVar shadow_tools_min_count_smallest
r5apex.exe!0x02318630 ConVar shadow_tools_mode
r5apex.exe!0x01f93b60 ConVar shadow_update_culling
r5apex.exe!0x02318010 ConVar shake_angleFactor_human
r5apex.exe!0x01f83d10 ConVar shake_angleFactor_titan
r5apex.exe!0x0233ec60 ConVar shake_basicPitchFactor
r5apex.exe!0x02341930 ConVar shake_basicRandomRollFactor
r5apex.exe!0x01f8f040 ConVar shake_offsetFactor_human
r5apex.exe!0x01f80030 ConVar shake_offsetFactor_titan
r5apex.exe!0x02319380 ConVar shake_viewmodelFactor_ads_human
r5apex.exe!0x02318750 ConVar shake_viewmodelFactor_ads_titan
r5apex.exe!0x01f810b0 ConVar shake_viewmodelFactor_human
r5apex.exe!0x02319420 ConVar shake_viewmodelFactor_titan
r5apex.exe!0x018e80e0 ConVar showfps_enabled
r5apex.exe!0x018e50c0 ConVar showfps_heightpercent
r5apex.exe!0x018e2670 ConVar showfps_mouse_latency
r5apex.exe!0x018e7820 ConVar showfps_smoothtime
r5apex.exe!0x018e8720 ConVar showfps_spinner
r5apex.exe!0x023fac40 ConVar showhitlocation
r5apex.exe!0x018e5340 ConVar showmem_enabled
r5apex.exe!0x018e8400 ConVar shownet_enabled
r5apex.exe!0x018e8040 ConVar showsnapshot_enabled
r5apex.exe!0x02815060 ConVar showtriggers
r5apex.exe!0x028122f0 ConVar showtriggers_distance
r5apex.exe!0x02803aa0 ConVar showtriggers_entindex
r5apex.exe!0x0232bc30 ConVar sidearmSwapSelectCooldown
r5apex.exe!0x02335e20 ConVar sidearmSwapSelectDoubleTapTime
r5apex.exe!0x0174db90 ConVar single_frame_shutdown_for_reload
r5apex.exe!0x017550c0 ConVar singlestep
r5apex.exe!0x02816970 ConVar sk_bullseye_health
r5apex.exe!0x023ee8a0 ConVar sk_healthcharger
r5apex.exe!0x017598d0 ConVar skill_arena
r5apex.exe!0x017593d0 ConVar skill_dediOnly
r5apex.exe!0x0175a830 ConVar skill_enabled
r5apex.exe!0x0174dcd0 ConVar skill_hostname
r5apex.exe!0x0234ba60 ConVar skip_jump_height_fraction
r5apex.exe!0x0239c330 ConVar skip_jump_height_fraction
r5apex.exe!0x02371a90 ConVar skip_jump_height_speed
r5apex.exe!0x023bc3a0 ConVar skip_jump_height_speed
r5apex.exe!0x023728b0 ConVar skip_replenish_double_jump
r5apex.exe!0x023cd3a0 ConVar skip_replenish_double_jump
r5apex.exe!0x0234aa70 ConVar skip_sounds
r5apex.exe!0x0239b4f0 ConVar skip_sounds
r5apex.exe!0x02372630 ConVar skip_speed_reduce
r5apex.exe!0x023cd050 ConVar skip_speed_reduce
r5apex.exe!0x02370ac0 ConVar skip_speed_retain
r5apex.exe!0x023bae60 ConVar skip_speed_retain
r5apex.exe!0x02372db0 ConVar skip_time
r5apex.exe!0x023cd940 ConVar skip_time
r5apex.exe!0x017581d0 ConVar sleep_when_meeting_framerate
r5apex.exe!0x0175b070 ConVar sleep_when_meeting_framerate_headroom_ms
r5apex.exe!0x023706c0 ConVar slide_auto_stand
r5apex.exe!0x023ba9c0 ConVar slide_auto_stand
r5apex.exe!0x02349f60 ConVar slide_max_angle_dot
r5apex.exe!0x0239a8e0 ConVar slide_max_angle_dot
r5apex.exe!0x0234bba0 ConVar slide_step_velocity_reduction
r5apex.exe!0x0239c4a0 ConVar slide_step_velocity_reduction
r5apex.exe!0x0237ca10 ConVar slide_viewTiltDecreaseSpeed
r5apex.exe!0x0237d3f0 ConVar slide_viewTiltIncreaseSpeed
r5apex.exe!0x02374090 ConVar slide_viewTiltPlayerSpeed
r5apex.exe!0x0237a880 ConVar slide_viewTiltSide
r5apex.exe!0x0236c6d0 ConVar slide_whileInAir
r5apex.exe!0x023b56a0 ConVar slide_whileInAir
r5apex.exe!0x01748bc0 ConVar slowconsolelog_old_logic
r5apex.exe!0x0281be90 ConVar smart_ammo_debug
r5apex.exe!0x023918e0 ConVar smart_ammo_interp_entity_fields
r5apex.exe!0x0237d0b0 ConVar smoothstairs_lunge
r5apex.exe!0x023e25d0 ConVar smoothstairs_lunge
r5apex.exe!0x01747ff0 ConVar sort_opaque_meshes
r5apex.exe!0x01f85c00 ConVar sound_classic_music
r5apex.exe!0x0233bed0 ConVar sound_entity_seek_snap
r5apex.exe!0x02377a60 ConVar sound_musicReduced
r5apex.exe!0x023250b0 ConVar sound_num_speakers
r5apex.exe!0x02385ca0 ConVar sound_only_warn_on_missing_sound_events_in_client_script
r5apex.exe!0x01820200 ConVar sound_printloaderrors
r5apex.exe!0x01f7fef0 ConVar sound_volume
r5apex.exe!0x01f8abd0 ConVar sound_volume_dialogue
r5apex.exe!0x01f8ab30 ConVar sound_volume_dialogue_sp
r5apex.exe!0x01f8d660 ConVar sound_volume_music_game
r5apex.exe!0x01f8b270 ConVar sound_volume_music_game_sp
r5apex.exe!0x01f8a410 ConVar sound_volume_music_lobby
r5apex.exe!0x01f74750 ConVar sound_volume_sfx
r5apex.exe!0x0231a900 ConVar sound_volume_sfx_sp
r5apex.exe!0x0231ab60 ConVar sound_volume_voice
r5apex.exe!0x01f93c00 ConVar sound_without_focus
r5apex.exe!0x02802ce0 ConVar soundscape_debug
r5apex.exe!0x02335d00 ConVar soundscape_fadetime
r5apex.exe!0x023341c0 ConVar soundscape_message
r5apex.exe!0x02337220 ConVar soundscape_radius_debug
r5apex.exe!0x01f8bad0 ConVar soundtrigger_repeat_interval
r5apex.exe!0x017578a0 ConVar sp_not_focus_pause
r5apex.exe!0x01f66840 ConVar spam_skinning_matrices_used
r5apex.exe!0x01f66980 ConVar spam_skinning_matrices_used_detailed
r5apex.exe!0x023d3690 ConVar spatial_partition_deadlock_assert
r5apex.exe!0x02805460 ConVar spawnpoint_avoid_npc_titan_sight
r5apex.exe!0x027fe610 ConVar spawnpoint_enemy_ai_far_dist
r5apex.exe!0x0280a740 ConVar spawnpoint_enemy_ai_near_dist
r5apex.exe!0x0280fa80 ConVar spawnpoint_enemy_titan_far_dist
r5apex.exe!0x02804900 ConVar spawnpoint_enemy_titan_near_dist
r5apex.exe!0x028109a0 ConVar spawnpoint_enemy_wallrun_far_dist
r5apex.exe!0x02811ca0 ConVar spawnpoint_enemy_wallrun_near_dist
r5apex.exe!0x02801440 ConVar spawnpoint_friendly_ai_far_dist
r5apex.exe!0x027fe6b0 ConVar spawnpoint_friendly_ai_near_dist
r5apex.exe!0x028116f0 ConVar spawnpoint_friendly_titan_far_dist
r5apex.exe!0x02804eb0 ConVar spawnpoint_friendly_titan_near_dist
r5apex.exe!0x027f84e0 ConVar spawnpoint_friendly_wallrun_far_dist
r5apex.exe!0x028140d0 ConVar spawnpoint_friendly_wallrun_near_dist
r5apex.exe!0x02810900 ConVar spawnpoint_last_spawn_rating
r5apex.exe!0x028067e0 ConVar spawnpoint_pet_titan_far_dist
r5apex.exe!0x02810530 ConVar spawnpoint_pet_titan_near_dist
r5apex.exe!0x02803c70 ConVar spawnpoint_show_all
r5apex.exe!0x02805a10 ConVar spawnpoint_show_class
r5apex.exe!0x02812570 ConVar spawnpoint_show_dist
r5apex.exe!0x027ff240 ConVar spawnpoint_show_sight
r5apex.exe!0x02802550 ConVar spawnpoint_text_dist
r5apex.exe!0x02801f00 ConVar spawnpoint_text_dynamic
r5apex.exe!0x028032b0 ConVar spawnpoint_text_team
r5apex.exe!0x028025f0 ConVar spawnpoint_velocity_predict_time
r5apex.exe!0x02813140 ConVar spec_chasecam_wait_on_dead_player_duration
r5apex.exe!0x02387dd0 ConVar speech_queue_bytes
r5apex.exe!0x018de8a0 ConVar speechtotext_audioenabled
r5apex.exe!0x018de760 ConVar speechtotext_enabled
r5apex.exe!0x018de6c0 ConVar speechtotext_forcedisabled
r5apex.exe!0x018de4e0 ConVar speechtotext_hostname
r5apex.exe!0x018de620 ConVar speechtotext_msg_droptimeout
r5apex.exe!0x018dd640 ConVar speechtotext_path
r5apex.exe!0x018de800 ConVar speechtotext_quiettime
r5apex.exe!0x018de260 ConVar speechtotext_stats_errorspermin
r5apex.exe!0x018de580 ConVar speechtotext_stats_interval
r5apex.exe!0x018de3a0 ConVar speechtotext_stats_senderrors
r5apex.exe!0x018de300 ConVar speechtotext_stats_sendrequests
r5apex.exe!0x018de440 ConVar speechtotext_stats_sendsuccess
r5apex.exe!0x0174fbf0 ConVar speechtotexttoken_hostname
r5apex.exe!0x0175cd60 ConVar speex_audio_recording
r5apex.exe!0x0175bfc0 ConVar speex_audio_value
r5apex.exe!0x018e07b0 ConVar speex_preprocess_agc_max_gain
r5apex.exe!0x018e03f0 ConVar speex_preprocess_noise_suppress
r5apex.exe!0x018e0710 ConVar speex_preprocess_set_agc_decrenment
r5apex.exe!0x018e0350 ConVar speex_preprocess_set_agc_increment
r5apex.exe!0x018e05d0 ConVar speex_preprocess_set_agc_target
r5apex.exe!0x0175c1a0 ConVar speex_quiet_threshold
r5apex.exe!0x0175c2e0 ConVar speex_quiet_window
r5apex.exe!0x018e0490 ConVar speex_set_enh
r5apex.exe!0x018e0530 ConVar speex_use_highpass
r5apex.exe!0x018e0670 ConVar speex_use_preproser
r5apex.exe!0x02380180 ConVar spinner_debug_info
r5apex.exe!0x02378750 ConVar sprint_powerdrain
r5apex.exe!0x023dd1f0 ConVar sprint_powerdrain
r5apex.exe!0x02318a50 ConVar sprint_view_shake_style
r5apex.exe!0x023d1ec0 ConVar sprinttilt_accel
r5apex.exe!0x023d4c70 ConVar sprinttilt_maxvel
r5apex.exe!0x023d1ab0 ConVar sprinttilt_turnrange
r5apex.exe!0x02340290 ConVar ss_enable
r5apex.exe!0x0233cce0 ConVar ss_force_primary_fullscreen
r5apex.exe!0x02334380 ConVar ss_mimic
r5apex.exe!0x023406d0 ConVar ss_splitmode
r5apex.exe!0x02344620 ConVar ss_verticalsplit
r5apex.exe!0x02324450 ConVar ss_viewmodelfov
r5apex.exe!0x01859370 ConVar ss_voice_hearpartner
r5apex.exe!0x019ebce0 ConVar ssao_allow_partial
r5apex.exe!0x019ebf60 ConVar ssao_blur
r5apex.exe!0x019ec320 ConVar ssao_blur_edge_sharpness
r5apex.exe!0x019ec140 ConVar ssao_depth_max
r5apex.exe!0x019ebba0 ConVar ssao_downsample
r5apex.exe!0x019ebec0 ConVar ssao_enabled
r5apex.exe!0x019ec1e0 ConVar ssao_exponent
r5apex.exe!0x019ebb00 ConVar ssao_jitter_scale
r5apex.exe!0x019ec460 ConVar ssao_max_res
r5apex.exe!0x019ebd80 ConVar ssao_max_res_threshold
r5apex.exe!0x019eba60 ConVar ssao_num_directions
r5apex.exe!0x019ec820 ConVar ssao_num_steps
r5apex.exe!0x019ec6e0 ConVar ssao_on_everything
r5apex.exe!0x019eb9c0 ConVar ssao_radius
r5apex.exe!0x019ec640 ConVar ssao_show
r5apex.exe!0x019ec960 ConVar ssao_show
r5apex.exe!0x01f696e0 ConVar ssao_show
r5apex.exe!0x019ebc40 ConVar ssao_snap_uv
r5apex.exe!0x019ec3c0 ConVar ssao_tech
r5apex.exe!0x023436a0 ConVar ssao_tech
r5apex.exe!0x019ec8c0 ConVar ssao_upsample_ranged
r5apex.exe!0x01759790 ConVar startButtonCommand
r5apex.exe!0x017583f0 ConVar staticProp_budget
r5apex.exe!0x01759970 ConVar staticProp_debug_draw
r5apex.exe!0x01758090 ConVar staticProp_earlyDepthPrepass
r5apex.exe!0x0175bae0 ConVar staticProp_earlyDepthPrepassDist
r5apex.exe!0x017596f0 ConVar staticProp_earlyDepthPrepassIncludeOpaques
r5apex.exe!0x0175b430 ConVar staticProp_earlyDepthPrepassIncludeOpaquesDist
r5apex.exe!0x0175acb0 ConVar staticProp_gather_size_weight
r5apex.exe!0x01758530 ConVar staticProp_max_scaled_dist
r5apex.exe!0x0175b4d0 ConVar staticProp_no_fade_scalar
r5apex.exe!0x023468b0 ConVar staticProp_refineDrawOnWorker
r5apex.exe!0x018e4830 ConVar static_shadow
r5apex.exe!0x01f75480 ConVar static_shadow
r5apex.exe!0x01f8a9f0 ConVar static_shadow_bounds_per_env
r5apex.exe!0x02341750 ConVar static_shadow_debug_2d
r5apex.exe!0x01f87ed0 ConVar static_shadow_debug_dirty_rects
r5apex.exe!0x023246d0 ConVar static_shadow_depth_bias_scale
r5apex.exe!0x01f83070 ConVar static_shadow_expand_z
r5apex.exe!0x01f87d90 ConVar static_shadow_good_merge_ratio
r5apex.exe!0x01f93560 ConVar static_shadow_good_merge_score
r5apex.exe!0x0231a9a0 ConVar static_shadow_prop_min_size
r5apex.exe!0x018e82c0 ConVar static_shadow_res
r5apex.exe!0x01f85380 ConVar static_shadow_shrink_culler
r5apex.exe!0x018e8540 ConVar static_shadow_use_d16
r5apex.exe!0x01f80350 ConVar static_shadow_uses_shadow_lod
r5apex.exe!0x0174f330 ConVar staticfile_hostname
r5apex.exe!0x017510f0 ConVar stats_hostname
r5apex.exe!0x02377920 ConVar status_effect_warning_level
r5apex.exe!0x023dc180 ConVar status_effect_warning_level
r5apex.exe!0x018de9c0 ConVar steam_id
r5apex.exe!0x018deb80 ConVar steam_name
r5apex.exe!0x018decc0 ConVar steamlink_hostname
r5apex.exe!0x019ea8f0 ConVar stream_addnoise
r5apex.exe!0x019ea5d0 ConVar stream_bsp_bucket_bias
r5apex.exe!0x019ead50 ConVar stream_bsp_dist_scale
r5apex.exe!0x017450d0 ConVar stream_cache_capacity
r5apex.exe!0x01744db0 ConVar stream_cache_high_priority_static_models
r5apex.exe!0x017449b0 ConVar stream_cache_multithreaded
r5apex.exe!0x01744b90 ConVar stream_cache_preload_from_rpak
r5apex.exe!0x01745030 ConVar stream_cache_read_buffer_cap
r5apex.exe!0x01745170 ConVar stream_cache_read_count_cap
r5apex.exe!0x01744f90 ConVar stream_cache_speculative_add_level
r5apex.exe!0x01745210 ConVar stream_cache_speculative_drop
r5apex.exe!0x019ea850 ConVar stream_drop_unused
r5apex.exe!0x019ea670 ConVar stream_enable
r5apex.exe!0x018e4b50 ConVar stream_freeze_camera
r5apex.exe!0x019ea530 ConVar stream_load_after_drop
r5apex.exe!0x019eadf0 ConVar stream_memory
r5apex.exe!0x019eaad0 ConVar stream_memory_ignore
r5apex.exe!0x019eaa30 ConVar stream_memory_ignore_vram
r5apex.exe!0x019ea990 ConVar stream_memory_while_loading
r5apex.exe!0x019eab70 ConVar stream_mode
r5apex.exe!0x019eac10 ConVar stream_never_high_priority_frac
r5apex.exe!0x019eae90 ConVar stream_overlay
r5apex.exe!0x019eacb0 ConVar stream_overlay_mode
r5apex.exe!0x019ea7b0 ConVar stream_pause
r5apex.exe!0x019ea710 ConVar stream_picmip
r5apex.exe!0x019eb380 ConVar stream_resource_max_commits_per_frame
r5apex.exe!0x019eb420 ConVar stream_resource_thread
r5apex.exe!0x019eb2e0 ConVar stream_resource_wait_copy_to_commit
r5apex.exe!0x019eb4c0 ConVar stream_resource_wait_creation_to_copy
r5apex.exe!0x019eb240 ConVar stream_resource_wait_for_additional_gpus
r5apex.exe!0x01756b20 ConVar stringtable_alwaysrebuilddictionaries
r5apex.exe!0x01751790 ConVar stringtable_compress
r5apex.exe!0x017528c0 ConVar stringtable_showsizes
r5apex.exe!0x018dc060 ConVar stryder_forceOriginUsersInvisible
r5apex.exe!0x018db200 ConVar stryder_security
r5apex.exe!0x0239c280 ConVar stuck_debugging
r5apex.exe!0x023d10f0 ConVar stuck_debugging_world_only
r5apex.exe!0x01744250 ConVar studiobonecache_unlimited
r5apex.exe!0x018daf80 ConVar subscription_hostname
r5apex.exe!0x0236f0d0 ConVar superjump_disabled_from_water
r5apex.exe!0x023b8cc0 ConVar superjump_disabled_from_water
r5apex.exe!0x02372d10 ConVar superjump_drain_power_onfail
r5apex.exe!0x023cd8a0 ConVar superjump_drain_power_onfail
r5apex.exe!0x0236fd60 ConVar superjump_fail_sound_when_jump_limit
r5apex.exe!0x023b9930 ConVar superjump_fail_sound_when_jump_limit
r5apex.exe!0x023baf00 ConVar superjump_limit
r5apex.exe!0x023d0c50 ConVar superjump_limitreset_onwallrun
r5apex.exe!0x023d4180 ConVar superjump_max_power_use
r5apex.exe!0x023d16a0 ConVar superjump_min_height_fraction
r5apex.exe!0x023d5080 ConVar superjump_min_power_use
r5apex.exe!0x023d0ac0 ConVar superjump_powerreset_onground
r5apex.exe!0x023ba2d0 ConVar sv_airaccelerate
r5apex.exe!0x01750190 ConVar sv_allTicksFinal
r5apex.exe!0x018581e0 ConVar sv_allowSendTableTransmitToClients
r5apex.exe!0x023ee7c0 ConVar sv_alltalk
r5apex.exe!0x02423790 ConVar sv_asyncAIInit
r5apex.exe!0x01859190 ConVar sv_asyncSendSnapshot
r5apex.exe!0x0239baa0 ConVar sv_backspeed
r5apex.exe!0x018594b0 ConVar sv_balanceTeams
r5apex.exe!0x023acc90 ConVar sv_bounce
r5apex.exe!0x02398f20 ConVar sv_bounds_show_errors
r5apex.exe!0x024768b0 ConVar sv_calcOriginsAnglesForSnapshotPacking
r5apex.exe!0x0185b430 ConVar sv_cheats
r5apex.exe!0x018588c0 ConVar sv_checkPropBudgets
r5apex.exe!0x023fe730 ConVar sv_checkTransmitEntitiesPerJob
r5apex.exe!0x02808fc0 ConVar sv_clampPlayerFrameTime
r5apex.exe!0x028128c0 ConVar sv_clockcorrection
r5apex.exe!0x028124d0 ConVar sv_clockcorrection_msecs
r5apex.exe!0x01858c50 ConVar sv_compressPlaylists
r5apex.exe!0x023eeed0 ConVar sv_compressTimeValEpsilon
r5apex.exe!0x023eef70 ConVar sv_compressTimeVals
r5apex.exe!0x01858460 ConVar sv_connectingClientDelay
r5apex.exe!0x0281abb0 ConVar sv_crossbowBoltAutoCull
r5apex.exe!0x023e24f0 ConVar sv_debug_deferred_trace
r5apex.exe!0x023ebe90 ConVar sv_debug_deferred_trace_overlay
r5apex.exe!0x01858fb0 ConVar sv_debug_prop_send
r5apex.exe!0x0185ab60 ConVar sv_debugmanualmode
r5apex.exe!0x0185afc0 ConVar sv_disconnectOnTooManySnapshotFrames
r5apex.exe!0x02423b50 ConVar sv_dispatchSpawnsForBaseline
r5apex.exe!0x024778c0 ConVar sv_distanceCull
r5apex.exe!0x0174d1b0 ConVar sv_dumpstringtables
r5apex.exe!0x0185ade0 ConVar sv_earlyPersistenceRead
r5apex.exe!0x01752c80 ConVar sv_everyThirdTick
r5apex.exe!0x0185a480 ConVar sv_extra_client_connect_time
r5apex.exe!0x023d6250 ConVar sv_footsteps
r5apex.exe!0x02435a50 ConVar sv_forceChatToTeamOnly
r5apex.exe!0x0239c680 ConVar sv_forceGrapplesToFail
r5apex.exe!0x023d2560 ConVar sv_friction
r5apex.exe!0x023d67b0 ConVar sv_gravity
r5apex.exe!0x01857d80 ConVar sv_hibernate_ms
r5apex.exe!0x01858a70 ConVar sv_hibernate_ms_vgui
r5apex.exe!0x0185ae80 ConVar sv_hibernate_postgame_delay
r5apex.exe!0x0185a840 ConVar sv_hibernate_when_empty
r5apex.exe!0x0185a0f0 ConVar sv_instancebaselines
r5apex.exe!0x023f3bc0 ConVar sv_interpolateAnimatedEntitiesPerJob
r5apex.exe!0x02476770 ConVar sv_kickPlayersTooFarInFuture
r5apex.exe!0x0280fdd0 ConVar sv_lagpushticks
r5apex.exe!0x023f25d0 ConVar sv_lerpAnims
r5apex.exe!0x0174a7f0 ConVar sv_loadMapModelEarly
r5apex.exe!0x01754060 ConVar sv_lobbyType
r5apex.exe!0x02477960 ConVar sv_massreport
r5apex.exe!0x02808a10 ConVar sv_maxUserCmdsPerPlayerPerFrame
r5apex.exe!0x018589d0 ConVar sv_max_prop_data_dwords_huge_lobby
r5apex.exe!0x0185a340 ConVar sv_max_prop_data_dwords_huge_multiplayer
r5apex.exe!0x01858820 ConVar sv_max_prop_data_dwords_lobby
r5apex.exe!0x01859230 ConVar sv_max_prop_data_dwords_multiplayer
r5apex.exe!0x018585a0 ConVar sv_max_prop_data_dwords_singleplayer
r5apex.exe!0x0185b240 ConVar sv_max_props_huge_lobby
r5apex.exe!0x01857e20 ConVar sv_max_props_huge_multiplayer
r5apex.exe!0x01858bb0 ConVar sv_max_props_lobby
r5apex.exe!0x01858500 ConVar sv_max_props_multiplayer
r5apex.exe!0x01858320 ConVar sv_max_props_singleplayer
r5apex.exe!0x01859fb0 ConVar sv_max_snapshots_lobby
r5apex.exe!0x018576d0 ConVar sv_max_snapshots_multiplayer
r5apex.exe!0x0185b100 ConVar sv_max_snapshots_singleplayer
r5apex.exe!0x01858e00 ConVar sv_maxclientframes
r5apex.exe!0x0185a7a0 ConVar sv_maxrate
r5apex.exe!0x01750b90 ConVar sv_maxroutable
r5apex.exe!0x023d5730 ConVar sv_maxspeed
r5apex.exe!0x02815c10 ConVar sv_maxunlag
r5apex.exe!0x01859e70 ConVar sv_maxupdaterate
r5apex.exe!0x023d57d0 ConVar sv_maxvelocity
r5apex.exe!0x01858cf0 ConVar sv_minrate
r5apex.exe!0x018590f0 ConVar sv_minupdaterate
r5apex.exe!0x023f7b00 ConVar sv_netvisdist
r5apex.exe!0x023d3060 ConVar sv_noclipaccelerate
r5apex.exe!0x023d4420 ConVar sv_noclipaccelerate_fast
r5apex.exe!0x023d5d10 ConVar sv_noclipaccelerate_slow
r5apex.exe!0x023d54d0 ConVar sv_noclipspeed
r5apex.exe!0x023d4080 ConVar sv_noclipspeed_fast
r5apex.exe!0x023d3cd0 ConVar sv_noclipspeed_slow
r5apex.exe!0x0280a100 ConVar sv_normalSimulationCommandThreshold
r5apex.exe!0x023d4ee0 ConVar sv_optimizedmovement
r5apex.exe!0x01858000 ConVar sv_parallel_sendsnapshot
r5apex.exe!0x01857ce0 ConVar sv_pausable
r5apex.exe!0x02421f00 ConVar sv_physics_maxvelocity
r5apex.exe!0x018574f0 ConVar sv_playerNameAppendCheater
r5apex.exe!0x02806f70 ConVar sv_playerSimTimeBuffer
r5apex.exe!0x023cd620 ConVar sv_players
r5apex.exe!0x02801fa0 ConVar sv_printClockCorrections
r5apex.exe!0x02806880 ConVar sv_printClockTiming
r5apex.exe!0x0185a230 ConVar sv_printHighWaterMark
r5apex.exe!0x02477790 ConVar sv_printNetReports
r5apex.exe!0x02434dc0 ConVar sv_printSnapshotDeltaStats
r5apex.exe!0x027f8750 ConVar sv_props_funnel_into_portals
r5apex.exe!0x02812960 ConVar sv_props_funnel_into_portals_deceleration
r5apex.exe!0x023d4840 ConVar sv_pushaway_accel
r5apex.exe!0x023d1d20 ConVar sv_pushaway_clientside
r5apex.exe!0x023dc920 ConVar sv_pushaway_clientside_size
r5apex.exe!0x023d4f80 ConVar sv_pushaway_debug
r5apex.exe!0x023d2870 ConVar sv_pushaway_dist
r5apex.exe!0x023d2fc0 ConVar sv_pushaway_min_player_speed
r5apex.exe!0x023d4d70 ConVar sv_pushaway_player_accel
r5apex.exe!0x023d2b70 ConVar sv_pushaway_player_dist
r5apex.exe!0x023fe9b0 ConVar sv_recalcOrigins_enabled
r5apex.exe!0x02423010 ConVar sv_recalcOrigins_entsPerJob
r5apex.exe!0x01859700 ConVar sv_rejectClientConnects
r5apex.exe!0x01857a60 ConVar sv_rejectConnections
r5apex.exe!0x01858640 ConVar sv_requireOriginToken
r5apex.exe!0x018597a0 ConVar sv_resendSignonData
r5apex.exe!0x023d04d0 ConVar sv_rollangle
r5apex.exe!0x023d0e20 ConVar sv_rollspeed
r5apex.exe!0x018579c0 ConVar sv_runSpatialOptimizeInJob
r5apex.exe!0x01859f10 ConVar sv_scarySnapDeltaPrints
r5apex.exe!0x027f86b0 ConVar sv_screenShake_debug
r5apex.exe!0x02810710 ConVar sv_screenShake_enabled
r5apex.exe!0x02806060 ConVar sv_screenShake_maxAmplitude
r5apex.exe!0x0280b710 ConVar sv_scriptCompileAsync
r5apex.exe!0x02814720 ConVar sv_script_perf_dump_on_shutdown
r5apex.exe!0x023f6920 ConVar sv_script_think_interval
r5apex.exe!0x0185a980 ConVar sv_sendEarlyServerInfo
r5apex.exe!0x028147c0 ConVar sv_sendPlayerDamageMsg
r5apex.exe!0x01858b10 ConVar sv_sendReplayNetMessagesOnNoDeltaSnaps
r5apex.exe!0x018598e0 ConVar sv_separate_freq_change_prop_send
r5apex.exe!0x02812390 ConVar sv_shiftPlayerSimTimeBackwards
r5apex.exe!0x01857ec0 ConVar sv_showClientTickCmds
r5apex.exe!0x01857770 ConVar sv_showLargeSnapshotSize
r5apex.exe!0x01857450 ConVar sv_showSnapshots
r5apex.exe!0x01857630 ConVar sv_showUserCmds
r5apex.exe!0x023fec40 ConVar sv_showWeirdDeltas
r5apex.exe!0x02818860 ConVar sv_show_placement_help_in_preview
r5apex.exe!0x023b8b80 ConVar sv_showfiredbullets
r5apex.exe!0x023fe690 ConVar sv_showhitboxes
r5apex.exe!0x02807a30 ConVar sv_showlagcompensation
r5apex.exe!0x0185a700 ConVar sv_single_core_dedi
r5apex.exe!0x01857920 ConVar sv_skipSendingUnnecessaryPersistence
r5apex.exe!0x023ba230 ConVar sv_skyname
r5apex.exe!0x01859dd0 ConVar sv_snapshot_uniform_interval
r5apex.exe!0x023f7c10 ConVar sv_spawnAIHintsInMP
r5apex.exe!0x023d2ce0 ConVar sv_specaccelerate
r5apex.exe!0x023d3e40 ConVar sv_specnoclip
r5apex.exe!0x023d18e0 ConVar sv_specspeed
r5apex.exe!0x018573b0 ConVar sv_stats
r5apex.exe!0x0239bb40 ConVar sv_stopspeed
r5apex.exe!0x0185a8e0 ConVar sv_stressbots
r5apex.exe!0x0185ac00 ConVar sv_struggleCheck
r5apex.exe!0x01858140 ConVar sv_struggleSpam
r5apex.exe!0x0185a5c0 ConVar sv_struggleSpamInterval
r5apex.exe!0x018592d0 ConVar sv_tempents_send_from_delta
r5apex.exe!0x01857590 ConVar sv_tempents_send_from_last_sent
r5apex.exe!0x018e0210 ConVar sv_testLargeDatablock
r5apex.exe!0x027f82a0 ConVar sv_teststepsimulation
r5apex.exe!0x023dec90 ConVar sv_thinktimecheck
r5apex.exe!0x023f73a0 ConVar sv_threaded_post_process_ai
r5apex.exe!0x027f8910 ConVar sv_threaded_post_process_players
r5apex.exe!0x023bc440 ConVar sv_threaded_pre_process_ents
r5apex.exe!0x028158e0 ConVar sv_turbophysics
r5apex.exe!0x02815840 ConVar sv_turbophysics_player
r5apex.exe!0x02812610 ConVar sv_unlag
r5apex.exe!0x0280fe70 ConVar sv_unlag_debug
r5apex.exe!0x01859660 ConVar sv_unnecessaryConnectDelay
r5apex.exe!0x01858280 ConVar sv_unreliableSnapMaxSize
r5apex.exe!0x01857c40 ConVar sv_updaterate_mp
r5apex.exe!0x0185a050 ConVar sv_updaterate_sp
r5apex.exe!0x023ed830 ConVar sv_useRK4forprojectiles
r5apex.exe!0x018583c0 ConVar sv_useReputation
r5apex.exe!0x0185a190 ConVar sv_useThreadsForSnapshots
r5apex.exe!0x02434d20 ConVar sv_usercmd_before_entities
r5apex.exe!0x02814fc0 ConVar sv_usercmd_fairness
r5apex.exe!0x02814de0 ConVar sv_usercmd_fairness_dediOnly
r5apex.exe!0x02421dc0 ConVar sv_usercmd_max_queued
r5apex.exe!0x024236f0 ConVar sv_usercmd_num_per_iteration
r5apex.exe!0x024354a0 ConVar sv_usercmd_shuffle_players
r5apex.exe!0x023ebf30 ConVar sv_visualizetraces
r5apex.exe!0x023ebfd0 ConVar sv_visualizetraces_duration
r5apex.exe!0x01858ea0 ConVar sv_voiceDebug
r5apex.exe!0x0185aa20 ConVar sv_voiceEcho
r5apex.exe!0x0185a520 ConVar sv_voiceenable
r5apex.exe!0x01858780 ConVar sv_warnAboutCmdNumJumps
r5apex.exe!0x01757240 ConVar sv_watchdogTimer
r5apex.exe!0x02398730 ConVar sv_wateraccelerate
r5apex.exe!0x023cdb20 ConVar sv_waterdist
r5apex.exe!0x0281bd50 ConVar sv_weapon_despawn_time
r5apex.exe!0x01755020 ConVar sv_writePersistenceOnShutdown
r5apex.exe!0x02384640 ConVar sys_attract_mode_timeout
r5apex.exe!0x018def40 ConVar sys_minidumpexpandedspew
r5apex.exe!0x018df610 ConVar sys_minidumpspewlines
r5apex.exe!0x01759dd0 ConVar system_alt_f4_closes_window
r5apex.exe!0x023f4130 ConVar teamSpot_costLimitPerFrame
r5apex.exe!0x023fa670 ConVar teamSpot_enabled
r5apex.exe!0x023f7260 ConVar teamSpot_lockOffTime
r5apex.exe!0x023ef050 ConVar teamSpot_lockOnTime
r5apex.exe!0x023f9a40 ConVar teamSpot_lockOnTimeForgiveness
r5apex.exe!0x023ef230 ConVar teamSpot_minimap_enabled
r5apex.exe!0x023f9900 ConVar teamSpot_threaded
r5apex.exe!0x023d51f0 ConVar teams_unassigned_are_friendly
r5apex.exe!0x01755b80 ConVar telemetry_client_debug
r5apex.exe!0x0174e1e0 ConVar telemetry_client_enable
r5apex.exe!0x01752be0 ConVar telemetry_client_sendInterval
r5apex.exe!0x02812820 ConVar template_debug
r5apex.exe!0x0280fbc0 ConVar test_massive_dmg
r5apex.exe!0x028105d0 ConVar test_massive_dmg_clip
r5apex.exe!0x023bb7d0 ConVar tether_damageScale
r5apex.exe!0x02398ae0 ConVar tether_dodge_damage
r5apex.exe!0x02398870 ConVar tether_healthDrain
r5apex.exe!0x0239bc80 ConVar tether_healthDrainNPC
r5apex.exe!0x023d4b10 ConVar tether_maxvel
r5apex.exe!0x023f8d90 ConVar tether_npc_strength
r5apex.exe!0x023d59d0 ConVar tether_radius
r5apex.exe!0x023d3160 ConVar tether_strength
r5apex.exe!0x023ee300 ConVar think_limit
r5apex.exe!0x0232fa70 ConVar thirdperson_mayamode
r5apex.exe!0x02324bd0 ConVar thirdperson_override
r5apex.exe!0x023384d0 ConVar thirdperson_screenspace
r5apex.exe!0x02806920 ConVar threat_detection_in_job
r5apex.exe!0x01749c70 ConVar timeout
r5apex.exe!0x01749d10 ConVar timeout_during_load
r5apex.exe!0x023f8490 ConVar titanSoul_debug
r5apex.exe!0x027f8340 ConVar titan_hideEnts
r5apex.exe!0x02813320 ConVar titan_hidePlayer
r5apex.exe!0x0237e520 ConVar titan_sprint_sound
r5apex.exe!0x0282a610 ConVar titan_step_damage_can_push_down
r5apex.exe!0x0282a570 ConVar titan_step_damage_debug
r5apex.exe!0x0282a6b0 ConVar titan_step_damage_rodeo_immunity_time
r5apex.exe!0x0185b4d0 ConVar tracehull_height_error_check
r5apex.exe!0x02332d60 ConVar tracer_debug
r5apex.exe!0x0232a130 ConVar tracer_extra
r5apex.exe!0x0238c070 ConVar trail_optimizedRemove
r5apex.exe!0x023d1f60 ConVar traversal_anim
r5apex.exe!0x02349d80 ConVar traversal_cooldown
r5apex.exe!0x0239a700 ConVar traversal_cooldown
r5apex.exe!0x0236f7e0 ConVar traversal_enable
r5apex.exe!0x023b93b0 ConVar traversal_enable
r5apex.exe!0x0234b210 ConVar traversal_hand_debug
r5apex.exe!0x0239c040 ConVar traversal_hand_debug
r5apex.exe!0x0236fb80 ConVar traversal_hand_required_width
r5apex.exe!0x023b96d0 ConVar traversal_hand_required_width
r5apex.exe!0x02827250 ConVar traversal_viewLerpInDuration
r5apex.exe!0x02390d00 ConVar traversal_viewLerpOut
r5apex.exe!0x02393ba0 ConVar traversal_viewLerpOutAngle
r5apex.exe!0x02388cf0 ConVar traversal_viewLerpOutDebug
r5apex.exe!0x023947c0 ConVar traversal_viewLerpOutPos
r5apex.exe!0x0234abb0 ConVar traversal_window_duration
r5apex.exe!0x0239b5c0 ConVar traversal_window_duration
r5apex.exe!0x02349e20 ConVar traversal_window_enable
r5apex.exe!0x0239a7a0 ConVar traversal_window_enable
r5apex.exe!0x0234b0d0 ConVar traversal_window_finish_angle
r5apex.exe!0x0239be60 ConVar traversal_window_finish_angle
r5apex.exe!0x0236eeb0 ConVar traversal_window_forward_offset
r5apex.exe!0x023b8ae0 ConVar traversal_window_forward_offset
r5apex.exe!0x023729f0 ConVar traversal_window_hand_vertical_offset
r5apex.exe!0x023cd440 ConVar traversal_window_hand_vertical_offset
r5apex.exe!0x02370d10 ConVar traversal_window_sideways_offset
r5apex.exe!0x023bb180 ConVar traversal_window_sideways_offset
r5apex.exe!0x01f87c70 ConVar traversal_window_view_pitch_max
r5apex.exe!0x01f82f30 ConVar traversal_window_view_pitch_min
r5apex.exe!0x01f97510 ConVar traversal_window_yaw_max
r5apex.exe!0x023718b0 ConVar trigger_ignore_nonsolids
r5apex.exe!0x023bc080 ConVar trigger_ignore_nonsolids
r5apex.exe!0x02806230 ConVar trigger_touch_on_spawn
r5apex.exe!0x027ff2e0 ConVar trigger_use_new_filters
r5apex.exe!0x01f6a220 ConVar tsaa_blendfactorincreaseatmaxvelocity
r5apex.exe!0x01f6a2c0 ConVar tsaa_blendfactorincreasewhenunoccluded
r5apex.exe!0x01f6a400 ConVar tsaa_blendfactormaxesoutatvelocity
r5apex.exe!0x01f6a5e0 ConVar tsaa_blendfactormodulationonsparklesandunocclusion
r5apex.exe!0x01f6a4a0 ConVar tsaa_blendfactoroverride
r5apex.exe!0x01f6a360 ConVar tsaa_curframeblendamount
r5apex.exe!0x01f6a540 ConVar tsaa_debugresponsiveflag
r5apex.exe!0x01f6a0e0 ConVar tsaa_neighborhoodclamping
r5apex.exe!0x01f6a180 ConVar tsaa_neighborhoodclampingsoftened
r5apex.exe!0x023449d0 ConVar tsaa_numsamples
r5apex.exe!0x023393e0 ConVar tweak_light_shadows_every_frame
r5apex.exe!0x0181e270 ConVar twitch_check_interval
r5apex.exe!0x0181e450 ConVar twitch_prime_linked
r5apex.exe!0x0181e930 ConVar twitch_shouldQuery
r5apex.exe!0x02384420 ConVar ui_fadecloud_time
r5apex.exe!0x02384380 ConVar ui_fadexui_time
r5apex.exe!0x023835e0 ConVar ui_gameui_ctrlr_title
r5apex.exe!0x02383d60 ConVar ui_gameui_modal
r5apex.exe!0x02383860 ConVar ui_loadingscreen_autotransition_time
r5apex.exe!0x02383fe0 ConVar ui_loadingscreen_fadein_time
r5apex.exe!0x017487d0 ConVar ui_loadingscreen_fadeout_time
r5apex.exe!0x02384080 ConVar ui_loadingscreen_fadeout_time
r5apex.exe!0x02384960 ConVar ui_loadingscreen_mintransition_time
r5apex.exe!0x02383b80 ConVar ui_loadingscreen_transition_time
r5apex.exe!0x023837c0 ConVar ui_lobby_jointimeout
r5apex.exe!0x023848c0 ConVar ui_lobby_noautostart
r5apex.exe!0x023842e0 ConVar ui_lobby_noresults_create_msg_time
r5apex.exe!0x023815f0 ConVar ui_posedebug_fade_in_time
r5apex.exe!0x02381550 ConVar ui_posedebug_fade_out_time
r5apex.exe!0x02383720 ConVar ui_virtualnav_render
r5apex.exe!0x02348960 ConVar unique_entity_names
r5apex.exe!0x0239a660 ConVar unique_entity_names
r5apex.exe!0x02385fc0 ConVar usePromptBaseColor
r5apex.exe!0x02387bf0 ConVar usePromptButtonTextColor
r5apex.exe!0x02385d40 ConVar usePromptImageScale
r5apex.exe!0x02387760 ConVar usePromptImageYOffset
r5apex.exe!0x02388730 ConVar usePromptTextColor
r5apex.exe!0x02343160 ConVar use_monitors
r5apex.exe!0x0175bde0 ConVar use_valve_auto_gain
r5apex.exe!0x018db980 ConVar user_tracking_enabled
r5apex.exe!0x0174d070 ConVar users_hostname
r5apex.exe!0x01f74570 ConVar v_centermove
r5apex.exe!0x01f82fd0 ConVar v_centerspeed
r5apex.exe!0x02379eb0 ConVar variable_sights_gravity_scale_override
r5apex.exe!0x01f6e410 ConVar vgui_EnableFixedAspectScaling
r5apex.exe!0x02384120 ConVar vgui_drawPolyShapes
r5apex.exe!0x01758270 ConVar vgui_drawfocus
r5apex.exe!0x01f6e060 ConVar vgui_drawfocus
r5apex.exe!0x0175a0f0 ConVar vgui_drawkeyfocus
r5apex.exe!0x01759470 ConVar vgui_drawtree
r5apex.exe!0x0175ab70 ConVar vgui_drawtree_bounds
r5apex.exe!0x0175ba40 ConVar vgui_drawtree_draw_selected
r5apex.exe!0x01759f10 ConVar vgui_drawtree_freeze
r5apex.exe!0x01757620 ConVar vgui_drawtree_hidden
r5apex.exe!0x0175a530 ConVar vgui_drawtree_panelalpha
r5apex.exe!0x017589d0 ConVar vgui_drawtree_panelptr
r5apex.exe!0x017586b0 ConVar vgui_drawtree_popupsonly
r5apex.exe!0x017576c0 ConVar vgui_drawtree_render_order
r5apex.exe!0x0175b110 ConVar vgui_drawtree_scheme
r5apex.exe!0x01759a10 ConVar vgui_drawtree_visible
r5apex.exe!0x01f749d0 ConVar vgui_interactive
r5apex.exe!0x01f6fe90 ConVar vgui_noquads
r5apex.exe!0x01f6fd50 ConVar vgui_notext
r5apex.exe!0x01f6fad0 ConVar vgui_paintEnabled
r5apex.exe!0x01f6e100 ConVar vgui_resize_on_resolution_change
r5apex.exe!0x01f6ea50 ConVar vgui_show_glyph_miss
r5apex.exe!0x01754a80 ConVar vgui_simulate_during_bone_setup
r5apex.exe!0x02386c70 ConVar video_menu_uiscript_reset
r5apex.exe!0x023ed6f0 ConVar viewDrift
r5apex.exe!0x023ecc00 ConVar viewDrift_ads_delay_debounce_time
r5apex.exe!0x023ede00 ConVar viewDrift_pitch_base1_amp
r5apex.exe!0x023ed050 ConVar viewDrift_pitch_base1_freq
r5apex.exe!0x023ecf10 ConVar viewDrift_pitch_base1_phase
r5apex.exe!0x023ec8e0 ConVar viewDrift_pitch_base2_amp
r5apex.exe!0x023ee120 ConVar viewDrift_pitch_base2_freq
r5apex.exe!0x023ed650 ConVar viewDrift_pitch_base2_phase
r5apex.exe!0x023ed230 ConVar viewDrift_pitch_scaler_amp
r5apex.exe!0x023ee260 ConVar viewDrift_pitch_scaler_base
r5apex.exe!0x023ed190 ConVar viewDrift_pitch_scaler_freq
r5apex.exe!0x023ed0f0 ConVar viewDrift_pitch_scaler_phase
r5apex.exe!0x023edd60 ConVar viewDrift_pitch_shifter_amp
r5apex.exe!0x023ecac0 ConVar viewDrift_pitch_shifter_freq
r5apex.exe!0x023edfe0 ConVar viewDrift_pitch_shifter_phase
r5apex.exe!0x023ee1c0 ConVar viewDrift_yaw_base1_amp
r5apex.exe!0x023ec430 ConVar viewDrift_yaw_base1_freq
r5apex.exe!0x023ec4d0 ConVar viewDrift_yaw_base1_phase
r5apex.exe!0x023ed790 ConVar viewDrift_yaw_base2_amp
r5apex.exe!0x023edb70 ConVar viewDrift_yaw_base2_freq
r5apex.exe!0x023ec390 ConVar viewDrift_yaw_base2_phase
r5apex.exe!0x023ed3e0 ConVar viewDrift_yaw_scaler_amp
r5apex.exe!0x023ec840 ConVar viewDrift_yaw_scaler_base
r5apex.exe!0x023ee080 ConVar viewDrift_yaw_scaler_freq
r5apex.exe!0x023ec980 ConVar viewDrift_yaw_scaler_phase
r5apex.exe!0x023ec110 ConVar viewDrift_yaw_shifter_amp
r5apex.exe!0x023edad0 ConVar viewDrift_yaw_shifter_freq
r5apex.exe!0x023edea0 ConVar viewDrift_yaw_shifter_phase
r5apex.exe!0x023ee440 ConVar view_offset_entity_enable
r5apex.exe!0x0233bb10 ConVar viewangle_debug
r5apex.exe!0x01f83ef0 ConVar viewangles_simpler
r5apex.exe!0x01f74ed0 ConVar viewmodelShake
r5apex.exe!0x01f938e0 ConVar viewmodelShake_sourceRollRange
r5apex.exe!0x02343ce0 ConVar viewmodel_bounds_draw
r5apex.exe!0x0233e410 ConVar viewmodel_bounds_draw_lock
r5apex.exe!0x018e85e0 ConVar viewmodel_selfshadow
r5apex.exe!0x02343420 ConVar viewmodel_selfshadow_debug_2d
r5apex.exe!0x0233d0c0 ConVar viewmodel_selfshadow_tightbounds
r5apex.exe!0x01f80170 ConVar viewportscale
r5apex.exe!0x023d44f0 ConVar viewpunch_base_springConstantX
r5apex.exe!0x023d2e20 ConVar viewpunch_base_springConstantY
r5apex.exe!0x023d5660 ConVar viewpunch_base_springConstantZ
r5apex.exe!0x023d3560 ConVar viewpunch_base_springDampingX
r5apex.exe!0x023d2390 ConVar viewpunch_base_springDampingY
r5apex.exe!0x023d3a90 ConVar viewpunch_base_springDampingZ
r5apex.exe!0x02809060 ConVar viewpunch_predictable_scalar
r5apex.exe!0x0174c3d0 ConVar violence_ablood
r5apex.exe!0x02376d80 ConVar violence_ablood
r5apex.exe!0x023db400 ConVar violence_ablood
r5apex.exe!0x0174ecc0 ConVar violence_agibs
r5apex.exe!0x02378cd0 ConVar violence_agibs
r5apex.exe!0x023dd920 ConVar violence_agibs
r5apex.exe!0x01756080 ConVar violence_hblood
r5apex.exe!0x0237de80 ConVar violence_hblood
r5apex.exe!0x023e30a0 ConVar violence_hblood
r5apex.exe!0x01750a50 ConVar violence_hgibs
r5apex.exe!0x0237aed0 ConVar violence_hgibs
r5apex.exe!0x023e00a0 ConVar violence_hgibs
r5apex.exe!0x0237df20 ConVar visible_ent_cone_debug_duration_client
r5apex.exe!0x023dd880 ConVar visible_ent_cone_debug_duration_server
r5apex.exe!0x0175d120 ConVar voice_absTriggerAmount
r5apex.exe!0x02388b30 ConVar voice_allow_mute_self
r5apex.exe!0x0175c240 ConVar voice_avggain
r5apex.exe!0x02380360 ConVar voice_clientdebug
r5apex.exe!0x0175c600 ConVar voice_debugAddSecondTalker
r5apex.exe!0x0175ccc0 ConVar voice_debugThresholds
r5apex.exe!0x01758310 ConVar voice_debugfeedback
r5apex.exe!0x023874b0 ConVar voice_decimate_at_bytes
r5apex.exe!0x02388a10 ConVar voice_decimate_rate
r5apex.exe!0x0175ca40 ConVar voice_enabled
r5apex.exe!0x0175c740 ConVar voice_energyPerZeroThreshold
r5apex.exe!0x0175cc20 ConVar voice_energyThreshold
r5apex.exe!0x0175cae0 ConVar voice_forcemicrecord
r5apex.exe!0x0174daf0 ConVar voice_inputfromfile
r5apex.exe!0x01f744d0 ConVar voice_late_update
r5apex.exe!0x0175cea0 ConVar voice_loopback
r5apex.exe!0x0175d080 ConVar voice_maxgain
r5apex.exe!0x0175c880 ConVar voice_minEnergyPerZeroThreshold
r5apex.exe!0x0175d1c0 ConVar voice_mixer_boost
r5apex.exe!0x0175d260 ConVar voice_mixer_mute
r5apex.exe!0x0175d300 ConVar voice_mixer_volume
r5apex.exe!0x023809b0 ConVar voice_modenable
r5apex.exe!0x0185aac0 ConVar voice_noxplat
r5apex.exe!0x0175cfe0 ConVar voice_profile
r5apex.exe!0x0174fdd0 ConVar voice_recordtofile
r5apex.exe!0x0175c380 ConVar voice_scale
r5apex.exe!0x023ee700 ConVar voice_serverdebug
r5apex.exe!0x0175c9a0 ConVar voice_showchannels
r5apex.exe!0x0175c060 ConVar voice_showincoming
r5apex.exe!0x0175c100 ConVar voice_threshold_delay
r5apex.exe!0x0175ce00 ConVar voice_triggerCrossingRate
r5apex.exe!0x0175c560 ConVar voice_triggerRate
r5apex.exe!0x0175cb80 ConVar voice_vox
r5apex.exe!0x0175bf20 ConVar voice_writevoices
r5apex.exe!0x0174d570 ConVar voice_xsend_debug
r5apex.exe!0x0175c420 ConVar voice_zeroCrossingThreshold
r5apex.exe!0x0281aed0 ConVar vortex_damageimpulsescale
r5apex.exe!0x02423bf0 ConVar vprof_scope_entity_gamephys
r5apex.exe!0x02475c90 ConVar vprof_scope_entity_thinks
r5apex.exe!0x01756420 ConVar vprof_server_spike_threshold
r5apex.exe!0x0174f470 ConVar vprof_server_thread
r5apex.exe!0x02424400 ConVar vprof_think_limit
r5apex.exe!0x02344c70 ConVar vscript_ui_do_delay_init
r5apex.exe!0x01f92670 ConVar vsm_culling
r5apex.exe!0x0231a360 ConVar vsm_ignore_edge_planes
r5apex.exe!0x01f755c0 ConVar vsm_ignore_face_planes
r5apex.exe!0x01757e10 ConVar vx_do_not_throttle_events
r5apex.exe!0x02379e10 ConVar wall_climb_pose_paramteter_hands_enabled
r5apex.exe!0x023deb50 ConVar wall_climb_pose_paramteter_hands_enabled
r5apex.exe!0x02371770 ConVar wallclimb_vertical_gain_reduction
r5apex.exe!0x023bbea0 ConVar wallclimb_vertical_gain_reduction
r5apex.exe!0x02372e50 ConVar wallrun_angleChangeMinCos
r5apex.exe!0x023cdce0 ConVar wallrun_angleChangeMinCos
r5apex.exe!0x02372c70 ConVar wallrun_avoid_wall_top_decel
r5apex.exe!0x023cd800 ConVar wallrun_avoid_wall_top_decel
r5apex.exe!0x028270e0 ConVar wallrun_curveDebug
r5apex.exe!0x02826d40 ConVar wallrun_curveEnable
r5apex.exe!0x023d3ee0 ConVar wallrun_debug
r5apex.exe!0x023d3da0 ConVar wallrun_enable
r5apex.exe!0x02370c70 ConVar wallrun_fallAwaySpeed
r5apex.exe!0x023bb0e0 ConVar wallrun_fallAwaySpeed
r5apex.exe!0x02370a20 ConVar wallrun_hangStopTime
r5apex.exe!0x023badc0 ConVar wallrun_hangStopTime
r5apex.exe!0x02349ec0 ConVar wallrun_hangslipduration
r5apex.exe!0x0239a840 ConVar wallrun_hangslipduration
r5apex.exe!0x0236f880 ConVar wallrun_hangslipstarttime
r5apex.exe!0x023b9450 ConVar wallrun_hangslipstarttime
r5apex.exe!0x0236e7f0 ConVar wallrun_hangslipvel
r5apex.exe!0x023b83f0 ConVar wallrun_hangslipvel
r5apex.exe!0x023cfab0 ConVar wallrun_maxViewTilt
r5apex.exe!0x0235c230 ConVar wallrun_minAngle_air
r5apex.exe!0x023acbf0 ConVar wallrun_minAngle_air
r5apex.exe!0x02370b60 ConVar wallrun_noInputSlipFrac
r5apex.exe!0x023bafa0 ConVar wallrun_noInputSlipFrac
r5apex.exe!0x023d1500 ConVar wallrun_pushAwayFallOffTime
r5apex.exe!0x0236c630 ConVar wallrun_repelEnable
r5apex.exe!0x023b5580 ConVar wallrun_repelEnable
r5apex.exe!0x0236f030 ConVar wallrun_repelSoftness
r5apex.exe!0x023b8c20 ConVar wallrun_repelSoftness
r5apex.exe!0x02372770 ConVar wallrun_repelTimeMax
r5apex.exe!0x023cd260 ConVar wallrun_repelTimeMax
r5apex.exe!0x0236f920 ConVar wallrun_repelTimeMin
r5apex.exe!0x023b94f0 ConVar wallrun_repelTimeMin
r5apex.exe!0x0236dac0 ConVar wallrun_retry_interval
r5apex.exe!0x023b70c0 ConVar wallrun_retry_interval
r5apex.exe!0x023d4910 ConVar wallrun_rotateMaxRate
r5apex.exe!0x023d2a10 ConVar wallrun_sameWallDist
r5apex.exe!0x023d47a0 ConVar wallrun_sameWallDot
r5apex.exe!0x023d2430 ConVar wallrun_sameWallSlope
r5apex.exe!0x023726d0 ConVar wallrun_slipduration
r5apex.exe!0x023cd0f0 ConVar wallrun_slipduration
r5apex.exe!0x0236dc50 ConVar wallrun_slipslowdown
r5apex.exe!0x023b77f0 ConVar wallrun_slipslowdown
r5apex.exe!0x02370800 ConVar wallrun_slipstarttime
r5apex.exe!0x023baba0 ConVar wallrun_slipstarttime
r5apex.exe!0x02371950 ConVar wallrun_slipvel
r5apex.exe!0x023bc120 ConVar wallrun_slipvel
r5apex.exe!0x0234b030 ConVar wallrun_strengthLossEnd
r5apex.exe!0x0239bdc0 ConVar wallrun_strengthLossEnd
r5apex.exe!0x0234acf0 ConVar wallrun_strengthLossStart
r5apex.exe!0x0239b700 ConVar wallrun_strengthLossStart
r5apex.exe!0x0236fc20 ConVar wallrun_upwardAutoPush
r5apex.exe!0x023b9770 ConVar wallrun_upwardAutoPush
r5apex.exe!0x023719f0 ConVar wallrun_viewTiltPredictTime
r5apex.exe!0x023bc300 ConVar wallrun_viewTiltPredictTime
r5apex.exe!0x023cfff0 ConVar wallrun_viewTiltSpeed
r5apex.exe!0x023d17a0 ConVar was_loaded
r5apex.exe!0x02814030 ConVar weaponAmmoPickupSound
r5apex.exe!0x02388db0 ConVar weaponFastHolsterScale
r5apex.exe!0x02818ee0 ConVar weaponFastHolsterScale
r5apex.exe!0x02372b30 ConVar weaponSwitch3p_checkNewWeapon
r5apex.exe!0x023cd580 ConVar weaponSwitch3p_checkNewWeapon
r5apex.exe!0x02393b00 ConVar weaponSwitch3p_onHolster
r5apex.exe!0x028240d0 ConVar weaponSwitch3p_onHolster
r5apex.exe!0x023df8e0 ConVar weapon_auto_swap_ordnance_no_ammo
r5apex.exe!0x02392b20 ConVar weapon_debugScript
r5apex.exe!0x02823230 ConVar weapon_debugScript
r5apex.exe!0x02391840 ConVar weapon_doIdleForSurvivalMelee
r5apex.exe!0x028220d0 ConVar weapon_doIdleForSurvivalMelee
r5apex.exe!0x01f97810 ConVar weapon_friendly_fire_prevent_ui
r5apex.exe!0x023ecfb0 ConVar weapon_meleeButtonPressProtection
r5apex.exe!0x0231a860 ConVar weapon_parentingFixLerp
r5apex.exe!0x023ed480 ConVar weapon_pickup_allow_dupes
r5apex.exe!0x01f75150 ConVar weapon_poseParamMaxDistance
r5apex.exe!0x02393480 ConVar weapon_render_with_fastpath
r5apex.exe!0x0281af70 ConVar weapon_showproficiency
r5apex.exe!0x028272f0 ConVar weapon_sprint_raise_delay
r5apex.exe!0x023946a0 ConVar weaponx_predicting_client_only_optimization
r5apex.exe!0x02824c30 ConVar weaponx_predicting_client_only_optimization
r5apex.exe!0x02394860 ConVar weaponx_smartammo_data_optimization
r5apex.exe!0x02824d70 ConVar weaponx_smartammo_data_optimization
r5apex.exe!0x028274b0 ConVar window_hint_debug
r5apex.exe!0x023d4250 ConVar window_hint_fov_down
r5apex.exe!0x023d46d0 ConVar window_hint_fov_horz
r5apex.exe!0x023d1430 ConVar window_hint_fov_up
r5apex.exe!0x023d5150 ConVar window_hint_keyboard_fov_horz
r5apex.exe!0x023d2100 ConVar window_hint_lookahead_time
r5apex.exe!0x023d5a70 ConVar window_hint_max_horz_vel_change_dot
r5apex.exe!0x023d4a40 ConVar window_hint_max_vel_change_down
r5apex.exe!0x023d27a0 ConVar window_hint_max_vel_change_up
r5apex.exe!0x023d2060 ConVar window_hint_min_horz_vel
r5apex.exe!0x023d5b40 ConVar window_hint_permissive_max_horz_vel_change_dot
r5apex.exe!0x023d3290 ConVar window_hint_permissive_max_vel_change_down
r5apex.exe!0x023d3490 ConVar window_hint_permissive_max_vel_change_up
r5apex.exe!0x028131e0 ConVar xc_crouch_debounce
r5apex.exe!0x01f93680 ConVar z_ragdoll_impact_strength
r5apex.exe!0x0238aa90 ConVar zipline_fade_dist
r5apex.exe!0x0232d060 ConVar zipline_subdiv_lod_dist_base
r5apex.exe!0x0233a4c0 ConVar zipline_subdiv_slices
r5apex.exe!0x02337d70 ConVar zipline_subdiv_slices_lod
r5apex.exe!0x02331bf0 ConVar zipline_subdiv_stacks
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
<summary><code>BuildAINFile</code></summary>

Build and Save the AI Node Graph (restarts map)

flags: `0x6`  
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
<summary><code>Test_InitRandomEntitySpawner</code></summary>



flags: `0x4004`  
</details>
<details>
<summary><code>Test_RandomizeInPVS</code></summary>



flags: `0x4004`  
</details>
<details>
<summary><code>Test_RemoveAllRandomEntities</code></summary>



flags: `0x4004`  
</details>
<details>
<summary><code>Test_SpawnRandomEntities</code></summary>



flags: `0x4004`  
</details>
<details>
<summary><code>_setClassVarServer</code></summary>

Set a class var on the server

flags: `0x40004006`  
</details>
<details>
<summary><code>adminmsg</code></summary>

Send text to the current community (if you are an admin)

flags: `0x2`  
</details>
<details>
<summary><code>ai_debug_node_connect</code></summary>

Debug the attempted connection between two nodes

flags: `0x6`  
</details>
<details>
<summary><code>ai_dump_hints</code></summary>



flags: `0x6`  
</details>
<details>
<summary><code>ai_set_move_height_epsilon</code></summary>

Set how high AI bumps up ground walkers when checking steps

flags: `0x6`  
</details>
<details>
<summary><code>air_density</code></summary>

Changes the density of air for drag computations.

flags: `0x4004`  
</details>
<details>
<summary><code>aisettings_reparse</code></summary>

Reloads the AI settings files

flags: `0x6`  
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
<summary><code>bot_loadout_server</code></summary>

Override bot loadout.

flags: `0x4014`  
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
<summary><code>cast_hull</code></summary>

Tests hull collision detection

flags: `0x4004`  
</details>
<details>
<summary><code>cast_ray</code></summary>

Tests collision detection

flags: `0x4004`  
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
<summary><code>clear_debug_overlays</code></summary>

clears debug overlays

flags: `0x6`  
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
<summary><code>csm_server_status</code></summary>

Usage:
 csm_server_status


flags: `0x2`  
</details>
<details>
<summary><code>csm_status</code></summary>

Usage:
   csm_status


flags: `0x2`  
</details>
<details>
<summary><code>damagedefs_reparse</code></summary>

Reloads the damage defs

flags: `0x6`  
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
<summary><code>drawline</code></summary>

Draws line between two 3D Points.
	Green if no collision
	Red is collides with something
	Arguments: x1 y1 z1 x2 y2 z2

flags: `0x4004`  
</details>
<details>
<summary><code>dumpClientStringTable</code></summary>

Dump the contents of the client's game string table to the console.

flags: `0x4000`  
</details>
<details>
<summary><code>dumpServerStringTable</code></summary>

Dump the contents of the server's game string table to the console.

flags: `0x4004`  
</details>
<details>
<summary><code>dump_entity_sizes</code></summary>

Print sizeof(entclass)

flags: `0x6`  
</details>
<details>
<summary><code>dump_generic_key_values</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>dumpentityfactories</code></summary>

Lists all entity factory names.

flags: `0x6`  
</details>
<details>
<summary><code>dumpeventqueue</code></summary>

Dump the contents of the Entity I/O event queue to the console.

flags: `0x2`  
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
<summary><code>ent_absbox</code></summary>

Displays the total bounding box for the given entity(s) in green.  Some entites will also display entity specific overlays.
	Arguments:   	{entity_name} / {class_name} / no argument picks what player is looking at 

flags: `0x4004`  
</details>
<details>
<summary><code>ent_animdump</code></summary>

Dumps the animation each frame the given entity(ies) in red.
	Arguments:   	{entity_name} / {class_name} / no argument picks what player is looking at 

flags: `0x4004`  
</details>
<details>
<summary><code>ent_attachments</code></summary>

Displays the attachment points on an entity.
	Arguments:   	{entity_name} / {class_name} / no argument picks what player is looking at 

flags: `0x4004`  
</details>
<details>
<summary><code>ent_bbox</code></summary>

Displays the movement bounding box for the given entity(ies) in orange.  Some entites will also display entity specific overlays.
	Arguments:   	{entity_name} / {class_name} / no argument picks what player is looking at 

flags: `0x4004`  
</details>
<details>
<summary><code>ent_cancelpendingentfires</code></summary>

Cancels all ent_fire created outputs that are currently waiting for their delay to expire.

flags: `0x2`  
</details>
<details>
<summary><code>ent_create</code></summary>

Creates an entity of the given type where the player is looking.

flags: `0x4004`  
</details>
<details>
<summary><code>ent_dump</code></summary>

Usage:
   ent_dump <entity name>


flags: `0x4004`  
</details>
<details>
<summary><code>ent_entitylinks</code></summary>

Displays all incoming and outgoing links to the given entity.
	Arguments:   	{entity_name} / {class_name} / no argument picks what player is looking at 

flags: `0x4004`  
</details>
<details>
<summary><code>ent_fire</code></summary>

Usage:
   ent_fire <target> [action] [value] [delay]


flags: `0x4004`  
</details>
<details>
<summary><code>ent_info</code></summary>

Usage:
   ent_info <class name>


flags: `0x4004`  
</details>
<details>
<summary><code>ent_messages</code></summary>

Toggles input/output message display for the selected entity(ies).  The name of the entity will be displayed as well as any messages that it sends or receives.
	Arguments:   	{entity_name} / {class_name} / no argument picks what player is looking at

flags: `0x4004`  
</details>
<details>
<summary><code>ent_name</code></summary>



flags: `0x4004`  
</details>
<details>
<summary><code>ent_orient</code></summary>

Orient the specified entity to match the player's angles. By default, only orients target entity's YAW. Use the 'allangles' option to orient on all axis.
	Format: ent_orient <entity name> <optional: allangles>

flags: `0x4004`  
</details>
<details>
<summary><code>ent_pause</code></summary>

Toggles pausing of input/output message processing for entities.  When turned on processing of all message will stop.  Any messages displayed with 'ent_messages' will stop fading and be displayed indefinitely. To step through the messages one by one use 'ent_step'.

flags: `0x4004`  
</details>
<details>
<summary><code>ent_pivot</code></summary>

Displays the pivot for the given entity(ies).
	(y=up=green, z=forward=blue, x=left=red). 
	Arguments:   	{entity_name} / {class_name} / no argument picks what player is looking at 

flags: `0x4004`  
</details>
<details>
<summary><code>ent_remove</code></summary>

Removes the given entity(s)
	Arguments:   	{entity_name} / {class_name} / no argument picks what player is looking at 

flags: `0x4004`  
</details>
<details>
<summary><code>ent_remove_all</code></summary>

Removes all entities of the specified type
	Arguments:   	{entity_name} / {class_name} 

flags: `0x4004`  
</details>
<details>
<summary><code>ent_script_dump</code></summary>

Dumps the names and values of this entity's script scope to the console
	Arguments:   	{entity_name} / {class_name} / no argument picks what player is looking at 

flags: `0x4004`  
</details>
<details>
<summary><code>ent_setname</code></summary>

Sets the targetname of the given entity(s)
	Arguments:   	{new entity name} {entity_name} / {class_name} / no argument picks what player is looking at 

flags: `0x4004`  
</details>
<details>
<summary><code>ent_step</code></summary>

When 'ent_pause' is set this will step through one waiting input / output message at a time.

flags: `0x4004`  
</details>
<details>
<summary><code>ent_teleport</code></summary>

Teleport the specified entity to where the player is looking.
	Format: ent_teleport <entity name>

flags: `0x4004`  
</details>
<details>
<summary><code>ent_text</code></summary>

Displays text debugging information about the given entity(ies) on top of the entity (See Overlay Text)
	Arguments:   	{entity_name} / {class_name} / no argument picks what player is looking at 

flags: `0x4004`  
</details>
<details>
<summary><code>ent_text_radius</code></summary>

Displays text debugging information about the given entity(ies) on top of the entity (See Overlay Text)
	Arguments:   	{entity_name} / {class_name} / no argument picks what player is looking at 

flags: `0x4004`  
</details>
<details>
<summary><code>ent_throw</code></summary>

Throws an entity of the given type where the player is looking.

flags: `0x4004`  
</details>
<details>
<summary><code>ent_viewoffset</code></summary>

Displays the eye position for the given entity(ies) in red.
	Arguments:   	{entity_name} / {class_name} / no argument picks what player is looking at 

flags: `0x4004`  
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
<summary><code>find_ent</code></summary>

Find and list all entities with classnames or targetnames that contain the specified substring.
Format: find_ent <substring>


flags: `0x4004`  
</details>
<details>
<summary><code>find_ent_index</code></summary>

Display data for entity matching specified index.
Format: find_ent_index <index>


flags: `0x4004`  
</details>
<details>
<summary><code>firetarget</code></summary>



flags: `0x4004`  
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
<summary><code>give_server</code></summary>

Give weapon to player.

flags: `0x4014`  
</details>
<details>
<summary><code>givecurrentammo</code></summary>

Give a supply of ammo for current weapon..


flags: `0x4004`  
</details>
<details>
<summary><code>groundlist</code></summary>

Display ground entity list <index>

flags: `0x6`  
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

flags: `0x2`  
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
<summary><code>mat_crosshair_edit_all</code></summary>

open the material under the crosshair (using an engine ray trace) in the editor defined by mat_crosshair_edit_editor

flags: `0x4004`  
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
<summary><code>melee_lunge_ent</code></summary>

Lunge player towards the entity with the given entity index

flags: `0x4004`  
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
<summary><code>particle_test_start</code></summary>

Dispatches the test particle system with the parameters specified in particle_test_file,
 particle_test_attach_mode and particle_test_attach_param on the entity the player is looking at.
	Arguments:   	{entity_name} / {class_name} / no argument picks what player is looking at 

flags: `0x4000`  
</details>
<details>
<summary><code>particle_test_stop</code></summary>

Stops all particle systems on the selected entities.
	Arguments:   	{entity_name} / {class_name} / no argument picks what player is looking at 

flags: `0x4000`  
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
<summary><code>physics_budget</code></summary>

Times the cost of each active object

flags: `0x6`  
</details>
<details>
<summary><code>physics_debug_entity</code></summary>

Dumps debug info for an entity

flags: `0x6`  
</details>
<details>
<summary><code>physics_highlight_active</code></summary>

Turns on the absbox for all active physics objects

flags: `0x6`  
</details>
<details>
<summary><code>physics_report_active</code></summary>

Lists all active physics objects

flags: `0x6`  
</details>
<details>
<summary><code>physics_select</code></summary>

Dumps debug info for an entity

flags: `0x6`  
</details>
<details>
<summary><code>picker</code></summary>

Toggles 'picker' mode.  When picker is on, the bounding box, pivot and debugging text is displayed for whatever entity the player is looking at.
	Arguments:	full - enables all debug information

flags: `0x4004`  
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
<summary><code>prop_debug</code></summary>

Toggle prop debug mode. If on, props will show colorcoded bounding boxes. Red means ignore all damage. White means respond physically to damage but never break. Green maps health in the range of 100 down to 1.

flags: `0x4004`  
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
<summary><code>r_volumetric_lighting_color</code></summary>



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
<summary><code>reload_script_callbacks_server</code></summary>

Reloads script callback function pointers for server.

flags: `0x4014`  
</details>
<details>
<summary><code>render_blanks</code></summary>

render N blank frames

flags: `0x2`  
</details>
<details>
<summary><code>report_entities</code></summary>

Lists all entities

flags: `0x6`  
</details>
<details>
<summary><code>report_simthinklist</code></summary>

Lists all simulating/thinking entities

flags: `0x6`  
</details>
<details>
<summary><code>report_touchlinks</code></summary>

Lists all touchlinks

flags: `0x6`  
</details>
<details>
<summary><code>reset_cam_ideal_angles</code></summary>

Resets camera ideal angles to its default

flags: `0x2`  
</details>
<details>
<summary><code>resetidletimer</code></summary>

Resets the idle timer.

flags: `0x6`  
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
<summary><code>script_printdiag</code></summary>

Print the printtodiag() buffer to the console

flags: `0x2`  
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
<summary><code>shake</code></summary>

Shake the screen.

flags: `0x4004`  
</details>
<details>
<summary><code>shake_ropes</code></summary>

Shakes ropes around the player.

flags: `0x4004`  
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
<summary><code>showtriggers_toggle</code></summary>

Toggle show triggers

flags: `0x4004`  
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
<summary><code>skybox_swap</code></summary>

Swap through the skyboxes in our queue

flags: `0x4004`  
</details>
<details>
<summary><code>snapshot_memory_report</code></summary>

Prints information about memory usage of snapshot data structures

flags: `0x20006`  
</details>
<details>
<summary><code>soundscape_dumpclient</code></summary>

Dumps the client's soundscape data.


flags: `0x4000`  
</details>
<details>
<summary><code>soundscape_flush</code></summary>

Flushes the server & client side soundscapes

flags: `0x6`  
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

Enable screen-space subsurface scattering.

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
<summary><code>stopserver</code></summary>

Stop the server  ( DEDI only ).

flags: `0x4004`  
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
<summary><code>surfaceprop</code></summary>

Reports the surface properties at the cursor

flags: `0x4004`  
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
<summary><code>sv_soundscape_printdebuginfo</code></summary>

print soundscapes

flags: `0x4004`  
</details>
<details>
<summary><code>sv_test_rotated_box</code></summary>



flags: `0x6`  
</details>
<details>
<summary><code>sv_trace_start_solid</code></summary>

Trace with given parameters and return start solid result

flags: `0x6`  
</details>
<details>
<summary><code>sv_writeSendTableStreamFile</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>swap_to_weapon</code></summary>

uses a weapon in your inventory by name

flags: `0x4004`  
</details>
<details>
<summary><code>switchPlayerClassActivityMod</code></summary>

Switches the player's class activity modifier to the given string

flags: `0x40004006`  
</details>
<details>
<summary><code>switchclass</code></summary>

Change player class: soldier, titan

flags: `0x40004006`  
</details>
<details>
<summary><code>takecurrentammo</code></summary>

Remove player's extra ammo for the current weapon.


flags: `0x4004`  
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
<summary><code>test_entity_blocker</code></summary>

Test command that drops an entity blocker out in front of the player.

flags: `0x4004`  
</details>
<details>
<summary><code>test_freezeframe</code></summary>

Test the freeze frame code.

flags: `0x4000`  
</details>
<details>
<summary><code>test_setteam</code></summary>

Sets target to the player's team.

flags: `0x4004`  
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
<summary><code>trace_capsule</code></summary>

Trace a capsule through the world and debug draw the result

flags: `0x6`  
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
r5apex.exe!0x02325e30 ConCommand +ability
r5apex.exe!0x0232f730 ConCommand +ability_held
r5apex.exe!0x02334140 ConCommand +attack
r5apex.exe!0x02338170 ConCommand +backward
r5apex.exe!0x02330c20 ConCommand +break
r5apex.exe!0x02334300 ConCommand +camdistance
r5apex.exe!0x0232f7b0 ConCommand +camin
r5apex.exe!0x0232c5f0 ConCommand +cammousemove
r5apex.exe!0x0232cad0 ConCommand +camout
r5apex.exe!0x023380f0 ConCommand +campitchdown
r5apex.exe!0x02325b50 ConCommand +campitchup
r5apex.exe!0x02330ca0 ConCommand +camyawleft
r5apex.exe!0x0232e880 ConCommand +camyawright
r5apex.exe!0x02332ce0 ConCommand +commandermousemove
r5apex.exe!0x01f74c50 ConCommand +csm_rot_x_neg
r5apex.exe!0x01f8b930 ConCommand +csm_rot_x_plus
r5apex.exe!0x01f85ae0 ConCommand +csm_rot_y_neg
r5apex.exe!0x01f8b1f0 ConCommand +csm_rot_y_plus
r5apex.exe!0x0233b800 ConCommand +displayFullscreenMap
r5apex.exe!0x0232dca0 ConCommand +dodge
r5apex.exe!0x02338cf0 ConCommand +duck
r5apex.exe!0x02326070 ConCommand +forward
r5apex.exe!0x02335600 ConCommand +graph
r5apex.exe!0x02339960 ConCommand +jump
r5apex.exe!0x02333360 ConCommand +klook
r5apex.exe!0x02337870 ConCommand +left
r5apex.exe!0x02335780 ConCommand +lookdown
r5apex.exe!0x023362a0 ConCommand +lookup
r5apex.exe!0x018554b0 ConCommand +mat_texture_list
r5apex.exe!0x02339240 ConCommand +melee
r5apex.exe!0x02335580 ConCommand +movedown
r5apex.exe!0x0232f4f0 ConCommand +moveleft
r5apex.exe!0x02326730 ConCommand +moveright
r5apex.exe!0x0232dec0 ConCommand +moveup
r5apex.exe!0x0233a180 ConCommand +offhand0
r5apex.exe!0x0232a7d0 ConCommand +offhand1
r5apex.exe!0x02338790 ConCommand +offhand2
r5apex.exe!0x02334040 ConCommand +offhand3
r5apex.exe!0x023381f0 ConCommand +offhand4
r5apex.exe!0x0233b880 ConCommand +pause_menu
r5apex.exe!0x02331b70 ConCommand +ping
r5apex.exe!0x02381f20 ConCommand +posedebug
r5apex.exe!0x0175cf40 ConCommand +pushtotalk
r5apex.exe!0x0232c950 ConCommand +reload
r5apex.exe!0x0232be50 ConCommand +right
r5apex.exe!0x0233aa00 ConCommand +score
r5apex.exe!0x02336a80 ConCommand +scriptCommand1
r5apex.exe!0x023392c0 ConCommand +scriptCommand2
r5apex.exe!0x0232bd50 ConCommand +scriptCommand3
r5apex.exe!0x0232e660 ConCommand +scriptCommand4
r5apex.exe!0x02335fe0 ConCommand +scriptCommand5
r5apex.exe!0x02336320 ConCommand +scriptCommand6
r5apex.exe!0x02339ba0 ConCommand +scriptCommand7
r5apex.exe!0x0232c3b0 ConCommand +scriptCommand8
r5apex.exe!0x02335da0 ConCommand +scriptCommand9
r5apex.exe!0x023269d0 ConCommand +showscores
r5apex.exe!0x02325710 ConCommand +speed
r5apex.exe!0x0233b780 ConCommand +strafe
r5apex.exe!0x0232c790 ConCommand +toggle_duck
r5apex.exe!0x02326830 ConCommand +toggle_zoom
r5apex.exe!0x02337f30 ConCommand +use
r5apex.exe!0x02336100 ConCommand +useAndReload
r5apex.exe!0x0232ec50 ConCommand +use_alt
r5apex.exe!0x02338d70 ConCommand +use_long
r5apex.exe!0x0232cd10 ConCommand +variableScopeToggle
r5apex.exe!0x0175ae90 ConCommand +vgui_drawtree
r5apex.exe!0x0174e430 ConCommand +voicerecord
r5apex.exe!0x0233b0f0 ConCommand +walk
r5apex.exe!0x0233a740 ConCommand +weaponCycle
r5apex.exe!0x02336440 ConCommand +weapon_discard
r5apex.exe!0x0232c9d0 ConCommand +zoom
r5apex.exe!0x02332e00 ConCommand -ability
r5apex.exe!0x0232d540 ConCommand -ability_held
r5apex.exe!0x0232fc30 ConCommand -attack
r5apex.exe!0x02325990 ConCommand -backward
r5apex.exe!0x02331c90 ConCommand -break
r5apex.exe!0x0233af60 ConCommand -camdistance
r5apex.exe!0x02337080 ConCommand -camin
r5apex.exe!0x0232c430 ConCommand -cammousemove
r5apex.exe!0x02331840 ConCommand -camout
r5apex.exe!0x02335ac0 ConCommand -campitchdown
r5apex.exe!0x02330dc0 ConCommand -campitchup
r5apex.exe!0x0232f610 ConCommand -camyawleft
r5apex.exe!0x0232de40 ConCommand -camyawright
r5apex.exe!0x02338e20 ConCommand -commandermousemove
r5apex.exe!0x01f8d110 ConCommand -csm_rot_x_neg
r5apex.exe!0x01f753e0 ConCommand -csm_rot_x_plus
r5apex.exe!0x02319260 ConCommand -csm_rot_y_neg
r5apex.exe!0x01f8a6f0 ConCommand -csm_rot_y_plus
r5apex.exe!0x02325f50 ConCommand -displayFullscreenMap
r5apex.exe!0x0233a440 ConCommand -dodge
r5apex.exe!0x02335700 ConCommand -duck
r5apex.exe!0x0232e760 ConCommand -forward
r5apex.exe!0x0232e9a0 ConCommand -graph
r5apex.exe!0x0232ef10 ConCommand -jump
r5apex.exe!0x0232d620 ConCommand -klook
r5apex.exe!0x023267b0 ConCommand -left
r5apex.exe!0x0232e080 ConCommand -lookdown
r5apex.exe!0x02325d10 ConCommand -lookup
r5apex.exe!0x01855250 ConCommand -mat_texture_list
r5apex.exe!0x023326e0 ConCommand -melee
r5apex.exe!0x0232c670 ConCommand -movedown
r5apex.exe!0x02337a80 ConCommand -moveleft
r5apex.exe!0x0232ef90 ConCommand -moveright
r5apex.exe!0x02335800 ConCommand -moveup
r5apex.exe!0x02336220 ConCommand -offhand0
r5apex.exe!0x023383b0 ConCommand -offhand1
r5apex.exe!0x023372c0 ConCommand -offhand2
r5apex.exe!0x0232d6a0 ConCommand -offhand3
r5apex.exe!0x02326470 ConCommand -offhand4
r5apex.exe!0x02337b80 ConCommand -pause_menu
r5apex.exe!0x023268b0 ConCommand -ping
r5apex.exe!0x02381fa0 ConCommand -posedebug
r5apex.exe!0x0175c920 ConCommand -pushtotalk
r5apex.exe!0x02326590 ConCommand -reload
r5apex.exe!0x02335680 ConCommand -right
r5apex.exe!0x02338570 ConCommand -score
r5apex.exe!0x023266b0 ConCommand -scriptCommand1
r5apex.exe!0x02335be0 ConCommand -scriptCommand2
r5apex.exe!0x0233a100 ConCommand -scriptCommand3
r5apex.exe!0x02339a80 ConCommand -scriptCommand4
r5apex.exe!0x02325790 ConCommand -scriptCommand5
r5apex.exe!0x02325570 ConCommand -scriptCommand6
r5apex.exe!0x0233a3c0 ConCommand -scriptCommand7
r5apex.exe!0x0232ba10 ConCommand -scriptCommand8
r5apex.exe!0x02335340 ConCommand -scriptCommand9
r5apex.exe!0x0232bcd0 ConCommand -showscores
r5apex.exe!0x0232ee90 ConCommand -speed
r5apex.exe!0x02338710 ConCommand -strafe
r5apex.exe!0x0232e6e0 ConCommand -toggle_duck
r5apex.exe!0x02335f60 ConCommand -toggle_zoom
r5apex.exe!0x0232ba90 ConCommand -use
r5apex.exe!0x02325690 ConCommand -useAndReload
r5apex.exe!0x02337eb0 ConCommand -use_alt
r5apex.exe!0x023309b0 ConCommand -use_long
r5apex.exe!0x023263f0 ConCommand -variableScopeToggle
r5apex.exe!0x0175a8d0 ConCommand -vgui_drawtree
r5apex.exe!0x0174a4d0 ConCommand -voicerecord
r5apex.exe!0x0233a7c0 ConCommand -walk
r5apex.exe!0x0232dd20 ConCommand -weaponCycle
r5apex.exe!0x0233b210 ConCommand -weapon_discard
r5apex.exe!0x02331aa0 ConCommand -zoom
r5apex.exe!0x01746dc0 ConCommand BindToggle
r5apex.exe!0x023f7950 ConCommand BuildAINFile
r5apex.exe!0x019eaf30 ConCommand DebugPrintUsedTextures
r5apex.exe!0x01855f80 ConCommand DumpClientDataBlockReceiver
r5apex.exe!0x0181f1b0 ConCommand MemTrackDeltaSnapshot
r5apex.exe!0x0181ec50 ConCommand MemTrackPrintStats
r5apex.exe!0x01f94230 ConCommand ReloadAimAssistSettings
r5apex.exe!0x02803d10 ConCommand Test_InitRandomEntitySpawner
r5apex.exe!0x0280fd40 ConCommand Test_RandomizeInPVS
r5apex.exe!0x0280a1a0 ConCommand Test_RemoveAllRandomEntities
r5apex.exe!0x02812a70 ConCommand Test_SpawnRandomEntities
r5apex.exe!0x023dda40 ConCommand _setClassVarServer
r5apex.exe!0x0181e6c0 ConCommand adminmsg
r5apex.exe!0x023f8b70 ConCommand ai_debug_node_connect
r5apex.exe!0x023f6310 ConCommand ai_dump_hints
r5apex.exe!0x023f7820 ConCommand ai_set_move_height_epsilon
r5apex.exe!0x024242d0 ConCommand air_density
r5apex.exe!0x0281bf30 ConCommand aisettings_reparse
r5apex.exe!0x02388c70 ConCommand aisettings_reparse_client
r5apex.exe!0x0174a290 ConCommand alias
r5apex.exe!0x023878a0 ConCommand applyVideoChangesDeferred
r5apex.exe!0x01751e20 ConCommand bind
r5apex.exe!0x0174e9f0 ConCommand bind_US_standard
r5apex.exe!0x01753c00 ConCommand bind_held
r5apex.exe!0x01753350 ConCommand bind_held_US_standard
r5apex.exe!0x0174f010 ConCommand bind_list
r5apex.exe!0x0174d970 ConCommand bind_list_abilities
r5apex.exe!0x0175d800 ConCommand bink_dump_precached_movies
r5apex.exe!0x02374130 ConCommand bot_loadout
r5apex.exe!0x023dd0b0 ConCommand bot_loadout_server
r5apex.exe!0x0181ecf0 ConCommand box
r5apex.exe!0x01747b30 ConCommand buildcubemaps
r5apex.exe!0x01750910 ConCommand cache_print
r5apex.exe!0x0174ff10 ConCommand cache_print_lru
r5apex.exe!0x0174a9b0 ConCommand cache_print_summary
r5apex.exe!0x02333b90 ConCommand cam_command
r5apex.exe!0x0232bbb0 ConCommand cancelselect
r5apex.exe!0x02476130 ConCommand cast_hull
r5apex.exe!0x02434bf0 ConCommand cast_ray
r5apex.exe!0x0232af20 ConCommand cc_emit
r5apex.exe!0x01f75960 ConCommand centerview
r5apex.exe!0x01754b20 ConCommand changelevel
r5apex.exe!0x01820100 ConCommand chat
r5apex.exe!0x018dfe50 ConCommand chatroom_adminsOnly
r5apex.exe!0x0181e590 ConCommand chatroom_away
r5apex.exe!0x018dfc70 ConCommand chatroom_freetalk
r5apex.exe!0x0181f8b0 ConCommand chatroom_present
r5apex.exe!0x0181ed90 ConCommand chatserver
r5apex.exe!0x0175d9c0 ConCommand chroma_base
r5apex.exe!0x0175d940 ConCommand chroma_layer
r5apex.exe!0x02347310 ConCommand cl_dump_particle_stats
r5apex.exe!0x0231a1a0 ConCommand cl_ent_absbox
r5apex.exe!0x01f7e3f0 ConCommand cl_ent_bbox
r5apex.exe!0x01f8d230 ConCommand cl_ent_rbox
r5apex.exe!0x01f8d420 ConCommand cl_find_ent
r5apex.exe!0x01f92c70 ConCommand cl_find_ent_index
r5apex.exe!0x01f8d3a0 ConCommand cl_flip_visibility
r5apex.exe!0x0181f250 ConCommand cl_fullupdate
r5apex.exe!0x01f94110 ConCommand cl_interpolation_report
r5apex.exe!0x0233ebe0 ConCommand cl_panelanimation
r5apex.exe!0x02340210 ConCommand cl_particles_dump_effects
r5apex.exe!0x023409c0 ConCommand cl_particles_dumplist
r5apex.exe!0x0181e4f0 ConCommand cl_precacheinfo
r5apex.exe!0x01f93de0 ConCommand cl_removedecals
r5apex.exe!0x0181efd0 ConCommand cl_showents
r5apex.exe!0x02339080 ConCommand cl_soundscape_flush
r5apex.exe!0x02348410 ConCommand cl_trace_start_solid
r5apex.exe!0x01f94b30 ConCommand cl_trace_test_hitbox_with_non_zero_start_offset
r5apex.exe!0x0231a620 ConCommand cl_updatevisibility
r5apex.exe!0x02423660 ConCommand clear_debug_overlays
r5apex.exe!0x01755310 ConCommand clear_loading_progress_detente
r5apex.exe!0x0174b910 ConCommand clear_loading_progress_sp_text
r5apex.exe!0x01747900 ConCommand cm_query_log_record
r5apex.exe!0x01749af0 ConCommand cm_query_log_replay
r5apex.exe!0x017479c0 ConCommand cmd
r5apex.exe!0x01747490 ConCommand cmd1
r5apex.exe!0x01747aa0 ConCommand cmd2
r5apex.exe!0x01747010 ConCommand cmd3
r5apex.exe!0x01747ba0 ConCommand cmd4
r5apex.exe!0x01f872a0 ConCommand collision_debug
r5apex.exe!0x0174a3b0 ConCommand colorcorrectionui
r5apex.exe!0x018da940 ConCommand community_browse
r5apex.exe!0x018dab20 ConCommand community_getPendingJoinRequest
r5apex.exe!0x018da780 ConCommand community_join
r5apex.exe!0x018dae40 ConCommand community_leave
r5apex.exe!0x018da800 ConCommand community_list
r5apex.exe!0x018dada0 ConCommand community_report
r5apex.exe!0x0181f790 ConCommand connect
r5apex.exe!0x0181e3b0 ConCommand connectWithKey
r5apex.exe!0x01820320 ConCommand connectwithtoken
r5apex.exe!0x017481a0 ConCommand convar_differences
r5apex.exe!0x01748130 ConCommand convar_findByFlags
r5apex.exe!0x01749b70 ConCommand convar_list
r5apex.exe!0x0181ef50 ConCommand createparty
r5apex.exe!0x0181fa50 ConCommand createpartyifnotinone
r5apex.exe!0x023fece0 ConCommand csm_server_status
r5apex.exe!0x0231a5a0 ConCommand csm_status
r5apex.exe!0x023db310 ConCommand damagedefs_reparse
r5apex.exe!0x0237d150 ConCommand damagedefs_reparse_client
r5apex.exe!0x0181eeb0 ConCommand debugModelPurge
r5apex.exe!0x01748290 ConCommand devshots_nextmap
r5apex.exe!0x0181f410 ConCommand devshots_screenshot
r5apex.exe!0x0174a650 ConCommand disconnect
r5apex.exe!0x01754f80 ConCommand display_elapsedtime
r5apex.exe!0x01f8a1b0 ConCommand dlight_debug
r5apex.exe!0x018dc740 ConCommand do_InvitePeople_test
r5apex.exe!0x018dc240 ConCommand do_Invite_friend_test
r5apex.exe!0x018dbde0 ConCommand do_joinPeople_test
r5apex.exe!0x018dc100 ConCommand do_origin_test_presence
r5apex.exe!0x0175a050 ConCommand downloadPlaylists
r5apex.exe!0x02477700 ConCommand drawline
r5apex.exe!0x0234af70 ConCommand dumpClientStringTable
r5apex.exe!0x0239ba20 ConCommand dumpServerStringTable
r5apex.exe!0x02803b40 ConCommand dump_entity_sizes
r5apex.exe!0x023f00c0 ConCommand dump_generic_key_values
r5apex.exe!0x028110b0 ConCommand dumpentityfactories
r5apex.exe!0x023f6760 ConCommand dumpeventqueue
r5apex.exe!0x01755cc0 ConCommand dumpstringtables
r5apex.exe!0x01747cc0 ConCommand echo
r5apex.exe!0x018df750 ConCommand editor_toggle
r5apex.exe!0x018202a0 ConCommand endmovie
r5apex.exe!0x023f3c60 ConCommand ent_absbox
r5apex.exe!0x023f69c0 ConCommand ent_animdump
r5apex.exe!0x023f2410 ConCommand ent_attachments
r5apex.exe!0x023f7ba0 ConCommand ent_bbox
r5apex.exe!0x023fd520 ConCommand ent_cancelpendingentfires
r5apex.exe!0x023f5ec0 ConCommand ent_create
r5apex.exe!0x023ef8f0 ConCommand ent_dump
r5apex.exe!0x023effb0 ConCommand ent_entitylinks
r5apex.exe!0x023f8f20 ConCommand ent_fire
r5apex.exe!0x023fc490 ConCommand ent_info
r5apex.exe!0x023fc090 ConCommand ent_messages
r5apex.exe!0x023f6610 ConCommand ent_name
r5apex.exe!0x023fc2e0 ConCommand ent_orient
r5apex.exe!0x023f1940 ConCommand ent_pause
r5apex.exe!0x023f7440 ConCommand ent_pivot
r5apex.exe!0x023f7df0 ConCommand ent_remove
r5apex.exe!0x023fc1d0 ConCommand ent_remove_all
r5apex.exe!0x023f3770 ConCommand ent_script_dump
r5apex.exe!0x023f8050 ConCommand ent_setname
r5apex.exe!0x023f4be0 ConCommand ent_step
r5apex.exe!0x023ef460 ConCommand ent_teleport
r5apex.exe!0x023fd490 ConCommand ent_text
r5apex.exe!0x023ef3f0 ConCommand ent_text_radius
r5apex.exe!0x023f9ae0 ConCommand ent_throw
r5apex.exe!0x023f3a60 ConCommand ent_viewoffset
r5apex.exe!0x0174b090 ConCommand entitlements_print
r5apex.exe!0x01856440 ConCommand entitlements_send
r5apex.exe!0x017520a0 ConCommand entitlements_set_bits
r5apex.exe!0x01747380 ConCommand envmap
r5apex.exe!0x0174b990 ConCommand escape
r5apex.exe!0x01749f50 ConCommand exec
r5apex.exe!0x01755560 ConCommand execPlayerConfig
r5apex.exe!0x01748220 ConCommand execifexists
r5apex.exe!0x0174bfd0 ConCommand exit
r5apex.exe!0x01f8ba50 ConCommand eyeInfo
r5apex.exe!0x023f8420 ConCommand find_ent
r5apex.exe!0x023f7cb0 ConCommand find_ent_index
r5apex.exe!0x023f5650 ConCommand firetarget
r5apex.exe!0x0232ea20 ConCommand firstperson
r5apex.exe!0x0174adb0 ConCommand flush
r5apex.exe!0x0174f290 ConCommand flush_locked
r5apex.exe!0x023385f0 ConCommand force_centerview
r5apex.exe!0x018e4dd0 ConCommand fps_stats_dump
r5apex.exe!0x018e4e40 ConCommand fps_stats_reset
r5apex.exe!0x018e4bf0 ConCommand fps_stats_start
r5apex.exe!0x018e5200 ConCommand fps_stats_stop
r5apex.exe!0x018deae0 ConCommand friends_update
r5apex.exe!0x018e0d50 ConCommand fs_clear_open_duplicate_times
r5apex.exe!0x018e0fd0 ConCommand fs_dump_open_duplicate_times
r5apex.exe!0x018e17d0 ConCommand fs_fios_cancel_prefetches
r5apex.exe!0x018e0cb0 ConCommand fs_fios_flush_cache
r5apex.exe!0x018e11b0 ConCommand fs_fios_prefetch_file
r5apex.exe!0x018e1560 ConCommand fs_fios_prefetch_file_in_pack
r5apex.exe!0x018e14c0 ConCommand fs_fios_print_prefetches
r5apex.exe!0x01748570 ConCommand fs_printopenfiles
r5apex.exe!0x01748960 ConCommand fs_warning_level
r5apex.exe!0x0232d380 ConCommand fx_impact_reparse
r5apex.exe!0x01757eb0 ConCommand gameui_activate
r5apex.exe!0x0175a190 ConCommand gameui_allowescape
r5apex.exe!0x0175bc20 ConCommand gameui_allowescapetoshow
r5apex.exe!0x01757c30 ConCommand gameui_hide
r5apex.exe!0x01757760 ConCommand gameui_preventescape
r5apex.exe!0x01759010 ConCommand gameui_preventescapetoshow
r5apex.exe!0x018e00a0 ConCommand getNewAuthToken
r5apex.exe!0x01f848a0 ConCommand getfov
r5apex.exe!0x0174d900 ConCommand gethttpdatacenterlist
r5apex.exe!0x01f758e0 ConCommand getpos
r5apex.exe!0x01f92de0 ConCommand getpos_bind
r5apex.exe!0x023189d0 ConCommand getposvec
r5apex.exe!0x0237e660 ConCommand give
r5apex.exe!0x023e2670 ConCommand give_server
r5apex.exe!0x028127b0 ConCommand givecurrentammo
r5apex.exe!0x02423950 ConCommand groundlist
r5apex.exe!0x017488f0 ConCommand help
r5apex.exe!0x02396ae0 ConCommand hidepanel
r5apex.exe!0x0233d480 ConCommand hidevideos
r5apex.exe!0x019eb600 ConCommand highlight_log
r5apex.exe!0x01752dc0 ConCommand host_runofftime
r5apex.exe!0x02330190 ConCommand hud_subtitles
r5apex.exe!0x018e0170 ConCommand huffman_readProps
r5apex.exe!0x023353c0 ConCommand impulse
r5apex.exe!0x018dabc0 ConCommand inboxmessage_report
r5apex.exe!0x01755b00 ConCommand incrementvar
r5apex.exe!0x02340050 ConCommand ingamemenu_activate
r5apex.exe!0x0174b7f0 ConCommand initMatchmaking
r5apex.exe!0x0232e100 ConCommand invnext
r5apex.exe!0x01f93600 ConCommand is_considered_sony_multiplayer
r5apex.exe!0x0181ee30 ConCommand joinopeninvite
r5apex.exe!0x02335a40 ConCommand joystick_initialize
r5apex.exe!0x0181f550 ConCommand jpeg
r5apex.exe!0x01755420 ConCommand key_listboundkeys
r5apex.exe!0x018e2710 ConCommand key_updatelayout
r5apex.exe!0x01759d30 ConCommand launchplaylist
r5apex.exe!0x0181ff40 ConCommand leaveopeninvite
r5apex.exe!0x02348490 ConCommand listClientFXScriptHandles
r5apex.exe!0x01750e90 ConCommand listmodels
r5apex.exe!0x017587f0 ConCommand loadPlaylists
r5apex.exe!0x0174ec20 ConCommand map
r5apex.exe!0x0174c940 ConCommand map_background
r5apex.exe!0x0174f150 ConCommand maps
r5apex.exe!0x0174dfc0 ConCommand mat_antialias_mode
r5apex.exe!0x01756da0 ConCommand mat_configcurrent
r5apex.exe!0x01748720 ConCommand mat_crosshair
r5apex.exe!0x01748450 ConCommand mat_crosshair_edit
r5apex.exe!0x02477280 ConCommand mat_crosshair_edit_all
r5apex.exe!0x01747260 ConCommand mat_crosshair_explorer
r5apex.exe!0x01748870 ConCommand mat_crosshair_printmaterial
r5apex.exe!0x01749bf0 ConCommand mat_crosshair_reloadmaterial
r5apex.exe!0x0174fab0 ConCommand mat_gamma
r5apex.exe!0x018e87c0 ConCommand mat_hdr_enabled
r5apex.exe!0x01f66e80 ConCommand mat_printLiveTex
r5apex.exe!0x0174edd0 ConCommand mat_savechanges
r5apex.exe!0x01752e60 ConCommand mat_setvideomode
r5apex.exe!0x01f670c0 ConCommand mat_shadercount
r5apex.exe!0x01f67050 ConCommand mat_spewvertexandpixelshaders
r5apex.exe!0x01756bc0 ConCommand mat_vsync
r5apex.exe!0x01752ef0 ConCommand match_abortAllSearches
r5apex.exe!0x0174d7f0 ConCommand match_showAllSearches
r5apex.exe!0x017561c0 ConCommand matchmake
r5apex.exe!0x01756560 ConCommand matchmake_cancel
r5apex.exe!0x017535e0 ConCommand matchmake_cleanupforparty
r5apex.exe!0x01857ba0 ConCommand maxplayers
r5apex.exe!0x0239a5e0 ConCommand melee_lunge_ent
r5apex.exe!0x01755600 ConCommand mem_compact
r5apex.exe!0x0174ee50 ConCommand mem_dump
r5apex.exe!0x0174e4a0 ConCommand mem_dump_vm
r5apex.exe!0x0174c180 ConCommand mem_eat
r5apex.exe!0x01754640 ConCommand mem_incremental_compact
r5apex.exe!0x01755200 ConCommand mem_leak_vm
r5apex.exe!0x0174f3d0 ConCommand mem_test
r5apex.exe!0x018e9960 ConCommand mem_textures
r5apex.exe!0x017567c0 ConCommand mem_verify
r5apex.exe!0x018e9e30 ConCommand mem_vram
r5apex.exe!0x01751710 ConCommand memory
r5apex.exe!0x01754240 ConCommand migrateme
r5apex.exe!0x023870f0 ConCommand miles_dump
r5apex.exe!0x02387250 ConCommand miles_event_info
r5apex.exe!0x02387c90 ConCommand miles_pauseui_byname
r5apex.exe!0x02388870 ConCommand miles_play
r5apex.exe!0x02388ab0 ConCommand miles_reboot
r5apex.exe!0x02385b80 ConCommand miles_record
r5apex.exe!0x02386990 ConCommand miles_record_that
r5apex.exe!0x023865c0 ConCommand miles_stop_all
r5apex.exe!0x02386060 ConCommand miles_unpauseui_byname
r5apex.exe!0x02386bf0 ConCommand miles_write_passive_dumpfile
r5apex.exe!0x0181e890 ConCommand mmdevinit
r5apex.exe!0x0174a550 ConCommand multvar
r5apex.exe!0x018e0000 ConCommand muteroom
r5apex.exe!0x0174ae30 ConCommand net_channels
r5apex.exe!0x0181fad0 ConCommand net_dumpIncomingStats
r5apex.exe!0x0181f670 ConCommand net_dumpOutgoingStats
r5apex.exe!0x0181f930 ConCommand net_dumpStats
r5apex.exe!0x017537a0 ConCommand net_start
r5apex.exe!0x017507d0 ConCommand net_status
r5apex.exe!0x01750c30 ConCommand net_writeStatsFile
r5apex.exe!0x0181fd10 ConCommand openinvite
r5apex.exe!0x0181f2d0 ConCommand openinvitecomplete
r5apex.exe!0x0181f110 ConCommand openinvitelaunch
r5apex.exe!0x018dc1a0 ConCommand origin_friendlist_dump
r5apex.exe!0x01f83f90 ConCommand particle_create
r5apex.exe!0x0231a6a0 ConCommand particle_create_ss
r5apex.exe!0x0233fe90 ConCommand particle_dump
r5apex.exe!0x01f75f10 ConCommand particle_kill
r5apex.exe!0x023456d0 ConCommand particle_list
r5apex.exe!0x01f83bf0 ConCommand particle_recreate
r5apex.exe!0x0233dfb0 ConCommand particle_remove_all
r5apex.exe!0x01f813d0 ConCommand particle_scrub_bake
r5apex.exe!0x02324ef0 ConCommand particle_scrub_play
r5apex.exe!0x023186d0 ConCommand particle_scrub_stop
r5apex.exe!0x023b7040 ConCommand particle_test_start
r5apex.exe!0x02398cc0 ConCommand particle_test_stop
r5apex.exe!0x018dd320 ConCommand party_leave
r5apex.exe!0x018dcd80 ConCommand party_serverChange
r5apex.exe!0x01749fd0 ConCommand path
r5apex.exe!0x0174be20 ConCommand pause
r5apex.exe!0x02342540 ConCommand pausevideos
r5apex.exe!0x02342720 ConCommand phys_objectDump
r5apex.exe!0x01f7de70 ConCommand phys_throw_client
r5apex.exe!0x02477a00 ConCommand physics_budget
r5apex.exe!0x02400d10 ConCommand physics_debug_entity
r5apex.exe!0x02422a70 ConCommand physics_highlight_active
r5apex.exe!0x02476500 ConCommand physics_report_active
r5apex.exe!0x024763e0 ConCommand physics_select
r5apex.exe!0x023f8c80 ConCommand picker
r5apex.exe!0x0174cef0 ConCommand ping
r5apex.exe!0x023420a0 ConCommand ping_specific_type
r5apex.exe!0x01756120 ConCommand pingdatacenters
r5apex.exe!0x01f75a00 ConCommand pixelvis_debug
r5apex.exe!0x02373f90 ConCommand playerSettings_reparse
r5apex.exe!0x0232ca50 ConCommand playsoundscape
r5apex.exe!0x02343620 ConCommand playvideo
r5apex.exe!0x02340490 ConCommand playvideo_end_level_transition
r5apex.exe!0x02345f10 ConCommand playvideo_exitcommand
r5apex.exe!0x02346350 ConCommand playvideo_exitcommand_nointerrupt
r5apex.exe!0x02346d10 ConCommand playvideo_nointerrupt
r5apex.exe!0x02343040 ConCommand playvideo_scaled
r5apex.exe!0x01747a30 ConCommand print_colorcorrection
r5apex.exe!0x0175a950 ConCommand progress_enable
r5apex.exe!0x02813ab0 ConCommand prop_debug
r5apex.exe!0x01750d50 ConCommand quit
r5apex.exe!0x0233eac0 ConCommand r_cheapwaterend
r5apex.exe!0x023454a0 ConCommand r_cheapwaterstart
r5apex.exe!0x0174e3c0 ConCommand r_cleardecals
r5apex.exe!0x019ed190 ConCommand r_dxgi_max_frame_latency
r5apex.exe!0x01759510 ConCommand r_printdecalinfo
r5apex.exe!0x019ecaa0 ConCommand r_volumetric_lighting_color
r5apex.exe!0x018da350 ConCommand readMsgs
r5apex.exe!0x01756980 ConCommand recheck
r5apex.exe!0x0174ed60 ConCommand recompute_speed
r5apex.exe!0x01753210 ConCommand reload
r5apex.exe!0x01856690 ConCommand reload_localization
r5apex.exe!0x023742d0 ConCommand reload_script_callbacks
r5apex.exe!0x023e3140 ConCommand reload_script_callbacks_server
r5apex.exe!0x018defe0 ConCommand render_blanks
r5apex.exe!0x02434650 ConCommand report_entities
r5apex.exe!0x02434f00 ConCommand report_simthinklist
r5apex.exe!0x02477830 ConCommand report_touchlinks
r5apex.exe!0x02330ad0 ConCommand reset_cam_ideal_angles
r5apex.exe!0x02476470 ConCommand resetidletimer
r5apex.exe!0x01753ac0 ConCommand restart
r5apex.exe!0x01750cd0 ConCommand restart_checkpoint
r5apex.exe!0x023879f0 ConCommand rumble_print
r5apex.exe!0x0174aa30 ConCommand savePlayerConfig
r5apex.exe!0x023451b0 ConCommand scoreboard_down
r5apex.exe!0x02341330 ConCommand scoreboard_focus
r5apex.exe!0x02345fb0 ConCommand scoreboard_mute
r5apex.exe!0x02347bf0 ConCommand scoreboard_profile
r5apex.exe!0x02345050 ConCommand scoreboard_toggle_focus
r5apex.exe!0x02343ec0 ConCommand scoreboard_up
r5apex.exe!0x0181e800 ConCommand screenshot
r5apex.exe!0x02811d40 ConCommand script_printdiag
r5apex.exe!0x01750870 ConCommand server_single_frame
r5apex.exe!0x01759290 ConCommand serverinfo
r5apex.exe!0x0237a6c0 ConCommand set
r5apex.exe!0x0174e170 ConCommand set_loading_progress_background
r5apex.exe!0x01756d00 ConCommand set_loading_progress_detente
r5apex.exe!0x0174aab0 ConCommand set_loading_progress_fadeout_enabled
r5apex.exe!0x0174a890 ConCommand set_loading_progress_sp_text
r5apex.exe!0x0181fbf0 ConCommand setinfo
r5apex.exe!0x017492f0 ConCommand settype
r5apex.exe!0x024241a0 ConCommand shake
r5apex.exe!0x028061a0 ConCommand shake_ropes
r5apex.exe!0x02346190 ConCommand shake_stop
r5apex.exe!0x02346610 ConCommand shake_testpunch
r5apex.exe!0x01756f20 ConCommand show_loading_progress
r5apex.exe!0x02396a60 ConCommand showpanel
r5apex.exe!0x0280f9c0 ConCommand showtriggers_toggle
r5apex.exe!0x02345520 ConCommand showvideos
r5apex.exe!0x0181eb10 ConCommand silentconnect
r5apex.exe!0x0175a5d0 ConCommand skill_writeTrainingData
r5apex.exe!0x02812a00 ConCommand skybox_swap
r5apex.exe!0x02423ab0 ConCommand snapshot_memory_report
r5apex.exe!0x01f92a10 ConCommand soundscape_dumpclient
r5apex.exe!0x027f8580 ConCommand soundscape_flush
r5apex.exe!0x0233e0f0 ConCommand spawn_as_pilot
r5apex.exe!0x02345750 ConCommand spawn_as_titan
r5apex.exe!0x01750410 ConCommand ss_map
r5apex.exe!0x0233c840 ConCommand ss_reloadletterbox
r5apex.exe!0x02340170 ConCommand sssss_enable
r5apex.exe!0x017591f0 ConCommand star_memory
r5apex.exe!0x01820180 ConCommand startmovie
r5apex.exe!0x01755e00 ConCommand status
r5apex.exe!0x018de940 ConCommand steamlink
r5apex.exe!0x018dea60 ConCommand steamunlink
r5apex.exe!0x023416d0 ConCommand stop_transition_videos_fadeout
r5apex.exe!0x024345c0 ConCommand stopserver
r5apex.exe!0x02338a90 ConCommand stopsoundscape
r5apex.exe!0x0233fac0 ConCommand stopvideos
r5apex.exe!0x0233c010 ConCommand stopvideos_fadeout
r5apex.exe!0x02424240 ConCommand surfaceprop
r5apex.exe!0x01857810 ConCommand sv_precacheinfo
r5apex.exe!0x01858f40 ConCommand sv_showents
r5apex.exe!0x01858960 ConCommand sv_shutdown
r5apex.exe!0x02814d70 ConCommand sv_soundscape_printdebuginfo
r5apex.exe!0x0282a4c0 ConCommand sv_test_rotated_box
r5apex.exe!0x023b5460 ConCommand sv_trace_start_solid
r5apex.exe!0x0185a2d0 ConCommand sv_writeSendTableStreamFile
r5apex.exe!0x0281b010 ConCommand swap_to_weapon
r5apex.exe!0x023dad90 ConCommand switchPlayerClassActivityMod
r5apex.exe!0x023d73e0 ConCommand switchclass
r5apex.exe!0x028130d0 ConCommand takecurrentammo
r5apex.exe!0x01f74cf0 ConCommand testCockpitJoltAngles
r5apex.exe!0x01f97e30 ConCommand testCockpitJoltOrigin
r5apex.exe!0x023bb870 ConCommand test_entity_blocker
r5apex.exe!0x0233c1f0 ConCommand test_freezeframe
r5apex.exe!0x02477310 ConCommand test_setteam
r5apex.exe!0x0233a840 ConCommand testhudanim
r5apex.exe!0x01753de0 ConCommand thread_test_tslist
r5apex.exe!0x0174a5d0 ConCommand thread_test_tsqueue
r5apex.exe!0x0233d2c0 ConCommand titan_loadout_select
r5apex.exe!0x01749930 ConCommand toggle
r5apex.exe!0x02345bf0 ConCommand toggle_inventory
r5apex.exe!0x0233fbe0 ConCommand toggle_map
r5apex.exe!0x023cdbc0 ConCommand trace_capsule
r5apex.exe!0x02384260 ConCommand ui_reloadscheme
r5apex.exe!0x02340510 ConCommand uiscript_reset
r5apex.exe!0x02343860 ConCommand uiscript_resolutionchanged
r5apex.exe!0x0174ce80 ConCommand unbind
r5apex.exe!0x0174eb10 ConCommand unbind_US_standard
r5apex.exe!0x01752a00 ConCommand unbind_all_gamepad
r5apex.exe!0x01751190 ConCommand unbind_batch
r5apex.exe!0x017525a0 ConCommand unbind_held
r5apex.exe!0x0174bda0 ConCommand unbind_held_US_standard
r5apex.exe!0x0174eef0 ConCommand unbindall
r5apex.exe!0x0174df50 ConCommand unbindall_ignoreGamepad
r5apex.exe!0x01752460 ConCommand unload_level_loadscreen
r5apex.exe!0x018dfd10 ConCommand unmuteroom
r5apex.exe!0x02343fe0 ConCommand unpausevideos
r5apex.exe!0x023437e0 ConCommand use_consumable
r5apex.exe!0x018595f0 ConCommand user
r5apex.exe!0x01858d90 ConCommand users
r5apex.exe!0x017526e0 ConCommand version
r5apex.exe!0x01759830 ConCommand vgui_drawtree_clear
r5apex.exe!0x01f6e4b0 ConCommand vgui_spew_fonts
r5apex.exe!0x0175adf0 ConCommand vgui_togglepanel
r5apex.exe!0x0174c6f0 ConCommand voicerecord_toggle
r5apex.exe!0x0174a6d0 ConCommand vx_datacache_list
r5apex.exe!0x0174d000 ConCommand vx_model_list
r5apex.exe!0x0232fb10 ConCommand weaponSelectOrdnance
r5apex.exe!0x02337b00 ConCommand weaponSelectPrimary0
r5apex.exe!0x0232cbf0 ConCommand weaponSelectPrimary1
r5apex.exe!0x0232bdd0 ConCommand weaponSelectPrimary2
r5apex.exe!0x02335920 ConCommand weapon_activity
r5apex.exe!0x02325910 ConCommand weapon_inspect
r5apex.exe!0x01f84a60 ConCommand weapon_list
r5apex.exe!0x0237c4b0 ConCommand weapon_reparse
r5apex.exe!0x0174e510 ConCommand xlog_list
r5apex.exe!0x01756aa0 ConCommand xlog_record
r5apex.exe!0x01752500 ConCommand xlog_record_that
r5apex.exe!0x0174d9e0 ConCommand xlog_stop
r5apex.exe!0x023340c0 ConCommand xlook
r5apex.exe!0x023262d0 ConCommand xmove
```

## Globals

List of global variables with an associated vtable and their type name.

```
r5apex.exe!0x02803d98 .?AUSQArray@@
r5apex.exe!0x027f80b8 .?AUSQClass@@
r5apex.exe!0x024778b8 .?AUSQClosure@@
r5apex.exe!0x02476468 .?AUSQFunctionProto@@
r5apex.exe!0x024242c8 .?AUSQInstance@@
r5apex.exe!0x02806228 .?AUSQNativeClosure@@
r5apex.exe!0x02434c78 .?AUSQString@@
r5apex.exe!0x02803588 .?AUSQStructDef@@
r5apex.exe!0x02475b48 .?AUSQStructInstance@@
r5apex.exe!0x024761b8 .?AUSQTable@@
r5apex.exe!0x02424358 .?AUSQUserData@@
r5apex.exe!0x0280a228 .?AUSQVM@@
r5apex.exe!0x0280b938 .?AUSQWeakRef@@
r5apex.exe!0x22ab3b50 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22ab3bb8 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22ab3bc0 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22ab3bf0 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22ab3c58 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22ab3c60 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22ab3c90 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22ab3cf8 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22ab3d00 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22ab3d30 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22ab3d98 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22ab3da0 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22ab3dd0 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22ab3e38 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22ab3e40 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22ab3e70 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22ab3ed8 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22ab3ee0 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x017466c8 .?AV?$CDataManager@UDataCacheItem_t@@UDataCacheItemData_t@@PEAU1@VCThreadFastMutex@@@@
r5apex.exe!0x0c6cf430 .?AV?$CDataManager@VCBoneCache@@Ubonecacheparams_t@@PEAV1@VCThreadFastMutex@@@@
r5apex.exe!0x0c6cf4e0 .?AV?$CDataManager@VCBoneCache@@Ubonecacheparams_t@@PEAV1@VCThreadFastMutex@@@@
r5apex.exe!0x02807fe0 .?AV?$CEntityClassList@VCPhysicsNPCSolver@@@@
r5apex.exe!0x028032a0 .?AV?$CEntityClassList@VCPointCamera@@@@
r5apex.exe!0x028122e0 .?AV?$CEntityClassList@VCSkyCamera@@@@
r5apex.exe!0x028bcd08 .?AV?$CEntityFactory@VCAI_BaseNPC@@@@
r5apex.exe!0x028bf930 .?AV?$CEntityFactory@VCAI_ChangeTarget@@@@
r5apex.exe!0x028bdc68 .?AV?$CEntityFactory@VCAI_DynamicLink@@@@
r5apex.exe!0x028bd438 .?AV?$CEntityFactory@VCAI_DynamicLinkController@@@@
r5apex.exe!0x028bdb28 .?AV?$CEntityFactory@VCAI_Hint@@@@
r5apex.exe!0x028bd270 .?AV?$CEntityFactory@VCAI_NetworkManager@@@@
r5apex.exe!0x028bc578 .?AV?$CEntityFactory@VCAI_RadialLinkController@@@@
r5apex.exe!0x028bd108 .?AV?$CEntityFactory@VCAI_SkitNode@@@@
r5apex.exe!0x028bd208 .?AV?$CEntityFactory@VCAI_TestHull@@@@
r5apex.exe!0x028bc908 .?AV?$CEntityFactory@VCAmbientGeneric@@@@
r5apex.exe!0x028bcd40 .?AV?$CEntityFactory@VCAssaultPoint@@@@
r5apex.exe!0x028bd820 .?AV?$CEntityFactory@VCBaseAnimating@@@@
r5apex.exe!0x028becc8 .?AV?$CEntityFactory@VCBaseDMStart@@@@
r5apex.exe!0x028bd000 .?AV?$CEntityFactory@VCBaseEntity@@@@
r5apex.exe!0x028ba028 .?AV?$CEntityFactory@VCBaseGrenade@@@@
r5apex.exe!0x028bf500 .?AV?$CEntityFactory@VCBaseTrigger@@@@
r5apex.exe!0x028b9188 .?AV?$CEntityFactory@VCBaseViewModel@@@@
r5apex.exe!0x028ba3d8 .?AV?$CEntityFactory@VCBeam@@@@
r5apex.exe!0x028bde58 .?AV?$CEntityFactory@VCBoneFollower@@@@
r5apex.exe!0x028be0f8 .?AV?$CEntityFactory@VCBreakable@@@@
r5apex.exe!0x028be100 .?AV?$CEntityFactory@VCBreakableSurface@@@@
r5apex.exe!0x028be1f0 .?AV?$CEntityFactory@VCCascadeLight@@@@
r5apex.exe!0x028be2b8 .?AV?$CEntityFactory@VCColorCorrection@@@@
r5apex.exe!0x028c4f08 .?AV?$CEntityFactory@VCCrossbowBolt@@@@
r5apex.exe!0x028c4788 .?AV?$CEntityFactory@VCDeathBoxProp@@@@
r5apex.exe!0x028c42e8 .?AV?$CEntityFactory@VCDropPodProp@@@@
r5apex.exe!0x028bf938 .?AV?$CEntityFactory@VCDropPodSpawnPoint@@@@
r5apex.exe!0x028bf9c8 .?AV?$CEntityFactory@VCDropPodSpawnPoint@@@@
r5apex.exe!0x028bf180 .?AV?$CEntityFactory@VCDropPoint@@@@
r5apex.exe!0x028bfe28 .?AV?$CEntityFactory@VCDropShipSpawnPoint@@@@
r5apex.exe!0x028c4158 .?AV?$CEntityFactory@VCDropShipSpawnPoint@@@@
r5apex.exe!0x028bdf90 .?AV?$CEntityFactory@VCDynamicLight@@@@
r5apex.exe!0x028beaf8 .?AV?$CEntityFactory@VCDynamicProp@@@@
r5apex.exe!0x028befb8 .?AV?$CEntityFactory@VCDynamicProp@@@@
r5apex.exe!0x028bfa40 .?AV?$CEntityFactory@VCDynamicProp@@@@
r5apex.exe!0x028bfc08 .?AV?$CEntityFactory@VCDynamicProp@@@@
r5apex.exe!0x028bfde8 .?AV?$CEntityFactory@VCDynamicProp@@@@
r5apex.exe!0x028bfe38 .?AV?$CEntityFactory@VCDynamicProp@@@@
r5apex.exe!0x028bfa30 .?AV?$CEntityFactory@VCDynamicPropLightweight@@@@
r5apex.exe!0x028bf298 .?AV?$CEntityFactory@VCEnableMotionFixup@@@@
r5apex.exe!0x028b9e48 .?AV?$CEntityFactory@VCEntityBlocker@@@@
r5apex.exe!0x028be870 .?AV?$CEntityFactory@VCEntityDissolve@@@@
r5apex.exe!0x028bf548 .?AV?$CEntityFactory@VCEntityLinkPage@@@@
r5apex.exe!0x028be650 .?AV?$CEntityFactory@VCEnvBeam@@@@
r5apex.exe!0x028bfa38 .?AV?$CEntityFactory@VCEnvDropZone@@@@
r5apex.exe!0x028be428 .?AV?$CEntityFactory@VCEnvExplosion@@@@
r5apex.exe!0x028be868 .?AV?$CEntityFactory@VCEnvLaser@@@@
r5apex.exe!0x028bc438 .?AV?$CEntityFactory@VCEnvLight@@@@
r5apex.exe!0x028be1a8 .?AV?$CEntityFactory@VCEnvShake@@@@
r5apex.exe!0x028bf7f0 .?AV?$CEntityFactory@VCEnvSoundscape@@@@
r5apex.exe!0x028bf9d0 .?AV?$CEntityFactory@VCEnvSoundscapeProxy@@@@
r5apex.exe!0x028bf630 .?AV?$CEntityFactory@VCEnvSoundscapeTriggerable@@@@
r5apex.exe!0x028bddd8 .?AV?$CEntityFactory@VCEnvTonemapController@@@@
r5apex.exe!0x028be7a0 .?AV?$CEntityFactory@VCEnvWind@@@@
r5apex.exe!0x028c5508 .?AV?$CEntityFactory@VCFirstPersonProxy@@@@
r5apex.exe!0x028bdd70 .?AV?$CEntityFactory@VCFogController@@@@
r5apex.exe!0x028bdef8 .?AV?$CEntityFactory@VCFogTrigger@@@@
r5apex.exe!0x028be538 .?AV?$CEntityFactory@VCFogVolume@@@@
r5apex.exe!0x028be440 .?AV?$CEntityFactory@VCFuncBrush@@@@
r5apex.exe!0x028bdfc8 .?AV?$CEntityFactory@VCFuncBrushLightweight@@@@
r5apex.exe!0x028bdfd0 .?AV?$CEntityFactory@VCFuncMoveLinear@@@@
r5apex.exe!0x028bb078 .?AV?$CEntityFactory@VCGameGibManager@@@@
r5apex.exe!0x028bdde0 .?AV?$CEntityFactory@VCGameOperator@@@@
r5apex.exe!0x028be328 .?AV?$CEntityFactory@VCGamePlayerEquip@@@@
r5apex.exe!0x028be048 .?AV?$CEntityFactory@VCGamePlayerTeam@@@@
r5apex.exe!0x028ba108 .?AV?$CEntityFactory@VCGameRulesProxy@@@@
r5apex.exe!0x028be438 .?AV?$CEntityFactory@VCGameText@@@@
r5apex.exe!0x028be040 .?AV?$CEntityFactory@VCGameUIEntity@@@@
r5apex.exe!0x028be798 .?AV?$CEntityFactory@VCGib@@@@
r5apex.exe!0x028ba018 .?AV?$CEntityFactory@VCGlobalNonRewinding@@@@
r5apex.exe!0x028c4998 .?AV?$CEntityFactory@VCGrappleHook@@@@
r5apex.exe!0x028be8b0 .?AV?$CEntityFactory@VCHardPointEntity@@@@
r5apex.exe!0x028c4618 .?AV?$CEntityFactory@VCHardPointFrontierEntity@@@@
r5apex.exe!0x028bc3c8 .?AV?$CEntityFactory@VCHealthKit@@@@
r5apex.exe!0x028bc3d0 .?AV?$CEntityFactory@VCHealthKit@@@@
r5apex.exe!0x028bc3d8 .?AV?$CEntityFactory@VCHealthKit@@@@
r5apex.exe!0x028bc3e0 .?AV?$CEntityFactory@VCHealthKit@@@@
r5apex.exe!0x028bc3e8 .?AV?$CEntityFactory@VCHealthKit@@@@
r5apex.exe!0x028bc3f0 .?AV?$CEntityFactory@VCHealthKit@@@@
r5apex.exe!0x028beb08 .?AV?$CEntityFactory@VCHumanSizeNPCSpawnPoint@@@@
r5apex.exe!0x028bfdb0 .?AV?$CEntityFactory@VCHumanSizeNPCSpawnPoint@@@@
r5apex.exe!0x028bea30 .?AV?$CEntityFactory@VCHumanSpawnPoint@@@@
r5apex.exe!0x028bf590 .?AV?$CEntityFactory@VCHumanSpawnPoint@@@@
r5apex.exe!0x028c46d8 .?AV?$CEntityFactory@VCImportantOnEntSound@@@@
r5apex.exe!0x028be430 .?AV?$CEntityFactory@VCInfoCameraLink@@@@
r5apex.exe!0x028be4f8 .?AV?$CEntityFactory@VCInfoIntermission@@@@
r5apex.exe!0x028c48c0 .?AV?$CEntityFactory@VCInfoPlacementHelper@@@@
r5apex.exe!0x028b8d38 .?AV?$CEntityFactory@VCInfoTarget@@@@
r5apex.exe!0x028b8f58 .?AV?$CEntityFactory@VCInfoTarget@@@@
r5apex.exe!0x028b9378 .?AV?$CEntityFactory@VCInfoTarget@@@@
r5apex.exe!0x028ba020 .?AV?$CEntityFactory@VCInfoTarget@@@@
r5apex.exe!0x028b9e50 .?AV?$CEntityFactory@VCInfoTargetMinimap@@@@
r5apex.exe!0x028bc428 .?AV?$CEntityFactory@VCLight@@@@
r5apex.exe!0x028bc430 .?AV?$CEntityFactory@VCLight@@@@
r5apex.exe!0x028bde90 .?AV?$CEntityFactory@VCMessageEntity@@@@
r5apex.exe!0x028c4c40 .?AV?$CEntityFactory@VCMissile@@@@
r5apex.exe!0x028c4630 .?AV?$CEntityFactory@VCMovementSpeedMod@@@@
r5apex.exe!0x028be390 .?AV?$CEntityFactory@VCMovieDisplay@@@@
r5apex.exe!0x028c45a0 .?AV?$CEntityFactory@VCNPCProwlerSpawnPoint@@@@
r5apex.exe!0x028c4858 .?AV?$CEntityFactory@VCNPC_Bullseye@@@@
r5apex.exe!0x028be050 .?AV?$CEntityFactory@VCNPC_Drone@@@@
r5apex.exe!0x028be320 .?AV?$CEntityFactory@VCNPC_Drone@@@@
r5apex.exe!0x028be5e8 .?AV?$CEntityFactory@VCNPC_Drone@@@@
r5apex.exe!0x028be6f0 .?AV?$CEntityFactory@VCNPC_Drone@@@@
r5apex.exe!0x028bdd88 .?AV?$CEntityFactory@VCNPC_Dropship@@@@
r5apex.exe!0x028be1a0 .?AV?$CEntityFactory@VCNPC_Dropship@@@@
r5apex.exe!0x028be058 .?AV?$CEntityFactory@VCNPC_Flyer@@@@
r5apex.exe!0x028be540 .?AV?$CEntityFactory@VCNPC_Goliath@@@@
r5apex.exe!0x028bdd08 .?AV?$CEntityFactory@VCNPC_Gunship@@@@
r5apex.exe!0x028c57d8 .?AV?$CEntityFactory@VCNPC_Marvin@@@@
r5apex.exe!0x028bdde8 .?AV?$CEntityFactory@VCNPC_MeleeOnly@@@@
r5apex.exe!0x028be478 .?AV?$CEntityFactory@VCNPC_MeleeOnly@@@@
r5apex.exe!0x028be878 .?AV?$CEntityFactory@VCNPC_MeleeOnly@@@@
r5apex.exe!0x028c5848 .?AV?$CEntityFactory@VCNPC_Pilot@@@@
r5apex.exe!0x028c5930 .?AV?$CEntityFactory@VCNPC_SentryTurret@@@@
r5apex.exe!0x028c5938 .?AV?$CEntityFactory@VCNPC_SentryTurret@@@@
r5apex.exe!0x028c57e8 .?AV?$CEntityFactory@VCNPC_Soldier@@@@
r5apex.exe!0x028c57f0 .?AV?$CEntityFactory@VCNPC_Soldier@@@@
r5apex.exe!0x028c5800 .?AV?$CEntityFactory@VCNPC_Soldier@@@@
r5apex.exe!0x028c5808 .?AV?$CEntityFactory@VCNPC_Soldier@@@@
r5apex.exe!0x028c5810 .?AV?$CEntityFactory@VCNPC_Soldier@@@@
r5apex.exe!0x028c5850 .?AV?$CEntityFactory@VCNPC_Soldier@@@@
r5apex.exe!0x028c57e0 .?AV?$CEntityFactory@VCNPC_Spectre@@@@
r5apex.exe!0x028c57f8 .?AV?$CEntityFactory@VCNPC_Spectre@@@@
r5apex.exe!0x028c5858 .?AV?$CEntityFactory@VCNPC_Spectre@@@@
r5apex.exe!0x028be760 .?AV?$CEntityFactory@VCNPC_SuperSpectre@@@@
r5apex.exe!0x028c58f0 .?AV?$CEntityFactory@VCNPC_Titan@@@@
r5apex.exe!0x028bc470 .?AV?$CEntityFactory@VCNodeEnt@@@@
r5apex.exe!0x028bc5e0 .?AV?$CEntityFactory@VCNodeEnt@@@@
r5apex.exe!0x028bc9d0 .?AV?$CEntityFactory@VCNodeEnt@@@@
r5apex.exe!0x028bca38 .?AV?$CEntityFactory@VCNodeEnt@@@@
r5apex.exe!0x028bcdf8 .?AV?$CEntityFactory@VCNodeEnt@@@@
r5apex.exe!0x028bcec0 .?AV?$CEntityFactory@VCNodeEnt@@@@
r5apex.exe!0x028bcf38 .?AV?$CEntityFactory@VCNodeEnt@@@@
r5apex.exe!0x028bd440 .?AV?$CEntityFactory@VCNodeEnt@@@@
r5apex.exe!0x028bd588 .?AV?$CEntityFactory@VCNodeEnt@@@@
r5apex.exe!0x028bd620 .?AV?$CEntityFactory@VCNodeEnt@@@@
r5apex.exe!0x028bd718 .?AV?$CEntityFactory@VCNodeEnt@@@@
r5apex.exe!0x028bd8f0 .?AV?$CEntityFactory@VCNodeEnt@@@@
r5apex.exe!0x028bdbf8 .?AV?$CEntityFactory@VCNodeEnt@@@@
r5apex.exe!0x028be9c8 .?AV?$CEntityFactory@VCNullEntity@@@@
r5apex.exe!0x028bf290 .?AV?$CEntityFactory@VCNullEntity@@@@
r5apex.exe!0x028be728 .?AV?$CEntityFactory@VCParticleSystem@@@@
r5apex.exe!0x028bdfd8 .?AV?$CEntityFactory@VCPathCorner@@@@
r5apex.exe!0x028be1b0 .?AV?$CEntityFactory@VCPathCorner@@@@
r5apex.exe!0x028be548 .?AV?$CEntityFactory@VCPathCorner@@@@
r5apex.exe!0x028be1e8 .?AV?$CEntityFactory@VCPathCornerCrash@@@@
r5apex.exe!0x028be168 .?AV?$CEntityFactory@VCPathTrack@@@@
r5apex.exe!0x028be688 .?AV?$CEntityFactory@VCPatrolPath@@@@
r5apex.exe!0x028bf5c8 .?AV?$CEntityFactory@VCPhysBox@@@@
r5apex.exe!0x028c3fc8 .?AV?$CEntityFactory@VCPhysExplosion@@@@
r5apex.exe!0x028c42f0 .?AV?$CEntityFactory@VCPhysImpact@@@@
r5apex.exe!0x028bfdf0 .?AV?$CEntityFactory@VCPhysicsEntitySolver@@@@
r5apex.exe!0x028bf9e0 .?AV?$CEntityFactory@VCPhysicsNPCSolver@@@@
r5apex.exe!0x028bf7f8 .?AV?$CEntityFactory@VCPhysicsProp@@@@
r5apex.exe!0x028c3f20 .?AV?$CEntityFactory@VCPhysicsProp@@@@
r5apex.exe!0x028c3fd0 .?AV?$CEntityFactory@VCPhysicsProp@@@@
r5apex.exe!0x028be8e8 .?AV?$CEntityFactory@VCPlayer@@@@
r5apex.exe!0x028bbe18 .?AV?$CEntityFactory@VCPlayerDecoy@@@@
r5apex.exe!0x028bf258 .?AV?$CEntityFactory@VCPlayerResource@@@@
r5apex.exe!0x028bc048 .?AV?$CEntityFactory@VCPlayerTasklist@@@@
r5apex.exe!0x028bf840 .?AV?$CEntityFactory@VCPlayerVehicle@@@@
r5apex.exe!0x028bbf08 .?AV?$CEntityFactory@VCPlayerWaypoint@@@@
r5apex.exe!0x028be530 .?AV?$CEntityFactory@VCPointBroadcastClientCommand@@@@
r5apex.exe!0x028bf830 .?AV?$CEntityFactory@VCPointCamera@@@@
r5apex.exe!0x028bdd00 .?AV?$CEntityFactory@VCPointClientCommand@@@@
r5apex.exe!0x028bd8e8 .?AV?$CEntityFactory@VCPointEntity@@@@
r5apex.exe!0x028bed38 .?AV?$CEntityFactory@VCPointEntity@@@@
r5apex.exe!0x028bef48 .?AV?$CEntityFactory@VCPointEntity@@@@
r5apex.exe!0x028bf438 .?AV?$CEntityFactory@VCPointEntity@@@@
r5apex.exe!0x028bf788 .?AV?$CEntityFactory@VCPointEntity@@@@
r5apex.exe!0x028bf958 .?AV?$CEntityFactory@VCPointEntity@@@@
r5apex.exe!0x028bf960 .?AV?$CEntityFactory@VCPointEntity@@@@
r5apex.exe!0x028bfa78 .?AV?$CEntityFactory@VCPointEntity@@@@
r5apex.exe!0x028c4390 .?AV?$CEntityFactory@VCPointEntity@@@@
r5apex.exe!0x028c4398 .?AV?$CEntityFactory@VCPointPlayerMoveConstraint@@@@
r5apex.exe!0x028be4b8 .?AV?$CEntityFactory@VCPointServerCommand@@@@
r5apex.exe!0x028bfd28 .?AV?$CEntityFactory@VCPointSpotlight@@@@
r5apex.exe!0x028c4478 .?AV?$CEntityFactory@VCPointTemplate@@@@
r5apex.exe!0x028c46e8 .?AV?$CEntityFactory@VCPointTemplate@@@@
r5apex.exe!0x028c48c8 .?AV?$CEntityFactory@VCPortal_PointPush@@@@
r5apex.exe!0x028bf178 .?AV?$CEntityFactory@VCPostProcessController@@@@
r5apex.exe!0x028c52d8 .?AV?$CEntityFactory@VCPredictedFirstPersonProxy@@@@
r5apex.exe!0x028b9458 .?AV?$CEntityFactory@VCPropDoor@@@@
r5apex.exe!0x028c44e8 .?AV?$CEntityFactory@VCPropSurvival@@@@
r5apex.exe!0x028bdd78 .?AV?$CEntityFactory@VCPushable@@@@
r5apex.exe!0x028bf940 .?AV?$CEntityFactory@VCRevertSaved@@@@
r5apex.exe!0x028bf838 .?AV?$CEntityFactory@VCRopeKeyframe@@@@
r5apex.exe!0x028c4750 .?AV?$CEntityFactory@VCRopeKeyframe@@@@
r5apex.exe!0x028bad48 .?AV?$CEntityFactory@VCScriptMover@@@@
r5apex.exe!0x028bb368 .?AV?$CEntityFactory@VCScriptMoverWaypoint@@@@
r5apex.exe!0x028bb820 .?AV?$CEntityFactory@VCScriptNetDataGlobal@@@@
r5apex.exe!0x028ba868 .?AV?$CEntityFactory@VCScriptNetData_SNDC_DEATH_BOX@@@@
r5apex.exe!0x028badb8 .?AV?$CEntityFactory@VCScriptNetData_SNDC_GLOBAL@@@@
r5apex.exe!0x028bb978 .?AV?$CEntityFactory@VCScriptNetData_SNDC_PLAYER_EXCLUSIVE@@@@
r5apex.exe!0x028bb818 .?AV?$CEntityFactory@VCScriptNetData_SNDC_PLAYER_GLOBAL@@@@
r5apex.exe!0x028bb738 .?AV?$CEntityFactory@VCScriptNetData_SNDC_TITAN_SOUL@@@@
r5apex.exe!0x028becc0 .?AV?$CEntityFactory@VCScriptProp@@@@
r5apex.exe!0x028bfc90 .?AV?$CEntityFactory@VCScriptProp@@@@
r5apex.exe!0x028c53b8 .?AV?$CEntityFactory@VCScriptTraceVolume@@@@
r5apex.exe!0x028be480 .?AV?$CEntityFactory@VCSearchPath@@@@
r5apex.exe!0x028bef80 .?AV?$CEntityFactory@VCShieldProp@@@@
r5apex.exe!0x028beb48 .?AV?$CEntityFactory@VCSimplePhysicsBrush@@@@
r5apex.exe!0x028bf9e8 .?AV?$CEntityFactory@VCSimplePhysicsProp@@@@
r5apex.exe!0x028bfa80 .?AV?$CEntityFactory@VCSkyCamera@@@@
r5apex.exe!0x028c40a8 .?AV?$CEntityFactory@VCSkyboxSwapper@@@@
r5apex.exe!0x028bfd40 .?AV?$CEntityFactory@VCSoundEnt@@@@
r5apex.exe!0x028bfc50 .?AV?$CEntityFactory@VCSpawnPointFlag@@@@
r5apex.exe!0x028beb00 .?AV?$CEntityFactory@VCSpawner@@@@
r5apex.exe!0x028becb8 .?AV?$CEntityFactory@VCSpotlightEnd@@@@
r5apex.exe!0x028bbe90 .?AV?$CEntityFactory@VCSprite@@@@
r5apex.exe!0x028ba7f8 .?AV?$CEntityFactory@VCSpriteOriented@@@@
r5apex.exe!0x028bb148 .?AV?$CEntityFactory@VCStatusEffectPlugin@@@@
r5apex.exe!0x028bf950 .?AV?$CEntityFactory@VCTeam@@@@
r5apex.exe!0x028bf8b8 .?AV?$CEntityFactory@VCTeamSpawnPoint@@@@
r5apex.exe!0x028bf6d8 .?AV?$CEntityFactory@VCTeamVehicleSpawnPoint@@@@
r5apex.exe!0x028be060 .?AV?$CEntityFactory@VCTempEntTester@@@@
r5apex.exe!0x028c5928 .?AV?$CEntityFactory@VCTitanSoul@@@@
r5apex.exe!0x028bf098 .?AV?$CEntityFactory@VCTitanSpawnPoint@@@@
r5apex.exe!0x028bf0d0 .?AV?$CEntityFactory@VCTitanSpawnPoint@@@@
r5apex.exe!0x028bfd38 .?AV?$CEntityFactory@VCTitanSpawnPoint@@@@
r5apex.exe!0x028be5e0 .?AV?$CEntityFactory@VCTonemapTrigger@@@@
r5apex.exe!0x028bd788 .?AV?$CEntityFactory@VCTraverseRef@@@@
r5apex.exe!0x028befc0 .?AV?$CEntityFactory@VCTriggerAutoCrouch@@@@
r5apex.exe!0x028bddf0 .?AV?$CEntityFactory@VCTriggerBrush@@@@
r5apex.exe!0x028bfc10 .?AV?$CEntityFactory@VCTriggerCamera@@@@
r5apex.exe!0x028beb40 .?AV?$CEntityFactory@VCTriggerCylinder@@@@
r5apex.exe!0x028bf9f0 .?AV?$CEntityFactory@VCTriggerCylinderHeavy@@@@
r5apex.exe!0x028bfc48 .?AV?$CEntityFactory@VCTriggerGravity@@@@
r5apex.exe!0x028bed30 .?AV?$CEntityFactory@VCTriggerHurt@@@@
r5apex.exe!0x028bf508 .?AV?$CEntityFactory@VCTriggerImpact@@@@
r5apex.exe!0x028c4408 .?AV?$CEntityFactory@VCTriggerLocation@@@@
r5apex.exe!0x028c4280 .?AV?$CEntityFactory@VCTriggerLocationSP@@@@
r5apex.exe!0x028bfc88 .?AV?$CEntityFactory@VCTriggerLook@@@@
r5apex.exe!0x028be920 .?AV?$CEntityFactory@VCTriggerMultiple@@@@
r5apex.exe!0x028bf090 .?AV?$CEntityFactory@VCTriggerMultiple@@@@
r5apex.exe!0x028bf428 .?AV?$CEntityFactory@VCTriggerMultiple@@@@
r5apex.exe!0x028bf430 .?AV?$CEntityFactory@VCTriggerMultiple@@@@
r5apex.exe!0x028bf588 .?AV?$CEntityFactory@VCTriggerMultiple@@@@
r5apex.exe!0x028bf948 .?AV?$CEntityFactory@VCTriggerMultiple@@@@
r5apex.exe!0x028bfe30 .?AV?$CEntityFactory@VCTriggerMultiple@@@@
r5apex.exe!0x028c40b8 .?AV?$CEntityFactory@VCTriggerMultiple@@@@
r5apex.exe!0x028c4328 .?AV?$CEntityFactory@VCTriggerMultiple@@@@
r5apex.exe!0x028c44f0 .?AV?$CEntityFactory@VCTriggerMultiple@@@@
r5apex.exe!0x028c44f8 .?AV?$CEntityFactory@VCTriggerMultiple@@@@
r5apex.exe!0x028c4568 .?AV?$CEntityFactory@VCTriggerMultiple@@@@
r5apex.exe!0x028c45a8 .?AV?$CEntityFactory@VCTriggerMultiple@@@@
r5apex.exe!0x028c4638 .?AV?$CEntityFactory@VCTriggerMultiple@@@@
r5apex.exe!0x028c46e0 .?AV?$CEntityFactory@VCTriggerMultiple@@@@
r5apex.exe!0x028bf9d8 .?AV?$CEntityFactory@VCTriggerNoGrapple@@@@
r5apex.exe!0x028c43d0 .?AV?$CEntityFactory@VCTriggerNoZipline@@@@
r5apex.exe!0x028bfd48 .?AV?$CEntityFactory@VCTriggerOnce@@@@
r5apex.exe!0x028bf710 .?AV?$CEntityFactory@VCTriggerPlayerMovement@@@@
r5apex.exe!0x028bfd30 .?AV?$CEntityFactory@VCTriggerPointGravity@@@@
r5apex.exe!0x028bfc00 .?AV?$CEntityFactory@VCTriggerProximity@@@@
r5apex.exe!0x028c4628 .?AV?$CEntityFactory@VCTriggerPush@@@@
r5apex.exe!0x028bfbc8 .?AV?$CEntityFactory@VCTriggerRemove@@@@
r5apex.exe!0x028bc238 .?AV?$CEntityFactory@VCTriggerSlip@@@@
r5apex.exe!0x028bf9f8 .?AV?$CEntityFactory@VCTriggerSoundscape@@@@
r5apex.exe!0x028c4820 .?AV?$CEntityFactory@VCTriggerTeleport@@@@
r5apex.exe!0x028bf580 .?AV?$CEntityFactory@VCTriggerViewProxy@@@@
r5apex.exe!0x028beff8 .?AV?$CEntityFactory@VCTriggerWind@@@@
r5apex.exe!0x028c5500 .?AV?$CEntityFactory@VCTurret@@@@
r5apex.exe!0x028bf928 .?AV?$CEntityFactory@VCVGuiScreen@@@@
r5apex.exe!0x028c40b0 .?AV?$CEntityFactory@VCVGuiScreen@@@@
r5apex.exe!0x028c4a98 .?AV?$CEntityFactory@VCVortexSphere@@@@
r5apex.exe!0x028c54c8 .?AV?$CEntityFactory@VCWallrunCurve@@@@
r5apex.exe!0x028bf540 .?AV?$CEntityFactory@VCWaterLODControl@@@@
r5apex.exe!0x028c56a8 .?AV?$CEntityFactory@VCWeaponX@@@@
r5apex.exe!0x028c5260 .?AV?$CEntityFactory@VCWindowHint@@@@
r5apex.exe!0x028bdd80 .?AV?$CEntityFactory@VCWindowPane@@@@
r5apex.exe!0x028c4620 .?AV?$CEntityFactory@VCWorld@@@@
r5apex.exe!0x028be4c0 .?AV?$CEntityFactory@VCWorldItem@@@@
r5apex.exe!0x028c5228 .?AV?$CEntityFactory@VCZipline@@@@
r5apex.exe!0x028c4c38 .?AV?$CEntityFactory@VCZiplineEnd@@@@
r5apex.exe!0x028bc1c8 .?AV?$CEntityFactory@VDoorMover@@@@
r5apex.exe!0x028bbe88 .?AV?$CEntityFactory@VScriptMoverLightweight@@@@
r5apex.exe!0x01859dc8 .?AV?$CPanelFactory@VCMovieDisplayScreen@@UVGuiScreenInitData_t@@@@
r5apex.exe!0x018564a8 .?AV?$CPanelFactory@VCVGuiScreenPanel@@UVGuiScreenInitData_t@@@@
r5apex.exe!0x023265f8 .?AV?$CParticleOperatorDefinition@VC_INIT_AgeNoise@@@@
r5apex.exe!0x02325778 .?AV?$CParticleOperatorDefinition@VC_INIT_ChaoticAttractor@@@@
r5apex.exe!0x02326818 .?AV?$CParticleOperatorDefinition@VC_INIT_ColorLitPerParticle@@@@
r5apex.exe!0x0231a208 .?AV?$CParticleOperatorDefinition@VC_INIT_CreateAlongPath@@@@
r5apex.exe!0x01f92e48 .?AV?$CParticleOperatorDefinition@VC_INIT_CreateFromParentParticles@@@@
r5apex.exe!0x02326898 .?AV?$CParticleOperatorDefinition@VC_INIT_CreateFromPlaneCache@@@@
r5apex.exe!0x0232a958 .?AV?$CParticleOperatorDefinition@VC_INIT_CreateInEpitrochoid@@@@
r5apex.exe!0x02325e98 .?AV?$CParticleOperatorDefinition@VC_INIT_CreateInHierarchy@@@@
r5apex.exe!0x01f8d488 .?AV?$CParticleOperatorDefinition@VC_INIT_CreateOnModel@@@@
r5apex.exe!0x02326918 .?AV?$CParticleOperatorDefinition@VC_INIT_CreateSequentialPath@@@@
r5apex.exe!0x02326338 .?AV?$CParticleOperatorDefinition@VC_INIT_CreateWithinBox@@@@
r5apex.exe!0x0231a708 .?AV?$CParticleOperatorDefinition@VC_INIT_CreateWithinControlPointBox@@@@
r5apex.exe!0x0232c418 .?AV?$CParticleOperatorDefinition@VC_INIT_CreateWithinSphere@@@@
r5apex.exe!0x01f97e98 .?AV?$CParticleOperatorDefinition@VC_INIT_CreationNoise@@@@
r5apex.exe!0x0232bc18 .?AV?$CParticleOperatorDefinition@VC_INIT_DistanceToCPInit@@@@
r5apex.exe!0x02318a38 .?AV?$CParticleOperatorDefinition@VC_INIT_InheritFromParentParticles@@@@
r5apex.exe!0x023260d8 .?AV?$CParticleOperatorDefinition@VC_INIT_InheritVelocity@@@@
r5apex.exe!0x023257f8 .?AV?$CParticleOperatorDefinition@VC_INIT_InitFromParentKilled@@@@
r5apex.exe!0x02325fb8 .?AV?$CParticleOperatorDefinition@VC_INIT_InitialRepulsionVelocity@@@@
r5apex.exe!0x0232c498 .?AV?$CParticleOperatorDefinition@VC_INIT_InitialVelocityNoise@@@@
r5apex.exe!0x01f94298 .?AV?$CParticleOperatorDefinition@VC_INIT_LifespanFromVelocity@@@@
r5apex.exe!0x0232a8b8 .?AV?$CParticleOperatorDefinition@VC_INIT_ModelCull@@@@
r5apex.exe!0x02325bb8 .?AV?$CParticleOperatorDefinition@VC_INIT_MoveBetweenPoints@@@@
r5apex.exe!0x02325d78 .?AV?$CParticleOperatorDefinition@VC_INIT_NormalAlignToCP@@@@
r5apex.exe!0x0232bdb8 .?AV?$CParticleOperatorDefinition@VC_INIT_NormalOffset@@@@
r5apex.exe!0x02318738 .?AV?$CParticleOperatorDefinition@VC_INIT_OffsetVectorToVector@@@@
r5apex.exe!0x02326458 .?AV?$CParticleOperatorDefinition@VC_INIT_PositionOffset@@@@
r5apex.exe!0x01f8d408 .?AV?$CParticleOperatorDefinition@VC_INIT_PositionPlaceOnGround@@@@
r5apex.exe!0x023264d8 .?AV?$CParticleOperatorDefinition@VC_INIT_PositionWarp@@@@
r5apex.exe!0x02326798 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomAlpha@@@@
r5apex.exe!0x01f94178 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomColor@@@@
r5apex.exe!0x02324f58 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomLifeTime@@@@
r5apex.exe!0x01f8bab8 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomRadius@@@@
r5apex.exe!0x01f93668 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomRotation@@@@
r5apex.exe!0x01f8b258 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomRotationSpeed@@@@
r5apex.exe!0x0231a608 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomScalar@@@@
r5apex.exe!0x023255d8 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomSecondSequence@@@@
r5apex.exe!0x0232beb8 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomSequence@@@@
r5apex.exe!0x01f8d298 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomTrailLength@@@@
r5apex.exe!0x0232c658 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomVector@@@@
r5apex.exe!0x0232a838 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomVectorComponent@@@@
r5apex.exe!0x0232bd38 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomYaw@@@@
r5apex.exe!0x01f8d178 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomYawFlip@@@@
r5apex.exe!0x01f94b98 .?AV?$CParticleOperatorDefinition@VC_INIT_RemapCPtoScalar@@@@
r5apex.exe!0x01f92a78 .?AV?$CParticleOperatorDefinition@VC_INIT_RemapCPtoVector@@@@
r5apex.exe!0x023192c8 .?AV?$CParticleOperatorDefinition@VC_INIT_RemapInitialCPDirectionToRotation@@@@
r5apex.exe!0x01f92cd8 .?AV?$CParticleOperatorDefinition@VC_INIT_RemapInitialDirectionToCPToVector@@@@
r5apex.exe!0x023256f8 .?AV?$CParticleOperatorDefinition@VC_INIT_RemapParticleCountToScalar@@@@
r5apex.exe!0x01f8d308 .?AV?$CParticleOperatorDefinition@VC_INIT_RemapScalar@@@@
r5apex.exe!0x0232af88 .?AV?$CParticleOperatorDefinition@VC_INIT_RemapScalarToVector@@@@
r5apex.exe!0x01f8b998 .?AV?$CParticleOperatorDefinition@VC_INIT_RemapSpeedToScalar@@@@
r5apex.exe!0x0232baf8 .?AV?$CParticleOperatorDefinition@VC_INIT_RemapWorldCPtoScreen@@@@
r5apex.exe!0x0232be38 .?AV?$CParticleOperatorDefinition@VC_INIT_RingWave@@@@
r5apex.exe!0x02326a38 .?AV?$CParticleOperatorDefinition@VC_INIT_SequenceFromCP@@@@
r5apex.exe!0x01f93e48 .?AV?$CParticleOperatorDefinition@VC_INIT_SequenceLifeTime@@@@
r5apex.exe!0x02326718 .?AV?$CParticleOperatorDefinition@VC_INIT_SetCPPosition@@@@
r5apex.exe!0x0231a688 .?AV?$CParticleOperatorDefinition@VC_INIT_SetHitboxToClosest@@@@
r5apex.exe!0x023259f8 .?AV?$CParticleOperatorDefinition@VC_INIT_SetHitboxToModel@@@@
r5apex.exe!0x02325978 .?AV?$CParticleOperatorDefinition@VC_INIT_VelocityFromCP@@@@
r5apex.exe!0x0232ba78 .?AV?$CParticleOperatorDefinition@VC_INIT_VelocityRandom@@@@
r5apex.exe!0x02337328 .?AV?$CParticleOperatorDefinition@VC_OP_AlphaDecay@@@@
r5apex.exe!0x0232cc58 .?AV?$CParticleOperatorDefinition@VC_OP_AttractToControlPoint@@@@
r5apex.exe!0x02337be8 .?AV?$CParticleOperatorDefinition@VC_OP_AxisSpin@@@@
r5apex.exe!0x0232ea08 .?AV?$CParticleOperatorDefinition@VC_OP_BasicMovement@@@@
r5apex.exe!0x01f84ac8 .?AV?$CParticleOperatorDefinition@VC_OP_BoxConstraint@@@@
r5apex.exe!0x0232dd08 .?AV?$CParticleOperatorDefinition@VC_OP_CPOffsetToPercentageBetweenCPs@@@@
r5apex.exe!0x02339c08 .?AV?$CParticleOperatorDefinition@VC_OP_ClampScalar@@@@
r5apex.exe!0x02334368 .?AV?$CParticleOperatorDefinition@VC_OP_ClampVector@@@@
r5apex.exe!0x02331bd8 .?AV?$CParticleOperatorDefinition@VC_OP_ColorInterpolate@@@@
r5apex.exe!0x01f87308 .?AV?$CParticleOperatorDefinition@VC_OP_ConstrainDistance@@@@
r5apex.exe!0x01f8a218 .?AV?$CParticleOperatorDefinition@VC_OP_ConstrainDistanceToPath@@@@
r5apex.exe!0x0232cab8 .?AV?$CParticleOperatorDefinition@VC_OP_ContinuousEmitter@@@@
r5apex.exe!0x02337f98 .?AV?$CParticleOperatorDefinition@VC_OP_ControlpointLight@@@@
r5apex.exe!0x0232ecb8 .?AV?$CParticleOperatorDefinition@VC_OP_Cull@@@@
r5apex.exe!0x02339ae8 .?AV?$CParticleOperatorDefinition@VC_OP_DampenToCP@@@@
r5apex.exe!0x0232e6c8 .?AV?$CParticleOperatorDefinition@VC_OP_Decay@@@@
r5apex.exe!0x023378d8 .?AV?$CParticleOperatorDefinition@VC_OP_DecayMaintainCount@@@@
r5apex.exe!0x02338418 .?AV?$CParticleOperatorDefinition@VC_OP_DifferencePreviousParticle@@@@
r5apex.exe!0x02335768 .?AV?$CParticleOperatorDefinition@VC_OP_DistanceBetweenCPs@@@@
r5apex.exe!0x023340a8 .?AV?$CParticleOperatorDefinition@VC_OP_DistanceBetweenCPsToCP@@@@
r5apex.exe!0x02338e08 .?AV?$CParticleOperatorDefinition@VC_OP_DistanceCull@@@@
r5apex.exe!0x0232c6d8 .?AV?$CParticleOperatorDefinition@VC_OP_DistanceEmitter@@@@
r5apex.exe!0x02332d48 .?AV?$CParticleOperatorDefinition@VC_OP_DistanceToCP@@@@
r5apex.exe!0x0232e7c8 .?AV?$CParticleOperatorDefinition@VC_OP_FadeAndKill@@@@
r5apex.exe!0x02338258 .?AV?$CParticleOperatorDefinition@VC_OP_FadeAndKillForTracers@@@@
r5apex.exe!0x02332748 .?AV?$CParticleOperatorDefinition@VC_OP_FadeIn@@@@
r5apex.exe!0x02330998 .?AV?$CParticleOperatorDefinition@VC_OP_FadeInSimple@@@@
r5apex.exe!0x02335668 .?AV?$CParticleOperatorDefinition@VC_OP_FadeOut@@@@
r5apex.exe!0x02336048 .?AV?$CParticleOperatorDefinition@VC_OP_FadeOutSimple@@@@
r5apex.exe!0x0232cd78 .?AV?$CParticleOperatorDefinition@VC_OP_ForceBasedOnDistanceToPlane@@@@
r5apex.exe!0x02336388 .?AV?$CParticleOperatorDefinition@VC_OP_GraphScalar@@@@
r5apex.exe!0x02338898 .?AV?$CParticleOperatorDefinition@VC_OP_GraphVector@@@@
r5apex.exe!0x0232f818 .?AV?$CParticleOperatorDefinition@VC_OP_InheritFromParentParticles@@@@
r5apex.exe!0x0232cb38 .?AV?$CParticleOperatorDefinition@VC_OP_InstantaneousDistanceEmitter@@@@
r5apex.exe!0x0232c7f8 .?AV?$CParticleOperatorDefinition@VC_OP_InstantaneousEmitter@@@@
r5apex.exe!0x02338e88 .?AV?$CParticleOperatorDefinition@VC_OP_InterpolateRadius@@@@
r5apex.exe!0x0232ea88 .?AV?$CParticleOperatorDefinition@VC_OP_LagCompensation@@@@
r5apex.exe!0x0232df28 .?AV?$CParticleOperatorDefinition@VC_OP_LerpEndCapScalar@@@@
r5apex.exe!0x023357e8 .?AV?$CParticleOperatorDefinition@VC_OP_LerpEndCapVector@@@@
r5apex.exe!0x02330b38 .?AV?$CParticleOperatorDefinition@VC_OP_LerpScalar@@@@
r5apex.exe!0x023356e8 .?AV?$CParticleOperatorDefinition@VC_OP_LerpVector@@@@
r5apex.exe!0x02331b08 .?AV?$CParticleOperatorDefinition@VC_OP_LockToBone@@@@
r5apex.exe!0x02335988 .?AV?$CParticleOperatorDefinition@VC_OP_LockToSavedSequentialPath@@@@
r5apex.exe!0x0232c9b8 .?AV?$CParticleOperatorDefinition@VC_OP_MaintainEmitter@@@@
r5apex.exe!0x023392a8 .?AV?$CParticleOperatorDefinition@VC_OP_MaintainSequentialPath@@@@
r5apex.exe!0x023355e8 .?AV?$CParticleOperatorDefinition@VC_OP_MaxVelocity@@@@
r5apex.exe!0x02331b58 .?AV?$CParticleOperatorDefinition@VC_OP_ModelCull@@@@
r5apex.exe!0x023381d8 .?AV?$CParticleOperatorDefinition@VC_OP_MoveToHitbox@@@@
r5apex.exe!0x0232ebc8 .?AV?$CParticleOperatorDefinition@VC_OP_MovementMaintainOffset@@@@
r5apex.exe!0x02338658 .?AV?$CParticleOperatorDefinition@VC_OP_MovementPlaceOnGround@@@@
r5apex.exe!0x02337ae8 .?AV?$CParticleOperatorDefinition@VC_OP_MovementRotateParticleAroundAxis@@@@
r5apex.exe!0x02332588 .?AV?$CParticleOperatorDefinition@VC_OP_Noise@@@@
r5apex.exe!0x0232ca38 .?AV?$CParticleOperatorDefinition@VC_OP_NoiseEmitter@@@@
r5apex.exe!0x02337858 .?AV?$CParticleOperatorDefinition@VC_OP_NormalLock@@@@
r5apex.exe!0x02333bf8 .?AV?$CParticleOperatorDefinition@VC_OP_NormalizeVector@@@@
r5apex.exe!0x023385d8 .?AV?$CParticleOperatorDefinition@VC_OP_Orient2DRelToCP@@@@
r5apex.exe!0x02338778 .?AV?$CParticleOperatorDefinition@VC_OP_OrientTo2dDirection@@@@
r5apex.exe!0x02335b28 .?AV?$CParticleOperatorDefinition@VC_OP_OrientTowardPlayer@@@@
r5apex.exe!0x0232f798 .?AV?$CParticleOperatorDefinition@VC_OP_OscillateScalar@@@@
r5apex.exe!0x0232eff8 .?AV?$CParticleOperatorDefinition@VC_OP_OscillateScalarSimple@@@@
r5apex.exe!0x02338af8 .?AV?$CParticleOperatorDefinition@VC_OP_OscillateVector@@@@
r5apex.exe!0x0232e8e8 .?AV?$CParticleOperatorDefinition@VC_OP_OscillateVectorSimple@@@@
r5apex.exe!0x0232d5a8 .?AV?$CParticleOperatorDefinition@VC_OP_ParentVortices@@@@
r5apex.exe!0x02336308 .?AV?$CParticleOperatorDefinition@VC_OP_PercentageBetweenCPs@@@@
r5apex.exe!0x0232fb78 .?AV?$CParticleOperatorDefinition@VC_OP_PercentageBetweenCPsVector@@@@
r5apex.exe!0x01f85b48 .?AV?$CParticleOperatorDefinition@VC_OP_PlanarConstraint@@@@
r5apex.exe!0x02336168 .?AV?$CParticleOperatorDefinition@VC_OP_PlaneCull@@@@
r5apex.exe!0x023353a8 .?AV?$CParticleOperatorDefinition@VC_OP_PositionBetweenCPs@@@@
r5apex.exe!0x0232e0e8 .?AV?$CParticleOperatorDefinition@VC_OP_PositionLock@@@@
r5apex.exe!0x02338158 .?AV?$CParticleOperatorDefinition@VC_OP_ProjectileArc@@@@
r5apex.exe!0x0232eef8 .?AV?$CParticleOperatorDefinition@VC_OP_RadiusDecay@@@@
r5apex.exe!0x023370e8 .?AV?$CParticleOperatorDefinition@VC_OP_RampScalarLinear@@@@
r5apex.exe!0x02338d58 .?AV?$CParticleOperatorDefinition@VC_OP_RampScalarLinearSimple@@@@
r5apex.exe!0x0232e748 .?AV?$CParticleOperatorDefinition@VC_OP_RampScalarSpline@@@@
r5apex.exe!0x02330c88 .?AV?$CParticleOperatorDefinition@VC_OP_RampScalarSplineSimple@@@@
r5apex.exe!0x0232d708 .?AV?$CParticleOperatorDefinition@VC_OP_RandomForce@@@@
r5apex.exe!0x023387f8 .?AV?$CParticleOperatorDefinition@VC_OP_RemapAverageScalarValuetoCP@@@@
r5apex.exe!0x0232dea8 .?AV?$CParticleOperatorDefinition@VC_OP_RemapBoundingVolumetoCP@@@@
r5apex.exe!0x023318a8 .?AV?$CParticleOperatorDefinition@VC_OP_RemapCPVelocityToVector@@@@
r5apex.exe!0x023390e8 .?AV?$CParticleOperatorDefinition@VC_OP_RemapCPtoScalar@@@@
r5apex.exe!0x02334128 .?AV?$CParticleOperatorDefinition@VC_OP_RemapCPtoVector@@@@
r5apex.exe!0x02335428 .?AV?$CParticleOperatorDefinition@VC_OP_RemapControlPointDirectionToVector@@@@
r5apex.exe!0x02335e08 .?AV?$CParticleOperatorDefinition@VC_OP_RemapDirectionToCPToVector@@@@
r5apex.exe!0x0232f678 .?AV?$CParticleOperatorDefinition@VC_OP_RemapDotProductToScalar@@@@
r5apex.exe!0x023364a8 .?AV?$CParticleOperatorDefinition@VC_OP_RemapModelVolumetoCP@@@@
r5apex.exe!0x02336288 .?AV?$CParticleOperatorDefinition@VC_OP_RemapScalar@@@@
r5apex.exe!0x02332e68 .?AV?$CParticleOperatorDefinition@VC_OP_RemapSpeed@@@@
r5apex.exe!0x02335868 .?AV?$CParticleOperatorDefinition@VC_OP_RemapSpeedtoCP@@@@
r5apex.exe!0x0232e168 .?AV?$CParticleOperatorDefinition@VC_OP_RemapVelocityToVector@@@@
r5apex.exe!0x02330a18 .?AV?$CParticleOperatorDefinition@VC_OP_RemapWorldCPToScreen@@@@
r5apex.exe!0x0233a168 .?AV?$CParticleOperatorDefinition@VC_OP_RenderDecal@@@@
r5apex.exe!0x0233a828 .?AV?$CParticleOperatorDefinition@VC_OP_RenderLightSource@@@@
r5apex.exe!0x0233a428 .?AV?$CParticleOperatorDefinition@VC_OP_RenderModels@@@@
r5apex.exe!0x01336270 .?AV?$CParticleOperatorDefinition@VC_OP_RenderPoints@@@@
r5apex.exe!0x0233aa68 .?AV?$CParticleOperatorDefinition@VC_OP_RenderRope@@@@
r5apex.exe!0x0233a7a8 .?AV?$CParticleOperatorDefinition@VC_OP_RenderScreenVelocityRotate@@@@
r5apex.exe!0x0233a1e8 .?AV?$CParticleOperatorDefinition@VC_OP_RenderScripts@@@@
r5apex.exe!0x0233a8a8 .?AV?$CParticleOperatorDefinition@VC_OP_RenderSprites@@@@
r5apex.exe!0x0233a4a8 .?AV?$CParticleOperatorDefinition@VC_OP_RenderSpritesTrail@@@@
r5apex.exe!0x02335fc8 .?AV?$CParticleOperatorDefinition@VC_OP_RestartAfterDuration@@@@
r5apex.exe!0x0232ef78 .?AV?$CParticleOperatorDefinition@VC_OP_RotateVector@@@@
r5apex.exe!0x023341a8 .?AV?$CParticleOperatorDefinition@VC_OP_SetCPOrientationToDirection@@@@
r5apex.exe!0x0232fc98 .?AV?$CParticleOperatorDefinition@VC_OP_SetChildControlPoints@@@@
r5apex.exe!0x02331cf8 .?AV?$CParticleOperatorDefinition@VC_OP_SetControlPointPositions@@@@
r5apex.exe!0x02339328 .?AV?$CParticleOperatorDefinition@VC_OP_SetControlPointRotation@@@@
r5apex.exe!0x02336ae8 .?AV?$CParticleOperatorDefinition@VC_OP_SetControlPointToCenter@@@@
r5apex.exe!0x0232f558 .?AV?$CParticleOperatorDefinition@VC_OP_SetControlPointToImpactPoint@@@@
r5apex.exe!0x02330e28 .?AV?$CParticleOperatorDefinition@VC_OP_SetControlPointToPlayer@@@@
r5apex.exe!0x023333c8 .?AV?$CParticleOperatorDefinition@VC_OP_SetControlPointsToParticle@@@@
r5apex.exe!0x023301f8 .?AV?$CParticleOperatorDefinition@VC_OP_SetPerChildControlPoint@@@@
r5apex.exe!0x02337f18 .?AV?$CParticleOperatorDefinition@VC_OP_SoundMeterScalar@@@@
r5apex.exe!0x0232dd88 .?AV?$CParticleOperatorDefinition@VC_OP_Spin@@@@
r5apex.exe!0x02330d08 .?AV?$CParticleOperatorDefinition@VC_OP_SpinUpdate@@@@
r5apex.exe!0x02335c48 .?AV?$CParticleOperatorDefinition@VC_OP_SpinYaw@@@@
r5apex.exe!0x023399c8 .?AV?$CParticleOperatorDefinition@VC_OP_StopAfterCPDuration@@@@
r5apex.exe!0x0232d3e8 .?AV?$CParticleOperatorDefinition@VC_OP_TimeVaryingForce@@@@
r5apex.exe!0x0232cf08 .?AV?$CParticleOperatorDefinition@VC_OP_TurbulenceForce@@@@
r5apex.exe!0x0232d688 .?AV?$CParticleOperatorDefinition@VC_OP_TwistAroundAxis@@@@
r5apex.exe!0x0232f948 .?AV?$CParticleOperatorDefinition@VC_OP_VectorNoise@@@@
r5apex.exe!0x02335aa8 .?AV?$CParticleOperatorDefinition@VC_OP_VelocityDecay@@@@
r5apex.exe!0x02337b68 .?AV?$CParticleOperatorDefinition@VC_OP_VelocityMatchingForce@@@@
r5apex.exe!0x01f84908 .?AV?$CParticleOperatorDefinition@VC_OP_WorldCollideConstraint@@@@
r5apex.exe!0x01f8a758 .?AV?$CParticleOperatorDefinition@VC_OP_WorldTraceConstraint@@@@
r5apex.exe!0x298f19c0 .?AV?$CUtlVectorDataOps@V?$CUtlVector@HV?$CUtlMemory@H_J@@H@@$04@@
r5apex.exe!0x29907d58 .?AV?$CUtlVectorDataOps@V?$CUtlVector@PEAVCBaseEntity@@V?$CUtlMemory@PEAVCBaseEntity@@_J@@H@@$0M@@@
r5apex.exe!0x29900d30 .?AV?$CUtlVectorDataOps@V?$CUtlVector@PEAVCPlayer@@V?$CUtlMemory@PEAVCPlayer@@_J@@H@@$0M@@@
r5apex.exe!0x299030c0 .?AV?$CUtlVectorDataOps@V?$CUtlVector@PEAVCTeamSpawnPoint@@V?$CUtlMemory@PEAVCTeamSpawnPoint@@_J@@H@@$0M@@@
r5apex.exe!0x298f0c38 .?AV?$CUtlVectorDataOps@V?$CUtlVector@UAIChannelScheduleState_t@@V?$CUtlMemory@UAIChannelScheduleState_t@@_J@@H@@$09@@
r5apex.exe!0x298f2cd8 .?AV?$CUtlVectorDataOps@V?$CUtlVector@UAISquadEnemyInfo_t@@V?$CUtlMemory@UAISquadEnemyInfo_t@@_J@@H@@$09@@
r5apex.exe!0x298ec780 .?AV?$CUtlVectorDataOps@V?$CUtlVector@UUnreachableEnt_t@@V?$CUtlMemory@UUnreachableEnt_t@@_J@@H@@$09@@
r5apex.exe!0x298f5518 .?AV?$CUtlVectorDataOps@V?$CUtlVector@Uphysfollower_t@@V?$CUtlMemory@Uphysfollower_t@@_J@@H@@$09@@
r5apex.exe!0x299007b8 .?AV?$CUtlVectorDataOps@V?$CUtlVector@Utemplate_t@@V?$CUtlMemory@Utemplate_t@@_J@@H@@$09@@
r5apex.exe!0x28ea96c8 .?AV?$CUtlVectorDataOps@V?$CUtlVector@V?$CHandle@VCBaseEntity@@@@V?$CUtlMemory@V?$CHandle@VCBaseEntity@@@@_J@@H@@$0N@@@
r5apex.exe!0x298ffd88 .?AV?$CUtlVectorDataOps@V?$CUtlVector@V?$CHandle@VCPlayer@@@@V?$CUtlMemory@V?$CHandle@VCPlayer@@@@_J@@H@@$0N@@@
r5apex.exe!0x298fdea0 .?AV?$CUtlVectorDataOps@V?$CUtlVector@V?$CHandle@VCTonemapTrigger@@@@V?$CUtlMemory@V?$CHandle@VCTonemapTrigger@@@@_J@@H@@$0N@@@
r5apex.exe!0x2991ea18 .?AV?$CUtlVectorDataOps@V?$CUtlVector@VVector@@V?$CUtlMemory@VVector@@_J@@H@@$02@@
r5apex.exe!0x298ec768 .?AV?$CUtlVectorDataOps@V?$CUtlVectorFixed@UAIDebouncedSyncedMelee@@$04_J@@$09@@
r5apex.exe!0x298f0ed0 .?AV?$CUtlVectorDataOps@V?$CUtlVectorFixed@UWeaponAnimEvent@@$0EA@_J@@$09@@
r5apex.exe!0x298f4458 .?AV?$CUtlVectorDataOps@V?$CUtlVectorFixed@V?$CHandle@VCBaseEntity@@@@$02_J@@$0N@@@
r5apex.exe!0x298fa468 .?AV?$CUtlVectorDataOps@V?$CUtlVectorFixed@V?$CHandle@VCBaseEntity@@@@$0BA@_J@@$04@@
r5apex.exe!0x2991d878 .?AV?$CUtlVectorDataOps@V?$CUtlVectorFixedGrowable@Tfloat3@@$0BA@_J@@$02@@
r5apex.exe!0x298f0c28 .?AV?$CUtlVectorDataOps@VCAI_InterestTarget@@$09@@
r5apex.exe!0x298eaf50 .?AV?$CVarBitVecSaveRestoreOps@V?$CBitVec@$07@@@@
r5apex.exe!0x01f847b8 .?AV?$C_EntityClassList@VC_PointCamera@@@@
r5apex.exe!0x02330988 .?AV?$C_EntityClassList@VC_TriggerPlayerMovement@@@@
r5apex.exe!0x02900ae0 .?AV?$_Ref_count_obj_alloc@V__ExceptionPtr@@U?$_StaticAllocator@H@@@std@@
r5apex.exe!0x023e31a8 .?AVActiveActModifiersDataOps@@
r5apex.exe!0x023fc348 .?AVActiveActModifiersSaveRestoreDataOps@@
r5apex.exe!0x023dd878 .?AVCAI_EnemiesListSaveRestoreOps@@
r5apex.exe!0x023ef4c8 .?AVCAI_SaveRestoreBlockHandler@@
r5apex.exe!0x023e26d8 .?AVCAI_SystemHook@@
r5apex.exe!0x02374338 .?AVCActivityDataOps@@
r5apex.exe!0x01f92d90 .?AVCAimAssistTargets@@
r5apex.exe!0x01f92d98 .?AVCAimAssistTargets@@
r5apex.exe!0x0175d6f0 .?AVCAvi@@
r5apex.exe!0x01335630 .?AVCBSPPack@@
r5apex.exe!0x017533b8 .?AVCBaseClientRenderTargets@@
r5apex.exe!0x023f8bd8 .?AVCBaseEntityScriptInstanceHelper@@
r5apex.exe!0x1fc51f40 .?AVCBik@@
r5apex.exe!0x0174d858 .?AVCBoolProperty@@
r5apex.exe!0x027fec60 .?AVCBreakModelsPrecached@@
r5apex.exe!0x023f9b48 .?AVCBullseyeList@@
r5apex.exe!0x023f19a8 .?AVCCSMLightManager@@
r5apex.exe!0x0231aa40 .?AVCCascadeLightManager@@
r5apex.exe!0x023474f0 .?AVCCenterPrint@@
r5apex.exe!0x02809620 .?AVCCheckClient@@
r5apex.exe!0x028b00f0 .?AVCClassMap@@
r5apex.exe!0x0233fd40 .?AVCClientCollisionEvent@@
r5apex.exe!0x01752ec8 .?AVCClientDLLSharedAppSystems@@
r5apex.exe!0x01f97eb0 .?AVCClientEntityList@@
r5apex.exe!0x02217f08 .?AVCClientEntityList@@
r5apex.exe!0x235e2390 .?AVCClientLeafSystem@@
r5apex.exe!0x235b5a30 .?AVCClientShadowMgr@@
r5apex.exe!0x01747d28 .?AVCClientSound@@
r5apex.exe!0x018207e0 .?AVCClientState@@
r5apex.exe!0x018207e8 .?AVCClientState@@
r5apex.exe!0x018207f0 .?AVCClientState@@
r5apex.exe!0x018207f8 .?AVCClientState@@
r5apex.exe!0x02325330 .?AVCClientThinkList@@
r5apex.exe!0x017449a0 .?AVCCmdLibFileLoggingListener@@
r5apex.exe!0x017446d8 .?AVCCmdLibStandardLoggingListener@@
r5apex.exe!0x01f75bc0 .?AVCColorCorrectionMgr@@
r5apex.exe!0x018e42c0 .?AVCColorCorrectionSystem@@
r5apex.exe!0x024344a8 .?AVCColorCorrectionSystem_Server@@
r5apex.exe!0x0174cf58 .?AVCColorProperty@@
r5apex.exe!0x01f6b070 .?AVCCommandLine@@
r5apex.exe!0x22ab14e0 .?AVCCountedStringPool@@
r5apex.exe!0x22ab1530 .?AVCCountedStringPool@@
r5apex.exe!0x01f72750 .?AVCCvar@@
r5apex.exe!0x01747e00 .?AVCCvarQuery@@
r5apex.exe!0x017466c0 .?AVCDataCache@@
r5apex.exe!0x023dc230 .?AVCDataObjectAccessSystem@@
r5apex.exe!0x0233c650 .?AVCDebugOverlayPanel@@
r5apex.exe!0x01748958 .?AVCDebugTextureInfoDX11@@
r5apex.exe!0x0174c758 .?AVCDefaultAccessor@@
r5apex.exe!0x0174edc8 .?AVCDefaultCvarQuery@@
r5apex.exe!0x023f8ce8 .?AVCDefaultParticleSystemQuery@@
r5apex.exe!0x0233c078 .?AVCDirtySpatialPartitionEntityList@@
r5apex.exe!0x01f903e0 .?AVCEffectsList@@
r5apex.exe!0x01f6ddd0 .?AVCEmptyConVar@@
r5apex.exe!0x01f6de10 .?AVCEmptyConVar@@
r5apex.exe!0x02383540 .?AVCEmptyGameUIConVar@@
r5apex.exe!0x02383580 .?AVCEmptyGameUIConVar@@
r5apex.exe!0x017583b0 .?AVCEngine@@
r5apex.exe!0x018df7f0 .?AVCEngineAPI@@
r5apex.exe!0x017447a8 .?AVCEngineClient@@
r5apex.exe!0x017485d8 .?AVCEngineConsoleLoggingListener@@
r5apex.exe!0x0b3698c8 .?AVCEngineRecipientFilter@@
r5apex.exe!0x01744758 .?AVCEngineTraceClient@@
r5apex.exe!0x23ad7e78 .?AVCEngineTraceClient@@
r5apex.exe!0x23d4a3b8 .?AVCEngineTraceClient@@
r5apex.exe!0x23dacf68 .?AVCEngineTraceClient@@
r5apex.exe!0x01744738 .?AVCEngineTraceClientDecals@@
r5apex.exe!0x017447c8 .?AVCEngineTraceServer@@
r5apex.exe!0x01747968 .?AVCEngineUniformRandomStream@@
r5apex.exe!0x01757b20 .?AVCEngineVGui@@
r5apex.exe!0x023f9530 .?AVCEntFireAutoCompletionFunctor@@
r5apex.exe!0x023f9538 .?AVCEntFireAutoCompletionFunctor@@
r5apex.exe!0x028bfeb0 .?AVCEntityFactoryDictionary@@
r5apex.exe!0x023f2478 .?AVCEntityListSystem@@
r5apex.exe!0x018560a0 .?AVCEntityReadInfo@@
r5apex.exe!0x023dc900 .?AVCEntitySaveRestoreBlockHandler@@
r5apex.exe!0x023dc908 .?AVCEntitySaveUtils@@
r5apex.exe!0x023f3af8 .?AVCEntityTouchManager@@
r5apex.exe!0x01f728e0 .?AVCEventSystem@@
r5apex.exe!0x023dd118 .?AVCEventsSaveDataOps@@
r5apex.exe!0x02338de8 .?AVCExampleEffect@@
r5apex.exe!0x0233de00 .?AVCFPS@@
r5apex.exe!0x22ab12f0 .?AVCFileSystem_Stdio@@
r5apex.exe!0x22ab12f8 .?AVCFileSystem_Stdio@@
r5apex.exe!0x0174da48 .?AVCFloatProperty@@
r5apex.exe!0x02475b08 .?AVCFogSystem@@
r5apex.exe!0x01755378 .?AVCGameClientExports@@
r5apex.exe!0x023f4c48 .?AVCGameDLL_ConVarAccessor@@
r5apex.exe!0x023b9bf0 .?AVCGameMovement@@
r5apex.exe!0x023acd50 .?AVCGameRules@@
r5apex.exe!0x24a16800 .?AVCGameStringPool@@
r5apex.exe!0x023438c8 .?AVCGameTimescale@@
r5apex.exe!0x023833a0 .?AVCGameUI@@
r5apex.exe!0x01748288 .?AVCGameUIFuncs@@
r5apex.exe!0x02478010 .?AVCGlobalEntityList@@
r5apex.exe!0x0174dfb8 .?AVCHFontProperty@@
r5apex.exe!0x01f747f0 .?AVCHLClient@@
r5apex.exe!0x01859658 .?AVCHudTextMessage@@
r5apex.exe!0x01857878 .?AVCHudTextureHandleProperty@@
r5apex.exe!0x023ef958 .?AVCIKSaveRestoreOps@@
r5apex.exe!0x017469a0 .?AVCIVDebugOverlay@@
r5apex.exe!0x017469a8 .?AVCIVDebugOverlay@@
r5apex.exe!0x23491420 .?AVCIVPMaterialManager@@
r5apex.exe!0x023fc0f8 .?AVCInfoPlacementManager@@
r5apex.exe!0x023344c0 .?AVCInput@@
r5apex.exe!0x018e2640 .?AVCInputStackSystem@@
r5apex.exe!0x018e2780 .?AVCInputSystem@@
r5apex.exe!0x23478160 .?AVCInputWin32@@
r5apex.exe!0x0174d9d8 .?AVCIntProperty@@
r5apex.exe!0x01f94db0 .?AVCKeyBindingListenerMgr@@
r5apex.exe!0x01f740d0 .?AVCKeyValuesSystem@@
r5apex.exe!0x0174be88 .?AVCLauncherLoggingListener@@
r5apex.exe!0x0174c1e8 .?AVCListOps@TSListTests@@
r5apex.exe!0x02340ba0 .?AVCLoadingDisc@@
r5apex.exe!0x018e4120 .?AVCLocalize@@
r5apex.exe!0x01744a50 .?AVCMDLCache@@
r5apex.exe!0x22ab3fd8 .?AVCMatQueuedRenderContext@@
r5apex.exe!0x22ab3fe0 .?AVCMatQueuedRenderContext@@
r5apex.exe!0x22ab41a8 .?AVCMatRenderContext@@
r5apex.exe!0x22ab41b0 .?AVCMatRenderContext@@
r5apex.exe!0x23480fc0 .?AVCMatSystemSurface@@
r5apex.exe!0x23480fc8 .?AVCMatSystemSurface@@
r5apex.exe!0x23480fd0 .?AVCMatSystemSurface@@
r5apex.exe!0x028b0b70 .?AVCMaterialProxyDict@@
r5apex.exe!0x01744968 .?AVCMaterialProxyFactory@@
r5apex.exe!0x22ab3b40 .?AVCMaterialSystem@@
r5apex.exe!0x22ab3b48 .?AVCMaterialSystem@@
r5apex.exe!0x0233dbe0 .?AVCMessageChars@@
r5apex.exe!0x01335f48 .?AVCMessageListener@vgui@@
r5apex.exe!0x023f8488 .?AVCModInventoryDataOps@@
r5apex.exe!0x01744b88 .?AVCModelInfoClient@@
r5apex.exe!0x01746db8 .?AVCModelInfoServer@@
r5apex.exe!0x01751a10 .?AVCModelLoader@@
r5apex.exe!0x023fd430 .?AVCModelPrecacheSystem@@
r5apex.exe!0x01756e40 .?AVCModelRender@@
r5apex.exe!0x02347430 .?AVCModelRenderSystem@@
r5apex.exe!0x02347448 .?AVCModelRenderSystem@@
r5apex.exe!0x023443d0 .?AVCMoveHelperClient@@
r5apex.exe!0x02434590 .?AVCMoveHelperServer@@
r5apex.exe!0x017448e8 .?AVCNetworkStringTableContainer@@
r5apex.exe!0x017552a0 .?AVCNetworkStringTableContainer@@
r5apex.exe!0x02422a40 .?AVCNotifyList@@
r5apex.exe!0x02422a48 .?AVCNotifyList@@
r5apex.exe!0x028b1410 .?AVCPanelMetaClassMgrImp@@
r5apex.exe!0x028b19b0 .?AVCParticleMgr@@
r5apex.exe!0x02345588 .?AVCParticleSystemQuery@@
r5apex.exe!0x023e1e10 .?AVCPhysObjSaveRestoreOps@@
r5apex.exe!0x023e1e20 .?AVCPhysObjSaveRestoreOps@@
r5apex.exe!0x023e1e30 .?AVCPhysObjSaveRestoreOps@@
r5apex.exe!0x023e1e40 .?AVCPhysSaveRestoreBlockHandler@@
r5apex.exe!0x023e1e48 .?AVCPhysSaveRestoreBlockHandler@@
r5apex.exe!0x023e1e50 .?AVCPhysSaveRestoreBlockHandler@@
r5apex.exe!0x0174ec88 .?AVCPhysicsCollision@@
r5apex.exe!0x024238e0 .?AVCPhysicsHook@@
r5apex.exe!0x01f721c0 .?AVCPhysicsInterface@@
r5apex.exe!0x023f78a8 .?AVCPhysicsPlayerCallback@@
r5apex.exe!0x02423830 .?AVCPhysicsPushedEntities@@
r5apex.exe!0x23491370 .?AVCPhysicsSurfaceProps@@
r5apex.exe!0x018599b8 .?AVCPhysicsSystem@@
r5apex.exe!0x0234bce0 .?AVCPickupList@@
r5apex.exe!0x017546a8 .?AVCPixelVisibilitySystem@@
r5apex.exe!0x023f8f88 .?AVCPlayerMove@@
r5apex.exe!0x023f7d18 .?AVCPointTemplatePrecacher@@
r5apex.exe!0x01f6a7d0 .?AVCPolyhedron_TempMemory@@
r5apex.exe!0x02381690 .?AVCPoseDebuggerImpl@@
r5apex.exe!0x027ff890 .?AVCPostProcessSystem@@
r5apex.exe!0x023d7448 .?AVCPrecacheHandler@@
r5apex.exe!0x028034f0 .?AVCPrecacheOtherList@@
r5apex.exe!0x023dadf8 .?AVCPrecacheRegister@@
r5apex.exe!0x01747b08 .?AVCPrecacheSystem@@
r5apex.exe!0x02382d10 .?AVCPrediction@@
r5apex.exe!0x023f3cc8 .?AVCPrefDataOps@@
r5apex.exe!0x01f74260 .?AVCProcessUtils@@
r5apex.exe!0x023da1e0 .?AVCPropData@@
r5apex.exe!0x023db378 .?AVCPropSurvivalList@@
r5apex.exe!0x0174c9a8 .?AVCProportionalFloatProperty@@
r5apex.exe!0x0174e1d8 .?AVCProportionalIntProperty@@
r5apex.exe!0x0174e498 .?AVCProportionalXPosProperty@@
r5apex.exe!0x0174cee8 .?AVCProportionalYPosProperty@@
r5apex.exe!0x0174c038 .?AVCQueueOps@TSListTests@@
r5apex.exe!0x01754d00 .?AVCQueuedPacketSender@@
r5apex.exe!0x02381150 .?AVCRagdollLRURetirement@@
r5apex.exe!0x01748208 .?AVCRegistry@@
r5apex.exe!0x01f6a9d0 .?AVCResListGenerator@@
r5apex.exe!0x01855568 .?AVCResourcePrecacher@ErrorPrecache@@
r5apex.exe!0x0181f6d8 .?AVCResourcePrecacher@ExplodeImpactPrecache@@
r5apex.exe!0x018201e8 .?AVCResourcePrecacher@FX_SplashPrecache@@
r5apex.exe!0x01820168 .?AVCResourcePrecacher@FX_WaterRipplePrecache@@
r5apex.exe!0x0181f998 .?AVCResourcePrecacher@GameMovementImpactEventPrecache@@
r5apex.exe!0x0181fb38 .?AVCResourcePrecacher@ImpactPrecache@@
r5apex.exe!0x0181fd78 .?AVCResourcePrecacher@MissileAirBurstPrecache@@
r5apex.exe!0x0181fc58 .?AVCResourcePrecacher@MissileImpactPrecache@@
r5apex.exe!0x017569e8 .?AVCResourcePrecacher@ParticleCreatePrecache@@
r5apex.exe!0x017565c8 .?AVCResourcePrecacher@ParticleEffectPrecache@@
r5apex.exe!0x01756828 .?AVCResourcePrecacher@ParticleEffectStopPrecache@@
r5apex.exe!0x0181ffa8 .?AVCResourcePrecacher@ParticleTracerPrecache@@
r5apex.exe!0x023db3e8 .?AVCResourcePrecacher@PhysFrictionEffectPrecache@@
r5apex.exe!0x01756b08 .?AVCResourcePrecacher@PlayParticlesBreakEffectPrecache@@
r5apex.exe!0x01f83c58 .?AVCResourcePrecacher@PlayWeaponParticleEffectPrecache@@
r5apex.exe!0x01756228 .?AVCResourcePrecacher@PrecacheEffectBuildPrecache@@
r5apex.exe!0x01f81438 .?AVCResourcePrecacher@PrecacheEffectCrossbowPrecache@@
r5apex.exe!0x0181f7f8 .?AVCResourcePrecacher@PrecacheEffectGlassShatterPrecache@@
r5apex.exe!0x0181f918 .?AVCResourcePrecacher@PrecacheEffectVGuiScreenPrecache@@
r5apex.exe!0x01855fe8 .?AVCResourcePrecacher@PrecacheLocatorTargetPrecache@@
r5apex.exe!0x0181fab8 .?AVCResourcePrecacher@RagdollImpactPrecache@@
r5apex.exe!0x0181f5b8 .?AVCResourcePrecacher@ShakeRopesPrecache@@
r5apex.exe!0x01820388 .?AVCResourcePrecacher@SplashImpactPrecache@@
r5apex.exe!0x01f83ff8 .?AVCResourcePrecacher@StopWeaponParticleEffectPrecache@@
r5apex.exe!0x023fd478 .?AVCResourcePrecacher@WeaponResourcesPrecache@@
r5apex.exe!0x023fc4f8 .?AVCResourcePrecacher@grapple_hookPrecache@@
r5apex.exe!0x023f7e58 .?AVCResourcePrecacher@playerPrecache@@
r5apex.exe!0x023f80b8 .?AVCResourcePrecacher@vgui_screenPrecache@@
r5apex.exe!0x01820538 .?AVCResourcePrecacher@waterripplePrecache@@
r5apex.exe!0x01820308 .?AVCResourcePrecacher@watersplashPrecache@@
r5apex.exe!0x0175b668 .?AVCRopeInitializer@@
r5apex.exe!0x0232d5c0 .?AVCRopeManager@@
r5apex.exe!0x01f670b8 .?AVCRunGameEngine@@
r5apex.exe!0x023e38a0 .?AVCSaveRestoreBlockSet@@
r5apex.exe!0x01746e28 .?AVCSaveRestoreFileSystemPassthrough@@
r5apex.exe!0x01f6e720 .?AVCSchemeManager@@
r5apex.exe!0x018589c8 .?AVCScreenSpaceEffectManager@@
r5apex.exe!0x017446f8 .?AVCScriptLib@@
r5apex.exe!0x1fc56e80 .?AVCServer@@
r5apex.exe!0x024770e0 .?AVCServerCollisionEvent@@
r5apex.exe!0x023f5f28 .?AVCServerDLLSharedAppSystems@@
r5apex.exe!0x023f37d8 .?AVCServerGameClients@@
r5apex.exe!0x023feaf0 .?AVCServerGameDLL@@
r5apex.exe!0x023f56b8 .?AVCServerGameEnts@@
r5apex.exe!0x02475e70 .?AVCServerRandomStream@@
r5apex.exe!0x01747b98 .?AVCServerSound@@
r5apex.exe!0x017489c8 .?AVCShader@Basic@@
r5apex.exe!0x01748f18 .?AVCShader@BasicForceWireframe@Basic@@
r5apex.exe!0x01749398 .?AVCShader@Bik@@
r5apex.exe!0x017493e8 .?AVCShader@Black@@
r5apex.exe!0x01f67c80 .?AVCShader@BlurFilter@@
r5apex.exe!0x01749998 .?AVCShader@BoxFilterCompute@@
r5apex.exe!0x01749b58 .?AVCShader@Cable@@
r5apex.exe!0x01749bd8 .?AVCShader@DebugDrawEnvmapMask@@
r5apex.exe!0x01749c58 .?AVCShader@DecalModulate@@
r5apex.exe!0x01749e98 .?AVCShader@DepthWrite@@
r5apex.exe!0x01749fb8 .?AVCShader@DoFBlurFilterCompute@@
r5apex.exe!0x0174a038 .?AVCShader@Downsample4x4@@
r5apex.exe!0x0174a2f8 .?AVCShader@Downsample@@
r5apex.exe!0x0174a1d8 .?AVCShader@Downsample_bloom@@
r5apex.exe!0x0174a418 .?AVCShader@Edge@@
r5apex.exe!0x0174a538 .?AVCShader@Engine_Post@@
r5apex.exe!0x0174a5b8 .?AVCShader@ExposureAdaptation@@
r5apex.exe!0x0174a638 .?AVCShader@FrameColorCompute@@
r5apex.exe!0x0174a6b8 .?AVCShader@Logluminance@@
r5apex.exe!0x0174a738 .?AVCShader@Modulate@@
r5apex.exe!0x0174a8f8 .?AVCShader@Occlusion@@
r5apex.exe!0x0174aa18 .?AVCShader@Refract@@
r5apex.exe!0x0174ab18 .?AVCShader@Sky@@
r5apex.exe!0x0174ae18 .?AVCShader@SplineRope@@
r5apex.exe!0x0174ae98 .?AVCShader@Sprite@@
r5apex.exe!0x0174b0f8 .?AVCShader@Spritecard@@
r5apex.exe!0x0174b858 .?AVCShader@TSAA@@
r5apex.exe!0x0174b978 .?AVCShader@UnlitTwoTexture@@
r5apex.exe!0x0174b9f8 .?AVCShader@VisQuery@@
r5apex.exe!0x0174bc48 .?AVCShader@Water@@
r5apex.exe!0x0174be08 .?AVCShader@WriteZ@@
r5apex.exe!0x0174aa98 .?AVCShader@screenspace_general@@
r5apex.exe!0x01335b40 .?AVCShaderLibConVarAccessor@@
r5apex.exe!0x019ea440 .?AVCShaderSystem@@
r5apex.exe!0x019ea448 .?AVCShaderSystem@@
r5apex.exe!0x024012b0 .?AVCSimThinkManager@@
r5apex.exe!0x017441b8 .?AVCSimpleLoggingListener@@
r5apex.exe!0x298557b8 .?AVCSimpleLoggingListener@@
r5apex.exe!0x017441c8 .?AVCSimpleWindowsLoggingListener@@
r5apex.exe!0x023dc348 .?AVCSolidSetDefaults@@
r5apex.exe!0x0280ff10 .?AVCSoundscapeSystem@@
r5apex.exe!0x018207d0 .?AVCSplitScreen@@
r5apex.exe!0x0175b680 .?AVCStaticPropMgr@@
r5apex.exe!0x0175b688 .?AVCStaticPropMgr@@
r5apex.exe!0x274c9bd0 .?AVCStdMemAlloc@@
r5apex.exe!0x0174d968 .?AVCStringProperty@@
r5apex.exe!0x02435af0 .?AVCStringTableSaveRestoreOps@@
r5apex.exe!0x01f6af30 .?AVCStudioRenderContext@@
r5apex.exe!0x0174ea58 .?AVCSurfaceDragDropTarget@@
r5apex.exe!0x01f6e780 .?AVCSystem@@
r5apex.exe!0x24fbdeb0 .?AVCTEBeamEntPoint@@
r5apex.exe!0x24ee7420 .?AVCTEBeamEnts@@
r5apex.exe!0x24fbe600 .?AVCTEBeamFollow@@
r5apex.exe!0x24fbdc40 .?AVCTEBeamLaser@@
r5apex.exe!0x24ee7a70 .?AVCTEBeamPoints@@
r5apex.exe!0x24fbdd10 .?AVCTEBeamRing@@
r5apex.exe!0x24ee7a00 .?AVCTEBeamRingPoint@@
r5apex.exe!0x0280fc60 .?AVCTEBeamSpline@@
r5apex.exe!0x02803490 .?AVCTEBreakModel@@
r5apex.exe!0x02810850 .?AVCTEEffectDispatch@@
r5apex.exe!0x027ff1e0 .?AVCTEExplosion@@
r5apex.exe!0x023f22a8 .?AVCTEGibEvent@@
r5apex.exe!0x027f8490 .?AVCTEPhysicsProp@@
r5apex.exe!0x02822170 .?AVCTEProjectileTrail@@
r5apex.exe!0x023b7160 .?AVCTEScriptParticleSystem@@
r5apex.exe!0x0239b590 .?AVCTEScriptParticleSystemOnEntity@@
r5apex.exe!0x0239c220 .?AVCTEScriptParticleSystemOnEntityWithPos@@
r5apex.exe!0x028126b0 .?AVCTEShatterSurface@@
r5apex.exe!0x023f74a8 .?AVCTESoundDispatch@@
r5apex.exe!0x0232a850 .?AVCTempEnts@@
r5apex.exe!0x0280b8f0 .?AVCTempEntsSystem@@
r5apex.exe!0x0280fa48 .?AVCTemplate_SaveRestoreBlockHandler@@
r5apex.exe!0x023f7808 .?AVCTemplatesHook@@
r5apex.exe!0x01f6eaf0 .?AVCTextureDictionary@@
r5apex.exe!0x0174e028 .?AVCTextureIdProperty@@
r5apex.exe!0x023f0018 .?AVCThinkContextsSaveDataOps@@
r5apex.exe!0x024239b8 .?AVCTonemapSystem@@
r5apex.exe!0x23adc670 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23adcd30 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23b0acb0 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23b4a110 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23b4aa10 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23b684d0 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23b79990 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23b79e10 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23b942f0 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23bc9310 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23bd4df0 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23bde010 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23bfa210 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23c0b910 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23c2d2d0 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23c2f910 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23c3aad0 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23c72a50 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23c72c90 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23c72ed0 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23c76950 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23c77fd0 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23c8ea10 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23c8f0d0 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23ca4d90 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23cab990 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23ce3010 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23cef190 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23cfd050 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23d3fd10 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23d41c90 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23d91cd0 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23d9d550 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23d9de50 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23dd54d0 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23ddb110 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23de2610 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23dfd610 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23e22c90 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23e30250 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23e53b90 .?AVCTraceFilterHitAll@@
r5apex.exe!0x23e5b090 .?AVCTraceFilterHitAll@@
r5apex.exe!0x023b52e0 .?AVCTraceFilterSkipTwoEntities@@
r5apex.exe!0x023b5310 .?AVCTraceFilterSkipTwoEntities@@
r5apex.exe!0x023b5340 .?AVCTraceFilterSkipTwoEntities@@
r5apex.exe!0x023b5370 .?AVCTraceFilterSkipTwoEntities@@
r5apex.exe!0x023b53a0 .?AVCTraceFilterSkipTwoEntities@@
r5apex.exe!0x023b53d0 .?AVCTraceFilterSkipTwoEntities@@
r5apex.exe!0x023b5400 .?AVCTraceFilterSkipTwoEntities@@
r5apex.exe!0x023b5430 .?AVCTraceFilterSkipTwoEntities@@
r5apex.exe!0x02824080 .?AVCTurretList@@
r5apex.exe!0x01746828 .?AVCUniformRandomStream@@
r5apex.exe!0x0174eeb8 .?AVCUniformRandomStream@@
r5apex.exe!0x01857c08 .?AVCUniformRandomStream@@
r5apex.exe!0x023f0128 .?AVCUniformRandomStream@@
r5apex.exe!0x02475e78 .?AVCUniformRandomStream@@
r5apex.exe!0x01f6ba90 .?AVCUtlCStringConversion@@
r5apex.exe!0x01f6ccb0 .?AVCUtlNoEscConversion@@
r5apex.exe!0x017473e8 .?AVCVEfx@@
r5apex.exe!0x01748198 .?AVCVEngineServer@@
r5apex.exe!0x23477fd0 .?AVCVGui@@
r5apex.exe!0x01747078 .?AVCVRenderView@@
r5apex.exe!0x02342e40 .?AVCVScriptGameSystem@@
r5apex.exe!0x02809610 .?AVCVScriptPostEntitySaveRestoreBlockHandler@@
r5apex.exe!0x028108f0 .?AVCVScriptPreEntitySaveRestoreBlockHandler@@
r5apex.exe!0x0280fa28 .?AVCVScriptServerGameSystem@@
r5apex.exe!0x023ee7b8 .?AVCVariantSaveDataOps@@
r5apex.exe!0x0233ed00 .?AVCViewEffects@@
r5apex.exe!0x02346db0 .?AVCViewEffects@@
r5apex.exe!0x2381b800 .?AVCViewRender@@
r5apex.exe!0x02341fc0 .?AVCViewRenderBeams@@
r5apex.exe!0x01858df8 .?AVCViewportClientSystem@@
r5apex.exe!0x023ee7a0 .?AVCVoiceGameMgr@@
r5apex.exe!0x017479b8 .?AVCVoiceServer@@
r5apex.exe!0x02824030 .?AVCWeaponXList@@
r5apex.exe!0x23477dd0 .?AVCWin32Surface@@
r5apex.exe!0x23477dd8 .?AVCWin32Surface@@
r5apex.exe!0x23477de0 .?AVCWin32Surface@@
r5apex.exe!0x01751778 .?AVC_BaseAnimatingGameSystem@@
r5apex.exe!0x02376680 .?AVC_DataObjectAccessSystem@@
r5apex.exe!0x0233afc8 .?AVC_DefaultParticleSystemQuery@@
r5apex.exe!0x018da7e8 .?AVC_DirtySpatialPartitionEntityList@@
r5apex.exe!0x0234b420 .?AVC_GameMovement@@
r5apex.exe!0x0235c2f0 .?AVC_GameRules@@
r5apex.exe!0x23a5d3a0 .?AVC_GameStringPool@@
r5apex.exe!0x018dc2a8 .?AVC_GameTimescale@@
r5apex.exe!0x0236ef50 .?AVC_ParticleSystemQuery@@
r5apex.exe!0x018e99c8 .?AVC_PrecacheHandler@@
r5apex.exe!0x018dfb28 .?AVC_PrecacheRegister@@
r5apex.exe!0x0237e790 .?AVC_PropData@@
r5apex.exe!0x018e5268 .?AVC_PropSurvivalList@@
r5apex.exe!0x02333df0 .?AVC_SoundscapeSystem@@
r5apex.exe!0x02338810 .?AVC_TEBeamEntPoint@@
r5apex.exe!0x02338820 .?AVC_TEBeamEntPoint@@
r5apex.exe!0x0232fa00 .?AVC_TEBeamEnts@@
r5apex.exe!0x0232fa10 .?AVC_TEBeamEnts@@
r5apex.exe!0x02333b20 .?AVC_TEBeamFollow@@
r5apex.exe!0x02333b30 .?AVC_TEBeamFollow@@
r5apex.exe!0x0232f890 .?AVC_TEBeamLaser@@
r5apex.exe!0x0232f8a0 .?AVC_TEBeamLaser@@
r5apex.exe!0x0233a340 .?AVC_TEBeamPoints@@
r5apex.exe!0x0233a350 .?AVC_TEBeamPoints@@
r5apex.exe!0x02330210 .?AVC_TEBeamRing@@
r5apex.exe!0x02330220 .?AVC_TEBeamRing@@
r5apex.exe!0x02337100 .?AVC_TEBeamRingPoint@@
r5apex.exe!0x02337110 .?AVC_TEBeamRingPoint@@
r5apex.exe!0x023378f0 .?AVC_TEBeamSpline@@
r5apex.exe!0x02337900 .?AVC_TEBeamSpline@@
r5apex.exe!0x0232ebe0 .?AVC_TEBreakModel@@
r5apex.exe!0x0232ebf0 .?AVC_TEBreakModel@@
r5apex.exe!0x0232a970 .?AVC_TEEffectDispatch@@
r5apex.exe!0x0232a980 .?AVC_TEEffectDispatch@@
r5apex.exe!0x0232a8d0 .?AVC_TEExplosion@@
r5apex.exe!0x0232a8e0 .?AVC_TEExplosion@@
r5apex.exe!0x01757f18 .?AVC_TEGibEvent@@
r5apex.exe!0x01757f28 .?AVC_TEGibEvent@@
r5apex.exe!0x023258b0 .?AVC_TEPhysicsProp@@
r5apex.exe!0x023258c0 .?AVC_TEPhysicsProp@@
r5apex.exe!0x0238ac20 .?AVC_TEProjectileTrail@@
r5apex.exe!0x0238ac30 .?AVC_TEProjectileTrail@@
r5apex.exe!0x0236dc00 .?AVC_TEScriptParticleSystem@@
r5apex.exe!0x0236dc10 .?AVC_TEScriptParticleSystem@@
r5apex.exe!0x0234aff0 .?AVC_TEScriptParticleSystemOnEntity@@
r5apex.exe!0x0234b000 .?AVC_TEScriptParticleSystemOnEntity@@
r5apex.exe!0x02370c00 .?AVC_TEScriptParticleSystemOnEntityWithPos@@
r5apex.exe!0x02370c10 .?AVC_TEScriptParticleSystemOnEntityWithPos@@
r5apex.exe!0x02332510 .?AVC_TEShatterSurface@@
r5apex.exe!0x02332520 .?AVC_TEShatterSurface@@
r5apex.exe!0x0232f900 .?AVC_TESoundDispatch@@
r5apex.exe!0x0232f910 .?AVC_TESoundDispatch@@
r5apex.exe!0x0181e5f8 .?AVC_TempEntsSystem@@
r5apex.exe!0x23ad5370 .?AVC_TraceFilterGroundSurfaceForCharacter@@
r5apex.exe!0x23ad55b0 .?AVC_TraceFilterGroundSurfaceForCharacter@@
r5apex.exe!0x23b0c5a0 .?AVC_TraceFilterGroundSurfaceForCharacter@@
r5apex.exe!0x23dffa30 .?AVC_TraceFilterGroundSurfaceForCharacter@@
r5apex.exe!0x23dffc70 .?AVC_TraceFilterGroundSurfaceForCharacter@@
r5apex.exe!0x0235c380 .?AVC_TraceFilterSkipTwoEntities@@
r5apex.exe!0x0235c3b0 .?AVC_TraceFilterSkipTwoEntities@@
r5apex.exe!0x0235c3e0 .?AVC_TraceFilterSkipTwoEntities@@
r5apex.exe!0x0235c410 .?AVC_TraceFilterSkipTwoEntities@@
r5apex.exe!0x0235c440 .?AVC_TraceFilterSkipTwoEntities@@
r5apex.exe!0x0235c470 .?AVC_TraceFilterSkipTwoEntities@@
r5apex.exe!0x0235c4a0 .?AVC_TraceFilterSkipTwoEntities@@
r5apex.exe!0x0235c4d0 .?AVC_TraceFilterSkipTwoEntities@@
r5apex.exe!0x023917e0 .?AVC_TurretList@@
r5apex.exe!0x02393aa0 .?AVC_WeaponXList@@
r5apex.exe!0x01838f40 .?AVClientDataBlockReceiver@@
r5apex.exe!0x01f8d820 .?AVClientModeFullscreen@@
r5apex.exe!0x01302a50 .?AVDNameStatusNode@@
r5apex.exe!0x01302a60 .?AVDNameStatusNode@@
r5apex.exe!0x01302a70 .?AVDNameStatusNode@@
r5apex.exe!0x01302a80 .?AVDNameStatusNode@@
r5apex.exe!0x01728850 .?AVDenuvoTrialV2@@
r5apex.exe!0x028263d0 .?AVDroppedWeaponManager@@
r5apex.exe!0x023e24e8 .?AVHSCRIPTSaveRestoreOps@@
r5apex.exe!0x023ef458 .?AVHSQOBJECTSaveRestoreOps@@
r5apex.exe!0x01f67040 .?AVHardwareConfigDX11@@
r5apex.exe!0x023f79b8 .?AVIPredictionSystem@@
r5apex.exe!0x0181e868 .?AVIPredictionSystem_Client@@
r5apex.exe!0x01f70070 .?AVIVP_BetterDebugmanager@@
r5apex.exe!0x018e1c60 .?AVImeTextStore@@
r5apex.exe!0x018e1c68 .?AVImeTextStore@@
r5apex.exe!0x018e1c70 .?AVImeTextStore@@
r5apex.exe!0x018e1c78 .?AVImeTextStore@@
r5apex.exe!0x018e1c80 .?AVImeTextStore@@
r5apex.exe!0x018e1c88 .?AVImeTextStore@@
r5apex.exe!0x018e1c90 .?AVImeTextStore@@
r5apex.exe!0x0181ee98 .?AVMapSettingsReseter@@
r5apex.exe!0x028af910 .?AVMonitorDefaultChanges@@
r5apex.exe!0x023f7c08 .?AVPilotClassActivityModifierSaveRestoreDataOps@@
r5apex.exe!0x01859d20 .?AVSVC_UserMessage@@
r5apex.exe!0x0174eb78 .?AVVPanelWrapper@@
r5apex.exe!0x02900b90 .?AVbad_alloc@std@@
r5apex.exe!0x1ec204d8 .?AVstl_critical_section_win7@details@Concurrency@@
r5apex.exe!0x1ec21538 .?AVstl_critical_section_win7@details@Concurrency@@
r5apex.exe!0x023fc238 .?AVweaponScriptCB_HSCRIPTSaveRestoreOps@@
```

