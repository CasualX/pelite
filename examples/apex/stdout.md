## Interfaces

```
r5apex.exe!0x017126a8 ClientRenderTargets001
r5apex.exe!0x01704248 EngineTraceClient004
r5apex.exe!0x017025a8 EngineTraceClientDecals004
r5apex.exe!0x01edc490 EventSystem001
r5apex.exe!0x022f5760 GameUI011
r5apex.exe!0x017069d8 ISoundC002
r5apex.exe!0x01ed86c8 RunGameEngine005
r5apex.exe!0x0236a1b8 ServerGameClients004
r5apex.exe!0x02371c60 ServerGameDLL005
r5apex.exe!0x0236c598 ServerGameEnts002
r5apex.exe!0x01953f48 ShaderSystem002
r5apex.exe!0x01ede3a0 VClient018
r5apex.exe!0x02181b58 VClientEntityList003
r5apex.exe!0x022f50d0 VClientPrediction001
r5apex.exe!0x01705860 VCvarQuery001
r5apex.exe!0x01704390 VDebugOverlay004
r5apex.exe!0x01707438 VENGINE_GAMEUIFUNCS_VERSION005
r5apex.exe!0x018483d0 VENGINE_LAUNCHER_API_VERSION004
r5apex.exe!0x017148f0 VEngineModel016
r5apex.exe!0x01705dd8 VEngineRandom001
r5apex.exe!0x01705bf8 VEngineRenderView013
r5apex.exe!0x01ed8340 VGUI_System010
r5apex.exe!0x22709378 VMaterialSystemConfig004
r5apex.exe!0x01704398 VPhysicsDebugOverlay001
r5apex.exe!0x0236c608 VServerDllSharedAppSystems001
```

## Miscellaneous

```
TimeDateStamp = 0x5d4ba818
CheckSum = 0x29c890d
GameVersion = "v3.0.2.428"
NUM_ENT_ENTRIES = 0x10000
r5apex.exe!0x1f01b08 cl_entitylist
r5apex.exe!0x1702d0c LocalEntityHandle
r5apex.exe!0x17ddf50 GlobalVars
r5apex.exe!0xc57bce0 PlayerResource
r5apex.exe!0xc57ae90 ViewRender
```

## Buttons

These are addresses to global instances of the [`kbutton_t`](https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/game/client/kbutton.h#L14-L20) struct.

```
r5apex.exe!0x0c57c670 kbutton_t in_attack
r5apex.exe!0x0c57c620 kbutton_t in_backward
r5apex.exe!0x2723a020 kbutton_t in_break
r5apex.exe!0x276143b0 kbutton_t in_camin
r5apex.exe!0x2723a350 kbutton_t in_camout
r5apex.exe!0x2723a300 kbutton_t in_campitchdown
r5apex.exe!0x27614440 kbutton_t in_campitchup
r5apex.exe!0x2723a390 kbutton_t in_camyawleft
r5apex.exe!0x2723a330 kbutton_t in_camyawright
r5apex.exe!0x0c57c648 kbutton_t in_commandermousemove
r5apex.exe!0x0c57c728 kbutton_t in_dodge
r5apex.exe!0x2723a040 kbutton_t in_duck
r5apex.exe!0x0c57c5f8 kbutton_t in_forward
r5apex.exe!0x0c57c6a0 kbutton_t in_graph
r5apex.exe!0x0c57c708 kbutton_t in_jump
r5apex.exe!0x27614418 kbutton_t in_klook
r5apex.exe!0x2723a378 kbutton_t in_left
r5apex.exe!0x2723a320 kbutton_t in_lookdown
r5apex.exe!0x2723a030 kbutton_t in_lookup
r5apex.exe!0x27614428 kbutton_t in_melee
r5apex.exe!0x2723a3a0 kbutton_t in_movedown
r5apex.exe!0x0c57c5e8 kbutton_t in_moveleft
r5apex.exe!0x0c57c610 kbutton_t in_moveright
r5apex.exe!0x276143f8 kbutton_t in_moveup
r5apex.exe!0x27614408 kbutton_t in_offhand0
r5apex.exe!0x2723a368 kbutton_t in_offhand1
r5apex.exe!0x2723a130 kbutton_t in_offhand2
r5apex.exe!0x2723a110 kbutton_t in_offhand3
r5apex.exe!0x2723a0d0 kbutton_t in_offhand4
r5apex.exe!0x0c57c6f8 kbutton_t in_pause_menu
r5apex.exe!0x0c57c660 kbutton_t in_ping
r5apex.exe!0x0c57c6b0 kbutton_t in_reload
r5apex.exe!0x2723a340 kbutton_t in_right
r5apex.exe!0x2723a140 kbutton_t in_score
r5apex.exe!0x2723a140 kbutton_t in_showscores
r5apex.exe!0x0c57c630 kbutton_t in_speed
r5apex.exe!0x0c57c6c0 kbutton_t in_strafe
r5apex.exe!0x276143d0 kbutton_t in_toggle_duck
r5apex.exe!0x276143c0 kbutton_t in_toggle_zoom
r5apex.exe!0x2723a120 kbutton_t in_use
r5apex.exe!0x2723a0c0 kbutton_t in_useAndReload
r5apex.exe!0x276143e0 kbutton_t in_use_alt
r5apex.exe!0x2723a0b0 kbutton_t in_use_long
r5apex.exe!0x0c57c6e0 kbutton_t in_variableScopeToggle
r5apex.exe!0x2723a0f0 kbutton_t in_walk
r5apex.exe!0x2723a310 kbutton_t in_weaponCycle
r5apex.exe!0x2723a0e0 kbutton_t in_weapon_discard
r5apex.exe!0x2723a100 kbutton_t in_zoom
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
sizeof: `2752`  
</details>
<details>
<summary><code>client_class CBaseAnimating</code></summary>

class_id: `2`  
sizeof: `4928`  
</details>
<details>
<summary><code>client_class CBaseAnimatingOverlay</code></summary>

class_id: `3`  
sizeof: `5824`  
</details>
<details>
<summary><code>client_class CBaseButton</code></summary>

class_id: `0`  
sizeof: `2752`  
</details>
<details>
<summary><code>client_class CBaseCombatCharacter</code></summary>

class_id: `4`  
sizeof: `6272`  
</details>
<details>
<summary><code>client_class CBaseEntity</code></summary>

class_id: `5`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CBaseGrenade</code></summary>

class_id: `6`  
sizeof: `10368`  
</details>
<details>
<summary><code>client_class CBaseParticleEntity</code></summary>

class_id: `0`  
sizeof: `2944`  
</details>
<details>
<summary><code>client_class CBaseTempEntity</code></summary>

class_id: `7`  
sizeof: `40`  
</details>
<details>
<summary><code>client_class CBaseToggle</code></summary>

class_id: `8`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CBaseTrigger</code></summary>

class_id: `9`  
sizeof: `2752`  
</details>
<details>
<summary><code>client_class CBaseVPhysicsTrigger</code></summary>

class_id: `11`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CBaseViewModel</code></summary>

class_id: `10`  
sizeof: `19584`  
</details>
<details>
<summary><code>client_class CBeam</code></summary>

class_id: `12`  
sizeof: `2816`  
</details>
<details>
<summary><code>client_class CBoneFollower</code></summary>

class_id: `13`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CBreakableProp</code></summary>

class_id: `14`  
sizeof: `4992`  
</details>
<details>
<summary><code>client_class CBreakableSurface</code></summary>

class_id: `15`  
sizeof: `3904`  
</details>
<details>
<summary><code>client_class CCascadeLight</code></summary>

class_id: `16`  
sizeof: `3008`  
</details>
<details>
<summary><code>client_class CColorCorrection</code></summary>

class_id: `17`  
sizeof: `3008`  
</details>
<details>
<summary><code>client_class CCrossbowBolt</code></summary>

class_id: `18`  
sizeof: `10240`  
</details>
<details>
<summary><code>client_class CDeathBoxProp</code></summary>

class_id: `19`  
sizeof: `5120`  
</details>
<details>
<summary><code>client_class CDynamicLight</code></summary>

class_id: `20`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CDynamicProp</code></summary>

class_id: `21`  
sizeof: `5056`  
</details>
<details>
<summary><code>client_class CDynamicPropLightweight</code></summary>

class_id: `22`  
sizeof: `5056`  
</details>
<details>
<summary><code>client_class CEntityBlocker</code></summary>

class_id: `23`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CEntityDissolve</code></summary>

class_id: `24`  
sizeof: `2752`  
</details>
<details>
<summary><code>client_class CEntityLinkPage</code></summary>

class_id: `25`  
sizeof: `4736`  
</details>
<details>
<summary><code>client_class CEnvTonemapController</code></summary>

class_id: `26`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CEnvWind</code></summary>

class_id: `27`  
sizeof: `3008`  
</details>
<details>
<summary><code>client_class CFirstPersonProxy</code></summary>

class_id: `28`  
sizeof: `5056`  
</details>
<details>
<summary><code>client_class CFogController</code></summary>

class_id: `29`  
sizeof: `2752`  
</details>
<details>
<summary><code>client_class CFuncBrush</code></summary>

class_id: `30`  
sizeof: `2752`  
</details>
<details>
<summary><code>client_class CFuncBrushLightweight</code></summary>

class_id: `31`  
sizeof: `2752`  
</details>
<details>
<summary><code>client_class CFuncMoveLinear</code></summary>

class_id: `32`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CGameRulesProxy</code></summary>

class_id: `33`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CGlobalNonRewinding</code></summary>

class_id: `34`  
sizeof: `3712`  
</details>
<details>
<summary><code>client_class CGrappleHook</code></summary>

class_id: `35`  
sizeof: `4992`  
</details>
<details>
<summary><code>client_class CHardPointEntity</code></summary>

class_id: `36`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CHardPointFrontierEntity</code></summary>

class_id: `37`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CHealthKit</code></summary>

class_id: `38`  
sizeof: `4928`  
</details>
<details>
<summary><code>client_class CImportantOnEntSound</code></summary>

class_id: `39`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CInfoPlacementHelper</code></summary>

class_id: `40`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CInfoTarget</code></summary>

class_id: `41`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CInfoTargetGravity</code></summary>

class_id: `42`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CInfoTargetMinimap</code></summary>

class_id: `43`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CMissile</code></summary>

class_id: `44`  
sizeof: `10432`  
</details>
<details>
<summary><code>client_class CMovieDisplay</code></summary>

class_id: `45`  
sizeof: `3008`  
</details>
<details>
<summary><code>client_class CNPC_Drone</code></summary>

class_id: `46`  
sizeof: `6784`  
</details>
<details>
<summary><code>client_class CNPC_Dropship</code></summary>

class_id: `47`  
sizeof: `6848`  
</details>
<details>
<summary><code>client_class CNPC_SentryTurret</code></summary>

class_id: `48`  
sizeof: `6784`  
</details>
<details>
<summary><code>client_class CNPC_Titan</code></summary>

class_id: `49`  
sizeof: `6912`  
</details>
<details>
<summary><code>client_class CParticleSystem</code></summary>

class_id: `50`  
sizeof: `2816`  
</details>
<details>
<summary><code>client_class CPhysBox</code></summary>

class_id: `51`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CPhysicsProp</code></summary>

class_id: `52`  
sizeof: `5120`  
</details>
<details>
<summary><code>client_class CPlayer</code></summary>

class_id: `53`  
sizeof: `17344`  
</details>
<details>
<summary><code>client_class CPlayerDecoy</code></summary>

class_id: `54`  
sizeof: `5056`  
</details>
<details>
<summary><code>client_class CPlayerResource</code></summary>

class_id: `55`  
sizeof: `12480`  
</details>
<details>
<summary><code>client_class CPlayerTasklist</code></summary>

class_id: `56`  
sizeof: `4032`  
</details>
<details>
<summary><code>client_class CPlayerVehicle</code></summary>

class_id: `57`  
sizeof: `4992`  
</details>
<details>
<summary><code>client_class CPlayerWaypoint</code></summary>

class_id: `58`  
sizeof: `3392`  
</details>
<details>
<summary><code>client_class CPointCamera</code></summary>

class_id: `59`  
sizeof: `2816`  
</details>
<details>
<summary><code>client_class CPortal_PointPush</code></summary>

class_id: `60`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CPostProcessController</code></summary>

class_id: `61`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CPredictedFirstPersonProxy</code></summary>

class_id: `62`  
sizeof: `5120`  
</details>
<details>
<summary><code>client_class CProjectile</code></summary>

class_id: `63`  
sizeof: `10176`  
</details>
<details>
<summary><code>client_class CPropDoor</code></summary>

class_id: `64`  
sizeof: `5248`  
</details>
<details>
<summary><code>client_class CPropSurvival</code></summary>

class_id: `65`  
sizeof: `4992`  
</details>
<details>
<summary><code>client_class CRopeKeyframe</code></summary>

class_id: `66`  
sizeof: `3904`  
</details>
<details>
<summary><code>client_class CScriptMover</code></summary>

class_id: `67`  
sizeof: `5504`  
</details>
<details>
<summary><code>client_class CScriptMoverTrainNode</code></summary>

class_id: `68`  
sizeof: `4224`  
</details>
<details>
<summary><code>client_class CScriptNetData</code></summary>

class_id: `69`  
sizeof: `3200`  
</details>
<details>
<summary><code>client_class CScriptNetDataGlobal</code></summary>

class_id: `75`  
sizeof: `3456`  
</details>
<details>
<summary><code>client_class CScriptNetData_SNDC_DEATH_BOX</code></summary>

class_id: `70`  
sizeof: `3264`  
</details>
<details>
<summary><code>client_class CScriptNetData_SNDC_GLOBAL</code></summary>

class_id: `71`  
sizeof: `3456`  
</details>
<details>
<summary><code>client_class CScriptNetData_SNDC_PLAYER_EXCLUSIVE</code></summary>

class_id: `72`  
sizeof: `3392`  
</details>
<details>
<summary><code>client_class CScriptNetData_SNDC_PLAYER_GLOBAL</code></summary>

class_id: `73`  
sizeof: `3392`  
</details>
<details>
<summary><code>client_class CScriptNetData_SNDC_TITAN_SOUL</code></summary>

class_id: `74`  
sizeof: `3328`  
</details>
<details>
<summary><code>client_class CScriptProp</code></summary>

class_id: `76`  
sizeof: `5248`  
</details>
<details>
<summary><code>client_class CScriptTraceVolume</code></summary>

class_id: `77`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CShieldProp</code></summary>

class_id: `78`  
sizeof: `5120`  
</details>
<details>
<summary><code>client_class CSkyCamera</code></summary>

class_id: `79`  
sizeof: `2624`  
</details>
<details>
<summary><code>client_class CSpotlightEnd</code></summary>

class_id: `80`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CSprite</code></summary>

class_id: `81`  
sizeof: `2752`  
</details>
<details>
<summary><code>client_class CSpriteOriented</code></summary>

class_id: `82`  
sizeof: `2752`  
</details>
<details>
<summary><code>client_class CStatueProp</code></summary>

class_id: `0`  
sizeof: `5184`  
</details>
<details>
<summary><code>client_class CStatusEffectPlugin</code></summary>

class_id: `83`  
sizeof: `2688`  
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
sizeof: `3008`  
</details>
<details>
<summary><code>client_class CTitanSoul</code></summary>

class_id: `106`  
sizeof: `3520`  
</details>
<details>
<summary><code>client_class CTriggerCylinderHeavy</code></summary>

class_id: `107`  
sizeof: `2880`  
</details>
<details>
<summary><code>client_class CTriggerNoGrapple</code></summary>

class_id: `108`  
sizeof: `2752`  
</details>
<details>
<summary><code>client_class CTriggerNoZipline</code></summary>

class_id: `109`  
sizeof: `2752`  
</details>
<details>
<summary><code>client_class CTriggerPlayerMovement</code></summary>

class_id: `110`  
sizeof: `2816`  
</details>
<details>
<summary><code>client_class CTriggerPointGravity</code></summary>

class_id: `111`  
sizeof: `2816`  
</details>
<details>
<summary><code>client_class CTriggerSlip</code></summary>

class_id: `112`  
sizeof: `2816`  
</details>
<details>
<summary><code>client_class CTurret</code></summary>

class_id: `113`  
sizeof: `6400`  
</details>
<details>
<summary><code>client_class CVGuiScreen</code></summary>

class_id: `114`  
sizeof: `2880`  
</details>
<details>
<summary><code>client_class CVortexSphere</code></summary>

class_id: `115`  
sizeof: `2752`  
</details>
<details>
<summary><code>client_class CWaterLODControl</code></summary>

class_id: `116`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class CWeaponX</code></summary>

class_id: `117`  
sizeof: `25472`  
</details>
<details>
<summary><code>client_class CWorld</code></summary>

class_id: `118`  
sizeof: `3008`  
</details>
<details>
<summary><code>client_class CZipline</code></summary>

class_id: `119`  
sizeof: `4224`  
</details>
<details>
<summary><code>client_class CZiplineEnd</code></summary>

class_id: `120`  
sizeof: `2688`  
</details>
<details>
<summary><code>client_class DoorMover</code></summary>

class_id: `121`  
sizeof: `5568`  
</details>
<details>
<summary><code>client_class ScriptMoverLightweight</code></summary>

class_id: `122`  
sizeof: `5568`  
</details>
<details>
<summary><code>client_class Titan_Cockpit</code></summary>

class_id: `0`  
sizeof: `5568`  
</details>

### Addresses

```
r5apex.exe!0x01717708 ClientClass CAI_BaseNPC
r5apex.exe!0x01712da8 ClientClass CAmbientGeneric
r5apex.exe!0x01716bc8 ClientClass CBaseAnimating
r5apex.exe!0x017135a8 ClientClass CBaseAnimatingOverlay
r5apex.exe!0x01717208 ClientClass CBaseButton
r5apex.exe!0x0170fff8 ClientClass CBaseCombatCharacter
r5apex.exe!0x01716c68 ClientClass CBaseEntity
r5apex.exe!0x01844ec8 ClientClass CBaseGrenade
r5apex.exe!0x01ed0e28 ClientClass CBaseParticleEntity
r5apex.exe!0x01716ee8 ClientClass CBaseTempEntity
r5apex.exe!0x01714298 ClientClass CBaseToggle
r5apex.exe!0x017dcc68 ClientClass CBaseTrigger
r5apex.exe!0x01717b68 ClientClass CBaseVPhysicsTrigger
r5apex.exe!0x01845f08 ClientClass CBaseViewModel
r5apex.exe!0x018453c8 ClientClass CBeam
r5apex.exe!0x0170e388 ClientClass CBoneFollower
r5apex.exe!0x0170f938 ClientClass CBreakableProp
r5apex.exe!0x01711de8 ClientClass CBreakableSurface
r5apex.exe!0x0170f798 ClientClass CCascadeLight
r5apex.exe!0x017148b8 ClientClass CColorCorrection
r5apex.exe!0x01eeafe8 ClientClass CCrossbowBolt
r5apex.exe!0x0170e2e8 ClientClass CDeathBoxProp
r5apex.exe!0x01714478 ClientClass CDynamicLight
r5apex.exe!0x017169e8 ClientClass CDynamicProp
r5apex.exe!0x0170e748 ClientClass CDynamicPropLightweight
r5apex.exe!0x01848398 ClientClass CEntityBlocker
r5apex.exe!0x01712f88 ClientClass CEntityDissolve
r5apex.exe!0x017dc948 ClientClass CEntityLinkPage
r5apex.exe!0x01711098 ClientClass CEnvTonemapController
r5apex.exe!0x0170f1e8 ClientClass CEnvWind
r5apex.exe!0x01edf638 ClientClass CFirstPersonProxy
r5apex.exe!0x01714818 ClientClass CFogController
r5apex.exe!0x01713148 ClientClass CFuncBrush
r5apex.exe!0x01711638 ClientClass CFuncBrushLightweight
r5apex.exe!0x01710dd8 ClientClass CFuncMoveLinear
r5apex.exe!0x01848c68 ClientClass CGameRulesProxy
r5apex.exe!0x01847c08 ClientClass CGlobalNonRewinding
r5apex.exe!0x01eed828 ClientClass CGrappleHook
r5apex.exe!0x01719038 ClientClass CHardPointEntity
r5apex.exe!0x0171b398 ClientClass CHardPointFrontierEntity
r5apex.exe!0x01ed5638 ClientClass CHealthKit
r5apex.exe!0x017dd308 ClientClass CImportantOnEntSound
r5apex.exe!0x01ed95a8 ClientClass CInfoPlacementHelper
r5apex.exe!0x01848708 ClientClass CInfoTarget
r5apex.exe!0x01844e28 ClientClass CInfoTargetGravity
r5apex.exe!0x01847528 ClientClass CInfoTargetMinimap
r5apex.exe!0x01ede908 ClientClass CMissile
r5apex.exe!0x0170fbb8 ClientClass CMovieDisplay
r5apex.exe!0x01710c98 ClientClass CNPC_Drone
r5apex.exe!0x01710978 ClientClass CNPC_Dropship
r5apex.exe!0x0170e7e8 ClientClass CNPC_SentryTurret
r5apex.exe!0x01712d38 ClientClass CNPC_Titan
r5apex.exe!0x01712e48 ClientClass CParticleSystem
r5apex.exe!0x017121a8 ClientClass CPhysBox
r5apex.exe!0x01713be8 ClientClass CPhysicsProp
r5apex.exe!0x0170ec48 ClientClass CPlayer
r5apex.exe!0x01ed0d88 ClientClass CPlayerDecoy
r5apex.exe!0x017138c8 ClientClass CPlayerResource
r5apex.exe!0x0184ed38 ClientClass CPlayerTasklist
r5apex.exe!0x017dc648 ClientClass CPlayerVehicle
r5apex.exe!0x0184e6f8 ClientClass CPlayerWaypoint
r5apex.exe!0x01712ab8 ClientClass CPointCamera
r5apex.exe!0x01edc458 ClientClass CPortal_PointPush
r5apex.exe!0x0170e928 ClientClass CPostProcessController
r5apex.exe!0x01edf4f8 ClientClass CPredictedFirstPersonProxy
r5apex.exe!0x01ede868 ClientClass CProjectile
r5apex.exe!0x01845a08 ClientClass CPropDoor
r5apex.exe!0x01711778 ClientClass CPropSurvival
r5apex.exe!0x01849998 ClientClass CRopeKeyframe
r5apex.exe!0x01ed0b98 ClientClass CScriptMover
r5apex.exe!0x018498f8 ClientClass CScriptMoverTrainNode
r5apex.exe!0x01953f58 ClientClass CScriptNetData
r5apex.exe!0x0184a148 ClientClass CScriptNetDataGlobal
r5apex.exe!0x01848db8 ClientClass CScriptNetData_SNDC_DEATH_BOX
r5apex.exe!0x0184e8d8 ClientClass CScriptNetData_SNDC_GLOBAL
r5apex.exe!0x0184bfd8 ClientClass CScriptNetData_SNDC_PLAYER_EXCLUSIVE
r5apex.exe!0x0184a278 ClientClass CScriptNetData_SNDC_PLAYER_GLOBAL
r5apex.exe!0x0184e978 ClientClass CScriptNetData_SNDC_TITAN_SOUL
r5apex.exe!0x01711278 ClientClass CScriptProp
r5apex.exe!0x01edde78 ClientClass CScriptTraceVolume
r5apex.exe!0x0170e6a8 ClientClass CShieldProp
r5apex.exe!0x01714018 ClientClass CSkyCamera
r5apex.exe!0x017182e8 ClientClass CSpotlightEnd
r5apex.exe!0x01849f98 ClientClass CSprite
r5apex.exe!0x018539c8 ClientClass CSpriteOriented
r5apex.exe!0x017146d8 ClientClass CStatueProp
r5apex.exe!0x018534c8 ClientClass CStatusEffectPlugin
r5apex.exe!0x01719678 ClientClass CTEBaseBeam
r5apex.exe!0x017dc818 ClientClass CTEBeamEntPoint
r5apex.exe!0x0171b438 ClientClass CTEBeamEnts
r5apex.exe!0x01817408 ClientClass CTEBeamFollow
r5apex.exe!0x01815618 ClientClass CTEBeamLaser
r5apex.exe!0x01817d88 ClientClass CTEBeamPoints
r5apex.exe!0x017dd3a8 ClientClass CTEBeamRing
r5apex.exe!0x017dc068 ClientClass CTEBeamRingPoint
r5apex.exe!0x017dca88 ClientClass CTEBeamSpline
r5apex.exe!0x017dc6e8 ClientClass CTEBreakModel
r5apex.exe!0x017188c8 ClientClass CTEEffectDispatch
r5apex.exe!0x01718828 ClientClass CTEExplosion
r5apex.exe!0x01713788 ClientClass CTEGibEvent
r5apex.exe!0x017dcec8 ClientClass CTEParticleSystem
r5apex.exe!0x0171b258 ClientClass CTEPhysicsProp
r5apex.exe!0x01ee7aa8 ClientClass CTEProjectileTrail
r5apex.exe!0x01847708 ClientClass CTEScriptParticleSystem
r5apex.exe!0x01847668 ClientClass CTEScriptParticleSystemOnEntity
r5apex.exe!0x018488e8 ClientClass CTEScriptParticleSystemOnEntityWithPos
r5apex.exe!0x017dcbc8 ClientClass CTEShatterSurface
r5apex.exe!0x01717fc8 ClientClass CTESoundDispatch
r5apex.exe!0x017dd1c8 ClientClass CTeam
r5apex.exe!0x017108d8 ClientClass CTitanSoul
r5apex.exe!0x0184a3b8 ClientClass CTriggerCylinderHeavy
r5apex.exe!0x017183c8 ClientClass CTriggerNoGrapple
r5apex.exe!0x017dcb28 ClientClass CTriggerNoZipline
r5apex.exe!0x01956e58 ClientClass CTriggerPlayerMovement
r5apex.exe!0x018522f8 ClientClass CTriggerPointGravity
r5apex.exe!0x0184dae8 ClientClass CTriggerSlip
r5apex.exe!0x01edfb48 ClientClass CTurret
r5apex.exe!0x017dd6a8 ClientClass CVGuiScreen
r5apex.exe!0x01eee688 ClientClass CVortexSphere
r5apex.exe!0x017dc228 ClientClass CWaterLODControl
r5apex.exe!0x01eef728 ClientClass CWeaponX
r5apex.exe!0x017dd568 ClientClass CWorld
r5apex.exe!0x01edf598 ClientClass CZipline
r5apex.exe!0x01edeff8 ClientClass CZiplineEnd
r5apex.exe!0x01849dc8 ClientClass DoorMover
r5apex.exe!0x01956768 ClientClass ScriptMoverLightweight
r5apex.exe!0x01edddd8 ClientClass Titan_Cockpit
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
DT_AI_BaseNPC!0x16f0 m_inventory
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
DT_AmbientGeneric!0x0a40 m_radius
DT_AmbientGeneric!0x0a44 m_isEnabled
DT_AmbientGeneric!0x0a50 m_networkTableSoundID
DT_AmbientGeneric!0x0a58 m_networkedSegmentEndpointWorldSpace
DT_AmbientGeneric!0x0a64 m_hasPolylineSegments
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
DT_BaseAnimating!0x075c m_passDamageToParent
DT_BaseAnimating!0x0a68 m_animNetworkFlags
DT_BaseAnimating!0x0a6c m_animActive
DT_BaseAnimating!0x0a6f m_animCollisionEnabled
DT_BaseAnimating!0x0a70 m_animPlantingEnabled
DT_BaseAnimating!0x0a74 m_animRelativeData
DT_BaseAnimating!0x0b64 m_syncingWithEntity
DT_BaseAnimating!0x0b68 m_predictedAnimEventData
DT_BaseAnimating!0x0bd4 m_nRagdollImpactFXTableId
DT_BaseAnimating!0x0bd8 m_flSkyScaleStartValue
DT_BaseAnimating!0x0bdc m_flSkyScaleEndValue
DT_BaseAnimating!0x0be0 m_flSkyScaleStartTime
DT_BaseAnimating!0x0be4 m_flSkyScaleEndTime
DT_BaseAnimating!0x0c00 m_SequenceTransitioner
DT_BaseAnimating!0x0e88 m_nSkin
DT_BaseAnimating!0x0e8c m_nBody
DT_BaseAnimating!0x0e90 m_camoIndex
DT_BaseAnimating!0x0ec8 m_nForceBone
DT_BaseAnimating!0x0f38 m_bSequenceFinished
DT_BaseAnimating!0x0f3c m_lockedAnimDeltaYaw
DT_BaseAnimating!0x0f44 m_flModelScale
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
DT_BaseCombatCharacter!0x0034 m_vecViewOffset.x
DT_BaseCombatCharacter!0x0038 m_vecViewOffset.y
DT_BaseCombatCharacter!0x003c m_vecViewOffset.z
DT_BaseCombatCharacter!0x018c m_cloakEndTime
DT_BaseCombatCharacter!0x0190 m_cloakFadeInEndTime
DT_BaseCombatCharacter!0x0194 m_cloakFadeOutStartTime
DT_BaseCombatCharacter!0x0198 m_cloakFadeInDuration
DT_BaseCombatCharacter!0x019c m_cloakFlickerAmount
DT_BaseCombatCharacter!0x01a0 m_cloakFlickerEndTime
DT_BaseCombatCharacter!0x0394 m_networkedFlags
DT_BaseCombatCharacter!0x0410 m_deathVelocity
DT_BaseCombatCharacter!0x0918 m_minimapData
DT_BaseCombatCharacter!0x0968 m_nameVisibilityFlags
DT_BaseCombatCharacter!0x16c4 m_lastFiredTime
DT_BaseCombatCharacter!0x16c8 m_lastFiredWeapon
DT_BaseCombatCharacter!0x16cc m_raiseFromMeleeEndTime
DT_BaseCombatCharacter!0x16d0 m_sharedEnergyCount
DT_BaseCombatCharacter!0x16d4 m_sharedEnergyTotal
DT_BaseCombatCharacter!0x16d8 m_sharedEnergyLockoutThreshold
DT_BaseCombatCharacter!0x16dc m_lastSharedEnergyRegenTime
DT_BaseCombatCharacter!0x16e0 m_sharedEnergyRegenRate
DT_BaseCombatCharacter!0x16e4 m_sharedEnergyRegenDelay
DT_BaseCombatCharacter!0x16e8 m_lastSharedEnergyTakeTime
DT_BaseCombatCharacter!0x1740 m_selectedWeapons
DT_BaseCombatCharacter!0x1744 m_latestPrimaryWeapons
DT_BaseCombatCharacter!0x174c m_latestNonOffhandWeapons
DT_BaseCombatCharacter!0x1754 m_lastCycleSlot
DT_BaseCombatCharacter!0x175c m_weaponPermission
DT_BaseCombatCharacter!0x1760 m_weaponDelayEnableTime
DT_BaseCombatCharacter!0x1764 m_weaponDisabledInScript
DT_BaseCombatCharacter!0x1789 m_weaponDisabledFlags
DT_BaseCombatCharacter!0x178a m_hudInfo_visibilityTestAlwaysPasses
DT_BaseCombatCharacter!0x179c m_contextAction
DT_BaseCombatCharacter!0x17c8 m_phaseShiftTimeStart
DT_BaseCombatCharacter!0x17cc m_phaseShiftTimeEnd
DT_BaseCombatCharacter!0x181c m_targetInfoIconName
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
	m_fEffects: Int,
	m_usableType: Int,
	m_cellX: Int,
	m_clrRender: Int,
	m_cellY: Int,
	m_clIntensity: Int,
	m_cellZ: Int,
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
	m_parentAttachmentIndex: Int,
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
DT_BaseEntity!0x0040 m_fEffects
DT_BaseEntity!0x0044 m_usableType
DT_BaseEntity!0x0048 m_cellX
DT_BaseEntity!0x0048 m_clrRender
DT_BaseEntity!0x004c m_cellY
DT_BaseEntity!0x004c m_clIntensity
DT_BaseEntity!0x0050 m_cellZ
DT_BaseEntity!0x0054 m_localOrigin
DT_BaseEntity!0x0060 m_nModelIndex
DT_BaseEntity!0x0114 m_bossPlayer
DT_BaseEntity!0x0160 m_shieldHealth
DT_BaseEntity!0x0164 m_shieldHealthMax
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
DT_BaseEntity!0x07d0 m_attachmentLerpStartTime
DT_BaseEntity!0x07d4 m_attachmentLerpEndTime
DT_BaseEntity!0x07d8 m_attachmentLerpStartOrigin
DT_BaseEntity!0x07e4 m_attachmentLerpStartAngles
DT_BaseEntity!0x07f4 m_parentAttachmentIndex
DT_BaseEntity!0x07fc m_parentAttachmentModel
DT_BaseEntity!0x0808 m_fadeDist
DT_BaseEntity!0x08c8 m_dissolveEffectEntityHandle
DT_BaseEntity!0x08d8 m_usablePriority
DT_BaseEntity!0x08dc m_usableDistanceOverride
DT_BaseEntity!0x08e0 m_usableFOV
DT_BaseEntity!0x08e4 m_usePromptSize
DT_BaseEntity!0x08f8 m_spottedByTeams
DT_BaseEntity!0x09f0 m_firstChildEntityLink
DT_BaseEntity!0x09f4 m_firstParentEntityLink
DT_BaseEntity!0x09f8 m_realmsBitMask
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
DT_BaseGrenade!0x018c m_cloakEndTime
DT_BaseGrenade!0x0190 m_cloakFadeInEndTime
DT_BaseGrenade!0x0194 m_cloakFadeOutStartTime
DT_BaseGrenade!0x0198 m_cloakFadeInDuration
DT_BaseGrenade!0x019c m_cloakFlickerAmount
DT_BaseGrenade!0x01a0 m_cloakFlickerEndTime
DT_BaseGrenade!0x0754 m_baseTakeDamage
DT_BaseGrenade!0x0758 m_invulnerableToDamageCount
DT_BaseGrenade!0x07f4 m_parentAttachmentIndex
DT_BaseGrenade!0x07f8 m_parentAttachmentHitbox
DT_BaseGrenade!0x07fc m_parentAttachmentModel
DT_BaseGrenade!0x27c1 m_doesExplode
DT_BaseGrenade!0x27c4 m_DmgRadius
DT_BaseGrenade!0x2850 m_ziplineGrenadeExpectedEndPosition
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
DT_BaseViewModel!0x0040 m_fEffects
DT_BaseViewModel!0x0048 m_clrRender
DT_BaseViewModel!0x005c m_animOverlayPlaybackRate
DT_BaseViewModel!0x0060 m_nModelIndex
DT_BaseViewModel!0x0080 m_animOverlayModelIndex
DT_BaseViewModel!0x00a4 m_animOverlaySequence
DT_BaseViewModel!0x00c8 m_animOverlayWeight
DT_BaseViewModel!0x00ec m_animOverlayOrder
DT_BaseViewModel!0x0110 m_animOverlayAnimTime
DT_BaseViewModel!0x0134 m_animOverlayFadeInDuration
DT_BaseViewModel!0x0158 m_animOverlayFadeOutDuration
DT_BaseViewModel!0x0451 m_nRenderMode
DT_BaseViewModel!0x0e8c m_nBody
DT_BaseViewModel!0x0e98 m_nResetEventsParity
DT_BaseViewModel!0x0f38 m_bSequenceFinished
DT_BaseViewModel!0x0f44 m_flModelScale
DT_BaseViewModel!0x1451 m_overlayEventParity
DT_BaseViewModel!0x1718 m_viewModelOwner
DT_BaseViewModel!0x171c m_projectileIsVisible
DT_BaseViewModel!0x1b00 m_bBlockEventLayer
DT_BaseViewModel!0x1b01 m_isAdsTransition
DT_BaseViewModel!0x1b04 m_hWeapon
DT_BaseViewModel!0x1b08 m_tracerAttachments
DT_BaseViewModel!0x1b10 m_tracerAttachmentsScoped
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
DT_Beam!0x0060 m_nModelIndex
DT_Beam!0x03e8 m_visibilityFlags
DT_Beam!0x03f0 m_iTeamNum
DT_Beam!0x043c m_hOwnerEntity
DT_Beam!0x0441 m_nRenderFX
DT_Beam!0x0451 m_nRenderMode
DT_Beam!0x07f4 m_parentAttachmentIndex
DT_Beam!0x07f8 m_parentAttachmentHitbox
DT_Beam!0x07fc m_parentAttachmentModel
DT_Beam!0x0a40 m_flFrameRate
DT_Beam!0x0a44 m_flHDRColorScale
DT_Beam!0x0a48 m_clrRenderFriendly
DT_Beam!0x0a54 m_nNumBeamEnts
DT_Beam!0x0a5c m_nHaloIndex
DT_Beam!0x0a60 m_nBeamType
DT_Beam!0x0a64 m_nBeamFlags
DT_Beam!0x0a68 m_hAttachEntity
DT_Beam!0x0a90 m_nAttachIndex
DT_Beam!0x0ab8 m_fWidth
DT_Beam!0x0abc m_fEndWidth
DT_Beam!0x0ac0 m_fFadeLength
DT_Beam!0x0ac4 m_fHaloScale
DT_Beam!0x0ac8 m_fAmplitude
DT_Beam!0x0acc m_fStartFrame
DT_Beam!0x0ad0 m_fSpeed
DT_Beam!0x0ad4 m_flFrame
DT_Beam!0x0ad8 m_nClipStyle
DT_Beam!0x0adc m_vecEndPos
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
DT_BoneFollower!0x0048 m_cellX
DT_BoneFollower!0x004c m_cellY
DT_BoneFollower!0x0050 m_cellZ
DT_BoneFollower!0x0054 m_localOrigin
DT_BoneFollower!0x0060 m_nModelIndex
DT_BoneFollower!0x0394 m_networkedFlags
DT_BoneFollower!0x0428 m_localAngles
DT_BoneFollower!0x043c m_hOwnerEntity
DT_BoneFollower!0x0458 m_Collision
DT_BoneFollower!0x04d8 m_CollisionGroup
DT_BoneFollower!0x0a40 m_modelIndex
DT_BoneFollower!0x0a44 m_boneIndex
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
DT_CPropDoor!0x0040 m_fEffects
DT_CPropDoor!0x0044 m_usableType
DT_CPropDoor!0x0048 m_cellX
DT_CPropDoor!0x004c m_cellY
DT_CPropDoor!0x0050 m_cellZ
DT_CPropDoor!0x0054 m_localOrigin
DT_CPropDoor!0x0060 m_nModelIndex
DT_CPropDoor!0x0394 m_networkedFlags
DT_CPropDoor!0x0428 m_localAngles
DT_CPropDoor!0x13f0 m_closedAngle
DT_CPropDoor!0x13f4 m_angle
DT_CPropDoor!0x13f8 m_startAngle
DT_CPropDoor!0x13fc m_startAngleVel
DT_CPropDoor!0x1400 m_startMoveTime
DT_CPropDoor!0x1404 m_isLocked
DT_CPropDoor!0x1408 m_oppositeDoor
DT_CPropDoor!0x1458 m_interactingPlayer
DT_CPropDoor!0x145c m_interactingPlayerWantsOpen
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
DT_CascadeLight!0x0a40 m_shadowDirection
DT_CascadeLight!0x0a58 m_envLightShadowDirection
DT_CascadeLight!0x0a6c m_bEnabled
DT_CascadeLight!0x0a6d m_bEnableShadows
DT_CascadeLight!0x0a6f m_LightColor
DT_CascadeLight!0x0a73 m_cloudMaskName
DT_CascadeLight!0x0b78 m_cloudOffset
DT_CascadeLight!0x0b84 m_cloudScale
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
DT_ColorCorrection!0x0a40 m_localOrigin
DT_ColorCorrection!0x0a4c m_MinFalloff
DT_ColorCorrection!0x0a50 m_MaxFalloff
DT_ColorCorrection!0x0a54 m_flFadeInDuration
DT_ColorCorrection!0x0a58 m_flFadeOutDuration
DT_ColorCorrection!0x0a5c m_flMaxWeight
DT_ColorCorrection!0x0a60 m_flCurWeight
DT_ColorCorrection!0x0a64 m_netLookupFilename
DT_ColorCorrection!0x0b68 m_bEnabled
DT_ColorCorrection!0x0b69 m_bMaster
DT_ColorCorrection!0x0b6a m_bClientSide
DT_ColorCorrection!0x0b6b m_bExclusive
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
DT_DoorMover!0x0040 m_fEffects
DT_DoorMover!0x0044 m_usableType
DT_DoorMover!0x0048 m_cellX
DT_DoorMover!0x004c m_cellY
DT_DoorMover!0x0050 m_cellZ
DT_DoorMover!0x0054 m_localOrigin
DT_DoorMover!0x0060 m_nModelIndex
DT_DoorMover!0x0118 m_vecAngVelocity
DT_DoorMover!0x0394 m_networkedFlags
DT_DoorMover!0x041c m_vecVelocity
DT_DoorMover!0x0428 m_localAngles
DT_DoorMover!0x0458 m_Collision
DT_DoorMover!0x04d8 m_CollisionGroup
DT_DoorMover!0x0518 m_iSignifierName
DT_DoorMover!0x0628 m_scriptNameIndex
DT_DoorMover!0x06b0 m_holdUsePrompt
DT_DoorMover!0x06b8 m_pressUsePrompt
DT_DoorMover!0x07f4 m_parentAttachmentIndex
DT_DoorMover!0x07f8 m_parentAttachmentHitbox
DT_DoorMover!0x07fc m_parentAttachmentModel
DT_DoorMover!0x0808 m_fadeDist
DT_DoorMover!0x08d8 m_usablePriority
DT_DoorMover!0x08dc m_usableDistanceOverride
DT_DoorMover!0x08e0 m_usableFOV
DT_DoorMover!0x08e4 m_usePromptSize
DT_DoorMover!0x1580 m_doorFlags
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
DT_DynamicLight!0x0a40 m_Flags
DT_DynamicLight!0x0a41 m_LightStyle
DT_DynamicLight!0x0a44 m_Radius
DT_DynamicLight!0x0a48 m_Exponent
DT_DynamicLight!0x0a4c m_InnerAngle
DT_DynamicLight!0x0a50 m_OuterAngle
DT_DynamicLight!0x0a54 m_SpotRadius
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
DT_DynamicProp!0x1381 m_bUseHitboxesForRenderBox
DT_DynamicProp!0x1382 m_bAnimateInStaticShadow
DT_DynamicProp!0x1383 m_wantsScopeHighlight
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
DT_DynamicPropLightweight!0x0040 m_fEffects
DT_DynamicPropLightweight!0x0048 m_cellX
DT_DynamicPropLightweight!0x004c m_cellY
DT_DynamicPropLightweight!0x0050 m_cellZ
DT_DynamicPropLightweight!0x0054 m_localOrigin
DT_DynamicPropLightweight!0x0060 m_nModelIndex
DT_DynamicPropLightweight!0x0394 m_networkedFlags
DT_DynamicPropLightweight!0x03e8 m_visibilityFlags
DT_DynamicPropLightweight!0x0428 m_localAngles
DT_DynamicPropLightweight!0x0458 m_Collision
DT_DynamicPropLightweight!0x04d8 m_CollisionGroup
DT_DynamicPropLightweight!0x07f4 m_parentAttachmentIndex
DT_DynamicPropLightweight!0x07fc m_parentAttachmentModel
DT_DynamicPropLightweight!0x0808 m_fadeDist
DT_DynamicPropLightweight!0x0e88 m_nSkin
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
DT_EntityDissolve!0x0a48 m_flStartTime
DT_EntityDissolve!0x0a4c m_flFadeOutStart
DT_EntityDissolve!0x0a50 m_flFadeOutLength
DT_EntityDissolve!0x0a54 m_flFadeOutModelStart
DT_EntityDissolve!0x0a58 m_flFadeOutModelLength
DT_EntityDissolve!0x0a5c m_flFadeInStart
DT_EntityDissolve!0x0a60 m_flFadeInLength
DT_EntityDissolve!0x0a64 m_nDissolveType
DT_EntityDissolve!0x0a6c fadeColorR
DT_EntityDissolve!0x0a70 fadeColorG
DT_EntityDissolve!0x0a74 fadeColorB
DT_EntityDissolve!0x0a78 m_isLethal
DT_EntityDissolve!0x0a7c m_vDissolverOrigin
DT_EntityDissolve!0x0a88 m_nMagnitude
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
DT_EntityLinkPage!0x0a40 pageIndex
DT_EntityLinkPage!0x0a44 next
DT_EntityLinkPage!0x0e44 entity
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
DT_EnvTonemapController!0x0a40 m_bUseCustomAutoExposureMin
DT_EnvTonemapController!0x0a41 m_bUseCustomAutoExposureMax
DT_EnvTonemapController!0x0a42 m_bUseCustomAutoExposureRate
DT_EnvTonemapController!0x0a43 m_bUseCustomBloomScale
DT_EnvTonemapController!0x0a44 m_flCustomAutoExposureMin
DT_EnvTonemapController!0x0a48 m_flCustomAutoExposureMax
DT_EnvTonemapController!0x0a4c m_flCustomAutoExposureRate
DT_EnvTonemapController!0x0a50 m_flCustomBloomScale
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
DT_FogController!0x0a40 m_fog.botAlt
DT_FogController!0x0a44 m_fog.topAlt
DT_FogController!0x0a48 m_fog.halfDistBot
DT_FogController!0x0a4c m_fog.halfDistTop
DT_FogController!0x0a50 m_fog.distColorStr
DT_FogController!0x0a54 m_fog.dirColorStr
DT_FogController!0x0a58 m_fog.distOffset
DT_FogController!0x0a5c m_fog.densityScale
DT_FogController!0x0a60 m_fog.halfAngleDeg
DT_FogController!0x0a64 m_fog.HDRColorScale
DT_FogController!0x0a68 m_fog.distColor
DT_FogController!0x0a6c m_fog.dirColor
DT_FogController!0x0a70 m_fog.direction
DT_FogController!0x0a7c m_fog.minFadeTime
DT_FogController!0x0a80 m_fog.forceOntoSky
DT_FogController!0x0a81 m_fog.enable
DT_FogController!0x0a84 m_fog.id
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
DT_FuncBrushLightweight!0x0048 m_cellX
DT_FuncBrushLightweight!0x004c m_cellY
DT_FuncBrushLightweight!0x0050 m_cellZ
DT_FuncBrushLightweight!0x0054 m_localOrigin
DT_FuncBrushLightweight!0x0060 m_nModelIndex
DT_FuncBrushLightweight!0x0394 m_networkedFlags
DT_FuncBrushLightweight!0x03e8 m_visibilityFlags
DT_FuncBrushLightweight!0x0428 m_localAngles
DT_FuncBrushLightweight!0x0458 m_Collision
DT_FuncBrushLightweight!0x04d8 m_CollisionGroup
DT_FuncBrushLightweight!0x07f4 m_parentAttachmentIndex
DT_FuncBrushLightweight!0x07f8 m_parentAttachmentHitbox
DT_FuncBrushLightweight!0x07fc m_parentAttachmentModel
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
DT_GrappleHook!0x0048 m_cellX
DT_GrappleHook!0x004c m_cellY
DT_GrappleHook!0x0050 m_cellZ
DT_GrappleHook!0x0054 m_localOrigin
DT_GrappleHook!0x0060 m_nModelIndex
DT_GrappleHook!0x03e8 m_visibilityFlags
DT_GrappleHook!0x0428 m_localAngles
DT_GrappleHook!0x043c m_hOwnerEntity
DT_GrappleHook!0x07f4 m_parentAttachmentIndex
DT_GrappleHook!0x07f8 m_parentAttachmentHitbox
DT_GrappleHook!0x09f8 m_realmsBitMask
DT_GrappleHook!0x1340 m_grappleZipline
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
DT_HardPointEntity!0x0918 m_minimapData
DT_HardPointEntity!0x0a44 m_state
DT_HardPointEntity!0x0a48 m_estimatedCaptureTime
DT_HardPointEntity!0x0a4c m_progressRefPoint
DT_HardPointEntity!0x0a50 m_teamMilitiaAICount
DT_HardPointEntity!0x0a54 m_teamIMCAICount
DT_HardPointEntity!0x0a58 m_teamMilitiaPlayerCount
DT_HardPointEntity!0x0a5c m_teamIMCPlayerCount
DT_HardPointEntity!0x0a60 m_teamMilitiaPlayerTitanCount
DT_HardPointEntity!0x0a64 m_teamIMCPlayerTitanCount
DT_HardPointEntity!0x0a68 m_hardpointID
DT_HardPointEntity!0x0a70 m_terminal
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
DT_HighlightSettings!0x01a8 m_highlightParams
DT_HighlightSettings!0x0268 m_highlightFunctionBits
DT_HighlightSettings!0x02a8 m_highlightServerFadeBases
DT_HighlightSettings!0x02b0 m_highlightServerFadeStartTimes
DT_HighlightSettings!0x02b8 m_highlightServerFadeEndTimes
DT_HighlightSettings!0x02f8 m_highlightServerContextID
DT_HighlightSettings!0x0304 m_highlightTeamBits
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
DT_ImportantOnEntSound!0x0a40 m_networkTableSoundID
DT_ImportantOnEntSound!0x0a44 m_hAttachedToEntity
DT_ImportantOnEntSound!0x0a48 m_beginTime
DT_ImportantOnEntSound!0x0a4c m_hSuppressedClient
DT_ImportantOnEntSound!0x0a50 m_milesSignal
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
DT_InfoPlacementHelper!0x0428 m_localAngles
DT_InfoPlacementHelper!0x07f0 m_parentAttachmentType
DT_InfoPlacementHelper!0x07f4 m_parentAttachmentIndex
DT_InfoPlacementHelper!0x07f8 m_parentAttachmentHitbox
DT_InfoPlacementHelper!0x07fc m_parentAttachmentModel
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
DT_InfoTarget!0x0048 m_cellX
DT_InfoTarget!0x004c m_cellY
DT_InfoTarget!0x0050 m_cellZ
DT_InfoTarget!0x0054 m_localOrigin
DT_InfoTarget!0x03f0 m_iTeamNum
DT_InfoTarget!0x0428 m_localAngles
DT_InfoTarget!0x043c m_hOwnerEntity
DT_InfoTarget!0x0518 m_iSignifierName
DT_InfoTarget!0x0521 m_iName
DT_InfoTarget!0x0628 m_scriptNameIndex
DT_InfoTarget!0x062c m_instanceNameIndex
DT_InfoTarget!0x07f0 m_parentAttachmentType
DT_InfoTarget!0x07f4 m_parentAttachmentIndex
DT_InfoTarget!0x07f8 m_parentAttachmentHitbox
DT_InfoTarget!0x07fc m_parentAttachmentModel
DT_InfoTarget!0x09f0 m_firstChildEntityLink
DT_InfoTarget!0x09f4 m_firstParentEntityLink
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
	m_skydiveFreelookEnabled: Int,
	m_skydiveFreelookLockedAngle: Vector,
	m_skydiveFollowing: Int,
	m_skydiveUnfollowVelocity: Vector,
	m_skydiveIsNearLeviathan: Int,
	m_skydiveLeviathanHitPosition: Vector,
	m_skydiveLeviathanHitNormal: Vector,
	m_skydiveSlipVelocity: Vector,
	m_twitchRewardBits: BitMask,
	m_playerKnockBacks: DataTable,
	m_armsModelIndex: Int,
}
```

### Offsets

```
DT_LocalPlayerExclusive!0x0000 NearbyPushers
DT_LocalPlayerExclusive!0x0004 m_localOrigin
DT_LocalPlayerExclusive!0x000c m_localOrigin.z
DT_LocalPlayerExclusive!0x0130 m_vecAbsVelocity
DT_LocalPlayerExclusive!0x03d0 m_vecBaseVelocity
DT_LocalPlayerExclusive!0x041c m_vecVelocity.x
DT_LocalPlayerExclusive!0x0420 m_vecVelocity.y
DT_LocalPlayerExclusive!0x0424 m_vecVelocity.z
DT_LocalPlayerExclusive!0x0434 m_flFriction
DT_LocalPlayerExclusive!0x185c m_tethers
DT_LocalPlayerExclusive!0x1910 m_lastUCmdSimulationTicks
DT_LocalPlayerExclusive!0x1914 m_lastUCmdSimulationRemainderTime
DT_LocalPlayerExclusive!0x1ad0 m_Local
DT_LocalPlayerExclusive!0x1f80 m_currentFrameLocalPlayer
DT_LocalPlayerExclusive!0x2180 m_hTonemapController
DT_LocalPlayerExclusive!0x22d4 m_modInventory
DT_LocalPlayerExclusive!0x2354 m_consumableInventory
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
DT_LocalPlayerExclusive!0x42cc m_skydiveFreelookEnabled
DT_LocalPlayerExclusive!0x42d0 m_skydiveFreelookLockedAngle
DT_LocalPlayerExclusive!0x42e4 m_skydiveFollowing
DT_LocalPlayerExclusive!0x42e8 m_skydiveUnfollowVelocity
DT_LocalPlayerExclusive!0x42f5 m_skydiveIsNearLeviathan
DT_LocalPlayerExclusive!0x42f8 m_skydiveLeviathanHitPosition
DT_LocalPlayerExclusive!0x4304 m_skydiveLeviathanHitNormal
DT_LocalPlayerExclusive!0x4310 m_skydiveSlipVelocity
DT_LocalPlayerExclusive!0x4320 m_twitchRewardBits
DT_LocalPlayerExclusive!0x4330 m_playerKnockBacks
DT_LocalPlayerExclusive!0x43b0 m_armsModelIndex
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
DT_MovieDisplay!0x0a40 m_bEnabled
DT_MovieDisplay!0x0a41 m_bLooping
DT_MovieDisplay!0x0a43 m_szMovieFilename
DT_MovieDisplay!0x0ac3 m_szGroupName
DT_MovieDisplay!0x0b43 m_szExternalAudioFilename
DT_MovieDisplay!0x0b83 m_bStretchToFill
DT_MovieDisplay!0x0b84 m_bLetterbox
DT_MovieDisplay!0x0b85 m_bPausesWithClient
DT_MovieDisplay!0x0b86 m_bForcedSlave
DT_MovieDisplay!0x0b87 m_bUseCustomUVs
DT_MovieDisplay!0x0b8c m_flUMin
DT_MovieDisplay!0x0b90 m_flUMax
DT_MovieDisplay!0x0b94 m_flVMin
DT_MovieDisplay!0x0b98 m_flVMax
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
DT_ParticleSystem!0x0040 m_fEffects
DT_ParticleSystem!0x03e8 m_visibilityFlags
DT_ParticleSystem!0x03f0 m_iTeamNum
DT_ParticleSystem!0x0428 m_localAngles
DT_ParticleSystem!0x043c m_hOwnerEntity
DT_ParticleSystem!0x07f4 m_parentAttachmentIndex
DT_ParticleSystem!0x07f8 m_parentAttachmentHitbox
DT_ParticleSystem!0x07fc m_parentAttachmentModel
DT_ParticleSystem!0x09f8 m_realmsBitMask
DT_ParticleSystem!0x0a40 m_iEffectIndex
DT_ParticleSystem!0x0a44 m_nStopType
DT_ParticleSystem!0x0a49 m_bActive
DT_ParticleSystem!0x0a4b m_bForceRenderAlways
DT_ParticleSystem!0x0a4c m_flStartTime
DT_ParticleSystem!0x0a55 m_bInSkybox
DT_ParticleSystem!0x0a56 m_killForReplay
DT_ParticleSystem!0x0a57 m_killIfOverLimit
DT_ParticleSystem!0x0a5c m_vServerControlPoints
DT_ParticleSystem!0x0a98 m_hControlPointEnts
DT_ParticleSystem!0x0aac m_controlPointAttachTypes
DT_ParticleSystem!0x0ac4 m_controlPoint1AttachmentIndex
DT_ParticleSystem!0x0ad4 m_vServerControlPointColorIds
DT_ParticleSystem!0x0adc m_parentAttachType
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
DT_PhysicsProp!0x07ac m_spawnflags
DT_PhysicsProp!0x1388 m_bAwake
DT_PhysicsProp!0x1389 m_ignoresCollisionWithPlayers
DT_PhysicsProp!0x13a4 m_iPhysicsMode
DT_PhysicsProp!0x13a8 m_fMass
DT_PhysicsProp!0x13ac m_collisionMins
DT_PhysicsProp!0x13b8 m_collisionMaxs
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
DT_Player!0x008c m_fFlags
DT_Player!0x03dc m_hGroundEntity
DT_Player!0x03e0 m_iHealth
DT_Player!0x03e4 m_flMaxspeed
DT_Player!0x0510 m_iMaxHealth
DT_Player!0x0730 m_lifeState
DT_Player!0x0e94 m_decalIndex
DT_Player!0x16f0 m_inventory
DT_Player!0x174e m_selectedOffhands
DT_Player!0x1751 m_selectedOffhandsPendingHybridAction
DT_Player!0x17c0 m_titanSoul
DT_Player!0x1881 m_bZooming
DT_Player!0x1884 m_zoomToggleOnStartTime
DT_Player!0x1888 m_zoomBaseFrac
DT_Player!0x188c m_zoomBaseTime
DT_Player!0x1890 m_zoomFullStartTime
DT_Player!0x1db0 m_currentFramePlayer
DT_Player!0x2188 pl
DT_Player!0x220c m_ammoPoolCapacity
DT_Player!0x2210 m_hasBadReputation
DT_Player!0x2211 m_happyHourActive
DT_Player!0x2219 m_communityName
DT_Player!0x2259 m_communityClanTag
DT_Player!0x2269 m_factionName
DT_Player!0x2279 m_hardwareIcon
DT_Player!0x2289 m_hardware
DT_Player!0x2290 m_platformUserId
DT_Player!0x22a0 m_classModsActive
DT_Player!0x2398 m_passives[ 0 ]
DT_Player!0x23b8 m_bleedoutState
DT_Player!0x23c0 m_statusEffectsTimedPlayerNV
DT_Player!0x24b0 m_statusEffectsEndlessPlayerNV
DT_Player!0x2564 m_damageComboLatestUpdateTime
DT_Player!0x2568 m_damageComboStartHealth
DT_Player!0x256c m_gestureSequences
DT_Player!0x258c m_gestureStartTimes
DT_Player!0x25ac m_gestureBlendInDuration
DT_Player!0x25cc m_gestureBlendOutDuration
DT_Player!0x25ec m_gestureFadeOutStartTime
DT_Player!0x260c m_gestureFadeOutDuration
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
DT_Player!0x42c0 m_skydiveIsDiving
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
DT_PlayerDecoy!0x018c m_cloakEndTime
DT_PlayerDecoy!0x0190 m_cloakFadeInEndTime
DT_PlayerDecoy!0x0194 m_cloakFadeOutStartTime
DT_PlayerDecoy!0x0198 m_cloakFadeInDuration
DT_PlayerDecoy!0x019c m_cloakFlickerAmount
DT_PlayerDecoy!0x01a0 m_cloakFlickerEndTime
DT_PlayerDecoy!0x03e0 m_iHealth
DT_PlayerDecoy!0x0510 m_iMaxHealth
DT_PlayerDecoy!0x0968 m_nameVisibilityFlags
DT_PlayerDecoy!0x1340 m_currentState
DT_PlayerDecoy!0x1344 m_decoyFlags
DT_PlayerDecoy!0x134c m_lastPulseTime
DT_PlayerDecoy!0x1350 m_currentClass
DT_PlayerDecoy!0x1358 m_classModsActive
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
DT_PlayerResource!0x1450 m_boolStats
DT_PlayerResource!0x2c80 m_iPing
DT_PlayerResource!0x2e84 m_bConnected
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
DT_PlayerTeamShared!0x2214 m_healResources_healthTarget
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
DT_PlayerVehicle!0x0034 m_vecViewOffset.x
DT_PlayerVehicle!0x0038 m_vecViewOffset.y
DT_PlayerVehicle!0x003c m_vecViewOffset.z
DT_PlayerVehicle!0x0428 m_localAngles
DT_PlayerVehicle!0x1344 m_vehicleDriver
DT_PlayerVehicle!0x134c m_vehiclePlayers[0]
DT_PlayerVehicle!0x135c m_vehiclePlayerCount
DT_PlayerVehicle!0x1360 m_vehicleActivated
DT_PlayerVehicle!0x1364 m_vehicleFlags
DT_PlayerVehicle!0x1368 m_vehicleType
DT_PlayerVehicle!0x136c m_vehicleLaunchTime
DT_PlayerVehicle!0x1374 m_vehicleVelocity
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
DT_PlayerWaypoint!0x0048 m_cellX
DT_PlayerWaypoint!0x004c m_cellY
DT_PlayerWaypoint!0x0050 m_cellZ
DT_PlayerWaypoint!0x0054 m_localOrigin
DT_PlayerWaypoint!0x0394 m_networkedFlags
DT_PlayerWaypoint!0x03e8 m_visibilityFlags
DT_PlayerWaypoint!0x03f0 m_iTeamNum
DT_PlayerWaypoint!0x03f4 m_teamMemberIndex
DT_PlayerWaypoint!0x043c m_hOwnerEntity
DT_PlayerWaypoint!0x0518 m_iSignifierName
DT_PlayerWaypoint!0x07f4 m_parentAttachmentIndex
DT_PlayerWaypoint!0x07fc m_parentAttachmentModel
DT_PlayerWaypoint!0x09f8 m_realmsBitMask
DT_PlayerWaypoint!0x0a40 m_waypointType
DT_PlayerWaypoint!0x0a44 m_waypointBitfield
DT_PlayerWaypoint!0x0a48 m_waypointEnts
DT_PlayerWaypoint!0x0a68 m_waypointVectors
DT_PlayerWaypoint!0x0ac8 m_waypointGameTimes
DT_PlayerWaypoint!0x0ae8 m_waypointInts
DT_PlayerWaypoint!0x0b08 m_waypointFloats
DT_PlayerWaypoint!0x0b28 m_objectivePackedInt
DT_PlayerWaypoint!0x0b2c m_waypointGroupName
DT_PlayerWaypoint!0x0b4c m_waypointGroupFlags
DT_PlayerWaypoint!0x0b50 m_waypointCustomType
DT_PlayerWaypoint!0x0b70 m_waypointStringA
DT_PlayerWaypoint!0x0bb0 m_waypointStringB
DT_PlayerWaypoint!0x0bf8 m_waypointAssetA
DT_PlayerWaypoint!0x0c78 m_waypointAssetB
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
DT_PortalNonLocalPlayerExclusive!0x0024 m_pusher
DT_PortalNonLocalPlayerExclusive!0x0028 m_originRelativeToPusher
DT_PortalNonLocalPlayerExclusive!0x0048 m_cellX
DT_PortalNonLocalPlayerExclusive!0x004c m_cellY
DT_PortalNonLocalPlayerExclusive!0x0050 m_cellZ
DT_PortalNonLocalPlayerExclusive!0x0054 m_localOrigin
DT_PortalNonLocalPlayerExclusive!0x005c m_localOrigin.z
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
DT_PortalPointPush!0x0a40 m_bEnabled
DT_PortalPointPush!0x0a44 m_flMagnitude
DT_PortalPointPush!0x0a48 m_flRadius
DT_PortalPointPush!0x0a4c m_flInnerRadius
DT_PortalPointPush!0x0a50 m_flConeOfInfluence
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
DT_Projectile!0x0048 m_cellX
DT_Projectile!0x004c m_cellY
DT_Projectile!0x0050 m_cellZ
DT_Projectile!0x0054 m_localOrigin
DT_Projectile!0x0060 m_nModelIndex
DT_Projectile!0x0394 m_networkedFlags
DT_Projectile!0x03f0 m_iTeamNum
DT_Projectile!0x041c m_vecVelocity
DT_Projectile!0x0428 m_localAngles
DT_Projectile!0x043c m_hOwnerEntity
DT_Projectile!0x04d8 m_CollisionGroup
DT_Projectile!0x0764 m_PredictableID
DT_Projectile!0x09f8 m_realmsBitMask
DT_Projectile!0x1340 m_weaponDataIsSet
DT_Projectile!0x1341 m_forceAdjustToGunBarrelDisabled
DT_Projectile!0x1344 m_weaponClassIndex
DT_Projectile!0x1348 m_destructionDistance
DT_Projectile!0x134c m_passThroughDepthTotal
DT_Projectile!0x1350 m_modBitfield
DT_Projectile!0x1354 m_overrideMods
DT_Projectile!0x1358 m_projectileTrailIndex
DT_Projectile!0x135c m_impactEffectTable
DT_Projectile!0x1360 m_reducedEffects
DT_Projectile!0x1364 m_projectileCreationTimeServer
DT_Projectile!0x1368 m_weaponSource
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
DT_PropSurvival!0x0040 m_fEffects
DT_PropSurvival!0x0044 m_usableType
DT_PropSurvival!0x0048 m_cellX
DT_PropSurvival!0x004c m_cellY
DT_PropSurvival!0x0050 m_cellZ
DT_PropSurvival!0x0054 m_localOrigin
DT_PropSurvival!0x0060 m_nModelIndex
DT_PropSurvival!0x0394 m_networkedFlags
DT_PropSurvival!0x03e8 m_visibilityFlags
DT_PropSurvival!0x0428 m_localAngles
DT_PropSurvival!0x0458 m_Collision
DT_PropSurvival!0x04d8 m_CollisionGroup
DT_PropSurvival!0x0518 m_iSignifierName
DT_PropSurvival!0x07f4 m_parentAttachmentIndex
DT_PropSurvival!0x07fc m_parentAttachmentModel
DT_PropSurvival!0x0808 m_fadeDist
DT_PropSurvival!0x08d8 m_usablePriority
DT_PropSurvival!0x08dc m_usableDistanceOverride
DT_PropSurvival!0x08e0 m_usableFOV
DT_PropSurvival!0x08e4 m_usePromptSize
DT_PropSurvival!0x09f8 m_realmsBitMask
DT_PropSurvival!0x0e88 m_nSkin
DT_PropSurvival!0x0e8c m_nBody
DT_PropSurvival!0x0e90 m_camoIndex
DT_PropSurvival!0x1344 m_ammoInClip
DT_PropSurvival!0x1348 m_customScriptInt
DT_PropSurvival!0x134c m_survivalProperty
DT_PropSurvival!0x1350 m_weaponNameIndex
DT_PropSurvival!0x1354 m_modBitField
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
DT_RopeKeyframe!0x043c m_hOwnerEntity
DT_RopeKeyframe!0x07f4 m_parentAttachmentIndex
DT_RopeKeyframe!0x07f8 m_parentAttachmentHitbox
DT_RopeKeyframe!0x07fc m_parentAttachmentModel
DT_RopeKeyframe!0x0808 m_fadeDist
DT_RopeKeyframe!0x0a40 m_ropeZiplineAutoDetachDistance
DT_RopeKeyframe!0x0a44 m_ziplineSagEnable
DT_RopeKeyframe!0x0a48 m_ziplineSagHeight
DT_RopeKeyframe!0x0b40 m_ziplineMoveSpeedScale
DT_RopeKeyframe!0x0b44 m_wiggleFadeStartTime
DT_RopeKeyframe!0x0b48 m_wiggleEndTime
DT_RopeKeyframe!0x0b4c m_wiggleMaxLen
DT_RopeKeyframe!0x0b50 m_wiggleMagnitude
DT_RopeKeyframe!0x0b54 m_wiggleSpeed
DT_RopeKeyframe!0x0b8c m_flScrollSpeed
DT_RopeKeyframe!0x0b90 m_RopeFlags
DT_RopeKeyframe!0x0b94 m_iRopeMaterialModelIndex
DT_RopeKeyframe!0x0e18 m_nSegments
DT_RopeKeyframe!0x0e1c m_hStartPoint
DT_RopeKeyframe!0x0e20 m_hEndPoint
DT_RopeKeyframe!0x0e24 m_hPrevPoint
DT_RopeKeyframe!0x0e28 m_iStartAttachment
DT_RopeKeyframe!0x0e2a m_iEndAttachment
DT_RopeKeyframe!0x0e54 m_subdivStackCount
DT_RopeKeyframe!0x0e58 m_subdivSliceCount
DT_RopeKeyframe!0x0e5c m_ropeLength
DT_RopeKeyframe!0x0e64 m_constraintIterations
DT_RopeKeyframe!0x0e68 m_ropeDampening
DT_RopeKeyframe!0x0e6c m_Slack
DT_RopeKeyframe!0x0e70 m_TextureScale
DT_RopeKeyframe!0x0e70 m_TextureScale
DT_RopeKeyframe!0x0e74 m_fLockedPoints
DT_RopeKeyframe!0x0e78 m_lockDirectionCutoffLength
DT_RopeKeyframe!0x0e7c m_lockDirectionStrength
DT_RopeKeyframe!0x0e80 m_nChangeCount
DT_RopeKeyframe!0x0e84 m_Width
DT_RopeKeyframe!0x0f10 m_bConstrainBetweenEndpoints
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
DT_ScriptMover!0x0118 m_vecAngVelocity
DT_ScriptMover!0x041c m_vecVelocity
DT_ScriptMover!0x0428 m_localAngles
DT_ScriptMover!0x07f8 m_parentAttachmentHitbox
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
	m_moveToTrainNodeStartTime: Time,
	m_moveToTrainNodeStopTime: Time,
	m_moveToTrainNodeStartDistance: Float,
	m_moveToTrainNodeEntity: Int,
	m_moveToTrainNodeInitialSpeed: Float,
	m_moveToTrainNodeGoalSpeed: Float,
	m_moveToTrainNodeGoalSpeedForceZero: Int,
	m_moveToTrainNodeAcceleration: Float,
	m_followMover: Int,
	m_followMoverDistance: Float,
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
DT_ScriptMoverLightweight!0x0040 m_fEffects
DT_ScriptMoverLightweight!0x0048 m_moverNetworkCellX
DT_ScriptMoverLightweight!0x004c m_moverNetworkCellY
DT_ScriptMoverLightweight!0x0050 m_moverNetworkCellZ
DT_ScriptMoverLightweight!0x0054 m_moverNetworkLocalOrigin
DT_ScriptMoverLightweight!0x0060 m_nModelIndex
DT_ScriptMoverLightweight!0x0118 m_moverNetworkAngularVelocity
DT_ScriptMoverLightweight!0x0394 m_networkedFlags
DT_ScriptMoverLightweight!0x041c m_moverNetworkLinearVelocity
DT_ScriptMoverLightweight!0x0428 m_moverNetworkLocalAngles
DT_ScriptMoverLightweight!0x07f4 m_parentAttachmentIndex
DT_ScriptMoverLightweight!0x07f8 m_parentAttachmentHitbox
DT_ScriptMoverLightweight!0x07fc m_parentAttachmentModel
DT_ScriptMoverLightweight!0x0808 m_fadeDist
DT_ScriptMoverLightweight!0x14c4 m_moveModeNonPhysics
DT_ScriptMoverLightweight!0x14c8 m_moveModeIsLocal
DT_ScriptMoverLightweight!0x14cc m_moveToStartPos
DT_ScriptMoverLightweight!0x14d8 m_moveToEndPos
DT_ScriptMoverLightweight!0x14e4 m_moveToTimeStart
DT_ScriptMoverLightweight!0x14e8 m_moveToTimeEnd
DT_ScriptMoverLightweight!0x14ec m_moveToTimeEaseIn
DT_ScriptMoverLightweight!0x14f0 m_moveToTimeEaseOut
DT_ScriptMoverLightweight!0x14f4 m_moveVelocity
DT_ScriptMoverLightweight!0x1500 m_moveGravity
DT_ScriptMoverLightweight!0x150c m_moveToTrainNodeStartTime
DT_ScriptMoverLightweight!0x1510 m_moveToTrainNodeStopTime
DT_ScriptMoverLightweight!0x1514 m_moveToTrainNodeStartDistance
DT_ScriptMoverLightweight!0x1518 m_moveToTrainNodeEntity
DT_ScriptMoverLightweight!0x151c m_moveToTrainNodeInitialSpeed
DT_ScriptMoverLightweight!0x1520 m_moveToTrainNodeGoalSpeed
DT_ScriptMoverLightweight!0x1524 m_moveToTrainNodeGoalSpeedForceZero
DT_ScriptMoverLightweight!0x1528 m_moveToTrainNodeAcceleration
DT_ScriptMoverLightweight!0x152c m_followMover
DT_ScriptMoverLightweight!0x1530 m_followMoverDistance
DT_ScriptMoverLightweight!0x1534 m_rotateModeNonPhysics
DT_ScriptMoverLightweight!0x1538 m_rotateModeIsLocal
DT_ScriptMoverLightweight!0x153c m_RotateToAnglesStart
DT_ScriptMoverLightweight!0x1548 m_RotateToAnglesEnd
DT_ScriptMoverLightweight!0x1554 m_rotateToTimeStart
DT_ScriptMoverLightweight!0x1558 m_rotateToTimeEnd
DT_ScriptMoverLightweight!0x155c m_rotateToTimeEaseIn
DT_ScriptMoverLightweight!0x1560 m_rotateToTimeEaseOut
DT_ScriptMoverLightweight!0x1564 m_rotateAxis
DT_ScriptMoverLightweight!0x1570 m_rotateSpeed
DT_ScriptMoverLightweight!0x1581 m_useNonPhysicsMoveInterpolation
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
DT_ScriptMoverTrainNode!0x0048 m_cellX
DT_ScriptMoverTrainNode!0x004c m_cellY
DT_ScriptMoverTrainNode!0x0050 m_cellZ
DT_ScriptMoverTrainNode!0x0054 m_localOrigin
DT_ScriptMoverTrainNode!0x0628 m_scriptNameIndex
DT_ScriptMoverTrainNode!0x09f0 m_firstChildEntityLink
DT_ScriptMoverTrainNode!0x09f4 m_firstParentEntityLink
DT_ScriptMoverTrainNode!0x0a40 m_numSmoothPoints
DT_ScriptMoverTrainNode!0x0a44 m_trainNodeMakeSmoothPointsParity
DT_ScriptMoverTrainNode!0x0a48 m_tangentType
DT_ScriptMoverTrainNode!0x0a4c m_perfectCircularRotation
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
DT_ScriptNetData_SNDC_DEATH_BOX!0x0c80 m_bools[0]
DT_ScriptNetData_SNDC_DEATH_BOX!0x0c82 m_ranges[0]
DT_ScriptNetData_SNDC_DEATH_BOX!0x0ca4 m_int32s[0]
DT_ScriptNetData_SNDC_DEATH_BOX!0x0ca8 m_times[0]
DT_ScriptNetData_SNDC_DEATH_BOX!0x0cac m_entities[0]
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
DT_ScriptNetData_SNDC_GLOBAL!0x0c80 m_bools[0]
DT_ScriptNetData_SNDC_GLOBAL!0x0c90 m_ranges[0]
DT_ScriptNetData_SNDC_GLOBAL!0x0cd0 m_int32s[0]
DT_ScriptNetData_SNDC_GLOBAL!0x0cf0 m_times[0]
DT_ScriptNetData_SNDC_GLOBAL!0x0d30 m_entities[0]
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
DT_ScriptNetData_SNDC_PLAYER_EXCLUSIVE!0x0c80 m_bools[0]
DT_ScriptNetData_SNDC_PLAYER_EXCLUSIVE!0x0c90 m_ranges[0]
DT_ScriptNetData_SNDC_PLAYER_EXCLUSIVE!0x0cd0 m_int32s[0]
DT_ScriptNetData_SNDC_PLAYER_EXCLUSIVE!0x0cf0 m_times[0]
DT_ScriptNetData_SNDC_PLAYER_EXCLUSIVE!0x0d10 m_entities[0]
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
DT_ScriptNetData_SNDC_PLAYER_GLOBAL!0x0c80 m_bools[0]
DT_ScriptNetData_SNDC_PLAYER_GLOBAL!0x0c88 m_ranges[0]
DT_ScriptNetData_SNDC_PLAYER_GLOBAL!0x0cc8 m_int32s[0]
DT_ScriptNetData_SNDC_PLAYER_GLOBAL!0x0ce8 m_times[0]
DT_ScriptNetData_SNDC_PLAYER_GLOBAL!0x0d08 m_entities[0]
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
DT_ScriptNetData_SNDC_TITAN_SOUL!0x0c80 m_bools[0]
DT_ScriptNetData_SNDC_TITAN_SOUL!0x0c88 m_ranges[0]
DT_ScriptNetData_SNDC_TITAN_SOUL!0x0ca8 m_int32s[0]
DT_ScriptNetData_SNDC_TITAN_SOUL!0x0cb0 m_times[0]
DT_ScriptNetData_SNDC_TITAN_SOUL!0x0cd0 m_entities[0]
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
DT_ScriptProp!0x0918 m_minimapData
DT_ScriptProp!0x0968 m_nameVisibilityFlags
DT_ScriptProp!0x13e0 m_title
DT_ScriptProp!0x1400 m_footstepType
DT_ScriptProp!0x1440 m_renderColorFriendlyIsValid
DT_ScriptProp!0x1441 m_renderColorFriendly
DT_ScriptProp!0x1448 m_armorType
DT_ScriptProp!0x144c m_scriptPropFlags
DT_ScriptProp!0x1450 m_scriptPropSmartAmmoLockType
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
DT_ScriptTraceVolume!0x0a40 m_shapeType
DT_ScriptTraceVolume!0x0a44 m_sphereRadius
DT_ScriptTraceVolume!0x0a48 m_boxMins
DT_ScriptTraceVolume!0x0a54 m_boxMaxs
DT_ScriptTraceVolume!0x0a60 m_drawDebug
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
DT_Sprite!0x0a58 m_hAttachedToEntity
DT_Sprite!0x0a5c m_nAttachment
DT_Sprite!0x0a60 m_flSpriteFramerate
DT_Sprite!0x0a64 m_flFrame
DT_Sprite!0x0a6c m_clrRenderFriendly
DT_Sprite!0x0a70 m_nBrightness
DT_Sprite!0x0a74 m_flBrightnessDuration
DT_Sprite!0x0a78 m_flSpriteScale
DT_Sprite!0x0a7c m_flScaleDuration
DT_Sprite!0x0a80 m_bWorldSpaceScale
DT_Sprite!0x0a84 m_flGlowProxySize
DT_Sprite!0x0a88 m_flHDRColorScale
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
DT_StatueProp!0x1400 m_hInitBaseAnimating
DT_StatueProp!0x1404 m_bShatter
DT_StatueProp!0x1408 m_nShatterFlags
DT_StatueProp!0x140c m_vShatterPosition
DT_StatueProp!0x1418 m_vShatterForce
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
DT_StatusEffectPlugin!0x0a40 m_statusEffectsTimedPluginNV
DT_StatusEffectPlugin!0x0a58 m_statusEffectsEndlessPluginNV
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
DT_Team!0x0a40 m_score
DT_Team!0x0a44 m_score2
DT_Team!0x0a48 m_kills
DT_Team!0x0a4c m_deaths
DT_Team!0x0a50 m_iRoundsWon
DT_Team!0x0a54 m_iTeamTeamNum
DT_Team!0x0a78 m_szTeamname
DT_Team!0x0b78 m_reservedPlayerCount
DT_Team!0x0b7c m_connectingPlayerCount
DT_Team!0x0b80 m_loadingPlayerCount
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
DT_TitanSoul!0x0114 m_bossPlayer
DT_TitanSoul!0x0160 m_shieldHealth
DT_TitanSoul!0x0164 m_shieldHealthMax
DT_TitanSoul!0x0394 m_networkedFlags
DT_TitanSoul!0x0a40 m_titan
DT_TitanSoul!0x0a48 m_titanSoulScriptNetData
DT_TitanSoul!0x0be0 m_lastRodeoHitTime
DT_TitanSoul!0x0be8 m_nextCoreChargeAvailable
DT_TitanSoul!0x0bf0 m_coreChargeExpireTime
DT_TitanSoul!0x0bf8 m_coreChargeStartTime
DT_TitanSoul!0x0bfc m_coreUseDuration
DT_TitanSoul!0x0c00 m_damageComboLatestUpdateTime
DT_TitanSoul!0x0c04 m_damageComboStartHealth
DT_TitanSoul!0x0da8 m_stance
DT_TitanSoul!0x0dac m_doomed
DT_TitanSoul!0x0db0 m_playerSettingsNum
DT_TitanSoul!0x0db8 m_invalidHealthBarEnt
DT_TitanSoul!0x0db9 m_bEjecting
DT_TitanSoul!0x0dba m_isValidRodeoTarget
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
DT_TriggerCylinderHeavy!0x0ac0 m_triggerFilterMask
DT_TriggerCylinderHeavy!0x0ac8 m_radius
DT_TriggerCylinderHeavy!0x0acc m_aboveHeight
DT_TriggerCylinderHeavy!0x0ad0 m_belowHeight
DT_TriggerCylinderHeavy!0x0ad4 m_teslaTrapBaseHeight
DT_TriggerCylinderHeavy!0x0ad8 m_vertOverride
DT_TriggerCylinderHeavy!0x0adc m_launchPower
DT_TriggerCylinderHeavy!0x0ae0 m_punchSoftAmount
DT_TriggerCylinderHeavy!0x0ae4 m_punchHardAmount
DT_TriggerCylinderHeavy!0x0ae8 m_punchRandomBoost
DT_TriggerCylinderHeavy!0x0aec m_triggerType
DT_TriggerCylinderHeavy!0x0af0 m_teslaTrapFXVisible
DT_TriggerCylinderHeavy!0x0af8 m_teslaTrapObstructedEndTime
DT_TriggerCylinderHeavy!0x0afc m_teslaTrapStart
DT_TriggerCylinderHeavy!0x0b00 m_teslaTrapEnd
DT_TriggerCylinderHeavy!0x0b04 m_teslaTrapUp
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
DT_TriggerPointGravity!0x0ac0 m_pullOuterRadius
DT_TriggerPointGravity!0x0ac4 m_pullInnerRadius
DT_TriggerPointGravity!0x0ac8 m_reduceSpeedOuterRadius
DT_TriggerPointGravity!0x0acc m_reduceSpeedInnerRadius
DT_TriggerPointGravity!0x0ad0 m_pullAccel
DT_TriggerPointGravity!0x0ad4 m_pullSpeed
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
DT_Turret!0x0510 m_iMaxHealth
DT_Turret!0x16f0 m_inventory
DT_Turret!0x1888 m_settingsIndex
DT_Turret!0x18a0 m_driver
DT_Turret!0x18b0 m_forceAimPitch
DT_Turret!0x18b4 m_forceAimYaw
DT_Turret!0x18b8 m_driverTimeStart
DT_Turret!0x18bc m_title
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
DT_VGuiScreen!0x0a40 m_flWidth
DT_VGuiScreen!0x0a44 m_flHeight
DT_VGuiScreen!0x0a50 m_nPanelName
DT_VGuiScreen!0x0a6c m_nAttachmentIndex
DT_VGuiScreen!0x0a70 m_nOverlayMaterial
DT_VGuiScreen!0x0a74 m_fScreenFlags
DT_VGuiScreen!0x0af8 m_hPlayerOwner
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
DT_VehicleNonDriverExclusive!0x0048 m_cellX
DT_VehicleNonDriverExclusive!0x004c m_cellY
DT_VehicleNonDriverExclusive!0x0050 m_cellZ
DT_VehicleNonDriverExclusive!0x0054 m_localOrigin
DT_VehicleNonDriverExclusive!0x005c m_localOrigin.z
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
DT_VortexSphere!0x07ac m_spawnflags
DT_VortexSphere!0x0a40 m_enabled
DT_VortexSphere!0x0a44 m_radius
DT_VortexSphere!0x0a48 m_height
DT_VortexSphere!0x0a4c m_bulletFov
DT_VortexSphere!0x0a50 m_bulletAbsorbedCount
DT_VortexSphere!0x0a54 m_projectileAbsorbedCount
DT_VortexSphere!0x0a58 m_ownerWeapon
DT_VortexSphere!0x0a5c m_vortexEffect
DT_VortexSphere!0x0a60 m_vortexLocalAngles
DT_VortexSphere!0x0a70 m_gunAttachment
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
DT_WeaponInventory!0x0064 activeWeapons
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
	m_oaActiveOverride: Int,
}
```

### Offsets

```
DT_WeaponX!0x0000 LocalWeaponData
DT_WeaponX!0x0000 predictingClientOnly
DT_WeaponX!0x0394 m_networkedFlags
DT_WeaponX!0x1028 m_bClientSideAnimation
DT_WeaponX!0x1340 m_weaponOwner
DT_WeaponX!0x1354 m_worldModelIndexOverride
DT_WeaponX!0x1358 m_iWorldModelIndex
DT_WeaponX!0x135c m_holsterModelIndex
DT_WeaponX!0x1360 m_droppedModelIndex
DT_WeaponX!0x1364 m_nIdealSequence
DT_WeaponX!0x1368 m_IdealActivity
DT_WeaponX!0x136c m_weaponActivity
DT_WeaponX!0x1370 m_ActiveState
DT_WeaponX!0x1384 m_weapState
DT_WeaponX!0x1388 m_allowedToUse
DT_WeaponX!0x1389 m_discarded
DT_WeaponX!0x138c m_forcedADS
DT_WeaponX!0x1390 m_tossRelease
DT_WeaponX!0x1394 m_customActivity
DT_WeaponX!0x1398 m_customActivitySequence
DT_WeaponX!0x139c m_customActivityOwner
DT_WeaponX!0x13a0 m_customActivityEndTime
DT_WeaponX!0x13a4 m_customActivityFlags
DT_WeaponX!0x13a8 m_playerData
DT_WeaponX!0x1478 m_smartAmmoEnable
DT_WeaponX!0x1480 m_smartAmmo
DT_WeaponX!0x1670 m_needsReloadCheck
DT_WeaponX!0x1671 m_needsEmptyCycleCheck
DT_WeaponX!0x1674 m_skinOverride
DT_WeaponX!0x1678 m_skinOverrideIsValid
DT_WeaponX!0x167c m_chargeStartTime
DT_WeaponX!0x1680 m_chargeEndTime
DT_WeaponX!0x1684 m_lastChargeFrac
DT_WeaponX!0x16a8 m_sustainedDischargeEndTime
DT_WeaponX!0x16ac m_sustainedDischargeIsInPrimaryAttack
DT_WeaponX!0x16b0 m_modBitfieldFromPlayer
DT_WeaponX!0x16b4 m_modBitfieldInternal
DT_WeaponX!0x16b8 m_modBitfieldCurrent
DT_WeaponX!0x16bc m_curSharedEnergyCost
DT_WeaponX!0x16c0 m_grappleWeaponNeedsDryfire
DT_WeaponX!0x16c1 m_scriptActivated
DT_WeaponX!0x16c2 m_isLoadoutPickup
DT_WeaponX!0x16c4 m_utilityEnt
DT_WeaponX!0x16cc m_weaponNameIndex
DT_WeaponX!0x16d8 m_shouldPlayIdleAnims
DT_WeaponX!0x16dc m_oaActiveOverride
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
DT_WeaponX_LocalWeaponData!0x1344 m_lastPrimaryAttack
DT_WeaponX_LocalWeaponData!0x1348 m_nextReadyTime
DT_WeaponX_LocalWeaponData!0x134c m_nextPrimaryAttackTime
DT_WeaponX_LocalWeaponData!0x1350 m_attackTimeThisFrame
DT_WeaponX_LocalWeaponData!0x1374 m_ammoInClip
DT_WeaponX_LocalWeaponData!0x1378 m_ammoInStockpile
DT_WeaponX_LocalWeaponData!0x137c m_lifetimeShots
DT_WeaponX_LocalWeaponData!0x1380 m_flTimeWeaponIdle
DT_WeaponX_LocalWeaponData!0x138a m_bInReload
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
DT_WeaponX_PredictingClientOnly!0x1688 m_lastRegenTime
DT_WeaponX_PredictingClientOnly!0x168c m_cooldownEndTime
DT_WeaponX_PredictingClientOnly!0x1690 m_stockPileWasDraining
DT_WeaponX_PredictingClientOnly!0x1694 m_lastChargeLevel
DT_WeaponX_PredictingClientOnly!0x1698 m_chargeEnergyDepleteStepCounter
DT_WeaponX_PredictingClientOnly!0x169c m_burstFireCount
DT_WeaponX_PredictingClientOnly!0x16a0 m_burstFireIndex
DT_WeaponX_PredictingClientOnly!0x16a4 m_shotCount
DT_WeaponX_PredictingClientOnly!0x16d0 m_animModelIndexPredictingClientOnly
DT_WeaponX_PredictingClientOnly!0x16d4 m_animSequencePredictingClientOnly
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
DT_World!0x0a40 m_WorldMins
DT_World!0x0a4c m_WorldMaxs
DT_World!0x0a58 m_bStartDark
DT_World!0x0a6c m_statusEffectsGenerationNV
DT_World!0x0a74 m_worldFlags
DT_World!0x0a78 m_timeshiftArmDeviceSkin
DT_World!0x0a7c m_spTitanLoadoutUnlocks
DT_World!0x0a80 m_deathFieldIsActive
DT_World!0x0a84 m_deathFieldOrigin
DT_World!0x0a90 m_deathFieldRadiusStart
DT_World!0x0a94 m_deathFieldRadiusEnd
DT_World!0x0a98 m_deathFieldTimeStart
DT_World!0x0a9c m_deathFieldTimeEnd
DT_World!0x0aa0 m_teamRelationRulesForPVE
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
CBaseGrenade!0x041c m_vecVelocity
CBaseGrenade!0x27c1 m_doesExplode
CBaseGrenade!0x27c4 m_DmgRadius
CBaseGrenade!0x27d4 m_grenadeCreationTime
CBaseGrenade!0x27d8 m_grenadeCreationOrigin
CBaseGrenade!0x2868 m_flDamage
CBaseGrenade!0x286c m_hThrower
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
CBaseViewModel!0x0124 m_angAbsRotation
CBaseViewModel!0x013c m_vecAbsOrigin
CBaseViewModel!0x0148 m_localOrigin
CBaseViewModel!0x0154 m_localAngles
CBaseViewModel!0x03ec m_fEffects
CBaseViewModel!0x0428 m_angNetworkAngles
CBaseViewModel!0x0e8c m_nBody
CBaseViewModel!0x0e98 m_nResetEventsParity
CBaseViewModel!0x0f38 m_bSequenceFinished
CBaseViewModel!0x0f50 m_currentFrameBaseAnimating.animStartTime
CBaseViewModel!0x0f54 m_currentFrameBaseAnimating.animStartCycle
CBaseViewModel!0x0f58 m_currentFrameBaseAnimating.animPlaybackRate
CBaseViewModel!0x0f60 m_currentFrameBaseAnimating.animModelIndex
CBaseViewModel!0x0f64 m_currentFrameBaseAnimating.animSequence
CBaseViewModel!0x0f68 m_currentFrameBaseAnimating.animSequenceParity
CBaseViewModel!0x0f6c m_currentFrameBaseAnimating.m_flPoseParameters
CBaseViewModel!0x1464 m_currentFrameAnimatingOverlay.animOverlayIsActive
CBaseViewModel!0x1470 m_currentFrameAnimatingOverlay.animOverlayStartTime
CBaseViewModel!0x1494 m_currentFrameAnimatingOverlay.animOverlayStartCycle
CBaseViewModel!0x14b8 m_currentFrameAnimatingOverlay.animOverlayPlaybackRate
CBaseViewModel!0x14dc m_currentFrameAnimatingOverlay.animOverlayModelIndex
CBaseViewModel!0x1500 m_currentFrameAnimatingOverlay.animOverlaySequence
CBaseViewModel!0x1524 m_currentFrameAnimatingOverlay.animOverlayWeight
CBaseViewModel!0x156c m_currentFrameAnimatingOverlay.animOverlayAnimTime
CBaseViewModel!0x1590 m_currentFrameAnimatingOverlay.animOverlayFadeInDuration
CBaseViewModel!0x15b4 m_currentFrameAnimatingOverlay.animOverlayFadeOutDuration
CBaseViewModel!0x15d8 m_currentFrameAnimatingOverlay.animOverlayCycle
CBaseViewModel!0x1718 m_viewModelOwner
CBaseViewModel!0x171c m_projectileIsVisible
CBaseViewModel!0x1b00 m_bBlockEventLayer
CBaseViewModel!0x1b01 m_isAdsTransition
CBaseViewModel!0x1b04 m_hWeapon
CBaseViewModel!0x1b08 m_tracerAttachments
CBaseViewModel!0x1b08 m_tracerAttachments
CBaseViewModel!0x1b10 m_tracerAttachmentsScoped
CBaseViewModel!0x1b10 m_tracerAttachmentsScoped
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
CBeam!0x0148 m_localOrigin
CBeam!0x0441 m_nRenderFX
CBeam!0x0451 m_nRenderMode
CBeam!0x0a40 m_flFrameRate
CBeam!0x0a54 m_nNumBeamEnts
CBeam!0x0a5c m_nHaloIndex
CBeam!0x0a60 m_nBeamType
CBeam!0x0a68 m_hAttachEntity
CBeam!0x0a90 m_nAttachIndex
CBeam!0x0ab8 m_fWidth
CBeam!0x0abc m_fEndWidth
CBeam!0x0ac0 m_fFadeLength
CBeam!0x0ac4 m_fHaloScale
CBeam!0x0ac8 m_fAmplitude
CBeam!0x0acc m_fStartFrame
CBeam!0x0ad0 m_fSpeed
CBeam!0x0ad4 m_flFrame
CBeam!0x0adc m_vecEndPos
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
CGrappleHook!0x0108 m_pMoveParent
CGrappleHook!0x0148 m_localOrigin
CGrappleHook!0x0154 m_localAngles
CGrappleHook!0x03e8 m_visibilityFlags
CGrappleHook!0x07f0 m_parentAttachmentType
CGrappleHook!0x07f4 m_parentAttachmentIndex
CGrappleHook!0x07f8 m_parentAttachmentHitbox
CGrappleHook!0x1340 m_grappleZipline
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
CPredictedFirstPersonProxy!0x0148 m_localOrigin
CPredictedFirstPersonProxy!0x0154 m_localAngles
CPredictedFirstPersonProxy!0x041c m_vecVelocity
CPredictedFirstPersonProxy!0x0428 m_angNetworkAngles
CPredictedFirstPersonProxy!0x0c00 m_SequenceTransitioner
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
CSprite!0x0a58 m_hAttachedToEntity
CSprite!0x0a5c m_nAttachment
CSprite!0x0a60 m_flSpriteFramerate
CSprite!0x0a64 m_flFrame
CSprite!0x0a68 m_flDieTime
CSprite!0x0a70 m_nBrightness
CSprite!0x0a74 m_flBrightnessDuration
CSprite!0x0a78 m_flSpriteScale
CSprite!0x0a7c m_flScaleDuration
CSprite!0x0a8c m_flLastTime
CSprite!0x0a90 m_flMaxFrame
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
CTurret!0x18a4 m_aimAngle
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
CWeaponX!0x0148 m_localOrigin
CWeaponX!0x050c m_nNextThinkTick
CWeaponX!0x0c00 m_SequenceTransitioner
CWeaponX!0x1340 m_weaponOwner
CWeaponX!0x1344 m_lastPrimaryAttack
CWeaponX!0x1348 m_nextReadyTime
CWeaponX!0x134c m_nextPrimaryAttackTime
CWeaponX!0x1350 m_attackTimeThisFrame
CWeaponX!0x1354 m_worldModelIndexOverride
CWeaponX!0x1358 m_iWorldModelIndex
CWeaponX!0x135c m_holsterModelIndex
CWeaponX!0x1360 m_droppedModelIndex
CWeaponX!0x1364 m_nIdealSequence
CWeaponX!0x1368 m_IdealActivity
CWeaponX!0x136c m_weaponActivity
CWeaponX!0x1370 m_ActiveState
CWeaponX!0x1374 m_ammoInClip
CWeaponX!0x1378 m_ammoInStockpile
CWeaponX!0x137c m_lifetimeShots
CWeaponX!0x1380 m_flTimeWeaponIdle
CWeaponX!0x1384 m_weapState
CWeaponX!0x1389 m_discarded
CWeaponX!0x138a m_bInReload
CWeaponX!0x1390 m_tossRelease
CWeaponX!0x1394 m_customActivity
CWeaponX!0x1398 m_customActivitySequence
CWeaponX!0x139c m_customActivityOwner
CWeaponX!0x13a0 m_customActivityEndTime
CWeaponX!0x13a4 m_customActivityFlags
CWeaponX!0x13a8 m_playerData
CWeaponX!0x1478 m_smartAmmoEnable
CWeaponX!0x1480 m_smartAmmo
CWeaponX!0x1670 m_needsReloadCheck
CWeaponX!0x1671 m_needsEmptyCycleCheck
CWeaponX!0x1674 m_skinOverride
CWeaponX!0x1678 m_skinOverrideIsValid
CWeaponX!0x167c m_chargeStartTime
CWeaponX!0x1680 m_chargeEndTime
CWeaponX!0x1684 m_lastChargeFrac
CWeaponX!0x1688 m_lastRegenTime
CWeaponX!0x168c m_cooldownEndTime
CWeaponX!0x1690 m_stockPileWasDraining
CWeaponX!0x1694 m_lastChargeLevel
CWeaponX!0x1698 m_chargeEnergyDepleteStepCounter
CWeaponX!0x169c m_burstFireCount
CWeaponX!0x16a0 m_burstFireIndex
CWeaponX!0x16a4 m_shotCount
CWeaponX!0x16a8 m_sustainedDischargeEndTime
CWeaponX!0x16ac m_sustainedDischargeIsInPrimaryAttack
CWeaponX!0x16b0 m_modBitfieldFromPlayer
CWeaponX!0x16b4 m_modBitfieldInternal
CWeaponX!0x16b8 m_modBitfieldCurrent
CWeaponX!0x16bc m_curSharedEnergyCost
CWeaponX!0x16c0 m_grappleWeaponNeedsDryfire
CWeaponX!0x16c1 m_scriptActivated
CWeaponX!0x2938 m_flNextEmptySoundTime
CWeaponX!0x295e m_bRemoveable
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
C_BaseAnimating!0x0a68 m_animNetworkFlags
C_BaseAnimating!0x0a6c m_networkAnimActive
C_BaseAnimating!0x0a6e m_animActive
C_BaseAnimating!0x0a6f m_animCollisionEnabled
C_BaseAnimating!0x0a70 m_animPlantingEnabled
C_BaseAnimating!0x0b68 m_predictedAnimEventData
C_BaseAnimating!0x0c00 m_SequenceTransitioner
C_BaseAnimating!0x0e88 m_nSkin
C_BaseAnimating!0x0e8c m_nBody
C_BaseAnimating!0x0e98 m_nResetEventsParity
C_BaseAnimating!0x0f38 m_bSequenceFinished
C_BaseAnimating!0x0f40 m_bSequenceLooped
C_BaseAnimating!0x0f41 m_bSequenceLoops
C_BaseAnimating!0x0f44 m_flModelScale
C_BaseAnimating!0x0f50 m_currentFrameBaseAnimating.animStartTime
C_BaseAnimating!0x0f54 m_currentFrameBaseAnimating.animStartCycle
C_BaseAnimating!0x0f58 m_currentFrameBaseAnimating.animPlaybackRate
C_BaseAnimating!0x0f60 m_currentFrameBaseAnimating.animModelIndex
C_BaseAnimating!0x0f64 m_currentFrameBaseAnimating.animSequence
C_BaseAnimating!0x0f68 m_currentFrameBaseAnimating.animSequenceParity
C_BaseAnimating!0x0f6c m_currentFrameBaseAnimating.m_flPoseParameters
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
C_BaseAnimatingOverlay!0x1348 m_AnimOverlay
C_BaseAnimatingOverlay!0x1420 m_AnimOverlayCount
C_BaseAnimatingOverlay!0x1464 m_currentFrameAnimatingOverlay.animOverlayIsActive
C_BaseAnimatingOverlay!0x1470 m_currentFrameAnimatingOverlay.animOverlayStartTime
C_BaseAnimatingOverlay!0x1494 m_currentFrameAnimatingOverlay.animOverlayStartCycle
C_BaseAnimatingOverlay!0x14b8 m_currentFrameAnimatingOverlay.animOverlayPlaybackRate
C_BaseAnimatingOverlay!0x14dc m_currentFrameAnimatingOverlay.animOverlayModelIndex
C_BaseAnimatingOverlay!0x1500 m_currentFrameAnimatingOverlay.animOverlaySequence
C_BaseAnimatingOverlay!0x1524 m_currentFrameAnimatingOverlay.animOverlayWeight
C_BaseAnimatingOverlay!0x1548 m_currentFrameAnimatingOverlay.animOverlayOrder
C_BaseAnimatingOverlay!0x156c m_currentFrameAnimatingOverlay.animOverlayAnimTime
C_BaseAnimatingOverlay!0x1590 m_currentFrameAnimatingOverlay.animOverlayFadeInDuration
C_BaseAnimatingOverlay!0x15b4 m_currentFrameAnimatingOverlay.animOverlayFadeOutDuration
C_BaseAnimatingOverlay!0x15d8 m_currentFrameAnimatingOverlay.animOverlayCycle
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
C_BaseCombatCharacter!0x00bc m_currentFrame.weaponGettingSwitchedOut
C_BaseCombatCharacter!0x00c4 m_currentFrame.showActiveWeapon3p
C_BaseCombatCharacter!0x0410 m_deathVelocity
C_BaseCombatCharacter!0x0750 m_phaseShiftFlags
C_BaseCombatCharacter!0x16c0 m_flNextAttack
C_BaseCombatCharacter!0x16c4 m_lastFiredTime
C_BaseCombatCharacter!0x16c8 m_lastFiredWeapon
C_BaseCombatCharacter!0x16cc m_raiseFromMeleeEndTime
C_BaseCombatCharacter!0x16d0 m_sharedEnergyCount
C_BaseCombatCharacter!0x16d4 m_sharedEnergyTotal
C_BaseCombatCharacter!0x16d8 m_sharedEnergyLockoutThreshold
C_BaseCombatCharacter!0x16dc m_lastSharedEnergyRegenTime
C_BaseCombatCharacter!0x16e0 m_sharedEnergyRegenRate
C_BaseCombatCharacter!0x16e4 m_sharedEnergyRegenDelay
C_BaseCombatCharacter!0x16e8 m_lastSharedEnergyTakeTime
C_BaseCombatCharacter!0x16f0 m_inventory
C_BaseCombatCharacter!0x1740 m_selectedWeapons
C_BaseCombatCharacter!0x1744 m_latestPrimaryWeapons
C_BaseCombatCharacter!0x174c m_latestNonOffhandWeapons
C_BaseCombatCharacter!0x174e m_selectedOffhands
C_BaseCombatCharacter!0x1751 m_selectedOffhandsPendingHybridAction
C_BaseCombatCharacter!0x1754 m_lastCycleSlot
C_BaseCombatCharacter!0x1758 m_latestMeleeWeapon
C_BaseCombatCharacter!0x175c m_weaponPermission
C_BaseCombatCharacter!0x1760 m_weaponDelayEnableTime
C_BaseCombatCharacter!0x1764 m_weaponDisabledInScript
C_BaseCombatCharacter!0x1789 m_weaponDisabledFlags
C_BaseCombatCharacter!0x178a m_hudInfo_visibilityTestAlwaysPasses
C_BaseCombatCharacter!0x179c m_contextAction
C_BaseCombatCharacter!0x17c8 m_phaseShiftTimeStart
C_BaseCombatCharacter!0x17cc m_phaseShiftTimeEnd
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
C_BaseEntity!0x0124 m_angAbsRotation
C_BaseEntity!0x013c m_vecAbsOrigin
C_BaseEntity!0x03bc m_vecPrevAbsOrigin
C_BaseEntity!0x03c8 m_flGravity
C_BaseEntity!0x0828 m_ModelName
C_BaseEntity!0x0880 m_rgflCoordinateFrame
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
C_BaseEntity!0x0118 m_vecAngVelocity
C_BaseEntity!0x0124 m_angAbsRotation
C_BaseEntity!0x0130 m_vecAbsVelocity
C_BaseEntity!0x013c m_vecAbsOrigin
C_BaseEntity!0x0148 m_localOrigin
C_BaseEntity!0x0154 m_localAngles
C_BaseEntity!0x03c8 m_flGravity
C_BaseEntity!0x03cc m_flProxyRandomValue
C_BaseEntity!0x03d0 m_vecBaseVelocity
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
<summary><code>class C_BreakableSurface extends C_BaseEntity</code></summary>

```
{
	m_nPanelBits: Char,
}
```

### Offsets

```
C_BreakableSurface!0x0cc8 m_nPanelBits
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
C_ClientRagdoll!0x0441 m_nRenderFX
C_ClientRagdoll!0x0451 m_nRenderMode
C_ClientRagdoll!0x0bc8 m_pRagdoll
C_ClientRagdoll!0x0e88 m_nSkin
C_ClientRagdoll!0x0e8c m_nBody
C_ClientRagdoll!0x1340 m_bFadeOut
C_ClientRagdoll!0x1341 m_bImportant
C_ClientRagdoll!0x1344 m_flEffectTime
C_ClientRagdoll!0x1348 m_iCurrentFriction
C_ClientRagdoll!0x134c m_iMinFriction
C_ClientRagdoll!0x1350 m_iMaxFriction
C_ClientRagdoll!0x1354 m_flFrictionModTime
C_ClientRagdoll!0x1358 m_flFrictionTime
C_ClientRagdoll!0x135c m_iFrictionAnimState
C_ClientRagdoll!0x1360 m_bReleaseRagdoll
C_ClientRagdoll!0x1361 m_bFadingOut
C_ClientRagdoll!0x1364 m_flScaleEnd
C_ClientRagdoll!0x138c m_flScaleTimeStart
C_ClientRagdoll!0x13b4 m_flScaleTimeEnd
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
C_CrossbowBolt!0x27c0 m_bounceCount
C_CrossbowBolt!0x27c4 m_maxBounceCount
C_CrossbowBolt!0x27c8 m_doesGrow
C_CrossbowBolt!0x27cc m_growStartSize
C_CrossbowBolt!0x27d0 m_growStage1Tick
C_CrossbowBolt!0x27d4 m_growStage1Size
C_CrossbowBolt!0x27d8 m_growStage2Tick
C_CrossbowBolt!0x27dc m_growStage2Size
C_CrossbowBolt!0x27e0 m_growStageFinalTick
C_CrossbowBolt!0x27e4 m_growStageFinalSize
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
C_DynamicProp!0x1380 m_bClientSide
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
C_GlobalNonRewinding!0x0a40 m_playerObserver
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
C_Missile!0x27c0 m_hasPlayedWhizby
C_Missile!0x27c4 m_whizByStart
C_Missile!0x27d0 m_whizBySoundName
C_Missile!0x2810 m_homingSpeed
C_Missile!0x2814 m_homingSpeedDodgingPlayer
C_Missile!0x2818 m_launchDir
C_Missile!0x2824 m_hSpecificTarget
C_Missile!0x2828 m_targetOffset
C_Missile!0x2834 m_targetPosition
C_Missile!0x2840 m_useTargetPosition
C_Missile!0x2844 m_postIgnitionSpeed
C_Missile!0x2848 m_flGracePeriodEndsAt
C_Missile!0x284c m_pathSettingsInitialized
C_Missile!0x284d m_expandContractMissile
C_Missile!0x284e m_spiralMissile
C_Missile!0x2850 m_spiralSettings
C_Missile!0x2874 m_expandContractSettings
C_Missile!0x28ac m_lastThinkTime
C_Missile!0x28b0 m_explosionIgnoreEntity
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
C_ParticleSystem!0x0a48 m_bClientSide
C_ParticleSystem!0x0a49 m_bActive
C_ParticleSystem!0x0a50 m_warmUpTime
C_ParticleSystem!0x0a54 m_pauseAfterWarmup
C_ParticleSystem!0x0a55 m_bInSkybox
C_ParticleSystem!0x0a56 m_killForReplay
C_ParticleSystem!0x0a57 m_killIfOverLimit
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
C_Player!0x0108 m_pMoveParent
C_Player!0x0130 m_vecAbsVelocity
C_Player!0x03d0 m_vecBaseVelocity
C_Player!0x03dc m_hGroundEntity
C_Player!0x03e4 m_flMaxspeed
C_Player!0x041c m_vecVelocity
C_Player!0x0434 m_flFriction
C_Player!0x050c m_nNextThinkTick
C_Player!0x0c00 m_SequenceTransitioner
C_Player!0x1881 m_bZooming
C_Player!0x1884 m_zoomToggleOnStartTime
C_Player!0x1888 m_zoomBaseFrac
C_Player!0x188c m_zoomBaseTime
C_Player!0x1890 m_zoomFullStartTime
C_Player!0x1910 m_lastUCmdSimulationTicks
C_Player!0x1914 m_lastUCmdSimulationRemainderTime
C_Player!0x1ad0 m_Local
C_Player!0x1db0 m_currentFramePlayer.timeBase
C_Player!0x1db8 m_currentFramePlayer.statusEffectsTimedPlayerCUR
C_Player!0x1ea8 m_currentFramePlayer.statusEffectsEndlessPlayerCUR
C_Player!0x1f48 m_currentFramePlayer.m_flHullHeight
C_Player!0x1f4c m_currentFramePlayer.m_traversalAnimProgress
C_Player!0x1f50 m_currentFramePlayer.m_sprintTiltFrac
C_Player!0x1f60 m_currentFramePlayer.m_ammoPoolCount
C_Player!0x2130 m_currentFrameLocalPlayer.m_stepSmoothingOffset
C_Player!0x213c m_currentFrameLocalPlayer.m_vecPunchBase_Angle
C_Player!0x2148 m_currentFrameLocalPlayer.m_vecPunchBase_AngleVel
C_Player!0x2154 m_currentFrameLocalPlayer.m_vecPunchWeapon_Angle
C_Player!0x2160 m_currentFrameLocalPlayer.m_vecPunchWeapon_AngleVel.x
C_Player!0x2164 m_currentFrameLocalPlayer.m_vecPunchWeapon_AngleVel.y
C_Player!0x2168 m_currentFrameLocalPlayer.m_vecPunchWeapon_AngleVel.z
C_Player!0x216c m_currentFrameLocalPlayer.m_localGravityRotation
C_Player!0x2188 pl
C_Player!0x220c m_ammoPoolCapacity
C_Player!0x256c m_gestureSequences
C_Player!0x258c m_gestureStartTimes
C_Player!0x25ac m_gestureBlendInDuration
C_Player!0x25cc m_gestureBlendOutDuration
C_Player!0x25ec m_gestureFadeOutStartTime
C_Player!0x260c m_gestureFadeOutDuration
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
C_Player!0x4330 m_playerKnockBacks
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
C_PlayerResource!0x0a40 m_szName
C_PlayerResource!0x1450 m_boolStats
C_PlayerResource!0x1654 m_killStats
C_PlayerResource!0x226c m_scoreStats
C_PlayerResource!0x2c80 m_iPing
C_PlayerResource!0x2e84 m_bConnected
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
C_PlayerVehicle!0x0148 m_localOrigin
C_PlayerVehicle!0x1344 m_vehicleDriver
C_PlayerVehicle!0x1360 m_vehicleActivated
C_PlayerVehicle!0x136c m_vehicleLaunchTime
C_PlayerVehicle!0x1374 m_vehicleVelocity
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
C_Projectile!0x1340 m_weaponDataIsSet
C_Projectile!0x1341 m_forceAdjustToGunBarrelDisabled
C_Projectile!0x1344 m_weaponClassIndex
C_Projectile!0x1348 m_destructionDistance
C_Projectile!0x134c m_passThroughDepthTotal
C_Projectile!0x1350 m_modBitfield
C_Projectile!0x1354 m_overrideMods
C_Projectile!0x1358 m_projectileTrailIndex
C_Projectile!0x135c m_impactEffectTable
C_Projectile!0x1360 m_reducedEffects
C_Projectile!0x1364 m_projectileCreationTimeServer
C_Projectile!0x1368 m_weaponSource
C_Projectile!0x1370 m_wpnData
C_Projectile!0x1378 m_hWeaponFileInfo
C_Projectile!0x137c m_weaponChargeLevel
C_Projectile!0x1380 m_modVars
C_Projectile!0x2460 m_modVarsAreValid
C_Projectile!0x2464 m_launchOrigin
C_Projectile!0x2470 m_scriptCB
C_Projectile!0x2498 m_hasPlayedTrailEffect
C_Projectile!0x249c m_projectileLifeTimeEndTick
C_Projectile!0x24a0 m_projectileCreationTime
C_Projectile!0x24a4 m_isVortexRefired
C_Projectile!0x24a5 m_damageAliveOnly
C_Projectile!0x24a6 m_usesPositionFunction
C_Projectile!0x24a8 m_lastCollisionNormal
C_Projectile!0x24b4 m_bounceIndex
C_Projectile!0x24b8 m_randomInt
C_Projectile!0x24bc m_thrownByAI
C_Projectile!0x24c0 m_perPolyRadius
C_Projectile!0x24c8 m_posBeforePhysicsSimulate
C_Projectile!0x24d4 m_hasIgnited
C_Projectile!0x24d5 m_inLagCompensation
C_Projectile!0x24d8 m_passEntities
C_Projectile!0x2540 m_projectileSpeed
C_Projectile!0x2560 m_wantStartTrailEffect
C_Projectile!0x2562 m_hasCalledPostDataUpdate
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
C_PropDoor!0x0148 m_localOrigin
C_PropDoor!0x0154 m_localAngles
C_PropDoor!0x050c m_nNextThinkTick
C_PropDoor!0x13f4 m_angle
C_PropDoor!0x13f8 m_startAngle
C_PropDoor!0x13fc m_startAngleVel
C_PropDoor!0x1400 m_startMoveTime
C_PropDoor!0x140c m_nextHitSoundTime
C_PropDoor!0x1410 m_lastThinkTime
C_PropDoor!0x1458 m_interactingPlayer
C_PropDoor!0x145c m_interactingPlayerWantsOpen
C_PropDoor!0x1460 m_useDebounceEndTime
C_PropDoor!0x1468 m_prevAngle
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
C_Team!0x0a40 m_score
C_Team!0x0a44 m_score2
C_Team!0x0a48 m_kills
C_Team!0x0a4c m_deaths
C_Team!0x0a50 m_iRoundsWon
C_Team!0x0a54 m_iTeamTeamNum
C_Team!0x0a78 m_szTeamname
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
C_TriggerCylinderHeavy!0x0af8 m_teslaTrapObstructedEndTime
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
C_VortexSphere!0x0a40 m_enabled
C_VortexSphere!0x0a44 m_radius
C_VortexSphere!0x0a48 m_height
C_VortexSphere!0x0a4c m_bulletFov
C_VortexSphere!0x0a50 m_bulletAbsorbedCount
C_VortexSphere!0x0a54 m_projectileAbsorbedCount
C_VortexSphere!0x0a58 m_ownerWeapon
C_VortexSphere!0x0a5c m_vortexEffect
C_VortexSphere!0x0a60 m_vortexLocalAngles
C_VortexSphere!0x0a70 m_gunAttachment
C_VortexSphere!0x0a78 m_listPrev
C_VortexSphere!0x0a80 m_listNext
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
C_WallrunCurve!0x0a80 width
C_WallrunCurve!0x0a84 height
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
C_WindowHint!0x0a80 normal
C_WindowHint!0x0a8c right
C_WindowHint!0x0a98 halfSize
C_WindowHint!0x0a98 halfSize[0]
C_WindowHint!0x0a9c halfSize[1]
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
C_Zipline!0x0a40 m_ziplinePhysics
C_Zipline!0x0d94 m_detachEndOnUse
C_Zipline!0x0e80 m_currentFrameZipline.numZiplinePoints
C_Zipline!0x0e84 m_currentFrameZipline.ziplinePositions
C_Zipline!0x0f44 m_currentFrameZipline.ziplinePreviousPositions
C_Zipline!0x1004 m_currentFrameZipline.ziplineDistances
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



default: `"100.0"`  
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
<summary><code>damageinfo_defendInvalidValues</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>damageinfo_defendInvalidValues</code></summary>



default: `"0"`  
flags: `0x2002`  
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
<summary><code>do_trigger_touch_before_spawn</code></summary>

Enables trigger touch testing before dispatch spawn has been called.

default: `"0"`  
flags: `0x2`  
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
<summary><code>fog_volume_find_debug_entindex</code></summary>

If enabled, prints diagnostic information about the current fog volume

default: `"-1"`  
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



default: `"45"`  
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
<summary><code>phys_threadGoWide</code></summary>

Go wide across threads with Physics.

default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>phys_timescale</code></summary>

Scale time for physics

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
<summary><code>player_testSpectateNetcode</code></summary>



default: `"0"`  
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
<summary><code>playlist_variableErrorsChecks</code></summary>



default: `"1"`  
flags: `0x2002`  
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

default: `"1"`  
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

0 - off, 1 - on, 2 - force on in Lobby

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
<summary><code>sv_distanceCull_cellWidth</code></summary>



default: `"1500"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_distanceCull_debug</code></summary>



default: `"0"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_distanceCull_debugPlayerEntindex</code></summary>



default: `"1"`  
flags: `0x2`  
</details>
<details>
<summary><code>sv_distanceCull_largeEntRadius</code></summary>



default: `"150"`  
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
<summary><code>sv_transmitToAllPlayersMask_allBitsSet</code></summary>

This enables the legacy behavior of setting all bits inside of PerPlayerBitMask when we want to transmit an entities to all clients. This includes setting bits to clients that can't even exist(compare GetMaxClients to ABSOLUTE_PLAYER_LIMIT)

default: `"0"`  
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
<summary><code>test_fakeTimeDays</code></summary>

Days worth of seconds that will be added to the result of GetUnixTimestamp(). Server authoritive.

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>test_fakeTimeDays</code></summary>

Days worth of seconds that will be added to the result of GetUnixTimestamp(). Server authoritive.

default: `"0"`  
flags: `0x2002`  
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
r5apex.exe!0x02835a00 ConVar 
r5apex.exe!0x01844c00 ConVar Allow_auto_Party
r5apex.exe!0x01701eb0 ConVar BlendBonesMode
r5apex.exe!0x02776710 ConVar CTeam_DontSave
r5apex.exe!0x0234cf50 ConVar DoorSoundPrefixDouble
r5apex.exe!0x0234f8a0 ConVar DoorSoundPrefixSingle
r5apex.exe!0x023672b0 ConVar ScriptDisallowedToUsePersistenceOnSP
r5apex.exe!0x0235a590 ConVar ScriptSaveAllowed
r5apex.exe!0x01719890 ConVar StreamMicDisabled
r5apex.exe!0x0171a0b0 ConVar TalkIsStream
r5apex.exe!0x01719ed0 ConVar VoiceNeedsReset
r5apex.exe!0x022ea410 ConVar When set to 0, player always returns false when asked if it has a vehicle
r5apex.exe!0x02356f30 ConVar When set to 0, player always returns false when asked if it has a vehicle
r5apex.exe!0x0278baa0 ConVar ai_ainRebuildOnMapStart
r5apex.exe!0x0278e180 ConVar ai_ain_crc_debug
r5apex.exe!0x02370d40 ConVar ai_anim_overlay_debug
r5apex.exe!0x0278cb10 ConVar ai_auto_contact_solver
r5apex.exe!0x0278d3d0 ConVar ai_choose_new_enemy_max_time
r5apex.exe!0x0278f0a0 ConVar ai_cluster_select
r5apex.exe!0x02790050 ConVar ai_collide_other_ai
r5apex.exe!0x0278d290 ConVar ai_current_enemy_bonus
r5apex.exe!0x0278a7a0 ConVar ai_debug_corpse
r5apex.exe!0x0278d4e0 ConVar ai_debug_directnavprobe
r5apex.exe!0x0278cbb0 ConVar ai_debug_doors
r5apex.exe!0x027902d0 ConVar ai_debug_draw_depth_test
r5apex.exe!0x0278b070 ConVar ai_debug_draw_nav_dist
r5apex.exe!0x0278d330 ConVar ai_debug_dyninteractions
r5apex.exe!0x0278fbf0 ConVar ai_debug_efficiency
r5apex.exe!0x0278a5c0 ConVar ai_debug_enemies
r5apex.exe!0x0278bbb0 ConVar ai_debug_enemy_memory
r5apex.exe!0x0278e3a0 ConVar ai_debug_engagement_dist
r5apex.exe!0x0278b6e0 ConVar ai_debug_follow
r5apex.exe!0x0278ad50 ConVar ai_debug_info_node_spectre
r5apex.exe!0x0236f900 ConVar ai_debug_los
r5apex.exe!0x0278de60 ConVar ai_debug_move_script
r5apex.exe!0x0278d150 ConVar ai_debug_move_transitions
r5apex.exe!0x0278fe70 ConVar ai_debug_nodes
r5apex.exe!0x0278c9d0 ConVar ai_debug_obstacle_avoid
r5apex.exe!0x0278d780 ConVar ai_debug_pieoff
r5apex.exe!0x023699b0 ConVar ai_debug_poseparameters
r5apex.exe!0x0278c700 ConVar ai_debug_savePosition
r5apex.exe!0x0278d0b0 ConVar ai_debug_search_paths
r5apex.exe!0x027a13d0 ConVar ai_debug_shoot_positions
r5apex.exe!0x0278dd20 ConVar ai_debug_squads
r5apex.exe!0x0236c930 ConVar ai_debug_stats
r5apex.exe!0x0278d6c0 ConVar ai_debug_test_anim_path
r5apex.exe!0x0278ced0 ConVar ai_debug_think_ticks
r5apex.exe!0x0278b110 ConVar ai_default_efficient
r5apex.exe!0x0278f5a0 ConVar ai_disable_task_announce_attack
r5apex.exe!0x0278e0e0 ConVar ai_draw_motor_movement
r5apex.exe!0x02789570 ConVar ai_efficiency_override
r5apex.exe!0x0278a2a0 ConVar ai_enable_corpse_manager
r5apex.exe!0x0278ae90 ConVar ai_excluded_clusters
r5apex.exe!0x0278a480 ConVar ai_follow_use_points
r5apex.exe!0x0278d580 ConVar ai_follow_use_points_when_moving
r5apex.exe!0x0278e4e0 ConVar ai_frametime_limit
r5apex.exe!0x0278adf0 ConVar ai_grenade_default_weapon
r5apex.exe!0x0278ddc0 ConVar ai_grenade_enabled
r5apex.exe!0x0278b640 ConVar ai_grenade_forced_weapon
r5apex.exe!0x0278a0c0 ConVar ai_grenade_fuse_time
r5apex.exe!0x0278ba00 ConVar ai_grenade_initial_contact_delay
r5apex.exe!0x0278a980 ConVar ai_grenade_max_throw_speed
r5apex.exe!0x02369ce0 ConVar ai_grenade_target_debounce_default
r5apex.exe!0x0278da00 ConVar ai_grenade_target_horizontal_offset
r5apex.exe!0x02790370 ConVar ai_grenade_target_variance_dist_scalar
r5apex.exe!0x02790230 ConVar ai_grenade_target_variance_min
r5apex.exe!0x0278a200 ConVar ai_grenade_throw_debounce
r5apex.exe!0x0278f140 ConVar ai_local_step_size
r5apex.exe!0x0236f010 ConVar ai_max_corpse_detect_dist
r5apex.exe!0x0236ef60 ConVar ai_max_look_at_friendly_dist
r5apex.exe!0x0278e760 ConVar ai_max_node_drop
r5apex.exe!0x0278ccf0 ConVar ai_max_triangulation_attempts
r5apex.exe!0x0278fdd0 ConVar ai_max_triangulation_dist
r5apex.exe!0x02790660 ConVar ai_melee_debug
r5apex.exe!0x0278e220 ConVar ai_melee_kill_sound_radius
r5apex.exe!0x0278d1f0 ConVar ai_min_signal_dist
r5apex.exe!0x02789610 ConVar ai_missFastPlayer_sideWindowYMax
r5apex.exe!0x0278ea80 ConVar ai_missFastPlayer_sideWindowYMin
r5apex.exe!0x0278c5c0 ConVar ai_missFastPlayer_sideWindowZMax
r5apex.exe!0x0278ce30 ConVar ai_missFastPlayer_sideWindowZMin
r5apex.exe!0x0278b5a0 ConVar ai_missFastPlayer_topWindowYMax
r5apex.exe!0x0278b780 ConVar ai_missFastPlayer_topWindowYMin
r5apex.exe!0x027896b0 ConVar ai_missFastPlayer_topWindowZMax
r5apex.exe!0x0278eec0 ConVar ai_missFastPlayer_topWindowZMin
r5apex.exe!0x02789250 ConVar ai_move_do_short_probe
r5apex.exe!0x0278b2f0 ConVar ai_move_probe_delay
r5apex.exe!0x0278daa0 ConVar ai_move_sanity_check
r5apex.exe!0x0278a700 ConVar ai_moveprobe_debug
r5apex.exe!0x0278e620 ConVar ai_moveprobe_jump_debug
r5apex.exe!0x0278b250 ConVar ai_near_node_for_hull_box_extent
r5apex.exe!0x0278cc50 ConVar ai_no_local_ground_paths
r5apex.exe!0x0278a3e0 ConVar ai_no_local_paths
r5apex.exe!0x02789390 ConVar ai_no_node_cache
r5apex.exe!0x0278f500 ConVar ai_no_select_box
r5apex.exe!0x0278ab70 ConVar ai_no_steer
r5apex.exe!0x0278d8c0 ConVar ai_node_draw_safety
r5apex.exe!0x02789930 ConVar ai_node_select
r5apex.exe!0x0278ec60 ConVar ai_pain_death_sound_radius
r5apex.exe!0x0278b1b0 ConVar ai_pain_on_repeat_damage_threshold
r5apex.exe!0x0278bf70 ConVar ai_pain_on_repeated_damage
r5apex.exe!0x027899d0 ConVar ai_path_adjust_speed_on_immediate_turns
r5apex.exe!0x0278a8e0 ConVar ai_path_dangerous_cluster_cost_scalar
r5apex.exe!0x0278bed0 ConVar ai_path_dangerous_cluster_death_time_inc
r5apex.exe!0x02789f80 ConVar ai_path_dangerous_cluster_exclude_dist
r5apex.exe!0x0278a020 ConVar ai_path_dangerous_cluster_look_ahead
r5apex.exe!0x0278bc50 ConVar ai_path_dangerous_cluster_min_time
r5apex.exe!0x0278bcf0 ConVar ai_path_insert_pause_at_est_end
r5apex.exe!0x0278e2c0 ConVar ai_path_insert_pause_at_obstruction
r5apex.exe!0x0278cd90 ConVar ai_physics_shadow
r5apex.exe!0x0278af30 ConVar ai_pos_debug
r5apex.exe!0x0278b8c0 ConVar ai_radial_max_link_dist
r5apex.exe!0x027900f0 ConVar ai_range_attack_twitch_debounce
r5apex.exe!0x02789750 ConVar ai_react_far_dist
r5apex.exe!0x0278b960 ConVar ai_reasonable_facing_min_dist
r5apex.exe!0x0278a840 ConVar ai_rebalance_thinks
r5apex.exe!0x027892f0 ConVar ai_recent_enemy_damage_dist_bonus
r5apex.exe!0x0278b390 ConVar ai_recent_enemy_damage_expire_time
r5apex.exe!0x027891b0 ConVar ai_require_pvs
r5apex.exe!0x0278e8a0 ConVar ai_route_simplify_interval
r5apex.exe!0x027a2ca0 ConVar ai_run_from_enemy_try_shoot_chance
r5apex.exe!0x02790190 ConVar ai_schedule_reset_conditions_on_gather
r5apex.exe!0x0236b890 ConVar ai_schedule_selector_debug
r5apex.exe!0x0278b820 ConVar ai_script_assault_points_validation_debug
r5apex.exe!0x0278a520 ConVar ai_script_nodes_draw
r5apex.exe!0x0278acb0 ConVar ai_shot_bias
r5apex.exe!0x0278f460 ConVar ai_shot_stats_term
r5apex.exe!0x02370110 ConVar ai_show_hull_attacks
r5apex.exe!0x0278a160 ConVar ai_show_path_search_nodes
r5apex.exe!0x0278dc80 ConVar ai_show_think_tolerance
r5apex.exe!0x0236e560 ConVar ai_sight_npc_search_time
r5apex.exe!0x0278dbe0 ConVar ai_simplify_path_dist
r5apex.exe!0x0278fd30 ConVar ai_simplify_path_quick_dist
r5apex.exe!0x0278aa20 ConVar ai_solid_spawn_script_error
r5apex.exe!0x0278e940 ConVar ai_sprint_min_enemy_dist
r5apex.exe!0x0236f860 ConVar ai_squad_cache_path_max_dest_diff
r5apex.exe!0x0236c890 ConVar ai_squad_cache_path_max_start_diff
r5apex.exe!0x0236e950 ConVar ai_squad_clear_assigned_node_time
r5apex.exe!0x0278e6c0 ConVar ai_squad_enemy_notify_delay
r5apex.exe!0x0236df60 ConVar ai_squad_keep_dist_increment
r5apex.exe!0x02370430 ConVar ai_squad_keep_dist_start
r5apex.exe!0x0236fb40 ConVar ai_squad_min_cache_path_length
r5apex.exe!0x0278b4d0 ConVar ai_squad_min_salute_interval
r5apex.exe!0x0278fc90 ConVar ai_squad_move_spread_factor
r5apex.exe!0x0236a0b0 ConVar ai_squad_num_LOFs
r5apex.exe!0x02370a80 ConVar ai_squad_num_chasers
r5apex.exe!0x0236ec70 ConVar ai_squad_stay_close_radius
r5apex.exe!0x027894d0 ConVar ai_stepsize
r5apex.exe!0x0278d820 ConVar ai_strong_optimizations
r5apex.exe!0x0278f280 ConVar ai_strong_optimizations_no_checkstand
r5apex.exe!0x0278b430 ConVar ai_team_enemy_notify_interval
r5apex.exe!0x0278c8c0 ConVar ai_team_enemy_notify_max_dist
r5apex.exe!0x027897f0 ConVar ai_test_hull_model_name
r5apex.exe!0x02790700 ConVar ai_threaded_post_process_is_delayed
r5apex.exe!0x0279dfe0 ConVar ai_titan_grapple_max_len
r5apex.exe!0x0278f1e0 ConVar ai_use_cached_squad_paths
r5apex.exe!0x0278f000 ConVar ai_use_clipped_paths
r5apex.exe!0x0278d960 ConVar ai_use_cluster_path
r5apex.exe!0x0278e9e0 ConVar ai_use_efficiency
r5apex.exe!0x0278df00 ConVar ai_use_frame_think_limits
r5apex.exe!0x0278aac0 ConVar ai_use_think_optimizations
r5apex.exe!0x0234df40 ConVar airslowmo_enabled
r5apex.exe!0x0234e930 ConVar airslowmo_enter_time
r5apex.exe!0x0234a4d0 ConVar airslowmo_ground_immediate_end
r5apex.exe!0x0234c7e0 ConVar airslowmo_leave_time
r5apex.exe!0x0234b5c0 ConVar airslowmo_scripted_speed
r5apex.exe!0x0234feb0 ConVar airslowmo_when_hovering
r5apex.exe!0x0236cf80 ConVar animEvent_debug
r5apex.exe!0x01ef7470 ConVar animEvent_debugEnt
r5apex.exe!0x01f013c0 ConVar animEvent_debug_cl
r5apex.exe!0x02792260 ConVar anim_estimateVelocity
r5apex.exe!0x02792ac0 ConVar anim_playerMovementAngleMargin
r5apex.exe!0x02792700 ConVar anim_player_ragdoll_fix
r5apex.exe!0x0235bdc0 ConVar anim_print_transition_overflow
r5apex.exe!0x027923e0 ConVar anim_runGestureAnimEventsToCompletionOnReset_client
r5apex.exe!0x01edfa40 ConVar anim_showPoseParamErrors
r5apex.exe!0x027925c0 ConVar anim_showstate
r5apex.exe!0x02792660 ConVar anim_showstatelog
r5apex.exe!0x02368d70 ConVar anim_transitionsequences
r5apex.exe!0x02368cd0 ConVar anim_view_entity_third_person_camera_use_move_parent
r5apex.exe!0x01843f10 ConVar announcement
r5apex.exe!0x018444b0 ConVar announcementImage
r5apex.exe!0x018440f0 ConVar announcementVersion
r5apex.exe!0x01716700 ConVar async_serialize
r5apex.exe!0x022e1680 ConVar automantle_backoff_anim_maxfrac
r5apex.exe!0x02333520 ConVar automantle_backoff_anim_maxfrac
r5apex.exe!0x022b23f0 ConVar automantle_cooldown
r5apex.exe!0x0230adf0 ConVar automantle_cooldown
r5apex.exe!0x022dff80 ConVar automantle_dangle_required_space
r5apex.exe!0x023318c0 ConVar automantle_dangle_required_space
r5apex.exe!0x022b4200 ConVar automantle_debug
r5apex.exe!0x0230cf50 ConVar automantle_debug
r5apex.exe!0x022e4d70 ConVar automantle_duration_above
r5apex.exe!0x02347ac0 ConVar automantle_duration_above
r5apex.exe!0x022b4e80 ConVar automantle_duration_below
r5apex.exe!0x0230dfb0 ConVar automantle_duration_below
r5apex.exe!0x022b4aa0 ConVar automantle_duration_high
r5apex.exe!0x0230d7b0 ConVar automantle_duration_high
r5apex.exe!0x022b2220 ConVar automantle_duration_level
r5apex.exe!0x0230a9c0 ConVar automantle_duration_level
r5apex.exe!0x022e1230 ConVar automantle_enable
r5apex.exe!0x02333160 ConVar automantle_enable
r5apex.exe!0x022e2400 ConVar automantle_forwarddist
r5apex.exe!0x023349f0 ConVar automantle_forwarddist
r5apex.exe!0x022e2220 ConVar automantle_gun_enable_height
r5apex.exe!0x02334130 ConVar automantle_gun_enable_height
r5apex.exe!0x022ce470 ConVar automantle_height_above
r5apex.exe!0x023270b0 ConVar automantle_height_above
r5apex.exe!0x022e2c60 ConVar automantle_height_below
r5apex.exe!0x02335300 ConVar automantle_height_below
r5apex.exe!0x022b5810 ConVar automantle_height_level
r5apex.exe!0x0230e3e0 ConVar automantle_height_level
r5apex.exe!0x022b5af0 ConVar automantle_jumpoff_anim_maxfrac
r5apex.exe!0x0230e5f0 ConVar automantle_jumpoff_anim_maxfrac
r5apex.exe!0x022e1190 ConVar automantle_jumpoff_duration
r5apex.exe!0x023330c0 ConVar automantle_jumpoff_duration
r5apex.exe!0x022e3af0 ConVar automantle_max_frac
r5apex.exe!0x02336520 ConVar automantle_max_frac
r5apex.exe!0x022e0110 ConVar automantle_maxangle_push
r5apex.exe!0x02331fb0 ConVar automantle_maxangle_push
r5apex.exe!0x022e22c0 ConVar automantle_maxangle_view
r5apex.exe!0x023341d0 ConVar automantle_maxangle_view
r5apex.exe!0x022b4960 ConVar automantle_min_frac
r5apex.exe!0x0230d670 ConVar automantle_min_frac
r5apex.exe!0x022b5c30 ConVar automantle_mindist
r5apex.exe!0x0230e7d0 ConVar automantle_mindist
r5apex.exe!0x022e2180 ConVar automantle_rest_frac
r5apex.exe!0x02334090 ConVar automantle_rest_frac
r5apex.exe!0x022b5b90 ConVar automantle_rest_frac_below
r5apex.exe!0x0230e730 ConVar automantle_rest_frac_below
r5apex.exe!0x022e4eb0 ConVar automantle_searchdist
r5apex.exe!0x02347d40 ConVar automantle_searchdist
r5apex.exe!0x01eef760 ConVar automantle_view_correction_speed
r5apex.exe!0x01eee3c0 ConVar automantle_view_high_yaw_max
r5apex.exe!0x01ef2190 ConVar automantle_view_pitch_max
r5apex.exe!0x02284a30 ConVar automantle_view_pitch_min
r5apex.exe!0x01eea040 ConVar automantle_view_yaw_max
r5apex.exe!0x022e2040 ConVar automantle_wallrun_maxangle_view
r5apex.exe!0x02333e90 ConVar automantle_wallrun_maxangle_view
r5apex.exe!0x0229db00 ConVar autosprint_type
r5apex.exe!0x02371940 ConVar base_tickinterval_mp
r5apex.exe!0x023e8e50 ConVar base_tickinterval_sp
r5apex.exe!0x022b1fe0 ConVar baseanimatingoverlay_playbackRateThreshold
r5apex.exe!0x0230a7e0 ConVar baseanimatingoverlay_playbackRateThreshold
r5apex.exe!0x01818cc0 ConVar baselines_print
r5apex.exe!0x0236ed10 ConVar bbox_draw_vphysics
r5apex.exe!0x022fe1e0 ConVar bhit_enable
r5apex.exe!0x02796ad0 ConVar bhit_enable
r5apex.exe!0x02302950 ConVar bhit_reliable
r5apex.exe!0x0279bbd0 ConVar bhit_reliable
r5apex.exe!0x0171b150 ConVar bink_materials_enabled
r5apex.exe!0x022ad9b0 ConVar bink_preload_videopanel_movies
r5apex.exe!0x0228e3c0 ConVar boost_jetwash_prediction_factor
r5apex.exe!0x01815970 ConVar bot_lagOut
r5apex.exe!0x02788380 ConVar breakable_disable_gib_limit
r5apex.exe!0x01706c60 ConVar budget_animatingEntities
r5apex.exe!0x01704ce0 ConVar budget_animationOverlayEntities
r5apex.exe!0x017060a0 ConVar budget_combatCharEntities
r5apex.exe!0x01704a80 ConVar budget_weaponEntities
r5apex.exe!0x01706540 ConVar budget_ziplineEntities
r5apex.exe!0x02366760 ConVar bug_reproNum
r5apex.exe!0x01707ac0 ConVar buildcubemaps_async
r5apex.exe!0x017071f0 ConVar buildcubemaps_index
r5apex.exe!0x01705270 ConVar buildcubemaps_pvs_start_early
r5apex.exe!0x017074e0 ConVar buildcubemaps_single_step
r5apex.exe!0x017065e0 ConVar building_cubemaps
r5apex.exe!0x027a0230 ConVar bulletPredictionDebug
r5apex.exe!0x0235ab90 ConVar bullet_trace_test_debug
r5apex.exe!0x0235d140 ConVar bullet_trace_test_enable
r5apex.exe!0x0228e140 ConVar c_dropship_ground_fx_dist_interval
r5apex.exe!0x01ef5e40 ConVar c_dropship_ground_fx_time_interval
r5apex.exe!0x01eef8a0 ConVar c_dropship_rope_debug
r5apex.exe!0x01efe7c0 ConVar c_dropship_rope_events
r5apex.exe!0x02281ee0 ConVar c_dropship_rope_magnitude
r5apex.exe!0x01eed120 ConVar c_dropship_rope_range
r5apex.exe!0x02295480 ConVar c_maxdistance
r5apex.exe!0x02290580 ConVar c_maxpitch
r5apex.exe!0x022a3eb0 ConVar c_maxyaw
r5apex.exe!0x02297050 ConVar c_mindistance
r5apex.exe!0x0229a630 ConVar c_minpitch
r5apex.exe!0x0229c070 ConVar c_minyaw
r5apex.exe!0x022a00f0 ConVar c_orthoheight
r5apex.exe!0x0228fb00 ConVar c_orthowidth
r5apex.exe!0x022a5860 ConVar c_thirdpersonshoulderaimdist
r5apex.exe!0x022a2bf0 ConVar c_thirdpersonshoulderdist
r5apex.exe!0x022a5900 ConVar c_thirdpersonshouldergetsviewpunch
r5apex.exe!0x022a5a40 ConVar c_thirdpersonshoulderheight
r5apex.exe!0x022a59a0 ConVar c_thirdpersonshoulderoffset
r5apex.exe!0x01ef4880 ConVar c_threadedAnimPostData
r5apex.exe!0x022967a0 ConVar cam_collision
r5apex.exe!0x0229d8b0 ConVar cam_idealdelta
r5apex.exe!0x02293760 ConVar cam_idealdist
r5apex.exe!0x0229ffd0 ConVar cam_ideallag
r5apex.exe!0x022a42b0 ConVar cam_idealpitch
r5apex.exe!0x0229fc90 ConVar cam_idealyaw
r5apex.exe!0x02290140 ConVar cam_pitchLock_feetRelative
r5apex.exe!0x02297820 ConVar cam_pitchlock_on
r5apex.exe!0x022a1be0 ConVar cam_pitchlock_period
r5apex.exe!0x02293ea0 ConVar cam_pitchlock_phase
r5apex.exe!0x022a2580 ConVar cam_pitchlock_pitchBase
r5apex.exe!0x02294c90 ConVar cam_pitchlock_pitchRange
r5apex.exe!0x0228f8c0 ConVar cam_pitchlock_pitchWiggleRoom
r5apex.exe!0x022a5680 ConVar cam_player_viewheight_scale
r5apex.exe!0x02295340 ConVar cam_showangles
r5apex.exe!0x022a1c80 ConVar cc_captiontrace
r5apex.exe!0x0229b600 ConVar cc_global_norepeat
r5apex.exe!0x022a4e30 ConVar cc_linger_time
r5apex.exe!0x02299190 ConVar cc_max_duration
r5apex.exe!0x022997b0 ConVar cc_minvisibleitems
r5apex.exe!0x0229d5e0 ConVar cc_predisplay_time
r5apex.exe!0x022a1ea0 ConVar cc_rui
r5apex.exe!0x02297b60 ConVar cc_text_size
r5apex.exe!0x02296100 ConVar cc_timeshift_norepeat
r5apex.exe!0x0171a1f0 ConVar chatroom_console_ptt
r5apex.exe!0x01845400 ConVar chatroom_debug
r5apex.exe!0x01848740 ConVar chatroom_doRealNameLookups
r5apex.exe!0x01848b60 ConVar chatroom_min_status_send_interval
r5apex.exe!0x022f6260 ConVar chatroom_nameLength
r5apex.exe!0x022f6fa0 ConVar chatroom_namePaddingX
r5apex.exe!0x022f6f00 ConVar chatroom_nameWidth
r5apex.exe!0x018194e0 ConVar chatroom_onlyWhenActive
r5apex.exe!0x022f6b40 ConVar chatroom_useSlopSpace
r5apex.exe!0x018487e0 ConVar chatroom_voiceMode
r5apex.exe!0x022f7180 ConVar chatroom_voiceMode
r5apex.exe!0x022aefe0 ConVar cheap_captions_fadetime
r5apex.exe!0x022ae6e0 ConVar cheap_captions_test
r5apex.exe!0x0230d8f0 ConVar checkstuck_nonworld
r5apex.exe!0x0171b290 ConVar chroma_enable
r5apex.exe!0x01ee9a00 ConVar cl_NotifyAllLevelAssetsLoaded_endframe
r5apex.exe!0x02284850 ConVar cl_RunClientConnectScripts_Before_ProcessOnDataChangedEvents
r5apex.exe!0x01ef55c0 ConVar cl_SetupAllBones
r5apex.exe!0x01eef0a0 ConVar cl_ShowBoneSetupEnts
r5apex.exe!0x018143b0 ConVar cl_adjustTimeEntsPerJob
r5apex.exe!0x022b07a0 ConVar cl_aggregate_particles
r5apex.exe!0x01ef4560 ConVar cl_allowABSCalculationDuringSnapshotScriptCalls
r5apex.exe!0x01ee8d20 ConVar cl_allowABSDuringSnapshotScriptCalls
r5apex.exe!0x01eeece0 ConVar cl_allowAnimsToInterpolateBackward
r5apex.exe!0x022b1ae0 ConVar cl_always_draw_3p_player
r5apex.exe!0x01efd630 ConVar cl_always_ragdoll_radius
r5apex.exe!0x022a24e0 ConVar cl_anglespeedkey
r5apex.exe!0x022857b0 ConVar cl_anim_blend_transition_dist
r5apex.exe!0x01ef1a30 ConVar cl_anim_detail_dist
r5apex.exe!0x01ef14a0 ConVar cl_anim_face_dist
r5apex.exe!0x01ef2d50 ConVar cl_anim_sequence_transition_full_weight_optimization
r5apex.exe!0x0228e500 ConVar cl_anim_sounds_seek
r5apex.exe!0x01edf670 ConVar cl_approx_footstep_origin
r5apex.exe!0x022a4610 ConVar cl_approx_tracer_origin
r5apex.exe!0x02282580 ConVar cl_async_bone_setup
r5apex.exe!0x01ef4260 ConVar cl_base_entity_effect_lock
r5apex.exe!0x01ee9c80 ConVar cl_bones_incremental_blend
r5apex.exe!0x01eecea0 ConVar cl_bones_incremental_transform
r5apex.exe!0x01f019e0 ConVar cl_bones_oldhack
r5apex.exe!0x022e4af0 ConVar cl_bounds_show_errors
r5apex.exe!0x022e9b60 ConVar cl_burninggibs
r5apex.exe!0x01707c60 ConVar cl_clock_correction
r5apex.exe!0x01706140 ConVar cl_clock_correction_ahead_correct_interval
r5apex.exe!0x01705060 ConVar cl_clock_correction_behind_correct_interval
r5apex.exe!0x017067c0 ConVar cl_clock_correction_force_server_tick
r5apex.exe!0x017dcca0 ConVar cl_cmdbackup
r5apex.exe!0x017dbd40 ConVar cl_cmdrate
r5apex.exe!0x0170e3c0 ConVar cl_configversion
r5apex.exe!0x0170c250 ConVar cl_configversion_dummy
r5apex.exe!0x022fda90 ConVar cl_cull_weapon_fx
r5apex.exe!0x01717ba0 ConVar cl_dataBlockFragmentPL
r5apex.exe!0x022ef590 ConVar cl_deathhints_enabled
r5apex.exe!0x01eedc00 ConVar cl_debugClientEntities
r5apex.exe!0x022ed6d0 ConVar cl_debug_deferred_trace
r5apex.exe!0x022efa10 ConVar cl_debug_deferred_trace_overlay
r5apex.exe!0x022a22a0 ConVar cl_debug_model_fx_sounds
r5apex.exe!0x022b4820 ConVar cl_decal_alwayswhite
r5apex.exe!0x022e4c30 ConVar cl_decal_backoff
r5apex.exe!0x022b1c20 ConVar cl_deferred_effects
r5apex.exe!0x022ec7f0 ConVar cl_deferred_trace_normal_priority
r5apex.exe!0x01efd590 ConVar cl_demoviewoverride
r5apex.exe!0x02284ad0 ConVar cl_disable_ragdolls
r5apex.exe!0x01ee8c80 ConVar cl_disable_splitscreen_cpu_level_cfgs_in_pip
r5apex.exe!0x0171b510 ConVar cl_disconnectOnTooManySnapshotFrames
r5apex.exe!0x023079b0 ConVar cl_doNetworkAsserts
r5apex.exe!0x0171b470 ConVar cl_doRecreateEnts
r5apex.exe!0x01ee85c0 ConVar cl_draw_player_model
r5apex.exe!0x02284cb0 ConVar cl_drawhud
r5apex.exe!0x022a82a0 ConVar cl_drawmonitors
r5apex.exe!0x01efc1e0 ConVar cl_ejectbrass
r5apex.exe!0x022a6c10 ConVar cl_enable_remote_splitscreen
r5apex.exe!0x0171b8d0 ConVar cl_entCreateDeleteDebug
r5apex.exe!0x01ef1540 ConVar cl_events_ignore_invalidate
r5apex.exe!0x01716ca0 ConVar cl_failremoteconnections
r5apex.exe!0x022a3f50 ConVar cl_fasttempentcollision
r5apex.exe!0x01eedde0 ConVar cl_flip_vis_bits
r5apex.exe!0x0171b650 ConVar cl_flushentitypacket
r5apex.exe!0x02282c30 ConVar cl_footstep_event_max_dist
r5apex.exe!0x01ef6700 ConVar cl_footstep_event_max_dist_titan
r5apex.exe!0x018134b0 ConVar cl_forceAdjustTime
r5apex.exe!0x018496b0 ConVar cl_fovScale
r5apex.exe!0x01849610 ConVar cl_gib_allow
r5apex.exe!0x02282f30 ConVar cl_gib_attack_dir_scale
r5apex.exe!0x01edec60 ConVar cl_gib_lifetime
r5apex.exe!0x022a9bc0 ConVar cl_idealpitchscale
r5apex.exe!0x017dd200 ConVar cl_ignorepackets
r5apex.exe!0x01ef75b0 ConVar cl_interp_all
r5apex.exe!0x01813370 ConVar cl_interpolate
r5apex.exe!0x01ef0920 ConVar cl_interpolate
r5apex.exe!0x01eed860 ConVar cl_interpolateSoAllAnimsLoop
r5apex.exe!0x0228e000 ConVar cl_interpolation_before_prediction
r5apex.exe!0x017192f0 ConVar cl_isUnderAge
r5apex.exe!0x017dd3e0 ConVar cl_is_softened_locale
r5apex.exe!0x022f36f0 ConVar cl_jiggle_bone_debug
r5apex.exe!0x022f35b0 ConVar cl_jiggle_bone_debug_pitch_constraints
r5apex.exe!0x022f3870 ConVar cl_jiggle_bone_debug_yaw_constraints
r5apex.exe!0x022f3650 ConVar cl_jiggle_bone_invert
r5apex.exe!0x022f3790 ConVar cl_jiggle_bone_sanity
r5apex.exe!0x01813190 ConVar cl_keepPersistentDataOnDisconnect
r5apex.exe!0x022abf50 ConVar cl_lagcompensation
r5apex.exe!0x017ddfc0 ConVar cl_language
r5apex.exe!0x0228e960 ConVar cl_leafsystemvis
r5apex.exe!0x02283d50 ConVar cl_lerpIfChildrenLerp
r5apex.exe!0x01717e20 ConVar cl_loadBspFromServerInfo
r5apex.exe!0x01714e10 ConVar cl_loadPostProcessShadersEarly
r5apex.exe!0x017142d0 ConVar cl_loadStaticPropsInJob
r5apex.exe!0x018141d0 ConVar cl_matchmaking_timeout
r5apex.exe!0x01efd8b0 ConVar cl_minimal_rtt_shadows
r5apex.exe!0x0229f890 ConVar cl_model_fx_gib_cull_front_dist
r5apex.exe!0x0229f4b0 ConVar cl_model_fx_gib_cull_radius
r5apex.exe!0x022a2880 ConVar cl_mouseenable
r5apex.exe!0x017ddad0 ConVar cl_move_use_dt
r5apex.exe!0x017dc0a0 ConVar cl_noTimeoutLocalHost
r5apex.exe!0x017de1a0 ConVar cl_overrideEventTimes
r5apex.exe!0x022ad690 ConVar cl_parallelParticlePreDrawWork
r5apex.exe!0x01eeed80 ConVar cl_parallel_clientside_animations
r5apex.exe!0x022f2cd0 ConVar cl_particle_batch_mode
r5apex.exe!0x01849430 ConVar cl_particle_fallback_base
r5apex.exe!0x01849750 ConVar cl_particle_fallback_multiplier
r5apex.exe!0x022af730 ConVar cl_particle_limiter_display_killed
r5apex.exe!0x02309ff0 ConVar cl_particle_limiter_hide_killable
r5apex.exe!0x02790ca0 ConVar cl_particle_limiter_hide_killable
r5apex.exe!0x022a7f80 ConVar cl_particle_limiter_max_particle_count
r5apex.exe!0x022a84f0 ConVar cl_particle_limiter_max_system_count
r5apex.exe!0x022a9ce0 ConVar cl_particle_limiter_min_kill_distance
r5apex.exe!0x022af2b0 ConVar cl_particle_limiter_overlay
r5apex.exe!0x022a94b0 ConVar cl_particle_max_count
r5apex.exe!0x022b0700 ConVar cl_particle_sim_fallback_base_multiplier
r5apex.exe!0x022a66f0 ConVar cl_particle_sim_fallback_threshold_ms
r5apex.exe!0x022adb70 ConVar cl_particle_snoozetime
r5apex.exe!0x022aa240 ConVar cl_particles_show_bbox
r5apex.exe!0x022ab400 ConVar cl_particles_show_controlpoints
r5apex.exe!0x022b1210 ConVar cl_pclass
r5apex.exe!0x022ae1d0 ConVar cl_pdump
r5apex.exe!0x022a6830 ConVar cl_phys_maxticks
r5apex.exe!0x022abe10 ConVar cl_phys_show_active
r5apex.exe!0x022ac610 ConVar cl_phys_timescale
r5apex.exe!0x022afcc0 ConVar cl_physics_invalidate_ents
r5apex.exe!0x022aff20 ConVar cl_physics_maxvelocity
r5apex.exe!0x022ea0e0 ConVar cl_physicsshadowupdate_render
r5apex.exe!0x022a2c90 ConVar cl_pitchspeed
r5apex.exe!0x017dd910 ConVar cl_playback_screenshots
r5apex.exe!0x01ef33f0 ConVar cl_player_fullupdate_predicted_origin_fix
r5apex.exe!0x022e2360 ConVar cl_player_touch_triggers
r5apex.exe!0x0171b6f0 ConVar cl_postSnapshotTransitionBlockCount
r5apex.exe!0x01efda70 ConVar cl_preSnapshotTransitionBlockCount
r5apex.exe!0x022f2e10 ConVar cl_pred_error_verbose
r5apex.exe!0x022aa870 ConVar cl_pred_optimize
r5apex.exe!0x017dd6e0 ConVar cl_predict
r5apex.exe!0x01ede260 ConVar cl_predict_basetoggles
r5apex.exe!0x022a6a50 ConVar cl_predict_cmdlimit
r5apex.exe!0x022b05c0 ConVar cl_predict_error_icon_duration
r5apex.exe!0x022a9590 ConVar cl_predict_error_icon_show
r5apex.exe!0x022aea20 ConVar cl_predict_error_icon_threshold_angle
r5apex.exe!0x022a7750 ConVar cl_predict_error_icon_threshold_dist
r5apex.exe!0x022e6750 ConVar cl_predict_motioncontrol
r5apex.exe!0x022b1b80 ConVar cl_predict_viewangles
r5apex.exe!0x022a76b0 ConVar cl_prediction_error_timestamps
r5apex.exe!0x022ae0f0 ConVar cl_predictionlist
r5apex.exe!0x022aa950 ConVar cl_predictweapons
r5apex.exe!0x022fa680 ConVar cl_prevent_weapon_text_hints
r5apex.exe!0x02283e70 ConVar cl_ragdoll_force_fade_time
r5apex.exe!0x022824e0 ConVar cl_ragdoll_force_fade_time_local_view_player
r5apex.exe!0x022f25e0 ConVar cl_ragdoll_force_fade_time_on_moving_geo
r5apex.exe!0x01efc4e0 ConVar cl_ragdoll_force_fade_time_titan
r5apex.exe!0x01849570 ConVar cl_ragdoll_maxcount
r5apex.exe!0x018497f0 ConVar cl_ragdoll_self_collision
r5apex.exe!0x01813ed0 ConVar cl_replayDelayTolerance
r5apex.exe!0x0228eda0 ConVar cl_requireAnimForAnimEventsHdr
r5apex.exe!0x01813bd0 ConVar cl_resend
r5apex.exe!0x01814010 ConVar cl_resend_timeout
r5apex.exe!0x017dc980 ConVar cl_retire_low_priority_lights
r5apex.exe!0x01ef7510 ConVar cl_runWeaponCloneThinkWhenHidden
r5apex.exe!0x022ef850 ConVar cl_safearea
r5apex.exe!0x017dcdc0 ConVar cl_screenshotname
r5apex.exe!0x022b1d60 ConVar cl_scriptCompileAsync
r5apex.exe!0x022abad0 ConVar cl_script_perf_dump_on_shutdown
r5apex.exe!0x022ed810 ConVar cl_shadowupdatespacing
r5apex.exe!0x01efc6a0 ConVar cl_showClanTags
r5apex.exe!0x022b1e00 ConVar cl_showLoadMovies
r5apex.exe!0x0229f5d0 ConVar cl_show_splashes
r5apex.exe!0x022b1a40 ConVar cl_showerror
r5apex.exe!0x022a78b0 ConVar cl_showerror_watchfield
r5apex.exe!0x02307090 ConVar cl_showfiredbullets
r5apex.exe!0x022ae780 ConVar cl_showfps
r5apex.exe!0x022a9a00 ConVar cl_showfps_altframetime
r5apex.exe!0x022acbf0 ConVar cl_showpausedimage
r5apex.exe!0x022a6310 ConVar cl_showpos
r5apex.exe!0x017162a0 ConVar cl_showsounds
r5apex.exe!0x022aa7d0 ConVar cl_showtime
r5apex.exe!0x01ef3d20 ConVar cl_simulateAllModelsRegardless
r5apex.exe!0x02284d50 ConVar cl_simulationtimefix
r5apex.exe!0x01eee580 ConVar cl_skipAnimEventsOnProps
r5apex.exe!0x022af4b0 ConVar cl_skipfastpath
r5apex.exe!0x0228eee0 ConVar cl_smooth
r5apex.exe!0x01ef9920 ConVar cl_smooth_debug
r5apex.exe!0x02284410 ConVar cl_smoothtime
r5apex.exe!0x02331960 ConVar cl_sticksCountAgainstIdle
r5apex.exe!0x01ee8660 ConVar cl_threaded_bone_setup
r5apex.exe!0x022ce8c0 ConVar cl_updatedirty_async
r5apex.exe!0x01ef1680 ConVar cl_updatedirty_early
r5apex.exe!0x017de060 ConVar cl_updaterate_mp
r5apex.exe!0x022a2b50 ConVar cl_upspeed
r5apex.exe!0x017de100 ConVar cl_useFutureSnapForEvents
r5apex.exe!0x01813a30 ConVar cl_useLobbyTypeForChatroom
r5apex.exe!0x0171b830 ConVar cl_use_calculate_local_player
r5apex.exe!0x01efdb10 ConVar cl_view_cone
r5apex.exe!0x01ef5da0 ConVar cl_view_cone_debug
r5apex.exe!0x01efe8a0 ConVar cl_viewmodel_pre_animate
r5apex.exe!0x022a0c10 ConVar cl_warnAboutSoundsOnInvalidEntities
r5apex.exe!0x0228f9e0 ConVar cl_yawspeed
r5apex.exe!0x0170ef00 ConVar clampHostFrameTimeToOneTick_enable
r5apex.exe!0x01ef17c0 ConVar clearOnAnimChange
r5apex.exe!0x022b1cc0 ConVar client_deferredSnapshotScriptCalls
r5apex.exe!0x0170d9e0 ConVar clientport
r5apex.exe!0x0184e9b0 ConVar cloak_enabled
r5apex.exe!0x01ed3180 ConVar cloak_pilotNoiseFactor
r5apex.exe!0x01ed32c0 ConVar cloak_pilotTint1
r5apex.exe!0x01ed2e60 ConVar cloak_pilotTint2
r5apex.exe!0x01ed2be0 ConVar cloak_pilotTint3
r5apex.exe!0x01706a80 ConVar clock_bias_mp
r5apex.exe!0x01707930 ConVar clock_bias_sp
r5apex.exe!0x01706fe0 ConVar clock_showcorrections
r5apex.exe!0x01707830 ConVar clock_showdebuginfo
r5apex.exe!0x1f7f8970 ConVar closecaption
r5apex.exe!0x022eae70 ConVar cockpitDrift_scalePitch
r5apex.exe!0x022ecfe0 ConVar cockpitDrift_scaleYaw
r5apex.exe!0x022efcc0 ConVar cockpitDrift_speedPitch
r5apex.exe!0x022ea220 ConVar cockpitDrift_speedYaw
r5apex.exe!0x022f9170 ConVar cockpitShake_sourceRollRange
r5apex.exe!0x022fa0d0 ConVar cockpitShake_translateRange
r5apex.exe!0x022f7ea0 ConVar cockpit_damage_chroma_scale
r5apex.exe!0x022f82e0 ConVar cockpit_hit_chroma_max_time
r5apex.exe!0x022f9bc0 ConVar cockpit_hit_chroma_scale
r5apex.exe!0x022f9370 ConVar cockpit_pitch_down_frac
r5apex.exe!0x022fa860 ConVar cockpit_pitch_up_frac
r5apex.exe!0x022f9730 ConVar cockpit_screen_boot_chroma_scale
r5apex.exe!0x022f8dd0 ConVar cockpit_screen_boot_delay_bottom
r5apex.exe!0x022fa900 ConVar cockpit_screen_boot_delay_left
r5apex.exe!0x022f7cc0 ConVar cockpit_screen_boot_delay_mid
r5apex.exe!0x022f9230 ConVar cockpit_screen_boot_delay_right
r5apex.exe!0x022facb0 ConVar cockpit_screen_boot_delay_top
r5apex.exe!0x01814b30 ConVar coll_spatial_entry_limit_client
r5apex.exe!0x01814a90 ConVar coll_spatial_entry_limit_server
r5apex.exe!0x01814bd0 ConVar coll_spatial_optimize_prefetch
r5apex.exe!0x01701f50 ConVar coll_use_bolt_size
r5apex.exe!0x022aab30 ConVar colorblind_mode
r5apex.exe!0x01819080 ConVar communities_doRealNameLookupsForCommunityCreators
r5apex.exe!0x018199e0 ConVar communities_enabled
r5apex.exe!0x01843dd0 ConVar communities_hostname
r5apex.exe!0x018191c0 ConVar community
r5apex.exe!0x01819300 ConVar community_abortCommunitySettingsTime
r5apex.exe!0x01819620 ConVar community_abortUserInfoTime
r5apex.exe!0x018435c0 ConVar community_browse_excludeMine
r5apex.exe!0x01818860 ConVar community_clantags
r5apex.exe!0x018431c0 ConVar community_doRealNameLookupsForInbox
r5apex.exe!0x01819940 ConVar community_frame_run
r5apex.exe!0x01845e00 ConVar community_queryServerWhenOrphaned
r5apex.exe!0x01843120 ConVar community_replaceInboxTokens
r5apex.exe!0x01843920 ConVar community_replaceInboxTokens
r5apex.exe!0x01843010 ConVar community_resolveNames
r5apex.exe!0x01843300 ConVar community_resolveNames
r5apex.exe!0x01819580 ConVar community_send_server_voice
r5apex.exe!0x018433a0 ConVar community_spam
r5apex.exe!0x01819a80 ConVar community_staleCommunitySettingsTime
r5apex.exe!0x018193a0 ConVar community_staleUserInfoTime
r5apex.exe!0x01848290 ConVar con_logfile
r5apex.exe!0x01706900 ConVar con_timestamp
r5apex.exe!0x018516b0 ConVar cpu_level
r5apex.exe!0x01efc000 ConVar cpu_level
r5apex.exe!0x01efe6a0 ConVar createentitydecals
r5apex.exe!0x023e9600 ConVar csm_auto_entity
r5apex.exe!0x018517f0 ConVar csm_cascade_res
r5apex.exe!0x01efc440 ConVar csm_cascade_res
r5apex.exe!0x0184eaf0 ConVar csm_coverage
r5apex.exe!0x022848f0 ConVar csm_culling_use_base_planes
r5apex.exe!0x01ef7c70 ConVar csm_culling_use_exclusion_planes
r5apex.exe!0x01ede1c0 ConVar csm_culling_use_inclusion_planes
r5apex.exe!0x0228ee40 ConVar csm_culling_use_planes
r5apex.exe!0x01eee6c0 ConVar csm_debug_2d
r5apex.exe!0x01ef3ee0 ConVar csm_debug_culling
r5apex.exe!0x022821e0 ConVar csm_debug_vis_hi_range
r5apex.exe!0x01ef3b50 ConVar csm_debug_vis_lo_range
r5apex.exe!0x01ef3c80 ConVar csm_depth_bias
r5apex.exe!0x02284050 ConVar csm_dropsequence_adjusted_coverage
r5apex.exe!0x01ede440 ConVar csm_dropsequence_adjustment
r5apex.exe!0x01851430 ConVar csm_enabled
r5apex.exe!0x01eeaee0 ConVar csm_fadeModels
r5apex.exe!0x01eee4e0 ConVar csm_force_no_csm_in_reflections
r5apex.exe!0x01eddfd0 ConVar csm_frustum_draw
r5apex.exe!0x01eddf30 ConVar csm_frustum_draw_lock
r5apex.exe!0x02281e40 ConVar csm_ignore_cascade12
r5apex.exe!0x01ee9e60 ConVar csm_ignore_edge_planes
r5apex.exe!0x01ef54a0 ConVar csm_ignore_face_planes
r5apex.exe!0x02283cb0 ConVar csm_max_z_offset
r5apex.exe!0x01ef1c10 ConVar csm_min_z_offset
r5apex.exe!0x01ede760 ConVar csm_renderable_shadows
r5apex.exe!0x02283f10 ConVar csm_rope_shadows
r5apex.exe!0x01ef1400 ConVar csm_rot_override
r5apex.exe!0x01ee93e0 ConVar csm_rot_x
r5apex.exe!0x01eece00 ConVar csm_rot_y
r5apex.exe!0x02284710 ConVar csm_shadow_split_lerp_factor_range
r5apex.exe!0x01eeec40 ConVar csm_texel_size_cascade_0
r5apex.exe!0x01ef4420 ConVar csm_texel_size_cascade_1
r5apex.exe!0x01ee9fa0 ConVar csm_texel_size_cascade_2
r5apex.exe!0x01efdec0 ConVar csm_texel_size_cascade_onecascade
r5apex.exe!0x01ee9340 ConVar csm_use_env_light_direction
r5apex.exe!0x01ef7bd0 ConVar csm_world_shadow_meshes
r5apex.exe!0x01efe580 ConVar csm_world_shadows
r5apex.exe!0x01efdbb0 ConVar csm_z_cover_world
r5apex.exe!0x01ef5d00 ConVar csm_z_coverage_jump_height
r5apex.exe!0x0228e6e0 ConVar csm_z_coverage_sea_level
r5apex.exe!0x01844730 ConVar curl_allowHTTPS
r5apex.exe!0x01844910 ConVar curl_preloadDlls
r5apex.exe!0x01844870 ConVar curl_spamAllQueryStates
r5apex.exe!0x232c46e0 ConVar cursorWide
r5apex.exe!0x022fa510 ConVar damageIndicatorReplayTimeOffset
r5apex.exe!0x0236d670 ConVar damage_debug
r5apex.exe!0x022ef970 ConVar damage_indicator_style_pilot
r5apex.exe!0x022ed080 ConVar damage_indicator_style_titan
r5apex.exe!0x022b1f40 ConVar damageinfo_defendInvalidValues
r5apex.exe!0x0230a740 ConVar damageinfo_defendInvalidValues
r5apex.exe!0x02351940 ConVar data_map_do_display
r5apex.exe!0x023575a0 ConVar data_map_do_validate
r5apex.exe!0x0236e430 ConVar death_velocityScale
r5apex.exe!0x01efdc50 ConVar debugFootstepEffects
r5apex.exe!0x01704bc0 ConVar debug_debug_overlay
r5apex.exe!0x023eb130 ConVar debug_draw_all_entity_links
r5apex.exe!0x0230c4b0 ConVar debug_draw_box_depth_test
r5apex.exe!0x018198a0 ConVar debug_force_textRestriction
r5apex.exe!0x01819260 ConVar debug_force_ugcRestriction
r5apex.exe!0x018196c0 ConVar debug_force_voiceRestriction
r5apex.exe!0x01706d00 ConVar debug_map_crc
r5apex.exe!0x0236f270 ConVar debug_overlay_fullposition
r5apex.exe!0x0277eb70 ConVar debug_physimpact
r5apex.exe!0x02356660 ConVar debug_touchlinks
r5apex.exe!0x01955150 ConVar decal_clip_debug_draw
r5apex.exe!0x01955070 ConVar decal_clip_debug_groups
r5apex.exe!0x023ea340 ConVar decalfrequency
r5apex.exe!0x01ede9e0 ConVar delayPostSnapshotNotificationsToAfterInterpolation
r5apex.exe!0x01813db0 ConVar demo_autoRecord
r5apex.exe!0x01813910 ConVar demo_autoRecordName
r5apex.exe!0x022f5d60 ConVar demo_connect_string
r5apex.exe!0x022f5a40 ConVar demo_ui_enable
r5apex.exe!0x02358380 ConVar devStats
r5apex.exe!0x01711980 ConVar developer
r5apex.exe!0x022f9910 ConVar disable_player_use_prompts
r5apex.exe!0x01814810 ConVar discord_largeImage
r5apex.exe!0x01814770 ConVar discord_smallImage
r5apex.exe!0x018148b0 ConVar discord_updatePresence
r5apex.exe!0x017051d0 ConVar dlight_default_falloff
r5apex.exe!0x017dd5a0 ConVar dlight_enable
r5apex.exe!0x017dc300 ConVar dlight_overlay
r5apex.exe!0x0236c9d0 ConVar do_trigger_touch_before_spawn
r5apex.exe!0x02349b70 ConVar dodge_cockpitHack
r5apex.exe!0x02349d70 ConVar dodge_cockpitOffsetMax
r5apex.exe!0x02349ca0 ConVar dodge_cockpitTiltMax
r5apex.exe!0x0234d4d0 ConVar dodge_vertical_enable
r5apex.exe!0x0234dd70 ConVar dodge_vertical_horzspeedscale
r5apex.exe!0x0234bff0 ConVar dodge_vertical_in_air
r5apex.exe!0x0234bb80 ConVar dodge_vertical_threshold
r5apex.exe!0x02349ff0 ConVar dodge_viewTiltDecreaseSpeed
r5apex.exe!0x0234b360 ConVar dodge_viewTiltFalloffTime
r5apex.exe!0x0234a7c0 ConVar dodge_viewTiltIncreaseSpeed
r5apex.exe!0x0234adc0 ConVar dodge_viewTiltMax
r5apex.exe!0x022ad5d0 ConVar dof_enable
r5apex.exe!0x01ed2280 ConVar dof_farDepthEnd
r5apex.exe!0x01ed2320 ConVar dof_farDepthStart
r5apex.exe!0x01ed2460 ConVar dof_monitorFarDepthEnd
r5apex.exe!0x01ed2140 ConVar dof_monitorFarDepthStart
r5apex.exe!0x01ed2000 ConVar dof_monitorNearDepthEnd
r5apex.exe!0x01ed21e0 ConVar dof_monitorNearDepthStart
r5apex.exe!0x01ed23c0 ConVar dof_nearDepthEnd
r5apex.exe!0x01ed20a0 ConVar dof_nearDepthStart
r5apex.exe!0x01ed2500 ConVar dof_overrideParams
r5apex.exe!0x022a93d0 ConVar dof_variable_blur
r5apex.exe!0x01eeae40 ConVar dormant_debug
r5apex.exe!0x022b4be0 ConVar drawBeams
r5apex.exe!0x022f81a0 ConVar draw_target_info_offscreen
r5apex.exe!0x0279e1d0 ConVar dropped_weapon_limit
r5apex.exe!0x01704fc0 ConVar dtwatchclass
r5apex.exe!0x01707ea0 ConVar dtwatchdecode
r5apex.exe!0x017048d0 ConVar dtwatchencode
r5apex.exe!0x01707d80 ConVar dtwatchent
r5apex.exe!0x01704e00 ConVar dtwatchvar
r5apex.exe!0x022e6130 ConVar dump_varsights_calculations
r5apex.exe!0x01eed9a0 ConVar durango_voice_chat_team_only
r5apex.exe!0x01851610 ConVar dvs_enable
r5apex.exe!0x01852470 ConVar dvs_gpuframetime_max
r5apex.exe!0x018525b0 ConVar dvs_gpuframetime_min
r5apex.exe!0x01852510 ConVar dvs_scale_min
r5apex.exe!0x01ed2820 ConVar edge_override_depth
r5apex.exe!0x01ed4260 ConVar edge_override_depth
r5apex.exe!0x01ed28c0 ConVar edge_override_silhouette
r5apex.exe!0x01ed4300 ConVar edge_override_silhouette
r5apex.exe!0x01718400 ConVar enable_KVFileOverrides
r5apex.exe!0x017059b0 ConVar enable_debug_overlays
r5apex.exe!0x0234fa10 ConVar enable_height_based_land_anims
r5apex.exe!0x0234cc10 ConVar enable_height_based_land_anims_titans
r5apex.exe!0x01ef2c10 ConVar enable_skeleton_draw
r5apex.exe!0x01713680 ConVar encrypt_multiKey
r5apex.exe!0x0236c750 ConVar ent_create_debug
r5apex.exe!0x0230df10 ConVar ent_debugkeys
r5apex.exe!0x01ef7b30 ConVar ent_lightweightEnts
r5apex.exe!0x0236fa40 ConVar ent_messages_draw
r5apex.exe!0x01ede620 ConVar ent_repack_almostFull
r5apex.exe!0x01eebec0 ConVar ent_repack_threshhold
r5apex.exe!0x0236ea90 ConVar ent_text_mode
r5apex.exe!0x02371d10 ConVar ent_text_no_player_ents
r5apex.exe!0x02371bc0 ConVar ent_text_only_transmitted_ents
r5apex.exe!0x02370320 ConVar ent_text_pick_type
r5apex.exe!0x0236c490 ConVar ent_text_radius_default
r5apex.exe!0x022ce3d0 ConVar entity_skipRedundantAddEffects
r5apex.exe!0x02327010 ConVar entity_skipRedundantAddEffects
r5apex.exe!0x01813050 ConVar entity_useNetworkFieldBuffer
r5apex.exe!0x01eedca0 ConVar error_if_non_standard_ent_create
r5apex.exe!0x022acef0 ConVar eula_version
r5apex.exe!0x022ab080 ConVar eula_version_accepted
r5apex.exe!0x01ef28f0 ConVar eventseq_debug
r5apex.exe!0x022e61d0 ConVar everything_unlocked
r5apex.exe!0x023516e0 ConVar everything_unlocked
r5apex.exe!0x023ea2a0 ConVar explosion_orientation_debug
r5apex.exe!0x0170fd30 ConVar fakelag_debug
r5apex.exe!0x0235be60 ConVar fast_iteration
r5apex.exe!0x01edbc30 ConVar fast_poly_convert
r5apex.exe!0x027a5330 ConVar fatal_script_error_prompt
r5apex.exe!0x027a51f0 ConVar fatal_script_errors
r5apex.exe!0x027a5290 ConVar fatal_script_errors_client
r5apex.exe!0x027a53d0 ConVar fatal_script_errors_server
r5apex.exe!0x01eed080 ConVar fd_playlist_bits
r5apex.exe!0x0184a2b0 ConVar filesystem_buffer_size
r5apex.exe!0x0184a5d0 ConVar filesystem_max_stdio_read
r5apex.exe!0x0184a670 ConVar filesystem_native
r5apex.exe!0x0184a490 ConVar filesystem_report_buffered_io
r5apex.exe!0x0184a3f0 ConVar filesystem_unbuffered_io
r5apex.exe!0x0184a530 ConVar filesystem_use_overlapped_io
r5apex.exe!0x023509d0 ConVar fire_animevents_overlay_not_active
r5apex.exe!0x0278bd90 ConVar fireteam_catchup_max_speed_scale
r5apex.exe!0x0278ff10 ConVar fireteam_catchup_sprint_dist
r5apex.exe!0x0278f640 ConVar fireteam_cover_search_tolerance
r5apex.exe!0x0278e040 ConVar fireteam_leader_cover_max_speed_threshold
r5apex.exe!0x0278c660 ConVar fireteam_leader_runtime_tolerance
r5apex.exe!0x0278f320 ConVar fireteam_member0_angle
r5apex.exe!0x0278a340 ConVar fireteam_member0_offset_x
r5apex.exe!0x0278eb20 ConVar fireteam_member0_offset_y
r5apex.exe!0x0278be30 ConVar fireteam_member1_angle
r5apex.exe!0x0278e580 ConVar fireteam_member1_offset_x
r5apex.exe!0x0278ef60 ConVar fireteam_member1_offset_y
r5apex.exe!0x0278a660 ConVar fireteam_member2_angle
r5apex.exe!0x0278ac10 ConVar fireteam_member2_offset_x
r5apex.exe!0x0278f3c0 ConVar fireteam_member2_offset_y
r5apex.exe!0x0278ebc0 ConVar fireteam_move_delay
r5apex.exe!0x0278db40 ConVar fireteam_move_tolerance
r5apex.exe!0x02789430 ConVar fireteam_use_cover_hints
r5apex.exe!0x02789890 ConVar fireteam_use_offsets
r5apex.exe!0x02304410 ConVar first_person_bullet_delay
r5apex.exe!0x0279e480 ConVar first_person_bullet_delay
r5apex.exe!0x0228ea00 ConVar first_person_proxy_blend_distance
r5apex.exe!0x0279d9d0 ConVar first_person_proxy_debug
r5apex.exe!0x022aae00 ConVar firsttime_mp_message
r5apex.exe!0x022b19a0 ConVar fog_enable
r5apex.exe!0x01712e80 ConVar fog_enable_water_fog
r5apex.exe!0x022adf60 ConVar fog_enableskybox
r5apex.exe!0x023a8cb0 ConVar fog_volume_find_debug_entindex
r5apex.exe!0x02306ff0 ConVar force3PLaserAttachment
r5apex.exe!0x027a0a00 ConVar force3PLaserAttachment
r5apex.exe!0x01819800 ConVar force_EAAccess
r5apex.exe!0x01718b80 ConVar fps_max
r5apex.exe!0x01717560 ConVar fps_max_use_refresh
r5apex.exe!0x01717c40 ConVar fps_max_vsync
r5apex.exe!0x01ee9dc0 ConVar freecam_swallowButtonInput
r5apex.exe!0x022b2080 ConVar freefall_sound_autoplay_time
r5apex.exe!0x0230a920 ConVar freefall_sound_autoplay_time
r5apex.exe!0x022dfe40 ConVar freefall_sound_height
r5apex.exe!0x023316c0 ConVar freefall_sound_height
r5apex.exe!0x018477e0 ConVar friends_onlineUpdateInterval
r5apex.exe!0x018499d0 ConVar fs_intralevel_reads
r5apex.exe!0x01849ea0 ConVar fs_monitor_read_from_pack
r5apex.exe!0x01849b10 ConVar fs_report_intra_level_readopens
r5apex.exe!0x01849fd0 ConVar fs_report_long_reads
r5apex.exe!0x01849a70 ConVar fs_report_sync_opens
r5apex.exe!0x01849cc0 ConVar fs_report_sync_opens_callstack
r5apex.exe!0x01849e00 ConVar fs_report_sync_opens_fatal
r5apex.exe!0x01849c20 ConVar fs_showAllReads
r5apex.exe!0x0184a740 ConVar fs_vpk_file_open
r5apex.exe!0x0184a180 ConVar fs_warning_mode
r5apex.exe!0x0276b460 ConVar func_break_max_pieces
r5apex.exe!0x023a80c0 ConVar func_break_reduction_factor
r5apex.exe!0x027886a0 ConVar func_breakdmg_bullet
r5apex.exe!0x02788600 ConVar func_breakdmg_club
r5apex.exe!0x02788420 ConVar func_breakdmg_explosive
r5apex.exe!0x0276b500 ConVar fx_debug
r5apex.exe!0x02309760 ConVar fx_deferWorldTraceConstraint
r5apex.exe!0x022969e0 ConVar fx_glass_velocity_cap
r5apex.exe!0x022b03a0 ConVar fx_impact_ally
r5apex.exe!0x022a5e80 ConVar fx_impact_enemy
r5apex.exe!0x022aaea0 ConVar fx_impact_neutral
r5apex.exe!0x02309eb0 ConVar fx_screenspacepass
r5apex.exe!0x02790b60 ConVar fx_screenspacepass
r5apex.exe!0x027864d0 ConVar g_debug_doors
r5apex.exe!0x02394e40 ConVar g_debug_flying_ai
r5apex.exe!0x022f2f50 ConVar g_debug_ragdoll_removal
r5apex.exe!0x0236bee0 ConVar g_debug_trackpather
r5apex.exe!0x02283690 ConVar g_ragdoll_fadespeed
r5apex.exe!0x022f2680 ConVar g_ragdoll_important_maxcount
r5apex.exe!0x01ef6da0 ConVar g_ragdoll_lvfadespeed
r5apex.exe!0x0229a750 ConVar gameCursor_ModeActive
r5apex.exe!0x0229b560 ConVar gameCursor_Velocity
r5apex.exe!0x01efc320 ConVar gamepad_aim_speed
r5apex.exe!0x0228e1e0 ConVar gamepad_aim_speed_ads_0
r5apex.exe!0x01efdf60 ConVar gamepad_aim_speed_ads_1
r5apex.exe!0x01eebf60 ConVar gamepad_aim_speed_ads_2
r5apex.exe!0x01efd950 ConVar gamepad_aim_speed_ads_3
r5apex.exe!0x01ef7150 ConVar gamepad_aim_speed_ads_4
r5apex.exe!0x01ef2ad0 ConVar gamepad_aim_speed_ads_5
r5apex.exe!0x022aafe0 ConVar gamepad_button_layout
r5apex.exe!0x022aa710 ConVar gamepad_buttons_are_southpaw
r5apex.exe!0x01efd6d0 ConVar gamepad_custom_ads_pitch
r5apex.exe!0x01ef2a30 ConVar gamepad_custom_ads_turn_delay
r5apex.exe!0x01edf2b0 ConVar gamepad_custom_ads_turn_pitch
r5apex.exe!0x02284b70 ConVar gamepad_custom_ads_turn_time
r5apex.exe!0x01eea0e0 ConVar gamepad_custom_ads_turn_yaw
r5apex.exe!0x02282d70 ConVar gamepad_custom_ads_yaw
r5apex.exe!0x01ee8dc0 ConVar gamepad_custom_assist_on
r5apex.exe!0x01ef9f50 ConVar gamepad_custom_curve
r5apex.exe!0x01efc140 ConVar gamepad_custom_deadzone_in
r5apex.exe!0x01edee40 ConVar gamepad_custom_deadzone_out
r5apex.exe!0x01ef67a0 ConVar gamepad_custom_enabled
r5apex.exe!0x01eecf40 ConVar gamepad_custom_hip_pitch
r5apex.exe!0x02281f80 ConVar gamepad_custom_hip_turn_delay
r5apex.exe!0x02284370 ConVar gamepad_custom_hip_turn_pitch
r5apex.exe!0x02283110 ConVar gamepad_custom_hip_turn_time
r5apex.exe!0x01efc900 ConVar gamepad_custom_hip_turn_yaw
r5apex.exe!0x01ef70b0 ConVar gamepad_custom_hip_yaw
r5apex.exe!0x022a6e30 ConVar gamepad_custom_pilot
r5apex.exe!0x022a5f20 ConVar gamepad_custom_titan
r5apex.exe!0x01ef6660 ConVar gamepad_deadzone_index_look
r5apex.exe!0x01eeee20 ConVar gamepad_deadzone_index_move
r5apex.exe!0x022a5720 ConVar gamepad_enabled
r5apex.exe!0x01efd450 ConVar gamepad_look_curve
r5apex.exe!0x022ad1d0 ConVar gamepad_stick_layout
r5apex.exe!0x0228f700 ConVar gamepad_toggle_ads
r5apex.exe!0x0229bdf0 ConVar gamepad_togglecrouch_hold
r5apex.exe!0x0184db20 ConVar gamepad_trigger_threshold
r5apex.exe!0x01efd330 ConVar gamepad_use_per_scope_ads_settings
r5apex.exe!0x022af690 ConVar gamepad_use_type
r5apex.exe!0x01715310 ConVar gameui_xbox
r5apex.exe!0x022ac3f0 ConVar gamma_adjusted
r5apex.exe!0x0184dee0 ConVar gfx_desaturate_force
r5apex.exe!0x022b17c0 ConVar gl_clear_color_buffer
r5apex.exe!0x022ab4a0 ConVar gl_clear_fogcolor
r5apex.exe!0x022ac530 ConVar gl_clear_randomcolor
r5apex.exe!0x0276b5a0 ConVar glass_break_required_speed
r5apex.exe!0x023ea8c0 ConVar glass_shatter_attack_speed_scale
r5apex.exe!0x022a4350 ConVar glass_shatter_direction_force_scale
r5apex.exe!0x023e9560 ConVar glass_shatter_drop_speed
r5apex.exe!0x023e9030 ConVar glass_shatter_explosive_scale
r5apex.exe!0x02298400 ConVar glass_shatter_force_scale
r5apex.exe!0x0229d810 ConVar glass_shatter_size_scale
r5apex.exe!0x02296c10 ConVar glass_shatter_use_real_direction
r5apex.exe!0x01ed3b80 ConVar glitch_aberrationScale
r5apex.exe!0x02333480 ConVar globalNonRewindingObject_DontSave
r5apex.exe!0x01ed0590 ConVar global_lighting_partial_update
r5apex.exe!0x02833720 ConVar gpu_count
r5apex.exe!0x01852650 ConVar gpu_level
r5apex.exe!0x0228d9e0 ConVar gpu_level
r5apex.exe!0x0184e370 ConVar gpu_mem_level
r5apex.exe!0x01edf210 ConVar gpu_mem_level
r5apex.exe!0x01852330 ConVar gpu_vram_size_mb
r5apex.exe!0x023496c0 ConVar grapple_accel_human
r5apex.exe!0x02348c20 ConVar grapple_accel_titan
r5apex.exe!0x0234da00 ConVar grapple_around_obstacle_accel
r5apex.exe!0x0234e590 ConVar grapple_autoMantle
r5apex.exe!0x02349800 ConVar grapple_autoMeleeConvergeTime
r5apex.exe!0x02349a10 ConVar grapple_autoMeleeOnDetach
r5apex.exe!0x02349580 ConVar grapple_autoMeleePredict
r5apex.exe!0x02351480 ConVar grapple_autoMeleePredictTime
r5apex.exe!0x023513e0 ConVar grapple_autoMeleeViewRotateSpeedFar
r5apex.exe!0x02351340 ConVar grapple_autoMeleeViewRotateSpeedNear
r5apex.exe!0x0234cd10 ConVar grapple_debug
r5apex.exe!0x02348540 ConVar grapple_decelMeleeStrength
r5apex.exe!0x02349300 ConVar grapple_decel_human
r5apex.exe!0x023489a0 ConVar grapple_decel_titan
r5apex.exe!0x02348ae0 ConVar grapple_detachExtraAllowedLength
r5apex.exe!0x0234a2b0 ConVar grapple_disableMeleeWhenActive
r5apex.exe!0x02348fe0 ConVar grapple_dontFightGravity
r5apex.exe!0x023484a0 ConVar grapple_fallSpeed
r5apex.exe!0x02349620 ConVar grapple_forcedRetractVel
r5apex.exe!0x02348900 ConVar grapple_gracePeriod
r5apex.exe!0x022b5090 ConVar grapple_gravityPushUnderContribution
r5apex.exe!0x0230e190 ConVar grapple_gravityPushUnderContribution
r5apex.exe!0x02348720 ConVar grapple_initialImpulseOffGround_human
r5apex.exe!0x02349760 ConVar grapple_initialImpulseOffGround_human_npc
r5apex.exe!0x02348ea0 ConVar grapple_initialImpulseOffGround_titan
r5apex.exe!0x02348a40 ConVar grapple_initialImpulse_human
r5apex.exe!0x02348680 ConVar grapple_initialImpulse_titan
r5apex.exe!0x02348d60 ConVar grapple_initialSlowFracVert_human
r5apex.exe!0x02348f40 ConVar grapple_initialSlowFracVert_titan
r5apex.exe!0x02349440 ConVar grapple_initialSlowFrac_human
r5apex.exe!0x023493a0 ConVar grapple_initialSlowFrac_titan
r5apex.exe!0x02349260 ConVar grapple_initialSpeedMin_human
r5apex.exe!0x023487c0 ConVar grapple_initialSpeedMin_titan
r5apex.exe!0x022dfcd0 ConVar grapple_jumpFrac
r5apex.exe!0x02331040 ConVar grapple_jumpFrac
r5apex.exe!0x022dfd70 ConVar grapple_letGravityHelpCosAngle
r5apex.exe!0x023310e0 ConVar grapple_letGravityHelpCosAngle
r5apex.exe!0x023491c0 ConVar grapple_lift
r5apex.exe!0x023505d0 ConVar grapple_pullDelay_human
r5apex.exe!0x02350530 ConVar grapple_pullDelay_titan
r5apex.exe!0x023498a0 ConVar grapple_retractVel
r5apex.exe!0x023477a0 ConVar grapple_rodeoVerticalImpulse
r5apex.exe!0x02351520 ConVar grapple_shootVel
r5apex.exe!0x02349120 ConVar grapple_speedRampMax_human
r5apex.exe!0x02349970 ConVar grapple_speedRampMax_titan
r5apex.exe!0x02348b80 ConVar grapple_speedRampMin_human
r5apex.exe!0x02348e00 ConVar grapple_speedRampMin_titan
r5apex.exe!0x023494e0 ConVar grapple_speedRampTime_human
r5apex.exe!0x02348860 ConVar grapple_speedRampTime_titan
r5apex.exe!0x02349080 ConVar grapple_swingAngle
r5apex.exe!0x0234e140 ConVar grapple_swingPullAngle
r5apex.exe!0x02348cc0 ConVar grapple_swingPullSpeedLength
r5apex.exe!0x023485e0 ConVar grapple_swingPullSpeedScale
r5apex.exe!0x023367a0 ConVar grapple_titanEmbarkDist
r5apex.exe!0x0234eb70 ConVar grapple_windowCheckDist
r5apex.exe!0x022ee7d0 ConVar gravity_grenade_decel
r5apex.exe!0x0235bc80 ConVar gravity_grenade_decel
r5apex.exe!0x022e9500 ConVar gravity_grenade_projectile_min_speed
r5apex.exe!0x023558a0 ConVar gravity_grenade_projectile_min_speed
r5apex.exe!0x022b2530 ConVar ground_debug
r5apex.exe!0x0230c410 ConVar ground_debug
r5apex.exe!0x02350ef0 ConVar ground_trace_hull_radius
r5apex.exe!0x0171b040 ConVar grx_hasUnknownItems
r5apex.exe!0x01956160 ConVar gtao_angle_bias
r5apex.exe!0x01955a80 ConVar gtao_intensity
r5apex.exe!0x019563e0 ConVar gtao_thickness_heuristic
r5apex.exe!0x01813730 ConVar hasAnyAssetsWithDiscardedStreamableData
r5apex.exe!0x01814130 ConVar hasMic
r5apex.exe!0x01813690 ConVar hasPartialInstall
r5apex.exe!0x01956200 ConVar hbao_angle_bias
r5apex.exe!0x01955d00 ConVar hbao_intensity
r5apex.exe!0x01955ee0 ConVar hbao_stepsize_random
r5apex.exe!0x01955c60 ConVar hbaobasic_tangent_bias
r5apex.exe!0x02396b20 ConVar hibernation_assumed_max_player_speed
r5apex.exe!0x023e99d0 ConVar hibernation_debounce_dist
r5apex.exe!0x02373d20 ConVar hibernation_enable
r5apex.exe!0x023e96a0 ConVar hibernation_far_dist
r5apex.exe!0x023e8ef0 ConVar hibernation_medium_dist
r5apex.exe!0x023a75f0 ConVar hibernation_min_reevaluate_time
r5apex.exe!0x023e9250 ConVar hibernation_near_dist
r5apex.exe!0x022a2e50 ConVar hidehud
r5apex.exe!0x02779d80 ConVar high_perf_dev_server
r5apex.exe!0x01ef3a10 ConVar highlight_deferred_update
r5apex.exe!0x019551f0 ConVar highlight_draw
r5apex.exe!0x019553a0 ConVar highlight_lazy_clear_buffers
r5apex.exe!0x01955300 ConVar highlight_object_max_count
r5apex.exe!0x01702910 ConVar hitbox_bodygroup_check
r5apex.exe!0x022abeb0 ConVar hitch_alert_active
r5apex.exe!0x022ab240 ConVar hitch_alert_color
r5apex.exe!0x022a8340 ConVar hitch_alert_show_large_snapshots
r5apex.exe!0x01713a40 ConVar host_RunFrameServerAlways
r5apex.exe!0x01711170 ConVar host_ShowIPCCallCount
r5apex.exe!0x018160f0 ConVar host_flush_threshold
r5apex.exe!0x01711c00 ConVar host_framerate
r5apex.exe!0x0170e820 ConVar host_limitlocal
r5apex.exe!0x017091c0 ConVar host_map
r5apex.exe!0x0170da80 ConVar host_preload_shaders
r5apex.exe!0x01713c90 ConVar host_print_frame_times
r5apex.exe!0x0170cca0 ConVar host_profile
r5apex.exe!0x017109b0 ConVar host_runframe_input_parcelremainder
r5apex.exe!0x0170ee60 ConVar host_server_thread_min_ticks
r5apex.exe!0x01708e00 ConVar host_sleep
r5apex.exe!0x0170b540 ConVar host_speeds
r5apex.exe!0x017dcfa0 ConVar host_syncfps
r5apex.exe!0x0170c5e0 ConVar host_thread_join_fast
r5apex.exe!0x017137c0 ConVar host_thread_mode
r5apex.exe!0x0170be10 ConVar host_threaded_sound
r5apex.exe!0x01714eb0 ConVar host_timescale
r5apex.exe!0x0170c6f0 ConVar hostip
r5apex.exe!0x0170d3f0 ConVar hostname
r5apex.exe!0x01709120 ConVar hostport
r5apex.exe!0x01844550 ConVar http_StryderKey
r5apex.exe!0x01843fb0 ConVar http_debug
r5apex.exe!0x01844410 ConVar http_debug_forceFailRate
r5apex.exe!0x01844230 ConVar http_debug_forceFailStatus
r5apex.exe!0x01844050 ConVar http_failuresAsErrors
r5apex.exe!0x01844370 ConVar http_maxAllocateAttempts
r5apex.exe!0x018447d0 ConVar http_recv_fail_realloc
r5apex.exe!0x018442d0 ConVar http_sandbox
r5apex.exe!0x01844190 ConVar http_showQueries
r5apex.exe!0x02308930 ConVar hud_autoreloadscript
r5apex.exe!0x022aeac0 ConVar hud_setting_accessibleChat
r5apex.exe!0x022a7ae0 ConVar hud_setting_adsDof
r5apex.exe!0x022f7fc0 ConVar hud_setting_compactOverHeadNames
r5apex.exe!0x022af8e0 ConVar hud_setting_damageIndicatorStyle
r5apex.exe!0x022aa9f0 ConVar hud_setting_damageTextStyle
r5apex.exe!0x022a61a0 ConVar hud_setting_enableModWheel
r5apex.exe!0x022a7390 ConVar hud_setting_healthUseOnHold
r5apex.exe!0x022a7ec0 ConVar hud_setting_healthWheelToggle
r5apex.exe!0x022a8750 ConVar hud_setting_healthWheelUseOnRelease
r5apex.exe!0x022a9f60 ConVar hud_setting_lootPromptStyle
r5apex.exe!0x022aa380 ConVar hud_setting_minimapRotate
r5apex.exe!0x022a6b30 ConVar hud_setting_ordnanceUseOnHold
r5apex.exe!0x022ad870 ConVar hud_setting_ordnanceWheelToggle
r5apex.exe!0x022a6f50 ConVar hud_setting_ordnanceWheelUseOnRelease
r5apex.exe!0x022af7d0 ConVar hud_setting_pingAlpha
r5apex.exe!0x022ab6c0 ConVar hud_setting_pingDoubleTapEnemy
r5apex.exe!0x022a9000 ConVar hud_setting_pingWheelToggle
r5apex.exe!0x022a6790 ConVar hud_setting_showButtonHints
r5apex.exe!0x022af210 ConVar hud_setting_showCallsigns
r5apex.exe!0x022adec0 ConVar hud_setting_showLevelUp
r5apex.exe!0x022ac6d0 ConVar hud_setting_showMedals
r5apex.exe!0x022b0660 ConVar hud_setting_showMeter
r5apex.exe!0x022ac950 ConVar hud_setting_showObituary
r5apex.exe!0x022a8590 ConVar hud_setting_showTips
r5apex.exe!0x022b0100 ConVar hud_setting_showWeaponFlyouts
r5apex.exe!0x022b01c0 ConVar hud_setting_streamerMode
r5apex.exe!0x0230c550 ConVar hudchat_dead_can_only_talk_to_other_dead
r5apex.exe!0x02296500 ConVar hudchat_new_message_fade_duration
r5apex.exe!0x0229d950 ConVar hudchat_new_message_shown_duration
r5apex.exe!0x02296df0 ConVar hudchat_play_text_to_speech
r5apex.exe!0x02296340 ConVar hudchat_transition_message_mode_fade_duration
r5apex.exe!0x022953e0 ConVar hudchat_visibility
r5apex.exe!0x01ed9b80 ConVar hudwarp_chopsize
r5apex.exe!0x01ed9860 ConVar hudwarp_override
r5apex.exe!0x01ed9720 ConVar hudwarp_viewDist
r5apex.exe!0x01ed9ae0 ConVar hudwarp_xScale
r5apex.exe!0x01ed87c0 ConVar hudwarp_xWarp
r5apex.exe!0x01ed8860 ConVar hudwarp_yScale
r5apex.exe!0x01ed97c0 ConVar hudwarp_yWarp
r5apex.exe!0x022a7950 ConVar idcolor_ally
r5apex.exe!0x022b08e0 ConVar idcolor_ally_cb1
r5apex.exe!0x022ae050 ConVar idcolor_ally_cb2
r5apex.exe!0x022ae820 ConVar idcolor_ally_cb3
r5apex.exe!0x022b1550 ConVar idcolor_enemy
r5apex.exe!0x022ad390 ConVar idcolor_enemy_cb1
r5apex.exe!0x022acab0 ConVar idcolor_enemy_cb2
r5apex.exe!0x022a6270 ConVar idcolor_enemy_cb3
r5apex.exe!0x022a7e20 ConVar idcolor_neutral
r5apex.exe!0x02394f80 ConVar idleKickTime_min_alive_seconds
r5apex.exe!0x02373dc0 ConVar idleKickTime_minutes
r5apex.exe!0x023e9b10 ConVar idleKickTime_party_minutes
r5apex.exe!0x023961d0 ConVar idleKickTime_privatematch_game_minutes
r5apex.exe!0x023719e0 ConVar idleKickTime_privatematch_lobby_minutes
r5apex.exe!0x02371c70 ConVar idleKickTime_training_minutes
r5apex.exe!0x02350cc0 ConVar ik_debug
r5apex.exe!0x0234c160 ConVar ik_debug_chain
r5apex.exe!0x0234de70 ConVar ik_debug_ent
r5apex.exe!0x02350220 ConVar ik_debug_text
r5apex.exe!0x0234c230 ConVar ik_enable
r5apex.exe!0x022dfc30 ConVar ik_enable_client
r5apex.exe!0x0232fb20 ConVar ik_enable_server
r5apex.exe!0x02336840 ConVar ik_height_adjust
r5apex.exe!0x0230d990 ConVar ik_height_adjust_debug
r5apex.exe!0x0230c9a0 ConVar ik_height_adjust_move_speed
r5apex.exe!0x0230ad50 ConVar ik_height_adjust_sine
r5apex.exe!0x023478e0 ConVar ik_height_adjust_speed
r5apex.exe!0x0234f3f0 ConVar ik_latch
r5apex.exe!0x0234be20 ConVar ik_normal_lerp_rate
r5apex.exe!0x0234d220 ConVar ik_unlatch_max_rate
r5apex.exe!0x0184c010 ConVar ime_enabled
r5apex.exe!0x01ed7ab0 ConVar imgui_buildmode
r5apex.exe!0x01ed7b80 ConVar imgui_buildmode
r5apex.exe!0x022a0db0 ConVar impact_allow
r5apex.exe!0x022ac9f0 ConVar impact_debug_info
r5apex.exe!0x022a2db0 ConVar impact_victim_offset_dist
r5apex.exe!0x022e6530 ConVar impulse_low_decel_duration_scalar
r5apex.exe!0x023519e0 ConVar impulse_low_decel_duration_scalar
r5apex.exe!0x01814270 ConVar inPartyChat
r5apex.exe!0x0170f830 ConVar in_forceuser
r5apex.exe!0x01718140 ConVar in_syncRT
r5apex.exe!0x0228f240 ConVar in_usekeyboardsampletime
r5apex.exe!0x01819760 ConVar inbox_enabled
r5apex.exe!0x027767b0 ConVar info_spawnpoint_human_classname
r5apex.exe!0x02786e70 ConVar info_spawnpoint_titan_classname
r5apex.exe!0x01819440 ConVar infoblock_requestInterval
r5apex.exe!0x01eedd40 ConVar interpolate_on_parent_change
r5apex.exe!0x022a8630 ConVar intro_viewed
r5apex.exe!0x01710b90 ConVar ip
r5apex.exe!0x022a1a40 ConVar joy_advaxisr
r5apex.exe!0x02290260 ConVar joy_advaxisu
r5apex.exe!0x022a4570 ConVar joy_advaxisv
r5apex.exe!0x0229c240 ConVar joy_advaxisx
r5apex.exe!0x0229f070 ConVar joy_advaxisy
r5apex.exe!0x0229be90 ConVar joy_advaxisz
r5apex.exe!0x022a37b0 ConVar joy_inverty
r5apex.exe!0x022a5b80 ConVar joy_legacy
r5apex.exe!0x022970f0 ConVar joy_movement_stick
r5apex.exe!0x0228f460 ConVar joy_requireFocus
r5apex.exe!0x0229faf0 ConVar joy_rumble
r5apex.exe!0x022a1610 ConVar joy_xcontroller_cfg_loaded
r5apex.exe!0x017dc4c0 ConVar jpeg_quality
r5apex.exe!0x01ed4bb0 ConVar jt_help_with_anything_ignore_preference
r5apex.exe!0x022e3e10 ConVar jump_graceperiod
r5apex.exe!0x02336ac0 ConVar jump_graceperiod
r5apex.exe!0x022e2a40 ConVar jump_keyboardgrace_max
r5apex.exe!0x023350e0 ConVar jump_keyboardgrace_max
r5apex.exe!0x022ce330 ConVar jump_keyboardgrace_strength
r5apex.exe!0x02326f70 ConVar jump_keyboardgrace_strength
r5apex.exe!0x022b4ff0 ConVar jump_keyboardgraceperiodmax
r5apex.exe!0x0230e0f0 ConVar jump_keyboardgraceperiodmax
r5apex.exe!0x022e2b80 ConVar jump_keyboardgraceperiodmin
r5apex.exe!0x02335220 ConVar jump_keyboardgraceperiodmin
r5apex.exe!0x01815290 ConVar killReplay_lagCompensate
r5apex.exe!0x022fa390 ConVar killReplay_playNonReplayRemoteCallsOnLocalClientPlayer
r5apex.exe!0x02783c10 ConVar lagcompensation_debug_ent
r5apex.exe!0x0276b780 ConVar lagcompensation_ignore_friendlies
r5apex.exe!0x01efca70 ConVar leaf_threadedRecompute
r5apex.exe!0x01ef6440 ConVar leaf_threadedRecompute_batchSize
r5apex.exe!0x027956d0 ConVar leech_lagcompensate
r5apex.exe!0x0279dd60 ConVar leech_npc_angle_cos
r5apex.exe!0x01ef9800 ConVar lerp_careAboutAttachmentBonePosition
r5apex.exe!0x0236a2d0 ConVar lerp_debugEnt
r5apex.exe!0x0236ebd0 ConVar lerp_debugEnt_server
r5apex.exe!0x01f012a0 ConVar lerp_opt
r5apex.exe!0x01edf350 ConVar lerp_threaded
r5apex.exe!0x01ef8bb0 ConVar lerp_threaded_numEntsPerTask
r5apex.exe!0x0170a050 ConVar light_maxcone
r5apex.exe!0x0184ec30 ConVar lightmap_realtimelight
r5apex.exe!0x01852150 ConVar lightmap_realtimeshadows
r5apex.exe!0x01710f90 ConVar load_during_video
r5apex.exe!0x022f6be0 ConVar loaderrorsCount
r5apex.exe!0x022f6580 ConVar loaderrorsNeedShown
r5apex.exe!0x022f70e0 ConVar localClientPlayerCachedLevel
r5apex.exe!0x017ddeb0 ConVar locationInfo
r5apex.exe!0x017dde10 ConVar locationInfo_nucleus
r5apex.exe!0x022955c0 ConVar locator_background_border_color
r5apex.exe!0x0228f120 ConVar locator_background_border_thickness
r5apex.exe!0x022979c0 ConVar locator_background_color
r5apex.exe!0x02297c00 ConVar locator_background_shift_x
r5apex.exe!0x02298a10 ConVar locator_background_shift_y
r5apex.exe!0x0229bfd0 ConVar locator_background_style
r5apex.exe!0x022988f0 ConVar locator_background_thickness_x
r5apex.exe!0x0228fe80 ConVar locator_background_thickness_y
r5apex.exe!0x022968c0 ConVar locator_fade_time
r5apex.exe!0x022a3690 ConVar locator_icon_max_size_non_ss
r5apex.exe!0x0229c840 ConVar locator_icon_min_size_non_ss
r5apex.exe!0x02296a80 ConVar locator_lerp_rest
r5apex.exe!0x022992b0 ConVar locator_lerp_speed
r5apex.exe!0x0229b4c0 ConVar locator_lerp_time
r5apex.exe!0x0229f770 ConVar locator_pulse_time
r5apex.exe!0x0228f820 ConVar locator_split_len
r5apex.exe!0x02299580 ConVar locator_split_maxwide_percent
r5apex.exe!0x0229f110 ConVar locator_start_at_crosshair
r5apex.exe!0x0229d4c0 ConVar locator_target_offset_x
r5apex.exe!0x02294bf0 ConVar locator_target_offset_y
r5apex.exe!0x022a1f40 ConVar locator_topdown_style
r5apex.exe!0x0229dba0 ConVar lookspring
r5apex.exe!0x022a2740 ConVar lookstrafe
r5apex.exe!0x02298520 ConVar m_acceleration
r5apex.exe!0x022a2620 ConVar m_forward
r5apex.exe!0x02296d50 ConVar m_invert_pitch
r5apex.exe!0x0229c1a0 ConVar m_side
r5apex.exe!0x022f6880 ConVar mainmenu_background_movie
r5apex.exe!0x022a18d0 ConVar map_settings_override
r5apex.exe!0x017126b0 ConVar map_wants_save_disable
r5apex.exe!0x01ed0810 ConVar mat_autoexposure_force_value
r5apex.exe!0x022aeb60 ConVar mat_autoexposure_max
r5apex.exe!0x022ab760 ConVar mat_autoexposure_max_multiplier
r5apex.exe!0x022b0300 ConVar mat_autoexposure_min
r5apex.exe!0x022ac490 ConVar mat_autoexposure_min_multiplier
r5apex.exe!0x022a8f60 ConVar mat_autoexposure_override_min_max
r5apex.exe!0x022a9630 ConVar mat_autoexposure_speed
r5apex.exe!0x022b1170 ConVar mat_autoexposure_uncap
r5apex.exe!0x022afe80 ConVar mat_bloom_cutoff
r5apex.exe!0x01ed0950 ConVar mat_bloom_max_lighting_value
r5apex.exe!0x022a7c60 ConVar mat_bloom_scalefactor_scalar
r5apex.exe!0x01ed2a00 ConVar mat_bloom_streak_amount
r5apex.exe!0x022ad730 ConVar mat_bloom_streak_cutoff
r5apex.exe!0x022ac810 ConVar mat_bloom_streak_cutoff_exposure_adapt
r5apex.exe!0x01ed3a40 ConVar mat_bloom_streak_exponent_post
r5apex.exe!0x022abd70 ConVar mat_bloom_streak_exponent_pre
r5apex.exe!0x01ed3540 ConVar mat_bloom_wide_amount
r5apex.exe!0x022ad530 ConVar mat_bloom_wide_exponent_pre
r5apex.exe!0x022afae0 ConVar mat_bloomamount_rate
r5apex.exe!0x022b1900 ConVar mat_bloomscale
r5apex.exe!0x01956d50 ConVar mat_checkStalls
r5apex.exe!0x0184ef50 ConVar mat_cloudmask
r5apex.exe!0x01706ea0 ConVar mat_colcorrection_disableentities
r5apex.exe!0x01814950 ConVar mat_colcorrection_disableentities
r5apex.exe!0x02281da0 ConVar mat_colcorrection_disableentities
r5apex.exe!0x01705870 ConVar mat_colcorrection_editor
r5apex.exe!0x01ef1b70 ConVar mat_colcorrection_editor
r5apex.exe!0x01ef27b0 ConVar mat_colcorrection_forceentitiesclientside
r5apex.exe!0x01705f90 ConVar mat_colorcorrection
r5apex.exe!0x01ed3ae0 ConVar mat_debug_postprocess_allowed
r5apex.exe!0x022adc90 ConVar mat_debug_postprocessing_effects
r5apex.exe!0x01ed3860 ConVar mat_debug_tonemapping
r5apex.exe!0x01ed3360 ConVar mat_debug_tonemapping_disable
r5apex.exe!0x01ed3220 ConVar mat_debug_tonemapping_mid1
r5apex.exe!0x01ed2f00 ConVar mat_debug_tonemapping_mid2
r5apex.exe!0x01ed30e0 ConVar mat_debug_tonemapping_shoulder
r5apex.exe!0x01ed3040 ConVar mat_debug_tonemapping_toe
r5apex.exe!0x018523d0 ConVar mat_debugalttab
r5apex.exe!0x01ed14a0 ConVar mat_depthbias_decal
r5apex.exe!0x01ed1680 ConVar mat_depthbias_normal
r5apex.exe!0x01ed1360 ConVar mat_depthbias_shadowmap
r5apex.exe!0x01ed1540 ConVar mat_depthbias_tightshadowmap
r5apex.exe!0x01ed1040 ConVar mat_depthbias_ui
r5apex.exe!0x01ed15e0 ConVar mat_depthbias_zfill
r5apex.exe!0x01ed0f00 ConVar mat_depthbiasclamp_decal
r5apex.exe!0x01ed1900 ConVar mat_depthbiasclamp_normal
r5apex.exe!0x01ed10e0 ConVar mat_depthbiasclamp_shadowmap
r5apex.exe!0x01ed12c0 ConVar mat_depthbiasclamp_ui
r5apex.exe!0x01ed1220 ConVar mat_depthbiasclamp_zfill
r5apex.exe!0x01ed0fa0 ConVar mat_depthtest_force_disabled
r5apex.exe!0x01851f70 ConVar mat_detail_tex
r5apex.exe!0x0184e190 ConVar mat_diffuse
r5apex.exe!0x022ad130 ConVar mat_disable_bloom
r5apex.exe!0x01ed0770 ConVar mat_disable_lightmap_ambient
r5apex.exe!0x01851250 ConVar mat_disable_lightmaps
r5apex.exe!0x0184e730 ConVar mat_disable_model_ambient
r5apex.exe!0x01716480 ConVar mat_drawMenuGrid
r5apex.exe!0x01718c20 ConVar mat_drawTitleSafe
r5apex.exe!0x0184e410 ConVar mat_drawflat
r5apex.exe!0x0184eeb0 ConVar mat_dxlevel
r5apex.exe!0x01705910 ConVar mat_dynamic_tonemapping
r5apex.exe!0x01852790 ConVar mat_dynamic_tonemapping
r5apex.exe!0x0184df80 ConVar mat_enable_ssr
r5apex.exe!0x01ed09f0 ConVar mat_envmap_scale
r5apex.exe!0x01704b20 ConVar mat_envmaptgasize
r5apex.exe!0x01851a70 ConVar mat_fastnobump
r5apex.exe!0x017112b0 ConVar mat_fastspecular
r5apex.exe!0x01851e30 ConVar mat_filterlightmaps
r5apex.exe!0x0184e550 ConVar mat_filtertextures
r5apex.exe!0x022a8430 ConVar mat_force_bloom
r5apex.exe!0x018519d0 ConVar mat_forceaniso
r5apex.exe!0x022a8ec0 ConVar mat_frame_color_bias
r5apex.exe!0x022ab140 ConVar mat_frame_color_enabled
r5apex.exe!0x022b12b0 ConVar mat_frame_color_scale
r5apex.exe!0x022a9870 ConVar mat_frame_color_spot_metering_screen_ratio
r5apex.exe!0x0170f2c0 ConVar mat_fullbright
r5apex.exe!0x01ed3680 ConVar mat_fxaa_enable
r5apex.exe!0x0184e0f0 ConVar mat_global_lighting
r5apex.exe!0x01957070 ConVar mat_global_lighting
r5apex.exe!0x022a6060 ConVar mat_global_lighting
r5apex.exe!0x0170e960 ConVar mat_hdr_level
r5apex.exe!0x018149f0 ConVar mat_hdrcolcorrection_editor
r5apex.exe!0x0184ddb0 ConVar mat_hdrcolorcorrection
r5apex.exe!0x019571c0 ConVar mat_hide_sun_in_last_cascade
r5apex.exe!0x01957120 ConVar mat_instancing
r5apex.exe!0x01956cb0 ConVar mat_letterbox_aspect_goal
r5apex.exe!0x01956c10 ConVar mat_letterbox_aspect_threshold
r5apex.exe!0x022a72d0 ConVar mat_lightcull_subview
r5apex.exe!0x022aa000 ConVar mat_lightcull_subviews
r5apex.exe!0x01ed3900 ConVar mat_local_contrast_edge_scale_override
r5apex.exe!0x01ed2c80 ConVar mat_local_contrast_midtone_mask_override
r5apex.exe!0x01ed2b40 ConVar mat_local_contrast_scale_override
r5apex.exe!0x01ed2d20 ConVar mat_local_contrast_vignette_end_override
r5apex.exe!0x01ed2dc0 ConVar mat_local_contrast_vignette_start_override
r5apex.exe!0x01853780 ConVar mat_materialmip_character_0
r5apex.exe!0x01852ab0 ConVar mat_materialmip_character_1
r5apex.exe!0x01852e70 ConVar mat_materialmip_character_2
r5apex.exe!0x01853230 ConVar mat_materialmip_character_3
r5apex.exe!0x01853190 ConVar mat_materialmip_character_4
r5apex.exe!0x01853500 ConVar mat_materialmip_cockpit_0
r5apex.exe!0x01852c90 ConVar mat_materialmip_cockpit_1
r5apex.exe!0x018528d0 ConVar mat_materialmip_cockpit_2
r5apex.exe!0x01852a10 ConVar mat_materialmip_cockpit_3
r5apex.exe!0x01853640 ConVar mat_materialmip_cockpit_4
r5apex.exe!0x01852970 ConVar mat_materialmip_model_0
r5apex.exe!0x01852fb0 ConVar mat_materialmip_model_1
r5apex.exe!0x018536e0 ConVar mat_materialmip_model_2
r5apex.exe!0x01853a00 ConVar mat_materialmip_model_3
r5apex.exe!0x018530f0 ConVar mat_materialmip_model_4
r5apex.exe!0x018535a0 ConVar mat_materialmip_other_0
r5apex.exe!0x01852dd0 ConVar mat_materialmip_other_1
r5apex.exe!0x01853aa0 ConVar mat_materialmip_other_2
r5apex.exe!0x01852b50 ConVar mat_materialmip_other_3
r5apex.exe!0x018538c0 ConVar mat_materialmip_other_4
r5apex.exe!0x01853050 ConVar mat_materialmip_world_0
r5apex.exe!0x01852830 ConVar mat_materialmip_world_1
r5apex.exe!0x01852bf0 ConVar mat_materialmip_world_2
r5apex.exe!0x01853820 ConVar mat_materialmip_world_3
r5apex.exe!0x01852d30 ConVar mat_materialmip_world_4
r5apex.exe!0x0170b240 ConVar mat_maxframelatency
r5apex.exe!0x0184ed70 ConVar mat_mip_linear
r5apex.exe!0x0184eff0 ConVar mat_mipmaptextures
r5apex.exe!0x017089c0 ConVar mat_norendering
r5apex.exe!0x0184e230 ConVar mat_norendering
r5apex.exe!0x01851570 ConVar mat_phong
r5apex.exe!0x0184e7d0 ConVar mat_picmip
r5apex.exe!0x01ed35e0 ConVar mat_postprocess_enable
r5apex.exe!0x022aed40 ConVar mat_postprocess_enable
r5apex.exe!0x01953c20 ConVar mat_processtoolvars
r5apex.exe!0x01851c50 ConVar mat_proxy
r5apex.exe!0x01851cf0 ConVar mat_reducefillrate
r5apex.exe!0x01853b40 ConVar mat_reduceparticles
r5apex.exe!0x01ed0e60 ConVar mat_remoteshadercompile
r5apex.exe!0x018526f0 ConVar mat_report_queue_status
r5apex.exe!0x01851110 ConVar mat_reversedepth
r5apex.exe!0x022a8d30 ConVar mat_screen_blur_enabled
r5apex.exe!0x01ed2fa0 ConVar mat_screen_blur_override
r5apex.exe!0x0170ad40 ConVar mat_shadowstate
r5apex.exe!0x01ed2960 ConVar mat_sharpen_amount
r5apex.exe!0x01ed34a0 ConVar mat_sharpen_threshold
r5apex.exe!0x01ed37c0 ConVar mat_sharpen_width
r5apex.exe!0x01812e40 ConVar mat_show_texture_memory_usage
r5apex.exe!0x01953f90 ConVar mat_showenvmapmask
r5apex.exe!0x0184e050 ConVar mat_showlowresimage
r5apex.exe!0x018514d0 ConVar mat_showmiplevels
r5apex.exe!0x0170db20 ConVar mat_skipid
r5apex.exe!0x01710e50 ConVar mat_sky_color
r5apex.exe!0x0170d6e0 ConVar mat_sky_scale
r5apex.exe!0x01ed1860 ConVar mat_slopescaledepthbias_decal
r5apex.exe!0x01ed1180 ConVar mat_slopescaledepthbias_normal
r5apex.exe!0x01ed17c0 ConVar mat_slopescaledepthbias_shadowmap
r5apex.exe!0x01ed1400 ConVar mat_slopescaledepthbias_ui
r5apex.exe!0x01ed1720 ConVar mat_slopescaledepthbias_zfill
r5apex.exe!0x01710cd0 ConVar mat_sun_color
r5apex.exe!0x0170bb20 ConVar mat_sun_scale
r5apex.exe!0x0170e0c0 ConVar mat_surfacefilter
r5apex.exe!0x0170d170 ConVar mat_surfaceid
r5apex.exe!0x017113f0 ConVar mat_surfacemat
r5apex.exe!0x01956f30 ConVar mat_syncGPU
r5apex.exe!0x01956e90 ConVar mat_syncInterval
r5apex.exe!0x01851750 ConVar mat_sync_rt
r5apex.exe!0x01953e00 ConVar mat_sync_rt_flushes_gpu
r5apex.exe!0x01812d00 ConVar mat_texture_list
r5apex.exe!0x01812da0 ConVar mat_texture_list_view
r5apex.exe!0x01953ea0 ConVar mat_translucency_errors
r5apex.exe!0x01ed3c20 ConVar mat_use_compressed_hdr_textures
r5apex.exe!0x01ed39a0 ConVar mat_vignette_enable
r5apex.exe!0x01852f10 ConVar mat_warn_texture_convert
r5apex.exe!0x0170a210 ConVar match_backingOutMaxTimeToWait
r5apex.exe!0x01710a50 ConVar match_backoutslow
r5apex.exe!0x01709480 ConVar match_connect
r5apex.exe!0x0170b860 ConVar match_defaultMap_party
r5apex.exe!0x01711670 ConVar match_dir
r5apex.exe!0x0170c4d0 ConVar match_dumpSearchResults
r5apex.exe!0x01708d60 ConVar match_emptyUpdateRate
r5apex.exe!0x01712500 ConVar match_enabled
r5apex.exe!0x0170b720 ConVar match_fakePort
r5apex.exe!0x01708f40 ConVar match_fakeS2SPort
r5apex.exe!0x01713dd0 ConVar match_forceVerboseSearches
r5apex.exe!0x0170e500 ConVar match_goodReputation
r5apex.exe!0x0170f0e0 ConVar match_maxPingsSent
r5apex.exe!0x017134a0 ConVar match_mixtape_unchecked
r5apex.exe!0x0170b350 ConVar match_mixtape_unchecked_version
r5apex.exe!0x01714190 ConVar match_mixtape_version
r5apex.exe!0x0170c390 ConVar match_mixtape_warnOnPlay
r5apex.exe!0x01711530 ConVar match_myBestDatacenter
r5apex.exe!0x01711490 ConVar match_myDatacenter
r5apex.exe!0x01711850 ConVar match_myRankedDatacenter
r5apex.exe!0x01709ad0 ConVar match_myTeam
r5apex.exe!0x01710570 ConVar match_partyChangeNum
r5apex.exe!0x01709910 ConVar match_partySize
r5apex.exe!0x01709080 ConVar match_partySub
r5apex.exe!0x017085a0 ConVar match_pingWaveInterval
r5apex.exe!0x017123c0 ConVar match_playlist
r5apex.exe!0x0170a760 ConVar match_precachemap
r5apex.exe!0x0170bd70 ConVar match_privateMatchListWithStryder
r5apex.exe!0x0170d030 ConVar match_rankedMaxPing
r5apex.exe!0x0170a350 ConVar match_rankedSwitchETA
r5apex.exe!0x01711f60 ConVar match_resetPlaylistBetweenMatches
r5apex.exe!0x0170d2b0 ConVar match_searchInterval
r5apex.exe!0x017129b0 ConVar match_searching
r5apex.exe!0x0170f360 ConVar match_teamNoFill
r5apex.exe!0x017104d0 ConVar match_updateNotableRate
r5apex.exe!0x01710750 ConVar match_updateRate
r5apex.exe!0x0170ea00 ConVar match_useMatchmaking
r5apex.exe!0x01711ec0 ConVar match_verbosePrintsInterval
r5apex.exe!0x0170c1b0 ConVar match_visiblePlaylists
r5apex.exe!0x0170df80 ConVar matchmaking_hostname
r5apex.exe!0x02330e60 ConVar matchresults_write_enabled
r5apex.exe!0x02366940 ConVar max_explosive_damage_mass
r5apex.exe!0x02368690 ConVar max_explosive_damage_velocity
r5apex.exe!0x01edf3f0 ConVar max_tweak_shadow_updates
r5apex.exe!0x022fd970 ConVar melee_aim_assist_can_lock_pitch
r5apex.exe!0x02796370 ConVar melee_aim_assist_can_lock_pitch
r5apex.exe!0x02303610 ConVar melee_aim_assist_use_target_velocity
r5apex.exe!0x022fb0f0 ConVar melee_attack_trace_can_use_lunge_distance
r5apex.exe!0x027939c0 ConVar melee_attack_trace_can_use_lunge_distance
r5apex.exe!0x022fcbf0 ConVar melee_cone_trace_box_check
r5apex.exe!0x02795630 ConVar melee_cone_trace_box_check
r5apex.exe!0x02796a30 ConVar melee_cone_trace_lag_compensate_user_command_target
r5apex.exe!0x022e1e60 ConVar melee_lunge_abort_distance
r5apex.exe!0x02333cb0 ConVar melee_lunge_abort_distance
r5apex.exe!0x022b4b40 ConVar melee_lunge_abort_if_blocked
r5apex.exe!0x0230d850 ConVar melee_lunge_abort_if_blocked
r5apex.exe!0x02301410 ConVar melee_lunge_adjust_trace_distance
r5apex.exe!0x0279a9b0 ConVar melee_lunge_adjust_trace_distance
r5apex.exe!0x02301a90 ConVar melee_lunge_align_eye_position
r5apex.exe!0x0279afb0 ConVar melee_lunge_align_eye_position
r5apex.exe!0x022fd3f0 ConVar melee_lunge_dot_check
r5apex.exe!0x02795dc0 ConVar melee_lunge_dot_check
r5apex.exe!0x022e4390 ConVar melee_lunge_force_enable_flying
r5apex.exe!0x02347080 ConVar melee_lunge_force_enable_flying
r5apex.exe!0x02350390 ConVar melee_lunge_lag_compensate_target
r5apex.exe!0x027a1800 ConVar melee_lunge_scale_by_speed
r5apex.exe!0x022b5a50 ConVar melee_lunge_slide
r5apex.exe!0x0230e550 ConVar melee_lunge_slide
r5apex.exe!0x022b2490 ConVar melee_lunge_use_closest_distance_between_cylinders
r5apex.exe!0x0230ae90 ConVar melee_lunge_use_closest_distance_between_cylinders
r5apex.exe!0x0279e080 ConVar melee_lunge_use_command_time
r5apex.exe!0x023668a0 ConVar melee_queue_attack_anim_event
r5apex.exe!0x02795450 ConVar melee_titan_execution_attacker_can_be_ref
r5apex.exe!0x01714d70 ConVar mem_dumpstats
r5apex.exe!0x017042f0 ConVar mem_force_flush
r5apex.exe!0x01704250 ConVar mem_force_flush_section
r5apex.exe!0x0170ae80 ConVar mem_incremental_compact_rate
r5apex.exe!0x0184ea50 ConVar mem_level
r5apex.exe!0x01eec9c0 ConVar mem_level
r5apex.exe!0x01717920 ConVar mem_max_heapsize
r5apex.exe!0x01715e20 ConVar mem_max_heapsize_dedicated
r5apex.exe!0x01715720 ConVar mem_min_heapsize
r5apex.exe!0x01ed5480 ConVar mem_runheapchecks
r5apex.exe!0x0170dc60 ConVar mem_test_each_frame
r5apex.exe!0x017100b0 ConVar mem_test_every_n_seconds
r5apex.exe!0x0170aa40 ConVar mem_test_quiet
r5apex.exe!0x022aa440 ConVar menu_faq_community_version
r5apex.exe!0x022a8200 ConVar menu_faq_patchnotes_version
r5apex.exe!0x022a6100 ConVar menu_faq_viewed
r5apex.exe!0x022a9200 ConVar menu_was_multiplayer_played_last
r5apex.exe!0x01845680 ConVar migrate_attempt_interval
r5apex.exe!0x022f9410 ConVar miles_actor_occlusion_radius
r5apex.exe!0x022f9e90 ConVar miles_channels
r5apex.exe!0x01ef2b70 ConVar miles_flip_active_window_logic
r5apex.exe!0x022f7c20 ConVar miles_force_emitter_environment
r5apex.exe!0x022f92d0 ConVar miles_force_listener_environment
r5apex.exe!0x022fab90 ConVar miles_freeze
r5apex.exe!0x022fa250 ConVar miles_initial_occlusion_delay
r5apex.exe!0x022f9ce0 ConVar miles_language
r5apex.exe!0x022f7e00 ConVar miles_listener_freeze
r5apex.exe!0x01719750 ConVar miles_max_sound_commands_per_server_frame
r5apex.exe!0x022f9550 ConVar miles_nonactor_occlusion
r5apex.exe!0x022fa470 ConVar miles_nonactor_occlusion_radius
r5apex.exe!0x022f9690 ConVar miles_nopandist
r5apex.exe!0x022fa2f0 ConVar miles_occlusion
r5apex.exe!0x022f99e0 ConVar miles_occlusion_force
r5apex.exe!0x022f8240 ConVar miles_occlusion_partial
r5apex.exe!0x022f8f10 ConVar miles_occlusion_use_reset_after_deferred_initial
r5apex.exe!0x022faa50 ConVar miles_samplerate
r5apex.exe!0x027a49a0 ConVar miles_server_disable_sounds
r5apex.exe!0x0171ad90 ConVar miles_server_sounds_debug
r5apex.exe!0x0171ae30 ConVar miles_server_sounds_print
r5apex.exe!0x027a4a40 ConVar miles_server_useSoundIDTable
r5apex.exe!0x022f7d60 ConVar miles_solo_ents
r5apex.exe!0x02293800 ConVar miles_soundscape_imgui
r5apex.exe!0x022f8cb0 ConVar miles_spatialize_front_degrees
r5apex.exe!0x022fa5b0 ConVar miles_spatialize_offplane_strength
r5apex.exe!0x022f8e70 ConVar miles_spatialize_on
r5apex.exe!0x022f97d0 ConVar miles_spatialize_rear_degrees
r5apex.exe!0x022fa7c0 ConVar miles_suffixes
r5apex.exe!0x02367c70 ConVar min_explosive_damage_mass
r5apex.exe!0x02795d20 ConVar missile_debug_draw
r5apex.exe!0x02303750 ConVar missile_default_speed
r5apex.exe!0x0279c820 ConVar missile_default_speed
r5apex.exe!0x02302ed0 ConVar missile_homing_speed
r5apex.exe!0x0279c180 ConVar missile_homing_speed
r5apex.exe!0x01702650 ConVar mod_check_vcollide
r5apex.exe!0x01702870 ConVar mod_trace_load
r5apex.exe!0x01719390 ConVar model_defaultFadeDistMin
r5apex.exe!0x0228e5a0 ConVar model_defaultFadeDistMin
r5apex.exe!0x017160e0 ConVar model_defaultFadeDistScale
r5apex.exe!0x01eeda40 ConVar model_defaultFadeDistScale
r5apex.exe!0x0228efe0 ConVar model_fadeRangeFraction
r5apex.exe!0x0228f080 ConVar model_fadeRangeFractionNear
r5apex.exe!0x01ed48b0 ConVar modeldecals_forceAllowed
r5apex.exe!0x022a6ff0 ConVar monitor_cc
r5apex.exe!0x01ed2aa0 ConVar monitor_mat_sharpen_amount
r5apex.exe!0x022a90a0 ConVar monitor_postfx
r5apex.exe!0x022ac8b0 ConVar monitor_rui_world_enabled
r5apex.exe!0x01edf0d0 ConVar monitor_snapshot_frame_delay
r5apex.exe!0x01eea180 ConVar monitor_zfar_default
r5apex.exe!0x022ac010 ConVar monitor_zfar_override
r5apex.exe!0x022a5fc0 ConVar monitor_zfar_override_enabled
r5apex.exe!0x0170d490 ConVar motd
r5apex.exe!0x022a2060 ConVar mouse_sensitivity
r5apex.exe!0x02296b70 ConVar mouse_use_per_scope_sensitivity_scalars
r5apex.exe!0x0229d680 ConVar mouse_zoomed_sensitivity_scalar_0
r5apex.exe!0x0228fd40 ConVar mouse_zoomed_sensitivity_scalar_1
r5apex.exe!0x02295520 ConVar mouse_zoomed_sensitivity_scalar_2
r5apex.exe!0x0229e030 ConVar mouse_zoomed_sensitivity_scalar_3
r5apex.exe!0x02295760 ConVar mouse_zoomed_sensitivity_scalar_4
r5apex.exe!0x022a1830 ConVar mouse_zoomed_sensitivity_scalar_5
r5apex.exe!0x01714f70 ConVar move_one_cmd_per_client_frame
r5apex.exe!0x027927a0 ConVar movement_anim_downed_playback_maxrate
r5apex.exe!0x027928e0 ConVar movement_anim_playback_maxrate
r5apex.exe!0x02792480 ConVar movement_anim_playback_minrate
r5apex.exe!0x02792520 ConVar movement_anim_sprint_playback_maxrate
r5apex.exe!0x01712750 ConVar mp_accountLink_requestInterval
r5apex.exe!0x023e9930 ConVar mp_allowNPCs
r5apex.exe!0x01709770 ConVar mp_allowed
r5apex.exe!0x02792840 ConVar mp_bodyyawrate
r5apex.exe!0x02335040 ConVar mp_class_max_dronecontroller
r5apex.exe!0x02348220 ConVar mp_class_max_fireteam
r5apex.exe!0x02330f00 ConVar mp_class_max_pilot
r5apex.exe!0x02335800 ConVar mp_class_max_titan
r5apex.exe!0x01813d10 ConVar mp_countRRNobodyAsLobby
r5apex.exe!0x02371a80 ConVar mp_defaultteam
r5apex.exe!0x0234f940 ConVar mp_enablematchending
r5apex.exe!0x0234c430 ConVar mp_enabletimelimit
r5apex.exe!0x023e92f0 ConVar mp_fraglimit
r5apex.exe!0x023515c0 ConVar mp_gamemode
r5apex.exe!0x018187c0 ConVar mp_huge_threshhold
r5apex.exe!0x018479c0 ConVar mp_linkingAccountTime
r5apex.exe!0x01847920 ConVar mp_linkingAccountWindow
r5apex.exe!0x02792980 ConVar mp_maxbodyyaw
r5apex.exe!0x01713900 ConVar mp_permission_requestInterval
r5apex.exe!0x01711ca0 ConVar mp_permission_rerequestInterval
r5apex.exe!0x022b5950 ConVar mp_player_level
r5apex.exe!0x02792a20 ConVar mp_scaleAnimationSpeeds
r5apex.exe!0x02792b60 ConVar mp_showgestureslots
r5apex.exe!0x023ea6a0 ConVar mp_teamlist
r5apex.exe!0x02397450 ConVar mp_teamoverride
r5apex.exe!0x023eb310 ConVar mp_weaponstay
r5apex.exe!0x0171aed0 ConVar mtx_svEdition
r5apex.exe!0x02788f20 ConVar multiplayer_animstate_once_per_frame_on_server
r5apex.exe!0x023044b0 ConVar muteWeaponSounds
r5apex.exe!0x0279e550 ConVar muteWeaponSounds
r5apex.exe!0x01814630 ConVar name
r5apex.exe!0x0278ffb0 ConVar navmesh_move_along_surface_asserts
r5apex.exe!0x0278e440 ConVar navmesh_normal_links_only
r5apex.exe!0x0278d010 ConVar navmesh_test_zone_connectivity_traverse_anim_type
r5apex.exe!0x01ef44c0 ConVar net_RunInvalidatePhysics
r5apex.exe!0x0170fa10 ConVar net_async_sendto
r5apex.exe!0x0170e020 ConVar net_autoUnthrottle
r5apex.exe!0x01714c30 ConVar net_bandwidthPrintThreshold
r5apex.exe!0x01713360 ConVar net_bindToSpecificAddress
r5apex.exe!0x017092e0 ConVar net_blockmsg
r5apex.exe!0x01848ac0 ConVar net_chatThroughChatserver
r5apex.exe!0x017144b0 ConVar net_chokeloop
r5apex.exe!0x0170dee0 ConVar net_clearReliableDataOnReset
r5apex.exe!0x02304370 ConVar net_client_side_weapon_animations
r5apex.exe!0x0279e2d0 ConVar net_client_side_weapon_animations
r5apex.exe!0x01714930 ConVar net_compressDataBlock
r5apex.exe!0x0170fbf0 ConVar net_compressLZValue
r5apex.exe!0x0170d350 ConVar net_compresspackets
r5apex.exe!0x0170a8a0 ConVar net_compresspackets_minsize
r5apex.exe!0x018137d0 ConVar net_connectPacketWarningThreshhold
r5apex.exe!0x01816f90 ConVar net_connectingDataRate
r5apex.exe!0x01706bc0 ConVar net_createUndoDeltas
r5apex.exe!0x01818e00 ConVar net_data_block_enabled
r5apex.exe!0x01712280 ConVar net_datablockPrintSummaries
r5apex.exe!0x01815510 ConVar net_datablock_fastRate
r5apex.exe!0x0170d5a0 ConVar net_datablock_longSendTime
r5apex.exe!0x01712fc0 ConVar net_datablock_minResendInterval
r5apex.exe!0x01817290 ConVar net_datablock_networkLossForSlowSpeed
r5apex.exe!0x01815ab0 ConVar net_datablock_resendRateForSlowSpeed
r5apex.exe!0x018180e0 ConVar net_datablock_slowRate
r5apex.exe!0x01708c20 ConVar net_debugDataBlockReceiver
r5apex.exe!0x01712460 ConVar net_debugDataBlockSender
r5apex.exe!0x0171b5b0 ConVar net_debugLerping
r5apex.exe!0x01814310 ConVar net_deltaFieldEntityBlockSize
r5apex.exe!0x018130f0 ConVar net_disconnectIfDeltaBufferIsFull
r5apex.exe!0x01708640 ConVar net_drawslider
r5apex.exe!0x017101d0 ConVar net_droppackets
r5apex.exe!0x01706f40 ConVar net_dumpChangesPrecise
r5apex.exe!0x0170ade0 ConVar net_encrypt_copyCtx
r5apex.exe!0x0170d780 ConVar net_encryptionDebug
r5apex.exe!0x0170e160 ConVar net_fakelag
r5apex.exe!0x017140f0 ConVar net_fakelag_clientOnly
r5apex.exe!0x01714b90 ConVar net_fakelagjitter
r5apex.exe!0x01714af0 ConVar net_fakeloss
r5apex.exe!0x01812fb0 ConVar net_forceDeltaBufferToOverflow
r5apex.exe!0x01705de0 ConVar net_forceUnnecessaryUndoDeltas
r5apex.exe!0x017120a0 ConVar net_forcetimeout
r5apex.exe!0x01714710 ConVar net_fullyConnectedDataRate
r5apex.exe!0x017dd0c0 ConVar net_highPacketLatencyThreshold
r5apex.exe!0x017dbf60 ConVar net_highPacketLossThreshold
r5apex.exe!0x017dbc00 ConVar net_ignoreAllSnapshots
r5apex.exe!0x018135f0 ConVar net_largeSnapshotThreshold
r5apex.exe!0x0171b790 ConVar net_lerpFields
r5apex.exe!0x01816e50 ConVar net_lowBandwidthConnect
r5apex.exe!0x01708cc0 ConVar net_maxAccumulatedClearTimeBalance
r5apex.exe!0x01709b70 ConVar net_maxcleartime
r5apex.exe!0x0170ac00 ConVar net_maxfilesize
r5apex.exe!0x01709d30 ConVar net_maxfragments
r5apex.exe!0x01708fe0 ConVar net_maxroutable
r5apex.exe!0x01709520 ConVar net_minConnectionTimeForSpam
r5apex.exe!0x017117b0 ConVar net_minQueuedPacketsForPrint
r5apex.exe!0x022ab800 ConVar net_minResetIdleTimerInterval
r5apex.exe!0x01709fb0 ConVar net_minimumPacketLossDC
r5apex.exe!0x01709dd0 ConVar net_minroutable
r5apex.exe!0x01814450 ConVar net_noPostDataForDeletedEnts
r5apex.exe!0x02367170 ConVar net_old_seed_generation
r5apex.exe!0x01816a90 ConVar net_optimize_persistent_data
r5apex.exe!0x01818b80 ConVar net_optimize_playlists
r5apex.exe!0x027a0880 ConVar net_optimize_weapons
r5apex.exe!0x01eef140 ConVar net_predictParentEntities
r5apex.exe!0x01813b30 ConVar net_predictedEntsUseFirstAvailableSnapshot
r5apex.exe!0x01813550 ConVar net_predictionDebug
r5apex.exe!0x0171b970 ConVar net_pretendSnapshotArrayFull
r5apex.exe!0x0170bbc0 ConVar net_printCompression
r5apex.exe!0x01813c70 ConVar net_printOutOfSnapshots
r5apex.exe!0x01705af0 ConVar net_printUnnecessaryDeltas
r5apex.exe!0x01707b60 ConVar net_propSkipPrintThreshold
r5apex.exe!0x0170f400 ConVar net_public_adr
r5apex.exe!0x0170b9a0 ConVar net_queue_trace
r5apex.exe!0x017081c0 ConVar net_queuedPackets_PrintOversleeps
r5apex.exe!0x0170dda0 ConVar net_queuedPackets_SkipSmallSleeps
r5apex.exe!0x0170c070 ConVar net_queued_packet_sender_nopacket_sleep
r5apex.exe!0x0170afc0 ConVar net_queued_packet_thread
r5apex.exe!0x0170cb80 ConVar net_recentNetworkGapWindow
r5apex.exe!0x0170ec80 ConVar net_recentNetworkGapsNeeded
r5apex.exe!0x018132d0 ConVar net_recreateScriptInstanceOnReplayTransition
r5apex.exe!0x0171bab0 ConVar net_recv_dumpChanges
r5apex.exe!0x017064a0 ConVar net_recv_dumpNetworkedChangesOnEntCreate
r5apex.exe!0x01706860 ConVar net_recv_watchEnt
r5apex.exe!0x01705e80 ConVar net_recv_watchField1
r5apex.exe!0x01707440 ConVar net_recv_watchField2
r5apex.exe!0x01710af0 ConVar net_resourcePrintMinimum
r5apex.exe!0x01705790 ConVar net_sendFloatDeltas
r5apex.exe!0x01817e60 ConVar net_sendProfileTotals
r5apex.exe!0x0170fdd0 ConVar net_sendtoInJob
r5apex.exe!0x01713f10 ConVar net_showFailedAuth
r5apex.exe!0x01813870 ConVar net_showLargeSnapshot
r5apex.exe!0x0170c2f0 ConVar net_showQueued
r5apex.exe!0x01704f20 ConVar net_showUndoDeltas
r5apex.exe!0x017dc3a0 ConVar net_showUserWarnings
r5apex.exe!0x01714370 ConVar net_showchoke
r5apex.exe!0x01708b80 ConVar net_showchokeInterval
r5apex.exe!0x0170d210 ConVar net_showdrop
r5apex.exe!0x01708ea0 ConVar net_showfragments
r5apex.exe!0x0170ed20 ConVar net_showmsg
r5apex.exe!0x0170b100 ConVar net_showpeaks
r5apex.exe!0x0170ca60 ConVar net_showsendrecv
r5apex.exe!0x0170b1a0 ConVar net_showsplits
r5apex.exe!0x0170d0d0 ConVar net_showudp
r5apex.exe!0x017086e0 ConVar net_showudp_oob
r5apex.exe!0x01714a50 ConVar net_showudp_remoteonly
r5apex.exe!0x022ef630 ConVar net_showusercmd
r5apex.exe!0x01704830 ConVar net_skipUnnecessaryDeltas
r5apex.exe!0x01710ef0 ConVar net_splitrate
r5apex.exe!0x0170f970 ConVar net_splitrateDefaultMP
r5apex.exe!0x0170c110 ConVar net_splitrateDefaultSP
r5apex.exe!0x0235def0 ConVar net_sv_showusercmd
r5apex.exe!0x01708380 ConVar net_tamperPackets
r5apex.exe!0x01813410 ConVar net_threadedEntityDeltas
r5apex.exe!0x018144f0 ConVar net_threadedProcessPacket
r5apex.exe!0x017099b0 ConVar net_timeoutUsesLastReadTime
r5apex.exe!0x017103b0 ConVar net_trackerWarningInterval
r5apex.exe!0x01713180 ConVar net_usesocketsforloopback
r5apex.exe!0x01711a20 ConVar net_verifyEncryption
r5apex.exe!0x018489a0 ConVar net_voiceEchoFromChatServer
r5apex.exe!0x017096d0 ConVar net_warnAboutSocketReadGaps
r5apex.exe!0x017121e0 ConVar net_warnGapTime
r5apex.exe!0x01713400 ConVar net_wifi
r5apex.exe!0x01813230 ConVar net_worldHitchSlopTime
r5apex.exe!0x01710270 ConVar next
r5apex.exe!0x02367490 ConVar noReloadAfterUse
r5apex.exe!0x023e9cf0 ConVar noclip_fixup
r5apex.exe!0x01ed3720 ConVar noise_filter_scale
r5apex.exe!0x01715f60 ConVar not_focus_sleep
r5apex.exe!0x01843260 ConVar notification_displayTime
r5apex.exe!0x0278cf70 ConVar npc_chancetohit_forcedOn
r5apex.exe!0x0278afd0 ConVar npc_sight_mode
r5apex.exe!0x027a4150 ConVar npc_titan_always_block_projectile_health
r5apex.exe!0x027a2e20 ConVar npc_titan_block_projectile_chance
r5apex.exe!0x027a41f0 ConVar npc_titan_footstep_sound_radius
r5apex.exe!0x027a3fd0 ConVar npc_titan_light_pain_threshold
r5apex.exe!0x027a2d80 ConVar npc_titan_phys_ignore_mass
r5apex.exe!0x027a38e0 ConVar npc_titan_phys_knock_damage
r5apex.exe!0x027a4070 ConVar npc_titan_phys_knock_mass
r5apex.exe!0x027a3a20 ConVar npc_titan_phys_knock_radius
r5apex.exe!0x027a3980 ConVar npc_titan_phys_knock_speed
r5apex.exe!0x01716fc0 ConVar nucleus_id
r5apex.exe!0x017165c0 ConVar nucleus_pid
r5apex.exe!0x022a6d70 ConVar number_shortenToMillionsAfter
r5apex.exe!0x023077b0 ConVar offhandTossOverheadPitchThreshold
r5apex.exe!0x027a10a0 ConVar offhandTossOverheadPitchThreshold
r5apex.exe!0x017185e0 ConVar old_culling
r5apex.exe!0x017168e0 ConVar old_gather_props
r5apex.exe!0x022e51d0 ConVar one_handed_change_rate
r5apex.exe!0x02348400 ConVar one_handed_change_rate
r5apex.exe!0x018439c0 ConVar openInvite_spam
r5apex.exe!0x01843440 ConVar openInvites_filterByLanguage
r5apex.exe!0x018436d0 ConVar openInvites_filterByRegion
r5apex.exe!0x01845c20 ConVar openinvite_duration_default
r5apex.exe!0x02784ab0 ConVar ordnancePickupSound
r5apex.exe!0x0228fde0 ConVar ordnanceSwapSelectCooldown
r5apex.exe!0x01845040 ConVar origin_Errorlevel_OldBehaviour
r5apex.exe!0x018452c0 ConVar origin_Errorlevel_Telementry
r5apex.exe!0x01845220 ConVar origin_authCodeFailureMaxBackoffSeconds
r5apex.exe!0x018450e0 ConVar origin_autoRefreshToken
r5apex.exe!0x01844f00 ConVar origin_debug
r5apex.exe!0x01844ac0 ConVar origin_disconnectWhenOffline
r5apex.exe!0x01844b60 ConVar origin_ignoreInvitesOnLoadScreen
r5apex.exe!0x018449b0 ConVar origin_igo_mutes_sound_enabled
r5apex.exe!0x01ef15e0 ConVar origin_igo_muting_sound
r5apex.exe!0x01844fa0 ConVar origin_presense_updateRate
r5apex.exe!0x01845180 ConVar origin_tokenFailureMaxBackoffSeconds
r5apex.exe!0x01ed8160 ConVar panel_showVisChanges
r5apex.exe!0x01ed7e20 ConVar panel_test_title_safe
r5apex.exe!0x0236eec0 ConVar parenting_clearParentOriginFix
r5apex.exe!0x01ede6c0 ConVar parenting_debug
r5apex.exe!0x022f2eb0 ConVar particleEffect_checkShouldStillPlay
r5apex.exe!0x0230a090 ConVar particle_alwayswakeonstop
r5apex.exe!0x02790d40 ConVar particle_alwayswakeonstop
r5apex.exe!0x018494d0 ConVar particle_cpu_level
r5apex.exe!0x0230a450 ConVar particle_delete_all_except
r5apex.exe!0x02791100 ConVar particle_delete_all_except
r5apex.exe!0x02284990 ConVar particle_dlights_enable
r5apex.exe!0x0228e8c0 ConVar particle_dlights_spew
r5apex.exe!0x022b1490 ConVar particle_gpu_level
r5apex.exe!0x019554e0 ConVar particle_lighting_clear_enable
r5apex.exe!0x01955440 ConVar particle_lighting_size
r5apex.exe!0x02309bf0 ConVar particle_lighting_viewmodel_enable
r5apex.exe!0x022a9750 ConVar particle_overlay
r5apex.exe!0x022aabd0 ConVar particle_overlay_detail_attributes
r5apex.exe!0x022b0440 ConVar particle_overlay_detail_filter
r5apex.exe!0x022b0060 ConVar particle_overlay_detail_list_particles
r5apex.exe!0x022ac2d0 ConVar particle_overlay_detail_scroll
r5apex.exe!0x022ae540 ConVar particle_overlay_hide_sleeping
r5apex.exe!0x022aeea0 ConVar particle_overlay_list_filter
r5apex.exe!0x022a92a0 ConVar particle_overlay_list_tally
r5apex.exe!0x022aaa90 ConVar particle_overlay_list_tally_collapse_children
r5apex.exe!0x022ace50 ConVar particle_overlay_old
r5apex.exe!0x022ad7d0 ConVar particle_overlay_scroll
r5apex.exe!0x023098a0 ConVar particle_remap_vol2cp_debug
r5apex.exe!0x02309550 ConVar particle_script_dump
r5apex.exe!0x027907a0 ConVar particle_script_dump
r5apex.exe!0x023095f0 ConVar particle_script_list
r5apex.exe!0x02790840 ConVar particle_script_list
r5apex.exe!0x02309690 ConVar particle_script_log
r5apex.exe!0x027908e0 ConVar particle_script_log
r5apex.exe!0x022a9b20 ConVar particle_scrub_debug
r5apex.exe!0x02309cd0 ConVar particle_scrub_debug_effect
r5apex.exe!0x02790980 ConVar particle_scrub_debug_effect
r5apex.exe!0x02791240 ConVar particle_scrub_is_using_time_scrub
r5apex.exe!0x0230a130 ConVar particle_scrub_max_dt
r5apex.exe!0x02790de0 ConVar particle_scrub_max_dt
r5apex.exe!0x0230a1d0 ConVar particle_scrub_play_speed
r5apex.exe!0x02790e80 ConVar particle_scrub_play_speed
r5apex.exe!0x0230a3b0 ConVar particle_scrub_quality
r5apex.exe!0x02791060 ConVar particle_scrub_quality
r5apex.exe!0x0230a270 ConVar particle_scrub_time
r5apex.exe!0x02790f20 ConVar particle_scrub_time
r5apex.exe!0x022aec00 ConVar particle_simulateoverflow
r5apex.exe!0x02348360 ConVar particle_test_attach_attachment
r5apex.exe!0x0230dbf0 ConVar particle_test_attach_mode
r5apex.exe!0x0230ac30 ConVar particle_test_file
r5apex.exe!0x0230a4f0 ConVar particles_cull_dlights
r5apex.exe!0x027911a0 ConVar particles_cull_dlights
r5apex.exe!0x0230a310 ConVar particles_max_passes
r5apex.exe!0x02790fc0 ConVar particles_max_passes
r5apex.exe!0x02309d70 ConVar particles_spawncull
r5apex.exe!0x02790a20 ConVar particles_spawncull
r5apex.exe!0x02309f50 ConVar particles_spawncull_report
r5apex.exe!0x02790c00 ConVar particles_spawncull_report
r5apex.exe!0x02309e10 ConVar particles_try_reloading_sheets
r5apex.exe!0x02790ac0 ConVar particles_try_reloading_sheets
r5apex.exe!0x01845cc0 ConVar parties_alwaysReadSubs
r5apex.exe!0x018454a0 ConVar party_autoCreatePartyAlways
r5apex.exe!0x01845fe0 ConVar party_autoCreatePartyDelay
r5apex.exe!0x022ef710 ConVar party_color_enabled
r5apex.exe!0x01845720 ConVar party_doRealNameLookups
r5apex.exe!0x018455e0 ConVar party_doRealNameLookupsForOwner
r5apex.exe!0x01843c90 ConVar party_hostname
r5apex.exe!0x01845540 ConVar party_httpHandleTimeout
r5apex.exe!0x01843770 ConVar party_keepAliveTime
r5apex.exe!0x01845ae0 ConVar party_keepAliveTime
r5apex.exe!0x01845900 ConVar party_leaderAlwaysDetectsChanges
r5apex.exe!0x01845b80 ConVar party_leaveMatchOnJoin
r5apex.exe!0x01845a40 ConVar party_lookupRealNamesForOpenInvites
r5apex.exe!0x01845d60 ConVar party_lookupRealNamesForOpenInvitesForOwner
r5apex.exe!0x01845860 ConVar party_minSize
r5apex.exe!0x01845f40 ConVar party_privacy
r5apex.exe!0x01846080 ConVar party_readyToSearch
r5apex.exe!0x01819120 ConVar party_relyOnPartyForMemberUserInfo
r5apex.exe!0x01846120 ConVar party_requireConsensusForSearch
r5apex.exe!0x01719570 ConVar perTriangleCollisionForced
r5apex.exe!0x01848e90 ConVar persistence_clForceNew
r5apex.exe!0x017132c0 ConVar persistence_hostname
r5apex.exe!0x01813f70 ConVar persistent_warningRate
r5apex.exe!0x01715980 ConVar pertrianglecollision
r5apex.exe!0x027921c0 ConVar phys_bounce
r5apex.exe!0x023091d0 ConVar phys_cfm
r5apex.exe!0x02791c60 ConVar phys_cfm
r5apex.exe!0x02309050 ConVar phys_cfm_anglejointstop
r5apex.exe!0x027919a0 ConVar phys_cfm_anglejointstop
r5apex.exe!0x023ea820 ConVar phys_damage_players
r5apex.exe!0x023090f0 ConVar phys_drawContacts
r5apex.exe!0x02791a40 ConVar phys_drawContacts
r5apex.exe!0x02791b80 ConVar phys_drawContactsDuration
r5apex.exe!0x027917c0 ConVar phys_drawGeoms
r5apex.exe!0x02792120 ConVar phys_drawTunnelChecks
r5apex.exe!0x02792080 ConVar phys_enableObjectPairCollidePrototype
r5apex.exe!0x02308c90 ConVar phys_erp
r5apex.exe!0x02791540 ConVar phys_erp
r5apex.exe!0x02308e70 ConVar phys_erp_anglejointstop
r5apex.exe!0x02791720 ConVar phys_erp_anglejointstop
r5apex.exe!0x02791fe0 ConVar phys_frictionDefault
r5apex.exe!0x023704d0 ConVar phys_impactforcescale
r5apex.exe!0x022af5f0 ConVar phys_showObjectCount
r5apex.exe!0x023e9430 ConVar phys_show_active
r5apex.exe!0x023e9c50 ConVar phys_speeds
r5apex.exe!0x02369870 ConVar phys_stressbodyweights
r5apex.exe!0x02791f40 ConVar phys_threadGoWide
r5apex.exe!0x023eb1d0 ConVar phys_timescale
r5apex.exe!0x0236dec0 ConVar phys_upimpactforcescale
r5apex.exe!0x023094b0 ConVar physics_async_cl
r5apex.exe!0x02791ae0 ConVar physics_async_sv
r5apex.exe!0x02309370 ConVar physics_autoSleepAngularThreshold
r5apex.exe!0x02791e00 ConVar physics_autoSleepAngularThreshold
r5apex.exe!0x02308fb0 ConVar physics_autoSleepDebug
r5apex.exe!0x02791900 ConVar physics_autoSleepDebug
r5apex.exe!0x02308f10 ConVar physics_autoSleepGroundHysteresis
r5apex.exe!0x02791860 ConVar physics_autoSleepGroundHysteresis
r5apex.exe!0x02309410 ConVar physics_autoSleepSpeedThreshold
r5apex.exe!0x02791ea0 ConVar physics_autoSleepSpeedThreshold
r5apex.exe!0x02308d30 ConVar physics_collideWithMovingGeo
r5apex.exe!0x027915e0 ConVar physics_collideWithMovingGeo
r5apex.exe!0x02326ed0 ConVar physics_defaultMaxAngularSpeed
r5apex.exe!0x02347fc0 ConVar physics_defaultMaxSpeed
r5apex.exe!0x01edbcd0 ConVar physics_scaled_mem
r5apex.exe!0x02308dd0 ConVar physics_tunnelChecks
r5apex.exe!0x02791680 ConVar physics_tunnelChecks
r5apex.exe!0x023092d0 ConVar physics_tunnelChecksForceAlways
r5apex.exe!0x02791d60 ConVar physics_tunnelChecksForceAlways
r5apex.exe!0x02356cc0 ConVar physicsshadowupdate_render
r5apex.exe!0x017189a0 ConVar pin_opt_in
r5apex.exe!0x01814590 ConVar pin_plat_id
r5apex.exe!0x01718cc0 ConVar pin_sid
r5apex.exe!0x01715680 ConVar pin_telemetry_actually_send
r5apex.exe!0x01718f40 ConVar pin_telemetry_debug_code
r5apex.exe!0x01715450 ConVar pin_telemetry_debug_payload
r5apex.exe!0x022f0980 ConVar pin_telemetry_debug_script
r5apex.exe!0x0235ddb0 ConVar pin_telemetry_debug_script
r5apex.exe!0x017158e0 ConVar pin_telemetry_dont_send_events
r5apex.exe!0x01716ac0 ConVar pin_telemetry_hostname
r5apex.exe!0x01716660 ConVar pin_telemetry_inactivity_send_time
r5apex.exe!0x01716840 ConVar pin_telemetry_max_payload_size
r5apex.exe!0x01717ce0 ConVar pin_telemetry_send_debug
r5apex.exe!0x0170eaa0 ConVar ping_debug
r5apex.exe!0x022f6aa0 ConVar ping_max_green
r5apex.exe!0x022f6920 ConVar ping_max_red
r5apex.exe!0x022f6e60 ConVar ping_max_yellow
r5apex.exe!0x0170cf90 ConVar ping_minSentForChoice
r5apex.exe!0x0170f040 ConVar ping_qos_units
r5apex.exe!0x02786f10 ConVar ping_show_measured
r5apex.exe!0x017139a0 ConVar ping_usePacketLoss
r5apex.exe!0x01ededa0 ConVar pixvis_enable
r5apex.exe!0x01ed0bd0 ConVar pixvis_maxquads
r5apex.exe!0x01edeb20 ConVar pixvis_spew
r5apex.exe!0x01844690 ConVar plat_environment
r5apex.exe!0x01842f70 ConVar plat_retryNameLookups
r5apex.exe!0x018146d0 ConVar platform_user_id
r5apex.exe!0x02787b80 ConVar playerDeathAnimMaxFrames
r5apex.exe!0x022f5fe0 ConVar playerListPartyColorB
r5apex.exe!0x022f6dc0 ConVar playerListPartyColorG
r5apex.exe!0x022f5cc0 ConVar playerListPartyColorR
r5apex.exe!0x022f7040 ConVar playerListUseFriendColor
r5apex.exe!0x022eaf10 ConVar player_ADS_buffer_time_seconds
r5apex.exe!0x02357d00 ConVar player_ADS_buffer_time_seconds
r5apex.exe!0x02783f60 ConVar player_adjustTimersWithTimeBase
r5apex.exe!0x0276b9b0 ConVar player_charDataMinInterval
r5apex.exe!0x02281d00 ConVar player_debugPredictedPosition
r5apex.exe!0x0276bdf0 ConVar player_debug_print_damage
r5apex.exe!0x01ede4e0 ConVar player_deltaAnimsMakeMeUnpredicted
r5apex.exe!0x02782ee0 ConVar player_disallow_negative_frametime
r5apex.exe!0x02788e80 ConVar player_dispatch_anim_events_per_frame
r5apex.exe!0x01ef8da0 ConVar player_doJetwashEffects
r5apex.exe!0x022e0b70 ConVar player_extraairaccelleration
r5apex.exe!0x02332a70 ConVar player_extraairaccelleration
r5apex.exe!0x02354de0 ConVar player_find_rodeo_target_per_cmd
r5apex.exe!0x02281bc0 ConVar player_highFrequencyThinkDistance
r5apex.exe!0x02783a30 ConVar player_maxTimerAdjust
r5apex.exe!0x02777200 ConVar player_max_command_contexts
r5apex.exe!0x0279c780 ConVar player_melee_cone_from_user_command_only
r5apex.exe!0x02350430 ConVar player_movementBounds_predictionShare
r5apex.exe!0x02777160 ConVar player_movement_debug
r5apex.exe!0x02792300 ConVar player_movingDeathThreshold
r5apex.exe!0x02783340 ConVar player_old_armor
r5apex.exe!0x01efc580 ConVar player_respawnInputDebounceDuration
r5apex.exe!0x02785890 ConVar player_restore_use_SetLocalAngles
r5apex.exe!0x02776000 ConVar player_restore_use_UpdateCurrentPlayerClass
r5apex.exe!0x02786780 ConVar player_share_squad_info
r5apex.exe!0x01ef2cb0 ConVar player_showEyePosition
r5apex.exe!0x023560b0 ConVar player_showpredictedposition
r5apex.exe!0x0235d1e0 ConVar player_showpredictedposition_timestep
r5apex.exe!0x02783b70 ConVar player_testSpectateNetcode
r5apex.exe!0x022b22d0 ConVar player_useMovementBounds
r5apex.exe!0x0230ab90 ConVar player_useMovementBounds
r5apex.exe!0x02776f90 ConVar player_userCmdsQueueWarning
r5apex.exe!0x01efc740 ConVar player_viewchange_debug_pitch
r5apex.exe!0x02283c10 ConVar player_viewchange_debug_roll
r5apex.exe!0x01ef7390 ConVar player_viewchange_debug_yaw
r5apex.exe!0x02371800 ConVar playerframetimekick_debug
r5apex.exe!0x023ea960 ConVar playerframetimekick_decayrate
r5apex.exe!0x023e90d0 ConVar playerframetimekick_includerealtime
r5apex.exe!0x023a7ee0 ConVar playerframetimekick_margin
r5apex.exe!0x022f61c0 ConVar playerlist_showGen
r5apex.exe!0x02786dd0 ConVar players_updatePingTickInterval
r5apex.exe!0x017184a0 ConVar playlist_changeGamemodeAutomatically
r5apex.exe!0x017174c0 ConVar playlist_debug
r5apex.exe!0x017153b0 ConVar playlist_debug_getvar
r5apex.exe!0x01716a20 ConVar playlist_debug_localization
r5apex.exe!0x022dfb90 ConVar playlist_variableErrorsChecks
r5apex.exe!0x02330fa0 ConVar playlist_variableErrorsChecks
r5apex.exe!0x02792cc0 ConVar portal_pointpush_debug
r5apex.exe!0x02792da0 ConVar portal_pointpush_think_rate
r5apex.exe!0x02368110 ConVar portal_use_player_avoidance
r5apex.exe!0x01ef46a0 ConVar postdataupdate_threaded
r5apex.exe!0x01ee9b40 ConVar postdataupdate_threaded_chunksize
r5apex.exe!0x0277d420 ConVar prevent_ammo_suck
r5apex.exe!0x0170fe70 ConVar printConnectTimings
r5apex.exe!0x01f01200 ConVar print_timeprefix
r5apex.exe!0x02301370 ConVar process_pending_vm_effects
r5apex.exe!0x022f5e00 ConVar progressbar_allow_wrap
r5apex.exe!0x022f6300 ConVar progressbar_high_precision
r5apex.exe!0x022f6080 ConVar progressbar_single_bar
r5apex.exe!0x02305ff0 ConVar projectile_fake_prediction_in_kill_replay
r5apex.exe!0x027a0190 ConVar projectile_fake_prediction_in_kill_replay
r5apex.exe!0x023020d0 ConVar projectile_faketrails
r5apex.exe!0x023036b0 ConVar projectile_filltrails
r5apex.exe!0x0279c8c0 ConVar projectile_lagCompensationDebug
r5apex.exe!0x02795590 ConVar projectile_lagCompensationDebugDrawTime
r5apex.exe!0x0279b5a0 ConVar projectile_lagCompensationDebugExtra
r5apex.exe!0x027953b0 ConVar projectile_lagCompensationDebugServerOffset
r5apex.exe!0x0279dcc0 ConVar projectile_lagCompensationMissileTimeStepScalar
r5apex.exe!0x023022b0 ConVar projectile_muzzleOffsetFirstPersonDecayDist
r5apex.exe!0x022fe010 ConVar projectile_muzzleOffsetFirstPersonDecayMaxTime
r5apex.exe!0x022fd350 ConVar projectile_muzzleOffsetThirdPersonDecayDist
r5apex.exe!0x02302350 ConVar projectile_muzzleOffsetThirdPersonDecayMaxTime
r5apex.exe!0x0279da70 ConVar projectile_prediction
r5apex.exe!0x02303cd0 ConVar projectile_predictionErrorCorrectTime
r5apex.exe!0x02778e70 ConVar prop_active_gib_limit
r5apex.exe!0x0276bb90 ConVar prop_active_gib_max_fade_time
r5apex.exe!0x0277ec10 ConVar prop_break_disable_float
r5apex.exe!0x02282140 ConVar prop_lightweightPropsSkipAnimData
r5apex.exe!0x01ede940 ConVar prop_survivalSkipsAnimData
r5apex.exe!0x02366ef0 ConVar props_break_burst_rotation
r5apex.exe!0x02367a90 ConVar props_break_max_pieces
r5apex.exe!0x023673f0 ConVar props_break_max_pieces_perframe
r5apex.exe!0x01843d30 ConVar publication_hostname
r5apex.exe!0x0228e780 ConVar push_cl
r5apex.exe!0x01ef07e0 ConVar push_cl_always_update_prev_matrix
r5apex.exe!0x0235de50 ConVar push_debug
r5apex.exe!0x0235bd20 ConVar push_debug_ent
r5apex.exe!0x02359810 ConVar push_debug_pause_always
r5apex.exe!0x0235ce80 ConVar push_player_nearby_dist
r5apex.exe!0x022ea360 ConVar push_ragdolls
r5apex.exe!0x02357bf0 ConVar pve_debug
r5apex.exe!0x022f4440 ConVar pvs_addWorkItemsAccum
r5apex.exe!0x022f49e0 ConVar pvs_addWorkItemsThreshold
r5apex.exe!0x022f4940 ConVar pvs_cullBoxes
r5apex.exe!0x022f4620 ConVar pvs_debug
r5apex.exe!0x022f4800 ConVar pvs_drawPortals
r5apex.exe!0x022f46c0 ConVar pvs_frustumCullOnly
r5apex.exe!0x0228eaa0 ConVar pvs_start_early
r5apex.exe!0x02335620 ConVar r_AirboatViewDampenDamp
r5apex.exe!0x02347ca0 ConVar r_AirboatViewDampenFreq
r5apex.exe!0x023363e0 ConVar r_AirboatViewZHeight
r5apex.exe!0x022b1ea0 ConVar r_DrawBeams
r5apex.exe!0x02348060 ConVar r_JeepViewDampenDamp
r5apex.exe!0x023365c0 ConVar r_JeepViewDampenFreq
r5apex.exe!0x0230dd30 ConVar r_VehicleViewDampen
r5apex.exe!0x022f4760 ConVar r_WaterDrawReflection
r5apex.exe!0x018511b0 ConVar r_WaterDrawRefraction
r5apex.exe!0x0170a0f0 ConVar r_aspectratio
r5apex.exe!0x01ed25a0 ConVar r_bloomtintb
r5apex.exe!0x01ed2780 ConVar r_bloomtintexponent
r5apex.exe!0x01ed26e0 ConVar r_bloomtintg
r5apex.exe!0x01ed2640 ConVar r_bloomtintr
r5apex.exe!0x022a7d80 ConVar r_blurmenubg
r5apex.exe!0x01717600 ConVar r_brush_queue_mode
r5apex.exe!0x017180a0 ConVar r_createmodeldecals
r5apex.exe!0x0170dd00 ConVar r_cullshadowworldmeshes
r5apex.exe!0x022b25d0 ConVar r_debug_draw_box_depth_test
r5apex.exe!0x01718720 ConVar r_decal_cover_count
r5apex.exe!0x01707330 ConVar r_decal_cull_stretch_limit
r5apex.exe!0x017163e0 ConVar r_decal_draw_basis
r5apex.exe!0x01706720 ConVar r_decal_drawclipped
r5apex.exe!0x01718d60 ConVar r_decal_overlap_area
r5apex.exe!0x01718000 ConVar r_decal_overlap_count
r5apex.exe!0x01716520 ConVar r_decal_test_scale
r5apex.exe!0x0170efa0 ConVar r_decals
r5apex.exe!0x01ed1ec0 ConVar r_ditherAlpha
r5apex.exe!0x01851390 ConVar r_ditherFade
r5apex.exe!0x01ef8cf0 ConVar r_ditherFade
r5apex.exe!0x01ed1f60 ConVar r_ditherFadeShadows
r5apex.exe!0x01ef3ab0 ConVar r_ditherFadeShadows
r5apex.exe!0x0228e460 ConVar r_drawallrenderables
r5apex.exe!0x022a8160 ConVar r_drawalphasort
r5apex.exe!0x01714050 ConVar r_drawbrushmodels
r5apex.exe!0x022b1000 ConVar r_drawbrushmodels
r5apex.exe!0x01717d80 ConVar r_drawdecals
r5apex.exe!0x022af980 ConVar r_drawdepth_of_blend2transparent
r5apex.exe!0x01705680 ConVar r_drawdlights
r5apex.exe!0x0170af20 ConVar r_drawentities
r5apex.exe!0x01707290 ConVar r_drawlightdist
r5apex.exe!0x017069e0 ConVar r_drawlightinfo
r5apex.exe!0x01ef4380 ConVar r_drawmodelsinzfill
r5apex.exe!0x01709f10 ConVar r_drawmodelstatsoverlay
r5apex.exe!0x01ee8700 ConVar r_drawmodelstatsoverlay
r5apex.exe!0x01711e20 ConVar r_drawmodelstatsoverlaydistance
r5apex.exe!0x0170edc0 ConVar r_drawmodelstatsoverlayfilter
r5apex.exe!0x0170b900 ConVar r_drawmodelstatsoverlaymax
r5apex.exe!0x0170d8c0 ConVar r_drawmodelstatsoverlaymin
r5apex.exe!0x022b13f0 ConVar r_drawopaquerenderables
r5apex.exe!0x01efdda0 ConVar r_drawothermodels
r5apex.exe!0x022b1350 ConVar r_drawparticles
r5apex.exe!0x01eecfe0 ConVar r_drawrenderboxes
r5apex.exe!0x022a7a20 ConVar r_drawscreenspaceparticles
r5apex.exe!0x022abc50 ConVar r_drawsky
r5apex.exe!0x022ac770 ConVar r_drawskybox_deprecated
r5apex.exe!0x0229de70 ConVar r_drawsprites
r5apex.exe!0x01707120 ConVar r_drawstaticlight
r5apex.exe!0x022af550 ConVar r_drawstaticprops
r5apex.exe!0x0229a920 ConVar r_drawtracers
r5apex.exe!0x017167a0 ConVar r_drawvgui
r5apex.exe!0x022abb70 ConVar r_drawviewmodel
r5apex.exe!0x01712000 ConVar r_drawworld
r5apex.exe!0x01708780 ConVar r_dynamic
r5apex.exe!0x022ada50 ConVar r_earlyRenderables
r5apex.exe!0x01ef3350 ConVar r_enableOriginSort
r5apex.exe!0x022a63b0 ConVar r_fadeincode
r5apex.exe!0x0228ec60 ConVar r_farz
r5apex.exe!0x01711350 ConVar r_fastzreject
r5apex.exe!0x022f44e0 ConVar r_forcecheapwater
r5apex.exe!0x01efcbb0 ConVar r_jiggle_bones
r5apex.exe!0x01712910 ConVar r_lightmap
r5apex.exe!0x01712320 ConVar r_lightprobe_force_trans_dist
r5apex.exe!0x01712c30 ConVar r_lightstyle
r5apex.exe!0x0170fc90 ConVar r_lod
r5apex.exe!0x022acd10 ConVar r_lod
r5apex.exe!0x017110d0 ConVar r_lod_switch_scale
r5apex.exe!0x01ef4f20 ConVar r_mapextents
r5apex.exe!0x01ed4710 ConVar r_modeldecal_maxtotal
r5apex.exe!0x01edebc0 ConVar r_nearz
r5apex.exe!0x01956fd0 ConVar r_no_stalls
r5apex.exe!0x01ed06d0 ConVar r_no_stalls
r5apex.exe!0x01ed0c70 ConVar r_no_stalls
r5apex.exe!0x0170a800 ConVar r_norefresh
r5apex.exe!0x022aa670 ConVar r_particle_lighting_debug
r5apex.exe!0x022a5d60 ConVar r_particle_lighting_enable
r5apex.exe!0x02309960 ConVar r_particle_lighting_enable
r5apex.exe!0x02309800 ConVar r_particle_lighting_force
r5apex.exe!0x02309b50 ConVar r_particle_lighting_force
r5apex.exe!0x022a6590 ConVar r_particle_low_res_debug
r5apex.exe!0x02309a00 ConVar r_particle_low_res_enable
r5apex.exe!0x02309aa0 ConVar r_particle_low_res_force
r5apex.exe!0x01955580 ConVar r_particle_low_res_tiled_composite
r5apex.exe!0x022aa1a0 ConVar r_particle_sim_spike_increment_ms
r5apex.exe!0x022afa20 ConVar r_particle_sim_spike_threshold_ms
r5apex.exe!0x022a5ca0 ConVar r_particle_timescale
r5apex.exe!0x01efcb10 ConVar r_pos_debug
r5apex.exe!0x01ed4990 ConVar r_randomflex
r5apex.exe!0x01ef4120 ConVar r_render_pos_debug
r5apex.exe!0x01851890 ConVar r_rimlight
r5apex.exe!0x01702730 ConVar r_rootlod
r5apex.exe!0x01713220 ConVar r_rootlod
r5apex.exe!0x02296e90 ConVar r_ropetranslucent
r5apex.exe!0x01851930 ConVar r_shadowrendertotexture
r5apex.exe!0x022a80c0 ConVar r_sky_ignoreAngles
r5apex.exe!0x02284c10 ConVar r_sort_trans_debug
r5apex.exe!0x01ef3f80 ConVar r_sort_trans_debug_dist
r5apex.exe!0x022a9140 ConVar r_threaded_particles
r5apex.exe!0x022a7430 ConVar r_updaterefracttexture
r5apex.exe!0x022b0840 ConVar r_updaterefracttexture_allowmultiple
r5apex.exe!0x01705100 ConVar r_visambient
r5apex.exe!0x01706b20 ConVar r_visambient_orig
r5apex.exe!0x01707080 ConVar r_visambient_point
r5apex.exe!0x01709c90 ConVar r_vislighting_sphereradius
r5apex.exe!0x0170c430 ConVar r_vismodellighting
r5apex.exe!0x01718900 ConVar r_vismodellighting_lightpos
r5apex.exe!0x01708920 ConVar r_vismodellighting_maxdist
r5apex.exe!0x022a69b0 ConVar r_vismodellighting_maxdist
r5apex.exe!0x0170fab0 ConVar r_vismodellighting_mindist
r5apex.exe!0x022acb50 ConVar r_vismodellighting_mindist
r5apex.exe!0x017177e0 ConVar r_vismodellighting_offset_x
r5apex.exe!0x01716200 ConVar r_vismodellighting_offset_y
r5apex.exe!0x01717420 ConVar r_vismodellighting_offset_z
r5apex.exe!0x01f010c0 ConVar r_visualizeproplightcaching
r5apex.exe!0x022f24a0 ConVar r_visualizetraces
r5apex.exe!0x022f1f20 ConVar r_visualizetraces_duration
r5apex.exe!0x019568e0 ConVar r_volumetric_lighting_blur_count
r5apex.exe!0x01956840 ConVar r_volumetric_lighting_blur_type
r5apex.exe!0x01956b70 ConVar r_volumetric_lighting_distFalloff
r5apex.exe!0x019567a0 ConVar r_volumetric_lighting_enabled
r5apex.exe!0x01956660 ConVar r_volumetric_lighting_intensity
r5apex.exe!0x01956a30 ConVar r_volumetric_lighting_numSteps
r5apex.exe!0x01956980 ConVar r_volumetric_lighting_rotate_dither
r5apex.exe!0x01956ad0 ConVar r_volumetric_lighting_scatter
r5apex.exe!0x022f4580 ConVar r_waterforceexpensive
r5apex.exe!0x022f48a0 ConVar r_waterforcereflectentities
r5apex.exe!0x022b0ee0 ConVar r_zfill
r5apex.exe!0x01ef2710 ConVar ragdoll_debug
r5apex.exe!0x0236dd90 ConVar ragdoll_debug
r5apex.exe!0x0236d710 ConVar ragdoll_skipDeathAcceleration
r5apex.exe!0x022afd60 ConVar ragdoll_sleepaftertime
r5apex.exe!0x022a8dd0 ConVar rankedplay_display_enabled
r5apex.exe!0x022a6650 ConVar rankedplay_voice_enabled
r5apex.exe!0x017dda30 ConVar rate
r5apex.exe!0x022961a0 ConVar real_time_update_dt
r5apex.exe!0x0171ba10 ConVar recalculateOrigin_threaded_chunksize
r5apex.exe!0x023eb270 ConVar reduced_trigger_checks
r5apex.exe!0x027794c0 ConVar reliable_effects_enable
r5apex.exe!0x022f90d0 ConVar remoteCalls_requireConnectionScriptsForViewPlayer
r5apex.exe!0x017151d0 ConVar remoteMatchInfo_print
r5apex.exe!0x017196b0 ConVar replay_enable
r5apex.exe!0x01716f20 ConVar replay_prediction_smooth
r5apex.exe!0x02282e10 ConVar report_cliententitysim
r5apex.exe!0x01ef3e40 ConVar report_clientthinklist
r5apex.exe!0x02785b90 ConVar requestBestObserverTargetFromScript
r5apex.exe!0x0228e280 ConVar rodeo_camera_smooth_blend_out_time
r5apex.exe!0x01ef64e0 ConVar rodeo_camera_smooth_enable
r5apex.exe!0x02368c30 ConVar rodeoed_anim_weight
r5apex.exe!0x022f5600 ConVar rodeoed_anims_enabled
r5apex.exe!0x022a4210 ConVar rope_collide
r5apex.exe!0x022a27e0 ConVar rope_debug_shake
r5apex.exe!0x0276bd50 ConVar rope_default_segment_length
r5apex.exe!0x02296460 ConVar rope_shake
r5apex.exe!0x022a2ff0 ConVar rope_texels_per_world_unit
r5apex.exe!0x0228ffa0 ConVar rope_wiggle_harmonic_falloff
r5apex.exe!0x022a4c90 ConVar rope_wiggle_magnitude_loose
r5apex.exe!0x02296f30 ConVar rope_wiggle_magnitude_tight
r5apex.exe!0x0229fdb0 ConVar rope_wiggle_oscillate_speed
r5apex.exe!0x0228fc20 ConVar rope_wiggle_rotate_speed
r5apex.exe!0x0229bf30 ConVar rope_wiggle_zipline_min_points
r5apex.exe!0x0228f660 ConVar rope_wind_dist
r5apex.exe!0x01ef5780 ConVar rotate_ents
r5apex.exe!0x017095c0 ConVar rspn_motd
r5apex.exe!0x01953cc0 ConVar rt_sync_message_pump
r5apex.exe!0x01953d60 ConVar rt_worker
r5apex.exe!0x022aef40 ConVar rui_asyncTracks
r5apex.exe!0x01ed95e0 ConVar rui_defaultDebugFontFace
r5apex.exe!0x01ed99a0 ConVar rui_defaultFontFace
r5apex.exe!0x01ed8720 ConVar rui_defaultFontHeight
r5apex.exe!0x01ed7f20 ConVar rui_overrideVguiTextRendering
r5apex.exe!0x01814d10 ConVar rui_padDist
r5apex.exe!0x01814c70 ConVar rui_safeAreaFrac
r5apex.exe!0x01715ac0 ConVar rui_standardTextHeight
r5apex.exe!0x0170a2b0 ConVar s2sPort
r5apex.exe!0x0235cde0 ConVar save_client_entity
r5apex.exe!0x01818a40 ConVar save_enable
r5apex.exe!0x02357640 ConVar save_thread_entities
r5apex.exe!0x0278dfa0 ConVar scene_clamplookat
r5apex.exe!0x01ed8210 ConVar scheme_manager_font_debug
r5apex.exe!0x022ab600 ConVar scr_centertime
r5apex.exe!0x01eeada0 ConVar screen_indicator_back_range
r5apex.exe!0x01efe4e0 ConVar screen_indicator_ellipse_height
r5apex.exe!0x01edeef0 ConVar screen_indicator_ellipse_width
r5apex.exe!0x01eecce0 ConVar screen_indicator_pitch_limit
r5apex.exe!0x01eed240 ConVar screen_indicator_pitch_scale
r5apex.exe!0x022ae980 ConVar screenfade_debug
r5apex.exe!0x02351780 ConVar script_compile_all_levels
r5apex.exe!0x022ac230 ConVar script_debugger_connect_client_on_mapspawn
r5apex.exe!0x02774230 ConVar script_debugger_connect_server_on_mapspawn
r5apex.exe!0x022b0e40 ConVar script_debugger_connect_ui_auto
r5apex.exe!0x027a50b0 ConVar script_debugger_host
r5apex.exe!0x027a5150 ConVar script_debugger_port_client
r5apex.exe!0x027a5010 ConVar script_debugger_port_server
r5apex.exe!0x027a4ed0 ConVar script_debugger_port_ui
r5apex.exe!0x027a5470 ConVar script_disallow_newslot_on_globals
r5apex.exe!0x027a4f70 ConVar script_dump_simple
r5apex.exe!0x02359950 ConVar script_error_on_midgame_load
r5apex.exe!0x027a4e30 ConVar script_infinite_loop_ms
r5apex.exe!0x02368360 ConVar script_parallel_trace_LOS_multiple
r5apex.exe!0x0234ec10 ConVar script_precache_errors
r5apex.exe!0x022a64f0 ConVar script_printDeferredCalls
r5apex.exe!0x02368af0 ConVar script_retry_after_compile_errors
r5apex.exe!0x02775f60 ConVar script_server_fps
r5apex.exe!0x022f5ea0 ConVar script_showErrorDialogs
r5apex.exe!0x01708a60 ConVar script_slopTimeBeforeBudgetEnforcement
r5apex.exe!0x02774190 ConVar send_data_to_all_players
r5apex.exe!0x023669e0 ConVar sequence_transitioner_enable
r5apex.exe!0x01713d30 ConVar serverFilter
r5apex.exe!0x01843b50 ConVar serverReports_hostname
r5apex.exe!0x01704970 ConVar server_concommands_allways_network
r5apex.exe!0x023e9390 ConVar server_helicopter_rope_events
r5apex.exe!0x018457c0 ConVar server_query_interval
r5apex.exe!0x0228ebc0 ConVar sfm_record_hz
r5apex.exe!0x01ef2850 ConVar shadow_always_update
r5apex.exe!0x01ed08b0 ConVar shadow_bleedfudge
r5apex.exe!0x0170a6c0 ConVar shadow_capable
r5apex.exe!0x022acfd0 ConVar shadow_clear_dist
r5apex.exe!0x01eef000 ConVar shadow_dbg_draw
r5apex.exe!0x01eeeec0 ConVar shadow_default_filter_size
r5apex.exe!0x02282cd0 ConVar shadow_depth_dimen_min
r5apex.exe!0x01ef1720 ConVar shadow_depth_upres_factor_max
r5apex.exe!0x01efc0a0 ConVar shadow_drawfrustum
r5apex.exe!0x01ee9960 ConVar shadow_dynamic_blendfactor
r5apex.exe!0x01711ac0 ConVar shadow_enable
r5apex.exe!0x01ef41c0 ConVar shadow_esm_enable
r5apex.exe!0x01ee8040 ConVar shadow_filter_maxstep
r5apex.exe!0x0228da80 ConVar shadow_info
r5apex.exe!0x022820a0 ConVar shadow_lobby_mode_allowed
r5apex.exe!0x01ef0880 ConVar shadow_max_dynamic
r5apex.exe!0x01eeca60 ConVar shadow_max_old_dynamic
r5apex.exe!0x01ef6580 ConVar shadow_max_spot_updates
r5apex.exe!0x0184e4b0 ConVar shadow_maxdynamic
r5apex.exe!0x01ef2990 ConVar shadow_min_count_smallest
r5apex.exe!0x01ed0a90 ConVar shadow_minvariance
r5apex.exe!0x0228e640 ConVar shadow_multisampled
r5apex.exe!0x01eead00 ConVar shadow_noLOD
r5apex.exe!0x02282440 ConVar shadow_show_spot_udpate_infos
r5apex.exe!0x01efc7e0 ConVar shadow_tools_depth_dimen_min
r5apex.exe!0x01ef3490 ConVar shadow_tools_depth_upres_factor_max
r5apex.exe!0x01ef9ea0 ConVar shadow_tools_min_count_smallest
r5apex.exe!0x02282280 ConVar shadow_tools_mode
r5apex.exe!0x01efd770 ConVar shadow_update_culling
r5apex.exe!0x02281c60 ConVar shake_angleFactor_human
r5apex.exe!0x01eed900 ConVar shake_angleFactor_titan
r5apex.exe!0x022a8870 ConVar shake_basicPitchFactor
r5apex.exe!0x022ab540 ConVar shake_basicRandomRollFactor
r5apex.exe!0x01ef8c50 ConVar shake_offsetFactor_human
r5apex.exe!0x01ee9be0 ConVar shake_offsetFactor_titan
r5apex.exe!0x02282fd0 ConVar shake_viewmodelFactor_ads_human
r5apex.exe!0x022823a0 ConVar shake_viewmodelFactor_ads_titan
r5apex.exe!0x01eeac60 ConVar shake_viewmodelFactor_human
r5apex.exe!0x02283070 ConVar shake_viewmodelFactor_titan
r5apex.exe!0x01851bb0 ConVar showfps_enabled
r5apex.exe!0x0184eb90 ConVar showfps_heightpercent
r5apex.exe!0x0184c0e0 ConVar showfps_mouse_latency
r5apex.exe!0x018512f0 ConVar showfps_smoothtime
r5apex.exe!0x018521f0 ConVar showfps_spinner
r5apex.exe!0x0278e800 ConVar showhitlocation
r5apex.exe!0x0184ee10 ConVar showmem_enabled
r5apex.exe!0x01851ed0 ConVar shownet_enabled
r5apex.exe!0x01851b10 ConVar showsnapshot_enabled
r5apex.exe!0x02788560 ConVar showtriggers
r5apex.exe!0x02785750 ConVar showtriggers_distance
r5apex.exe!0x02776e60 ConVar showtriggers_entindex
r5apex.exe!0x02295880 ConVar sidearmSwapSelectCooldown
r5apex.exe!0x0229fa50 ConVar sidearmSwapSelectDoubleTapTime
r5apex.exe!0x0170b680 ConVar single_frame_shutdown_for_reload
r5apex.exe!0x01712b90 ConVar singlestep
r5apex.exe!0x02791460 ConVar sk_bullseye_health
r5apex.exe!0x02369020 ConVar sk_healthcharger
r5apex.exe!0x01717240 ConVar skill_arena
r5apex.exe!0x01716d40 ConVar skill_dediOnly
r5apex.exe!0x017181e0 ConVar skill_enabled
r5apex.exe!0x0170b7c0 ConVar skill_hostname
r5apex.exe!0x022b5770 ConVar skip_jump_height_fraction
r5apex.exe!0x0230e340 ConVar skip_jump_height_fraction
r5apex.exe!0x022e3d70 ConVar skip_jump_height_speed
r5apex.exe!0x02336980 ConVar skip_jump_height_speed
r5apex.exe!0x022e4b90 ConVar skip_replenish_double_jump
r5apex.exe!0x02347980 ConVar skip_replenish_double_jump
r5apex.exe!0x022b4780 ConVar skip_sounds
r5apex.exe!0x0230d500 ConVar skip_sounds
r5apex.exe!0x022e4910 ConVar skip_speed_reduce
r5apex.exe!0x02347630 ConVar skip_speed_reduce
r5apex.exe!0x022e2da0 ConVar skip_speed_retain
r5apex.exe!0x02335440 ConVar skip_speed_retain
r5apex.exe!0x022e5090 ConVar skip_time
r5apex.exe!0x02347f20 ConVar skip_time
r5apex.exe!0x01715b60 ConVar sleep_when_meeting_framerate
r5apex.exe!0x01718a40 ConVar sleep_when_meeting_framerate_headroom_ms
r5apex.exe!0x022e29a0 ConVar slide_auto_stand
r5apex.exe!0x02334fa0 ConVar slide_auto_stand
r5apex.exe!0x022b3c70 ConVar slide_max_angle_dot
r5apex.exe!0x0230c8f0 ConVar slide_max_angle_dot
r5apex.exe!0x022b58b0 ConVar slide_step_velocity_reduction
r5apex.exe!0x0230e4b0 ConVar slide_step_velocity_reduction
r5apex.exe!0x022eedd0 ConVar slide_viewTiltDecreaseSpeed
r5apex.exe!0x022ef7b0 ConVar slide_viewTiltIncreaseSpeed
r5apex.exe!0x022e6370 ConVar slide_viewTiltPlayerSpeed
r5apex.exe!0x022ed120 ConVar slide_viewTiltSide
r5apex.exe!0x022dea10 ConVar slide_whileInAir
r5apex.exe!0x0232fce0 ConVar slide_whileInAir
r5apex.exe!0x01706680 ConVar slowconsolelog_old_logic
r5apex.exe!0x02796410 ConVar smart_ammo_debug
r5apex.exe!0x02301510 ConVar smart_ammo_interp_entity_fields
r5apex.exe!0x022ef470 ConVar smoothstairs_lunge
r5apex.exe!0x0235ccd0 ConVar smoothstairs_lunge
r5apex.exe!0x01705a50 ConVar sort_opaque_meshes
r5apex.exe!0x01eef800 ConVar sound_classic_music
r5apex.exe!0x022a5ae0 ConVar sound_entity_seek_snap
r5apex.exe!0x022ea2c0 ConVar sound_musicReduced
r5apex.exe!0x0228ed00 ConVar sound_num_speakers
r5apex.exe!0x022f8060 ConVar sound_only_warn_on_missing_sound_events_in_client_script
r5apex.exe!0x017ddc70 ConVar sound_printloaderrors
r5apex.exe!0x01ee9aa0 ConVar sound_volume
r5apex.exe!0x01ef47e0 ConVar sound_volume_dialogue
r5apex.exe!0x01ef4740 ConVar sound_volume_dialogue_sp
r5apex.exe!0x01ef7270 ConVar sound_volume_music_game
r5apex.exe!0x01ef4e80 ConVar sound_volume_music_game_sp
r5apex.exe!0x01ef4020 ConVar sound_volume_music_lobby
r5apex.exe!0x01ede300 ConVar sound_volume_sfx
r5apex.exe!0x02284550 ConVar sound_volume_sfx_sp
r5apex.exe!0x022847b0 ConVar sound_volume_voice
r5apex.exe!0x01efd810 ConVar sound_without_focus
r5apex.exe!0x027760a0 ConVar soundscape_debug
r5apex.exe!0x0229f930 ConVar soundscape_fadetime
r5apex.exe!0x0229ddc0 ConVar soundscape_message
r5apex.exe!0x022a0e50 ConVar soundscape_radius_debug
r5apex.exe!0x01ef56e0 ConVar soundtrigger_repeat_interval
r5apex.exe!0x01715270 ConVar sp_not_focus_pause
r5apex.exe!0x01ed04f0 ConVar spam_skinning_matrices_used
r5apex.exe!0x01ed0630 ConVar spam_skinning_matrices_used_detailed
r5apex.exe!0x0234dc70 ConVar spatial_partition_deadlock_assert
r5apex.exe!0x02778820 ConVar spawnpoint_avoid_npc_titan_sight
r5apex.exe!0x027719b0 ConVar spawnpoint_enemy_ai_far_dist
r5apex.exe!0x0277db00 ConVar spawnpoint_enemy_ai_near_dist
r5apex.exe!0x02782e40 ConVar spawnpoint_enemy_titan_far_dist
r5apex.exe!0x02777cc0 ConVar spawnpoint_enemy_titan_near_dist
r5apex.exe!0x02783e00 ConVar spawnpoint_enemy_wallrun_far_dist
r5apex.exe!0x02785100 ConVar spawnpoint_enemy_wallrun_near_dist
r5apex.exe!0x027747e0 ConVar spawnpoint_friendly_ai_far_dist
r5apex.exe!0x02771a50 ConVar spawnpoint_friendly_ai_near_dist
r5apex.exe!0x02784b50 ConVar spawnpoint_friendly_titan_far_dist
r5apex.exe!0x02778270 ConVar spawnpoint_friendly_titan_near_dist
r5apex.exe!0x0276b880 ConVar spawnpoint_friendly_wallrun_far_dist
r5apex.exe!0x027875d0 ConVar spawnpoint_friendly_wallrun_near_dist
r5apex.exe!0x02783d60 ConVar spawnpoint_last_spawn_rating
r5apex.exe!0x02779ba0 ConVar spawnpoint_pet_titan_far_dist
r5apex.exe!0x027838f0 ConVar spawnpoint_pet_titan_near_dist
r5apex.exe!0x02777030 ConVar spawnpoint_show_all
r5apex.exe!0x02778dd0 ConVar spawnpoint_show_class
r5apex.exe!0x027859d0 ConVar spawnpoint_show_dist
r5apex.exe!0x027725e0 ConVar spawnpoint_show_sight
r5apex.exe!0x02775910 ConVar spawnpoint_text_dist
r5apex.exe!0x027752a0 ConVar spawnpoint_text_dynamic
r5apex.exe!0x02776670 ConVar spawnpoint_text_team
r5apex.exe!0x027759b0 ConVar spawnpoint_velocity_predict_time
r5apex.exe!0x02786640 ConVar spec_chasecam_wait_on_dead_player_duration
r5apex.exe!0x022fa190 ConVar speech_queue_bytes
r5apex.exe!0x01847420 ConVar speechtotext_audioenabled
r5apex.exe!0x018472e0 ConVar speechtotext_enabled
r5apex.exe!0x01847240 ConVar speechtotext_forcedisabled
r5apex.exe!0x01847060 ConVar speechtotext_hostname
r5apex.exe!0x018471a0 ConVar speechtotext_msg_droptimeout
r5apex.exe!0x018461c0 ConVar speechtotext_path
r5apex.exe!0x01847380 ConVar speechtotext_quiettime
r5apex.exe!0x01846de0 ConVar speechtotext_stats_errorspermin
r5apex.exe!0x01847100 ConVar speechtotext_stats_interval
r5apex.exe!0x01846f20 ConVar speechtotext_stats_senderrors
r5apex.exe!0x01846e80 ConVar speechtotext_stats_sendrequests
r5apex.exe!0x01846fc0 ConVar speechtotext_stats_sendsuccess
r5apex.exe!0x0170d640 ConVar speechtotexttoken_hostname
r5apex.exe!0x0171a770 ConVar speex_audio_recording
r5apex.exe!0x017199d0 ConVar speex_audio_value
r5apex.exe!0x01849390 ConVar speex_preprocess_agc_max_gain
r5apex.exe!0x01848fd0 ConVar speex_preprocess_noise_suppress
r5apex.exe!0x018492f0 ConVar speex_preprocess_set_agc_decrenment
r5apex.exe!0x01848f30 ConVar speex_preprocess_set_agc_increment
r5apex.exe!0x018491b0 ConVar speex_preprocess_set_agc_target
r5apex.exe!0x01719bb0 ConVar speex_quiet_threshold
r5apex.exe!0x01719cf0 ConVar speex_quiet_window
r5apex.exe!0x01849070 ConVar speex_set_enh
r5apex.exe!0x01849110 ConVar speex_use_highpass
r5apex.exe!0x01849250 ConVar speex_use_preproser
r5apex.exe!0x022f2540 ConVar spinner_debug_info
r5apex.exe!0x022eafb0 ConVar sprint_powerdrain
r5apex.exe!0x02357dd0 ConVar sprint_powerdrain
r5apex.exe!0x022826a0 ConVar sprint_view_shake_style
r5apex.exe!0x0234c4d0 ConVar sprinttilt_accel
r5apex.exe!0x0234f280 ConVar sprinttilt_maxvel
r5apex.exe!0x0234c090 ConVar sprinttilt_turnrange
r5apex.exe!0x022a9ea0 ConVar ss_enable
r5apex.exe!0x022a68f0 ConVar ss_force_primary_fullscreen
r5apex.exe!0x0229df90 ConVar ss_mimic
r5apex.exe!0x022aa2e0 ConVar ss_splitmode
r5apex.exe!0x022ae270 ConVar ss_verticalsplit
r5apex.exe!0x0228e0a0 ConVar ss_viewmodelfov
r5apex.exe!0x01816db0 ConVar ss_voice_hearpartner
r5apex.exe!0x01955940 ConVar ssao_allow_partial
r5apex.exe!0x01955bc0 ConVar ssao_blur
r5apex.exe!0x01955f80 ConVar ssao_blur_edge_sharpness
r5apex.exe!0x01955da0 ConVar ssao_depth_max
r5apex.exe!0x01955800 ConVar ssao_downsample
r5apex.exe!0x01955b20 ConVar ssao_enabled
r5apex.exe!0x01955e40 ConVar ssao_exponent
r5apex.exe!0x01955760 ConVar ssao_jitter_scale
r5apex.exe!0x019560c0 ConVar ssao_max_res
r5apex.exe!0x019559e0 ConVar ssao_max_res_threshold
r5apex.exe!0x019556c0 ConVar ssao_num_directions
r5apex.exe!0x01956480 ConVar ssao_num_steps
r5apex.exe!0x01956340 ConVar ssao_on_everything
r5apex.exe!0x01955620 ConVar ssao_radius
r5apex.exe!0x019562a0 ConVar ssao_show
r5apex.exe!0x019565c0 ConVar ssao_show
r5apex.exe!0x01ed3400 ConVar ssao_show
r5apex.exe!0x019558a0 ConVar ssao_snap_uv
r5apex.exe!0x01956020 ConVar ssao_tech
r5apex.exe!0x022ad2f0 ConVar ssao_tech
r5apex.exe!0x01956520 ConVar ssao_upsample_ranged
r5apex.exe!0x01717100 ConVar startButtonCommand
r5apex.exe!0x01715d80 ConVar staticProp_budget
r5apex.exe!0x017172e0 ConVar staticProp_debug_draw
r5apex.exe!0x01715a20 ConVar staticProp_earlyDepthPrepass
r5apex.exe!0x017194d0 ConVar staticProp_earlyDepthPrepassDist
r5apex.exe!0x01717060 ConVar staticProp_earlyDepthPrepassIncludeOpaques
r5apex.exe!0x01718e00 ConVar staticProp_earlyDepthPrepassIncludeOpaquesDist
r5apex.exe!0x01718680 ConVar staticProp_gather_size_weight
r5apex.exe!0x01715ec0 ConVar staticProp_max_scaled_dist
r5apex.exe!0x01718ea0 ConVar staticProp_no_fade_scalar
r5apex.exe!0x022b0520 ConVar staticProp_refineDrawOnWorker
r5apex.exe!0x0184e2d0 ConVar static_shadow
r5apex.exe!0x01edf030 ConVar static_shadow
r5apex.exe!0x01ef4600 ConVar static_shadow_bounds_per_env
r5apex.exe!0x022ab360 ConVar static_shadow_debug_2d
r5apex.exe!0x01ef1ad0 ConVar static_shadow_debug_dirty_rects
r5apex.exe!0x0228e320 ConVar static_shadow_depth_bias_scale
r5apex.exe!0x01eecc40 ConVar static_shadow_expand_z
r5apex.exe!0x01ef1990 ConVar static_shadow_good_merge_ratio
r5apex.exe!0x01efd170 ConVar static_shadow_good_merge_score
r5apex.exe!0x022845f0 ConVar static_shadow_prop_min_size
r5apex.exe!0x01851d90 ConVar static_shadow_res
r5apex.exe!0x01eeef60 ConVar static_shadow_shrink_culler
r5apex.exe!0x01852010 ConVar static_shadow_use_d16
r5apex.exe!0x01ee9f00 ConVar static_shadow_uses_shadow_lod
r5apex.exe!0x0170cdb0 ConVar staticfile_hostname
r5apex.exe!0x0170eb40 ConVar stats_hostname
r5apex.exe!0x022ea180 ConVar status_effect_warning_level
r5apex.exe!0x02356d60 ConVar status_effect_warning_level
r5apex.exe!0x01847560 ConVar steam_id
r5apex.exe!0x01847740 ConVar steam_name
r5apex.exe!0x01847880 ConVar steamlink_hostname
r5apex.exe!0x019543f0 ConVar stream_addnoise
r5apex.exe!0x019540d0 ConVar stream_bsp_bucket_bias
r5apex.exe!0x01954850 ConVar stream_bsp_dist_scale
r5apex.exe!0x01702af0 ConVar stream_cache_capacity
r5apex.exe!0x017027d0 ConVar stream_cache_high_priority_static_models
r5apex.exe!0x017023d0 ConVar stream_cache_multithreaded
r5apex.exe!0x017025b0 ConVar stream_cache_preload_from_rpak
r5apex.exe!0x01702a50 ConVar stream_cache_read_buffer_cap
r5apex.exe!0x01702b90 ConVar stream_cache_read_count_cap
r5apex.exe!0x017029b0 ConVar stream_cache_speculative_add_level
r5apex.exe!0x01702c30 ConVar stream_cache_speculative_drop
r5apex.exe!0x01954350 ConVar stream_drop_unused
r5apex.exe!0x01954170 ConVar stream_enable
r5apex.exe!0x0184e5f0 ConVar stream_freeze_camera
r5apex.exe!0x01954030 ConVar stream_load_after_drop
r5apex.exe!0x019548f0 ConVar stream_memory
r5apex.exe!0x019545d0 ConVar stream_memory_ignore
r5apex.exe!0x01954530 ConVar stream_memory_ignore_vram
r5apex.exe!0x01954490 ConVar stream_memory_while_loading
r5apex.exe!0x01954670 ConVar stream_mode
r5apex.exe!0x01954710 ConVar stream_never_high_priority_frac
r5apex.exe!0x01954990 ConVar stream_overlay
r5apex.exe!0x019547b0 ConVar stream_overlay_mode
r5apex.exe!0x019542b0 ConVar stream_pause
r5apex.exe!0x01954210 ConVar stream_picmip
r5apex.exe!0x01954e90 ConVar stream_resource_max_commits_per_frame
r5apex.exe!0x01954f30 ConVar stream_resource_thread
r5apex.exe!0x01954df0 ConVar stream_resource_wait_copy_to_commit
r5apex.exe!0x01954fd0 ConVar stream_resource_wait_creation_to_copy
r5apex.exe!0x01954d50 ConVar stream_resource_wait_for_additional_gpus
r5apex.exe!0x017145d0 ConVar stringtable_alwaysrebuilddictionaries
r5apex.exe!0x0170f220 ConVar stringtable_compress
r5apex.exe!0x01710310 ConVar stringtable_showsizes
r5apex.exe!0x01844ca0 ConVar stryder_forceOriginUsersInvisible
r5apex.exe!0x01843e70 ConVar stryder_security
r5apex.exe!0x0230e290 ConVar stuck_debugging
r5apex.exe!0x0234b700 ConVar stuck_debugging_world_only
r5apex.exe!0x01701e10 ConVar studiobonecache_unlimited
r5apex.exe!0x01843bf0 ConVar subscription_hostname
r5apex.exe!0x022e14f0 ConVar superjump_disabled_from_water
r5apex.exe!0x023333e0 ConVar superjump_disabled_from_water
r5apex.exe!0x022e4ff0 ConVar superjump_drain_power_onfail
r5apex.exe!0x02347e80 ConVar superjump_drain_power_onfail
r5apex.exe!0x022e20e0 ConVar superjump_fail_sound_when_jump_limit
r5apex.exe!0x02333fb0 ConVar superjump_fail_sound_when_jump_limit
r5apex.exe!0x023354e0 ConVar superjump_limit
r5apex.exe!0x0234b260 ConVar superjump_limitreset_onwallrun
r5apex.exe!0x0234e760 ConVar superjump_max_power_use
r5apex.exe!0x0234bcb0 ConVar superjump_min_height_fraction
r5apex.exe!0x0234f660 ConVar superjump_min_power_use
r5apex.exe!0x0234b0d0 ConVar superjump_powerreset_onground
r5apex.exe!0x02334950 ConVar sv_airaccelerate
r5apex.exe!0x0170dbc0 ConVar sv_allTicksFinal
r5apex.exe!0x01815bf0 ConVar sv_allowSendTableTransmitToClients
r5apex.exe!0x02368f40 ConVar sv_alltalk
r5apex.exe!0x023968b0 ConVar sv_asyncAIInit
r5apex.exe!0x01816bd0 ConVar sv_asyncSendSnapshot
r5apex.exe!0x0230dab0 ConVar sv_backspeed
r5apex.exe!0x01816ef0 ConVar sv_balanceTeams
r5apex.exe!0x023272d0 ConVar sv_bounce
r5apex.exe!0x0230af30 ConVar sv_bounds_show_errors
r5apex.exe!0x023e9bb0 ConVar sv_calcOriginsAnglesForSnapshotPacking
r5apex.exe!0x01818f40 ConVar sv_cheats
r5apex.exe!0x018162d0 ConVar sv_checkPropBudgets
r5apex.exe!0x0277c380 ConVar sv_clampPlayerFrameTime
r5apex.exe!0x02785d60 ConVar sv_clockcorrection
r5apex.exe!0x02785930 ConVar sv_clockcorrection_msecs
r5apex.exe!0x01816670 ConVar sv_compressPlaylists
r5apex.exe!0x02369650 ConVar sv_compressTimeValEpsilon
r5apex.exe!0x023696f0 ConVar sv_compressTimeVals
r5apex.exe!0x01815e70 ConVar sv_connectingClientDelay
r5apex.exe!0x027954f0 ConVar sv_crossbowBoltAutoCull
r5apex.exe!0x0235cbf0 ConVar sv_debug_deferred_trace
r5apex.exe!0x02366580 ConVar sv_debug_deferred_trace_overlay
r5apex.exe!0x018169f0 ConVar sv_debug_prop_send
r5apex.exe!0x01818680 ConVar sv_debugmanualmode
r5apex.exe!0x01818ae0 ConVar sv_disconnectOnTooManySnapshotFrames
r5apex.exe!0x02396c50 ConVar sv_dispatchSpawnsForBaseline
r5apex.exe!0x023eac60 ConVar sv_distanceCull
r5apex.exe!0x023e9170 ConVar sv_distanceCull_cellWidth
r5apex.exe!0x023a7760 ConVar sv_distanceCull_debug
r5apex.exe!0x023a7e40 ConVar sv_distanceCull_debugPlayerEntindex
r5apex.exe!0x023eaa90 ConVar sv_distanceCull_largeEntRadius
r5apex.exe!0x0170aca0 ConVar sv_dumpstringtables
r5apex.exe!0x01818900 ConVar sv_earlyPersistenceRead
r5apex.exe!0x017106b0 ConVar sv_everyThirdTick
r5apex.exe!0x01817f00 ConVar sv_extra_client_connect_time
r5apex.exe!0x02350830 ConVar sv_footsteps
r5apex.exe!0x023a8d50 ConVar sv_forceChatToTeamOnly
r5apex.exe!0x0230e690 ConVar sv_forceGrapplesToFail
r5apex.exe!0x0234cb70 ConVar sv_friction
r5apex.exe!0x02350dc0 ConVar sv_gravity
r5apex.exe!0x01815790 ConVar sv_hibernate_ms
r5apex.exe!0x01816490 ConVar sv_hibernate_ms_vgui
r5apex.exe!0x018189a0 ConVar sv_hibernate_postgame_delay
r5apex.exe!0x018182c0 ConVar sv_hibernate_when_empty
r5apex.exe!0x01817b40 ConVar sv_instancebaselines
r5apex.exe!0x0236c630 ConVar sv_interpolateAnimatedEntitiesPerJob
r5apex.exe!0x023e9a70 ConVar sv_kickPlayersTooFarInFuture
r5apex.exe!0x02783190 ConVar sv_lagpushticks
r5apex.exe!0x0236b930 ConVar sv_lerpAnims
r5apex.exe!0x01708260 ConVar sv_loadMapModelEarly
r5apex.exe!0x01711b60 ConVar sv_lobbyType
r5apex.exe!0x023ead00 ConVar sv_massreport
r5apex.exe!0x0277bdd0 ConVar sv_maxUserCmdsPerPlayerPerFrame
r5apex.exe!0x018163f0 ConVar sv_max_prop_data_dwords_huge_lobby
r5apex.exe!0x01817dc0 ConVar sv_max_prop_data_dwords_huge_multiplayer
r5apex.exe!0x01816230 ConVar sv_max_prop_data_dwords_lobby
r5apex.exe!0x01816c70 ConVar sv_max_prop_data_dwords_multiplayer
r5apex.exe!0x01815fb0 ConVar sv_max_prop_data_dwords_singleplayer
r5apex.exe!0x01818d60 ConVar sv_max_props_huge_lobby
r5apex.exe!0x01815830 ConVar sv_max_props_huge_multiplayer
r5apex.exe!0x018165d0 ConVar sv_max_props_lobby
r5apex.exe!0x01815f10 ConVar sv_max_props_multiplayer
r5apex.exe!0x01815d30 ConVar sv_max_props_singleplayer
r5apex.exe!0x01817a00 ConVar sv_max_snapshots_lobby
r5apex.exe!0x018150d0 ConVar sv_max_snapshots_multiplayer
r5apex.exe!0x01818c20 ConVar sv_max_snapshots_singleplayer
r5apex.exe!0x01816830 ConVar sv_maxclientframes
r5apex.exe!0x01818220 ConVar sv_maxrate
r5apex.exe!0x0170e5a0 ConVar sv_maxroutable
r5apex.exe!0x0234fd10 ConVar sv_maxspeed
r5apex.exe!0x02789110 ConVar sv_maxunlag
r5apex.exe!0x018178c0 ConVar sv_maxupdaterate
r5apex.exe!0x0234fde0 ConVar sv_maxvelocity
r5apex.exe!0x01816710 ConVar sv_minrate
r5apex.exe!0x01816b30 ConVar sv_minupdaterate
r5apex.exe!0x0236e6a0 ConVar sv_netvisdist
r5apex.exe!0x0234d640 ConVar sv_noclipaccelerate
r5apex.exe!0x0234ea30 ConVar sv_noclipaccelerate_fast
r5apex.exe!0x023502f0 ConVar sv_noclipaccelerate_slow
r5apex.exe!0x0234fab0 ConVar sv_noclipspeed
r5apex.exe!0x0234e660 ConVar sv_noclipspeed_fast
r5apex.exe!0x0234e2b0 ConVar sv_noclipspeed_slow
r5apex.exe!0x0277d4c0 ConVar sv_normalSimulationCommandThreshold
r5apex.exe!0x0234f4c0 ConVar sv_optimizedmovement
r5apex.exe!0x01815a10 ConVar sv_parallel_sendsnapshot
r5apex.exe!0x018156f0 ConVar sv_pausable
r5apex.exe!0x02395020 ConVar sv_physics_maxvelocity
r5apex.exe!0x01814ef0 ConVar sv_playerNameAppendCheater
r5apex.exe!0x0277a330 ConVar sv_playerSimTimeBuffer
r5apex.exe!0x02347c00 ConVar sv_players
r5apex.exe!0x02775340 ConVar sv_printClockCorrections
r5apex.exe!0x02779c40 ConVar sv_printClockTiming
r5apex.exe!0x01817c80 ConVar sv_printHighWaterMark
r5apex.exe!0x023eab30 ConVar sv_printNetReports
r5apex.exe!0x023a8020 ConVar sv_printSnapshotDeltaStats
r5apex.exe!0x0276baf0 ConVar sv_props_funnel_into_portals
r5apex.exe!0x02785e00 ConVar sv_props_funnel_into_portals_deceleration
r5apex.exe!0x0234ee20 ConVar sv_pushaway_accel
r5apex.exe!0x0234c300 ConVar sv_pushaway_clientside
r5apex.exe!0x02357500 ConVar sv_pushaway_clientside_size
r5apex.exe!0x0234f560 ConVar sv_pushaway_debug
r5apex.exe!0x0234ce50 ConVar sv_pushaway_dist
r5apex.exe!0x0234d5a0 ConVar sv_pushaway_min_player_speed
r5apex.exe!0x0234f350 ConVar sv_pushaway_player_accel
r5apex.exe!0x0234d150 ConVar sv_pushaway_player_dist
r5apex.exe!0x02371b20 ConVar sv_recalcOrigins_enabled
r5apex.exe!0x02396130 ConVar sv_recalcOrigins_entsPerJob
r5apex.exe!0x01817150 ConVar sv_rejectClientConnects
r5apex.exe!0x01815470 ConVar sv_rejectConnections
r5apex.exe!0x01816050 ConVar sv_requireOriginToken
r5apex.exe!0x018171f0 ConVar sv_resendSignonData
r5apex.exe!0x0234aae0 ConVar sv_rollangle
r5apex.exe!0x0234b430 ConVar sv_rollspeed
r5apex.exe!0x018153d0 ConVar sv_runSpatialOptimizeInJob
r5apex.exe!0x01817960 ConVar sv_scarySnapDeltaPrints
r5apex.exe!0x0276ba50 ConVar sv_screenShake_debug
r5apex.exe!0x02783ad0 ConVar sv_screenShake_enabled
r5apex.exe!0x02779420 ConVar sv_screenShake_maxAmplitude
r5apex.exe!0x0277ead0 ConVar sv_scriptCompileAsync
r5apex.exe!0x02787c20 ConVar sv_script_perf_dump_on_shutdown
r5apex.exe!0x0236e1f0 ConVar sv_script_think_interval
r5apex.exe!0x018184a0 ConVar sv_sendEarlyServerInfo
r5apex.exe!0x02787cc0 ConVar sv_sendPlayerDamageMsg
r5apex.exe!0x01816530 ConVar sv_sendReplayNetMessagesOnNoDeltaSnaps
r5apex.exe!0x01817330 ConVar sv_separate_freq_change_prop_send
r5apex.exe!0x027857f0 ConVar sv_shiftPlayerSimTimeBackwards
r5apex.exe!0x018158d0 ConVar sv_showClientTickCmds
r5apex.exe!0x01815170 ConVar sv_showLargeSnapshotSize
r5apex.exe!0x01814e50 ConVar sv_showSnapshots
r5apex.exe!0x01815030 ConVar sv_showUserCmds
r5apex.exe!0x02371db0 ConVar sv_showWeirdDeltas
r5apex.exe!0x02793350 ConVar sv_show_placement_help_in_preview
r5apex.exe!0x023332a0 ConVar sv_showfiredbullets
r5apex.exe!0x023718a0 ConVar sv_showhitboxes
r5apex.exe!0x0277adf0 ConVar sv_showlagcompensation
r5apex.exe!0x01818180 ConVar sv_single_core_dedi
r5apex.exe!0x01815330 ConVar sv_skipSendingUnnecessaryPersistence
r5apex.exe!0x023348b0 ConVar sv_skyname
r5apex.exe!0x01817820 ConVar sv_snapshot_uniform_interval
r5apex.exe!0x0278ca70 ConVar sv_spawnAIHintsInMP
r5apex.exe!0x0234d2c0 ConVar sv_specaccelerate
r5apex.exe!0x0234e420 ConVar sv_specnoclip
r5apex.exe!0x0234bec0 ConVar sv_specspeed
r5apex.exe!0x01814db0 ConVar sv_stats
r5apex.exe!0x0230db50 ConVar sv_stopspeed
r5apex.exe!0x01818400 ConVar sv_stressbots
r5apex.exe!0x01818720 ConVar sv_struggleCheck
r5apex.exe!0x01815b50 ConVar sv_struggleSpam
r5apex.exe!0x01818040 ConVar sv_struggleSpamInterval
r5apex.exe!0x01816d10 ConVar sv_tempents_send_from_delta
r5apex.exe!0x01814f90 ConVar sv_tempents_send_from_last_sent
r5apex.exe!0x01848df0 ConVar sv_testLargeDatablock
r5apex.exe!0x0276b640 ConVar sv_teststepsimulation
r5apex.exe!0x023598b0 ConVar sv_thinktimecheck
r5apex.exe!0x0278c520 ConVar sv_threaded_post_process_ai
r5apex.exe!0x0276bcb0 ConVar sv_threaded_post_process_players
r5apex.exe!0x02336a20 ConVar sv_threaded_pre_process_ents
r5apex.exe!0x01818360 ConVar sv_transmitToAllPlayersMask_allBitsSet
r5apex.exe!0x02788de0 ConVar sv_turbophysics
r5apex.exe!0x02788d40 ConVar sv_turbophysics_player
r5apex.exe!0x02785a70 ConVar sv_unlag
r5apex.exe!0x02783230 ConVar sv_unlag_debug
r5apex.exe!0x018170b0 ConVar sv_unnecessaryConnectDelay
r5apex.exe!0x01815c90 ConVar sv_unreliableSnapMaxSize
r5apex.exe!0x01815650 ConVar sv_updaterate_mp
r5apex.exe!0x01817aa0 ConVar sv_updaterate_sp
r5apex.exe!0x02367f80 ConVar sv_useRK4forprojectiles
r5apex.exe!0x01815dd0 ConVar sv_useReputation
r5apex.exe!0x01817be0 ConVar sv_useThreadsForSnapshots
r5apex.exe!0x023a7f80 ConVar sv_usercmd_before_entities
r5apex.exe!0x027884c0 ConVar sv_usercmd_fairness
r5apex.exe!0x027882e0 ConVar sv_usercmd_fairness_dediOnly
r5apex.exe!0x02394ee0 ConVar sv_usercmd_max_queued
r5apex.exe!0x02396810 ConVar sv_usercmd_num_per_iteration
r5apex.exe!0x023a8700 ConVar sv_usercmd_shuffle_players
r5apex.exe!0x02366620 ConVar sv_visualizetraces
r5apex.exe!0x023666c0 ConVar sv_visualizetraces_duration
r5apex.exe!0x018168d0 ConVar sv_voiceDebug
r5apex.exe!0x01818540 ConVar sv_voiceEcho
r5apex.exe!0x01817fa0 ConVar sv_voiceenable
r5apex.exe!0x01816190 ConVar sv_warnAboutCmdNumJumps
r5apex.exe!0x01714cd0 ConVar sv_watchdogTimer
r5apex.exe!0x0230a6a0 ConVar sv_wateraccelerate
r5apex.exe!0x02348100 ConVar sv_waterdist
r5apex.exe!0x0279e5f0 ConVar sv_weapon_despawn_time
r5apex.exe!0x01712af0 ConVar sv_writePersistenceOnShutdown
r5apex.exe!0x022f6a00 ConVar sys_attract_mode_timeout
r5apex.exe!0x01847b00 ConVar sys_minidumpexpandedspew
r5apex.exe!0x018481f0 ConVar sys_minidumpspewlines
r5apex.exe!0x01717740 ConVar system_alt_f4_closes_window
r5apex.exe!0x0236c7f0 ConVar teamSpot_costLimitPerFrame
r5apex.exe!0x0236f9a0 ConVar teamSpot_enabled
r5apex.exe!0x0236e390 ConVar teamSpot_lockOffTime
r5apex.exe!0x023697d0 ConVar teamSpot_lockOnTime
r5apex.exe!0x0236f150 ConVar teamSpot_lockOnTimeForgiveness
r5apex.exe!0x02369910 ConVar teamSpot_minimap_enabled
r5apex.exe!0x0236f0b0 ConVar teamSpot_threaded
r5apex.exe!0x0234f7d0 ConVar teams_unassigned_are_friendly
r5apex.exe!0x017135e0 ConVar telemetry_client_debug
r5apex.exe!0x0170bcd0 ConVar telemetry_client_enable
r5apex.exe!0x01710610 ConVar telemetry_client_sendInterval
r5apex.exe!0x02785cc0 ConVar template_debug
r5apex.exe!0x022e6410 ConVar test_fakeTimeDays
r5apex.exe!0x02351820 ConVar test_fakeTimeDays
r5apex.exe!0x02782f80 ConVar test_massive_dmg
r5apex.exe!0x02783990 ConVar test_massive_dmg_clip
r5apex.exe!0x02335db0 ConVar tether_damageScale
r5apex.exe!0x0230aaf0 ConVar tether_dodge_damage
r5apex.exe!0x0230a880 ConVar tether_healthDrain
r5apex.exe!0x0230dc90 ConVar tether_healthDrainNPC
r5apex.exe!0x0234f0f0 ConVar tether_maxvel
r5apex.exe!0x0278d620 ConVar tether_npc_strength
r5apex.exe!0x0234ffb0 ConVar tether_radius
r5apex.exe!0x0234d770 ConVar tether_strength
r5apex.exe!0x02368a50 ConVar think_limit
r5apex.exe!0x02299690 ConVar thirdperson_mayamode
r5apex.exe!0x0228e820 ConVar thirdperson_override
r5apex.exe!0x022a2100 ConVar thirdperson_screenspace
r5apex.exe!0x02779ce0 ConVar threat_detection_in_job
r5apex.exe!0x017076f0 ConVar timeout
r5apex.exe!0x01707790 ConVar timeout_during_load
r5apex.exe!0x0236eb30 ConVar titanSoul_debug
r5apex.exe!0x0276b6e0 ConVar titan_hideEnts
r5apex.exe!0x02786820 ConVar titan_hidePlayer
r5apex.exe!0x022f08e0 ConVar titan_sprint_sound
r5apex.exe!0x027a4c70 ConVar titan_step_damage_can_push_down
r5apex.exe!0x027a4bd0 ConVar titan_step_damage_debug
r5apex.exe!0x027a4d10 ConVar titan_step_damage_rodeo_immunity_time
r5apex.exe!0x01818fe0 ConVar tracehull_height_error_check
r5apex.exe!0x0229c960 ConVar tracer_debug
r5apex.exe!0x02293d80 ConVar tracer_extra
r5apex.exe!0x022fe130 ConVar trail_optimizedRemove
r5apex.exe!0x0234c570 ConVar traversal_anim
r5apex.exe!0x022b3a90 ConVar traversal_cooldown
r5apex.exe!0x0230c710 ConVar traversal_cooldown
r5apex.exe!0x022e1c00 ConVar traversal_enable
r5apex.exe!0x02333ad0 ConVar traversal_enable
r5apex.exe!0x022b4f20 ConVar traversal_hand_debug
r5apex.exe!0x0230e050 ConVar traversal_hand_debug
r5apex.exe!0x022e1f00 ConVar traversal_hand_required_width
r5apex.exe!0x02333d50 ConVar traversal_hand_required_width
r5apex.exe!0x0279dea0 ConVar traversal_viewLerpInDuration
r5apex.exe!0x023012d0 ConVar traversal_viewLerpOut
r5apex.exe!0x02302f70 ConVar traversal_viewLerpOutAngle
r5apex.exe!0x022fb030 ConVar traversal_viewLerpOutDebug
r5apex.exe!0x02303570 ConVar traversal_viewLerpOutPos
r5apex.exe!0x022b48c0 ConVar traversal_window_duration
r5apex.exe!0x0230d5d0 ConVar traversal_window_duration
r5apex.exe!0x022b3b30 ConVar traversal_window_enable
r5apex.exe!0x0230c7b0 ConVar traversal_window_enable
r5apex.exe!0x022b4de0 ConVar traversal_window_finish_angle
r5apex.exe!0x0230de70 ConVar traversal_window_finish_angle
r5apex.exe!0x022e12d0 ConVar traversal_window_forward_offset
r5apex.exe!0x02333200 ConVar traversal_window_forward_offset
r5apex.exe!0x022e4cd0 ConVar traversal_window_hand_vertical_offset
r5apex.exe!0x02347a20 ConVar traversal_window_hand_vertical_offset
r5apex.exe!0x022e2ff0 ConVar traversal_window_sideways_offset
r5apex.exe!0x02335760 ConVar traversal_window_sideways_offset
r5apex.exe!0x01ef1870 ConVar traversal_window_view_pitch_max
r5apex.exe!0x01eecb00 ConVar traversal_window_view_pitch_min
r5apex.exe!0x01f01160 ConVar traversal_window_yaw_max
r5apex.exe!0x022e3b90 ConVar trigger_ignore_nonsolids
r5apex.exe!0x02336660 ConVar trigger_ignore_nonsolids
r5apex.exe!0x027795f0 ConVar trigger_touch_on_spawn
r5apex.exe!0x02772680 ConVar trigger_use_new_filters
r5apex.exe!0x01ed3e00 ConVar tsaa_blendfactorincreaseatmaxvelocity
r5apex.exe!0x01ed3ea0 ConVar tsaa_blendfactorincreasewhenunoccluded
r5apex.exe!0x01ed3fe0 ConVar tsaa_blendfactormaxesoutatvelocity
r5apex.exe!0x01ed41c0 ConVar tsaa_blendfactormodulationonsparklesandunocclusion
r5apex.exe!0x01ed4080 ConVar tsaa_blendfactoroverride
r5apex.exe!0x01ed3f40 ConVar tsaa_curframeblendamount
r5apex.exe!0x01ed4120 ConVar tsaa_debugresponsiveflag
r5apex.exe!0x01ed3cc0 ConVar tsaa_neighborhoodclamping
r5apex.exe!0x01ed3d60 ConVar tsaa_neighborhoodclampingsoftened
r5apex.exe!0x022ae620 ConVar tsaa_numsamples
r5apex.exe!0x022a3090 ConVar tweak_light_shadows_every_frame
r5apex.exe!0x017dbca0 ConVar twitch_check_interval
r5apex.exe!0x017dcf00 ConVar twitch_prime_rewards
r5apex.exe!0x017dc260 ConVar twitch_shouldQuery
r5apex.exe!0x022f67e0 ConVar ui_fadecloud_time
r5apex.exe!0x022f6740 ConVar ui_fadexui_time
r5apex.exe!0x022f59a0 ConVar ui_gameui_ctrlr_title
r5apex.exe!0x022f6120 ConVar ui_gameui_modal
r5apex.exe!0x022f5c20 ConVar ui_loadingscreen_autotransition_time
r5apex.exe!0x022f63a0 ConVar ui_loadingscreen_fadein_time
r5apex.exe!0x01706280 ConVar ui_loadingscreen_fadeout_time
r5apex.exe!0x022f6440 ConVar ui_loadingscreen_fadeout_time
r5apex.exe!0x022f6d20 ConVar ui_loadingscreen_mintransition_time
r5apex.exe!0x022f5f40 ConVar ui_loadingscreen_transition_time
r5apex.exe!0x022f5b80 ConVar ui_lobby_jointimeout
r5apex.exe!0x022f6c80 ConVar ui_lobby_noautostart
r5apex.exe!0x022f66a0 ConVar ui_lobby_noresults_create_msg_time
r5apex.exe!0x022f39b0 ConVar ui_posedebug_fade_in_time
r5apex.exe!0x022f3910 ConVar ui_posedebug_fade_out_time
r5apex.exe!0x022f5ae0 ConVar ui_virtualnav_render
r5apex.exe!0x022b2670 ConVar unique_entity_names
r5apex.exe!0x0230c670 ConVar unique_entity_names
r5apex.exe!0x022f8380 ConVar usePromptBaseColor
r5apex.exe!0x022f9fb0 ConVar usePromptButtonTextColor
r5apex.exe!0x022f8100 ConVar usePromptImageScale
r5apex.exe!0x022f9b20 ConVar usePromptImageYOffset
r5apex.exe!0x022faaf0 ConVar usePromptTextColor
r5apex.exe!0x022acdb0 ConVar use_monitors
r5apex.exe!0x017197f0 ConVar use_valve_auto_gain
r5apex.exe!0x018445f0 ConVar user_tracking_enabled
r5apex.exe!0x0170ab60 ConVar users_hostname
r5apex.exe!0x01ede120 ConVar v_centermove
r5apex.exe!0x01eecba0 ConVar v_centerspeed
r5apex.exe!0x022ec750 ConVar variable_sights_gravity_scale_override
r5apex.exe!0x01ed7ff0 ConVar vgui_EnableFixedAspectScaling
r5apex.exe!0x022f64e0 ConVar vgui_drawPolyShapes
r5apex.exe!0x01715c00 ConVar vgui_drawfocus
r5apex.exe!0x01ed7c40 ConVar vgui_drawfocus
r5apex.exe!0x01717a60 ConVar vgui_drawkeyfocus
r5apex.exe!0x01716de0 ConVar vgui_drawtree
r5apex.exe!0x01718540 ConVar vgui_drawtree_bounds
r5apex.exe!0x01719430 ConVar vgui_drawtree_draw_selected
r5apex.exe!0x01717880 ConVar vgui_drawtree_freeze
r5apex.exe!0x01715010 ConVar vgui_drawtree_hidden
r5apex.exe!0x01717ec0 ConVar vgui_drawtree_panelalpha
r5apex.exe!0x01716340 ConVar vgui_drawtree_panelptr
r5apex.exe!0x01716040 ConVar vgui_drawtree_popupsonly
r5apex.exe!0x017150b0 ConVar vgui_drawtree_render_order
r5apex.exe!0x01718ae0 ConVar vgui_drawtree_scheme
r5apex.exe!0x01717380 ConVar vgui_drawtree_visible
r5apex.exe!0x01ede580 ConVar vgui_interactive
r5apex.exe!0x01ed9a40 ConVar vgui_noquads
r5apex.exe!0x01ed9900 ConVar vgui_notext
r5apex.exe!0x01ed9680 ConVar vgui_paintEnabled
r5apex.exe!0x01ed7ce0 ConVar vgui_resize_on_resolution_change
r5apex.exe!0x01ed85f0 ConVar vgui_show_glyph_miss
r5apex.exe!0x017125a0 ConVar vgui_simulate_during_bone_setup
r5apex.exe!0x022f9030 ConVar video_menu_uiscript_reset
r5apex.exe!0x02367e40 ConVar viewDrift
r5apex.exe!0x02367350 ConVar viewDrift_ads_delay_debounce_time
r5apex.exe!0x02368550 ConVar viewDrift_pitch_base1_amp
r5apex.exe!0x023677a0 ConVar viewDrift_pitch_base1_freq
r5apex.exe!0x02367660 ConVar viewDrift_pitch_base1_phase
r5apex.exe!0x02367030 ConVar viewDrift_pitch_base2_amp
r5apex.exe!0x02368870 ConVar viewDrift_pitch_base2_freq
r5apex.exe!0x02367da0 ConVar viewDrift_pitch_base2_phase
r5apex.exe!0x02367980 ConVar viewDrift_pitch_scaler_amp
r5apex.exe!0x023689b0 ConVar viewDrift_pitch_scaler_base
r5apex.exe!0x023678e0 ConVar viewDrift_pitch_scaler_freq
r5apex.exe!0x02367840 ConVar viewDrift_pitch_scaler_phase
r5apex.exe!0x023684b0 ConVar viewDrift_pitch_shifter_amp
r5apex.exe!0x02367210 ConVar viewDrift_pitch_shifter_freq
r5apex.exe!0x02368730 ConVar viewDrift_pitch_shifter_phase
r5apex.exe!0x02368910 ConVar viewDrift_yaw_base1_amp
r5apex.exe!0x02366b20 ConVar viewDrift_yaw_base1_freq
r5apex.exe!0x02366c20 ConVar viewDrift_yaw_base1_phase
r5apex.exe!0x02367ee0 ConVar viewDrift_yaw_base2_amp
r5apex.exe!0x023682c0 ConVar viewDrift_yaw_base2_freq
r5apex.exe!0x02366a80 ConVar viewDrift_yaw_base2_phase
r5apex.exe!0x02367b30 ConVar viewDrift_yaw_scaler_amp
r5apex.exe!0x02366f90 ConVar viewDrift_yaw_scaler_base
r5apex.exe!0x023687d0 ConVar viewDrift_yaw_scaler_freq
r5apex.exe!0x023670d0 ConVar viewDrift_yaw_scaler_phase
r5apex.exe!0x02366800 ConVar viewDrift_yaw_shifter_amp
r5apex.exe!0x02368220 ConVar viewDrift_yaw_shifter_freq
r5apex.exe!0x023685f0 ConVar viewDrift_yaw_shifter_phase
r5apex.exe!0x02368b90 ConVar view_offset_entity_enable
r5apex.exe!0x022a57c0 ConVar viewangle_debug
r5apex.exe!0x01eedae0 ConVar viewangles_simpler
r5apex.exe!0x01edea80 ConVar viewmodelShake
r5apex.exe!0x01efd4f0 ConVar viewmodelShake_sourceRollRange
r5apex.exe!0x022ad910 ConVar viewmodel_bounds_draw
r5apex.exe!0x022a8020 ConVar viewmodel_bounds_draw_lock
r5apex.exe!0x018520b0 ConVar viewmodel_selfshadow
r5apex.exe!0x022ad070 ConVar viewmodel_selfshadow_debug_2d
r5apex.exe!0x022a6cd0 ConVar viewmodel_selfshadow_tightbounds
r5apex.exe!0x01ee9d20 ConVar viewportscale
r5apex.exe!0x0234ead0 ConVar viewpunch_base_springConstantX
r5apex.exe!0x0234d430 ConVar viewpunch_base_springConstantY
r5apex.exe!0x0234fc70 ConVar viewpunch_base_springConstantZ
r5apex.exe!0x0234db70 ConVar viewpunch_base_springDampingX
r5apex.exe!0x0234c9a0 ConVar viewpunch_base_springDampingY
r5apex.exe!0x0234e070 ConVar viewpunch_base_springDampingZ
r5apex.exe!0x0277c420 ConVar viewpunch_predictable_scalar
r5apex.exe!0x01709e70 ConVar violence_ablood
r5apex.exe!0x022e95e0 ConVar violence_ablood
r5apex.exe!0x02355fb0 ConVar violence_ablood
r5apex.exe!0x0170c800 ConVar violence_agibs
r5apex.exe!0x022eb530 ConVar violence_agibs
r5apex.exe!0x02358500 ConVar violence_agibs
r5apex.exe!0x01713ae0 ConVar violence_hblood
r5apex.exe!0x022f02c0 ConVar violence_hblood
r5apex.exe!0x0235d790 ConVar violence_hblood
r5apex.exe!0x0170e460 ConVar violence_hgibs
r5apex.exe!0x022ed770 ConVar violence_hgibs
r5apex.exe!0x0235ac30 ConVar violence_hgibs
r5apex.exe!0x022f0360 ConVar visible_ent_cone_debug_duration_client
r5apex.exe!0x02358460 ConVar visible_ent_cone_debug_duration_server
r5apex.exe!0x0171ab10 ConVar voice_absTriggerAmount
r5apex.exe!0x022faef0 ConVar voice_allow_mute_self
r5apex.exe!0x01719c50 ConVar voice_avggain
r5apex.exe!0x022f2720 ConVar voice_clientdebug
r5apex.exe!0x0171a010 ConVar voice_debugAddSecondTalker
r5apex.exe!0x0171a6d0 ConVar voice_debugThresholds
r5apex.exe!0x01715ca0 ConVar voice_debugfeedback
r5apex.exe!0x022f9870 ConVar voice_decimate_at_bytes
r5apex.exe!0x022fadd0 ConVar voice_decimate_rate
r5apex.exe!0x0171a450 ConVar voice_enabled
r5apex.exe!0x0171a150 ConVar voice_energyPerZeroThreshold
r5apex.exe!0x0171a630 ConVar voice_energyThreshold
r5apex.exe!0x0171a4f0 ConVar voice_forcemicrecord
r5apex.exe!0x0170b5e0 ConVar voice_inputfromfile
r5apex.exe!0x01ede080 ConVar voice_late_update
r5apex.exe!0x0171a8b0 ConVar voice_loopback
r5apex.exe!0x0171aa70 ConVar voice_maxgain
r5apex.exe!0x0171a290 ConVar voice_minEnergyPerZeroThreshold
r5apex.exe!0x0171abb0 ConVar voice_mixer_boost
r5apex.exe!0x0171ac50 ConVar voice_mixer_mute
r5apex.exe!0x0171acf0 ConVar voice_mixer_volume
r5apex.exe!0x022f2d70 ConVar voice_modenable
r5apex.exe!0x018185e0 ConVar voice_noxplat
r5apex.exe!0x0171a9d0 ConVar voice_profile
r5apex.exe!0x0170d820 ConVar voice_recordtofile
r5apex.exe!0x01719d90 ConVar voice_scale
r5apex.exe!0x02368e50 ConVar voice_serverdebug
r5apex.exe!0x0171a3b0 ConVar voice_showchannels
r5apex.exe!0x01719a70 ConVar voice_showincoming
r5apex.exe!0x01719b10 ConVar voice_threshold_delay
r5apex.exe!0x0171a810 ConVar voice_triggerCrossingRate
r5apex.exe!0x01719f70 ConVar voice_triggerRate
r5apex.exe!0x0171a590 ConVar voice_vox
r5apex.exe!0x01719930 ConVar voice_writevoices
r5apex.exe!0x0170b060 ConVar voice_xsend_debug
r5apex.exe!0x01719e30 ConVar voice_zeroCrossingThreshold
r5apex.exe!0x02795770 ConVar vortex_damageimpulsescale
r5apex.exe!0x02396cf0 ConVar vprof_scope_entity_gamephys
r5apex.exe!0x023e8f90 ConVar vprof_scope_entity_thinks
r5apex.exe!0x01713e70 ConVar vprof_server_spike_threshold
r5apex.exe!0x0170cef0 ConVar vprof_server_thread
r5apex.exe!0x023974f0 ConVar vprof_think_limit
r5apex.exe!0x022ae8c0 ConVar vscript_ui_do_delay_init
r5apex.exe!0x01efc280 ConVar vsm_culling
r5apex.exe!0x02283fb0 ConVar vsm_ignore_edge_planes
r5apex.exe!0x01edf170 ConVar vsm_ignore_face_planes
r5apex.exe!0x017157c0 ConVar vx_do_not_throttle_events
r5apex.exe!0x022ec6b0 ConVar wall_climb_pose_paramteter_hands_enabled
r5apex.exe!0x02359770 ConVar wall_climb_pose_paramteter_hands_enabled
r5apex.exe!0x022e3a50 ConVar wallclimb_vertical_gain_reduction
r5apex.exe!0x02336480 ConVar wallclimb_vertical_gain_reduction
r5apex.exe!0x022e5130 ConVar wallrun_angleChangeMinCos
r5apex.exe!0x023482c0 ConVar wallrun_angleChangeMinCos
r5apex.exe!0x022e4f50 ConVar wallrun_avoid_wall_top_decel
r5apex.exe!0x02347de0 ConVar wallrun_avoid_wall_top_decel
r5apex.exe!0x0279de00 ConVar wallrun_curveDebug
r5apex.exe!0x0279dc20 ConVar wallrun_curveEnable
r5apex.exe!0x0234e4f0 ConVar wallrun_debug
r5apex.exe!0x0234e380 ConVar wallrun_enable
r5apex.exe!0x022e2f50 ConVar wallrun_fallAwaySpeed
r5apex.exe!0x023356c0 ConVar wallrun_fallAwaySpeed
r5apex.exe!0x022e2d00 ConVar wallrun_hangStopTime
r5apex.exe!0x023353a0 ConVar wallrun_hangStopTime
r5apex.exe!0x022b3bd0 ConVar wallrun_hangslipduration
r5apex.exe!0x0230c850 ConVar wallrun_hangslipduration
r5apex.exe!0x022e1ca0 ConVar wallrun_hangslipstarttime
r5apex.exe!0x02333b70 ConVar wallrun_hangslipstarttime
r5apex.exe!0x022e0c10 ConVar wallrun_hangslipvel
r5apex.exe!0x02332b10 ConVar wallrun_hangslipvel
r5apex.exe!0x0234a0c0 ConVar wallrun_maxViewTilt
r5apex.exe!0x022ce570 ConVar wallrun_minAngle_air
r5apex.exe!0x02327230 ConVar wallrun_minAngle_air
r5apex.exe!0x022e2e40 ConVar wallrun_noInputSlipFrac
r5apex.exe!0x02335580 ConVar wallrun_noInputSlipFrac
r5apex.exe!0x0234bae0 ConVar wallrun_pushAwayFallOffTime
r5apex.exe!0x022de970 ConVar wallrun_repelEnable
r5apex.exe!0x0232fbc0 ConVar wallrun_repelEnable
r5apex.exe!0x022e1450 ConVar wallrun_repelSoftness
r5apex.exe!0x02333340 ConVar wallrun_repelSoftness
r5apex.exe!0x022e4a50 ConVar wallrun_repelTimeMax
r5apex.exe!0x02347840 ConVar wallrun_repelTimeMax
r5apex.exe!0x022e1d40 ConVar wallrun_repelTimeMin
r5apex.exe!0x02333c10 ConVar wallrun_repelTimeMin
r5apex.exe!0x022dfee0 ConVar wallrun_retry_interval
r5apex.exe!0x023317e0 ConVar wallrun_retry_interval
r5apex.exe!0x0234ef20 ConVar wallrun_rotateMaxRate
r5apex.exe!0x0234cff0 ConVar wallrun_sameWallDist
r5apex.exe!0x0234ed80 ConVar wallrun_sameWallDot
r5apex.exe!0x0234ca40 ConVar wallrun_sameWallSlope
r5apex.exe!0x022e49b0 ConVar wallrun_slipduration
r5apex.exe!0x023476d0 ConVar wallrun_slipduration
r5apex.exe!0x022e0070 ConVar wallrun_slipslowdown
r5apex.exe!0x02331f10 ConVar wallrun_slipslowdown
r5apex.exe!0x022e2ae0 ConVar wallrun_slipstarttime
r5apex.exe!0x02335180 ConVar wallrun_slipstarttime
r5apex.exe!0x022e3c30 ConVar wallrun_slipvel
r5apex.exe!0x02336700 ConVar wallrun_slipvel
r5apex.exe!0x022b4d40 ConVar wallrun_strengthLossEnd
r5apex.exe!0x0230ddd0 ConVar wallrun_strengthLossEnd
r5apex.exe!0x022b4a00 ConVar wallrun_strengthLossStart
r5apex.exe!0x0230d710 ConVar wallrun_strengthLossStart
r5apex.exe!0x022e1fa0 ConVar wallrun_upwardAutoPush
r5apex.exe!0x02333df0 ConVar wallrun_upwardAutoPush
r5apex.exe!0x022e3cd0 ConVar wallrun_viewTiltPredictTime
r5apex.exe!0x023368e0 ConVar wallrun_viewTiltPredictTime
r5apex.exe!0x0234a600 ConVar wallrun_viewTiltSpeed
r5apex.exe!0x0234bd80 ConVar was_loaded
r5apex.exe!0x02787530 ConVar weaponAmmoPickupSound
r5apex.exe!0x023042d0 ConVar weaponFastHolsterScale
r5apex.exe!0x0279e130 ConVar weaponFastHolsterScale
r5apex.exe!0x022e4e10 ConVar weaponSwitch3p_checkNewWeapon
r5apex.exe!0x02347b60 ConVar weaponSwitch3p_checkNewWeapon
r5apex.exe!0x02307230 ConVar weaponSwitch3p_onHolster
r5apex.exe!0x027a0af0 ConVar weaponSwitch3p_onHolster
r5apex.exe!0x0235a4f0 ConVar weapon_auto_swap_ordnance_no_ammo
r5apex.exe!0x02306f10 ConVar weapon_debugScript
r5apex.exe!0x027a0920 ConVar weapon_debugScript
r5apex.exe!0x02306990 ConVar weapon_doIdleForSurvivalMelee
r5apex.exe!0x027a02d0 ConVar weapon_doIdleForSurvivalMelee
r5apex.exe!0x01f01460 ConVar weapon_friendly_fire_prevent_ui
r5apex.exe!0x02367700 ConVar weapon_meleeButtonPressProtection
r5apex.exe!0x022844b0 ConVar weapon_parentingFixLerp
r5apex.exe!0x02367bd0 ConVar weapon_pickup_allow_dupes
r5apex.exe!0x01eded00 ConVar weapon_poseParamMaxDistance
r5apex.exe!0x02307130 ConVar weapon_render_with_fastpath
r5apex.exe!0x0279e370 ConVar weapon_showproficiency
r5apex.exe!0x027a1b30 ConVar weapon_sprint_raise_delay
r5apex.exe!0x02307850 ConVar weaponx_predicting_client_only_optimization
r5apex.exe!0x027a1140 ConVar weaponx_predicting_client_only_optimization
r5apex.exe!0x023078f0 ConVar weaponx_smartammo_data_optimization
r5apex.exe!0x027a11e0 ConVar weaponx_smartammo_data_optimization
r5apex.exe!0x0279df40 ConVar window_hint_debug
r5apex.exe!0x0234e860 ConVar window_hint_fov_down
r5apex.exe!0x0234ece0 ConVar window_hint_fov_horz
r5apex.exe!0x0234ba40 ConVar window_hint_fov_up
r5apex.exe!0x0234f730 ConVar window_hint_keyboard_fov_horz
r5apex.exe!0x0234c710 ConVar window_hint_lookahead_time
r5apex.exe!0x02350050 ConVar window_hint_max_horz_vel_change_dot
r5apex.exe!0x0234f050 ConVar window_hint_max_vel_change_down
r5apex.exe!0x0234cdb0 ConVar window_hint_max_vel_change_up
r5apex.exe!0x0234c670 ConVar window_hint_min_horz_vel
r5apex.exe!0x02350120 ConVar window_hint_permissive_max_horz_vel_change_dot
r5apex.exe!0x0234d870 ConVar window_hint_permissive_max_vel_change_down
r5apex.exe!0x0234daa0 ConVar window_hint_permissive_max_vel_change_up
r5apex.exe!0x027866e0 ConVar xc_crouch_debounce
r5apex.exe!0x01efd290 ConVar z_ragdoll_impact_strength
r5apex.exe!0x022fcc90 ConVar zipline_fade_dist
r5apex.exe!0x02296cb0 ConVar zipline_subdiv_lod_dist_base
r5apex.exe!0x022a4170 ConVar zipline_subdiv_slices
r5apex.exe!0x022a19a0 ConVar zipline_subdiv_slices_lod
r5apex.exe!0x0229b7f0 ConVar zipline_subdiv_stacks
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
r5apex.exe!0x0228fa80 ConCommand +ability
r5apex.exe!0x02299350 ConCommand +ability_held
r5apex.exe!0x0229dd40 ConCommand +attack
r5apex.exe!0x022a1da0 ConCommand +backward
r5apex.exe!0x0229a820 ConCommand +break
r5apex.exe!0x0229df10 ConCommand +camdistance
r5apex.exe!0x022993d0 ConCommand +camin
r5apex.exe!0x02296240 ConCommand +cammousemove
r5apex.exe!0x02296720 ConCommand +camout
r5apex.exe!0x022a1d20 ConCommand +campitchdown
r5apex.exe!0x0228f7a0 ConCommand +campitchup
r5apex.exe!0x0229a8a0 ConCommand +camyawleft
r5apex.exe!0x022984a0 ConCommand +camyawright
r5apex.exe!0x0229c8e0 ConCommand +commandermousemove
r5apex.exe!0x01ede800 ConCommand +csm_rot_x_neg
r5apex.exe!0x01ef5540 ConCommand +csm_rot_x_plus
r5apex.exe!0x01eef6c0 ConCommand +csm_rot_y_neg
r5apex.exe!0x01ef4e00 ConCommand +csm_rot_y_plus
r5apex.exe!0x022a54b0 ConCommand +displayFullscreenMap
r5apex.exe!0x022978c0 ConCommand +dodge
r5apex.exe!0x022a2920 ConCommand +duck
r5apex.exe!0x0228fcc0 ConCommand +forward
r5apex.exe!0x0229f230 ConCommand +graph
r5apex.exe!0x022a3610 ConCommand +jump
r5apex.exe!0x0229cf60 ConCommand +klook
r5apex.exe!0x022a14a0 ConCommand +left
r5apex.exe!0x0229f3b0 ConCommand +lookdown
r5apex.exe!0x0229fed0 ConCommand +lookup
r5apex.exe!0x01812ee0 ConCommand +mat_texture_list
r5apex.exe!0x022a2ef0 ConCommand +melee
r5apex.exe!0x0229f1b0 ConCommand +movedown
r5apex.exe!0x02299110 ConCommand +moveleft
r5apex.exe!0x02290380 ConCommand +moveright
r5apex.exe!0x02297ae0 ConCommand +moveup
r5apex.exe!0x022a3e30 ConCommand +offhand0
r5apex.exe!0x02294420 ConCommand +offhand1
r5apex.exe!0x022a23c0 ConCommand +offhand2
r5apex.exe!0x0229dc40 ConCommand +offhand3
r5apex.exe!0x022a1e20 ConCommand +offhand4
r5apex.exe!0x022a5530 ConCommand +pause_menu
r5apex.exe!0x0229b770 ConCommand +ping
r5apex.exe!0x022f42e0 ConCommand +posedebug
r5apex.exe!0x0171a950 ConCommand +pushtotalk
r5apex.exe!0x022965a0 ConCommand +reload
r5apex.exe!0x02295aa0 ConCommand +right
r5apex.exe!0x022a46b0 ConCommand +score
r5apex.exe!0x022a06b0 ConCommand +scriptCommand1
r5apex.exe!0x022a2f70 ConCommand +scriptCommand2
r5apex.exe!0x022959a0 ConCommand +scriptCommand3
r5apex.exe!0x02298280 ConCommand +scriptCommand4
r5apex.exe!0x0229fc10 ConCommand +scriptCommand5
r5apex.exe!0x0229ff50 ConCommand +scriptCommand6
r5apex.exe!0x022a3850 ConCommand +scriptCommand7
r5apex.exe!0x02296000 ConCommand +scriptCommand8
r5apex.exe!0x0229f9d0 ConCommand +scriptCommand9
r5apex.exe!0x02290620 ConCommand +showscores
r5apex.exe!0x0228f360 ConCommand +speed
r5apex.exe!0x022a5440 ConCommand +strafe
r5apex.exe!0x022963e0 ConCommand +toggle_duck
r5apex.exe!0x02290480 ConCommand +toggle_zoom
r5apex.exe!0x022a1b60 ConCommand +use
r5apex.exe!0x0229fd30 ConCommand +useAndReload
r5apex.exe!0x02298870 ConCommand +use_alt
r5apex.exe!0x022a29a0 ConCommand +use_long
r5apex.exe!0x02296960 ConCommand +variableScopeToggle
r5apex.exe!0x01718860 ConCommand +vgui_drawtree
r5apex.exe!0x0170bf20 ConCommand +voicerecord
r5apex.exe!0x022a4db0 ConCommand +walk
r5apex.exe!0x022a43f0 ConCommand +weaponCycle
r5apex.exe!0x022a0070 ConCommand +weapon_discard
r5apex.exe!0x02296620 ConCommand +zoom
r5apex.exe!0x0229ca00 ConCommand -ability
r5apex.exe!0x02297190 ConCommand -ability_held
r5apex.exe!0x02299850 ConCommand -attack
r5apex.exe!0x0228f5e0 ConCommand -backward
r5apex.exe!0x0229b890 ConCommand -break
r5apex.exe!0x022a4c10 ConCommand -camdistance
r5apex.exe!0x022a0cb0 ConCommand -camin
r5apex.exe!0x02296080 ConCommand -cammousemove
r5apex.exe!0x0229b440 ConCommand -camout
r5apex.exe!0x0229f6f0 ConCommand -campitchdown
r5apex.exe!0x0229a9c0 ConCommand -campitchup
r5apex.exe!0x02299230 ConCommand -camyawleft
r5apex.exe!0x02297a60 ConCommand -camyawright
r5apex.exe!0x022a2ad0 ConCommand -commandermousemove
r5apex.exe!0x01ef6d20 ConCommand -csm_rot_x_neg
r5apex.exe!0x01edef90 ConCommand -csm_rot_x_plus
r5apex.exe!0x02282eb0 ConCommand -csm_rot_y_neg
r5apex.exe!0x01ef4300 ConCommand -csm_rot_y_plus
r5apex.exe!0x0228fba0 ConCommand -displayFullscreenMap
r5apex.exe!0x022a40f0 ConCommand -dodge
r5apex.exe!0x0229f330 ConCommand -duck
r5apex.exe!0x02298380 ConCommand -forward
r5apex.exe!0x022985c0 ConCommand -graph
r5apex.exe!0x02298b30 ConCommand -jump
r5apex.exe!0x02297240 ConCommand -klook
r5apex.exe!0x02290400 ConCommand -left
r5apex.exe!0x02297ca0 ConCommand -lookdown
r5apex.exe!0x0228f960 ConCommand -lookup
r5apex.exe!0x01812c80 ConCommand -mat_texture_list
r5apex.exe!0x0229c2e0 ConCommand -melee
r5apex.exe!0x022962c0 ConCommand -movedown
r5apex.exe!0x022a16b0 ConCommand -moveleft
r5apex.exe!0x02298bb0 ConCommand -moveright
r5apex.exe!0x0229f430 ConCommand -moveup
r5apex.exe!0x0229fe50 ConCommand -offhand0
r5apex.exe!0x022a1fe0 ConCommand -offhand1
r5apex.exe!0x022a0ef0 ConCommand -offhand2
r5apex.exe!0x022972c0 ConCommand -offhand3
r5apex.exe!0x022900c0 ConCommand -offhand4
r5apex.exe!0x022a17b0 ConCommand -pause_menu
r5apex.exe!0x02290500 ConCommand -ping
r5apex.exe!0x022f4360 ConCommand -posedebug
r5apex.exe!0x0171a330 ConCommand -pushtotalk
r5apex.exe!0x022901e0 ConCommand -reload
r5apex.exe!0x0229f2b0 ConCommand -right
r5apex.exe!0x022a21a0 ConCommand -score
r5apex.exe!0x02290300 ConCommand -scriptCommand1
r5apex.exe!0x0229f810 ConCommand -scriptCommand2
r5apex.exe!0x022a3db0 ConCommand -scriptCommand3
r5apex.exe!0x022a3730 ConCommand -scriptCommand4
r5apex.exe!0x0228f3e0 ConCommand -scriptCommand5
r5apex.exe!0x0228f1c0 ConCommand -scriptCommand6
r5apex.exe!0x022a4070 ConCommand -scriptCommand7
r5apex.exe!0x02295660 ConCommand -scriptCommand8
r5apex.exe!0x0229ef70 ConCommand -scriptCommand9
r5apex.exe!0x02295920 ConCommand -showscores
r5apex.exe!0x02298ab0 ConCommand -speed
r5apex.exe!0x022a2340 ConCommand -strafe
r5apex.exe!0x02298300 ConCommand -toggle_duck
r5apex.exe!0x0229fb90 ConCommand -toggle_zoom
r5apex.exe!0x022956e0 ConCommand -use
r5apex.exe!0x0228f2e0 ConCommand -useAndReload
r5apex.exe!0x022a1ae0 ConCommand -use_alt
r5apex.exe!0x0229a5b0 ConCommand -use_long
r5apex.exe!0x02290040 ConCommand -variableScopeToggle
r5apex.exe!0x01718280 ConCommand -vgui_drawtree
r5apex.exe!0x01707f40 ConCommand -voicerecord
r5apex.exe!0x022a4470 ConCommand -walk
r5apex.exe!0x02297940 ConCommand -weaponCycle
r5apex.exe!0x022a4ed0 ConCommand -weapon_discard
r5apex.exe!0x0229b6a0 ConCommand -zoom
r5apex.exe!0x017047c0 ConCommand BindToggle
r5apex.exe!0x0278c960 ConCommand BuildAINFile
r5apex.exe!0x01954a30 ConCommand DebugPrintUsedTextures
r5apex.exe!0x018139b0 ConCommand DumpClientDataBlockReceiver
r5apex.exe!0x017dcac0 ConCommand MemTrackDeltaSnapshot
r5apex.exe!0x017dc560 ConCommand MemTrackPrintStats
r5apex.exe!0x01efde40 ConCommand ReloadAimAssistSettings
r5apex.exe!0x027770d0 ConCommand Test_InitRandomEntitySpawner
r5apex.exe!0x02783100 ConCommand Test_RandomizeInPVS
r5apex.exe!0x0277d560 ConCommand Test_RemoveAllRandomEntities
r5apex.exe!0x02785f30 ConCommand Test_SpawnRandomEntities
r5apex.exe!0x02358620 ConCommand _setClassVarServer
r5apex.exe!0x017dc000 ConCommand adminmsg
r5apex.exe!0x0278d470 ConCommand ai_debug_node_connect
r5apex.exe!0x0278bb40 ConCommand ai_dump_hints
r5apex.exe!0x0278c850 ConCommand ai_set_move_height_epsilon
r5apex.exe!0x023973c0 ConCommand air_density
r5apex.exe!0x027964b0 ConCommand aisettings_reparse
r5apex.exe!0x022fafb0 ConCommand aisettings_reparse_client
r5apex.exe!0x01707d00 ConCommand alias
r5apex.exe!0x022f9c60 ConCommand applyVideoChangesDeferred
r5apex.exe!0x0170f8d0 ConCommand bind
r5apex.exe!0x0170c570 ConCommand bind_US_standard
r5apex.exe!0x01711710 ConCommand bind_held
r5apex.exe!0x01710d70 ConCommand bind_held_US_standard
r5apex.exe!0x0170cb00 ConCommand bind_list
r5apex.exe!0x0170b460 ConCommand bind_list_abilities
r5apex.exe!0x0171b1f0 ConCommand bink_dump_precached_movies
r5apex.exe!0x022e64b0 ConCommand bot_loadout
r5apex.exe!0x02357c90 ConCommand bot_loadout_server
r5apex.exe!0x017dc5e0 ConCommand box
r5apex.exe!0x01705570 ConCommand buildcubemaps
r5apex.exe!0x0170e320 ConCommand cache_print
r5apex.exe!0x0170d960 ConCommand cache_print_lru
r5apex.exe!0x01708420 ConCommand cache_print_summary
r5apex.exe!0x0229d790 ConCommand cam_command
r5apex.exe!0x02295800 ConCommand cancelselect
r5apex.exe!0x023e94d0 ConCommand cast_hull
r5apex.exe!0x023a7db0 ConCommand cast_ray
r5apex.exe!0x02294b70 ConCommand cc_emit
r5apex.exe!0x01edf530 ConCommand centerview
r5apex.exe!0x01712640 ConCommand changelevel
r5apex.exe!0x017ddb70 ConCommand chat
r5apex.exe!0x01848a40 ConCommand chatroom_adminsOnly
r5apex.exe!0x017dbee0 ConCommand chatroom_away
r5apex.exe!0x01848880 ConCommand chatroom_freetalk
r5apex.exe!0x017dd2a0 ConCommand chatroom_present
r5apex.exe!0x017dc680 ConCommand chatserver
r5apex.exe!0x0171b3d0 ConCommand chroma_base
r5apex.exe!0x0171b330 ConCommand chroma_layer
r5apex.exe!0x022b0f80 ConCommand cl_dump_particle_stats
r5apex.exe!0x02283df0 ConCommand cl_ent_absbox
r5apex.exe!0x01ee7fc0 ConCommand cl_ent_bbox
r5apex.exe!0x01ef6e40 ConCommand cl_ent_rbox
r5apex.exe!0x01ef7030 ConCommand cl_find_ent
r5apex.exe!0x01efc880 ConCommand cl_find_ent_index
r5apex.exe!0x01ef6fb0 ConCommand cl_flip_visibility
r5apex.exe!0x017dcb60 ConCommand cl_fullupdate
r5apex.exe!0x01efdd20 ConCommand cl_interpolation_report
r5apex.exe!0x022a87f0 ConCommand cl_panelanimation
r5apex.exe!0x022a9e20 ConCommand cl_particles_dump_effects
r5apex.exe!0x022aa5d0 ConCommand cl_particles_dumplist
r5apex.exe!0x017dbe60 ConCommand cl_precacheinfo
r5apex.exe!0x01efd9f0 ConCommand cl_removedecals
r5apex.exe!0x017dc8e0 ConCommand cl_showents
r5apex.exe!0x022a2d30 ConCommand cl_soundscape_flush
r5apex.exe!0x022b2120 ConCommand cl_trace_start_solid
r5apex.exe!0x01efe740 ConCommand cl_trace_test_hitbox_with_non_zero_start_offset
r5apex.exe!0x02284270 ConCommand cl_updatevisibility
r5apex.exe!0x02396780 ConCommand clear_debug_overlays
r5apex.exe!0x01712de0 ConCommand clear_loading_progress_detente
r5apex.exe!0x01709380 ConCommand clear_loading_progress_sp_text
r5apex.exe!0x01705310 ConCommand cm_query_log_record
r5apex.exe!0x01707580 ConCommand cm_query_log_replay
r5apex.exe!0x01705410 ConCommand cmd
r5apex.exe!0x01704ea0 ConCommand cmd1
r5apex.exe!0x01705500 ConCommand cmd2
r5apex.exe!0x01704a10 ConCommand cmd3
r5apex.exe!0x01705610 ConCommand cmd4
r5apex.exe!0x01ef0ea0 ConCommand collision_debug
r5apex.exe!0x01707e20 ConCommand colorcorrectionui
r5apex.exe!0x01843660 ConCommand community_browse
r5apex.exe!0x01843810 ConCommand community_getPendingJoinRequest
r5apex.exe!0x018434e0 ConCommand community_join
r5apex.exe!0x01843ad0 ConCommand community_leave
r5apex.exe!0x01843550 ConCommand community_list
r5apex.exe!0x01843a60 ConCommand community_report
r5apex.exe!0x017dd160 ConCommand connect
r5apex.exe!0x017dbde0 ConCommand connectWithKey
r5apex.exe!0x017ddd90 ConCommand connectwithtoken
r5apex.exe!0x01705c00 ConCommand convar_differences
r5apex.exe!0x01705b90 ConCommand convar_findByFlags
r5apex.exe!0x01707600 ConCommand convar_list
r5apex.exe!0x017dc850 ConCommand createparty
r5apex.exe!0x017dd480 ConCommand createpartyifnotinone
r5apex.exe!0x02371e50 ConCommand csm_server_status
r5apex.exe!0x022841f0 ConCommand csm_status
r5apex.exe!0x02355ef0 ConCommand damagedefs_reparse
r5apex.exe!0x022ef510 ConCommand damagedefs_reparse_client
r5apex.exe!0x017dc7b0 ConCommand debugModelPurge
r5apex.exe!0x01705d70 ConCommand devshots_nextmap
r5apex.exe!0x017dcd40 ConCommand devshots_screenshot
r5apex.exe!0x017080c0 ConCommand disconnect
r5apex.exe!0x01712a50 ConCommand display_elapsedtime
r5apex.exe!0x01ef3dc0 ConCommand dlight_debug
r5apex.exe!0x01845360 ConCommand do_InvitePeople_test
r5apex.exe!0x01844e60 ConCommand do_Invite_friend_test
r5apex.exe!0x01844a50 ConCommand do_joinPeople_test
r5apex.exe!0x01844d40 ConCommand do_origin_test_presence
r5apex.exe!0x017179c0 ConCommand downloadPlaylists
r5apex.exe!0x023eaa00 ConCommand drawline
r5apex.exe!0x022b4c80 ConCommand dumpClientStringTable
r5apex.exe!0x0230da30 ConCommand dumpServerStringTable
r5apex.exe!0x02776f00 ConCommand dump_entity_sizes
r5apex.exe!0x0236a1c0 ConCommand dump_generic_key_values
r5apex.exe!0x02784510 ConCommand dumpentityfactories
r5apex.exe!0x0236e170 ConCommand dumpeventqueue
r5apex.exe!0x01713720 ConCommand dumpstringtables
r5apex.exe!0x01705720 ConCommand echo
r5apex.exe!0x01705cf0 ConCommand echo_error
r5apex.exe!0x01848330 ConCommand editor_toggle
r5apex.exe!0x017ddd10 ConCommand endmovie
r5apex.exe!0x0236c6d0 ConCommand ent_absbox
r5apex.exe!0x0236e290 ConCommand ent_animdump
r5apex.exe!0x0236b820 ConCommand ent_attachments
r5apex.exe!0x0236e740 ConCommand ent_bbox
r5apex.exe!0x02370c40 ConCommand ent_cancelpendingentfires
r5apex.exe!0x0236de40 ConCommand ent_create
r5apex.exe!0x02369c60 ConCommand ent_dump
r5apex.exe!0x0236a150 ConCommand ent_entitylinks
r5apex.exe!0x0236ee50 ConCommand ent_fire
r5apex.exe!0x023703c0 ConCommand ent_info
r5apex.exe!0x023701c0 ConCommand ent_messages
r5apex.exe!0x0236e000 ConCommand ent_name
r5apex.exe!0x023702b0 ConCommand ent_orient
r5apex.exe!0x0236b2a0 ConCommand ent_pause
r5apex.exe!0x0236e4d0 ConCommand ent_pivot
r5apex.exe!0x0236e850 ConCommand ent_remove
r5apex.exe!0x02370240 ConCommand ent_remove_all
r5apex.exe!0x0236c530 ConCommand ent_script_dump
r5apex.exe!0x0236e8d0 ConCommand ent_setname
r5apex.exe!0x0236d030 ConCommand ent_step
r5apex.exe!0x02369ad0 ConCommand ent_teleport
r5apex.exe!0x02370ba0 ConCommand ent_text
r5apex.exe!0x02369a50 ConCommand ent_text_radius
r5apex.exe!0x0236f1f0 ConCommand ent_throw
r5apex.exe!0x0236c5a0 ConCommand ent_viewoffset
r5apex.exe!0x01708b00 ConCommand entitlements_print
r5apex.exe!0x01813e50 ConCommand entitlements_send
r5apex.exe!0x0170fb50 ConCommand entitlements_set_bits
r5apex.exe!0x01704d80 ConCommand envmap
r5apex.exe!0x01709400 ConCommand escape
r5apex.exe!0x017079d0 ConCommand exec
r5apex.exe!0x01713060 ConCommand execPlayerConfig
r5apex.exe!0x01705c80 ConCommand execifexists
r5apex.exe!0x01709a50 ConCommand exit
r5apex.exe!0x01ef5660 ConCommand eyeInfo
r5apex.exe!0x0236ea20 ConCommand find_ent
r5apex.exe!0x0236e7b0 ConCommand find_ent_index
r5apex.exe!0x0236d7d0 ConCommand firetarget
r5apex.exe!0x02298640 ConCommand firstperson
r5apex.exe!0x01708820 ConCommand flush
r5apex.exe!0x0170cd40 ConCommand flush_locked
r5apex.exe!0x022a2220 ConCommand force_centerview
r5apex.exe!0x0184e870 ConCommand fps_stats_dump
r5apex.exe!0x0184e910 ConCommand fps_stats_reset
r5apex.exe!0x0184e690 ConCommand fps_stats_start
r5apex.exe!0x0184ecd0 ConCommand fps_stats_stop
r5apex.exe!0x018476a0 ConCommand friends_update
r5apex.exe!0x01849930 ConCommand fs_clear_open_duplicate_times
r5apex.exe!0x01849bb0 ConCommand fs_dump_open_duplicate_times
r5apex.exe!0x0184a350 ConCommand fs_fios_cancel_prefetches
r5apex.exe!0x01849890 ConCommand fs_fios_flush_cache
r5apex.exe!0x01849d60 ConCommand fs_fios_prefetch_file
r5apex.exe!0x0184a0e0 ConCommand fs_fios_prefetch_file_in_pack
r5apex.exe!0x0184a070 ConCommand fs_fios_print_prefetches
r5apex.exe!0x01706030 ConCommand fs_printopenfiles
r5apex.exe!0x01706420 ConCommand fs_warning_level
r5apex.exe!0x02296fd0 ConCommand fx_impact_reparse
r5apex.exe!0x01715860 ConCommand gameui_activate
r5apex.exe!0x01717b00 ConCommand gameui_allowescape
r5apex.exe!0x01719610 ConCommand gameui_allowescapetoshow
r5apex.exe!0x01715600 ConCommand gameui_hide
r5apex.exe!0x01715150 ConCommand gameui_preventescape
r5apex.exe!0x01716980 ConCommand gameui_preventescapetoshow
r5apex.exe!0x01848ca0 ConCommand getNewAuthToken
r5apex.exe!0x01eee460 ConCommand getfov
r5apex.exe!0x0170b3f0 ConCommand gethttpdatacenterlist
r5apex.exe!0x01edf490 ConCommand getpos
r5apex.exe!0x01efc9f0 ConCommand getpos_bind
r5apex.exe!0x02282620 ConCommand getposvec
r5apex.exe!0x022f0a20 ConCommand give
r5apex.exe!0x0235cd70 ConCommand give_server
r5apex.exe!0x02785c30 ConCommand givecurrentammo
r5apex.exe!0x02396a70 ConCommand groundlist
r5apex.exe!0x017063b0 ConCommand help
r5apex.exe!0x02308a50 ConCommand hidepanel
r5apex.exe!0x022a7090 ConCommand hidevideos
r5apex.exe!0x01955290 ConCommand highlight_log
r5apex.exe!0x017107f0 ConCommand host_runofftime
r5apex.exe!0x02299db0 ConCommand hud_subtitles
r5apex.exe!0x01848d50 ConCommand huffman_readProps
r5apex.exe!0x0229eff0 ConCommand impulse
r5apex.exe!0x018438b0 ConCommand inboxmessage_report
r5apex.exe!0x01713540 ConCommand incrementvar
r5apex.exe!0x022a9c60 ConCommand ingamemenu_activate
r5apex.exe!0x01709260 ConCommand initMatchmaking
r5apex.exe!0x02297d20 ConCommand invnext
r5apex.exe!0x01efd210 ConCommand is_considered_sony_multiplayer
r5apex.exe!0x017dc720 ConCommand joinopeninvite
r5apex.exe!0x0229f670 ConCommand joystick_initialize
r5apex.exe!0x017dce60 ConCommand jpeg
r5apex.exe!0x01712f20 ConCommand key_listboundkeys
r5apex.exe!0x0184da80 ConCommand key_updatelayout
r5apex.exe!0x017176a0 ConCommand launchplaylist
r5apex.exe!0x017dd9b0 ConCommand leaveopeninvite
r5apex.exe!0x022b21a0 ConCommand listClientFXScriptHandles
r5apex.exe!0x0170e8c0 ConCommand listmodels
r5apex.exe!0x01716180 ConCommand loadPlaylists
r5apex.exe!0x0170c790 ConCommand map
r5apex.exe!0x0170a3f0 ConCommand map_background
r5apex.exe!0x0170cc20 ConCommand maps
r5apex.exe!0x0170bab0 ConCommand mat_antialias_mode
r5apex.exe!0x01714850 ConCommand mat_configcurrent
r5apex.exe!0x017061e0 ConCommand mat_crosshair
r5apex.exe!0x01705f20 ConCommand mat_crosshair_edit
r5apex.exe!0x023ea580 ConCommand mat_crosshair_edit_all
r5apex.exe!0x01704c60 ConCommand mat_crosshair_explorer
r5apex.exe!0x01706320 ConCommand mat_crosshair_printmaterial
r5apex.exe!0x01707680 ConCommand mat_crosshair_reloadmaterial
r5apex.exe!0x0170d530 ConCommand mat_gamma
r5apex.exe!0x01852290 ConCommand mat_hdr_enabled
r5apex.exe!0x01ed0b30 ConCommand mat_printLiveTex
r5apex.exe!0x0170c910 ConCommand mat_savechanges
r5apex.exe!0x01710870 ConCommand mat_setvideomode
r5apex.exe!0x01ed0dc0 ConCommand mat_shadercount
r5apex.exe!0x01ed0d20 ConCommand mat_spewvertexandpixelshaders
r5apex.exe!0x01714670 ConCommand mat_vsync
r5apex.exe!0x01710910 ConCommand match_abortAllSearches
r5apex.exe!0x0170b2e0 ConCommand match_showAllSearches
r5apex.exe!0x01713c20 ConCommand matchmake
r5apex.exe!0x01713fb0 ConCommand matchmake_cancel
r5apex.exe!0x01711030 ConCommand matchmake_cleanupforparty
r5apex.exe!0x018155b0 ConCommand maxplayers
r5apex.exe!0x0230c5f0 ConCommand melee_lunge_ent
r5apex.exe!0x017130e0 ConCommand mem_compact
r5apex.exe!0x0170c980 ConCommand mem_dump
r5apex.exe!0x0170bf90 ConCommand mem_dump_vm
r5apex.exe!0x01709c10 ConCommand mem_eat
r5apex.exe!0x01712140 ConCommand mem_incremental_compact
r5apex.exe!0x01712cd0 ConCommand mem_leak_vm
r5apex.exe!0x0170ce50 ConCommand mem_test
r5apex.exe!0x01853460 ConCommand mem_textures
r5apex.exe!0x01714230 ConCommand mem_verify
r5apex.exe!0x01853960 ConCommand mem_vram
r5apex.exe!0x0170f180 ConCommand memory
r5apex.exe!0x01711d40 ConCommand migrateme
r5apex.exe!0x022f94b0 ConCommand miles_dump
r5apex.exe!0x022f9610 ConCommand miles_event_info
r5apex.exe!0x022fa050 ConCommand miles_pauseui_byname
r5apex.exe!0x022fac30 ConCommand miles_play
r5apex.exe!0x022fae70 ConCommand miles_reboot
r5apex.exe!0x022f7f40 ConCommand miles_record
r5apex.exe!0x022f8d50 ConCommand miles_record_that
r5apex.exe!0x022f8980 ConCommand miles_stop_all
r5apex.exe!0x022f8420 ConCommand miles_unpauseui_byname
r5apex.exe!0x022f8fb0 ConCommand miles_write_passive_dumpfile
r5apex.exe!0x017dc1c0 ConCommand mmdevinit
r5apex.exe!0x01707fc0 ConCommand multvar
r5apex.exe!0x01848c00 ConCommand muteroom
r5apex.exe!0x017088a0 ConCommand net_channels
r5apex.exe!0x017dd500 ConCommand net_dumpIncomingStats
r5apex.exe!0x017dd040 ConCommand net_dumpOutgoingStats
r5apex.exe!0x017dd340 ConCommand net_dumpStats
r5apex.exe!0x01711210 ConCommand net_start
r5apex.exe!0x0170e200 ConCommand net_status
r5apex.exe!0x0170e640 ConCommand net_writeStatsFile
r5apex.exe!0x017dd780 ConCommand openinvite
r5apex.exe!0x017dcc00 ConCommand openinvitecomplete
r5apex.exe!0x017dca20 ConCommand openinvitelaunch
r5apex.exe!0x01844dc0 ConCommand origin_friendlist_dump
r5apex.exe!0x01eedb80 ConCommand particle_create
r5apex.exe!0x022842f0 ConCommand particle_create_ss
r5apex.exe!0x022a9aa0 ConCommand particle_dump
r5apex.exe!0x01edfae0 ConCommand particle_kill
r5apex.exe!0x022af350 ConCommand particle_list
r5apex.exe!0x01eed7c0 ConCommand particle_recreate
r5apex.exe!0x022a7bc0 ConCommand particle_remove_all
r5apex.exe!0x01eeaf80 ConCommand particle_scrub_bake
r5apex.exe!0x0228eb40 ConCommand particle_scrub_play
r5apex.exe!0x02282320 ConCommand particle_scrub_stop
r5apex.exe!0x02331760 ConCommand particle_test_start
r5apex.exe!0x0230acd0 ConCommand particle_test_stop
r5apex.exe!0x01845ea0 ConCommand party_leave
r5apex.exe!0x018459a0 ConCommand party_serverChange
r5apex.exe!0x01707a50 ConCommand path
r5apex.exe!0x01709890 ConCommand pause
r5apex.exe!0x022ac190 ConCommand pausevideos
r5apex.exe!0x022ac370 ConCommand phys_objectDump
r5apex.exe!0x01ee7a40 ConCommand phys_throw_client
r5apex.exe!0x023eada0 ConCommand physics_budget
r5apex.exe!0x02373e60 ConCommand physics_debug_entity
r5apex.exe!0x02395b90 ConCommand physics_highlight_active
r5apex.exe!0x023e98a0 ConCommand physics_report_active
r5apex.exe!0x023e9780 ConCommand physics_select
r5apex.exe!0x0236edb0 ConCommand picker
r5apex.exe!0x0170a9c0 ConCommand ping
r5apex.exe!0x022abcf0 ConCommand ping_specific_type
r5apex.exe!0x01713b80 ConCommand pingdatacenters
r5apex.exe!0x01edf5d0 ConCommand pixelvis_debug
r5apex.exe!0x022e6270 ConCommand playerSettings_reparse
r5apex.exe!0x022966a0 ConCommand playsoundscape
r5apex.exe!0x022ad270 ConCommand playvideo
r5apex.exe!0x022aa0a0 ConCommand playvideo_end_level_transition
r5apex.exe!0x022afb80 ConCommand playvideo_exitcommand
r5apex.exe!0x022affc0 ConCommand playvideo_exitcommand_nointerrupt
r5apex.exe!0x022b0980 ConCommand playvideo_nointerrupt
r5apex.exe!0x022acc90 ConCommand playvideo_scaled
r5apex.exe!0x01705490 ConCommand print_colorcorrection
r5apex.exe!0x01718320 ConCommand progress_enable
r5apex.exe!0x02786fb0 ConCommand prop_debug
r5apex.exe!0x0170e780 ConCommand quit
r5apex.exe!0x022a86d0 ConCommand r_cheapwaterend
r5apex.exe!0x022af110 ConCommand r_cheapwaterstart
r5apex.exe!0x0170beb0 ConCommand r_cleardecals
r5apex.exe!0x01956df0 ConCommand r_dxgi_max_frame_latency
r5apex.exe!0x01716e80 ConCommand r_printdecalinfo
r5apex.exe!0x01956700 ConCommand r_volumetric_lighting_color
r5apex.exe!0x018430b0 ConCommand readMsgs
r5apex.exe!0x01714410 ConCommand recheck
r5apex.exe!0x0170c8a0 ConCommand recompute_speed
r5apex.exe!0x01710c30 ConCommand reload
r5apex.exe!0x018140b0 ConCommand reload_localization
r5apex.exe!0x022e6650 ConCommand reload_script_callbacks
r5apex.exe!0x0235d830 ConCommand reload_script_callbacks_server
r5apex.exe!0x01847ba0 ConCommand render_blanks
r5apex.exe!0x023a7800 ConCommand report_entities
r5apex.exe!0x023a8160 ConCommand report_simthinklist
r5apex.exe!0x023eabd0 ConCommand report_touchlinks
r5apex.exe!0x0229a6d0 ConCommand reset_cam_ideal_angles
r5apex.exe!0x023e9810 ConCommand resetidletimer
r5apex.exe!0x017115d0 ConCommand restart
r5apex.exe!0x0170e6e0 ConCommand restart_checkpoint
r5apex.exe!0x022f9db0 ConCommand rumble_print
r5apex.exe!0x017084a0 ConCommand savePlayerConfig
r5apex.exe!0x022aee20 ConCommand scoreboard_down
r5apex.exe!0x022aaf40 ConCommand scoreboard_focus
r5apex.exe!0x022afc20 ConCommand scoreboard_mute
r5apex.exe!0x022b1860 ConCommand scoreboard_profile
r5apex.exe!0x022aeca0 ConCommand scoreboard_toggle_focus
r5apex.exe!0x022adaf0 ConCommand scoreboard_up
r5apex.exe!0x017dc140 ConCommand screenshot
r5apex.exe!0x027851a0 ConCommand script_printdiag
r5apex.exe!0x0170e280 ConCommand server_single_frame
r5apex.exe!0x01716c00 ConCommand serverinfo
r5apex.exe!0x022ecf60 ConCommand set
r5apex.exe!0x0170bc60 ConCommand set_loading_progress_background
r5apex.exe!0x017147b0 ConCommand set_loading_progress_detente
r5apex.exe!0x01708520 ConCommand set_loading_progress_fadeout_enabled
r5apex.exe!0x01708300 ConCommand set_loading_progress_sp_text
r5apex.exe!0x017dd640 ConCommand setinfo
r5apex.exe!0x01706da0 ConCommand settype
r5apex.exe!0x023972a0 ConCommand shake
r5apex.exe!0x02779560 ConCommand shake_ropes
r5apex.exe!0x022afe00 ConCommand shake_stop
r5apex.exe!0x022b0280 ConCommand shake_testpunch
r5apex.exe!0x017149d0 ConCommand show_loading_progress
r5apex.exe!0x023089d0 ConCommand showpanel
r5apex.exe!0x02782d80 ConCommand showtriggers_toggle
r5apex.exe!0x022af190 ConCommand showvideos
r5apex.exe!0x017dc440 ConCommand silentconnect
r5apex.exe!0x01717f60 ConCommand skill_writeTrainingData
r5apex.exe!0x02785ea0 ConCommand skybox_swap
r5apex.exe!0x02396bc0 ConCommand snapshot_memory_report
r5apex.exe!0x01efc620 ConCommand soundscape_dumpclient
r5apex.exe!0x0276b920 ConCommand soundscape_flush
r5apex.exe!0x022a7d00 ConCommand spawn_as_pilot
r5apex.exe!0x022af3d0 ConCommand spawn_as_titan
r5apex.exe!0x0170de40 ConCommand ss_map
r5apex.exe!0x022a6450 ConCommand ss_reloadletterbox
r5apex.exe!0x022a9d80 ConCommand sssss_enable
r5apex.exe!0x01716b60 ConCommand star_memory
r5apex.exe!0x017ddbf0 ConCommand startmovie
r5apex.exe!0x01713860 ConCommand status
r5apex.exe!0x018474c0 ConCommand steamlink
r5apex.exe!0x01847600 ConCommand steamunlink
r5apex.exe!0x022ab2e0 ConCommand stop_transition_videos_fadeout
r5apex.exe!0x023a76c0 ConCommand stopserver
r5apex.exe!0x022a26c0 ConCommand stopsoundscape
r5apex.exe!0x022a96d0 ConCommand stopvideos
r5apex.exe!0x022a5c20 ConCommand stopvideos_fadeout
r5apex.exe!0x02397330 ConCommand surfaceprop
r5apex.exe!0x01815210 ConCommand sv_precacheinfo
r5apex.exe!0x01816970 ConCommand sv_showents
r5apex.exe!0x01816370 ConCommand sv_shutdown
r5apex.exe!0x02788270 ConCommand sv_soundscape_printdebuginfo
r5apex.exe!0x027a4b20 ConCommand sv_test_rotated_box
r5apex.exe!0x0232faa0 ConCommand sv_trace_start_solid
r5apex.exe!0x01817d20 ConCommand sv_writeSendTableStreamFile
r5apex.exe!0x0279e410 ConCommand swap_to_weapon
r5apex.exe!0x02355940 ConCommand switchPlayerClassActivityMod
r5apex.exe!0x02351a80 ConCommand switchclass
r5apex.exe!0x027865d0 ConCommand takecurrentammo
r5apex.exe!0x01ede8a0 ConCommand testCockpitJoltAngles
r5apex.exe!0x01f01a80 ConCommand testCockpitJoltOrigin
r5apex.exe!0x02335e50 ConCommand test_entity_blocker
r5apex.exe!0x022a5e00 ConCommand test_freezeframe
r5apex.exe!0x023ea610 ConCommand test_setteam
r5apex.exe!0x022a44f0 ConCommand testhudanim
r5apex.exe!0x017118f0 ConCommand thread_test_tslist
r5apex.exe!0x01708040 ConCommand thread_test_tsqueue
r5apex.exe!0x022a6ed0 ConCommand titan_loadout_select
r5apex.exe!0x017073d0 ConCommand toggle
r5apex.exe!0x022af870 ConCommand toggle_inventory
r5apex.exe!0x022a97f0 ConCommand toggle_map
r5apex.exe!0x023481a0 ConCommand trace_capsule
r5apex.exe!0x022f6620 ConCommand ui_reloadscheme
r5apex.exe!0x022aa120 ConCommand uiscript_reset
r5apex.exe!0x022ad4b0 ConCommand uiscript_resolutionchanged
r5apex.exe!0x0170a940 ConCommand unbind
r5apex.exe!0x0170c680 ConCommand unbind_US_standard
r5apex.exe!0x01710450 ConCommand unbind_all_gamepad
r5apex.exe!0x0170ebe0 ConCommand unbind_batch
r5apex.exe!0x01710030 ConCommand unbind_held
r5apex.exe!0x01709810 ConCommand unbind_held_US_standard
r5apex.exe!0x0170c9f0 ConCommand unbindall
r5apex.exe!0x0170ba40 ConCommand unbindall_ignoreGamepad
r5apex.exe!0x0170ff10 ConCommand unload_level_loadscreen
r5apex.exe!0x01848920 ConCommand unmuteroom
r5apex.exe!0x022adc10 ConCommand unpausevideos
r5apex.exe!0x022ad430 ConCommand use_consumable
r5apex.exe!0x01817030 ConCommand user
r5apex.exe!0x018167b0 ConCommand users
r5apex.exe!0x01710150 ConCommand version
r5apex.exe!0x017171a0 ConCommand vgui_drawtree_clear
r5apex.exe!0x01ed8090 ConCommand vgui_spew_fonts
r5apex.exe!0x017187c0 ConCommand vgui_togglepanel
r5apex.exe!0x0170a190 ConCommand voicerecord_toggle
r5apex.exe!0x01708140 ConCommand vx_datacache_list
r5apex.exe!0x0170aae0 ConCommand vx_model_list
r5apex.exe!0x02299730 ConCommand weaponSelectOrdnance
r5apex.exe!0x022a1730 ConCommand weaponSelectPrimary0
r5apex.exe!0x02296840 ConCommand weaponSelectPrimary1
r5apex.exe!0x02295a20 ConCommand weaponSelectPrimary2
r5apex.exe!0x0229f550 ConCommand weapon_activity
r5apex.exe!0x0228f560 ConCommand weapon_inspect
r5apex.exe!0x01eee620 ConCommand weapon_list
r5apex.exe!0x022ee870 ConCommand weapon_reparse
r5apex.exe!0x0170c000 ConCommand xlog_list
r5apex.exe!0x01714550 ConCommand xlog_record
r5apex.exe!0x0170ff90 ConCommand xlog_record_that
r5apex.exe!0x0170b4d0 ConCommand xlog_stop
r5apex.exe!0x0229dcc0 ConCommand xlook
r5apex.exe!0x0228ff20 ConCommand xmove
```

## Globals

List of global variables with an associated vtable and their type name.

```
r5apex.exe!0x02782e38 .?AUSQArray@@
r5apex.exe!0x02777158 .?AUSQClass@@
r5apex.exe!0x02776948 .?AUSQClosure@@
r5apex.exe!0x023ea818 .?AUSQFunctionProto@@
r5apex.exe!0x023a7e38 .?AUSQInstance@@
r5apex.exe!0x02783188 .?AUSQNativeClosure@@
r5apex.exe!0x023e9898 .?AUSQString@@
r5apex.exe!0x0277ecf8 .?AUSQStructDef@@
r5apex.exe!0x023ea608 .?AUSQStructInstance@@
r5apex.exe!0x023ea698 .?AUSQTable@@
r5apex.exe!0x023a81e8 .?AUSQUserData@@
r5apex.exe!0x02784598 .?AUSQVM@@
r5apex.exe!0x02785228 .?AUSQWeakRef@@
r5apex.exe!0x22708990 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x227089f8 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22708a00 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22708a30 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22708a98 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22708aa0 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22708ad0 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22708b38 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22708b40 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22708b70 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22708bd8 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22708be0 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22708c10 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22708c78 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22708c80 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22708cb0 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22708d18 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x22708d20 .?AV?$CConCommandMemberAccessor@VCMaterialSystem@@@@
r5apex.exe!0x017040e8 .?AV?$CDataManager@UDataCacheItem_t@@UDataCacheItemData_t@@PEAU1@VCThreadFastMutex@@@@
r5apex.exe!0x0c65a1e0 .?AV?$CDataManager@VCBoneCache@@Ubonecacheparams_t@@PEAV1@VCThreadFastMutex@@@@
r5apex.exe!0x0c65a290 .?AV?$CDataManager@VCBoneCache@@Ubonecacheparams_t@@PEAV1@VCThreadFastMutex@@@@
r5apex.exe!0x0277b3a0 .?AV?$CEntityClassList@VCPhysicsNPCSolver@@@@
r5apex.exe!0x02776660 .?AV?$CEntityClassList@VCPointCamera@@@@
r5apex.exe!0x02785740 .?AV?$CEntityClassList@VCSkyCamera@@@@
r5apex.exe!0x02848270 .?AV?$CEntityFactory@VCAI_BaseNPC@@@@
r5apex.exe!0x028430b0 .?AV?$CEntityFactory@VCAI_ChangeTarget@@@@
r5apex.exe!0x02848828 .?AV?$CEntityFactory@VCAI_DynamicLink@@@@
r5apex.exe!0x02848598 .?AV?$CEntityFactory@VCAI_DynamicLinkController@@@@
r5apex.exe!0x028487e8 .?AV?$CEntityFactory@VCAI_Hint@@@@
r5apex.exe!0x02848500 .?AV?$CEntityFactory@VCAI_NetworkManager@@@@
r5apex.exe!0x02848010 .?AV?$CEntityFactory@VCAI_RadialLinkController@@@@
r5apex.exe!0x02840cd8 .?AV?$CEntityFactory@VCAI_SkitNode@@@@
r5apex.exe!0x028484c8 .?AV?$CEntityFactory@VCAI_TestHull@@@@
r5apex.exe!0x028407d8 .?AV?$CEntityFactory@VCAmbientGeneric@@@@
r5apex.exe!0x028482a8 .?AV?$CEntityFactory@VCAssaultPoint@@@@
r5apex.exe!0x02841178 .?AV?$CEntityFactory@VCBaseAnimating@@@@
r5apex.exe!0x02842448 .?AV?$CEntityFactory@VCBaseDMStart@@@@
r5apex.exe!0x02840c08 .?AV?$CEntityFactory@VCBaseEntity@@@@
r5apex.exe!0x0283df18 .?AV?$CEntityFactory@VCBaseGrenade@@@@
r5apex.exe!0x02842c80 .?AV?$CEntityFactory@VCBaseTrigger@@@@
r5apex.exe!0x0283d078 .?AV?$CEntityFactory@VCBaseViewModel@@@@
r5apex.exe!0x0283e2c8 .?AV?$CEntityFactory@VCBeam@@@@
r5apex.exe!0x02841618 .?AV?$CEntityFactory@VCBoneFollower@@@@
r5apex.exe!0x028418b0 .?AV?$CEntityFactory@VCBreakable@@@@
r5apex.exe!0x028418b8 .?AV?$CEntityFactory@VCBreakableSurface@@@@
r5apex.exe!0x028419a8 .?AV?$CEntityFactory@VCCascadeLight@@@@
r5apex.exe!0x02841a70 .?AV?$CEntityFactory@VCColorCorrection@@@@
r5apex.exe!0x02848e18 .?AV?$CEntityFactory@VCCrossbowBolt@@@@
r5apex.exe!0x02847f08 .?AV?$CEntityFactory@VCDeathBoxProp@@@@
r5apex.exe!0x02847a68 .?AV?$CEntityFactory@VCDropPodProp@@@@
r5apex.exe!0x028430b8 .?AV?$CEntityFactory@VCDropPodSpawnPoint@@@@
r5apex.exe!0x02843148 .?AV?$CEntityFactory@VCDropPodSpawnPoint@@@@
r5apex.exe!0x02842900 .?AV?$CEntityFactory@VCDropPoint@@@@
r5apex.exe!0x028435a8 .?AV?$CEntityFactory@VCDropShipSpawnPoint@@@@
r5apex.exe!0x028478d8 .?AV?$CEntityFactory@VCDropShipSpawnPoint@@@@
r5apex.exe!0x02841748 .?AV?$CEntityFactory@VCDynamicLight@@@@
r5apex.exe!0x02842278 .?AV?$CEntityFactory@VCDynamicProp@@@@
r5apex.exe!0x02842738 .?AV?$CEntityFactory@VCDynamicProp@@@@
r5apex.exe!0x028431c0 .?AV?$CEntityFactory@VCDynamicProp@@@@
r5apex.exe!0x02843388 .?AV?$CEntityFactory@VCDynamicProp@@@@
r5apex.exe!0x02843568 .?AV?$CEntityFactory@VCDynamicProp@@@@
r5apex.exe!0x028435b8 .?AV?$CEntityFactory@VCDynamicProp@@@@
r5apex.exe!0x028431b0 .?AV?$CEntityFactory@VCDynamicPropLightweight@@@@
r5apex.exe!0x02842a18 .?AV?$CEntityFactory@VCEnableMotionFixup@@@@
r5apex.exe!0x0283dd38 .?AV?$CEntityFactory@VCEntityBlocker@@@@
r5apex.exe!0x02841ff0 .?AV?$CEntityFactory@VCEntityDissolve@@@@
r5apex.exe!0x02842cc8 .?AV?$CEntityFactory@VCEntityLinkPage@@@@
r5apex.exe!0x02841dd0 .?AV?$CEntityFactory@VCEnvBeam@@@@
r5apex.exe!0x028431b8 .?AV?$CEntityFactory@VCEnvDropZone@@@@
r5apex.exe!0x02841bb0 .?AV?$CEntityFactory@VCEnvExplosion@@@@
r5apex.exe!0x02841fe8 .?AV?$CEntityFactory@VCEnvLaser@@@@
r5apex.exe!0x028404a8 .?AV?$CEntityFactory@VCEnvLight@@@@
r5apex.exe!0x02841960 .?AV?$CEntityFactory@VCEnvShake@@@@
r5apex.exe!0x02842f70 .?AV?$CEntityFactory@VCEnvSoundscape@@@@
r5apex.exe!0x02843150 .?AV?$CEntityFactory@VCEnvSoundscapeProxy@@@@
r5apex.exe!0x02842db0 .?AV?$CEntityFactory@VCEnvSoundscapeTriggerable@@@@
r5apex.exe!0x028415c8 .?AV?$CEntityFactory@VCEnvTonemapController@@@@
r5apex.exe!0x02841f20 .?AV?$CEntityFactory@VCEnvWind@@@@
r5apex.exe!0x02849418 .?AV?$CEntityFactory@VCFirstPersonProxy@@@@
r5apex.exe!0x02841558 .?AV?$CEntityFactory@VCFogController@@@@
r5apex.exe!0x02841cc0 .?AV?$CEntityFactory@VCFogVolume@@@@
r5apex.exe!0x02841bc8 .?AV?$CEntityFactory@VCFuncBrush@@@@
r5apex.exe!0x02841780 .?AV?$CEntityFactory@VCFuncBrushLightweight@@@@
r5apex.exe!0x02841788 .?AV?$CEntityFactory@VCFuncMoveLinear@@@@
r5apex.exe!0x0283f0b8 .?AV?$CEntityFactory@VCGameGibManager@@@@
r5apex.exe!0x028415d0 .?AV?$CEntityFactory@VCGameOperator@@@@
r5apex.exe!0x02841ae0 .?AV?$CEntityFactory@VCGamePlayerEquip@@@@
r5apex.exe!0x02841800 .?AV?$CEntityFactory@VCGamePlayerTeam@@@@
r5apex.exe!0x0283dff8 .?AV?$CEntityFactory@VCGameRulesProxy@@@@
r5apex.exe!0x02841bc0 .?AV?$CEntityFactory@VCGameText@@@@
r5apex.exe!0x028417f8 .?AV?$CEntityFactory@VCGameUIEntity@@@@
r5apex.exe!0x02841f18 .?AV?$CEntityFactory@VCGib@@@@
r5apex.exe!0x0283df08 .?AV?$CEntityFactory@VCGlobalNonRewinding@@@@
r5apex.exe!0x028489a8 .?AV?$CEntityFactory@VCGrappleHook@@@@
r5apex.exe!0x02842030 .?AV?$CEntityFactory@VCHardPointEntity@@@@
r5apex.exe!0x02847d98 .?AV?$CEntityFactory@VCHardPointFrontierEntity@@@@
r5apex.exe!0x02840438 .?AV?$CEntityFactory@VCHealthKit@@@@
r5apex.exe!0x02840440 .?AV?$CEntityFactory@VCHealthKit@@@@
r5apex.exe!0x02840448 .?AV?$CEntityFactory@VCHealthKit@@@@
r5apex.exe!0x02840450 .?AV?$CEntityFactory@VCHealthKit@@@@
r5apex.exe!0x02840458 .?AV?$CEntityFactory@VCHealthKit@@@@
r5apex.exe!0x02840460 .?AV?$CEntityFactory@VCHealthKit@@@@
r5apex.exe!0x02842288 .?AV?$CEntityFactory@VCHumanSizeNPCSpawnPoint@@@@
r5apex.exe!0x02843530 .?AV?$CEntityFactory@VCHumanSizeNPCSpawnPoint@@@@
r5apex.exe!0x028421b0 .?AV?$CEntityFactory@VCHumanSpawnPoint@@@@
r5apex.exe!0x02842d10 .?AV?$CEntityFactory@VCHumanSpawnPoint@@@@
r5apex.exe!0x02847e58 .?AV?$CEntityFactory@VCImportantOnEntSound@@@@
r5apex.exe!0x02841bb8 .?AV?$CEntityFactory@VCInfoCameraLink@@@@
r5apex.exe!0x02841c80 .?AV?$CEntityFactory@VCInfoIntermission@@@@
r5apex.exe!0x028488c8 .?AV?$CEntityFactory@VCInfoPlacementHelper@@@@
r5apex.exe!0x0283cc28 .?AV?$CEntityFactory@VCInfoTarget@@@@
r5apex.exe!0x0283ce48 .?AV?$CEntityFactory@VCInfoTarget@@@@
r5apex.exe!0x0283d268 .?AV?$CEntityFactory@VCInfoTarget@@@@
r5apex.exe!0x0283df10 .?AV?$CEntityFactory@VCInfoTarget@@@@
r5apex.exe!0x0283dd40 .?AV?$CEntityFactory@VCInfoTargetMinimap@@@@
r5apex.exe!0x02840498 .?AV?$CEntityFactory@VCLight@@@@
r5apex.exe!0x028404a0 .?AV?$CEntityFactory@VCLight@@@@
r5apex.exe!0x02841650 .?AV?$CEntityFactory@VCMessageEntity@@@@
r5apex.exe!0x02848bf0 .?AV?$CEntityFactory@VCMissile@@@@
r5apex.exe!0x02847db0 .?AV?$CEntityFactory@VCMovementSpeedMod@@@@
r5apex.exe!0x02841b48 .?AV?$CEntityFactory@VCMovieDisplay@@@@
r5apex.exe!0x02847d20 .?AV?$CEntityFactory@VCNPCProwlerSpawnPoint@@@@
r5apex.exe!0x02848860 .?AV?$CEntityFactory@VCNPC_Bullseye@@@@
r5apex.exe!0x02841808 .?AV?$CEntityFactory@VCNPC_Drone@@@@
r5apex.exe!0x02841ad8 .?AV?$CEntityFactory@VCNPC_Drone@@@@
r5apex.exe!0x02841d68 .?AV?$CEntityFactory@VCNPC_Drone@@@@
r5apex.exe!0x02841e70 .?AV?$CEntityFactory@VCNPC_Drone@@@@
r5apex.exe!0x02841570 .?AV?$CEntityFactory@VCNPC_Dropship@@@@
r5apex.exe!0x02841958 .?AV?$CEntityFactory@VCNPC_Dropship@@@@
r5apex.exe!0x02841810 .?AV?$CEntityFactory@VCNPC_Flyer@@@@
r5apex.exe!0x02841cc8 .?AV?$CEntityFactory@VCNPC_Goliath@@@@
r5apex.exe!0x028414f0 .?AV?$CEntityFactory@VCNPC_Gunship@@@@
r5apex.exe!0x02849750 .?AV?$CEntityFactory@VCNPC_Marvin@@@@
r5apex.exe!0x028415d8 .?AV?$CEntityFactory@VCNPC_MeleeOnly@@@@
r5apex.exe!0x02841c00 .?AV?$CEntityFactory@VCNPC_MeleeOnly@@@@
r5apex.exe!0x02841ff8 .?AV?$CEntityFactory@VCNPC_MeleeOnly@@@@
r5apex.exe!0x028497c0 .?AV?$CEntityFactory@VCNPC_Pilot@@@@
r5apex.exe!0x028498a8 .?AV?$CEntityFactory@VCNPC_SentryTurret@@@@
r5apex.exe!0x028498b0 .?AV?$CEntityFactory@VCNPC_SentryTurret@@@@
r5apex.exe!0x02849760 .?AV?$CEntityFactory@VCNPC_Soldier@@@@
r5apex.exe!0x02849768 .?AV?$CEntityFactory@VCNPC_Soldier@@@@
r5apex.exe!0x02849778 .?AV?$CEntityFactory@VCNPC_Soldier@@@@
r5apex.exe!0x02849780 .?AV?$CEntityFactory@VCNPC_Soldier@@@@
r5apex.exe!0x02849788 .?AV?$CEntityFactory@VCNPC_Soldier@@@@
r5apex.exe!0x028497c8 .?AV?$CEntityFactory@VCNPC_Soldier@@@@
r5apex.exe!0x02849758 .?AV?$CEntityFactory@VCNPC_Spectre@@@@
r5apex.exe!0x02849770 .?AV?$CEntityFactory@VCNPC_Spectre@@@@
r5apex.exe!0x028497d0 .?AV?$CEntityFactory@VCNPC_Spectre@@@@
r5apex.exe!0x02841ee0 .?AV?$CEntityFactory@VCNPC_SuperSpectre@@@@
r5apex.exe!0x02849868 .?AV?$CEntityFactory@VCNPC_Titan@@@@
r5apex.exe!0x02847fd8 .?AV?$CEntityFactory@VCNodeEnt@@@@
r5apex.exe!0x02848078 .?AV?$CEntityFactory@VCNodeEnt@@@@
r5apex.exe!0x02848200 .?AV?$CEntityFactory@VCNodeEnt@@@@
r5apex.exe!0x02848268 .?AV?$CEntityFactory@VCNodeEnt@@@@
r5apex.exe!0x02848300 .?AV?$CEntityFactory@VCNodeEnt@@@@
r5apex.exe!0x02848398 .?AV?$CEntityFactory@VCNodeEnt@@@@
r5apex.exe!0x028483a0 .?AV?$CEntityFactory@VCNodeEnt@@@@
r5apex.exe!0x028485a0 .?AV?$CEntityFactory@VCNodeEnt@@@@
r5apex.exe!0x028485a8 .?AV?$CEntityFactory@VCNodeEnt@@@@
r5apex.exe!0x02848610 .?AV?$CEntityFactory@VCNodeEnt@@@@
r5apex.exe!0x028486a8 .?AV?$CEntityFactory@VCNodeEnt@@@@
r5apex.exe!0x028486f0 .?AV?$CEntityFactory@VCNodeEnt@@@@
r5apex.exe!0x02848820 .?AV?$CEntityFactory@VCNodeEnt@@@@
r5apex.exe!0x02842148 .?AV?$CEntityFactory@VCNullEntity@@@@
r5apex.exe!0x02842a10 .?AV?$CEntityFactory@VCNullEntity@@@@
r5apex.exe!0x02841ea8 .?AV?$CEntityFactory@VCParticleSystem@@@@
r5apex.exe!0x02841790 .?AV?$CEntityFactory@VCPathCorner@@@@
r5apex.exe!0x02841968 .?AV?$CEntityFactory@VCPathCorner@@@@
r5apex.exe!0x02841cd0 .?AV?$CEntityFactory@VCPathCorner@@@@
r5apex.exe!0x028419a0 .?AV?$CEntityFactory@VCPathCornerCrash@@@@
r5apex.exe!0x02841920 .?AV?$CEntityFactory@VCPathTrack@@@@
r5apex.exe!0x02841e08 .?AV?$CEntityFactory@VCPatrolPath@@@@
r5apex.exe!0x02842d48 .?AV?$CEntityFactory@VCPhysBox@@@@
r5apex.exe!0x02847748 .?AV?$CEntityFactory@VCPhysExplosion@@@@
r5apex.exe!0x02847a70 .?AV?$CEntityFactory@VCPhysImpact@@@@
r5apex.exe!0x02843570 .?AV?$CEntityFactory@VCPhysicsEntitySolver@@@@
r5apex.exe!0x02843160 .?AV?$CEntityFactory@VCPhysicsNPCSolver@@@@
r5apex.exe!0x02842f78 .?AV?$CEntityFactory@VCPhysicsProp@@@@
r5apex.exe!0x028476a0 .?AV?$CEntityFactory@VCPhysicsProp@@@@
r5apex.exe!0x02847750 .?AV?$CEntityFactory@VCPhysicsProp@@@@
r5apex.exe!0x02842068 .?AV?$CEntityFactory@VCPlayer@@@@
r5apex.exe!0x0283fe58 .?AV?$CEntityFactory@VCPlayerDecoy@@@@
r5apex.exe!0x028429d8 .?AV?$CEntityFactory@VCPlayerResource@@@@
r5apex.exe!0x028400b8 .?AV?$CEntityFactory@VCPlayerTasklist@@@@
r5apex.exe!0x02842fc0 .?AV?$CEntityFactory@VCPlayerVehicle@@@@
r5apex.exe!0x0283ff48 .?AV?$CEntityFactory@VCPlayerWaypoint@@@@
r5apex.exe!0x02841cb8 .?AV?$CEntityFactory@VCPointBroadcastClientCommand@@@@
r5apex.exe!0x02842fb0 .?AV?$CEntityFactory@VCPointCamera@@@@
r5apex.exe!0x028414e8 .?AV?$CEntityFactory@VCPointClientCommand@@@@
r5apex.exe!0x028424b8 .?AV?$CEntityFactory@VCPointEntity@@@@
r5apex.exe!0x028426c8 .?AV?$CEntityFactory@VCPointEntity@@@@
r5apex.exe!0x02842bb8 .?AV?$CEntityFactory@VCPointEntity@@@@
r5apex.exe!0x02842f08 .?AV?$CEntityFactory@VCPointEntity@@@@
r5apex.exe!0x028430d8 .?AV?$CEntityFactory@VCPointEntity@@@@
r5apex.exe!0x028430e0 .?AV?$CEntityFactory@VCPointEntity@@@@
r5apex.exe!0x028431f8 .?AV?$CEntityFactory@VCPointEntity@@@@
r5apex.exe!0x02847b10 .?AV?$CEntityFactory@VCPointEntity@@@@
r5apex.exe!0x028486e8 .?AV?$CEntityFactory@VCPointEntity@@@@
r5apex.exe!0x02847b18 .?AV?$CEntityFactory@VCPointPlayerMoveConstraint@@@@
r5apex.exe!0x02841c40 .?AV?$CEntityFactory@VCPointServerCommand@@@@
r5apex.exe!0x028434a8 .?AV?$CEntityFactory@VCPointSpotlight@@@@
r5apex.exe!0x02847bf8 .?AV?$CEntityFactory@VCPointTemplate@@@@
r5apex.exe!0x02847e68 .?AV?$CEntityFactory@VCPointTemplate@@@@
r5apex.exe!0x028488d0 .?AV?$CEntityFactory@VCPortal_PointPush@@@@
r5apex.exe!0x028428f8 .?AV?$CEntityFactory@VCPostProcessController@@@@
r5apex.exe!0x028491e8 .?AV?$CEntityFactory@VCPredictedFirstPersonProxy@@@@
r5apex.exe!0x0283d348 .?AV?$CEntityFactory@VCPropDoor@@@@
r5apex.exe!0x02847c68 .?AV?$CEntityFactory@VCPropSurvival@@@@
r5apex.exe!0x02841560 .?AV?$CEntityFactory@VCPushable@@@@
r5apex.exe!0x028430c0 .?AV?$CEntityFactory@VCRevertSaved@@@@
r5apex.exe!0x02842fb8 .?AV?$CEntityFactory@VCRopeKeyframe@@@@
r5apex.exe!0x02847ed0 .?AV?$CEntityFactory@VCRopeKeyframe@@@@
r5apex.exe!0x0283ed58 .?AV?$CEntityFactory@VCScriptMover@@@@
r5apex.exe!0x0283f228 .?AV?$CEntityFactory@VCScriptMoverTrainNode@@@@
r5apex.exe!0x0283f860 .?AV?$CEntityFactory@VCScriptNetDataGlobal@@@@
r5apex.exe!0x0283e758 .?AV?$CEntityFactory@VCScriptNetData_SNDC_DEATH_BOX@@@@
r5apex.exe!0x0283edf8 .?AV?$CEntityFactory@VCScriptNetData_SNDC_GLOBAL@@@@
r5apex.exe!0x0283f9b8 .?AV?$CEntityFactory@VCScriptNetData_SNDC_PLAYER_EXCLUSIVE@@@@
r5apex.exe!0x0283f858 .?AV?$CEntityFactory@VCScriptNetData_SNDC_PLAYER_GLOBAL@@@@
r5apex.exe!0x0283f778 .?AV?$CEntityFactory@VCScriptNetData_SNDC_TITAN_SOUL@@@@
r5apex.exe!0x02842440 .?AV?$CEntityFactory@VCScriptProp@@@@
r5apex.exe!0x02843410 .?AV?$CEntityFactory@VCScriptProp@@@@
r5apex.exe!0x028492c8 .?AV?$CEntityFactory@VCScriptTraceVolume@@@@
r5apex.exe!0x02841c08 .?AV?$CEntityFactory@VCSearchPath@@@@
r5apex.exe!0x02842700 .?AV?$CEntityFactory@VCShieldProp@@@@
r5apex.exe!0x028422c8 .?AV?$CEntityFactory@VCSimplePhysicsBrush@@@@
r5apex.exe!0x02843168 .?AV?$CEntityFactory@VCSimplePhysicsProp@@@@
r5apex.exe!0x02843200 .?AV?$CEntityFactory@VCSkyCamera@@@@
r5apex.exe!0x02847828 .?AV?$CEntityFactory@VCSkyboxSwapper@@@@
r5apex.exe!0x028434c0 .?AV?$CEntityFactory@VCSoundEnt@@@@
r5apex.exe!0x028433d0 .?AV?$CEntityFactory@VCSpawnPointFlag@@@@
r5apex.exe!0x02842280 .?AV?$CEntityFactory@VCSpawner@@@@
r5apex.exe!0x02842438 .?AV?$CEntityFactory@VCSpotlightEnd@@@@
r5apex.exe!0x0283fed0 .?AV?$CEntityFactory@VCSprite@@@@
r5apex.exe!0x0283e6e8 .?AV?$CEntityFactory@VCSpriteOriented@@@@
r5apex.exe!0x0283f188 .?AV?$CEntityFactory@VCStatusEffectPlugin@@@@
r5apex.exe!0x028430d0 .?AV?$CEntityFactory@VCTeam@@@@
r5apex.exe!0x02843038 .?AV?$CEntityFactory@VCTeamSpawnPoint@@@@
r5apex.exe!0x02842e58 .?AV?$CEntityFactory@VCTeamVehicleSpawnPoint@@@@
r5apex.exe!0x02841818 .?AV?$CEntityFactory@VCTempEntTester@@@@
r5apex.exe!0x028498a0 .?AV?$CEntityFactory@VCTitanSoul@@@@
r5apex.exe!0x02842818 .?AV?$CEntityFactory@VCTitanSpawnPoint@@@@
r5apex.exe!0x02842850 .?AV?$CEntityFactory@VCTitanSpawnPoint@@@@
r5apex.exe!0x028434b8 .?AV?$CEntityFactory@VCTitanSpawnPoint@@@@
r5apex.exe!0x028486b0 .?AV?$CEntityFactory@VCTraverseRef@@@@
r5apex.exe!0x02842740 .?AV?$CEntityFactory@VCTriggerAutoCrouch@@@@
r5apex.exe!0x028415e0 .?AV?$CEntityFactory@VCTriggerBrush@@@@
r5apex.exe!0x02843390 .?AV?$CEntityFactory@VCTriggerCamera@@@@
r5apex.exe!0x028422c0 .?AV?$CEntityFactory@VCTriggerCylinder@@@@
r5apex.exe!0x02843170 .?AV?$CEntityFactory@VCTriggerCylinderHeavy@@@@
r5apex.exe!0x028433c8 .?AV?$CEntityFactory@VCTriggerGravity@@@@
r5apex.exe!0x028424b0 .?AV?$CEntityFactory@VCTriggerHurt@@@@
r5apex.exe!0x02842c88 .?AV?$CEntityFactory@VCTriggerImpact@@@@
r5apex.exe!0x02847b88 .?AV?$CEntityFactory@VCTriggerLocation@@@@
r5apex.exe!0x02847a00 .?AV?$CEntityFactory@VCTriggerLocationSP@@@@
r5apex.exe!0x02843408 .?AV?$CEntityFactory@VCTriggerLook@@@@
r5apex.exe!0x028420a0 .?AV?$CEntityFactory@VCTriggerMultiple@@@@
r5apex.exe!0x02842810 .?AV?$CEntityFactory@VCTriggerMultiple@@@@
r5apex.exe!0x02842ba8 .?AV?$CEntityFactory@VCTriggerMultiple@@@@
r5apex.exe!0x02842bb0 .?AV?$CEntityFactory@VCTriggerMultiple@@@@
r5apex.exe!0x02842d08 .?AV?$CEntityFactory@VCTriggerMultiple@@@@
r5apex.exe!0x028430c8 .?AV?$CEntityFactory@VCTriggerMultiple@@@@
r5apex.exe!0x028435b0 .?AV?$CEntityFactory@VCTriggerMultiple@@@@
r5apex.exe!0x02847838 .?AV?$CEntityFactory@VCTriggerMultiple@@@@
r5apex.exe!0x02847aa8 .?AV?$CEntityFactory@VCTriggerMultiple@@@@
r5apex.exe!0x02847c70 .?AV?$CEntityFactory@VCTriggerMultiple@@@@
r5apex.exe!0x02847c78 .?AV?$CEntityFactory@VCTriggerMultiple@@@@
r5apex.exe!0x02847ce8 .?AV?$CEntityFactory@VCTriggerMultiple@@@@
r5apex.exe!0x02847d28 .?AV?$CEntityFactory@VCTriggerMultiple@@@@
r5apex.exe!0x02847db8 .?AV?$CEntityFactory@VCTriggerMultiple@@@@
r5apex.exe!0x02847e60 .?AV?$CEntityFactory@VCTriggerMultiple@@@@
r5apex.exe!0x02843158 .?AV?$CEntityFactory@VCTriggerNoGrapple@@@@
r5apex.exe!0x02847b50 .?AV?$CEntityFactory@VCTriggerNoZipline@@@@
r5apex.exe!0x028434c8 .?AV?$CEntityFactory@VCTriggerOnce@@@@
r5apex.exe!0x02842e90 .?AV?$CEntityFactory@VCTriggerPlayerMovement@@@@
r5apex.exe!0x028434b0 .?AV?$CEntityFactory@VCTriggerPointGravity@@@@
r5apex.exe!0x02843380 .?AV?$CEntityFactory@VCTriggerProximity@@@@
r5apex.exe!0x02847da8 .?AV?$CEntityFactory@VCTriggerPush@@@@
r5apex.exe!0x02843348 .?AV?$CEntityFactory@VCTriggerRemove@@@@
r5apex.exe!0x028402a8 .?AV?$CEntityFactory@VCTriggerSlip@@@@
r5apex.exe!0x02843178 .?AV?$CEntityFactory@VCTriggerSoundscape@@@@
r5apex.exe!0x02847fa0 .?AV?$CEntityFactory@VCTriggerTeleport@@@@
r5apex.exe!0x02842d00 .?AV?$CEntityFactory@VCTriggerViewProxy@@@@
r5apex.exe!0x02842778 .?AV?$CEntityFactory@VCTriggerWind@@@@
r5apex.exe!0x02849410 .?AV?$CEntityFactory@VCTurret@@@@
r5apex.exe!0x028430a8 .?AV?$CEntityFactory@VCVGuiScreen@@@@
r5apex.exe!0x02847830 .?AV?$CEntityFactory@VCVGuiScreen@@@@
r5apex.exe!0x02848a78 .?AV?$CEntityFactory@VCVortexSphere@@@@
r5apex.exe!0x028493d8 .?AV?$CEntityFactory@VCWallrunCurve@@@@
r5apex.exe!0x02842cc0 .?AV?$CEntityFactory@VCWaterLODControl@@@@
r5apex.exe!0x02849688 .?AV?$CEntityFactory@VCWeaponX@@@@
r5apex.exe!0x02849170 .?AV?$CEntityFactory@VCWindowHint@@@@
r5apex.exe!0x02841568 .?AV?$CEntityFactory@VCWindowPane@@@@
r5apex.exe!0x02847da0 .?AV?$CEntityFactory@VCWorld@@@@
r5apex.exe!0x02841c48 .?AV?$CEntityFactory@VCWorldItem@@@@
r5apex.exe!0x02849138 .?AV?$CEntityFactory@VCZipline@@@@
r5apex.exe!0x02848be8 .?AV?$CEntityFactory@VCZiplineEnd@@@@
r5apex.exe!0x02840238 .?AV?$CEntityFactory@VDoorMover@@@@
r5apex.exe!0x0283fec8 .?AV?$CEntityFactory@VScriptMoverLightweight@@@@
r5apex.exe!0x01843918 .?AV?$CPanelFactory@VCMovieDisplayScreen@@UVGuiScreenInitData_t@@@@
r5apex.exe!0x01817818 .?AV?$CPanelFactory@VCVGuiScreenPanel@@UVGuiScreenInitData_t@@@@
r5apex.exe!0x02290468 .?AV?$CParticleOperatorDefinition@VC_INIT_AgeNoise@@@@
r5apex.exe!0x0228f648 .?AV?$CParticleOperatorDefinition@VC_INIT_ChaoticAttractor@@@@
r5apex.exe!0x02290688 .?AV?$CParticleOperatorDefinition@VC_INIT_ColorLitPerParticle@@@@
r5apex.exe!0x02284358 .?AV?$CParticleOperatorDefinition@VC_INIT_CreateAlongPath@@@@
r5apex.exe!0x01efdd88 .?AV?$CParticleOperatorDefinition@VC_INIT_CreateFromParentParticles@@@@
r5apex.exe!0x02294488 .?AV?$CParticleOperatorDefinition@VC_INIT_CreateFromPlaneCache@@@@
r5apex.exe!0x02295748 .?AV?$CParticleOperatorDefinition@VC_INIT_CreateInEpitrochoid@@@@
r5apex.exe!0x0228ff88 .?AV?$CParticleOperatorDefinition@VC_INIT_CreateInHierarchy@@@@
r5apex.exe!0x01efca58 .?AV?$CParticleOperatorDefinition@VC_INIT_CreateOnModel@@@@
r5apex.exe!0x02294508 .?AV?$CParticleOperatorDefinition@VC_INIT_CreateSequentialPath@@@@
r5apex.exe!0x02290248 .?AV?$CParticleOperatorDefinition@VC_INIT_CreateWithinBox@@@@
r5apex.exe!0x0228f348 .?AV?$CParticleOperatorDefinition@VC_INIT_CreateWithinControlPointBox@@@@
r5apex.exe!0x02296328 .?AV?$CParticleOperatorDefinition@VC_INIT_CreateWithinSphere@@@@
r5apex.exe!0x02282f18 .?AV?$CParticleOperatorDefinition@VC_INIT_CreationNoise@@@@
r5apex.exe!0x02295a88 .?AV?$CParticleOperatorDefinition@VC_INIT_DistanceToCPInit@@@@
r5apex.exe!0x02284258 .?AV?$CParticleOperatorDefinition@VC_INIT_InheritFromParentParticles@@@@
r5apex.exe!0x02290128 .?AV?$CParticleOperatorDefinition@VC_INIT_InheritVelocity@@@@
r5apex.exe!0x0228f808 .?AV?$CParticleOperatorDefinition@VC_INIT_InitFromParentKilled@@@@
r5apex.exe!0x022900a8 .?AV?$CParticleOperatorDefinition@VC_INIT_InitialRepulsionVelocity@@@@
r5apex.exe!0x02296448 .?AV?$CParticleOperatorDefinition@VC_INIT_InitialVelocityNoise@@@@
r5apex.exe!0x02282388 .?AV?$CParticleOperatorDefinition@VC_INIT_LifespanFromVelocity@@@@
r5apex.exe!0x022956c8 .?AV?$CParticleOperatorDefinition@VC_INIT_ModelCull@@@@
r5apex.exe!0x0228fc08 .?AV?$CParticleOperatorDefinition@VC_INIT_MoveBetweenPoints@@@@
r5apex.exe!0x0228fd28 .?AV?$CParticleOperatorDefinition@VC_INIT_NormalAlignToCP@@@@
r5apex.exe!0x02296068 .?AV?$CParticleOperatorDefinition@VC_INIT_NormalOffset@@@@
r5apex.exe!0x02283e58 .?AV?$CParticleOperatorDefinition@VC_INIT_OffsetVectorToVector@@@@
r5apex.exe!0x02290368 .?AV?$CParticleOperatorDefinition@VC_INIT_PositionOffset@@@@
r5apex.exe!0x01efc8e8 .?AV?$CParticleOperatorDefinition@VC_INIT_PositionPlaceOnGround@@@@
r5apex.exe!0x022903e8 .?AV?$CParticleOperatorDefinition@VC_INIT_PositionWarp@@@@
r5apex.exe!0x02290568 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomAlpha@@@@
r5apex.exe!0x01f01ae8 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomColor@@@@
r5apex.exe!0x0228f3c8 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomLifeTime@@@@
r5apex.exe!0x01ef6f18 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomRadius@@@@
r5apex.exe!0x01efdea8 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomRotation@@@@
r5apex.exe!0x01ef6d88 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomRotationSpeed@@@@
r5apex.exe!0x0228eba8 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomScalar@@@@
r5apex.exe!0x0228f448 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomSecondSequence@@@@
r5apex.exe!0x022962a8 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomSequence@@@@
r5apex.exe!0x01ef7098 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomTrailLength@@@@
r5apex.exe!0x02296608 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomVector@@@@
r5apex.exe!0x02294bd8 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomVectorComponent@@@@
r5apex.exe!0x02295b08 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomYaw@@@@
r5apex.exe!0x01ef7018 .?AV?$CParticleOperatorDefinition@VC_INIT_RandomYawFlip@@@@
r5apex.exe!0x02282688 .?AV?$CParticleOperatorDefinition@VC_INIT_RemapCPtoScalar@@@@
r5apex.exe!0x01efd278 .?AV?$CParticleOperatorDefinition@VC_INIT_RemapCPtoVector@@@@
r5apex.exe!0x022842d8 .?AV?$CParticleOperatorDefinition@VC_INIT_RemapInitialCPDirectionToRotation@@@@
r5apex.exe!0x01efda58 .?AV?$CParticleOperatorDefinition@VC_INIT_RemapInitialDirectionToCPToVector@@@@
r5apex.exe!0x0228f5c8 .?AV?$CParticleOperatorDefinition@VC_INIT_RemapParticleCountToScalar@@@@
r5apex.exe!0x01efc688 .?AV?$CParticleOperatorDefinition@VC_INIT_RemapScalar@@@@
r5apex.exe!0x02295868 .?AV?$CParticleOperatorDefinition@VC_INIT_RemapScalarToVector@@@@
r5apex.exe!0x01ef6ea8 .?AV?$CParticleOperatorDefinition@VC_INIT_RemapSpeedToScalar@@@@
r5apex.exe!0x02295a08 .?AV?$CParticleOperatorDefinition@VC_INIT_RemapWorldCPtoScreen@@@@
r5apex.exe!0x022960e8 .?AV?$CParticleOperatorDefinition@VC_INIT_RingWave@@@@
r5apex.exe!0x022945a8 .?AV?$CParticleOperatorDefinition@VC_INIT_SequenceFromCP@@@@
r5apex.exe!0x01efe7a8 .?AV?$CParticleOperatorDefinition@VC_INIT_SequenceLifeTime@@@@
r5apex.exe!0x022904e8 .?AV?$CParticleOperatorDefinition@VC_INIT_SetCPPosition@@@@
r5apex.exe!0x0228f228 .?AV?$CParticleOperatorDefinition@VC_INIT_SetHitboxToClosest@@@@
r5apex.exe!0x0228fae8 .?AV?$CParticleOperatorDefinition@VC_INIT_SetHitboxToModel@@@@
r5apex.exe!0x0228f9c8 .?AV?$CParticleOperatorDefinition@VC_INIT_VelocityFromCP@@@@
r5apex.exe!0x02295988 .?AV?$CParticleOperatorDefinition@VC_INIT_VelocityRandom@@@@
r5apex.exe!0x022a1718 .?AV?$CParticleOperatorDefinition@VC_OP_AlphaDecay@@@@
r5apex.exe!0x02297038 .?AV?$CParticleOperatorDefinition@VC_OP_AttractToControlPoint@@@@
r5apex.exe!0x022a1d88 .?AV?$CParticleOperatorDefinition@VC_OP_AxisSpin@@@@
r5apex.exe!0x022988d8 .?AV?$CParticleOperatorDefinition@VC_OP_BasicMovement@@@@
r5apex.exe!0x01ef3e28 .?AV?$CParticleOperatorDefinition@VC_OP_BoxConstraint@@@@
r5apex.exe!0x02297b48 .?AV?$CParticleOperatorDefinition@VC_OP_CPOffsetToPercentageBetweenCPs@@@@
r5apex.exe!0x022a40d8 .?AV?$CParticleOperatorDefinition@VC_OP_ClampScalar@@@@
r5apex.exe!0x0229f218 .?AV?$CParticleOperatorDefinition@VC_OP_ClampVector@@@@
r5apex.exe!0x0229c948 .?AV?$CParticleOperatorDefinition@VC_OP_ColorInterpolate@@@@
r5apex.exe!0x01ef4e68 .?AV?$CParticleOperatorDefinition@VC_OP_ConstrainDistance@@@@
r5apex.exe!0x01ef55a8 .?AV?$CParticleOperatorDefinition@VC_OP_ConstrainDistanceToPath@@@@
r5apex.exe!0x022969c8 .?AV?$CParticleOperatorDefinition@VC_OP_ContinuousEmitter@@@@
r5apex.exe!0x022a1e88 .?AV?$CParticleOperatorDefinition@VC_OP_ControlpointLight@@@@
r5apex.exe!0x02298c18 .?AV?$CParticleOperatorDefinition@VC_OP_Cull@@@@
r5apex.exe!0x022a3e98 .?AV?$CParticleOperatorDefinition@VC_OP_DampenToCP@@@@
r5apex.exe!0x02298508 .?AV?$CParticleOperatorDefinition@VC_OP_Decay@@@@
r5apex.exe!0x022a1818 .?AV?$CParticleOperatorDefinition@VC_OP_DecayMaintainCount@@@@
r5apex.exe!0x022a23a8 .?AV?$CParticleOperatorDefinition@VC_OP_DifferencePreviousParticle@@@@
r5apex.exe!0x0229f5b8 .?AV?$CParticleOperatorDefinition@VC_OP_DistanceBetweenCPs@@@@
r5apex.exe!0x0229ef58 .?AV?$CParticleOperatorDefinition@VC_OP_DistanceBetweenCPsToCP@@@@
r5apex.exe!0x022a2f58 .?AV?$CParticleOperatorDefinition@VC_OP_DistanceCull@@@@
r5apex.exe!0x02296688 .?AV?$CParticleOperatorDefinition@VC_OP_DistanceEmitter@@@@
r5apex.exe!0x0229dca8 .?AV?$CParticleOperatorDefinition@VC_OP_DistanceToCP@@@@
r5apex.exe!0x022986a8 .?AV?$CParticleOperatorDefinition@VC_OP_FadeAndKill@@@@
r5apex.exe!0x022a2288 .?AV?$CParticleOperatorDefinition@VC_OP_FadeAndKillForTracers@@@@
r5apex.exe!0x0229d7f8 .?AV?$CParticleOperatorDefinition@VC_OP_FadeIn@@@@
r5apex.exe!0x0229a908 .?AV?$CParticleOperatorDefinition@VC_OP_FadeInSimple@@@@
r5apex.exe!0x0229f418 .?AV?$CParticleOperatorDefinition@VC_OP_FadeOut@@@@
r5apex.exe!0x0229ff38 .?AV?$CParticleOperatorDefinition@VC_OP_FadeOutSimple@@@@
r5apex.exe!0x02297228 .?AV?$CParticleOperatorDefinition@VC_OP_ForceBasedOnDistanceToPlane@@@@
r5apex.exe!0x022a0d18 .?AV?$CParticleOperatorDefinition@VC_OP_GraphScalar@@@@
r5apex.exe!0x022a2a38 .?AV?$CParticleOperatorDefinition@VC_OP_GraphVector@@@@
r5apex.exe!0x022998b8 .?AV?$CParticleOperatorDefinition@VC_OP_InheritFromParentParticles@@@@
r5apex.exe!0x02296b58 .?AV?$CParticleOperatorDefinition@VC_OP_InstantaneousDistanceEmitter@@@@
r5apex.exe!0x02296708 .?AV?$CParticleOperatorDefinition@VC_OP_InstantaneousEmitter@@@@
r5apex.exe!0x022a2fd8 .?AV?$CParticleOperatorDefinition@VC_OP_InterpolateRadius@@@@
r5apex.exe!0x02298b18 .?AV?$CParticleOperatorDefinition@VC_OP_LagCompensation@@@@
r5apex.exe!0x022982e8 .?AV?$CParticleOperatorDefinition@VC_OP_LerpEndCapScalar@@@@
r5apex.exe!0x0229f6d8 .?AV?$CParticleOperatorDefinition@VC_OP_LerpEndCapVector@@@@
r5apex.exe!0x0229b4a8 .?AV?$CParticleOperatorDefinition@VC_OP_LerpScalar@@@@
r5apex.exe!0x0229f498 .?AV?$CParticleOperatorDefinition@VC_OP_LerpVector@@@@
r5apex.exe!0x0229c188 .?AV?$CParticleOperatorDefinition@VC_OP_LockToBone@@@@
r5apex.exe!0x0229f878 .?AV?$CParticleOperatorDefinition@VC_OP_LockToSavedSequentialPath@@@@
r5apex.exe!0x02296788 .?AV?$CParticleOperatorDefinition@VC_OP_MaintainEmitter@@@@
r5apex.exe!0x022a3798 .?AV?$CParticleOperatorDefinition@VC_OP_MaintainSequentialPath@@@@
r5apex.exe!0x0229f398 .?AV?$CParticleOperatorDefinition@VC_OP_MaxVelocity@@@@
r5apex.exe!0x0229c348 .?AV?$CParticleOperatorDefinition@VC_OP_ModelCull@@@@
r5apex.exe!0x022a2208 .?AV?$CParticleOperatorDefinition@VC_OP_MoveToHitbox@@@@
r5apex.exe!0x02298b98 .?AV?$CParticleOperatorDefinition@VC_OP_MovementMaintainOffset@@@@
r5apex.exe!0x022a24c8 .?AV?$CParticleOperatorDefinition@VC_OP_MovementPlaceOnGround@@@@
r5apex.exe!0x022a1b48 .?AV?$CParticleOperatorDefinition@VC_OP_MovementRotateParticleAroundAxis@@@@
r5apex.exe!0x0229cfc8 .?AV?$CParticleOperatorDefinition@VC_OP_Noise@@@@
r5apex.exe!0x022968a8 .?AV?$CParticleOperatorDefinition@VC_OP_NoiseEmitter@@@@
r5apex.exe!0x022a1798 .?AV?$CParticleOperatorDefinition@VC_OP_NormalLock@@@@
r5apex.exe!0x0229df78 .?AV?$CParticleOperatorDefinition@VC_OP_NormalizeVector@@@@
r5apex.exe!0x022a2428 .?AV?$CParticleOperatorDefinition@VC_OP_Orient2DRelToCP@@@@
r5apex.exe!0x022a2728 .?AV?$CParticleOperatorDefinition@VC_OP_OrientTo2dDirection@@@@
r5apex.exe!0x0229fbf8 .?AV?$CParticleOperatorDefinition@VC_OP_OrientTowardPlayer@@@@
r5apex.exe!0x02299798 .?AV?$CParticleOperatorDefinition@VC_OP_OscillateScalar@@@@
r5apex.exe!0x022993b8 .?AV?$CParticleOperatorDefinition@VC_OP_OscillateScalarSimple@@@@
r5apex.exe!0x022a2b38 .?AV?$CParticleOperatorDefinition@VC_OP_OscillateVector@@@@
r5apex.exe!0x022987e8 .?AV?$CParticleOperatorDefinition@VC_OP_OscillateVectorSimple@@@@
r5apex.exe!0x02297928 .?AV?$CParticleOperatorDefinition@VC_OP_ParentVortices@@@@
r5apex.exe!0x022a0718 .?AV?$CParticleOperatorDefinition@VC_OP_PercentageBetweenCPs@@@@
r5apex.exe!0x0229a618 .?AV?$CParticleOperatorDefinition@VC_OP_PercentageBetweenCPsVector@@@@
r5apex.exe!0x01ef4368 .?AV?$CParticleOperatorDefinition@VC_OP_PlanarConstraint@@@@
r5apex.exe!0x0229ffb8 .?AV?$CParticleOperatorDefinition@VC_OP_PlaneCull@@@@
r5apex.exe!0x0229f298 .?AV?$CParticleOperatorDefinition@VC_OP_PositionBetweenCPs@@@@
r5apex.exe!0x02298368 .?AV?$CParticleOperatorDefinition@VC_OP_PositionLock@@@@
r5apex.exe!0x022a2048 .?AV?$CParticleOperatorDefinition@VC_OP_ProjectileArc@@@@
r5apex.exe!0x02299178 .?AV?$CParticleOperatorDefinition@VC_OP_RadiusDecay@@@@
r5apex.exe!0x022a1508 .?AV?$CParticleOperatorDefinition@VC_OP_RampScalarLinear@@@@
r5apex.exe!0x022a2d98 .?AV?$CParticleOperatorDefinition@VC_OP_RampScalarLinearSimple@@@@
r5apex.exe!0x02298628 .?AV?$CParticleOperatorDefinition@VC_OP_RampScalarSpline@@@@
r5apex.exe!0x0229b708 .?AV?$CParticleOperatorDefinition@VC_OP_RampScalarSplineSimple@@@@
r5apex.exe!0x02297ac8 .?AV?$CParticleOperatorDefinition@VC_OP_RandomForce@@@@
r5apex.exe!0x022a2988 .?AV?$CParticleOperatorDefinition@VC_OP_RemapAverageScalarValuetoCP@@@@
r5apex.exe!0x02297d88 .?AV?$CParticleOperatorDefinition@VC_OP_RemapBoundingVolumetoCP@@@@
r5apex.exe!0x0229b8f8 .?AV?$CParticleOperatorDefinition@VC_OP_RemapCPVelocityToVector@@@@
r5apex.exe!0x022a3678 .?AV?$CParticleOperatorDefinition@VC_OP_RemapCPtoScalar@@@@
r5apex.exe!0x0229efd8 .?AV?$CParticleOperatorDefinition@VC_OP_RemapCPtoVector@@@@
r5apex.exe!0x0229f318 .?AV?$CParticleOperatorDefinition@VC_OP_RemapControlPointDirectionToVector@@@@
r5apex.exe!0x0229fd98 .?AV?$CParticleOperatorDefinition@VC_OP_RemapDirectionToCPToVector@@@@
r5apex.exe!0x02299568 .?AV?$CParticleOperatorDefinition@VC_OP_RemapDotProductToScalar@@@@
r5apex.exe!0x022a0f58 .?AV?$CParticleOperatorDefinition@VC_OP_RemapModelVolumetoCP@@@@
r5apex.exe!0x022a00d8 .?AV?$CParticleOperatorDefinition@VC_OP_RemapScalar@@@@
r5apex.exe!0x0229dd28 .?AV?$CParticleOperatorDefinition@VC_OP_RemapSpeed@@@@
r5apex.exe!0x0229f758 .?AV?$CParticleOperatorDefinition@VC_OP_RemapSpeedtoCP@@@@
r5apex.exe!0x022983e8 .?AV?$CParticleOperatorDefinition@VC_OP_RemapVelocityToVector@@@@
r5apex.exe!0x0229aa28 .?AV?$CParticleOperatorDefinition@VC_OP_RemapWorldCPToScreen@@@@
r5apex.exe!0x022a4158 .?AV?$CParticleOperatorDefinition@VC_OP_RenderDecal@@@@
r5apex.exe!0x022a4c78 .?AV?$CParticleOperatorDefinition@VC_OP_RenderLightSource@@@@
r5apex.exe!0x022a44d8 .?AV?$CParticleOperatorDefinition@VC_OP_RenderModels@@@@
r5apex.exe!0x012f3160 .?AV?$CParticleOperatorDefinition@VC_OP_RenderPoints@@@@
r5apex.exe!0x022a4f38 .?AV?$CParticleOperatorDefinition@VC_OP_RenderRope@@@@
r5apex.exe!0x022a4718 .?AV?$CParticleOperatorDefinition@VC_OP_RenderScreenVelocityRotate@@@@
r5apex.exe!0x022a4458 .?AV?$CParticleOperatorDefinition@VC_OP_RenderScripts@@@@
r5apex.exe!0x022a4e18 .?AV?$CParticleOperatorDefinition@VC_OP_RenderSprites@@@@
r5apex.exe!0x022a4558 .?AV?$CParticleOperatorDefinition@VC_OP_RenderSpritesTrail@@@@
r5apex.exe!0x0229feb8 .?AV?$CParticleOperatorDefinition@VC_OP_RestartAfterDuration@@@@
r5apex.exe!0x02299298 .?AV?$CParticleOperatorDefinition@VC_OP_RotateVector@@@@
r5apex.exe!0x0229f058 .?AV?$CParticleOperatorDefinition@VC_OP_SetCPOrientationToDirection@@@@
r5apex.exe!0x0229a738 .?AV?$CParticleOperatorDefinition@VC_OP_SetChildControlPoints@@@@
r5apex.exe!0x0229ca68 .?AV?$CParticleOperatorDefinition@VC_OP_SetControlPointPositions@@@@
r5apex.exe!0x022a38b8 .?AV?$CParticleOperatorDefinition@VC_OP_SetControlPointRotation@@@@
r5apex.exe!0x022a1488 .?AV?$CParticleOperatorDefinition@VC_OP_SetControlPointToCenter@@@@
r5apex.exe!0x02299438 .?AV?$CParticleOperatorDefinition@VC_OP_SetControlPointToImpactPoint@@@@
r5apex.exe!0x0229b7d8 .?AV?$CParticleOperatorDefinition@VC_OP_SetControlPointToPlayer@@@@
r5apex.exe!0x0229dda8 .?AV?$CParticleOperatorDefinition@VC_OP_SetControlPointsToParticle@@@@
r5apex.exe!0x0229a888 .?AV?$CParticleOperatorDefinition@VC_OP_SetPerChildControlPoint@@@@
r5apex.exe!0x022a1e08 .?AV?$CParticleOperatorDefinition@VC_OP_SoundMeterScalar@@@@
r5apex.exe!0x02297d08 .?AV?$CParticleOperatorDefinition@VC_OP_Spin@@@@
r5apex.exe!0x0229b758 .?AV?$CParticleOperatorDefinition@VC_OP_SpinUpdate@@@@
r5apex.exe!0x0229fc78 .?AV?$CParticleOperatorDefinition@VC_OP_SpinYaw@@@@
r5apex.exe!0x022a3e18 .?AV?$CParticleOperatorDefinition@VC_OP_StopAfterCPDuration@@@@
r5apex.exe!0x02297328 .?AV?$CParticleOperatorDefinition@VC_OP_TimeVaryingForce@@@@
r5apex.exe!0x022972a8 .?AV?$CParticleOperatorDefinition@VC_OP_TurbulenceForce@@@@
r5apex.exe!0x022979a8 .?AV?$CParticleOperatorDefinition@VC_OP_TwistAroundAxis@@@@
r5apex.exe!0x02299e18 .?AV?$CParticleOperatorDefinition@VC_OP_VectorNoise@@@@
r5apex.exe!0x0229fa38 .?AV?$CParticleOperatorDefinition@VC_OP_VelocityDecay@@@@
r5apex.exe!0x022a1bc8 .?AV?$CParticleOperatorDefinition@VC_OP_VelocityMatchingForce@@@@
r5apex.exe!0x01ef0f08 .?AV?$CParticleOperatorDefinition@VC_OP_WorldCollideConstraint@@@@
r5apex.exe!0x01ef56c8 .?AV?$CParticleOperatorDefinition@VC_OP_WorldTraceConstraint@@@@
r5apex.exe!0x2930bc30 .?AV?$CUtlVectorDataOps@V?$CUtlVector@HV?$CUtlMemory@H_J@@H@@$04@@
r5apex.exe!0x292f8c98 .?AV?$CUtlVectorDataOps@V?$CUtlVector@PEAVCBaseEntity@@V?$CUtlMemory@PEAVCBaseEntity@@_J@@H@@$0M@@@
r5apex.exe!0x292f1ad0 .?AV?$CUtlVectorDataOps@V?$CUtlVector@PEAVCPlayer@@V?$CUtlMemory@PEAVCPlayer@@_J@@H@@$0M@@@
r5apex.exe!0x292f3fe0 .?AV?$CUtlVectorDataOps@V?$CUtlVector@PEAVCTeamSpawnPoint@@V?$CUtlMemory@PEAVCTeamSpawnPoint@@_J@@H@@$0M@@@
r5apex.exe!0x2930ba80 .?AV?$CUtlVectorDataOps@V?$CUtlVector@UAIChannelScheduleState_t@@V?$CUtlMemory@UAIChannelScheduleState_t@@_J@@H@@$09@@
r5apex.exe!0x292e4438 .?AV?$CUtlVectorDataOps@V?$CUtlVector@UAISquadEnemyInfo_t@@V?$CUtlMemory@UAISquadEnemyInfo_t@@_J@@H@@$09@@
r5apex.exe!0x2930b7e0 .?AV?$CUtlVectorDataOps@V?$CUtlVector@UUnreachableEnt_t@@V?$CUtlMemory@UUnreachableEnt_t@@_J@@H@@$09@@
r5apex.exe!0x292e61e8 .?AV?$CUtlVectorDataOps@V?$CUtlVector@Uphysfollower_t@@V?$CUtlMemory@Uphysfollower_t@@_J@@H@@$09@@
r5apex.exe!0x292f1558 .?AV?$CUtlVectorDataOps@V?$CUtlVector@Utemplate_t@@V?$CUtlMemory@Utemplate_t@@_J@@H@@$09@@
r5apex.exe!0x2889a218 .?AV?$CUtlVectorDataOps@V?$CUtlVector@V?$CHandle@VCBaseEntity@@@@V?$CUtlMemory@V?$CHandle@VCBaseEntity@@@@_J@@H@@$0N@@@
r5apex.exe!0x292f0b28 .?AV?$CUtlVectorDataOps@V?$CUtlVector@V?$CHandle@VCPlayer@@@@V?$CUtlMemory@V?$CHandle@VCPlayer@@@@_J@@H@@$0N@@@
r5apex.exe!0x2930fe68 .?AV?$CUtlVectorDataOps@V?$CUtlVector@VVector@@V?$CUtlMemory@VVector@@_J@@H@@$02@@
r5apex.exe!0x2930b7d8 .?AV?$CUtlVectorDataOps@V?$CUtlVectorFixed@UAIDebouncedSyncedMelee@@$04_J@@$09@@
r5apex.exe!0x292e27a0 .?AV?$CUtlVectorDataOps@V?$CUtlVectorFixed@UWeaponAnimEvent@@$0EA@_J@@$09@@
r5apex.exe!0x292e5128 .?AV?$CUtlVectorDataOps@V?$CUtlVectorFixed@V?$CHandle@VCBaseEntity@@@@$02_J@@$0N@@@
r5apex.exe!0x292eb128 .?AV?$CUtlVectorDataOps@V?$CUtlVectorFixed@V?$CHandle@VCBaseEntity@@@@$0BA@_J@@$04@@
r5apex.exe!0x2930ecc8 .?AV?$CUtlVectorDataOps@V?$CUtlVectorFixedGrowable@Tfloat3@@$0BA@_J@@$02@@
r5apex.exe!0x2930ba70 .?AV?$CUtlVectorDataOps@VCAI_InterestTarget@@$09@@
r5apex.exe!0x292dcc20 .?AV?$CVarBitVecSaveRestoreOps@V?$CBitVec@$07@@@@
r5apex.exe!0x01ef3bf0 .?AV?$C_EntityClassList@VC_PointCamera@@@@
r5apex.exe!0x0229de60 .?AV?$C_EntityClassList@VC_TriggerPlayerMovement@@@@
r5apex.exe!0x02882370 .?AV?$_Ref_count_obj_alloc@V__ExceptionPtr@@U?$_StaticAllocator@H@@@std@@
r5apex.exe!0x023702a8 .?AVActiveActModifiersDataOps@@
r5apex.exe!0x02373ec8 .?AVActiveActModifiersSaveRestoreDataOps@@
r5apex.exe!0x0236eeb8 .?AVCAI_EnemiesListSaveRestoreOps@@
r5apex.exe!0x0235cbe8 .?AVCAI_SaveRestoreBlockHandler@@
r5apex.exe!0x02370228 .?AVCAI_SystemHook@@
r5apex.exe!0x022e95d8 .?AVCActivityDataOps@@
r5apex.exe!0x01efc9a0 .?AVCAimAssistTargets@@
r5apex.exe!0x01efc9a8 .?AVCAimAssistTargets@@
r5apex.exe!0x0171b0e0 .?AVCAvi@@
r5apex.exe!0x012f2510 .?AVCBSPPack@@
r5apex.exe!0x017126a8 .?AVCBaseClientRenderTargets@@
r5apex.exe!0x0236ee18 .?AVCBaseEntityScriptInstanceHelper@@
r5apex.exe!0x1f8984c0 .?AVCBik@@
r5apex.exe!0x0170bf88 .?AVCBoolProperty@@
r5apex.exe!0x02772000 .?AVCBreakModelsPrecached@@
r5apex.exe!0x02370b68 .?AVCBullseyeList@@
r5apex.exe!0x02369ab8 .?AVCCSMLightManager@@
r5apex.exe!0x02284690 .?AVCCascadeLightManager@@
r5apex.exe!0x022b1160 .?AVCCenterPrint@@
r5apex.exe!0x0277c9e0 .?AVCCheckClient@@
r5apex.exe!0x02833e70 .?AVCClassMap@@
r5apex.exe!0x022a9950 .?AVCClientCollisionEvent@@
r5apex.exe!0x01711958 .?AVCClientDLLSharedAppSystems@@
r5apex.exe!0x01f01b00 .?AVCClientEntityList@@
r5apex.exe!0x02181b58 .?AVCClientEntityList@@
r5apex.exe!0x2342ee40 .?AVCClientLeafSystem@@
r5apex.exe!0x23402540 .?AVCClientShadowMgr@@
r5apex.exe!0x017069d8 .?AVCClientSound@@
r5apex.exe!0x017de250 .?AVCClientState@@
r5apex.exe!0x017de258 .?AVCClientState@@
r5apex.exe!0x017de260 .?AVCClientState@@
r5apex.exe!0x017de268 .?AVCClientState@@
r5apex.exe!0x0228ef80 .?AVCClientThinkList@@
r5apex.exe!0x017023c0 .?AVCCmdLibFileLoggingListener@@
r5apex.exe!0x01702380 .?AVCCmdLibStandardLoggingListener@@
r5apex.exe!0x01edf790 .?AVCColorCorrectionMgr@@
r5apex.exe!0x0184dd60 .?AVCColorCorrectionSystem@@
r5apex.exe!0x023a7598 .?AVCColorCorrectionSystem_Server@@
r5apex.exe!0x0170bcc8 .?AVCColorProperty@@
r5apex.exe!0x01ed4c50 .?AVCCommandLine@@
r5apex.exe!0x22706330 .?AVCCountedStringPool@@
r5apex.exe!0x22706380 .?AVCCountedStringPool@@
r5apex.exe!0x01edc300 .?AVCCvar@@
r5apex.exe!0x01705860 .?AVCCvarQuery@@
r5apex.exe!0x017040e0 .?AVCDataCache@@
r5apex.exe!0x02356e10 .?AVCDataObjectAccessSystem@@
r5apex.exe!0x022a6260 .?AVCDebugOverlayPanel@@
r5apex.exe!0x01707ab8 .?AVCDebugTextureInfoDX11@@
r5apex.exe!0x0170b538 .?AVCDefaultAccessor@@
r5apex.exe!0x0170d598 .?AVCDefaultCvarQuery@@
r5apex.exe!0x02370318 .?AVCDefaultParticleSystemQuery@@
r5apex.exe!0x022a6f38 .?AVCDirtySpatialPartitionEntityList@@
r5apex.exe!0x01ef9ff0 .?AVCEffectsList@@
r5apex.exe!0x01ed79b0 .?AVCEmptyConVar@@
r5apex.exe!0x01ed79f0 .?AVCEmptyConVar@@
r5apex.exe!0x022f5900 .?AVCEmptyGameUIConVar@@
r5apex.exe!0x022f5940 .?AVCEmptyGameUIConVar@@
r5apex.exe!0x01715d40 .?AVCEngine@@
r5apex.exe!0x018483d0 .?AVCEngineAPI@@
r5apex.exe!0x01704828 .?AVCEngineClient@@
r5apex.exe!0x017076e8 .?AVCEngineConsoleLoggingListener@@
r5apex.exe!0x0b2eb198 .?AVCEngineRecipientFilter@@
r5apex.exe!0x01704248 .?AVCEngineTraceClient@@
r5apex.exe!0x017025a8 .?AVCEngineTraceClientDecals@@
r5apex.exe!0x01704a78 .?AVCEngineTraceServer@@
r5apex.exe!0x01705dd8 .?AVCEngineUniformRandomStream@@
r5apex.exe!0x017154f0 .?AVCEngineVGui@@
r5apex.exe!0x0236f000 .?AVCEntFireAutoCompletionFunctor@@
r5apex.exe!0x0236f008 .?AVCEntFireAutoCompletionFunctor@@
r5apex.exe!0x02843630 .?AVCEntityFactoryDictionary@@
r5apex.exe!0x02369cc8 .?AVCEntityListSystem@@
r5apex.exe!0x01813ad0 .?AVCEntityReadInfo@@
r5apex.exe!0x023574e0 .?AVCEntitySaveRestoreBlockHandler@@
r5apex.exe!0x023574e8 .?AVCEntitySaveUtils@@
r5apex.exe!0x0236a228 .?AVCEntityTouchManager@@
r5apex.exe!0x01edc490 .?AVCEventSystem@@
r5apex.exe!0x02356f28 .?AVCEventsSaveDataOps@@
r5apex.exe!0x022a2a18 .?AVCExampleEffect@@
r5apex.exe!0x022a7a10 .?AVCFPS@@
r5apex.exe!0x22706140 .?AVCFileSystem_Stdio@@
r5apex.exe!0x22706148 .?AVCFileSystem_Stdio@@
r5apex.exe!0x0170c5d8 .?AVCFloatProperty@@
r5apex.exe!0x023e8e08 .?AVCFogSystem@@
r5apex.exe!0x01713c88 .?AVCGameClientExports@@
r5apex.exe!0x0236b888 .?AVCGameDLL_ConVarAccessor@@
r5apex.exe!0x02334270 .?AVCGameMovement@@
r5apex.exe!0x02327390 .?AVCGameRules@@
r5apex.exe!0x24863380 .?AVCGameStringPool@@
r5apex.exe!0x022aed08 .?AVCGameTimescale@@
r5apex.exe!0x022f5760 .?AVCGameUI@@
r5apex.exe!0x01707438 .?AVCGameUIFuncs@@
r5apex.exe!0x023eb3b0 .?AVCGlobalEntityList@@
r5apex.exe!0x0170c6e8 .?AVCHFontProperty@@
r5apex.exe!0x01ede3a0 .?AVCHLClient@@
r5apex.exe!0x018436c8 .?AVCHudTextMessage@@
r5apex.exe!0x01818f38 .?AVCHudTextureHandleProperty@@
r5apex.exe!0x0235cdd8 .?AVCIKSaveRestoreOps@@
r5apex.exe!0x01704390 .?AVCIVDebugOverlay@@
r5apex.exe!0x01704398 .?AVCIVDebugOverlay@@
r5apex.exe!0x232ddf20 .?AVCIVPMaterialManager@@
r5apex.exe!0x02370c08 .?AVCInfoPlacementManager@@
r5apex.exe!0x0229e0d0 .?AVCInput@@
r5apex.exe!0x0184c0b0 .?AVCInputStackSystem@@
r5apex.exe!0x0184c180 .?AVCInputSystem@@
r5apex.exe!0x232c4c20 .?AVCInputWin32@@
r5apex.exe!0x0170c068 .?AVCIntProperty@@
r5apex.exe!0x01efe9c0 .?AVCKeyBindingListenerMgr@@
r5apex.exe!0x01eddc80 .?AVCKeyValuesSystem@@
r5apex.exe!0x0170b348 .?AVCLauncherLoggingListener@@
r5apex.exe!0x0170b4c8 .?AVCListOps@TSListTests@@
r5apex.exe!0x022aa7b0 .?AVCLoadingDisc@@
r5apex.exe!0x0184dbc0 .?AVCLocalize@@
r5apex.exe!0x01702470 .?AVCMDLCache@@
r5apex.exe!0x22708e18 .?AVCMatQueuedRenderContext@@
r5apex.exe!0x22708e20 .?AVCMatQueuedRenderContext@@
r5apex.exe!0x22708fe8 .?AVCMatRenderContext@@
r5apex.exe!0x22708ff0 .?AVCMatRenderContext@@
r5apex.exe!0x232cdac0 .?AVCMatSystemSurface@@
r5apex.exe!0x232cdac8 .?AVCMatSystemSurface@@
r5apex.exe!0x232cdad0 .?AVCMatSystemSurface@@
r5apex.exe!0x02834910 .?AVCMaterialProxyDict@@
r5apex.exe!0x017054f8 .?AVCMaterialProxyFactory@@
r5apex.exe!0x22708980 .?AVCMaterialSystem@@
r5apex.exe!0x22708988 .?AVCMaterialSystem@@
r5apex.exe!0x022a77f0 .?AVCMessageChars@@
r5apex.exe!0x012f2e28 .?AVCMessageListener@vgui@@
r5apex.exe!0x0236ea88 .?AVCModInventoryDataOps@@
r5apex.exe!0x01705568 .?AVCModelInfoClient@@
r5apex.exe!0x01705678 .?AVCModelInfoServer@@
r5apex.exe!0x0170f4a0 .?AVCModelLoader@@
r5apex.exe!0x02370b20 .?AVCModelPrecacheSystem@@
r5apex.exe!0x017148f0 .?AVCModelRender@@
r5apex.exe!0x022b10a0 .?AVCModelRenderSystem@@
r5apex.exe!0x022b10b8 .?AVCModelRenderSystem@@
r5apex.exe!0x022ae020 .?AVCMoveHelperClient@@
r5apex.exe!0x023a7690 .?AVCMoveHelperServer@@
r5apex.exe!0x017053d8 .?AVCNetworkStringTableContainer@@
r5apex.exe!0x01712d70 .?AVCNetworkStringTableContainer@@
r5apex.exe!0x02395b60 .?AVCNotifyList@@
r5apex.exe!0x02395b68 .?AVCNotifyList@@
r5apex.exe!0x028351b0 .?AVCPanelMetaClassMgrImp@@
r5apex.exe!0x02835750 .?AVCParticleMgr@@
r5apex.exe!0x022af8d8 .?AVCParticleSystemQuery@@
r5apex.exe!0x0235c490 .?AVCPhysObjSaveRestoreOps@@
r5apex.exe!0x0235c4a0 .?AVCPhysObjSaveRestoreOps@@
r5apex.exe!0x0235c4b0 .?AVCPhysObjSaveRestoreOps@@
r5apex.exe!0x0235c4c0 .?AVCPhysSaveRestoreBlockHandler@@
r5apex.exe!0x0235c4c8 .?AVCPhysSaveRestoreBlockHandler@@
r5apex.exe!0x0235c4d0 .?AVCPhysSaveRestoreBlockHandler@@
r5apex.exe!0x0170ceb8 .?AVCPhysicsCollision@@
r5apex.exe!0x02396a00 .?AVCPhysicsHook@@
r5apex.exe!0x01edbd70 .?AVCPhysicsInterface@@
r5apex.exe!0x0236e2f8 .?AVCPhysicsPlayerCallback@@
r5apex.exe!0x02396950 .?AVCPhysicsPushedEntities@@
r5apex.exe!0x232dde70 .?AVCPhysicsSurfaceProps@@
r5apex.exe!0x01843878 .?AVCPhysicsSystem@@
r5apex.exe!0x022b59f0 .?AVCPickupList@@
r5apex.exe!0x017130c8 .?AVCPixelVisibilitySystem@@
r5apex.exe!0x02370428 .?AVCPlayerMove@@
r5apex.exe!0x0236e818 .?AVCPointTemplatePrecacher@@
r5apex.exe!0x01ed43b0 .?AVCPolyhedron_TempMemory@@
r5apex.exe!0x022f3a50 .?AVCPoseDebuggerImpl@@
r5apex.exe!0x02772c30 .?AVCPostProcessSystem@@
r5apex.exe!0x02351338 .?AVCPrecacheHandler@@
r5apex.exe!0x027768b0 .?AVCPrecacheOtherList@@
r5apex.exe!0x02351ae8 .?AVCPrecacheRegister@@
r5apex.exe!0x01706388 .?AVCPrecacheSystem@@
r5apex.exe!0x022f50d0 .?AVCPrediction@@
r5apex.exe!0x0236b308 .?AVCPrefDataOps@@
r5apex.exe!0x01edde10 .?AVCProcessUtils@@
r5apex.exe!0x02354d90 .?AVCPropData@@
r5apex.exe!0x023559a8 .?AVCPropSurvivalList@@
r5apex.exe!0x0170baa8 .?AVCProportionalFloatProperty@@
r5apex.exe!0x0170c908 .?AVCProportionalIntProperty@@
r5apex.exe!0x0170c9e8 .?AVCProportionalXPosProperty@@
r5apex.exe!0x0170bb18 .?AVCProportionalYPosProperty@@
r5apex.exe!0x0170b458 .?AVCQueueOps@TSListTests@@
r5apex.exe!0x017127f0 .?AVCQueuedPacketSender@@
r5apex.exe!0x022f3510 .?AVCRagdollLRURetirement@@
r5apex.exe!0x01706e88 .?AVCRegistry@@
r5apex.exe!0x01ed45b0 .?AVCResListGenerator@@
r5apex.exe!0x018169d8 .?AVCResourcePrecacher@ErrorPrecache@@
r5apex.exe!0x017dda18 .?AVCResourcePrecacher@ExplodeImpactPrecache@@
r5apex.exe!0x01813b18 .?AVCResourcePrecacher@FX_SplashPrecache@@
r5apex.exe!0x01813a18 .?AVCResourcePrecacher@FX_WaterRipplePrecache@@
r5apex.exe!0x017ddd78 .?AVCResourcePrecacher@GameMovementImpactEventPrecache@@
r5apex.exe!0x017ddfa8 .?AVCResourcePrecacher@ImpactPrecache@@
r5apex.exe!0x01812f48 .?AVCResourcePrecacher@MissileAirBurstPrecache@@
r5apex.exe!0x01812ce8 .?AVCResourcePrecacher@MissileImpactPrecache@@
r5apex.exe!0x017158c8 .?AVCResourcePrecacher@ParticleCreatePrecache@@
r5apex.exe!0x017151b8 .?AVCResourcePrecacher@ParticleEffectPrecache@@
r5apex.exe!0x01715668 .?AVCResourcePrecacher@ParticleEffectStopPrecache@@
r5apex.exe!0x01812f98 .?AVCResourcePrecacher@ParticleTracerPrecache@@
r5apex.exe!0x02355f58 .?AVCResourcePrecacher@PhysFrictionEffectPrecache@@
r5apex.exe!0x017161e8 .?AVCResourcePrecacher@PlayParticlesBreakEffectPrecache@@
r5apex.exe!0x01eee3a8 .?AVCResourcePrecacher@PlayWeaponParticleEffectPrecache@@
r5apex.exe!0x01714a38 .?AVCResourcePrecacher@PrecacheEffectBuildPrecache@@
r5apex.exe!0x01eedbe8 .?AVCResourcePrecacher@PrecacheEffectCrossbowPrecache@@
r5apex.exe!0x017ddbd8 .?AVCResourcePrecacher@PrecacheEffectGlassShatterPrecache@@
r5apex.exe!0x017ddc58 .?AVCResourcePrecacher@PrecacheEffectVGuiScreenPrecache@@
r5apex.exe!0x01817098 .?AVCResourcePrecacher@PrecacheLocatorTargetPrecache@@
r5apex.exe!0x017dddf8 .?AVCResourcePrecacher@RagdollImpactPrecache@@
r5apex.exe!0x017dd7e8 .?AVCResourcePrecacher@ShakeRopesPrecache@@
r5apex.exe!0x01814118 .?AVCResourcePrecacher@SplashImpactPrecache@@
r5apex.exe!0x01eee4c8 .?AVCResourcePrecacher@StopWeaponParticleEffectPrecache@@
r5apex.exe!0x02394408 .?AVCResourcePrecacher@WeaponResourcesPrecache@@
r5apex.exe!0x02370ca8 .?AVCResourcePrecacher@grapple_hookPrecache@@
r5apex.exe!0x0236e8b8 .?AVCResourcePrecacher@playerPrecache@@
r5apex.exe!0x0236e938 .?AVCResourcePrecacher@vgui_screenPrecache@@
r5apex.exe!0x01815278 .?AVCResourcePrecacher@waterripplePrecache@@
r5apex.exe!0x01813eb8 .?AVCResourcePrecacher@watersplashPrecache@@
r5apex.exe!0x017dbe48 .?AVCRopeInitializer@@
r5apex.exe!0x022971f8 .?AVCRopeManager@@
r5apex.exe!0x01ed86c8 .?AVCRunGameEngine@@
r5apex.exe!0x0235df90 .?AVCSaveRestoreBlockSet@@
r5apex.exe!0x01705788 .?AVCSaveRestoreFileSystemPassthrough@@
r5apex.exe!0x01ed82e0 .?AVCSchemeManager@@
r5apex.exe!0x01843118 .?AVCScreenSpaceEffectManager@@
r5apex.exe!0x01702388 .?AVCScriptLib@@
r5apex.exe!0x1f89d400 .?AVCServer@@
r5apex.exe!0x023ea3e0 .?AVCServerCollisionEvent@@
r5apex.exe!0x0236c608 .?AVCServerDLLSharedAppSystems@@
r5apex.exe!0x0236a1b8 .?AVCServerGameClients@@
r5apex.exe!0x02371c60 .?AVCServerGameDLL@@
r5apex.exe!0x0236c598 .?AVCServerGameEnts@@
r5apex.exe!0x023e9210 .?AVCServerRandomStream@@
r5apex.exe!0x01706418 .?AVCServerSound@@
r5apex.exe!0x01707c48 .?AVCShader@Basic@@
r5apex.exe!0x01707d68 .?AVCShader@BasicForceWireframe@Basic@@
r5apex.exe!0x01707e88 .?AVCShader@Bik@@
r5apex.exe!0x01707fa8 .?AVCShader@Black@@
r5apex.exe!0x01ed19a0 .?AVCShader@BlurFilter@@
r5apex.exe!0x01708028 .?AVCShader@BoxFilterCompute@@
r5apex.exe!0x017080a8 .?AVCShader@DebugDrawEnvmapMask@@
r5apex.exe!0x01708128 .?AVCShader@DecalModulate@@
r5apex.exe!0x017081a8 .?AVCShader@DepthWrite@@
r5apex.exe!0x01708368 .?AVCShader@DoFBlurFilterCompute@@
r5apex.exe!0x01708488 .?AVCShader@Downsample4x4@@
r5apex.exe!0x01708588 .?AVCShader@Downsample@@
r5apex.exe!0x01708508 .?AVCShader@Downsample_bloom@@
r5apex.exe!0x01708888 .?AVCShader@Edge@@
r5apex.exe!0x01708908 .?AVCShader@Engine_Post@@
r5apex.exe!0x01708b68 .?AVCShader@ExposureAdaptation@@
r5apex.exe!0x017092c8 .?AVCShader@FrameColorCompute@@
r5apex.exe!0x017093e8 .?AVCShader@Logluminance@@
r5apex.exe!0x01709468 .?AVCShader@Modulate@@
r5apex.exe!0x017096b8 .?AVCShader@Occlusion@@
r5apex.exe!0x01709878 .?AVCShader@Refract@@
r5apex.exe!0x01709ab8 .?AVCShader@Sky@@
r5apex.exe!0x01709c78 .?AVCShader@Sprite@@
r5apex.exe!0x0170a1f8 .?AVCShader@TSAA@@
r5apex.exe!0x0170a458 .?AVCShader@UnlitTwoTexture@@
r5apex.exe!0x0170a9a8 .?AVCShader@VisQuery@@
r5apex.exe!0x0170aa28 .?AVCShader@Water@@
r5apex.exe!0x0170ab48 .?AVCShader@WriteZ@@
r5apex.exe!0x017098f8 .?AVCShader@screenspace_general@@
r5apex.exe!0x012f2a28 .?AVCShaderLibConVarAccessor@@
r5apex.exe!0x01953f40 .?AVCShaderSystem@@
r5apex.exe!0x01953f48 .?AVCShaderSystem@@
r5apex.exe!0x023743e0 .?AVCSimThinkManager@@
r5apex.exe!0x01701d78 .?AVCSimpleLoggingListener@@
r5apex.exe!0x29246328 .?AVCSimpleLoggingListener@@
r5apex.exe!0x01701d88 .?AVCSimpleWindowsLoggingListener@@
r5apex.exe!0x02355fa8 .?AVCSolidSetDefaults@@
r5apex.exe!0x027832d0 .?AVCSoundscapeSystem@@
r5apex.exe!0x017de240 .?AVCSplitScreen@@
r5apex.exe!0x01719070 .?AVCStaticPropMgr@@
r5apex.exe!0x01719078 .?AVCStaticPropMgr@@
r5apex.exe!0x26fe6350 .?AVCStdMemAlloc@@
r5apex.exe!0x0170bff8 .?AVCStringProperty@@
r5apex.exe!0x023a8df0 .?AVCStringTableSaveRestoreOps@@
r5apex.exe!0x01ed4a70 .?AVCStudioRenderContext@@
r5apex.exe!0x0170cc88 .?AVCSurfaceDragDropTarget@@
r5apex.exe!0x01ed8340 .?AVCSystem@@
r5apex.exe!0x24e06c20 .?AVCTEBeamEntPoint@@
r5apex.exe!0x24d30170 .?AVCTEBeamEnts@@
r5apex.exe!0x24e07390 .?AVCTEBeamFollow@@
r5apex.exe!0x24e06a00 .?AVCTEBeamLaser@@
r5apex.exe!0x24d30830 .?AVCTEBeamPoints@@
r5apex.exe!0x24e06ad0 .?AVCTEBeamRing@@
r5apex.exe!0x24d30750 .?AVCTEBeamRingPoint@@
r5apex.exe!0x02783020 .?AVCTEBeamSpline@@
r5apex.exe!0x02776850 .?AVCTEBreakModel@@
r5apex.exe!0x02783cb0 .?AVCTEEffectDispatch@@
r5apex.exe!0x02772580 .?AVCTEExplosion@@
r5apex.exe!0x02369b38 .?AVCTEGibEvent@@
r5apex.exe!0x0276b830 .?AVCTEPhysicsProp@@
r5apex.exe!0x0279aa50 .?AVCTEProjectileTrail@@
r5apex.exe!0x02331880 .?AVCTEScriptParticleSystem@@
r5apex.exe!0x0230d5a0 .?AVCTEScriptParticleSystemOnEntity@@
r5apex.exe!0x0230e230 .?AVCTEScriptParticleSystemOnEntityWithPos@@
r5apex.exe!0x02785b10 .?AVCTEShatterSurface@@
r5apex.exe!0x0236e068 .?AVCTESoundDispatch@@
r5apex.exe!0x022944a0 .?AVCTempEnts@@
r5apex.exe!0x0277ecb0 .?AVCTempEntsSystem@@
r5apex.exe!0x02782e08 .?AVCTemplate_SaveRestoreBlockHandler@@
r5apex.exe!0x0236e1d8 .?AVCTemplatesHook@@
r5apex.exe!0x01ed8690 .?AVCTextureDictionary@@
r5apex.exe!0x0170c7f8 .?AVCTextureIdProperty@@
r5apex.exe!0x0235d898 .?AVCThinkContextsSaveDataOps@@
r5apex.exe!0x02396ad8 .?AVCTonemapSystem@@
r5apex.exe!0x0232f920 .?AVCTraceFilterSkipTwoEntities@@
r5apex.exe!0x0232f950 .?AVCTraceFilterSkipTwoEntities@@
r5apex.exe!0x0232f980 .?AVCTraceFilterSkipTwoEntities@@
r5apex.exe!0x0232f9b0 .?AVCTraceFilterSkipTwoEntities@@
r5apex.exe!0x0232f9e0 .?AVCTraceFilterSkipTwoEntities@@
r5apex.exe!0x0232fa10 .?AVCTraceFilterSkipTwoEntities@@
r5apex.exe!0x0232fa40 .?AVCTraceFilterSkipTwoEntities@@
r5apex.exe!0x0232fa70 .?AVCTraceFilterSkipTwoEntities@@
r5apex.exe!0x0279c220 .?AVCTurretList@@
r5apex.exe!0x017055d8 .?AVCUniformRandomStream@@
r5apex.exe!0x0170dea8 .?AVCUniformRandomStream@@
r5apex.exe!0x01842f38 .?AVCUniformRandomStream@@
r5apex.exe!0x02368f08 .?AVCUniformRandomStream@@
r5apex.exe!0x023e9218 .?AVCUniformRandomStream@@
r5apex.exe!0x01ed5670 .?AVCUtlCStringConversion@@
r5apex.exe!0x01ed6890 .?AVCUtlNoEscConversion@@
r5apex.exe!0x01705ce8 .?AVCVEfx@@
r5apex.exe!0x01706e48 .?AVCVEngineServer@@
r5apex.exe!0x232c4a90 .?AVCVGui@@
r5apex.exe!0x01705bf8 .?AVCVRenderView@@
r5apex.exe!0x022aca90 .?AVCVScriptGameSystem@@
r5apex.exe!0x0277c9d0 .?AVCVScriptPostEntitySaveRestoreBlockHandler@@
r5apex.exe!0x02783d50 .?AVCVScriptPreEntitySaveRestoreBlockHandler@@
r5apex.exe!0x02782de8 .?AVCVScriptServerGameSystem@@
r5apex.exe!0x02358688 .?AVCVariantSaveDataOps@@
r5apex.exe!0x022a8910 .?AVCViewEffects@@
r5apex.exe!0x022b0a20 .?AVCViewEffects@@
r5apex.exe!0x2361a0c0 .?AVCViewRender@@
r5apex.exe!0x022abc10 .?AVCViewRenderBeams@@
r5apex.exe!0x01843548 .?AVCViewportClientSystem@@
r5apex.exe!0x02368ef0 .?AVCVoiceGameMgr@@
r5apex.exe!0x01705f88 .?AVCVoiceServer@@
r5apex.exe!0x027a0aa0 .?AVCWeaponXList@@
r5apex.exe!0x232c4890 .?AVCWin32Surface@@
r5apex.exe!0x232c4898 .?AVCWin32Surface@@
r5apex.exe!0x232c48a0 .?AVCWin32Surface@@
r5apex.exe!0x017104b8 .?AVC_BaseAnimatingGameSystem@@
r5apex.exe!0x022e8ef0 .?AVC_DataObjectAccessSystem@@
r5apex.exe!0x022a54a8 .?AVC_DefaultParticleSystemQuery@@
r5apex.exe!0x01844da8 .?AVC_DirtySpatialPartitionEntityList@@
r5apex.exe!0x022b5130 .?AVC_GameMovement@@
r5apex.exe!0x022ce630 .?AVC_GameRules@@
r5apex.exe!0x238a9db0 .?AVC_GameStringPool@@
r5apex.exe!0x018481b8 .?AVC_GameTimescale@@
r5apex.exe!0x022e1370 .?AVC_ParticleSystemQuery@@
r5apex.exe!0x019552f8 .?AVC_PrecacheHandler@@
r5apex.exe!0x01849c18 .?AVC_PrecacheRegister@@
r5apex.exe!0x022f0b50 .?AVC_PropData@@
r5apex.exe!0x01954a98 .?AVC_PropSurvivalList@@
r5apex.exe!0x0229d9f0 .?AVC_SoundscapeSystem@@
r5apex.exe!0x022a2440 .?AVC_TEBeamEntPoint@@
r5apex.exe!0x022a2450 .?AVC_TEBeamEntPoint@@
r5apex.exe!0x02299620 .?AVC_TEBeamEnts@@
r5apex.exe!0x02299630 .?AVC_TEBeamEnts@@
r5apex.exe!0x0229d720 .?AVC_TEBeamFollow@@
r5apex.exe!0x0229d730 .?AVC_TEBeamFollow@@
r5apex.exe!0x022994b0 .?AVC_TEBeamLaser@@
r5apex.exe!0x022994c0 .?AVC_TEBeamLaser@@
r5apex.exe!0x022a3ff0 .?AVC_TEBeamPoints@@
r5apex.exe!0x022a4000 .?AVC_TEBeamPoints@@
r5apex.exe!0x02299e30 .?AVC_TEBeamRing@@
r5apex.exe!0x02299e40 .?AVC_TEBeamRing@@
r5apex.exe!0x022a0d30 .?AVC_TEBeamRingPoint@@
r5apex.exe!0x022a0d40 .?AVC_TEBeamRingPoint@@
r5apex.exe!0x022a1520 .?AVC_TEBeamSpline@@
r5apex.exe!0x022a1530 .?AVC_TEBeamSpline@@
r5apex.exe!0x02298800 .?AVC_TEBreakModel@@
r5apex.exe!0x02298810 .?AVC_TEBreakModel@@
r5apex.exe!0x022945c0 .?AVC_TEEffectDispatch@@
r5apex.exe!0x022945d0 .?AVC_TEEffectDispatch@@
r5apex.exe!0x02294520 .?AVC_TEExplosion@@
r5apex.exe!0x02294530 .?AVC_TEExplosion@@
r5apex.exe!0x01717a28 .?AVC_TEGibEvent@@
r5apex.exe!0x01717a38 .?AVC_TEGibEvent@@
r5apex.exe!0x0228f500 .?AVC_TEPhysicsProp@@
r5apex.exe!0x0228f510 .?AVC_TEPhysicsProp@@
r5apex.exe!0x022fcd80 .?AVC_TEProjectileTrail@@
r5apex.exe!0x022fcd90 .?AVC_TEProjectileTrail@@
r5apex.exe!0x022e0020 .?AVC_TEScriptParticleSystem@@
r5apex.exe!0x022e0030 .?AVC_TEScriptParticleSystem@@
r5apex.exe!0x022b4d00 .?AVC_TEScriptParticleSystemOnEntity@@
r5apex.exe!0x022b4d10 .?AVC_TEScriptParticleSystemOnEntity@@
r5apex.exe!0x022e2ee0 .?AVC_TEScriptParticleSystemOnEntityWithPos@@
r5apex.exe!0x022e2ef0 .?AVC_TEScriptParticleSystemOnEntityWithPos@@
r5apex.exe!0x0229c110 .?AVC_TEShatterSurface@@
r5apex.exe!0x0229c120 .?AVC_TEShatterSurface@@
r5apex.exe!0x02299520 .?AVC_TESoundDispatch@@
r5apex.exe!0x02299530 .?AVC_TESoundDispatch@@
r5apex.exe!0x017dc788 .?AVC_TempEntsSystem@@
r5apex.exe!0x022ce6c0 .?AVC_TraceFilterSkipTwoEntities@@
r5apex.exe!0x022ce6f0 .?AVC_TraceFilterSkipTwoEntities@@
r5apex.exe!0x022ce720 .?AVC_TraceFilterSkipTwoEntities@@
r5apex.exe!0x022ce750 .?AVC_TraceFilterSkipTwoEntities@@
r5apex.exe!0x022ce780 .?AVC_TraceFilterSkipTwoEntities@@
r5apex.exe!0x022ce7b0 .?AVC_TraceFilterSkipTwoEntities@@
r5apex.exe!0x022ce7e0 .?AVC_TraceFilterSkipTwoEntities@@
r5apex.exe!0x022ce810 .?AVC_TraceFilterSkipTwoEntities@@
r5apex.exe!0x023014b0 .?AVC_TurretList@@
r5apex.exe!0x023071d0 .?AVC_WeaponXList@@
r5apex.exe!0x017f69a8 .?AVClientDataBlockReceiver@@
r5apex.exe!0x01ef7430 .?AVClientModeFullscreen@@
r5apex.exe!0x012bf7f0 .?AVDNameStatusNode@@
r5apex.exe!0x012bf800 .?AVDNameStatusNode@@
r5apex.exe!0x012bf810 .?AVDNameStatusNode@@
r5apex.exe!0x012bf820 .?AVDNameStatusNode@@
r5apex.exe!0x016e6400 .?AVDenuvoTrialV2@@
r5apex.exe!0x027a1350 .?AVDroppedWeaponManager@@
r5apex.exe!0x02358458 .?AVHSCRIPTSaveRestoreOps@@
r5apex.exe!0x0235a4e8 .?AVHSQOBJECTSaveRestoreOps@@
r5apex.exe!0x01ed0d10 .?AVHardwareConfigDX11@@
r5apex.exe!0x0236e538 .?AVIPredictionSystem@@
r5apex.exe!0x017dc8b8 .?AVIPredictionSystem_Client@@
r5apex.exe!0x01ed9c20 .?AVIVP_BetterDebugmanager@@
r5apex.exe!0x0184a7e0 .?AVImeTextStore@@
r5apex.exe!0x0184a7e8 .?AVImeTextStore@@
r5apex.exe!0x0184a7f0 .?AVImeTextStore@@
r5apex.exe!0x0184a7f8 .?AVImeTextStore@@
r5apex.exe!0x0184a800 .?AVImeTextStore@@
r5apex.exe!0x0184a808 .?AVImeTextStore@@
r5apex.exe!0x0184a810 .?AVImeTextStore@@
r5apex.exe!0x0184a818 .?AVImeTextStore@@
r5apex.exe!0x017dcda8 .?AVMapSettingsReseter@@
r5apex.exe!0x028336a0 .?AVMonitorDefaultChanges@@
r5apex.exe!0x0236e7a8 .?AVPilotClassActivityModifierSaveRestoreDataOps@@
r5apex.exe!0x01817770 .?AVSVC_UserMessage@@
r5apex.exe!0x0170cda8 .?AVVPanelWrapper@@
r5apex.exe!0x02882420 .?AVbad_alloc@std@@
r5apex.exe!0x1ebab278 .?AVstl_critical_section_win7@details@Concurrency@@
r5apex.exe!0x1ebac2d8 .?AVstl_critical_section_win7@details@Concurrency@@
r5apex.exe!0x02371eb8 .?AVweaponScriptCB_HSCRIPTSaveRestoreOps@@
```

