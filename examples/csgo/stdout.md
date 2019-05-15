# Counter-Strike Global Offensive

Demonstrates static analysis by dumping Counter-Strike Global Offensive offsets.

## Engine.dll

### Interfaces

```
engine.dll!0x007cc0d8 BlackBoxVersion001
engine.dll!0x0059539c BugReporterUserName001
engine.dll!0x0059649c EngineTraceClient004
engine.dll!0x00596638 EngineTraceServer004
engine.dll!0x0059ee30 FileLoggingListener001
engine.dll!0x00596958 GAMEEVENTSMANAGER001
engine.dll!0x007db6c8 GAMEEVENTSMANAGER002
engine.dll!0x007cbcd8 GameServerData001
engine.dll!0x005183fc IEngineSoundClient003
engine.dll!0x0051846c IEngineSoundServer003
engine.dll!0x0059d33c INETSUPPORT_003
engine.dll!0x005936c8 ISERVERPLUGINHELPERS001
engine.dll!0x00593be0 ServerUploadGameStats001
engine.dll!0x00858ac8 SpatialPartition001
engine.dll!0x00858d1c StaticPropMgrClient005
engine.dll!0x00858d20 StaticPropMgrServer002
engine.dll!0x005a0580 VCLIENTENGINETOOLS001
engine.dll!0x00595b60 VCvarQuery001
engine.dll!0x0058ca70 VDebugOverlay004
engine.dll!0x0085a9f0 VENGINETOOL003
engine.dll!0x0085a9f0 VENGINETOOLFRAMEWORK003
engine.dll!0x0059f484 VENGINE_GAMEUIFUNCS_VERSION005
engine.dll!0x0059f488 VENGINE_HLDS_API_VERSION002
engine.dll!0x0059f580 VENGINE_LAUNCHER_API_VERSION004
engine.dll!0x0058a0a0 VEngineClient014
engine.dll!0x0059df18 VEngineClientStringTable001
engine.dll!0x00590a4c VEngineEffects001
engine.dll!0x00776050 VEngineModel016
engine.dll!0x0059e364 VEngineRandom001
engine.dll!0x005910a0 VEngineRenderView014
engine.dll!0x00594c08 VEngineServer023
engine.dll!0x0059df54 VEngineServerStringTable001
engine.dll!0x00798d70 VEngineShadowMgr002
engine.dll!0x0085ac28 VEngineVGui001
engine.dll!0x0059c940 VModelInfoClient004
engine.dll!0x0059c9a8 VModelInfoServer002
engine.dll!0x0058ca74 VPhysicsDebugOverlay001
engine.dll!0x0059fca8 VProfExport001
engine.dll!0x005a0528 VSERVERENGINETOOLS001
engine.dll!0x005a052c VTOOLFRAMEWORKVERSION002
engine.dll!0x0059fc68 VoiceServer002
engine.dll!0x005a03c8 XboxSystemInterface002
```

### ConVars

<details>
<summary><code>adsp_debug</code></summary>

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>budget_averages_window</code></summary>

number of frames to look at when figuring out average frametimes

default: `"30"`  
flags: `0x80`  
</details>
<details>
<summary><code>budget_background_alpha</code></summary>

how translucent the budget panel is

default: `"128"`  
flags: `0x80`  
</details>
<details>
<summary><code>budget_bargraph_background_alpha</code></summary>

how translucent the budget panel is

default: `"128"`  
flags: `0x80`  
</details>
<details>
<summary><code>budget_history_numsamplesvisible</code></summary>

number of samples to draw in the budget history window.  The lower the better as far as rendering overhead of the budget panel

default: `"100"`  
flags: `0x80`  
</details>
<details>
<summary><code>budget_peaks_window</code></summary>

number of frames to look at when figuring out peak frametimes

default: `"30"`  
flags: `0x80`  
</details>
<details>
<summary><code>budget_show_averages</code></summary>

enable/disable averages in the budget panel

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>budget_show_history</code></summary>

turn history graph off and on. . good to turn off on low end

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>budget_show_peaks</code></summary>

enable/disable peaks in the budget panel

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>bugreporter_uploadasync</code></summary>

Upload attachments asynchronously

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>bugreporter_username</code></summary>

Username to use for bugreporter

default: `""`  
flags: `0x80`  
</details>
<details>
<summary><code>cl_allowdownload</code></summary>

Client downloads customization files

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>cl_allowupload</code></summary>

Client uploads customization files

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>cl_clock_correction</code></summary>

Enable/disable clock correction on the client.

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_clock_correction_adjustment_max_amount</code></summary>

Sets the maximum number of milliseconds per second it is allowed to correct the client clock. It will only correct this amount if the difference between the client and server clock is equal to or larger than cl_clock_correction_adjustment_max_offset.

default: `"200"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_clock_correction_adjustment_max_offset</code></summary>

As the clock offset goes from cl_clock_correction_adjustment_min_offset to this value (in milliseconds), it moves towards applying cl_clock_correction_adjustment_max_amount of adjustment. That way, the response is small when the offset is small.

default: `"90"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_clock_correction_adjustment_min_offset</code></summary>

If the clock offset is less than this amount (in milliseconds), then no clock correction is applied.

default: `"10"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_clock_correction_force_server_tick</code></summary>

Force clock correction to match the server tick + this offset (-999 disables it).

default: `"999"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_clock_showdebuginfo</code></summary>

Show debugging info about the clock drift. 

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_clockdrift_max_ms</code></summary>

Maximum number of milliseconds the clock is allowed to drift before the client snaps its clock to the server's.

default: `"150"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_clockdrift_max_ms_threadmode</code></summary>

Maximum number of milliseconds the clock is allowed to drift before the client snaps its clock to the server's.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_color</code></summary>

Preferred teammate color

default: `"0"`  
flags: `0x280`  
min value: `0`  
max value: `4`  
</details>
<details>
<summary><code>cl_debug_ugc_downloads</code></summary>

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>cl_decryptdata_key</code></summary>

Key to decrypt encrypted GOTV messages

default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>cl_decryptdata_key_pub</code></summary>

Key to decrypt public encrypted GOTV messages

default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>cl_download_demoplayer</code></summary>

Determines whether downloads of external resources are allowed during demo playback (0:no,1:workshop,2:all)

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>cl_downloadfilter</code></summary>

Determines which files can be downloaded from the server (all, none, nosounds)

default: `"all"`  
flags: `0x80`  
</details>
<details>
<summary><code>cl_entityreport</code></summary>

For debugging, draw entity states to console

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_flushentitypacket</code></summary>

For debugging. Force the engine to flush an entity packet.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_forcepreload</code></summary>

Whether we should force preloading.

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>cl_hideserverip</code></summary>

If set to 1, server IPs will be hidden in the console (except when you type 'status')

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>cl_ignorepackets</code></summary>

Force client to ignore packets (for debugging).

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_interpolate</code></summary>

Enables or disables interpolation on listen servers or during demo playback

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>cl_interpolate</code></summary>

Enables or disables interpolation on listen servers or during demo playback

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>cl_resend</code></summary>

Delay in seconds before the client will resend the 'connect' attempt

default: `"2"`  
flags: `0x80000`  
min value: `1.5`  
max value: `20`  
</details>
<details>
<summary><code>cl_resend_timeout</code></summary>

Total time allowed for the client to resend the 'connect' attempt

default: `"60"`  
flags: `0x80000`  
min value: `1.5`  
max value: `20000`  
</details>
<details>
<summary><code>cl_showevents</code></summary>

Print event firing info in the console

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_showpluginmessages2</code></summary>

Allow plugins to display messages to you

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>cl_skipslowpath</code></summary>

Set to 1 to skip any models that don't go through the model fast path

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_timeout</code></summary>

After this many seconds without receiving a packet from the server, the client will disconnect itself

default: `"30"`  
flags: `0x80`  
min value: `4`  
max value: `30`  
</details>
<details>
<summary><code>clientport</code></summary>

Host game client port

default: `"27005"`  
flags: `0x80000`  
</details>
<details>
<summary><code>closecaption</code></summary>

Enable close captioning.

default: `"0"`  
flags: `0x1000080`  
</details>
<details>
<summary><code>con_enable</code></summary>

Allows the console to be activated.

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>con_filter_enable</code></summary>

Filters console output based on the setting of con_filter_text. 1 filters completely, 2 displays filtered text brighter than other text.

default: `"0"`  
flags: `0x880000`  
</details>
<details>
<summary><code>con_filter_text</code></summary>

Text with which to filter console spew. Set con_filter_enable 1 or 2 to activate.

default: `""`  
flags: `0x880000`  
</details>
<details>
<summary><code>con_filter_text_out</code></summary>

Text with which to filter OUT of console spew. Set con_filter_enable 1 or 2 to activate.

default: `""`  
flags: `0x880000`  
</details>
<details>
<summary><code>con_notifytime</code></summary>

How long to display recent console text to the upper part of the game window

default: `"8"`  
flags: `0x800000`  
</details>
<details>
<summary><code>con_timestamp</code></summary>

Prefix console.log entries with timestamps

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>con_trace</code></summary>

Print console text to low level printout.

default: `"0"`  
flags: `0x800000`  
</details>
<details>
<summary><code>contimes</code></summary>

Number of console lines to overlay for debugging.

default: `"8"`  
flags: `0x800000`  
</details>
<details>
<summary><code>coop</code></summary>

Cooperative play.

default: `"0"`  
flags: `0x100`  
</details>
<details>
<summary><code>deathmatch</code></summary>

Running a deathmatch server.

default: `"0"`  
flags: `0x100`  
</details>
<details>
<summary><code>debug_map_crc</code></summary>

Prints CRC for each map lump loaded

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>demo_recordcommands</code></summary>

Record commands typed at console into .dem files.

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>demo_strict_validation</code></summary>

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>developer</code></summary>

Set developer message level

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>display_game_events</code></summary>

Show listening addition/removals

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>dsp_automatic</code></summary>

default: `"0"`  
flags: `0x10000`  
</details>
<details>
<summary><code>dsp_db_min</code></summary>

default: `"80"`  
flags: `0x14000`  
</details>
<details>
<summary><code>dsp_db_mixdrop</code></summary>

default: `"0.5"`  
flags: `0x14000`  
</details>
<details>
<summary><code>dsp_dist_max</code></summary>

default: `"1440.0"`  
flags: `0x14000`  
</details>
<details>
<summary><code>dsp_dist_min</code></summary>

default: `"0.0"`  
flags: `0x14000`  
</details>
<details>
<summary><code>dsp_enhance_stereo</code></summary>

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>dsp_facingaway</code></summary>

default: `"0"`  
flags: `0x10000`  
</details>
<details>
<summary><code>dsp_mix_max</code></summary>

default: `"0.8"`  
flags: `0x14000`  
</details>
<details>
<summary><code>dsp_mix_min</code></summary>

default: `"0.2"`  
flags: `0x14000`  
</details>
<details>
<summary><code>dsp_off</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>dsp_room</code></summary>

Spews text every time a DSP effect is applied if set to 1. 0 to turn the spew off (default).

default: `"0"`  
flags: `0x10000`  
</details>
<details>
<summary><code>dsp_slow_cpu</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>dsp_spatial</code></summary>

default: `"40"`  
flags: `0x10000`  
</details>
<details>
<summary><code>dsp_speaker</code></summary>

default: `"50"`  
flags: `0x10000`  
</details>
<details>
<summary><code>dsp_vol_2ch</code></summary>

default: `"1.0"`  
flags: `0x10000`  
</details>
<details>
<summary><code>dsp_vol_4ch</code></summary>

default: `"0.5"`  
flags: `0x10000`  
</details>
<details>
<summary><code>dsp_vol_5ch</code></summary>

default: `"0.5"`  
flags: `0x10000`  
</details>
<details>
<summary><code>dsp_volume</code></summary>

default: `"0.8"`  
flags: `0x4000`  
</details>
<details>
<summary><code>dsp_water</code></summary>

default: `"14"`  
flags: `0x10000`  
</details>
<details>
<summary><code>enable_debug_overlays</code></summary>

Enable rendering of debug overlays

default: `"1"`  
flags: `0x4004`  
</details>
<details>
<summary><code>engine_no_focus_sleep</code></summary>

default: `"50"`  
flags: `0x80`  
</details>
<details>
<summary><code>fog_enable_water_fog</code></summary>

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>force_audio_english</code></summary>

Keeps track of whether we're forcing english in a localized language.

default: `"0"`  
flags: `0x1000080`  
</details>
<details>
<summary><code>fps_max_menu</code></summary>

Frame rate limiter, main menu

default: `"120"`  
flags: `0x80000`  
</details>
<details>
<summary><code>fps_screenshot_frequency</code></summary>

While the fps is below the threshold we will dump a screen shot this often in seconds (i.e. 10 = screen shot every 10 seconds when under the given fps.)

default: `"10"`  
flags: `0x4000`  
</details>
<details>
<summary><code>fps_screenshot_threshold</code></summary>

Dump a screenshot when the FPS drops below the given value.

default: `"-1"`  
flags: `0x4000`  
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
flags: `0x6000`  
</details>
<details>
<summary><code>host_info_show</code></summary>

How server info gets disclosed in server queries: 0 - query disabled, 1 - show only general info, 2 - show full info

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>host_map</code></summary>

Current map name.

default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>host_name_store</code></summary>

Whether hostname is recorded in game events and GOTV.

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>host_players_show</code></summary>

How players are disclosed in server queries: 0 - query disabled, 1 - show only max players count, 2 - show all players

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>host_rules_show</code></summary>

How server rules get disclosed in server queries: 0 - query disabled, 1 - query enabled

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>host_sleep</code></summary>

Force the host to sleep a certain number of milliseconds each frame.

default: `"0"`  
flags: `0x4000`  
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
<summary><code>hostport</code></summary>

Host game server port

default: `"27015"`  
flags: `0x80000`  
</details>
<details>
<summary><code>in_forceuser</code></summary>

Force user input to this split screen player.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>ip</code></summary>

Overrides IP for multihomed hosts

default: `"localhost"`  
flags: `0x80000`  
</details>
<details>
<summary><code>ip_relay</code></summary>

Overrides IP used to redirect TV relay connections for NAT hosts

default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>ip_steam</code></summary>

Overrides IP used to bind Steam port for multihomed hosts

default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>ip_tv</code></summary>

Overrides IP used to bind TV port for multihomed hosts

default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>ip_tv1</code></summary>

Overrides IP used to bind TV1 port for multihomed hosts

default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>lightcache_maxmiss</code></summary>

default: `"2"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_ambient_light_b</code></summary>

default: `"0.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_ambient_light_g</code></summary>

default: `"0.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_ambient_light_r</code></summary>

default: `"0.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_bumpbasis</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_colorcorrection</code></summary>

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_debugalttab</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_depthbias_normal</code></summary>

default: `"0.0f"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_dynamic_tonemapping</code></summary>

Force connection attempts to time out

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_force_tonemap_scale</code></summary>

default: `"0.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_forcedynamic</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_fullbright</code></summary>

Enable/Disable specularity for visual testing.  Will not reload materials and will not affect perf.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_loadtextures</code></summary>

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_luxels</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_monitorgamma</code></summary>

monitor gamma (typically 2.2 for CRT and 1.7 for LCD)

default: `"2.2"`  
flags: `0x1000080`  
min value: `1.6`  
max value: `2.6`  
</details>
<details>
<summary><code>mat_monitorgamma_tv_enabled</code></summary>



default: `"0"`  
flags: `0x1000080`  
</details>
<details>
<summary><code>mat_norendering</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_normals</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_show_texture_memory_usage</code></summary>

Display the texture memory usage on the HUD.

default: `"0"`  
flags: `0x5000`  
</details>
<details>
<summary><code>mat_softwareskin</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_surfaceid</code></summary>

Use shadow fast path for CSM rendering - minimize number of draw call

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_surfacemat</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_texture_list</code></summary>

For debugging, show a list of used textures per frame

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_texture_list_all</code></summary>

If this is nonzero, then the texture list panel will show all currently-loaded textures.

default: `"0"`  
flags: `0x1000`  
</details>
<details>
<summary><code>mat_texture_list_view</code></summary>

If this is nonzero, then the texture list panel will render thumbnails of currently-loaded textures.

default: `"1"`  
flags: `0x1000`  
</details>
<details>
<summary><code>mat_wireframe</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mem_incremental_compact_rate</code></summary>

Rate at which to attempt internal heap compation

default: `".5"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mod_dynamicloadpause</code></summary>

default: `"0"`  
flags: `0x24010`  
</details>
<details>
<summary><code>mod_dynamicloadspew</code></summary>

default: `"0"`  
flags: `0x20010`  
</details>
<details>
<summary><code>mod_dynamicloadthrottle</code></summary>

default: `"0"`  
flags: `0x24010`  
</details>
<details>
<summary><code>mod_dynamicunloadtex</code></summary>

default: `"1"`  
flags: `0x20010`  
</details>
<details>
<summary><code>mod_dynamicunloadtime</code></summary>



default: `"150"`  
flags: `0x20010`  
</details>
<details>
<summary><code>net_blockmsg</code></summary>

Discards incoming message: <0|1|name>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>net_droponsendoverflow</code></summary>

If enabled, channel will drop client when sending too much data causes buffer overrun

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>net_droppackets</code></summary>

Drops next n packets on client

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>net_earliertempents</code></summary>

Low priority dlights are replaced by high priority ones

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>net_fakejitter</code></summary>

Jitter fakelag packet time

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>net_fakelag</code></summary>

Lag all incoming network data (including loopback) by this many milliseconds.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>net_fakeloss</code></summary>

Simulate packet loss as a percentage (negative means drop 1/n packets)

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>net_maxroutable</code></summary>

Requested max packet size before packets are 'split'.

default: `"1200"`  
flags: `0x280`  
min value: `576`  
max value: `1200`  
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
flags: `0x2000000`  
</details>
<details>
<summary><code>net_showreliablesounds</code></summary>

default: `"0"`  
flags: `0x4000`  
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
<summary><code>net_splitrate</code></summary>

Number of fragments for a splitpacket that can be sent per frame

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>net_steamcnx_allowrelay</code></summary>

Allow steam connections to attempt to use relay servers as fallback (best if specified on command line:  +net_steamcnx_allowrelay 1)

default: `"1"`  
flags: `0x80080`  
</details>
<details>
<summary><code>net_steamcnx_enabled</code></summary>

Use steam connections on listen server as a fallback, 2 forces use of steam connections instead of raw UDP.

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>net_threaded_socket_burst_cap</code></summary>

Max number of packets per burst beyond which threaded socket pump algorithm will start dropping packets.

default: `"1024"`  
flags: `0x80000`  
</details>
<details>
<summary><code>net_threaded_socket_recovery_rate</code></summary>

Number of packets per second that threaded socket pump algorithm allows from client.

default: `"6400"`  
flags: `0x80000`  
</details>
<details>
<summary><code>net_threaded_socket_recovery_time</code></summary>

Number of seconds over which the threaded socket pump algorithm will fully recover client ratelimit.

default: `"60"`  
flags: `0x80000`  
</details>
<details>
<summary><code>next</code></summary>

Set to 1 to advance to next frame ( when singlestep == 1 )

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>occlusion_old</code></summary>

Disable area to area connection testing.

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>occlusion_test_async_jitter</code></summary>

default: `"2"`  
flags: `0x4000`  
</details>
<details>
<summary><code>occlusion_test_async_move_tolerance</code></summary>

default: `"8.25"`  
flags: `0x4000`  
</details>
<details>
<summary><code>occlusion_test_jump_margin</code></summary>

Amount by which the player bounding box is expanded up for occlusion test to account for jumping. This margin should be large enough to accommodate player movement within a frame or two. Affects both camera box and player box.

default: `"12"`  
flags: `0x80000`  
</details>
<details>
<summary><code>occlusion_test_margins</code></summary>

Amount by which the player bounding box is expanded for occlusion test. This margin should be large enough to accommodate player movement within a frame or two, and the longest weapon they might hold. Shadow does not take this into account.

default: `"36"`  
flags: `0x80000`  
</details>
<details>
<summary><code>occlusion_test_shadow_max_distance</code></summary>

Max distance at which to consider shadows for occlusion computations

default: `"1500"`  
flags: `0x80000`  
</details>
<details>
<summary><code>paint_alpha_offset_enabled</code></summary>

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>paint_max_surface_border_alpha</code></summary>

Buffer size for writing string table baselines

default: `"0.7f"`  
flags: `0x2002`  
</details>
<details>
<summary><code>paint_min_valid_alpha_value</code></summary>

default: `"0.7f"`  
flags: `0x2002`  
</details>
<details>
<summary><code>paintsplat_bias</code></summary>

Change bias value for computing circle buffer

default: `"0.1f"`  
flags: `0x6000`  
</details>
<details>
<summary><code>paintsplat_max_alpha_noise</code></summary>

Max noise value of circle alpha

default: `"0.1f"`  
flags: `0x6000`  
</details>
<details>
<summary><code>paintsplat_noise_enabled</code></summary>

default: `"1"`  
flags: `0x6000`  
</details>
<details>
<summary><code>panel_test_title_safe</code></summary>

Test vgui panel positioning with title safe indentation

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>password</code></summary>

Current server access password

default: `""`  
flags: `0x20020080`  
</details>
<details>
<summary><code>r_ClipAreaFrustums</code></summary>

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_ClipAreaPortals</code></summary>

Prevents occlusion testing for entities that take up more than X% of the screen. 0 means use whatever the level said to use.

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_DispBuildable</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_DispWalkable</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_DrawBeams</code></summary>

0=Off, 1=Normal, 2=Wireframe

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_DrawDisp</code></summary>

Toggles rendering of displacment maps

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_DrawModelLightOrigin</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_DrawPortals</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_ambientfraction</code></summary>

Fraction of direct lighting used to boost lighting when model requests

default: `"0.2"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_ambientlightingonly</code></summary>

Set this to 1 to light models with only ambient lighting (and no static lighting).

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_avglight</code></summary>

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_avglightmap</code></summary>

default: `"0"`  
flags: `0x804000`  
</details>
<details>
<summary><code>r_brush_queue_mode</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_colorstaticprops</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_debugrandomstaticlighting</code></summary>

Set to 1 to randomize static lighting for debugging.  Must restart for change to take affect.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_disable_static_prop_loading</code></summary>

If non-zero when a map loads, static props won't be loaded

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_dlightsenable</code></summary>

default: `"1"`  
flags: `0x804000`  
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
<summary><code>r_drawentities</code></summary>

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawfuncdetail</code></summary>

Render func_detail

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawleaf</code></summary>

Draw the specified leaf.

default: `"-1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawlightcache</code></summary>

0: off
1: draw light cache entries
2: draw rays


default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawlightcache</code></summary>

0: off
1: draw light cache entries
2: draw rays


default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawlightinfo</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawlights</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawmodelstatsoverlay</code></summary>

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
<summary><code>r_drawskybox</code></summary>

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawstaticprops</code></summary>

0=Off, 1=Normal, 2=Wireframe

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawtranslucentworld</code></summary>

If enabled, hides all surfaces which have been painted.

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
<summary><code>r_drawworld</code></summary>

Render the world.

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_dscale_basefov</code></summary>

default: `"90"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_dscale_fardist</code></summary>

default: `"2000"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_dscale_farscale</code></summary>

default: `"4"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_dscale_neardist</code></summary>

default: `"100"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_dscale_nearscale</code></summary>

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_dynamic</code></summary>

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>r_dynamiclighting</code></summary>

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_entity</code></summary>

default: `"-1"`  
flags: `0x4002`  
</details>
<details>
<summary><code>r_eyemove</code></summary>

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>r_eyeshift_x</code></summary>

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>r_eyeshift_y</code></summary>

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>r_eyeshift_z</code></summary>

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>r_eyesize</code></summary>

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>r_flashlightclip</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_flashlightdrawclip</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_flashlightscissor</code></summary>

default: `"0"`  
flags: `0x800000`  
</details>
<details>
<summary><code>r_ignoreStaticColorChecksum</code></summary>

0 - validate vhvhdr and studiohdr checksum, 1 - default, ignore checksum (useful if iterating physics model only for example)

default: `"1"`  
flags: `0x4002`  
</details>
<details>
<summary><code>r_itemblinkmax</code></summary>

0=Off, 1=On, 2=Show Errors

default: `".3"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_itemblinkrate</code></summary>

default: `"4.5"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_lightcache_numambientsamples</code></summary>

number of random directions to fire rays when computing ambient lighting

default: `"162"`  
flags: `0x4000`  
min value: `1`  
max value: `162`  
</details>
<details>
<summary><code>r_lightcache_radiusfactor</code></summary>

Allow lights to influence lightcaches beyond the lights' radii

default: `"1000"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_lightcachecenter</code></summary>

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_lightcachemodel</code></summary>



default: `"-1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_lightinterp</code></summary>

Controls the speed of light interpolation, 0 turns off interpolation

default: `"5"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_lightmap</code></summary>

default: `"-1"`  
flags: `0x804000`  
</details>
<details>
<summary><code>r_lightstyle</code></summary>

default: `"-1"`  
flags: `0x804000`  
</details>
<details>
<summary><code>r_lockpvs</code></summary>

Lock the PVS so you can fly around and inspect what is being drawn.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_modelAmbientMin</code></summary>

Minimum value for the ambient lighting on dynamic models with more than one bone (like players and their guns).

default: `"0.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_modelwireframedecal</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_nohw</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_nosw</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_novis</code></summary>

Turn off the PVS.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_occlusionspew</code></summary>

Activate/deactivates spew about what the occlusion system is doing.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_oldlightselection</code></summary>

Set this to revert to HL2's method of selecting lights

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_partition_level</code></summary>

Displays a particular level of the spatial partition system. Use -1 to disable it.

default: `"-1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_portalsopenall</code></summary>

Open all portals

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_proplightingpooling</code></summary>

0 - off, 1 - static prop color meshes are allocated from a single shared vertex buffer (on hardware that supports stream offset)

default: `"-1.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_radiosity</code></summary>

0: no radiosity
1: radiosity with ambient cube (6 samples)
2: radiosity with 162 samples
3: 162 samples for static props, 6 samples for everything else

default: `"4"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_shadow_deferred</code></summary>

Toggle deferred shadow rendering

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_shadowids</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_shadows_gamecontrol</code></summary>

Render decals batched.

default: `"-1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_shadowwireframe</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_showenvcubemap</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_skin</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_slowpathwireframe</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_visocclusion</code></summary>

Activate/deactivate wireframe rendering of what the occlusion system is doing.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_visualizelighttraces</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_visualizelighttracesshowfulltrace</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_visualizetraces</code></summary>

Default brightness for lightmaps where none have been created in the level.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>replay_debug</code></summary>

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>room_type</code></summary>

default: `"0"`  
flags: `0x10000`  
</details>
<details>
<summary><code>rpt_vprof_time</code></summary>



default: `"0.25"`  
flags: `0x20010`  
</details>
<details>
<summary><code>showbudget_texture</code></summary>

Enable the texture budget panel.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>singlestep</code></summary>

Run engine in single step mode ( set next to 1 to advance a frame )

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>skill</code></summary>

Game skill level (1-3).

default: `"1"`  
flags: `0x80`  
min value: `1`  
max value: `3`  
</details>
<details>
<summary><code>snd_deathcamera_volume</code></summary>

Relative volume of the death camera music.

default: `"1.0"`  
flags: `0x80080`  
</details>
<details>
<summary><code>snd_debug_panlaw</code></summary>

Visualize panning crossfade curves

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_disable_mixer_duck</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_disable_mixer_solo</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_duckerattacktime</code></summary>

default: `"0.5"`  
flags: `0x80`  
</details>
<details>
<summary><code>snd_duckerreleasetime</code></summary>

default: `"2.5"`  
flags: `0x80`  
</details>
<details>
<summary><code>snd_duckerthreshold</code></summary>

default: `"0.15"`  
flags: `0x80`  
</details>
<details>
<summary><code>snd_ducking_off</code></summary>

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>snd_ducktovolume</code></summary>

default: `"0.55"`  
flags: `0x80`  
</details>
<details>
<summary><code>snd_dvar_dist_max</code></summary>

Play full 'far' sound at this distance

default: `"1320"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_dvar_dist_min</code></summary>

Play full 'near' sound at this distance

default: `"240"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_dzmusic_volume</code></summary>

Relative volume of the Danger Zone victory music.

default: `"0.2"`  
flags: `0x80080`  
</details>
<details>
<summary><code>snd_filter</code></summary>

Doppler effect is extremely sensible to volume variation. To reduce the pops, the cross-fade has to be very slow.

default: `""`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_foliage_db_loss</code></summary>

foliage dB loss per 1200 units

default: `"4"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_gain</code></summary>

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_gain_max</code></summary>

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_gain_min</code></summary>

default: `"0.01"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_hrtf_distance_behind</code></summary>

HRTF calculations will calculate the player as being this far behind the camera


default: `"100"`  
flags: `0x80`  
</details>
<details>
<summary><code>snd_hrtf_lerp_max_distance</code></summary>

default: `"800.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_hrtf_lerp_min_distance</code></summary>

default: `"100.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_hrtf_stereo_blend</code></summary>

Filter used to spew sounds that starts late. Use an empty string "" to display all sounds. By default only the VO are displayed.

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_hrtf_voice_delay</code></summary>

default: `"0.1"`  
flags: `0x80080`  
</details>
<details>
<summary><code>snd_hrtf_volume</code></summary>

Controls volume of HRTF sounds

default: `"0.8"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_hwcompat</code></summary>

default: `"0"`  
flags: `0x80080`  
</details>
<details>
<summary><code>snd_list</code></summary>

default: `""`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_mapobjective_volume</code></summary>

Relative volume of map objective music.

default: `"1.0"`  
flags: `0x80080`  
</details>
<details>
<summary><code>snd_max_same_sounds</code></summary>

default: `"4"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_max_same_weapon_sounds</code></summary>

default: `"3"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_menumusic_volume</code></summary>

Relative volume of the main menu music.

default: `"1.0"`  
flags: `0x80080`  
</details>
<details>
<summary><code>snd_mix_async</code></summary>

Sets sound to get mixed asynchronously on a different thread

default: `"1"`  
flags: `0x80080`  
</details>
<details>
<summary><code>snd_mix_async_onetime_reset</code></summary>

default: `"0"`  
flags: `0x80090`  
</details>
<details>
<summary><code>snd_mixahead</code></summary>

default: `"0.025"`  
flags: `0x80`  
</details>
<details>
<summary><code>snd_mixer_master_dsp</code></summary>

default: `"1.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_mixer_master_level</code></summary>

Defer sound recording until next tick when laying off movies.

default: `"1.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_music_volume_onetime_reset_2</code></summary>

default: `"0"`  
flags: `0x80090`  
</details>
<details>
<summary><code>snd_musicvolume_multiplier_inoverlay</code></summary>

Music volume multiplier when Steam Overlay is active

default: `"0.1"`  
flags: `0x1000080`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>snd_mute_losefocus</code></summary>

default: `"1"`  
flags: `0x400080`  
</details>
<details>
<summary><code>snd_mvp_volume</code></summary>

Relative volume of the MVP music.

default: `"1.0"`  
flags: `0x80080`  
</details>
<details>
<summary><code>snd_obscured_gain_dB</code></summary>

default: `"-2.70"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_occlusion_bounces</code></summary>

default: `"1"`  
flags: `0x6000`  
</details>
<details>
<summary><code>snd_occlusion_collide_min_distance</code></summary>

default: `"4.0"`  
flags: `0x4002`  
</details>
<details>
<summary><code>snd_occlusion_eq_high</code></summary>

default: `"0.20"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_occlusion_eq_low</code></summary>

default: `"0.10"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_occlusion_eq_mid</code></summary>

default: `"1.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_occlusion_indirect_max</code></summary>

default: `"0.85"`  
flags: `0x4002`  
</details>
<details>
<summary><code>snd_occlusion_indirect_min</code></summary>

default: `"0.3"`  
flags: `0x4002`  
</details>
<details>
<summary><code>snd_occlusion_indirect_radius</code></summary>

default: `"120.0"`  
flags: `0x4002`  
</details>
<details>
<summary><code>snd_occlusion_material_override</code></summary>

default: `""`  
flags: `0x4002`  
</details>
<details>
<summary><code>snd_occlusion_no_eq_scale</code></summary>

default: `"1.05"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_occlusion_pos_override</code></summary>

default: `""`  
flags: `0x4002`  
</details>
<details>
<summary><code>snd_occlusion_rays</code></summary>

default: `"4"`  
flags: `0x6000`  
</details>
<details>
<summary><code>snd_occlusion_visualize</code></summary>

default: `"0"`  
flags: `0x4002`  
</details>
<details>
<summary><code>snd_occlusion_visualize_filter</code></summary>

default: `""`  
flags: `0x4002`  
</details>
<details>
<summary><code>snd_op_test_convar</code></summary>

Disables using an audio cache and relies on streaming audio data instead.

default: `"1.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_pause_all</code></summary>

Specifies to pause all sounds and not just voice

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_pitchquality</code></summary>

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>snd_pre_gain_dist_falloff</code></summary>

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_prefetch_common</code></summary>

Prefetch common sounds from directories specified in scripts/sound_prefetch.txt

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>snd_profile</code></summary>

default: `"0"`  
flags: `0x10000`  
</details>
<details>
<summary><code>snd_rear_speaker_scale</code></summary>

How much to scale rear speaker contribution to front stereo output

default: `"1.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_refdist</code></summary>

Reference distance for snd_refdb

default: `"36"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_report_format_sound</code></summary>

If set to 1, report all sound formats.


default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_report_loop_sound</code></summary>

If set to 1, report all sounds that just looped.


default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_report_start_sound</code></summary>

If set to 1, report all sounds played with S_StartSound(). The sound may not end up being played (if error occurred for example). Use snd_showstart to see the sounds that are really played.


default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_report_stop_sound</code></summary>

If set to 1, report all sounds stopped with S_StopSound().


default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_report_verbose_error</code></summary>

If set to 1, report more error found when playing sounds.


default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_roundend_volume</code></summary>

Relative volume of round end music.

default: `"1.0"`  
flags: `0x80080`  
</details>
<details>
<summary><code>snd_roundstart_volume</code></summary>

Relative volume of round start music.

default: `"1.0"`  
flags: `0x80080`  
</details>
<details>
<summary><code>snd_show</code></summary>

Show sounds info

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_show_filter</code></summary>

Limit debug sounds to those containing this substring

default: `""`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_showclassname</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_showmixer</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_showstart</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_sos_list_operator_updates</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_sos_show_block_debug</code></summary>

Spew data about the list of block entries.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_sos_show_client_rcv</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_sos_show_operator_entry_filter</code></summary>

default: `""`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_sos_show_operator_init</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_sos_show_operator_parse</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_sos_show_operator_prestart</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_sos_show_operator_shutdown</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_sos_show_operator_start</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_sos_show_operator_stop_entry</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_sos_show_operator_updates</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_sos_show_queuetotrack</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_sos_show_startqueue</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_surround_speakers</code></summary>

default: `"-1"`  
flags: `0x80080`  
</details>
<details>
<summary><code>snd_tensecondwarning_volume</code></summary>

Relative volume of ten second warning music.

default: `"1.0"`  
flags: `0x80080`  
</details>
<details>
<summary><code>snd_visualize</code></summary>

Show sounds location in world

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>sound_device_override</code></summary>

ID of the sound device to use

default: `""`  
flags: `0x480080`  
</details>
<details>
<summary><code>spec_replay_enable</code></summary>

Enable Killer Replay, requires hltv server running (0:off, 1:default, 2:force)

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>spec_replay_leadup_time</code></summary>

Replay time in seconds before the highlighted event

default: `"5.3438"`  
flags: `0x82000`  
</details>
<details>
<summary><code>spec_replay_message_time</code></summary>

How long to show the message about Killer Replay after death. The best setting is a bit shorter than spec_replay_autostart_delay + spec_replay_leadup_time + spec_replay_winddown_time

default: `"9.5"`  
flags: `0x82000`  
</details>
<details>
<summary><code>spec_replay_on_death</code></summary>

When > 0, sets the mode whereas players see delayed replay, and are segregated into a domain of chat and voice separate from the alive players

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>spec_replay_rate_base</code></summary>

Base time scale of Killer Replay.Experimental.

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>spec_replay_rate_limit</code></summary>

Minimum allowable pause between replay requests in seconds

default: `"3"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_allow_legacy_cmd_execution_from_client</code></summary>

Enables old concommand execution behavior allowing remote clients to run any command not explicitly flagged as disallowed.

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_allow_wait_command</code></summary>

Allow or disallow the wait command on clients connected to this server.

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_allowdownload</code></summary>

Allow clients to download files

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_allowupload</code></summary>

Allow clients to upload customizations files

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_alternateticks</code></summary>

If set, server only simulates entities on even numbered ticks.


default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_client_cmdrate_difference</code></summary>

cl_cmdrate is moved to within sv_client_cmdrate_difference units of cl_updaterate before it is clamped between sv_mincmdrate and sv_maxcmdrate.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_client_max_interp_ratio</code></summary>

This can be used to limit the value of cl_interp_ratio for connected clients (only while they are connected). If sv_client_min_interp_ratio is -1, then this cvar has no effect.

default: `"5"`  
flags: `0x2000`  
</details>
<details>
<summary><code>sv_client_min_interp_ratio</code></summary>

This can be used to limit the value of cl_interp_ratio for connected clients (only while they are connected).
              -1 = let clients set cl_interp_ratio to anything
 any other value = set minimum value for cl_interp_ratio

default: `"1"`  
flags: `0x2000`  
</details>
<details>
<summary><code>sv_client_predict</code></summary>

This can be used to force the value of cl_predict for connected clients (only while they are connected).
   -1 = let clients set cl_predict to anything
    0 = force cl_predict to 0
    1 = force cl_predict to 1

default: `"-1"`  
flags: `0x2000`  
</details>
<details>
<summary><code>sv_consistency</code></summary>

Whether the server enforces file consistency for critical files

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_contact</code></summary>

Contact email for server sysop

default: `""`  
flags: `0x80100`  
</details>
<details>
<summary><code>sv_creationtickcheck</code></summary>

Do extended check for encoding of timestamps against tickcount

default: `"1"`  
flags: `0x4002`  
</details>
<details>
<summary><code>sv_debugmanualmode</code></summary>

Make sure entities correctly report whether or not their network data has changed.

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_downloadurl</code></summary>

Location from which clients can download missing files

default: `""`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_dumpstringtables</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>sv_duplicate_playernames_ok</code></summary>

When enabled player names won't have the (#) in front of their names its the same as another player.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_enable_delta_packing</code></summary>

When enabled, this allows for entity packing to use the property changes for building up the data. This is many times faster, but can be disabled for error checking.

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_forcepreload</code></summary>

Force server side preloading.

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>sv_hibernate_ms</code></summary>

# of milliseconds to sleep per frame while hibernating

default: `"20"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_hibernate_ms_vgui</code></summary>

# of milliseconds to sleep per frame while hibernating but running the vgui dedicated server frontend

default: `"20"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_hibernate_postgame_delay</code></summary>

# of seconds to wait after final client leaves before hibernating.

default: `"5"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_hibernate_punt_tv_clients</code></summary>

When enabled will punt all GOTV clients during hibernation

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_hosting_lobby</code></summary>

Show temp entity bandwidth usage.

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_lan</code></summary>

Server is a lan server ( no heartbeat, no authentication, no non-class C addresses )

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_log_onefile</code></summary>

Log server information to only one file.

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>sv_logbans</code></summary>

Log server bans in the server logs.

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>sv_logblocks</code></summary>

If true when log when a query is blocked (can cause very large log files)

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_logecho</code></summary>

Echo log information to the console.

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>sv_logfile</code></summary>

Log server information in the log file.

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>sv_logflush</code></summary>

Flush the log file to disk on each write (slow).

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>sv_logsdir</code></summary>

Folder in the game directory where server logs will be stored.

default: `"logs"`  
flags: `0x80`  
</details>
<details>
<summary><code>sv_logsecret</code></summary>

If set then include this secret when doing UDP logging (will use 0x53 as packet type, not usual 0x52)

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_logsocket2</code></summary>

Uses a specific outgoing socket for second source of sv udp logging

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_logsocket2_substr</code></summary>

Uses a substring match for second source of sv udp logging

default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_max_dropped_packets_to_process</code></summary>

Max dropped packets to process. Lower settings prevent lagged players from simulating too far in the past. Setting of 0 disables cap.

default: `"10"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_max_queries_sec</code></summary>

Maximum queries per second to respond to from a single IP address.

default: `"10.0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_max_queries_sec_global</code></summary>

Maximum queries per second to respond to from anywhere.

default: `"500"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_max_queries_tracked_ips_max</code></summary>

Window over which to average queries per second averages.

default: `"50000"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_max_queries_tracked_ips_prune</code></summary>

Window over which to average queries per second averages.

default: `"10"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_max_queries_window</code></summary>

Window over which to average queries per second averages.

default: `"30"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_maxcmdrate</code></summary>

(If sv_mincmdrate is > 0), this sets the maximum value for cl_cmdrate.

default: `"64"`  
flags: `0x2000`  
</details>
<details>
<summary><code>sv_maxrate</code></summary>

Max bandwidth rate allowed on server, 0 == unlimited

default: `"0"`  
flags: `0x82000`  
min value: `0`  
max value: `786432`  
</details>
<details>
<summary><code>sv_maxupdaterate</code></summary>

Maximum updates per second that the server will allow

default: `"64"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_maxuptimelimit</code></summary>

If set, whenever a game ends, if the server uptime exceeds this number of hours, the server will exit.

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_memlimit</code></summary>

If set, whenever a game ends, if the total memory used by the server is greater than this # of megabytes, the server will exit.

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_mincmdrate</code></summary>

This sets the minimum value for cl_cmdrate. 0 == unlimited.

default: `"64"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_minrate</code></summary>

Min bandwidth rate allowed on server, 0 == unlimited

default: `"16000"`  
flags: `0x82000`  
min value: `0`  
max value: `786432`  
</details>
<details>
<summary><code>sv_minupdaterate</code></summary>

Minimum updates per second that the server will allow

default: `"64"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_minuptimelimit</code></summary>

If set, whenever a game ends, if the server uptime is less than this number of hours, the server will continue running regardless of sv_memlimit.

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_parallel_packentities</code></summary>

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_parallel_sendsnapshot</code></summary>

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_pausable</code></summary>

Is the server pausable.

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_pure_consensus</code></summary>

Minimum number of file hashes to agree to form a consensus.

default: `"99999999"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_pure_kick_clients</code></summary>

If set to 1, the server will kick clients with mismatching files. Otherwise, it will issue a warning to the client.

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_pure_retiretime</code></summary>

Seconds of server idle time to flush the sv_pure file hash cache.

default: `"900"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_pure_trace</code></summary>

If set to 1, the server will print a message whenever a client is verifying a CRC for a file.

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_quota_stringcmdspersecond</code></summary>

How many string commands per second clients are allowed to submit, 0 to disallow all string commands

default: `"40"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_rcon_whitelist_address</code></summary>

When set, rcon failed authentications will never ban this address, e.g. '127.0.0.1'

default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_region</code></summary>

The region of the world to report this server in.

default: `"-1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_reliableavatardata</code></summary>

When enabled player avatars are exchanged via gameserver (0: off, 1: players, 2: server)

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_replaybots</code></summary>

If set to 1, the server records data needed to replay network stream from bot's perspective

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_reservation_tickrate_adjustment</code></summary>

Adjust server tickrate upon reservation

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_reservation_timeout</code></summary>

Time in seconds before lobby reservation expires.

default: `"45"`  
flags: `0x80000`  
min value: `5`  
max value: `180`  
</details>
<details>
<summary><code>sv_show_cull_props</code></summary>

Print out props that are being culled/added by recipent proxies.

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_steamauth_enforce</code></summary>

By default, player must maintain a reliable connection to Steam servers. When player Steam session drops, enforce it: 2 = instantly kick, 1 = kick at next spawn, 0 = do not kick.

default: `"2"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_steamgroup_exclusive</code></summary>

If set, only members of Steam group will be able to join the server when it's empty, public people will be able to join the server only if it has players.

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_stressbots</code></summary>

If set to 1, the server calculates data and fills packets to bots. Used for perf testing.

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_unlockedchapters</code></summary>

Highest unlocked game chapter.

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>sv_validate_edict_change_infos</code></summary>

Verify that edict changeinfos are being calculated properly (used to debug local network backdoor mode).

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_visiblemaxplayers</code></summary>

Overrides the max players reported to prospective clients

default: `"-1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_voice_proximity_minvolume</code></summary>

Maximum height of player and still test for adsp

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_voice_proximity_use_falloff</code></summary>

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_voicecodec</code></summary>

Specifies which voice codec DLL to use in a game. Set to the name of the DLL without the extension.

default: `"vaudio_celt"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_voiceenable</code></summary>

default: `"1"`  
flags: `0x80180`  
</details>
<details>
<summary><code>sys_minidumpspewlines</code></summary>

Lines of crash dump console spew to keep.

default: `"500"`  
flags: `0x80000`  
</details>
<details>
<summary><code>texture_budget_background_alpha</code></summary>

how translucent the budget panel is

default: `"128"`  
flags: `0x80`  
</details>
<details>
<summary><code>think_trace_limit</code></summary>

Break into the debugger if this many or more traces are performed in a single think function. Negative numbers mean that the same think function may be broken into many times (once per [x] may traces), positive numbers mean each think will break only once.

default: `"0"`  
flags: `0x4002`  
</details>
<details>
<summary><code>tv_advertise_watchable</code></summary>

GOTV advertises the match as watchable via game UI, clients watching via UI will not need to type password

default: `"0"`  
flags: `0xa0120`  
</details>
<details>
<summary><code>tv_allow_camera_man_override</code></summary>

Allows cameraman_override to have effect. When this is set, the primary interactive caster will have all the relevant fields present in all network packets, in every snapshot. This allows the secondary cameraman (-interactivecaster that connects to a tv port) to override those fields some seconds later regardless of whether they changed originally or not.

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>tv_autorecord</code></summary>

Automatically records all games as GOTV demos.

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>tv_autoretry</code></summary>

Relay proxies retry connection after network timeout

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>tv_broadcast_drop_fragments</code></summary>

Drop every Nth fragment

default: `"0"`  
flags: `0x80010`  
</details>
<details>
<summary><code>tv_broadcast_keyframe_interval</code></summary>

The frequency, in seconds, of sending keyframes and delta fragments to the broadcast relay server

default: `"3"`  
flags: `0x80000`  
</details>
<details>
<summary><code>tv_broadcast_keyframe_interval1</code></summary>

The frequency, in seconds, of sending keyframes and delta fragments to the broadcast1 relay server

default: `"3"`  
flags: `0x80000`  
</details>
<details>
<summary><code>tv_broadcast_max_requests</code></summary>

Max number of broadcast http requests in flight. If there is a network issue, the requests may start piling up, degrading server performance. If more than the specified number of requests are in flight, the new requests are dropped.

default: `"20"`  
flags: `0x80000`  
</details>
<details>
<summary><code>tv_broadcast_max_requests1</code></summary>

Max number of broadcast1 http requests in flight. If there is a network issue, the requests may start piling up, degrading server performance. If more than the specified number of requests are in flight, the new requests are dropped.

default: `"20"`  
flags: `0x80000`  
</details>
<details>
<summary><code>tv_broadcast_origin_auth</code></summary>

X-Origin-Auth header of the broadcast POSTs

default: `"gocastauth"`  
flags: `0x80010`  
</details>
<details>
<summary><code>tv_broadcast_origin_auth1</code></summary>

X-Origin-Auth header of the broadcast1 POSTs

default: `"gocastauth"`  
flags: `0x80010`  
</details>
<details>
<summary><code>tv_broadcast_startup_resend_interval</code></summary>

The interval, in seconds, of re-sending startup data to the broadcast relay server (useful in case relay crashes, restarts or startup data http request fails)

default: `"10"`  
flags: `0x80000`  
</details>
<details>
<summary><code>tv_broadcast_terminate</code></summary>

Terminate every broadcast with a stop command

default: `"1"`  
flags: `0x80010`  
</details>
<details>
<summary><code>tv_broadcast_url</code></summary>

URL of the broadcast relay

default: `"http://localhost:8080"`  
flags: `0x80000`  
</details>
<details>
<summary><code>tv_broadcast_url1</code></summary>

URL of the broadcast relay1

default: `"http://localhost:8080"`  
flags: `0x80000`  
</details>
<details>
<summary><code>tv_chatgroupsize</code></summary>

Set the default chat group size

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>tv_chattimelimit</code></summary>

Limits spectators to chat only every n seconds

default: `"8"`  
flags: `0x80000`  
</details>
<details>
<summary><code>tv_debug</code></summary>

GOTV debug info.

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>tv_deltacache</code></summary>

Enable delta entity bit stream cache

default: `"2"`  
flags: `0x80000`  
</details>
<details>
<summary><code>tv_dispatchmode</code></summary>

Dispatch clients to relay proxies: 0=never, 1=if appropriate, 2=always

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>tv_dispatchweight</code></summary>

Dispatch clients to relay proxies based on load, 1.25 will prefer for every 4 local clients to put 5 clients on every connected relay

default: `"1.25"`  
flags: `0x80000`  
</details>
<details>
<summary><code>tv_enable_delta_frames</code></summary>

Indicates whether or not the tv should use delta frames for storage of intermediate frames. This takes more CPU but significantly less memory.

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>tv_encryptdata_key</code></summary>

When set to a valid key communication messages will be encrypted for GOTV

default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>tv_encryptdata_key_pub</code></summary>

When set to a valid key public communication messages will be encrypted for GOTV

default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>tv_maxclients</code></summary>

Maximum client number on GOTV server.

default: `"128"`  
flags: `0x80000`  
min value: `0`  
max value: `255`  
</details>
<details>
<summary><code>tv_maxclients_relayreserved</code></summary>

Reserves a certain number of GOTV client slots for relays.

default: `"0"`  
flags: `0x80000`  
min value: `0`  
max value: `255`  
</details>
<details>
<summary><code>tv_maxrate</code></summary>

Max GOTV spectator bandwidth rate allowed, 0 == unlimited

default: `"196608"`  
flags: `0x80000`  
</details>
<details>
<summary><code>tv_nochat</code></summary>

Don't receive chat messages from other GOTV spectators

default: `"0"`  
flags: `0x280`  
</details>
<details>
<summary><code>tv_overridemaster</code></summary>

Overrides the GOTV master root address.

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>tv_password</code></summary>

GOTV password for all clients

default: `""`  
flags: `0xa0120`  
</details>
<details>
<summary><code>tv_playcast_delay_prediction</code></summary>

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>tv_playcast_delay_resync</code></summary>

To alleviate intermittent network connectivity problems, this is the number of seconds to wait before actually re-syncing the stream after failure

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>tv_playcast_max_rcvage</code></summary>

default: `"15"`  
flags: `0x80010`  
</details>
<details>
<summary><code>tv_playcast_max_rtdelay</code></summary>

default: `"55"`  
flags: `0x80010`  
</details>
<details>
<summary><code>tv_playcast_origin_auth</code></summary>

Get request X-Origin-Auth string

default: `""`  
flags: `0x80010`  
</details>
<details>
<summary><code>tv_playcast_retry_timeout</code></summary>

In case of intermittent network problems, how long should playcast retry fragment retrieval before resorting to resync

default: `"12"`  
flags: `0x80000`  
</details>
<details>
<summary><code>tv_port</code></summary>

Host GOTV[0] port

default: `"27020"`  
flags: `0x80000`  
</details>
<details>
<summary><code>tv_port1</code></summary>

Host GOTV[1] port

default: `"27021"`  
flags: `0x80000`  
</details>
<details>
<summary><code>tv_relaypassword</code></summary>

GOTV password for relay proxies

default: `""`  
flags: `0xa0120`  
</details>
<details>
<summary><code>tv_relayvoice</code></summary>

Relay voice data: 0=off, 1=on

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>tv_snapshotrate</code></summary>

Snapshots broadcasted per second

default: `"32"`  
flags: `0x82000`  
</details>
<details>
<summary><code>tv_snapshotrate1</code></summary>

Snapshots broadcasted per second, GOTV[1]

default: `"32"`  
flags: `0x80000`  
</details>
<details>
<summary><code>tv_timeout</code></summary>

GOTV connection timeout in seconds.

default: `"30"`  
flags: `0x80000`  
</details>
<details>
<summary><code>tv_transmitall</code></summary>

Transmit all entities (not only director view)

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>vgui_drawtree</code></summary>

Draws the vgui panel hiearchy to the specified depth level.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>voice_caster_enable</code></summary>

Toggle voice transmit and receive for casters. 0 = no caster, account number of caster to enable.

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>voice_caster_scale</code></summary>

Caster Volume 0.0-1.0

default: `"1"`  
flags: `0x80`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>voice_enable</code></summary>

Toggle voice transmit and receive.

default: `"1"`  
flags: `0x80`  
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
<summary><code>voice_loopback</code></summary>

default: `"0"`  
flags: `0x200`  
</details>
<details>
<summary><code>voice_mixer_boost</code></summary>

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>voice_mixer_mute</code></summary>

Saves each speaker's voice data into separate .wav files


default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>voice_mixer_volume</code></summary>

default: `"1.0"`  
flags: `0x80`  
</details>
<details>
<summary><code>voice_positional</code></summary>

default: `"0"`  
flags: `0x80080`  
</details>
<details>
<summary><code>voice_positional_seconds_after_death</code></summary>

default: `"6.0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>voice_recordtofile</code></summary>

Record mic data and decompressed voice data into 'voice_micdata.wav' and 'voice_decompressed.wav'

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>voice_scale</code></summary>

Overall volume of voice over IP 0.0-1.0

default: `"1.0"`  
flags: `0x80080`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>voice_threshold</code></summary>

default: `"4000"`  
flags: `0x88`  
</details>
<details>
<summary><code>volume</code></summary>

Sound volume

default: `"1.0"`  
flags: `0x1000080`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>vprof_graphheight</code></summary>

default: `"256"`  
flags: `0x80`  
</details>
<details>
<summary><code>vprof_graphwidth</code></summary>

Draw the vprof graph.

default: `"512"`  
flags: `0x80`  
</details>
<details>
<summary><code>vprof_unaccounted_limit</code></summary>

number of milliseconds that a node must exceed to turn red in the vprof panel

default: `"0.3"`  
flags: `0x80`  
</details>
<details>
<summary><code>vprof_verbose</code></summary>

Set to one to show average and peak times

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>vprof_warningmsec</code></summary>

Above this many milliseconds render the label red to indicate slow code.

default: `"10"`  
flags: `0x80`  
</details>
<details>
<summary><code>xbox_arcade_remaining_trial_time</code></summary>

time remaining in trial mode

default: `"2700.0"`  
flags: `0x1008002`  
</details>

#### Addresses

```
engine.dll!0x00519be8 ConVar adsp_debug
engine.dll!0x005a0b28 ConVar budget_averages_window
engine.dll!0x005a0cf8 ConVar budget_background_alpha
engine.dll!0x005a0ad0 ConVar budget_bargraph_background_alpha
engine.dll!0x005a0bd8 ConVar budget_history_numsamplesvisible
engine.dll!0x005a0a20 ConVar budget_peaks_window
engine.dll!0x005a0b80 ConVar budget_show_averages
engine.dll!0x005a0c30 ConVar budget_show_history
engine.dll!0x005a0a78 ConVar budget_show_peaks
engine.dll!0x005954e0 ConVar bugreporter_uploadasync
engine.dll!0x00595320 ConVar bugreporter_username
engine.dll!0x00591b10 ConVar cl_allowdownload
engine.dll!0x0059e568 ConVar cl_allowupload
engine.dll!0x005957f8 ConVar cl_clock_correction
engine.dll!0x00595640 ConVar cl_clock_correction_adjustment_max_amount
engine.dll!0x00595698 ConVar cl_clock_correction_adjustment_max_offset
engine.dll!0x005957a0 ConVar cl_clock_correction_adjustment_min_offset
engine.dll!0x005956f0 ConVar cl_clock_correction_force_server_tick
engine.dll!0x00595748 ConVar cl_clock_showdebuginfo
engine.dll!0x00595590 ConVar cl_clockdrift_max_ms
engine.dll!0x005955e8 ConVar cl_clockdrift_max_ms_threadmode
engine.dll!0x00589c08 ConVar cl_color
engine.dll!0x0058c0f0 ConVar cl_debug_ugc_downloads
engine.dll!0x00589c60 ConVar cl_decryptdata_key
engine.dll!0x00589cb8 ConVar cl_decryptdata_key_pub
engine.dll!0x0058c098 ConVar cl_download_demoplayer
engine.dll!0x0058c040 ConVar cl_downloadfilter
engine.dll!0x0058b0c0 ConVar cl_entityreport
engine.dll!0x0058b220 ConVar cl_flushentitypacket
engine.dll!0x0058bfe8 ConVar cl_forcepreload
engine.dll!0x00589d10 ConVar cl_hideserverip
engine.dll!0x0058b350 ConVar cl_ignorepackets
engine.dll!0x0058ac98 ConVar cl_interpolate
engine.dll!0x00589b58 ConVar cl_interpolate
engine.dll!0x005899f8 ConVar cl_resend
engine.dll!0x00589a50 ConVar cl_resend_timeout
engine.dll!0x0058bba8 ConVar cl_showevents
engine.dll!0x0058bc00 ConVar cl_showpluginmessages2
engine.dll!0x0059e978 ConVar cl_skipslowpath
engine.dll!0x0058bf90 ConVar cl_timeout
engine.dll!0x0059d678 ConVar clientport
engine.dll!0x00597fe8 ConVar closecaption
engine.dll!0x0058c4d8 ConVar con_enable
engine.dll!0x0058c530 ConVar con_filter_enable
engine.dll!0x0058c588 ConVar con_filter_text
engine.dll!0x0058c5e0 ConVar con_filter_text_out
engine.dll!0x0058c3d0 ConVar con_notifytime
engine.dll!0x0058c2c8 ConVar con_timestamp
engine.dll!0x0058c378 ConVar con_trace
engine.dll!0x0058c428 ConVar contimes
engine.dll!0x00598670 ConVar coop
engine.dll!0x00598618 ConVar deathmatch
engine.dll!0x00595538 ConVar debug_map_crc
engine.dll!0x0058a158 ConVar demo_recordcommands
engine.dll!0x0058cf08 ConVar demo_strict_validation
engine.dll!0x00598828 ConVar developer
engine.dll!0x00596820 ConVar display_game_events
engine.dll!0x0051aa48 ConVar dsp_automatic
engine.dll!0x00519980 ConVar dsp_db_min
engine.dll!0x005199d8 ConVar dsp_db_mixdrop
engine.dll!0x00519878 ConVar dsp_dist_max
engine.dll!0x00519820 ConVar dsp_dist_min
engine.dll!0x0051ad08 ConVar dsp_enhance_stereo
engine.dll!0x0051a998 ConVar dsp_facingaway
engine.dll!0x00519928 ConVar dsp_mix_max
engine.dll!0x005198d0 ConVar dsp_mix_min
engine.dll!0x0051b258 ConVar dsp_off
engine.dll!0x0051b360 ConVar dsp_room
engine.dll!0x0051aaf8 ConVar dsp_slow_cpu
engine.dll!0x0051b2b0 ConVar dsp_spatial
engine.dll!0x0051a9f0 ConVar dsp_speaker
engine.dll!0x0051acb0 ConVar dsp_vol_2ch
engine.dll!0x0051ac58 ConVar dsp_vol_4ch
engine.dll!0x0051ac00 ConVar dsp_vol_5ch
engine.dll!0x0051aba8 ConVar dsp_volume
engine.dll!0x0051a940 ConVar dsp_water
engine.dll!0x0058ca18 ConVar enable_debug_overlays
engine.dll!0x0059f838 ConVar engine_no_focus_sleep
engine.dll!0x0058da40 ConVar fog_enable_water_fog
engine.dll!0x0051c3b0 ConVar force_audio_english
engine.dll!0x0059f708 ConVar fps_max_menu
engine.dll!0x0058b7c0 ConVar fps_screenshot_frequency
engine.dll!0x0058b768 ConVar fps_screenshot_threshold
engine.dll!0x00592858 ConVar host_flush_threshold
engine.dll!0x005987d0 ConVar host_framerate
engine.dll!0x00599170 ConVar host_info_show
engine.dll!0x00599220 ConVar host_map
engine.dll!0x00599118 ConVar host_name_store
engine.dll!0x005990c0 ConVar host_players_show
engine.dll!0x00599068 ConVar host_rules_show
engine.dll!0x005979e8 ConVar host_sleep
engine.dll!0x00598880 ConVar host_timescale
engine.dll!0x0059d598 ConVar hostip
engine.dll!0x0059d3e0 ConVar hostport
engine.dll!0x0059c148 ConVar in_forceuser
engine.dll!0x0059da00 ConVar ip
engine.dll!0x0059dc00 ConVar ip_relay
engine.dll!0x0059d490 ConVar ip_steam
engine.dll!0x0059dc58 ConVar ip_tv
engine.dll!0x0059d7c8 ConVar ip_tv1
engine.dll!0x0059c6b0 ConVar lightcache_maxmiss
engine.dll!0x0059c600 ConVar mat_ambient_light_b
engine.dll!0x0059c308 ConVar mat_ambient_light_g
engine.dll!0x0059c658 ConVar mat_ambient_light_r
engine.dll!0x0058f478 ConVar mat_bumpbasis
engine.dll!0x0058c170 ConVar mat_colorcorrection
engine.dll!0x0058ef00 ConVar mat_debugalttab
engine.dll!0x0058f218 ConVar mat_depthbias_normal
engine.dll!0x00589de8 ConVar mat_dynamic_tonemapping
engine.dll!0x00589ef0 ConVar mat_force_tonemap_scale
engine.dll!0x0058d8e0 ConVar mat_forcedynamic
engine.dll!0x0058f840 ConVar mat_fullbright
engine.dll!0x0058ffd8 ConVar mat_loadtextures
engine.dll!0x0058f790 ConVar mat_luxels
engine.dll!0x0058efb0 ConVar mat_monitorgamma
engine.dll!0x0058f110 ConVar mat_monitorgamma_tv_enabled
engine.dll!0x0058f7e8 ConVar mat_norendering
engine.dll!0x0058f6e0 ConVar mat_normals
engine.dll!0x0058beb8 ConVar mat_show_texture_memory_usage
engine.dll!0x0058e680 ConVar mat_softwareskin
engine.dll!0x0058dd60 ConVar mat_surfaceid
engine.dll!0x0058ddb8 ConVar mat_surfacemat
engine.dll!0x0058bd10 ConVar mat_texture_list
engine.dll!0x0058bdb0 ConVar mat_texture_list_all
engine.dll!0x0058be60 ConVar mat_texture_list_view
engine.dll!0x0058f738 ConVar mat_wireframe
engine.dll!0x00597b60 ConVar mem_incremental_compact_rate
engine.dll!0x0058fd10 ConVar mod_dynamicloadpause
engine.dll!0x0058fdc0 ConVar mod_dynamicloadspew
engine.dll!0x0058fd68 ConVar mod_dynamicloadthrottle
engine.dll!0x0058fcb8 ConVar mod_dynamicunloadtex
engine.dll!0x0058fc60 ConVar mod_dynamicunloadtime
engine.dll!0x0059cb58 ConVar net_blockmsg
engine.dll!0x0059d028 ConVar net_droponsendoverflow
engine.dll!0x0059d5f0 ConVar net_droppackets
engine.dll!0x0058b560 ConVar net_earliertempents
engine.dll!0x0059d8f8 ConVar net_fakejitter
engine.dll!0x0059d438 ConVar net_fakelag
engine.dll!0x0059dba8 ConVar net_fakeloss
engine.dll!0x0059dcb0 ConVar net_maxroutable
engine.dll!0x0059d950 ConVar net_public_adr
engine.dll!0x0059dd08 ConVar net_queue_trace
engine.dll!0x00591870 ConVar net_showreliablesounds
engine.dll!0x0059d540 ConVar net_showsplits
engine.dll!0x0059cd68 ConVar net_showudp
engine.dll!0x0059ca50 ConVar net_showudp_oob
engine.dll!0x0059d080 ConVar net_showudp_remoteonly
engine.dll!0x0059da58 ConVar net_splitrate
engine.dll!0x0059d280 ConVar net_steamcnx_allowrelay
engine.dll!0x0059d1a8 ConVar net_steamcnx_enabled
engine.dll!0x0059d150 ConVar net_threaded_socket_burst_cap
engine.dll!0x0059d200 ConVar net_threaded_socket_recovery_rate
engine.dll!0x0059d2d8 ConVar net_threaded_socket_recovery_time
engine.dll!0x005981a0 ConVar next
engine.dll!0x00595998 ConVar occlusion_old
engine.dll!0x00596588 ConVar occlusion_test_async_jitter
engine.dll!0x00596530 ConVar occlusion_test_async_move_tolerance
engine.dll!0x00596690 ConVar occlusion_test_jump_margin
engine.dll!0x005965e0 ConVar occlusion_test_margins
engine.dll!0x00596710 ConVar occlusion_test_shadow_max_distance
engine.dll!0x0059dfd0 ConVar paint_alpha_offset_enabled
engine.dll!0x0059df78 ConVar paint_max_surface_border_alpha
engine.dll!0x0059e130 ConVar paint_min_valid_alpha_value
engine.dll!0x0059e188 ConVar paintsplat_bias
engine.dll!0x0059e0d8 ConVar paintsplat_max_alpha_noise
engine.dll!0x0059e028 ConVar paintsplat_noise_enabled
engine.dll!0x005a3958 ConVar panel_test_title_safe
engine.dll!0x00589b00 ConVar password
engine.dll!0x005904a8 ConVar r_ClipAreaFrustums
engine.dll!0x00590450 ConVar r_ClipAreaPortals
engine.dll!0x00595cd8 ConVar r_DispBuildable
engine.dll!0x00595c80 ConVar r_DispWalkable
engine.dll!0x00589e98 ConVar r_DrawBeams
engine.dll!0x00595c28 ConVar r_DrawDisp
engine.dll!0x0058dfd0 ConVar r_DrawModelLightOrigin
engine.dll!0x00590660 ConVar r_DrawPortals
engine.dll!0x0058eb90 ConVar r_ambientfraction
engine.dll!0x0059c490 ConVar r_ambientlightingonly
engine.dll!0x0059c7b8 ConVar r_avglight
engine.dll!0x0058d338 ConVar r_avglightmap
engine.dll!0x005910b8 ConVar r_brush_queue_mode
engine.dll!0x0059e920 ConVar r_colorstaticprops
engine.dll!0x0058e2b8 ConVar r_debugrandomstaticlighting
engine.dll!0x0059ebc8 ConVar r_disable_static_prop_loading
engine.dll!0x0058d3e8 ConVar r_dlightsenable
engine.dll!0x0058f580 ConVar r_drawbrushmodels
engine.dll!0x00590978 ConVar r_drawdecals
engine.dll!0x0058e7e0 ConVar r_drawentities
engine.dll!0x0058d9e8 ConVar r_drawfuncdetail
engine.dll!0x0058d938 ConVar r_drawleaf
engine.dll!0x0058ec98 ConVar r_drawlightcache
engine.dll!0x0059c438 ConVar r_drawlightcache
engine.dll!0x0058d2e0 ConVar r_drawlightinfo
engine.dll!0x0058d288 ConVar r_drawlights
engine.dll!0x0058dec8 ConVar r_drawmodelstatsoverlay
engine.dll!0x0058df20 ConVar r_drawmodelstatsoverlaydistance
engine.dll!0x0058df78 ConVar r_drawmodelstatsoverlayfilter
engine.dll!0x0058eda0 ConVar r_drawmodelstatsoverlaymax
engine.dll!0x0058ed48 ConVar r_drawmodelstatsoverlaymin
engine.dll!0x0058de70 ConVar r_drawskybox
engine.dll!0x0059eb70 ConVar r_drawstaticprops
engine.dll!0x0058d888 ConVar r_drawtranslucentworld
engine.dll!0x005a05b0 ConVar r_drawvgui
engine.dll!0x0058d990 ConVar r_drawworld
engine.dll!0x00590920 ConVar r_dscale_basefov
engine.dll!0x005908c8 ConVar r_dscale_fardist
engine.dll!0x00590870 ConVar r_dscale_farscale
engine.dll!0x00590818 ConVar r_dscale_neardist
engine.dll!0x005907c0 ConVar r_dscale_nearscale
engine.dll!0x0058f688 ConVar r_dynamic
engine.dll!0x0058d6d0 ConVar r_dynamiclighting
engine.dll!0x0058e080 ConVar r_entity
engine.dll!0x0058e4c8 ConVar r_eyemove
engine.dll!0x0058e520 ConVar r_eyeshift_x
engine.dll!0x0058e578 ConVar r_eyeshift_y
engine.dll!0x0058e5d0 ConVar r_eyeshift_z
engine.dll!0x0058e628 ConVar r_eyesize
engine.dll!0x00590f98 ConVar r_flashlightclip
engine.dll!0x00590ff0 ConVar r_flashlightdrawclip
engine.dll!0x00591048 ConVar r_flashlightscissor
engine.dll!0x0058ea48 ConVar r_ignoreStaticColorChecksum
engine.dll!0x0058e368 ConVar r_itemblinkmax
engine.dll!0x0058e3c0 ConVar r_itemblinkrate
engine.dll!0x0059c2b0 ConVar r_lightcache_numambientsamples
engine.dll!0x0059c540 ConVar r_lightcache_radiusfactor
engine.dll!0x0059c3b8 ConVar r_lightcachecenter
engine.dll!0x0058ec40 ConVar r_lightcachemodel
engine.dll!0x0058e130 ConVar r_lightinterp
engine.dll!0x0058f5d8 ConVar r_lightmap
engine.dll!0x0058f630 ConVar r_lightstyle
engine.dll!0x0059c8d0 ConVar r_lockpvs
engine.dll!0x0058ecf0 ConVar r_modelAmbientMin
engine.dll!0x0058e940 ConVar r_modelwireframedecal
engine.dll!0x0058e6d8 ConVar r_nohw
engine.dll!0x0058e730 ConVar r_nosw
engine.dll!0x0059c820 ConVar r_novis
engine.dll!0x00590088 ConVar r_occlusionspew
engine.dll!0x0059c5a8 ConVar r_oldlightselection
engine.dll!0x0059e828 ConVar r_partition_level
engine.dll!0x005905b0 ConVar r_portalsopenall
engine.dll!0x0058e418 ConVar r_proplightingpooling
engine.dll!0x0059c360 ConVar r_radiosity
engine.dll!0x0059e8c8 ConVar r_shadow_deferred
engine.dll!0x00590b68 ConVar r_shadowids
engine.dll!0x00590ab8 ConVar r_shadows_gamecontrol
engine.dll!0x00590b10 ConVar r_shadowwireframe
engine.dll!0x0058e470 ConVar r_showenvcubemap
engine.dll!0x0058e8e8 ConVar r_skin
engine.dll!0x0058e9f0 ConVar r_slowpathwireframe
engine.dll!0x005901e8 ConVar r_visocclusion
engine.dll!0x0058d5c8 ConVar r_visualizelighttraces
engine.dll!0x0058d620 ConVar r_visualizelighttracesshowfulltrace
engine.dll!0x0058d570 ConVar r_visualizetraces
engine.dll!0x00591bc0 ConVar replay_debug
engine.dll!0x0051aaa0 ConVar room_type
engine.dll!0x0059fee0 ConVar rpt_vprof_time
engine.dll!0x005a1448 ConVar showbudget_texture
engine.dll!0x00598360 ConVar singlestep
engine.dll!0x00598778 ConVar skill
engine.dll!0x00519310 ConVar snd_deathcamera_volume
engine.dll!0x0051a6d8 ConVar snd_debug_panlaw
engine.dll!0x0051b998 ConVar snd_disable_mixer_duck
engine.dll!0x0051b940 ConVar snd_disable_mixer_solo
engine.dll!0x0051a280 ConVar snd_duckerattacktime
engine.dll!0x0051a2d8 ConVar snd_duckerreleasetime
engine.dll!0x0051a330 ConVar snd_duckerthreshold
engine.dll!0x0051a388 ConVar snd_ducking_off
engine.dll!0x0051a228 ConVar snd_ducktovolume
engine.dll!0x00519a88 ConVar snd_dvar_dist_max
engine.dll!0x00519a30 ConVar snd_dvar_dist_min
engine.dll!0x00519100 ConVar snd_dzmusic_volume
engine.dll!0x0051a178 ConVar snd_filter
engine.dll!0x0059e618 ConVar snd_foliage_db_loss
engine.dll!0x0059e778 ConVar snd_gain
engine.dll!0x0059e6c8 ConVar snd_gain_max
engine.dll!0x0059e5c0 ConVar snd_gain_min
engine.dll!0x00518a70 ConVar snd_hrtf_distance_behind
engine.dll!0x005187b0 ConVar snd_hrtf_lerp_max_distance
engine.dll!0x00518758 ConVar snd_hrtf_lerp_min_distance
engine.dll!0x0051c0f0 ConVar snd_hrtf_stereo_blend
engine.dll!0x0051c148 ConVar snd_hrtf_voice_delay
engine.dll!0x0051b470 ConVar snd_hrtf_volume
engine.dll!0x00518700 ConVar snd_hwcompat
engine.dll!0x0051b890 ConVar snd_list
engine.dll!0x00519260 ConVar snd_mapobjective_volume
engine.dll!0x00518b48 ConVar snd_max_same_sounds
engine.dll!0x00518ba0 ConVar snd_max_same_weapon_sounds
engine.dll!0x00519158 ConVar snd_menumusic_volume
engine.dll!0x005194c8 ConVar snd_mix_async
engine.dll!0x00519520 ConVar snd_mix_async_onetime_reset
engine.dll!0x005193c0 ConVar snd_mixahead
engine.dll!0x0051b7e0 ConVar snd_mixer_master_dsp
engine.dll!0x0051b788 ConVar snd_mixer_master_level
engine.dll!0x00519578 ConVar snd_music_volume_onetime_reset_2
engine.dll!0x00519050 ConVar snd_musicvolume_multiplier_inoverlay
engine.dll!0x005195d0 ConVar snd_mute_losefocus
engine.dll!0x00519368 ConVar snd_mvp_volume
engine.dll!0x00519b90 ConVar snd_obscured_gain_dB
engine.dll!0x0051c9e8 ConVar snd_occlusion_bounces
engine.dll!0x0051c7d8 ConVar snd_occlusion_collide_min_distance
engine.dll!0x0051c8e0 ConVar snd_occlusion_eq_high
engine.dll!0x0051c830 ConVar snd_occlusion_eq_low
engine.dll!0x0051c888 ConVar snd_occlusion_eq_mid
engine.dll!0x0051c780 ConVar snd_occlusion_indirect_max
engine.dll!0x0051c728 ConVar snd_occlusion_indirect_min
engine.dll!0x0051c6d0 ConVar snd_occlusion_indirect_radius
engine.dll!0x0051c5c8 ConVar snd_occlusion_material_override
engine.dll!0x0051c938 ConVar snd_occlusion_no_eq_scale
engine.dll!0x0051c570 ConVar snd_occlusion_pos_override
engine.dll!0x0051c990 ConVar snd_occlusion_rays
engine.dll!0x0051c620 ConVar snd_occlusion_visualize
engine.dll!0x0051c678 ConVar snd_occlusion_visualize_filter
engine.dll!0x0051c518 ConVar snd_op_test_convar
engine.dll!0x0051b4c8 ConVar snd_pause_all
engine.dll!0x00518fa0 ConVar snd_pitchquality
engine.dll!0x00519b38 ConVar snd_pre_gain_dist_falloff
engine.dll!0x0051c358 ConVar snd_prefetch_common
engine.dll!0x0051ab50 ConVar snd_profile
engine.dll!0x00518650 ConVar snd_rear_speaker_scale
engine.dll!0x0059e720 ConVar snd_refdist
engine.dll!0x005189c0 ConVar snd_report_format_sound
engine.dll!0x00518968 ConVar snd_report_loop_sound
engine.dll!0x005188b8 ConVar snd_report_start_sound
engine.dll!0x00518910 ConVar snd_report_stop_sound
engine.dll!0x00518a18 ConVar snd_report_verbose_error
engine.dll!0x00519208 ConVar snd_roundend_volume
engine.dll!0x005191b0 ConVar snd_roundstart_volume
engine.dll!0x00518de8 ConVar snd_show
engine.dll!0x00518e98 ConVar snd_show_filter
engine.dll!0x0051b838 ConVar snd_showclassname
engine.dll!0x0051b8e8 ConVar snd_showmixer
engine.dll!0x00519ae0 ConVar snd_showstart
engine.dll!0x0051cc50 ConVar snd_sos_list_operator_updates
engine.dll!0x0051a1d0 ConVar snd_sos_show_block_debug
engine.dll!0x005186a8 ConVar snd_sos_show_client_rcv
engine.dll!0x0051cf08 ConVar snd_sos_show_operator_entry_filter
engine.dll!0x0051cb48 ConVar snd_sos_show_operator_init
engine.dll!0x0051cbf8 ConVar snd_sos_show_operator_parse
engine.dll!0x0051ce58 ConVar snd_sos_show_operator_prestart
engine.dll!0x0051ce00 ConVar snd_sos_show_operator_shutdown
engine.dll!0x0051ceb0 ConVar snd_sos_show_operator_start
engine.dll!0x0051ca98 ConVar snd_sos_show_operator_stop_entry
engine.dll!0x0051cba0 ConVar snd_sos_show_operator_updates
engine.dll!0x0051caf0 ConVar snd_sos_show_queuetotrack
engine.dll!0x0051ccd0 ConVar snd_sos_show_startqueue
engine.dll!0x0051a730 ConVar snd_surround_speakers
engine.dll!0x005192b8 ConVar snd_tensecondwarning_volume
engine.dll!0x00518f48 ConVar snd_visualize
engine.dll!0x00519628 ConVar sound_device_override
engine.dll!0x00591660 ConVar spec_replay_enable
engine.dll!0x005973b8 ConVar spec_replay_leadup_time
engine.dll!0x005916b8 ConVar spec_replay_message_time
engine.dll!0x00591920 ConVar spec_replay_on_death
engine.dll!0x00591b68 ConVar spec_replay_rate_base
engine.dll!0x00591710 ConVar spec_replay_rate_limit
engine.dll!0x005918c8 ConVar sv_allow_legacy_cmd_execution_from_client
engine.dll!0x00594cc8 ConVar sv_allow_wait_command
engine.dll!0x00591558 ConVar sv_allowdownload
engine.dll!0x005915b0 ConVar sv_allowupload
engine.dll!0x00594e80 ConVar sv_alternateticks
engine.dll!0x00592cf8 ConVar sv_client_cmdrate_difference
engine.dll!0x00592da8 ConVar sv_client_max_interp_ratio
engine.dll!0x00592d50 ConVar sv_client_min_interp_ratio
engine.dll!0x00592e00 ConVar sv_client_predict
engine.dll!0x005932d8 ConVar sv_consistency
engine.dll!0x00592ae8 ConVar sv_contact
engine.dll!0x00591f90 ConVar sv_creationtickcheck
engine.dll!0x00593578 ConVar sv_debugmanualmode
engine.dll!0x00592bf0 ConVar sv_downloadurl
engine.dll!0x0059dd60 ConVar sv_dumpstringtables
engine.dll!0x00591138 ConVar sv_duplicate_playernames_ok
engine.dll!0x00593520 ConVar sv_enable_delta_packing
engine.dll!0x005937f0 ConVar sv_forcepreload
engine.dll!0x00592750 ConVar sv_hibernate_ms
engine.dll!0x005927a8 ConVar sv_hibernate_ms_vgui
engine.dll!0x00592800 ConVar sv_hibernate_postgame_delay
engine.dll!0x005926f8 ConVar sv_hibernate_punt_tv_clients
engine.dll!0x00595060 ConVar sv_hosting_lobby
engine.dll!0x00592a38 ConVar sv_lan
engine.dll!0x00592358 ConVar sv_log_onefile
engine.dll!0x005923b0 ConVar sv_logbans
engine.dll!0x00591fe8 ConVar sv_logblocks
engine.dll!0x00592300 ConVar sv_logecho
engine.dll!0x00592250 ConVar sv_logfile
engine.dll!0x005922a8 ConVar sv_logflush
engine.dll!0x005921f8 ConVar sv_logsdir
engine.dll!0x00592408 ConVar sv_logsecret
engine.dll!0x00592460 ConVar sv_logsocket2
engine.dll!0x005924b8 ConVar sv_logsocket2_substr
engine.dll!0x005917c0 ConVar sv_max_dropped_packets_to_process
engine.dll!0x005921a0 ConVar sv_max_queries_sec
engine.dll!0x00592098 ConVar sv_max_queries_sec_global
engine.dll!0x005920f0 ConVar sv_max_queries_tracked_ips_max
engine.dll!0x00592040 ConVar sv_max_queries_tracked_ips_prune
engine.dll!0x00592148 ConVar sv_max_queries_window
engine.dll!0x00592ca0 ConVar sv_maxcmdrate
engine.dll!0x00591348 ConVar sv_maxrate
engine.dll!0x005913f8 ConVar sv_maxupdaterate
engine.dll!0x00593048 ConVar sv_maxuptimelimit
engine.dll!0x00592f98 ConVar sv_memlimit
engine.dll!0x00592c48 ConVar sv_mincmdrate
engine.dll!0x005913a0 ConVar sv_minrate
engine.dll!0x00591450 ConVar sv_minupdaterate
engine.dll!0x00592ff0 ConVar sv_minuptimelimit
engine.dll!0x00593628 ConVar sv_parallel_packentities
engine.dll!0x005930c8 ConVar sv_parallel_sendsnapshot
engine.dll!0x00592a90 ConVar sv_pausable
engine.dll!0x00592988 ConVar sv_pure_consensus
engine.dll!0x005931d0 ConVar sv_pure_kick_clients
engine.dll!0x005929e0 ConVar sv_pure_retiretime
engine.dll!0x00593228 ConVar sv_pure_trace
engine.dll!0x00591818 ConVar sv_quota_stringcmdspersecond
engine.dll!0x00593920 ConVar sv_rcon_whitelist_address
engine.dll!0x00595168 ConVar sv_region
engine.dll!0x00591298 ConVar sv_reliableavatardata
engine.dll!0x00591500 ConVar sv_replaybots
engine.dll!0x00594f30 ConVar sv_reservation_tickrate_adjustment
engine.dll!0x005950b8 ConVar sv_reservation_timeout
engine.dll!0x00591cf0 ConVar sv_show_cull_props
engine.dll!0x00593b30 ConVar sv_steamauth_enforce
engine.dll!0x00594c70 ConVar sv_steamgroup_exclusive
engine.dll!0x005914a8 ConVar sv_stressbots
engine.dll!0x0058b2a0 ConVar sv_unlockedchapters
engine.dll!0x005934c8 ConVar sv_validate_edict_change_infos
engine.dll!0x00594dd0 ConVar sv_visiblemaxplayers
engine.dll!0x00519ee8 ConVar sv_voice_proximity_minvolume
engine.dll!0x00589468 ConVar sv_voice_proximity_use_falloff
engine.dll!0x00593398 ConVar sv_voicecodec
engine.dll!0x00592b98 ConVar sv_voiceenable
engine.dll!0x0059f410 ConVar sys_minidumpspewlines
engine.dll!0x005a1740 ConVar texture_budget_background_alpha
engine.dll!0x0059e7d0 ConVar think_trace_limit
engine.dll!0x005971b0 ConVar tv_advertise_watchable
engine.dll!0x005935d0 ConVar tv_allow_camera_man_override
engine.dll!0x00597840 ConVar tv_autorecord
engine.dll!0x00596ee0 ConVar tv_autoretry
engine.dll!0x00596af0 ConVar tv_broadcast_drop_fragments
engine.dll!0x00596c20 ConVar tv_broadcast_keyframe_interval
engine.dll!0x00596a40 ConVar tv_broadcast_keyframe_interval1
engine.dll!0x00596ba0 ConVar tv_broadcast_max_requests
engine.dll!0x00596b48 ConVar tv_broadcast_max_requests1
engine.dll!0x00596a98 ConVar tv_broadcast_origin_auth
engine.dll!0x005969e8 ConVar tv_broadcast_origin_auth1
engine.dll!0x00596c78 ConVar tv_broadcast_startup_resend_interval
engine.dll!0x00596990 ConVar tv_broadcast_terminate
engine.dll!0x005976b8 ConVar tv_broadcast_url
engine.dll!0x00597738 ConVar tv_broadcast_url1
engine.dll!0x00596cd0 ConVar tv_chatgroupsize
engine.dll!0x00596d80 ConVar tv_chattimelimit
engine.dll!0x00597790 ConVar tv_debug
engine.dll!0x00597638 ConVar tv_deltacache
engine.dll!0x00597158 ConVar tv_dispatchmode
engine.dll!0x00597230 ConVar tv_dispatchweight
engine.dll!0x005974b0 ConVar tv_enable_delta_frames
engine.dll!0x00596fb8 ConVar tv_encryptdata_key
engine.dll!0x00597508 ConVar tv_encryptdata_key_pub
engine.dll!0x00597898 ConVar tv_maxclients
engine.dll!0x005978f0 ConVar tv_maxclients_relayreserved
engine.dll!0x00596dd8 ConVar tv_maxrate
engine.dll!0x0058b2f8 ConVar tv_nochat
engine.dll!0x00597288 ConVar tv_overridemaster
engine.dll!0x00597588 ConVar tv_password
engine.dll!0x0058d068 ConVar tv_playcast_delay_prediction
engine.dll!0x0058a0d8 ConVar tv_playcast_delay_resync
engine.dll!0x0058d118 ConVar tv_playcast_max_rcvage
engine.dll!0x0058d010 ConVar tv_playcast_max_rtdelay
engine.dll!0x0058cf60 ConVar tv_playcast_origin_auth
engine.dll!0x0058cfb8 ConVar tv_playcast_retry_timeout
engine.dll!0x0059d9a8 ConVar tv_port
engine.dll!0x0059d878 ConVar tv_port1
engine.dll!0x00596d28 ConVar tv_relaypassword
engine.dll!0x00597080 ConVar tv_relayvoice
engine.dll!0x00596f38 ConVar tv_snapshotrate
engine.dll!0x00596e30 ConVar tv_snapshotrate1
engine.dll!0x00596e88 ConVar tv_timeout
engine.dll!0x00597948 ConVar tv_transmitall
engine.dll!0x005a0fb8 ConVar vgui_drawtree
engine.dll!0x0051d328 ConVar voice_caster_enable
engine.dll!0x0051d118 ConVar voice_caster_scale
engine.dll!0x0051d2d0 ConVar voice_enable
engine.dll!0x0051d3d8 ConVar voice_forcemicrecord
engine.dll!0x00598b20 ConVar voice_inputfromfile
engine.dll!0x0051cf60 ConVar voice_loopback
engine.dll!0x005893b8 ConVar voice_mixer_boost
engine.dll!0x00589360 ConVar voice_mixer_mute
engine.dll!0x00589410 ConVar voice_mixer_volume
engine.dll!0x005894c0 ConVar voice_positional
engine.dll!0x00519f40 ConVar voice_positional_seconds_after_death
engine.dll!0x00598c98 ConVar voice_recordtofile
engine.dll!0x0051d0c0 ConVar voice_scale
engine.dll!0x0051d380 ConVar voice_threshold
engine.dll!0x00518ff8 ConVar volume
engine.dll!0x005a1848 ConVar vprof_graphheight
engine.dll!0x005a17f0 ConVar vprof_graphwidth
engine.dll!0x005a1988 ConVar vprof_unaccounted_limit
engine.dll!0x005a1930 ConVar vprof_verbose
engine.dll!0x005a19e0 ConVar vprof_warningmsec
engine.dll!0x005a0438 ConVar xbox_arcade_remaining_trial_time
```

### ConCommands

<details>
<summary><code>+mat_texture_list</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+showbudget</code></summary>



flags: `0x4000`  
</details>
<details>
<summary><code>+showbudget_texture</code></summary>



flags: `0x4000`  
</details>
<details>
<summary><code>+showbudget_texture_global</code></summary>



flags: `0x4000`  
</details>
<details>
<summary><code>+showvprof</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+vgui_drawtree</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+voicerecord</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-mat_texture_list</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-showbudget</code></summary>



flags: `0x4000`  
</details>
<details>
<summary><code>-showbudget_texture</code></summary>



flags: `0x4000`  
</details>
<details>
<summary><code>-showbudget_texture_global</code></summary>



flags: `0x4000`  
</details>
<details>
<summary><code>-showvprof</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-vgui_drawtree</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-voicerecord</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>BindToggle</code></summary>

Performs a bind <key> "increment var <cvar> 0 1 1"

flags: `0x0`  
</details>
<details>
<summary><code>Test_Loop</code></summary>

Test_Loop <loop name> - loop back to the specified loop start point unconditionally.

flags: `0x4000`  
</details>
<details>
<summary><code>Test_LoopCount</code></summary>

Test_LoopCount <loop name> <count> - loop back to the specified loop start point the specified # of times.

flags: `0x4000`  
</details>
<details>
<summary><code>Test_LoopForNumSeconds</code></summary>

Test_LoopForNumSeconds <loop name> <time> - loop back to the specified start point for the specified # of seconds.

flags: `0x4000`  
</details>
<details>
<summary><code>Test_RandomChance</code></summary>

Test_RandomChance <percent chance, 0-100> <token1> <token2...> - Roll the dice and maybe run the command following the percentage chance.

flags: `0x4000`  
</details>
<details>
<summary><code>Test_RunFrame</code></summary>



flags: `0x4000`  
</details>
<details>
<summary><code>Test_SendKey</code></summary>



flags: `0x4000`  
</details>
<details>
<summary><code>Test_StartLoop</code></summary>

Test_StartLoop <loop name> - Denote the start of a loop. Really just defines a named point you can jump to.

flags: `0x4000`  
</details>
<details>
<summary><code>Test_StartScript</code></summary>

Start a test script running..

flags: `0x4000`  
</details>
<details>
<summary><code>Test_Wait</code></summary>



flags: `0x4000`  
</details>
<details>
<summary><code>Test_WaitForCheckPoint</code></summary>



flags: `0x4000`  
</details>
<details>
<summary><code>TransmitEvents</code></summary>

Transmits Game Events to <address:port>

flags: `0x2`  
</details>
<details>
<summary><code>_autosave</code></summary>

Autosave

flags: `0x0`  
</details>
<details>
<summary><code>_autosavedangerous</code></summary>

AutoSaveDangerous

flags: `0x0`  
</details>
<details>
<summary><code>_bugreporter_restart</code></summary>

Restarts bug reporter .dll

flags: `0x0`  
</details>
<details>
<summary><code>_record</code></summary>

Record a demo incrementally.

flags: `0x20000`  
</details>
<details>
<summary><code>_restart</code></summary>

Shutdown and restart the engine.

flags: `0x0`  
</details>
<details>
<summary><code>addip</code></summary>

Add an IP address to the ban list.

flags: `0x0`  
</details>
<details>
<summary><code>adsp_reset_nodes</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>alias</code></summary>

Alias a command.

flags: `0x0`  
</details>
<details>
<summary><code>askconnect_accept</code></summary>

Accept a redirect request by the server.

flags: `0x20000`  
</details>
<details>
<summary><code>asw_engine_finished_building_map</code></summary>

Notify engine that we've finished building a map

flags: `0x0`  
</details>
<details>
<summary><code>audit_save_in_memory</code></summary>

Audit the memory usage and files in the save-to-memory system

flags: `0x0`  
</details>
<details>
<summary><code>autosave</code></summary>

Autosave

flags: `0x0`  
</details>
<details>
<summary><code>autosavedangerous</code></summary>

AutoSaveDangerous

flags: `0x0`  
</details>
<details>
<summary><code>autosavedangerousissafe</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>banid</code></summary>

Add a user ID to the ban list.

flags: `0x0`  
</details>
<details>
<summary><code>banip</code></summary>

Add an IP address to the ban list.

flags: `0x0`  
</details>
<details>
<summary><code>bench_end</code></summary>

Ends gathering of info.

flags: `0x4000`  
</details>
<details>
<summary><code>bench_start</code></summary>

Starts gathering of info. Arguments: filename to write results into

flags: `0x4000`  
</details>
<details>
<summary><code>bench_upload</code></summary>

Uploads most recent benchmark stats to the Valve servers.

flags: `0x4000`  
</details>
<details>
<summary><code>benchframe</code></summary>

Takes a snapshot of a particular frame in a time demo.

flags: `0x0`  
</details>
<details>
<summary><code>bind</code></summary>

Bind a key.

flags: `0x0`  
</details>
<details>
<summary><code>bind_osx</code></summary>

Bind a key for OSX only.

flags: `0x0`  
</details>
<details>
<summary><code>blackbox_dump</code></summary>

Dump the contents of the blackbox

flags: `0x24002`  
</details>
<details>
<summary><code>blackbox_record</code></summary>

Record an entry into the blackbox

flags: `0x20000`  
</details>
<details>
<summary><code>box</code></summary>

Draw a debug box.

flags: `0x4000`  
</details>
<details>
<summary><code>budget_toggle_group</code></summary>

Turn a budget group on/off

flags: `0x0`  
</details>
<details>
<summary><code>bug</code></summary>

Show the bug reporting UI.

flags: `0x20000`  
</details>
<details>
<summary><code>buildcubemaps</code></summary>

Rebuild cubemaps.

flags: `0x0`  
</details>
<details>
<summary><code>buildmodelforworld</code></summary>

buildmodelforworld

flags: `0x0`  
</details>
<details>
<summary><code>cache_print</code></summary>

cache_print [section]
Print out contents of cache memory.

flags: `0x0`  
</details>
<details>
<summary><code>cache_print_lru</code></summary>

cache_print_lru [section]
Print out contents of cache memory.

flags: `0x0`  
</details>
<details>
<summary><code>cache_print_summary</code></summary>

cache_print_summary [section]
Print out a summary contents of cache memory.

flags: `0x0`  
</details>
<details>
<summary><code>changelevel</code></summary>

Change server to the specified map

flags: `0x20000`  
</details>
<details>
<summary><code>changelevel2</code></summary>

Transition to the specified map in single player

flags: `0x20000`  
</details>
<details>
<summary><code>cl_fullupdate</code></summary>

Forces the server to send a full update packet

flags: `0x4000`  
</details>
<details>
<summary><code>cl_precacheinfo</code></summary>

Show precache info (client).

flags: `0x0`  
</details>
<details>
<summary><code>cl_showents</code></summary>

Dump entity list to console.

flags: `0x4000`  
</details>
<details>
<summary><code>cl_view</code></summary>

Set the view entity index.

flags: `0x4000`  
</details>
<details>
<summary><code>clear</code></summary>

Clear all console output.

flags: `0x20000`  
</details>
<details>
<summary><code>cmd</code></summary>

Forward command to server.

flags: `0x0`  
</details>
<details>
<summary><code>cmd1</code></summary>

sets userinfo string for split screen player in slot 1

flags: `0x0`  
</details>
<details>
<summary><code>cmd2</code></summary>

sets userinfo string for split screen player in slot 2

flags: `0x0`  
</details>
<details>
<summary><code>cmd3</code></summary>

sets userinfo string for split screen player in slot 3

flags: `0x0`  
</details>
<details>
<summary><code>cmd4</code></summary>

sets userinfo string for split screen player in slot 4

flags: `0x0`  
</details>
<details>
<summary><code>colorcorrectionui</code></summary>

Show/hide the color correction tools UI.

flags: `0x4000`  
</details>
<details>
<summary><code>con_min_severity</code></summary>

Minimum severity level for messages sent to any logging channel: LS_MESSAGE=0, LS_WARNING=1, LS_ASSERT=2, LS_ERROR=3.

flags: `0x40000000`  
</details>
<details>
<summary><code>connect</code></summary>

Connect to specified server.

flags: `0x20000`  
</details>
<details>
<summary><code>connect_splitscreen</code></summary>

Connect to specified server. With multiple players.

flags: `0xa0010`  
</details>
<details>
<summary><code>crash</code></summary>

Cause the engine to crash (Debug!!)

flags: `0x4000`  
</details>
<details>
<summary><code>cvarlist</code></summary>

Show the list of convars/concommands.

flags: `0x0`  
</details>
<details>
<summary><code>debug_drawbox</code></summary>

Create debug box

flags: `0x4000`  
</details>
<details>
<summary><code>debug_drawdisp_boundbox</code></summary>

Create debug boxes for invalid displacements

flags: `0x4000`  
</details>
<details>
<summary><code>demo_goto</code></summary>

Skips to location in demo.

flags: `0x0`  
</details>
<details>
<summary><code>demo_gototick</code></summary>

Skips to a tick in demo.

flags: `0x0`  
</details>
<details>
<summary><code>demo_info</code></summary>

Print information about currently playing demo.

flags: `0x0`  
</details>
<details>
<summary><code>demo_listhighlights</code></summary>

List all highlights data for the demo.

flags: `0x0`  
</details>
<details>
<summary><code>demo_listimportantticks</code></summary>

List all important ticks in the demo.

flags: `0x0`  
</details>
<details>
<summary><code>demo_pause</code></summary>

Pauses demo playback.

flags: `0x0`  
</details>
<details>
<summary><code>demo_resume</code></summary>

Resumes demo playback.

flags: `0x0`  
</details>
<details>
<summary><code>demo_timescale</code></summary>

Sets demo replay speed.

flags: `0x0`  
</details>
<details>
<summary><code>demo_togglepause</code></summary>

Toggles demo playback.

flags: `0x0`  
</details>
<details>
<summary><code>demolist</code></summary>

Print demo sequence list.

flags: `0x0`  
</details>
<details>
<summary><code>demos</code></summary>

Demo demo file sequence.

flags: `0x0`  
</details>
<details>
<summary><code>demoui</code></summary>

Show/hide the demo player UI.

flags: `0x20000`  
</details>
<details>
<summary><code>devshots_nextmap</code></summary>

Used by the devshots system to go to the next map in the devshots maplist.

flags: `0x0`  
</details>
<details>
<summary><code>devshots_screenshot</code></summary>

Used by the -makedevshots system to take a screenshot. For taking your own screenshots, use the 'screenshot' command instead.

flags: `0x20000`  
</details>
<details>
<summary><code>differences</code></summary>

Show all convars which are not at their default values.

flags: `0x0`  
</details>
<details>
<summary><code>disconnect</code></summary>

Disconnect game from server.

flags: `0x10000000`  
</details>
<details>
<summary><code>disp_list_all_collideable</code></summary>

List all collideable displacements

flags: `0x0`  
</details>
<details>
<summary><code>display_elapsedtime</code></summary>

Displays how much time has elapsed since the game started

flags: `0x4000`  
</details>
<details>
<summary><code>dsp_reload</code></summary>



flags: `0x4000`  
</details>
<details>
<summary><code>dti_flush</code></summary>

Write out the datatable instrumentation files (you must run with -dti for this to work).

flags: `0x0`  
</details>
<details>
<summary><code>dumpstringtables</code></summary>

Print string tables to console.

flags: `0x0`  
</details>
<details>
<summary><code>echo</code></summary>

Echo text to console.

flags: `0x10000000`  
</details>
<details>
<summary><code>editdemo</code></summary>

Edit a recorded demo file (.dem ).

flags: `0x0`  
</details>
<details>
<summary><code>editor_toggle</code></summary>

Disables the simulation and returns focus to the editor

flags: `0x4000`  
</details>
<details>
<summary><code>endmovie</code></summary>

Stop recording movie frames.

flags: `0x20000`  
</details>
<details>
<summary><code>envmap</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>escape</code></summary>

Escape key pressed.

flags: `0x40000000`  
</details>
<details>
<summary><code>exec</code></summary>

Execute script file.

flags: `0x0`  
</details>
<details>
<summary><code>execifexists</code></summary>

Execute script file if file exists.

flags: `0x0`  
</details>
<details>
<summary><code>execwithwhitelist</code></summary>

Execute script file, only execing convars on a whitelist.

flags: `0x0`  
</details>
<details>
<summary><code>exit</code></summary>

Exit the engine.

flags: `0x0`  
</details>
<details>
<summary><code>findflags</code></summary>

Find concommands by flags.

flags: `0x0`  
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
<summary><code>fogui</code></summary>

Show/hide fog control UI.

flags: `0x20000`  
</details>
<details>
<summary><code>forcebind</code></summary>

Bind a command to an available key. (forcebind command opt:suggestedKey)

flags: `0x0`  
</details>
<details>
<summary><code>fs_printopenfiles</code></summary>

Show all files currently opened by the engine.

flags: `0x0`  
</details>
<details>
<summary><code>fs_syncdvddevcache</code></summary>

Force the 360 to get updated files that are in your p4 changelist(s) from the host PC when running with -dvddev.

flags: `0x0`  
</details>
<details>
<summary><code>fs_warning_level</code></summary>

Set the filesystem warning level.

flags: `0x0`  
</details>
<details>
<summary><code>gameui_activate</code></summary>

Shows the game UI

flags: `0x0`  
</details>
<details>
<summary><code>gameui_allowescape</code></summary>

Escape key allowed to hide game UI

flags: `0x0`  
</details>
<details>
<summary><code>gameui_allowescapetoshow</code></summary>

Escape key allowed to show game UI

flags: `0x0`  
</details>
<details>
<summary><code>gameui_hide</code></summary>

Hides the game UI

flags: `0x0`  
</details>
<details>
<summary><code>gameui_preventescape</code></summary>

Escape key doesn't hide game UI

flags: `0x0`  
</details>
<details>
<summary><code>gameui_preventescapetoshow</code></summary>

Escape key doesn't show game UI

flags: `0x0`  
</details>
<details>
<summary><code>heartbeat</code></summary>

Force heartbeat of master servers

flags: `0x0`  
</details>
<details>
<summary><code>help</code></summary>

Find help about a convar/concommand.

flags: `0x0`  
</details>
<details>
<summary><code>hideconsole</code></summary>

Hide the console.

flags: `0x20000`  
</details>
<details>
<summary><code>hltv_replay_status</code></summary>

Show Killer Replay status and some statistics, works on listen or dedicated server.

flags: `0x0`  
</details>
<details>
<summary><code>host_filtered_time_report</code></summary>

Dumps time spent idle in previous frames in ms(dedicated only).

flags: `0x0`  
</details>
<details>
<summary><code>host_reset_config</code></summary>

reset config (for testing) with param as splitscreen index.

flags: `0x0`  
</details>
<details>
<summary><code>host_runofftime</code></summary>

Run off some time without rendering/updating sounds


flags: `0x0`  
</details>
<details>
<summary><code>host_timer_report</code></summary>

Spew CPU timer jitter for the last 128 frames in microseconds (dedicated only)

flags: `0x0`  
</details>
<details>
<summary><code>host_writeconfig</code></summary>

Store current settings to config.cfg (or specified .cfg file).

flags: `0x0`  
</details>
<details>
<summary><code>host_writeconfig_ss</code></summary>

Store current settings to config.cfg (or specified .cfg file) with first param as splitscreen index.

flags: `0x0`  
</details>
<details>
<summary><code>incrementvar</code></summary>

Increment specified convar value.

flags: `0x20000`  
</details>
<details>
<summary><code>ipc_console_disable</code></summary>

Disable IPC console(s)

flags: `0x40020000`  
</details>
<details>
<summary><code>ipc_console_disable_all</code></summary>

Disable all IPC consoles

flags: `0x40020000`  
</details>
<details>
<summary><code>ipc_console_enable</code></summary>

Enable IPC console

flags: `0x40024000`  
</details>
<details>
<summary><code>ipc_console_show</code></summary>

Show status of IPC consoles

flags: `0x40020000`  
</details>
<details>
<summary><code>jpeg</code></summary>

Take a jpeg screenshot:  jpeg <filename> <quality 1-100>.

flags: `0x0`  
</details>
<details>
<summary><code>key_findbinding</code></summary>

Find key bound to specified command string.

flags: `0x0`  
</details>
<details>
<summary><code>key_listboundkeys</code></summary>

List bound keys with bindings.

flags: `0x0`  
</details>
<details>
<summary><code>kick</code></summary>

Kick a player by name.

flags: `0x0`  
</details>
<details>
<summary><code>kickid</code></summary>

Kick a player by userid or uniqueid, with a message.

flags: `0x0`  
</details>
<details>
<summary><code>kickid_ex</code></summary>

Kick a player by userid or uniqueid, provide a force-the-kick flag and also assign a message.

flags: `0x0`  
</details>
<details>
<summary><code>killserver</code></summary>

Shutdown the server.

flags: `0x0`  
</details>
<details>
<summary><code>light_crosshair</code></summary>

Show texture color at crosshair

flags: `0x4000`  
</details>
<details>
<summary><code>lightprobe</code></summary>

Samples the lighting environment.
Creates a cubemap and a file indicating the local lighting in a subdirectory called 'materials/lightprobes'
.The lightprobe command requires you specify a base file name.


flags: `0x0`  
</details>
<details>
<summary><code>linefile</code></summary>

Parses map leak data from .lin file

flags: `0x4000`  
</details>
<details>
<summary><code>listdemo</code></summary>

List demo file contents.

flags: `0x0`  
</details>
<details>
<summary><code>listid</code></summary>

Lists banned users.

flags: `0x0`  
</details>
<details>
<summary><code>listip</code></summary>

List IP addresses on the ban list.

flags: `0x0`  
</details>
<details>
<summary><code>listmodels</code></summary>

List loaded models.

flags: `0x0`  
</details>
<details>
<summary><code>load</code></summary>

Load a saved game.

flags: `0x0`  
</details>
<details>
<summary><code>log</code></summary>

Enables logging to file, console, and udp < on | off >.

flags: `0x0`  
</details>
<details>
<summary><code>log_color</code></summary>

Set the color of a logging channel.

flags: `0x20000`  
</details>
<details>
<summary><code>log_dumpchannels</code></summary>

Dumps information about all logging channels.

flags: `0x20000`  
</details>
<details>
<summary><code>log_flags</code></summary>

Set the flags on a logging channel.

flags: `0x20000`  
</details>
<details>
<summary><code>log_level</code></summary>

Set the spew level of a logging channel.

flags: `0x20000`  
</details>
<details>
<summary><code>logaddress_add</code></summary>

Set address and port for remote host <ip:port>.

flags: `0x0`  
</details>
<details>
<summary><code>logaddress_add_ex</code></summary>

Set address and port for remote host <ip:port> and supplies a unique token in the UDP packets.

flags: `0x0`  
</details>
<details>
<summary><code>logaddress_add_ts</code></summary>

Set address and port for remote host <ip:port> and uses a unique checksum from logaddress_token_secret in the UDP packets.

flags: `0x0`  
</details>
<details>
<summary><code>logaddress_del</code></summary>

Remove address and port for remote host <ip:port>.

flags: `0x0`  
</details>
<details>
<summary><code>logaddress_delall</code></summary>

Remove all udp addresses being logged to

flags: `0x0`  
</details>
<details>
<summary><code>logaddress_list</code></summary>

List all addresses currently being used by logaddress.

flags: `0x0`  
</details>
<details>
<summary><code>map</code></summary>

Start playing on specified map.

flags: `0x20000`  
</details>
<details>
<summary><code>map_background</code></summary>

Runs a map as the background to the main menu.

flags: `0x20000`  
</details>
<details>
<summary><code>map_commentary</code></summary>

Start playing, with commentary, on a specified map.

flags: `0x20000`  
</details>
<details>
<summary><code>map_edit</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>mapgroup</code></summary>

Specify a map group

flags: `0x20000`  
</details>
<details>
<summary><code>maps</code></summary>

Displays list of maps.

flags: `0x0`  
</details>
<details>
<summary><code>mat_configcurrent</code></summary>

show the current video control panel config for the material system

flags: `0x0`  
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
<summary><code>mat_debug</code></summary>

Activates debugging spew for a specific material.

flags: `0x4002`  
</details>
<details>
<summary><code>mat_edit</code></summary>

Bring up the material under the crosshair in the editor

flags: `0x4000`  
</details>
<details>
<summary><code>mat_info</code></summary>

Shows material system info

flags: `0x0`  
</details>
<details>
<summary><code>mat_savechanges</code></summary>

saves current video configuration to the registry

flags: `0x0`  
</details>
<details>
<summary><code>mat_setvideomode</code></summary>

sets the width, height, windowed state of the material system

flags: `0x0`  
</details>
<details>
<summary><code>mat_suppress</code></summary>

Suppress a material from drawing

flags: `0x4002`  
</details>
<details>
<summary><code>mat_texture_list_txlod</code></summary>

Adjust LOD of the last viewed texture +1 to inc resolution, -1 to dec resolution

flags: `0x20000`  
</details>
<details>
<summary><code>mat_updateconvars</code></summary>

updates the video config convars

flags: `0x0`  
</details>
<details>
<summary><code>maxplayers</code></summary>

Change the maximum number of players allowed on this server.

flags: `0x0`  
</details>
<details>
<summary><code>mem_compact</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>mem_dump</code></summary>

Dump memory stats to text file.

flags: `0x0`  
</details>
<details>
<summary><code>mem_eat</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>mem_incremental_compact</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>mem_test</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>mem_vcollide</code></summary>

Dumps the memory used by vcollides

flags: `0x0`  
</details>
<details>
<summary><code>mem_verify</code></summary>

Verify the validity of the heap

flags: `0x0`  
</details>
<details>
<summary><code>memory</code></summary>

Print memory stats.

flags: `0x0`  
</details>
<details>
<summary><code>minisave</code></summary>

Saves game (for current level only!)

flags: `0x20000`  
</details>
<details>
<summary><code>mod_DumpWeaponWiewModelCache</code></summary>

Dumps the weapon view model cache contents

flags: `0x0`  
</details>
<details>
<summary><code>mod_DumpWeaponWorldModelCache</code></summary>

Dumps the weapon world model cache contents

flags: `0x0`  
</details>
<details>
<summary><code>mod_combiner_info</code></summary>

debug spew for Combiner Info

flags: `0x0`  
</details>
<details>
<summary><code>mod_dynamicmodeldebug</code></summary>

debug spew for dynamic model loading

flags: `0x20010`  
</details>
<details>
<summary><code>movie_fixwave</code></summary>

Fixup corrupted .wav file if engine crashed during startmovie/endmovie, etc.

flags: `0x0`  
</details>
<details>
<summary><code>multvar</code></summary>

Multiply specified convar value.

flags: `0x20000`  
</details>
<details>
<summary><code>net_channels</code></summary>

Shows net channel info

flags: `0x0`  
</details>
<details>
<summary><code>net_connections_stats</code></summary>

Print detailed network statistics for each network connection

flags: `0x0`  
</details>
<details>
<summary><code>net_dumpeventstats</code></summary>

Dumps out a report of game event network usage

flags: `0x0`  
</details>
<details>
<summary><code>net_start</code></summary>

Inits multiplayer network sockets

flags: `0x0`  
</details>
<details>
<summary><code>net_status</code></summary>

Shows current network status

flags: `0x0`  
</details>
<details>
<summary><code>net_steamcnx_status</code></summary>

Print status of steam connection sockets.

flags: `0x0`  
</details>
<details>
<summary><code>nextdemo</code></summary>

Play next demo in sequence.

flags: `0x0`  
</details>
<details>
<summary><code>occlusion_stats</code></summary>

Occlusion statistics; [-jitter] [-reset]

flags: `0x80000`  
</details>
<details>
<summary><code>panorama_dump_deny_input</code></summary>

Dumps panels currently denying all input to the game

flags: `0x2`  
</details>
<details>
<summary><code>path</code></summary>

Show the engine filesystem path.

flags: `0x0`  
</details>
<details>
<summary><code>pause</code></summary>

Toggle the server pause state.

flags: `0x0`  
</details>
<details>
<summary><code>perfui</code></summary>

Show/hide the level performance tools UI.

flags: `0x4000`  
</details>
<details>
<summary><code>ping</code></summary>

Display ping to server.

flags: `0x0`  
</details>
<details>
<summary><code>play</code></summary>

Play a sound.

flags: `0x10000000`  
</details>
<details>
<summary><code>play_hrtf</code></summary>

Play a sound with HRTF spatialization.

flags: `0x10000000`  
</details>
<details>
<summary><code>playcast</code></summary>

Play a broadcast

flags: `0x0`  
</details>
<details>
<summary><code>playdemo</code></summary>

Play a recorded demo file (.dem ).

flags: `0x0`  
</details>
<details>
<summary><code>playflush</code></summary>

Play a sound, reloading from disk in case of changes.

flags: `0x0`  
</details>
<details>
<summary><code>playoverwatchevidence</code></summary>

Play evidence for an overwatch case.

flags: `0x10`  
</details>
<details>
<summary><code>playvol</code></summary>

Play a sound at a specified volume.

flags: `0x0`  
</details>
<details>
<summary><code>plugin_load</code></summary>

plugin_load <filename> : loads a plugin

flags: `0x0`  
</details>
<details>
<summary><code>plugin_pause</code></summary>

plugin_pause <index> : pauses a loaded plugin

flags: `0x0`  
</details>
<details>
<summary><code>plugin_pause_all</code></summary>

pauses all loaded plugins

flags: `0x0`  
</details>
<details>
<summary><code>plugin_print</code></summary>

Prints details about loaded plugins

flags: `0x0`  
</details>
<details>
<summary><code>plugin_unload</code></summary>

plugin_unload <index> : unloads a plugin

flags: `0x0`  
</details>
<details>
<summary><code>plugin_unpause</code></summary>

plugin_unpause <index> : unpauses a disabled plugin

flags: `0x0`  
</details>
<details>
<summary><code>plugin_unpause_all</code></summary>

unpauses all disabled plugins

flags: `0x0`  
</details>
<details>
<summary><code>print_colorcorrection</code></summary>

Display the color correction layer information.

flags: `0x4000`  
</details>
<details>
<summary><code>progress_enable</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>prop_crosshair</code></summary>

Shows name for prop looking at

flags: `0x4000`  
</details>
<details>
<summary><code>quit</code></summary>

Exit the engine.

flags: `0x0`  
</details>
<details>
<summary><code>r_cleardecals</code></summary>

Usage r_cleardecals <permanent>.

flags: `0x40000000`  
</details>
<details>
<summary><code>r_flushlod</code></summary>

Flush and reload LODs.

flags: `0x4000`  
</details>
<details>
<summary><code>r_lightcache_invalidate</code></summary>



flags: `0x4000`  
</details>
<details>
<summary><code>r_printdecalinfo</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>rcon</code></summary>

Issue an rcon command.

flags: `0x20000`  
</details>
<details>
<summary><code>recompute_speed</code></summary>

Recomputes clock speed (for debugging purposes).

flags: `0x4000`  
</details>
<details>
<summary><code>record</code></summary>

Record a demo.

flags: `0x20000`  
</details>
<details>
<summary><code>reload</code></summary>

Reload the most recent saved game (add setpos to jump to current view position on reload).

flags: `0x0`  
</details>
<details>
<summary><code>reload_vjobs</code></summary>

reload vjobs module

flags: `0x0`  
</details>
<details>
<summary><code>removeallids</code></summary>

Remove all user IDs from the ban list.

flags: `0x0`  
</details>
<details>
<summary><code>removeid</code></summary>

Remove a user ID from the ban list.

flags: `0x0`  
</details>
<details>
<summary><code>removeip</code></summary>

Remove an IP address from the ban list.

flags: `0x0`  
</details>
<details>
<summary><code>render_blanks</code></summary>

render N blank frames

flags: `0x0`  
</details>
<details>
<summary><code>reset_gameconvars</code></summary>

Reset a bunch of game convars to default values

flags: `0x4000`  
</details>
<details>
<summary><code>restart</code></summary>

Restart the game on the same level (add setpos to jump to current view position on restart).

flags: `0x0`  
</details>
<details>
<summary><code>retry</code></summary>

Retry connection to last server.

flags: `0x50020000`  
</details>
<details>
<summary><code>save</code></summary>

Saves current game.

flags: `0x20000`  
</details>
<details>
<summary><code>save_finish_async</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>scandemo</code></summary>

Scan a recorded demo file (.dem ) for specific game events and dump data.

flags: `0x0`  
</details>
<details>
<summary><code>screenshot</code></summary>

Take a screenshot.

flags: `0x40000000`  
</details>
<details>
<summary><code>sdr</code></summary>

SteamDatagram Network Configuration

flags: `0x0`  
</details>
<details>
<summary><code>setinfo</code></summary>

Adds a new user info value

flags: `0x40000000`  
</details>
<details>
<summary><code>setpause</code></summary>

Set the pause state of the server.

flags: `0x0`  
</details>
<details>
<summary><code>showconsole</code></summary>

Show the console.

flags: `0x20000`  
</details>
<details>
<summary><code>snd_async_flush</code></summary>

Flush all unlocked async audio data

flags: `0x0`  
</details>
<details>
<summary><code>snd_async_showmem</code></summary>

Show async memory stats

flags: `0x0`  
</details>
<details>
<summary><code>snd_async_showmem_music</code></summary>

Show async memory stats for just non-streamed music

flags: `0x0`  
</details>
<details>
<summary><code>snd_async_showmem_summary</code></summary>

Show brief async memory stats

flags: `0x0`  
</details>
<details>
<summary><code>snd_debug_sleep</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>snd_dump_filepaths</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>snd_dumpclientsounds</code></summary>

Dump sounds to console

flags: `0x4000`  
</details>
<details>
<summary><code>snd_front_headphone_position</code></summary>

Specifies the position (in degrees) of the virtual front left/right headphones.

flags: `0x0`  
</details>
<details>
<summary><code>snd_front_stereo_speaker_position</code></summary>

Specifies the position (in degrees) of the virtual front left/right speakers.

flags: `0x0`  
</details>
<details>
<summary><code>snd_front_surround_speaker_position</code></summary>

Specifies the position (in degrees) of the virtual front left/right speakers.

flags: `0x0`  
</details>
<details>
<summary><code>snd_getmixer</code></summary>

Get data related to mix group matching string

flags: `0x0`  
</details>
<details>
<summary><code>snd_headphone_pan_exponent</code></summary>

Specifies the exponent for the pan xfade from phone to phone if the "exp" pan law is being used.

flags: `0x0`  
</details>
<details>
<summary><code>snd_headphone_pan_radial_weight</code></summary>

Apply cos(angle) * weight before pan law

flags: `0x0`  
</details>
<details>
<summary><code>snd_print_channel_by_guid</code></summary>

Prints the content of a channel from its guid. snd_print_channel_by_guid <guid>.

flags: `0x4000`  
</details>
<details>
<summary><code>snd_print_channel_by_index</code></summary>

Prints the content of a channel from its index. snd_print_channel_by_index <index>.

flags: `0x4000`  
</details>
<details>
<summary><code>snd_print_channels</code></summary>

Prints all the active channel.

flags: `0x4000`  
</details>
<details>
<summary><code>snd_print_dsp_effect</code></summary>

Prints the content of a dsp effect.

flags: `0x0`  
</details>
<details>
<summary><code>snd_rear_headphone_position</code></summary>

Specifies the position  (in degrees) of the virtual rear left/right headphones.

flags: `0x0`  
</details>
<details>
<summary><code>snd_rear_stereo_speaker_position</code></summary>

Specifies the position (in degrees) of the virtual rear left/right speakers.

flags: `0x0`  
</details>
<details>
<summary><code>snd_rear_surround_speaker_position</code></summary>

Specifies the position (in degrees) of the virtual rear left/right speakers.

flags: `0x0`  
</details>
<details>
<summary><code>snd_restart</code></summary>

Restart sound system.

flags: `0x0`  
</details>
<details>
<summary><code>snd_set_master_volume</code></summary>

Sets the master volume for a channel. snd_set_master_volume <guid> <mastervolume>.

flags: `0x4000`  
</details>
<details>
<summary><code>snd_setmixer</code></summary>

Set named Mixgroup of current mixer to mix vol, mute, solo.

flags: `0x4000`  
</details>
<details>
<summary><code>snd_setmixlayer</code></summary>

Set named Mixgroup of named mix layer to mix vol, mute, solo.

flags: `0x4000`  
</details>
<details>
<summary><code>snd_setmixlayer_amount</code></summary>

Set named mix layer mix amount.

flags: `0x4000`  
</details>
<details>
<summary><code>snd_sos_flush_operators</code></summary>

Flush and re-parse the sound operator system

flags: `0x4000`  
</details>
<details>
<summary><code>snd_sos_print_operators</code></summary>

Prints a list of currently available operators

flags: `0x4000`  
</details>
<details>
<summary><code>snd_soundmixer_flush</code></summary>

Reload soundmixers.txt file.

flags: `0x4000`  
</details>
<details>
<summary><code>snd_soundmixer_list_mix_groups</code></summary>

List all mix groups to dev console.

flags: `0x0`  
</details>
<details>
<summary><code>snd_soundmixer_list_mix_layers</code></summary>

List all mix layers to dev console.

flags: `0x0`  
</details>
<details>
<summary><code>snd_soundmixer_list_mixers</code></summary>

List all mixers to dev console.

flags: `0x0`  
</details>
<details>
<summary><code>snd_soundmixer_set_trigger_factor</code></summary>

Set named mix layer / mix group, trigger amount.

flags: `0x4000`  
</details>
<details>
<summary><code>snd_stereo_speaker_pan_exponent</code></summary>

Specifies the exponent for the pan xfade from speaker to speaker if the "exp" pan law is being used.

flags: `0x0`  
</details>
<details>
<summary><code>snd_stereo_speaker_pan_radial_weight</code></summary>

Apply cos(angle) * weight before pan law

flags: `0x0`  
</details>
<details>
<summary><code>snd_surround_speaker_pan_exponent</code></summary>

Specifies the exponent for the pan xfade from speaker to speaker if the "exp" pan law is being used.

flags: `0x0`  
</details>
<details>
<summary><code>snd_surround_speaker_pan_radial_weight</code></summary>

Apply cos(angle) * weight before pan law

flags: `0x0`  
</details>
<details>
<summary><code>snd_writemanifest</code></summary>

If running a game, outputs the precache manifest for the current level


flags: `0x0`  
</details>
<details>
<summary><code>sndplaydelay</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>sound_device_list</code></summary>

Lists all available audio devices.

flags: `0x0`  
</details>
<details>
<summary><code>soundfade</code></summary>

Fade client volume.

flags: `0x10000000`  
</details>
<details>
<summary><code>soundinfo</code></summary>

Describe the current sound device.

flags: `0x0`  
</details>
<details>
<summary><code>soundlist</code></summary>

List all known sounds.

flags: `0x0`  
</details>
<details>
<summary><code>speak</code></summary>

Play a constructed sentence.

flags: `0x0`  
</details>
<details>
<summary><code>spike</code></summary>

generates a fake spike

flags: `0x4000`  
</details>
<details>
<summary><code>spincycle</code></summary>

Cause the engine to spincycle (Debug!!)

flags: `0x4000`  
</details>
<details>
<summary><code>ss_connect</code></summary>

If connected with available split screen slots, connects a split screen player to this machine.

flags: `0x2`  
</details>
<details>
<summary><code>ss_disconnect</code></summary>

If connected with available split screen slots, connects a split screen player to this machine.

flags: `0x2`  
</details>
<details>
<summary><code>ss_map</code></summary>

Start playing on specified map with max allowed splitscreen players.

flags: `0x20000`  
</details>
<details>
<summary><code>star_memory</code></summary>

Dump memory stats

flags: `0x0`  
</details>
<details>
<summary><code>startdemos</code></summary>

Play demos in demo sequence.

flags: `0x0`  
</details>
<details>
<summary><code>startmovie</code></summary>

Start recording movie frames.

flags: `0x20000`  
</details>
<details>
<summary><code>startupmenu</code></summary>

Opens initial menu screen and loads the background bsp, but only if no other level is being loaded, and we're not in developer mode.

flags: `0x0`  
</details>
<details>
<summary><code>stats</code></summary>

Prints server performance variables

flags: `0x0`  
</details>
<details>
<summary><code>status</code></summary>

Display map and connection status.

flags: `0x0`  
</details>
<details>
<summary><code>stop</code></summary>

Finish recording demo.

flags: `0x0`  
</details>
<details>
<summary><code>stopdemo</code></summary>

Stop playing back a demo.

flags: `0x20000`  
</details>
<details>
<summary><code>stopsound</code></summary>



flags: `0x4000`  
</details>
<details>
<summary><code>stringtabledictionary</code></summary>

Create dictionary for current strings.

flags: `0x0`  
</details>
<details>
<summary><code>stuffcmds</code></summary>

Parses and stuffs command line + commands to command buffer.

flags: `0x0`  
</details>
<details>
<summary><code>sv_dump_class_info</code></summary>

Dump server class infos.

flags: `0x0`  
</details>
<details>
<summary><code>sv_dump_class_table</code></summary>

Dump server class table matching the pattern (substr).

flags: `0x0`  
</details>
<details>
<summary><code>sv_dump_serialized_entities_mem</code></summary>

Dump serialized entity allocations stats.

flags: `0x0`  
</details>
<details>
<summary><code>sv_getinfo</code></summary>

Show user info of a connected client

flags: `0x0`  
</details>
<details>
<summary><code>sv_precacheinfo</code></summary>

Show precache info.

flags: `0x0`  
</details>
<details>
<summary><code>sv_pure</code></summary>

Show user data.

flags: `0x0`  
</details>
<details>
<summary><code>sv_pure_checkvpk</code></summary>

CheckPureServerVPKFiles

flags: `0x0`  
</details>
<details>
<summary><code>sv_pure_finduserfiles</code></summary>

ListPureServerFiles

flags: `0x0`  
</details>
<details>
<summary><code>sv_pure_listfiles</code></summary>

ListPureServerFiles

flags: `0x0`  
</details>
<details>
<summary><code>sv_pure_listuserfiles</code></summary>

ListPureServerFiles

flags: `0x0`  
</details>
<details>
<summary><code>sv_send_stats</code></summary>

show stats of running parallel send

flags: `0x0`  
</details>
<details>
<summary><code>sv_setsteamaccount</code></summary>

token
Set game server account token to use for logging in to a persistent game server account

flags: `0x0`  
</details>
<details>
<summary><code>sv_showtags</code></summary>

Describe current gametags.

flags: `0x0`  
</details>
<details>
<summary><code>sv_shutdown</code></summary>

Sets the server to shutdown when all games have completed

flags: `0x80000`  
</details>
<details>
<summary><code>thread_test_tslist</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>thread_test_tsqueue</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>threadpool_cycle_reserve</code></summary>

Cycles threadpool reservation by powers of 2

flags: `0x0`  
</details>
<details>
<summary><code>threadpool_run_tests</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>timedemo</code></summary>

Play a demo and report performance info.

flags: `0x0`  
</details>
<details>
<summary><code>timedemo_vprofrecord</code></summary>

Play a demo and report performance info.  Also record vprof data for the span of the demo

flags: `0x0`  
</details>
<details>
<summary><code>timedemoquit</code></summary>

Play a demo, report performance info, and then exit

flags: `0x0`  
</details>
<details>
<summary><code>timerefresh</code></summary>

Profile the renderer.

flags: `0x4000`  
</details>
<details>
<summary><code>toggle</code></summary>

Toggles a convar on or off, or cycles through a set of values.

flags: `0x0`  
</details>
<details>
<summary><code>toggleconsole</code></summary>

Show/hide the console.

flags: `0x20000`  
</details>
<details>
<summary><code>toolload</code></summary>

Load a tool.

flags: `0x0`  
</details>
<details>
<summary><code>toolunload</code></summary>

Unload a tool.

flags: `0x0`  
</details>
<details>
<summary><code>tv_broadcast_resend</code></summary>

resend broadcast data to broadcast relay

flags: `0x0`  
</details>
<details>
<summary><code>tv_broadcast_status</code></summary>

Print out broadcast status

flags: `0x0`  
</details>
<details>
<summary><code>tv_clients</code></summary>

Shows list of connected GOTV clients [-instance <inst> ]

flags: `0x0`  
</details>
<details>
<summary><code>tv_mem</code></summary>

hltv memory statistics

flags: `0x0`  
</details>
<details>
<summary><code>tv_msg</code></summary>

Send a screen message to all clients [-instance <inst> ]

flags: `0x0`  
</details>
<details>
<summary><code>tv_record</code></summary>

Starts GOTV demo recording [-instance <inst> ]

flags: `0x0`  
</details>
<details>
<summary><code>tv_relay</code></summary>

Connect to GOTV server and relay broadcast.

flags: `0x0`  
</details>
<details>
<summary><code>tv_retry</code></summary>

Reconnects the GOTV relay proxy 

flags: `0x0`  
</details>
<details>
<summary><code>tv_status</code></summary>

Show GOTV server status.

flags: `0x0`  
</details>
<details>
<summary><code>tv_stop</code></summary>

Stops the GOTV broadcast [-instance <inst> ]

flags: `0x0`  
</details>
<details>
<summary><code>tv_stoprecord</code></summary>

Stops GOTV demo recording [-instance <inst> ]

flags: `0x0`  
</details>
<details>
<summary><code>unbind</code></summary>

Unbind a key.

flags: `0x0`  
</details>
<details>
<summary><code>unbindall</code></summary>

Unbind all keys.

flags: `0x0`  
</details>
<details>
<summary><code>unbindalljoystick</code></summary>

Unbind all joystick keys.

flags: `0x0`  
</details>
<details>
<summary><code>unbindallmousekeyboard</code></summary>

Unbind all mouse / keyboard keys.

flags: `0x0`  
</details>
<details>
<summary><code>unpause</code></summary>

Unpause the game.

flags: `0x0`  
</details>
<details>
<summary><code>user</code></summary>

Show user data.

flags: `0x0`  
</details>
<details>
<summary><code>users</code></summary>

Show user info for players on server.

flags: `0x0`  
</details>
<details>
<summary><code>version</code></summary>

Print version info string.

flags: `0x0`  
</details>
<details>
<summary><code>vgui_drawtree_clear</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>vgui_dump_panels</code></summary>

vgui_dump_panels [visible]

flags: `0x0`  
</details>
<details>
<summary><code>vgui_togglepanel</code></summary>

show/hide vgui panel by name.

flags: `0x0`  
</details>
<details>
<summary><code>voice_player_volume</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>voicerecord_toggle</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>vox_reload</code></summary>

Reload sentences.txt file

flags: `0x4000`  
</details>
<details>
<summary><code>vprof</code></summary>

Toggle VProf profiler

flags: `0x0`  
</details>
<details>
<summary><code>vprof_adddebuggroup1</code></summary>

add a new budget group dynamically for debugging

flags: `0x0`  
</details>
<details>
<summary><code>vprof_cachemiss</code></summary>

Toggle VProf cache miss checking

flags: `0x0`  
</details>
<details>
<summary><code>vprof_cachemiss_off</code></summary>

Turn off VProf cache miss checking

flags: `0x0`  
</details>
<details>
<summary><code>vprof_cachemiss_on</code></summary>

Turn on VProf cache miss checking

flags: `0x0`  
</details>
<details>
<summary><code>vprof_child</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>vprof_collapse_all</code></summary>

Collapse the whole vprof tree

flags: `0x0`  
</details>
<details>
<summary><code>vprof_dump_counters</code></summary>

Dump vprof counters to the console

flags: `0x0`  
</details>
<details>
<summary><code>vprof_dump_groupnames</code></summary>

Write the names of all of the vprof groups to the console.

flags: `0x0`  
</details>
<details>
<summary><code>vprof_expand_all</code></summary>

Expand the whole vprof tree

flags: `0x0`  
</details>
<details>
<summary><code>vprof_expand_group</code></summary>

Expand a budget group in the vprof tree by name

flags: `0x0`  
</details>
<details>
<summary><code>vprof_generate_report</code></summary>

Generate a report to the console.

flags: `0x0`  
</details>
<details>
<summary><code>vprof_generate_report_AI</code></summary>

Generate a report to the console.

flags: `0x0`  
</details>
<details>
<summary><code>vprof_generate_report_AI_only</code></summary>

Generate a report to the console.

flags: `0x0`  
</details>
<details>
<summary><code>vprof_generate_report_budget</code></summary>

Generate a report to the console based on budget group.

flags: `0x0`  
</details>
<details>
<summary><code>vprof_generate_report_hierarchy</code></summary>

Generate a report to the console.

flags: `0x0`  
</details>
<details>
<summary><code>vprof_generate_report_hierarchy_per_frame_and_count_only</code></summary>

Generate a minimal hiearchical report to the console.

flags: `0x0`  
</details>
<details>
<summary><code>vprof_generate_report_map_load</code></summary>

Generate a report to the console.

flags: `0x0`  
</details>
<details>
<summary><code>vprof_nextsibling</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>vprof_off</code></summary>

Turn off VProf profiler

flags: `0x0`  
</details>
<details>
<summary><code>vprof_on</code></summary>

Turn on VProf profiler

flags: `0x0`  
</details>
<details>
<summary><code>vprof_parent</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>vprof_playback_average</code></summary>

Average the next N frames.

flags: `0x0`  
</details>
<details>
<summary><code>vprof_playback_start</code></summary>

Start playing back a recorded .vprof file.

flags: `0x0`  
</details>
<details>
<summary><code>vprof_playback_step</code></summary>

While playing back a .vprof file, step to the next tick.

flags: `0x0`  
</details>
<details>
<summary><code>vprof_playback_stepback</code></summary>

While playing back a .vprof file, step to the previous tick.

flags: `0x0`  
</details>
<details>
<summary><code>vprof_playback_stop</code></summary>

Stop playing back a recorded .vprof file.

flags: `0x0`  
</details>
<details>
<summary><code>vprof_prevsibling</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>vprof_record_start</code></summary>

Start recording vprof data for playback later.

flags: `0x0`  
</details>
<details>
<summary><code>vprof_record_stop</code></summary>

Stop recording vprof data

flags: `0x0`  
</details>
<details>
<summary><code>vprof_remote_start</code></summary>

Request a VProf data stream from the remote server (requires authentication)

flags: `0x0`  
</details>
<details>
<summary><code>vprof_remote_stop</code></summary>

Stop an existing remote VProf data request

flags: `0x0`  
</details>
<details>
<summary><code>vprof_reset</code></summary>

Reset the stats in VProf profiler

flags: `0x0`  
</details>
<details>
<summary><code>vprof_reset_peaks</code></summary>

Reset just the peak time in VProf profiler

flags: `0x0`  
</details>
<details>
<summary><code>vprof_to_csv</code></summary>

Convert a recorded .vprof file to .csv.

flags: `0x0`  
</details>
<details>
<summary><code>vprof_vtune_group</code></summary>

enable vtune for a particular vprof group ("disable" to disable)

flags: `0x0`  
</details>
<details>
<summary><code>vtune</code></summary>

Controls VTune's sampling.

flags: `0x0`  
</details>
<details>
<summary><code>vx_model_list</code></summary>

Dump models to VXConsole

flags: `0x0`  
</details>
<details>
<summary><code>whitelistcmd</code></summary>

Runs a whitelisted command.

flags: `0x0`  
</details>
<details>
<summary><code>writeid</code></summary>

Writes a list of permanently-banned user IDs to banned_user.cfg.

flags: `0x0`  
</details>
<details>
<summary><code>writeip</code></summary>

Save the ban list to banned_ip.cfg.

flags: `0x0`  
</details>
<details>
<summary><code>xload</code></summary>

Load a saved game from a console storage device.

flags: `0x0`  
</details>
<details>
<summary><code>xsave</code></summary>

Saves current game to a console storage device.

flags: `0x20000`  
</details>

#### Addresses

```
engine.dll!0x0058bd68 ConCommand +mat_texture_list
engine.dll!0x005a0cac ConCommand +showbudget
engine.dll!0x005a14f8 ConCommand +showbudget_texture
engine.dll!0x005a1540 ConCommand +showbudget_texture_global
engine.dll!0x005a1aa4 ConCommand +showvprof
engine.dll!0x005a1380 ConCommand +vgui_drawtree
engine.dll!0x00598b78 ConCommand +voicerecord
engine.dll!0x0058bd8c ConCommand -mat_texture_list
engine.dll!0x005a0cd0 ConCommand -showbudget
engine.dll!0x005a151c ConCommand -showbudget_texture
engine.dll!0x005a1564 ConCommand -showbudget_texture_global
engine.dll!0x005a1ac8 ConCommand -showvprof
engine.dll!0x005a13a4 ConCommand -vgui_drawtree
engine.dll!0x0059896c ConCommand -voicerecord
engine.dll!0x00595850 ConCommand BindToggle
engine.dll!0x0059fb6c ConCommand Test_Loop
engine.dll!0x0059fb48 ConCommand Test_LoopCount
engine.dll!0x0059fc44 ConCommand Test_LoopForNumSeconds
engine.dll!0x0059fbb4 ConCommand Test_RandomChance
engine.dll!0x0059fb90 ConCommand Test_RunFrame
engine.dll!0x0059fc20 ConCommand Test_SendKey
engine.dll!0x0059fbd8 ConCommand Test_StartLoop
engine.dll!0x0059fb00 ConCommand Test_StartScript
engine.dll!0x0059fb24 ConCommand Test_Wait
engine.dll!0x0059fbfc ConCommand Test_WaitForCheckPoint
engine.dll!0x00596968 ConCommand TransmitEvents
engine.dll!0x00599590 ConCommand _autosave
engine.dll!0x005998d0 ConCommand _autosavedangerous
engine.dll!0x005953a0 ConCommand _bugreporter_restart
engine.dll!0x0058ade8 ConCommand _record
engine.dll!0x00598f68 ConCommand _restart
engine.dll!0x00591df8 ConCommand addip
engine.dll!0x00519c40 ConCommand adsp_reset_nodes
engine.dll!0x0059594c ConCommand alias
engine.dll!0x005899d0 ConCommand askconnect_accept
engine.dll!0x0058c148 ConCommand asw_engine_finished_building_map
engine.dll!0x0059e3e0 ConCommand audit_save_in_memory
engine.dll!0x00599950 ConCommand autosave
engine.dll!0x005998ac ConCommand autosavedangerous
engine.dll!0x00599610 ConCommand autosavedangerousissafe
engine.dll!0x00591f3c ConCommand banid
engine.dll!0x00591e1c ConCommand banip
engine.dll!0x0058a058 ConCommand bench_end
engine.dll!0x0058a034 ConCommand bench_start
engine.dll!0x0058a07c ConCommand bench_upload
engine.dll!0x0058af08 ConCommand benchframe
engine.dll!0x0059c0dc ConCommand bind
engine.dll!0x0059c04c ConCommand bind_osx
engine.dll!0x005952f8 ConCommand blackbox_dump
engine.dll!0x005952d4 ConCommand blackbox_record
engine.dll!0x0058b884 ConCommand box
engine.dll!0x0059fda8 ConCommand budget_toggle_group
engine.dll!0x00595378 ConCommand bug
engine.dll!0x00589f90 ConCommand buildcubemaps
engine.dll!0x0058a010 ConCommand buildmodelforworld
engine.dll!0x00598d5c ConCommand cache_print
engine.dll!0x00598cf0 ConCommand cache_print_lru
engine.dll!0x005989fc ConCommand cache_print_summary
engine.dll!0x005993b4 ConCommand changelevel
engine.dll!0x005993d8 ConCommand changelevel2
engine.dll!0x0058b970 ConCommand cl_fullupdate
engine.dll!0x0058b9b8 ConCommand cl_precacheinfo
engine.dll!0x0058b8cc ConCommand cl_showents
engine.dll!0x0058b8a8 ConCommand cl_view
engine.dll!0x0058c754 ConCommand clear
engine.dll!0x00595874 ConCommand cmd
engine.dll!0x00595af4 ConCommand cmd1
engine.dll!0x00595b18 ConCommand cmd2
engine.dll!0x00595ad0 ConCommand cmd3
engine.dll!0x00595b3c ConCommand cmd4
engine.dll!0x0058c280 ConCommand colorcorrectionui
engine.dll!0x0058b9dc ConCommand con_min_severity
engine.dll!0x0058b600 ConCommand connect
engine.dll!0x0058b624 ConCommand connect_splitscreen
engine.dll!0x00598ea0 ConCommand crash
engine.dll!0x00595aac ConCommand cvarlist
engine.dll!0x00595a6c ConCommand debug_drawbox
engine.dll!0x00595a48 ConCommand debug_drawdisp_boundbox
engine.dll!0x0058afbc ConCommand demo_goto
engine.dll!0x0058afe0 ConCommand demo_gototick
engine.dll!0x0058b004 ConCommand demo_info
engine.dll!0x0058b070 ConCommand demo_listhighlights
engine.dll!0x0058b04c ConCommand demo_listimportantticks
engine.dll!0x0058af50 ConCommand demo_pause
engine.dll!0x0058af74 ConCommand demo_resume
engine.dll!0x0058b028 ConCommand demo_timescale
engine.dll!0x0058af98 ConCommand demo_togglepause
engine.dll!0x00598da4 ConCommand demolist
engine.dll!0x00598b9c ConCommand demos
engine.dll!0x0058b09c ConCommand demoui
engine.dll!0x0058d170 ConCommand devshots_nextmap
engine.dll!0x0058b6c4 ConCommand devshots_screenshot
engine.dll!0x00595b68 ConCommand differences
engine.dll!0x005989b4 ConCommand disconnect
engine.dll!0x005966e8 ConCommand disp_list_all_collideable
engine.dll!0x0059833c ConCommand display_elapsedtime
engine.dll!0x0051a5dc ConCommand dsp_reload
engine.dll!0x00597c38 ConCommand dti_flush
engine.dll!0x00598ff8 ConCommand dumpstringtables
engine.dll!0x00595898 ConCommand echo
engine.dll!0x00597468 ConCommand editdemo
engine.dll!0x0059f55c ConCommand editor_toggle
engine.dll!0x0058b83c ConCommand endmovie
engine.dll!0x00589f48 ConCommand envmap
engine.dll!0x00599f40 ConCommand escape
engine.dll!0x00595928 ConCommand exec
engine.dll!0x005958bc ConCommand execifexists
engine.dll!0x00595970 ConCommand execwithwhitelist
engine.dll!0x00598ab0 ConCommand exit
engine.dll!0x00595bb0 ConCommand findflags
engine.dll!0x00598a44 ConCommand flush
engine.dll!0x00598bc0 ConCommand flush_locked
engine.dll!0x0058b278 ConCommand fogui
engine.dll!0x0059c070 ConCommand forcebind
engine.dll!0x0059678c ConCommand fs_printopenfiles
engine.dll!0x005967f8 ConCommand fs_syncdvddevcache
engine.dll!0x005967b0 ConCommand fs_warning_level
engine.dll!0x005a078c ConCommand gameui_activate
engine.dll!0x005a081c ConCommand gameui_allowescape
engine.dll!0x005a07d4 ConCommand gameui_allowescapetoshow
engine.dll!0x005a0768 ConCommand gameui_hide
engine.dll!0x005a07b0 ConCommand gameui_preventescape
engine.dll!0x005a07f8 ConCommand gameui_preventescapetoshow
engine.dll!0x005934a0 ConCommand heartbeat
engine.dll!0x00595b8c ConCommand help
engine.dll!0x0058c70c ConCommand hideconsole
engine.dll!0x00598d14 ConCommand hltv_replay_status
engine.dll!0x0059f7b8 ConCommand host_filtered_time_report
engine.dll!0x00597a40 ConCommand host_reset_config
engine.dll!0x005981f8 ConCommand host_runofftime
engine.dll!0x00598318 ConCommand host_timer_report
engine.dll!0x005979c4 ConCommand host_writeconfig
engine.dll!0x00597a64 ConCommand host_writeconfig_ss
engine.dll!0x00598fd4 ConCommand incrementvar
engine.dll!0x00599f1c ConCommand ipc_console_disable
engine.dll!0x00599ef8 ConCommand ipc_console_disable_all
engine.dll!0x00599eb0 ConCommand ipc_console_enable
engine.dll!0x00599ed4 ConCommand ipc_console_show
engine.dll!0x0058b6e8 ConCommand jpeg
engine.dll!0x0059c0b8 ConCommand key_findbinding
engine.dll!0x0059c124 ConCommand key_listboundkeys
engine.dll!0x00598c08 ConCommand kick
engine.dll!0x00598c2c ConCommand kickid
engine.dll!0x00599040 ConCommand kickid_ex
engine.dll!0x00598e7c ConCommand killserver
engine.dll!0x00598c50 ConCommand light_crosshair
engine.dll!0x00589f6c ConCommand lightprobe
engine.dll!0x0058d7e4 ConCommand linefile
engine.dll!0x0058aee4 ConCommand listdemo
engine.dll!0x00591f18 ConCommand listid
engine.dll!0x00591e64 ConCommand listip
engine.dll!0x00598d80 ConCommand listmodels
engine.dll!0x00599690 ConCommand load
engine.dll!0x00592510 ConCommand log
engine.dll!0x0058c7c0 ConCommand log_color
engine.dll!0x0058c778 ConCommand log_dumpchannels
engine.dll!0x0058c7e4 ConCommand log_flags
engine.dll!0x0058c79c ConCommand log_level
engine.dll!0x00592534 ConCommand logaddress_add
engine.dll!0x00592558 ConCommand logaddress_add_ex
engine.dll!0x005925d8 ConCommand logaddress_add_ts
engine.dll!0x00592620 ConCommand logaddress_del
engine.dll!0x005925fc ConCommand logaddress_delall
engine.dll!0x00592644 ConCommand logaddress_list
engine.dll!0x005992f8 ConCommand map
engine.dll!0x00599278 ConCommand map_background
engine.dll!0x005993fc ConCommand map_commentary
engine.dll!0x00598e34 ConCommand map_edit
engine.dll!0x00599364 ConCommand mapgroup
engine.dll!0x0059931c ConCommand maps
engine.dll!0x0058f320 ConCommand mat_configcurrent
engine.dll!0x0058d4bc ConCommand mat_crosshair
engine.dll!0x0058d4e0 ConCommand mat_crosshair_edit
engine.dll!0x0058d504 ConCommand mat_crosshair_explorer
engine.dll!0x0058d54c ConCommand mat_crosshair_printmaterial
engine.dll!0x0058d528 ConCommand mat_crosshair_reloadmaterial
engine.dll!0x0058f3b0 ConCommand mat_debug
engine.dll!0x0058d498 ConCommand mat_edit
engine.dll!0x0058f3f8 ConCommand mat_info
engine.dll!0x0058f368 ConCommand mat_savechanges
engine.dll!0x0058f344 ConCommand mat_setvideomode
engine.dll!0x0058f3d4 ConCommand mat_suppress
engine.dll!0x0058bf68 ConCommand mat_texture_list_txlod
engine.dll!0x0058f38c ConCommand mat_updateconvars
engine.dll!0x00592f50 ConCommand maxplayers
engine.dll!0x005983b8 ConCommand mem_compact
engine.dll!0x00597e6c ConCommand mem_dump
engine.dll!0x00597a88 ConCommand mem_eat
engine.dll!0x005982f4 ConCommand mem_incremental_compact
engine.dll!0x00597ee8 ConCommand mem_test
engine.dll!0x0058fe48 ConCommand mem_vcollide
engine.dll!0x00598540 ConCommand mem_verify
engine.dll!0x00598dc8 ConCommand memory
engine.dll!0x00599710 ConCommand minisave
engine.dll!0x0058fe6c ConCommand mod_DumpWeaponWiewModelCache
engine.dll!0x0058fe90 ConCommand mod_DumpWeaponWorldModelCache
engine.dll!0x0058ff00 ConCommand mod_combiner_info
engine.dll!0x0058fedc ConCommand mod_dynamicmodeldebug
engine.dll!0x0051c488 ConCommand movie_fixwave
engine.dll!0x00598990 ConCommand multvar
engine.dll!0x0059d398 ConCommand net_channels
engine.dll!0x0059d6d0 ConCommand net_connections_stats
engine.dll!0x00596878 ConCommand net_dumpeventstats
engine.dll!0x0059dab0 ConCommand net_start
engine.dll!0x0059d6f4 ConCommand net_status
engine.dll!0x0059d258 ConCommand net_steamcnx_status
engine.dll!0x00598c74 ConCommand nextdemo
engine.dll!0x00596768 ConCommand occlusion_stats
engine.dll!0x0059e248 ConCommand panorama_dump_deny_input
engine.dll!0x005967d4 ConCommand path
engine.dll!0x00598a8c ConCommand pause
engine.dll!0x005a0588 ConCommand perfui
engine.dll!0x00598af8 ConCommand ping
engine.dll!0x005196a4 ConCommand play
engine.dll!0x005196c8 ConCommand play_hrtf
engine.dll!0x0058a130 ConCommand playcast
engine.dll!0x0058ae30 ConCommand playdemo
engine.dll!0x005196ec ConCommand playflush
engine.dll!0x0058ae78 ConCommand playoverwatchevidence
engine.dll!0x00519710 ConCommand playvol
engine.dll!0x005937a4 ConCommand plugin_load
engine.dll!0x00593714 ConCommand plugin_pause
engine.dll!0x0059375c ConCommand plugin_pause_all
engine.dll!0x005936f0 ConCommand plugin_print
engine.dll!0x005937c8 ConCommand plugin_unload
engine.dll!0x00593738 ConCommand plugin_unpause
engine.dll!0x00593780 ConCommand plugin_unpause_all
engine.dll!0x0058c2a4 ConCommand print_colorcorrection
engine.dll!0x005a0840 ConCommand progress_enable
engine.dll!0x0059eae4 ConCommand prop_crosshair
engine.dll!0x005989d8 ConCommand quit
engine.dll!0x0058d808 ConCommand r_cleardecals
engine.dll!0x0058e290 ConCommand r_flushlod
engine.dll!0x0059c410 ConCommand r_lightcache_invalidate
engine.dll!0x00590a28 ConCommand r_printdecalinfo
engine.dll!0x0058b860 ConCommand rcon
engine.dll!0x00597e48 ConCommand recompute_speed
engine.dll!0x0058adc4 ConCommand record
engine.dll!0x00598ee8 ConCommand reload
engine.dll!0x0059f4e8 ConCommand reload_vjobs
engine.dll!0x00591ed0 ConCommand removeallids
engine.dll!0x00591ef4 ConCommand removeid
engine.dll!0x00591e40 ConCommand removeip
engine.dll!0x0059f530 ConCommand render_blanks
engine.dll!0x00595c04 ConCommand reset_gameconvars
engine.dll!0x00598f8c ConCommand restart
engine.dll!0x0058b5dc ConCommand retry
engine.dll!0x005997b0 ConCommand save
engine.dll!0x00599460 ConCommand save_finish_async
engine.dll!0x0058ae54 ConCommand scandemo
engine.dll!0x0058b6a0 ConCommand screenshot
engine.dll!0x0059db80 ConCommand sdr
engine.dll!0x0058b994 ConCommand setinfo
engine.dll!0x00598a68 ConCommand setpause
engine.dll!0x0058c730 ConCommand showconsole
engine.dll!0x0051c060 ConCommand snd_async_flush
engine.dll!0x0051c084 ConCommand snd_async_showmem
engine.dll!0x0051c0cc ConCommand snd_async_showmem_music
engine.dll!0x0051c0a8 ConCommand snd_async_showmem_summary
engine.dll!0x0051a624 ConCommand snd_debug_sleep
engine.dll!0x0051a150 ConCommand snd_dump_filepaths
engine.dll!0x0051a500 ConCommand snd_dumpclientsounds
engine.dll!0x0051849c ConCommand snd_front_headphone_position
engine.dll!0x005184e4 ConCommand snd_front_stereo_speaker_position
engine.dll!0x0051852c ConCommand snd_front_surround_speaker_position
engine.dll!0x0051bbc4 ConCommand snd_getmixer
engine.dll!0x00518574 ConCommand snd_headphone_pan_exponent
engine.dll!0x005185e0 ConCommand snd_headphone_pan_radial_weight
engine.dll!0x0051a66c ConCommand snd_print_channel_by_guid
engine.dll!0x0051a648 ConCommand snd_print_channel_by_index
engine.dll!0x0051a690 ConCommand snd_print_channels
engine.dll!0x0051b1d8 ConCommand snd_print_dsp_effect
engine.dll!0x005184c0 ConCommand snd_rear_headphone_position
engine.dll!0x00518508 ConCommand snd_rear_stereo_speaker_position
engine.dll!0x00518550 ConCommand snd_rear_surround_speaker_position
engine.dll!0x00597f68 ConCommand snd_restart
engine.dll!0x0051a6b4 ConCommand snd_set_master_volume
engine.dll!0x0051b9f0 ConCommand snd_setmixer
engine.dll!0x0051ba14 ConCommand snd_setmixlayer
engine.dll!0x0051ba38 ConCommand snd_setmixlayer_amount
engine.dll!0x0051cdd8 ConCommand snd_sos_flush_operators
engine.dll!0x0051cca8 ConCommand snd_sos_print_operators
engine.dll!0x0051bad8 ConCommand snd_soundmixer_flush
engine.dll!0x0051bba0 ConCommand snd_soundmixer_list_mix_groups
engine.dll!0x0051bb7c ConCommand snd_soundmixer_list_mix_layers
engine.dll!0x0051bb58 ConCommand snd_soundmixer_list_mixers
engine.dll!0x0051ba5c ConCommand snd_soundmixer_set_trigger_factor
engine.dll!0x00518598 ConCommand snd_stereo_speaker_pan_exponent
engine.dll!0x00518604 ConCommand snd_stereo_speaker_pan_radial_weight
engine.dll!0x005185bc ConCommand snd_surround_speaker_pan_exponent
engine.dll!0x00518628 ConCommand snd_surround_speaker_pan_radial_weight
engine.dll!0x0051c408 ConCommand snd_writemanifest
engine.dll!0x0051a600 ConCommand sndplaydelay
engine.dll!0x00519680 ConCommand sound_device_list
engine.dll!0x00598be4 ConCommand soundfade
engine.dll!0x005197a0 ConCommand soundinfo
engine.dll!0x0051977c ConCommand soundlist
engine.dll!0x00519734 ConCommand speak
engine.dll!0x0059fe94 ConCommand spike
engine.dll!0x00598ec4 ConCommand spincycle
engine.dll!0x00598a20 ConCommand ss_connect
engine.dll!0x00598fb0 ConCommand ss_disconnect
engine.dll!0x00599340 ConCommand ss_map
engine.dll!0x0059f334 ConCommand star_memory
engine.dll!0x00598e10 ConCommand startdemos
engine.dll!0x0058b818 ConCommand startmovie
engine.dll!0x0058b8f0 ConCommand startupmenu
engine.dll!0x00593ae8 ConCommand stats
engine.dll!0x0059901c ConCommand status
engine.dll!0x0058ada0 ConCommand stop
engine.dll!0x00598dec ConCommand stopdemo
engine.dll!0x00519758 ConCommand stopsound
engine.dll!0x00598d38 ConCommand stringtabledictionary
engine.dll!0x005958e0 ConCommand stuffcmds
engine.dll!0x00593680 ConCommand sv_dump_class_info
engine.dll!0x005936a4 ConCommand sv_dump_class_table
engine.dll!0x00591110 ConCommand sv_dump_serialized_entities_mem
engine.dll!0x00597034 ConCommand sv_getinfo
engine.dll!0x00593848 ConCommand sv_precacheinfo
engine.dll!0x00592960 ConCommand sv_pure
engine.dll!0x0059e2c8 ConCommand sv_pure_checkvpk
engine.dll!0x0059e334 ConCommand sv_pure_finduserfiles
engine.dll!0x0059e2ec ConCommand sv_pure_listfiles
engine.dll!0x0059e310 ConCommand sv_pure_listuserfiles
engine.dll!0x005930a0 ConCommand sv_send_stats
engine.dll!0x00593b0c ConCommand sv_setsteamaccount
engine.dll!0x00594f88 ConCommand sv_showtags
engine.dll!0x00592f74 ConCommand sv_shutdown
engine.dll!0x005982d0 ConCommand thread_test_tslist
engine.dll!0x005979a0 ConCommand thread_test_tsqueue
engine.dll!0x00597d10 ConCommand threadpool_cycle_reserve
engine.dll!0x00597bb8 ConCommand threadpool_run_tests
engine.dll!0x0058ae9c ConCommand timedemo
engine.dll!0x0058af2c ConCommand timedemo_vprofrecord
engine.dll!0x0058aec0 ConCommand timedemoquit
engine.dll!0x0058d7c0 ConCommand timerefresh
engine.dll!0x00595be0 ConCommand toggle
engine.dll!0x0058c6e8 ConCommand toggleconsole
engine.dll!0x005a04e0 ConCommand toolload
engine.dll!0x005a0504 ConCommand toolunload
engine.dll!0x00596bf8 ConCommand tv_broadcast_resend
engine.dll!0x00597710 ConCommand tv_broadcast_status
engine.dll!0x00597058 ConCommand tv_clients
engine.dll!0x00597010 ConCommand tv_mem
engine.dll!0x00597560 ConCommand tv_msg
engine.dll!0x00597208 ConCommand tv_record
engine.dll!0x0059748c ConCommand tv_relay
engine.dll!0x00596f90 ConCommand tv_retry
engine.dll!0x00597690 ConCommand tv_status
engine.dll!0x00597130 ConCommand tv_stop
engine.dll!0x00597390 ConCommand tv_stoprecord
engine.dll!0x0059c004 ConCommand unbind
engine.dll!0x0059c094 ConCommand unbindall
engine.dll!0x0059c100 ConCommand unbindalljoystick
engine.dll!0x0059c028 ConCommand unbindallmousekeyboard
engine.dll!0x00598948 ConCommand unpause
engine.dll!0x00592f08 ConCommand user
engine.dll!0x00592f2c ConCommand users
engine.dll!0x00598e58 ConCommand version
engine.dll!0x005a13c8 ConCommand vgui_drawtree_clear
engine.dll!0x005a09a4 ConCommand vgui_dump_panels
engine.dll!0x005a0980 ConCommand vgui_togglepanel
engine.dll!0x00589338 ConCommand voice_player_volume
engine.dll!0x00598ad4 ConCommand voicerecord_toggle
engine.dll!0x00589658 ConCommand vox_reload
engine.dll!0x005a0090 ConCommand vprof
engine.dll!0x005a0c88 ConCommand vprof_adddebuggroup1
engine.dll!0x005a00d8 ConCommand vprof_cachemiss
engine.dll!0x0059fd28 ConCommand vprof_cachemiss_off
engine.dll!0x0059fc84 ConCommand vprof_cachemiss_on
engine.dll!0x005a190c ConCommand vprof_child
engine.dll!0x005a1a5c ConCommand vprof_collapse_all
engine.dll!0x005a00fc ConCommand vprof_dump_counters
engine.dll!0x0059fe70 ConCommand vprof_dump_groupnames
engine.dll!0x005a1a38 ConCommand vprof_expand_all
engine.dll!0x005a1a80 ConCommand vprof_expand_group
engine.dll!0x0059fe28 ConCommand vprof_generate_report
engine.dll!0x005a01e4 ConCommand vprof_generate_report_AI
engine.dll!0x005a00b4 ConCommand vprof_generate_report_AI_only
engine.dll!0x0059feb8 ConCommand vprof_generate_report_budget
engine.dll!0x005a0178 ConCommand vprof_generate_report_hierarchy
engine.dll!0x005a0260 ConCommand vprof_generate_report_hierarchy_per_frame_and_count_only
engine.dll!0x0059ff38 ConCommand vprof_generate_report_map_load
engine.dll!0x005a18c4 ConCommand vprof_nextsibling
engine.dll!0x005a0284 ConCommand vprof_off
engine.dll!0x005a01c0 ConCommand vprof_on
engine.dll!0x005a18e8 ConCommand vprof_parent
engine.dll!0x005a0380 ConCommand vprof_playback_average
engine.dll!0x005a02a8 ConCommand vprof_playback_start
engine.dll!0x005a02cc ConCommand vprof_playback_step
engine.dll!0x005a035c ConCommand vprof_playback_stepback
engine.dll!0x005a03a4 ConCommand vprof_playback_stop
engine.dll!0x005a18a0 ConCommand vprof_prevsibling
engine.dll!0x005a02f0 ConCommand vprof_record_start
engine.dll!0x005a0338 ConCommand vprof_record_stop
engine.dll!0x0058bc7c ConCommand vprof_remote_start
engine.dll!0x0058bc58 ConCommand vprof_remote_stop
engine.dll!0x005a0010 ConCommand vprof_reset
engine.dll!0x005a019c ConCommand vprof_reset_peaks
engine.dll!0x005a0314 ConCommand vprof_to_csv
engine.dll!0x0059fe4c ConCommand vprof_vtune_group
engine.dll!0x0058ae0c ConCommand vtune
engine.dll!0x0058feb8 ConCommand vx_model_list
engine.dll!0x00595904 ConCommand whitelistcmd
engine.dll!0x00591eac ConCommand writeid
engine.dll!0x00591e88 ConCommand writeip
engine.dll!0x00599888 ConCommand xload
engine.dll!0x00599734 ConCommand xsave
```

## Client.dll

### Interfaces

```
client_panorama.dll!0x04d2d604 ClientAlphaPropertyMgrV001
client_panorama.dll!0x04d2d5f8 ClientLeafSystem002
client_panorama.dll!0x00ce33a0 ClientRenderTargets001
client_panorama.dll!0x00ce8848 GameClientExports001
client_panorama.dll!0x00d0689c GameConsole004
client_panorama.dll!0x05147710 GameMovement001
client_panorama.dll!0x05149ad0 GameUI011
client_panorama.dll!0x00cea930 IEffects001
client_panorama.dll!0x05137718 RenderToRTHelper001
client_panorama.dll!0x00d06a7c RunGameEngine005
client_panorama.dll!0x00ce8ac8 VCLIENTMATERIALSYSTEM001
client_panorama.dll!0x0518d26c VCLIENTTOOLS001
client_panorama.dll!0x04cf8280 VClient018
client_panorama.dll!0x04cf8268 VClientDllSharedAppSystems001
client_panorama.dll!0x04d1d57c VClientEntityList003
client_panorama.dll!0x05148a20 VClientPrediction001
client_panorama.dll!0x051493e0 VENGINE_GAMETYPES_VERSION002
client_panorama.dll!0x00d06cbc VGuiModuleLoader003
client_panorama.dll!0x00cf3280 VParticleSystemQuery004
```

### ConVars

<details>
<summary><code>BlendBonesMode</code></summary>

default: `"2"`  
flags: `0x2000`  
</details>
<details>
<summary><code>achievement_debug</code></summary>

Turn on achievement debug msgs.

default: `"0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>achievement_disable</code></summary>

Turn off achievements.

default: `"0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>ai_debug_shoot_positions</code></summary>

default: `"0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>ai_shot_bias_max</code></summary>

default: `"1.0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ai_shot_bias_min</code></summary>

default: `"-1.0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_338mag_headshot_mult</code></summary>

You must enable tweaking via tweak_ammo_impulses to use this value.

default: `"1.0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_338mag_impulse</code></summary>

You must enable tweaking via tweak_ammo_impulses to use this value.

default: `"2800"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_338mag_max</code></summary>

default: `"30"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ammo_357sig_headshot_mult</code></summary>

You must enable tweaking via tweak_ammo_impulses to use this value.

default: `"1.0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_357sig_impulse</code></summary>

You must enable tweaking via tweak_ammo_impulses to use this value.

default: `"2000"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_357sig_max</code></summary>

default: `"52"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ammo_357sig_min_max</code></summary>

default: `"12"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ammo_357sig_p250_max</code></summary>

default: `"26"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ammo_357sig_small_max</code></summary>

default: `"24"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ammo_45acp_headshot_mult</code></summary>

You must enable tweaking via tweak_ammo_impulses to use this value.

default: `"1.0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_45acp_impulse</code></summary>

You must enable tweaking via tweak_ammo_impulses to use this value.

default: `"2100"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_45acp_max</code></summary>

default: `"100"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ammo_50AE_headshot_mult</code></summary>

You must enable tweaking via tweak_ammo_impulses to use this value.

default: `"1.0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_50AE_impulse</code></summary>

You must enable tweaking via tweak_ammo_impulses to use this value.

default: `"2400"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_50AE_max</code></summary>

default: `"35"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ammo_556mm_box_headshot_mult</code></summary>

You must enable tweaking via tweak_ammo_impulses to use this value.

default: `"1.0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_556mm_box_impulse</code></summary>

You must enable tweaking via tweak_ammo_impulses to use this value.

default: `"2400"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_556mm_box_max</code></summary>

default: `"200"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ammo_556mm_headshot_mult</code></summary>

You must enable tweaking via tweak_ammo_impulses to use this value.

default: `"1.0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_556mm_impulse</code></summary>

You must enable tweaking via tweak_ammo_impulses to use this value.

default: `"2400"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_556mm_max</code></summary>

default: `"90"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ammo_556mm_small_max</code></summary>

default: `"40"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ammo_57mm_headshot_mult</code></summary>

You must enable tweaking via tweak_ammo_impulses to use this value.

default: `"1.0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_57mm_impulse</code></summary>

You must enable tweaking via tweak_ammo_impulses to use this value.

default: `"2000"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_57mm_max</code></summary>

default: `"100"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ammo_762mm_headshot_mult</code></summary>

You must enable tweaking via tweak_ammo_impulses to use this value.

default: `"1.0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_762mm_impulse</code></summary>

You must enable tweaking via tweak_ammo_impulses to use this value.

default: `"2400"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_762mm_max</code></summary>

default: `"90"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ammo_9mm_headshot_mult</code></summary>

You must enable tweaking via tweak_ammo_impulses to use this value.

default: `"1.0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_9mm_impulse</code></summary>

You must enable tweaking via tweak_ammo_impulses to use this value.

default: `"2000"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_9mm_max</code></summary>

default: `"120"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ammo_buckshot_headshot_mult</code></summary>

You must enable tweaking via tweak_ammo_impulses to use this value.

default: `"1.0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_buckshot_impulse</code></summary>

You must enable tweaking via tweak_ammo_impulses to use this value.

default: `"600"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_buckshot_max</code></summary>

default: `"32"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ammo_grenade_limit_breachcharge</code></summary>

default: `"3"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ammo_grenade_limit_bumpmine</code></summary>

default: `"3"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ammo_grenade_limit_default</code></summary>

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ammo_grenade_limit_flashbang</code></summary>

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ammo_grenade_limit_snowballs</code></summary>

default: `"3"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ammo_grenade_limit_total</code></summary>

default: `"3"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ammo_item_limit_healthshot</code></summary>

default: `"4"`  
flags: `0x82000`  
</details>
<details>
<summary><code>anim_3wayblend</code></summary>

Toggle the 3-way animation blending code.

default: `"1"`  
flags: `0x2000`  
</details>
<details>
<summary><code>anim_twistbones_enabled</code></summary>

Enable procedural twist bones.

default: `"0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>bot_autodifficulty_threshold_high</code></summary>

Upper bound above Average Human Contribution Score that a bot must be above to change its difficulty

default: `"5.0"`  
flags: `0x82000`  
min value: `-20`  
max value: `20`  
</details>
<details>
<summary><code>bot_autodifficulty_threshold_low</code></summary>

Lower bound below Average Human Contribution Score that a bot must be below to change its difficulty

default: `"-2.0"`  
flags: `0x82000`  
min value: `-20`  
max value: `20`  
</details>
<details>
<summary><code>c_maxdistance</code></summary>

default: `"200"`  
flags: `0x80`  
</details>
<details>
<summary><code>c_maxpitch</code></summary>

default: `"90"`  
flags: `0x80`  
</details>
<details>
<summary><code>c_maxyaw</code></summary>

default: `"135"`  
flags: `0x80`  
</details>
<details>
<summary><code>c_mindistance</code></summary>

default: `"30"`  
flags: `0x80`  
</details>
<details>
<summary><code>c_minpitch</code></summary>

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>c_minyaw</code></summary>

default: `"-135"`  
flags: `0x80`  
</details>
<details>
<summary><code>c_orthoheight</code></summary>

default: `"100"`  
flags: `0x80`  
</details>
<details>
<summary><code>c_orthowidth</code></summary>

default: `"100"`  
flags: `0x80`  
</details>
<details>
<summary><code>c_thirdpersonshoulder</code></summary>

default: `"false"`  
flags: `0x80`  
</details>
<details>
<summary><code>c_thirdpersonshoulderaimdist</code></summary>

default: `"120.0"`  
flags: `0x80`  
</details>
<details>
<summary><code>c_thirdpersonshoulderdist</code></summary>

default: `"40.0"`  
flags: `0x80`  
</details>
<details>
<summary><code>c_thirdpersonshoulderheight</code></summary>

default: `"5.0"`  
flags: `0x80`  
</details>
<details>
<summary><code>c_thirdpersonshoulderoffset</code></summary>

default: `"20.0"`  
flags: `0x80`  
</details>
<details>
<summary><code>cachedvalue_count_partybrowser</code></summary>

Location of the cached blog file

default: `"0"`  
flags: `0x90`  
</details>
<details>
<summary><code>cachedvalue_count_teammates</code></summary>

Twitch.tv account channel URL

default: `"0"`  
flags: `0x90`  
</details>
<details>
<summary><code>cam_collision</code></summary>

When in thirdperson and cam_collision is set to 1, an attempt is made to keep the camera from passing though walls.

default: `"1"`  
flags: `0x10000080`  
</details>
<details>
<summary><code>cam_idealdelta</code></summary>

Controls the speed when matching offset to ideal angles in thirdperson view

default: `"4.0"`  
flags: `0x80`  
</details>
<details>
<summary><code>cam_idealdist</code></summary>

default: `"150"`  
flags: `0x10000080`  
</details>
<details>
<summary><code>cam_idealdistright</code></summary>

default: `"0"`  
flags: `0x10000080`  
</details>
<details>
<summary><code>cam_idealdistup</code></summary>

default: `"0"`  
flags: `0x10000080`  
</details>
<details>
<summary><code>cam_ideallag</code></summary>

Amount of lag used when matching offset to ideal angles in thirdperson view

default: `"4.0"`  
flags: `0x80`  
</details>
<details>
<summary><code>cam_idealpitch</code></summary>

default: `"0"`  
flags: `0x10000080`  
</details>
<details>
<summary><code>cam_idealyaw</code></summary>

default: `"0"`  
flags: `0x10000080`  
</details>
<details>
<summary><code>cam_showangles</code></summary>

When in thirdperson, print viewangles/idealangles/cameraoffsets to the console.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cam_snapto</code></summary>

List HUD elements and their alt_tick usage

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>cash_player_bomb_defused</code></summary>

default: `"300"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_player_bomb_planted</code></summary>

default: `"300"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_player_damage_hostage</code></summary>

default: `"-30"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_player_get_killed</code></summary>

default: `"0"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_player_interact_with_hostage</code></summary>

default: `"150"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_player_killed_enemy_default</code></summary>

default: `"300"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_player_killed_enemy_factor</code></summary>

default: `"1"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_player_killed_hostage</code></summary>

default: `"-1000"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_player_killed_teammate</code></summary>

default: `"-300"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_player_rescued_hostage</code></summary>

default: `"1000"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_player_respawn_amount</code></summary>

default: `"0"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_team_elimination_bomb_map</code></summary>

default: `"3250"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_team_elimination_hostage_map_ct</code></summary>

default: `"2000"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_team_elimination_hostage_map_t</code></summary>

default: `"1000"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_team_hostage_alive</code></summary>

default: `"0"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_team_hostage_interaction</code></summary>

default: `"500"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_team_loser_bonus</code></summary>

default: `"1400"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_team_loser_bonus_consecutive_rounds</code></summary>

default: `"500"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_team_planted_bomb_but_defused</code></summary>

default: `"800"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_team_rescued_hostage</code></summary>

default: `"0"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_team_survive_guardian_wave</code></summary>

default: `"1000"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_team_terrorist_win_bomb</code></summary>

default: `"3500"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_team_win_by_defusing_bomb</code></summary>

default: `"3250"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_team_win_by_hostage_rescue</code></summary>

default: `"3500"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_team_win_by_time_running_out_bomb</code></summary>

default: `"3250"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_team_win_by_time_running_out_hostage</code></summary>

default: `"3250"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_team_winner_bonus_consecutive_rounds</code></summary>

default: `"0"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cc_linger_time</code></summary>

Close caption linger time.

default: `"1.0"`  
flags: `0x80`  
</details>
<details>
<summary><code>cc_predisplay_time</code></summary>

Close caption delay before showing caption.

default: `"0.25"`  
flags: `0x80`  
</details>
<details>
<summary><code>cc_showmissing</code></summary>

Show missing closecaption entries.

default: `"0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>cc_subtitles</code></summary>

If set, don't show sound effect captions, just voice overs (i.e., won't help hearing impaired players).

default: `"0"`  
flags: `0x1000080`  
</details>
<details>
<summary><code>choreo_spew_filter</code></summary>

Spew choreo. Use a sub-string or * to display all events.

default: `""`  
flags: `0x2000`  
</details>
<details>
<summary><code>cl_autobuy</code></summary>

The order in which autobuy will attempt to purchase items

default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>cl_autohelp</code></summary>

Auto-help

default: `"1"`  
flags: `0x280`  
</details>
<details>
<summary><code>cl_autowepswitch</code></summary>

Automatically switch to picked up weapons (if more powerful)

default: `"1"`  
flags: `0x1008280`  
</details>
<details>
<summary><code>cl_backspeed</code></summary>

default: `"450"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_bob_lower_amt</code></summary>

The amount the viewmodel lowers when running

default: `"21"`  
flags: `0x80`  
min value: `5`  
max value: `30`  
</details>
<details>
<summary><code>cl_bob_version</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_bobamt_lat</code></summary>

The amount the viewmodel moves side to side when running

default: `"0.4"`  
flags: `0x80`  
min value: `0.1`  
max value: `2`  
</details>
<details>
<summary><code>cl_bobamt_vert</code></summary>

The amount the viewmodel moves up and down when running

default: `"0.25"`  
flags: `0x80`  
min value: `0.1`  
max value: `2`  
</details>
<details>
<summary><code>cl_bobcycle</code></summary>

the frequency at which the viewmodel bobs.

default: `"0.98"`  
flags: `0x80`  
min value: `0.1`  
max value: `2`  
</details>
<details>
<summary><code>cl_bobup</code></summary>

default: `"0.5"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_brushfastpath</code></summary>

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_cam_driver_compensation_scale</code></summary>



default: `"0.75"`  
flags: `0x80000`  
</details>
<details>
<summary><code>cl_camera_follow_bone_index</code></summary>

Index of the bone to follow.  -2 == disabled.  -1 == root bone.  0+ is bone index.

default: `"-2"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_camera_height_restriction_debug</code></summary>



default: `"0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>cl_chatfilter_version</code></summary>

Stores the chat filter version

default: `"0"`  
flags: `0x98`  
</details>
<details>
<summary><code>cl_chatfilters</code></summary>

Stores the chat filter settings 

default: `"63"`  
flags: `0x88`  
</details>
<details>
<summary><code>cl_clanid</code></summary>

Current clan ID for name decoration

default: `"0"`  
flags: `0x290`  
</details>
<details>
<summary><code>cl_compass_enabled</code></summary>



default: `"1"`  
flags: `0x80080`  
</details>
<details>
<summary><code>cl_connection_trouble_show</code></summary>

Show connection trouble HUD warnings

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>cl_countbones</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_crosshair_drawoutline</code></summary>

Draws a black outline around the crosshair for better visibility

default: `"1"`  
flags: `0x8088`  
</details>
<details>
<summary><code>cl_crosshair_dynamic_maxdist_splitratio</code></summary>

If using cl_crosshairstyle 2, this is the ratio used to determine how long the inner and outer xhair pips will be. [inner = cl_crosshairsize*(1-cl_crosshair_dynamic_maxdist_splitratio), outer = cl_crosshairsize*cl_crosshair_dynamic_maxdist_splitratio]  [0 - 1]

default: `"0.35"`  
flags: `0x8088`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>cl_crosshair_dynamic_splitalpha_innermod</code></summary>

If using cl_crosshairstyle 2, this is the alpha modification that will be used for the INNER crosshair pips once they've split. [0 - 1]

default: `"1"`  
flags: `0x8088`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>cl_crosshair_dynamic_splitalpha_outermod</code></summary>

If using cl_crosshairstyle 2, this is the alpha modification that will be used for the OUTER crosshair pips once they've split. [0.3 - 1]

default: `"0.5"`  
flags: `0x8088`  
min value: `0.3`  
max value: `1`  
</details>
<details>
<summary><code>cl_crosshair_dynamic_splitdist</code></summary>

If using cl_crosshairstyle 2, this is the distance that the crosshair pips will split into 2. (default is 7)

default: `"7"`  
flags: `0x8088`  
</details>
<details>
<summary><code>cl_crosshair_outlinethickness</code></summary>

Set how thick you want your crosshair outline to draw (0.1-3)

default: `"1"`  
flags: `0x8088`  
min value: `0.1`  
max value: `3`  
</details>
<details>
<summary><code>cl_crosshair_recoil</code></summary>

Recoil/aimpunch will move the user's crosshair to show the effect

default: `"0"`  
flags: `0x84000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>cl_crosshair_sniper_show_normal_inaccuracy</code></summary>

Include standing inaccuracy when determining sniper crosshair blur

default: `"0"`  
flags: `0x8088`  
</details>
<details>
<summary><code>cl_crosshair_sniper_width</code></summary>

If >1 sniper scope cross lines gain extra width (1 for single-pixel hairline)

default: `"1"`  
flags: `0x8088`  
</details>
<details>
<summary><code>cl_crosshair_t</code></summary>

T style crosshair

default: `"0"`  
flags: `0x8088`  
</details>
<details>
<summary><code>cl_crosshairalpha</code></summary>

default: `"200"`  
flags: `0x8088`  
</details>
<details>
<summary><code>cl_crosshaircolor</code></summary>

Set crosshair color as defined in game_options.consoles.txt

default: `"1"`  
flags: `0x1008088`  
</details>
<details>
<summary><code>cl_crosshaircolor_b</code></summary>

default: `"50"`  
flags: `0x8088`  
</details>
<details>
<summary><code>cl_crosshaircolor_g</code></summary>

default: `"250"`  
flags: `0x8088`  
</details>
<details>
<summary><code>cl_crosshaircolor_r</code></summary>

default: `"50"`  
flags: `0x8088`  
</details>
<details>
<summary><code>cl_crosshairdot</code></summary>

default: `"1"`  
flags: `0x8088`  
</details>
<details>
<summary><code>cl_crosshairgap</code></summary>

default: `"1"`  
flags: `0x8088`  
</details>
<details>
<summary><code>cl_crosshairgap_useweaponvalue</code></summary>

If set to 1, the gap will update dynamically based on which weapon is currently equipped

default: `"0"`  
flags: `0x8088`  
</details>
<details>
<summary><code>cl_crosshairscale</code></summary>

Crosshair scaling factor (deprecated)

default: `"0"`  
flags: `0x8088`  
</details>
<details>
<summary><code>cl_crosshairsize</code></summary>

default: `"5"`  
flags: `0x8088`  
</details>
<details>
<summary><code>cl_crosshairstyle</code></summary>

0 = DEFAULT, 1 = DEFAULT STATIC, 2 = ACCURATE SPLIT (accurate recoil/spread feedback with a fixed inner part), 3 = ACCURATE DYNAMIC (accurate recoil/spread feedback), 4 = CLASSIC STATIC, 5 = OLD CS STYLE (fake recoil - inaccurate feedback)

default: `"2"`  
flags: `0x8088`  
</details>
<details>
<summary><code>cl_crosshairthickness</code></summary>

default: `"0.5"`  
flags: `0x8088`  
</details>
<details>
<summary><code>cl_crosshairusealpha</code></summary>

default: `"1"`  
flags: `0x8088`  
</details>
<details>
<summary><code>cl_custommaterial_debug_graph</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_dangerzone_approaching_sound_radius</code></summary>

default: `"700"`  
flags: `0x84008`  
</details>
<details>
<summary><code>cl_dangerzone_moving_sound_volume</code></summary>

default: `"0.5"`  
flags: `0x84008`  
</details>
<details>
<summary><code>cl_dangerzone_sound_volume</code></summary>

default: `"0.2"`  
flags: `0x84008`  
</details>
<details>
<summary><code>cl_debugrumble</code></summary>

Turn on rumble debugging spew

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>cl_detail_avoid_force</code></summary>

force with which to avoid players ( in units, percentage of the width of the detail sprite )

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>cl_detail_avoid_radius</code></summary>

radius around detail sprite to avoid players

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>cl_detail_avoid_recover_speed</code></summary>

how fast to recover position after avoiding players

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>cl_detail_max_sway</code></summary>

Amplitude of the detail prop sway

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>cl_detail_multiplier</code></summary>

extra details to create

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_disable_ragdolls</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_disablefreezecam</code></summary>

Turn on/off freezecam on client

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>cl_disablehtmlmotd</code></summary>

Disable HTML motds.

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>cl_dm_buyrandomweapons</code></summary>

Player will automatically receive a random weapon on spawn in deathmatch if this is set to 1 (otherwise, they will receive the last weapon)

default: `"1"`  
flags: `0x80088`  
</details>
<details>
<summary><code>cl_draw_only_deathnotices</code></summary>

For drawing only the crosshair and death notices (used for moviemaking)

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>cl_drawhud</code></summary>

Enable the rendering of the hud

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_drawhud_force_deathnotices</code></summary>

0: default; 1: draw deathnotices even if hud disabled; -1: force no deathnotices

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>cl_drawhud_force_radar</code></summary>

0: default; 1: draw radar even if hud disabled; -1: force no radar

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>cl_drawhud_force_teamid_overhead</code></summary>

0: default; 1: draw teamid even if hud disabled; -1: force no teamid

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>cl_drawhud_specvote</code></summary>

1: default; 0: disables vote UI for spectators

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>cl_drawleaf</code></summary>

default: `"-1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_drawmaterial</code></summary>

Draw a particular material over the frame

default: `""`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_drawshadowtexture</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_dz_playagain_auto_spectate</code></summary>

Automatically switch to spectate mode after clicking the 'Play Again' button in end of match screen

default: `"0"`  
flags: `0x80080`  
</details>
<details>
<summary><code>cl_embedded_stream_audio_volume_xmaster</code></summary>

Whether embedded stream audio volume gets multiplied by master volume

default: `"1"`  
flags: `0x90`  
</details>
<details>
<summary><code>cl_extrapolate</code></summary>

Enable/disable extrapolation if interpolation history runs out.

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_extrapolate_amount</code></summary>

Set how many seconds the client will extrapolate entities for.

default: `"0.25"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_fastdetailsprites</code></summary>

whether to use new detail sprite system

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_fixedcrosshairgap</code></summary>

How big to make the gap between the pips in the fixed crosshair

default: `"3"`  
flags: `0x8080`  
</details>
<details>
<summary><code>cl_foot_contact_shadows</code></summary>



default: `"1"`  
flags: `0x80008`  
</details>
<details>
<summary><code>cl_forwardspeed</code></summary>

default: `"450"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_freezecameffects_showholiday</code></summary>

Happy holidays from the CS:GO team and Valve!

default: `"0"`  
flags: `0x80008`  
</details>
<details>
<summary><code>cl_freezecampanel_position_dynamic</code></summary>

Turn on/off freezecam's kill panel dynamic Y movement

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>cl_http_log_enable</code></summary>

Allows sending HTTP log from client main menu.

default: `"0"`  
flags: `0x400a0000`  
</details>
<details>
<summary><code>cl_hud_background_alpha</code></summary>



default: `"0.5"`  
flags: `0x80088`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>cl_hud_bomb_under_radar</code></summary>



default: `"1"`  
flags: `0x80088`  
</details>
<details>
<summary><code>cl_hud_color</code></summary>

0 = default, 1 = light blue, 2 = orange, 3 = green, 4 = purple, 5 = white.

default: `"0"`  
flags: `0x80088`  
</details>
<details>
<summary><code>cl_hud_healthammo_style</code></summary>



default: `"0"`  
flags: `0x80088`  
</details>
<details>
<summary><code>cl_hud_playercount_pos</code></summary>

0 = default (top), 1 = bottom

default: `"0"`  
flags: `0x80088`  
</details>
<details>
<summary><code>cl_hud_playercount_showcount</code></summary>

0 = show player avatars (default), 1 = just show count number (no avatars)

default: `"0"`  
flags: `0x80088`  
</details>
<details>
<summary><code>cl_idealpitchscale</code></summary>

default: `"0.8"`  
flags: `0x80`  
</details>
<details>
<summary><code>cl_interpolate</code></summary>

Enables or disables interpolation on listen servers or during demo playback

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>cl_inventory_debug_tooltip</code></summary>

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>cl_inventory_saved_filter2</code></summary>

Embedded stream video playing state

default: `"all"`  
flags: `0x80080`  
</details>
<details>
<summary><code>cl_inventory_saved_sort2</code></summary>

default: `"inv_sort_age"`  
flags: `0x80080`  
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
<summary><code>cl_join_advertise</code></summary>

Advertise joinable game in progress to Steam friends, otherwise need a Steam invite (2: all servers, 1: official servers, 0: none)

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>cl_lagcompensation</code></summary>

Perform server side lag compensation of weapon firing events.

default: `"1"`  
flags: `0x400200`  
</details>
<details>
<summary><code>cl_leafsystemvis</code></summary>

Support casting RTT shadows onto other renderables

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_leveloverview</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_leveloverviewmarker</code></summary>

Automatically reloads the animation script each time one is ran

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_lock_camera</code></summary>

Override view during demo playback

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_mainmenu_show_datagraph</code></summary>



default: `""`  
flags: `0x80008`  
</details>
<details>
<summary><code>cl_maxrenderable_dist</code></summary>

Max distance from the camera at which things will be rendered

default: `"3000"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_minimal_rtt_shadows</code></summary>

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>cl_mouselook</code></summary>

Set to 1 to use mouse for look, 0 for keyboard look. Cannot be set while connected to a server.

default: `"1"`  
flags: `0x408080`  
</details>
<details>
<summary><code>cl_mute_all_but_friends_and_party</code></summary>

Only allow communication from friends and matchmaking party members. Doesn't apply to competitive matchmaking games.

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>cl_mute_enemy_team</code></summary>

Block all communication from players on the enemy team.

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>cl_obs_interp_enable</code></summary>

Enables interpolation between observer targets

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>cl_observercrosshair</code></summary>

default: `"1"`  
flags: `0x8080`  
</details>
<details>
<summary><code>cl_overdraw_test</code></summary>

default: `"0"`  
flags: `0x5000`  
</details>
<details>
<summary><code>cl_particle_retire_cost</code></summary>

How aggressive the switch to fallbacks will be depending on how far over the cl_particle_sim_fallback_threshold_ms the sim time is.  Higher numbers are more aggressive.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_particles_show_bbox</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_particles_show_controlpoints</code></summary>

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
<summary><code>cl_phys_show_active</code></summary>

Use a prediction-friendly physics interface on the client

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
<summary><code>cl_pitchdown</code></summary>

0-analog stick style. 1-pointer style.

default: `"89"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_pitchup</code></summary>

default: `"89"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_player_ping_mute</code></summary>

If 1, player pinging will make a sound, if 0, pings will be silent

default: `"0"`  
flags: `0x80080`  
</details>
<details>
<summary><code>cl_player_proximity_debug</code></summary>



default: `"0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>cl_playerspray_auto_apply</code></summary>

Automatically apply graffiti when graffiti menu closes

default: `"1"`  
flags: `0x80080`  
</details>
<details>
<summary><code>cl_portal_use_new_dissolve</code></summary>

Use new dissolve effect

default: `"1"`  
flags: `0x4000`  
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
flags: `0x400200`  
</details>
<details>
<summary><code>cl_quickinventory_lastinv</code></summary>

default: `"1"`  
flags: `0x80088`  
</details>
<details>
<summary><code>cl_quickinventory_line_update_speed</code></summary>

default: `"65.0f"`  
flags: `0x80088`  
</details>
<details>
<summary><code>cl_radar_always_centered</code></summary>

If set to 0, the radar is maximally used. Otherwise the player is always centered, even at map extents.

default: `"1"`  
flags: `0x80080`  
</details>
<details>
<summary><code>cl_radar_icon_scale_min</code></summary>

Sets the minimum icon scale. Valid values are 0.4 to 1.25.

default: `"0.6"`  
flags: `0x80080`  
min value: `0.4`  
max value: `1.25`  
</details>
<details>
<summary><code>cl_radar_rotate</code></summary>

1

default: `"1"`  
flags: `0x80080`  
</details>
<details>
<summary><code>cl_radar_scale</code></summary>

Sets the radar scale. Valid values are 0.25 to 1.0.

default: `"0.7"`  
flags: `0x80080`  
min value: `0.25`  
max value: `1`  
</details>
<details>
<summary><code>cl_radar_square_with_scoreboard</code></summary>

If set, the radar will toggle to square when the scoreboard is visible.

default: `"1"`  
flags: `0x80080`  
</details>
<details>
<summary><code>cl_radial_radio_tab</code></summary>



default: `"0"`  
flags: `0x80088`  
min value: `0`  
max value: `3`  
</details>
<details>
<summary><code>cl_radialmenu_deadzone_size</code></summary>



default: `"0.04"`  
flags: `0x80088`  
min value: `0.01`  
max value: `0.2`  
</details>
<details>
<summary><code>cl_ragdoll_workaround_threshold</code></summary>

Mainly cosmetic, client-only effect: when client doesn't know the last position of another player that spawns a ragdoll, the ragdoll creation is simplified and ragdoll is created in the right place. If you increase this significantly, ragdoll positions on your client may be dramatically wrong, but it won't affect other clients

default: `"4"`  
flags: `0x80000`  
</details>
<details>
<summary><code>cl_rappel_tilt</code></summary>

default: `"0.35"`  
flags: `0x80000`  
</details>
<details>
<summary><code>cl_rebuy</code></summary>

The order in which rebuy will attempt to repurchase items

default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>cl_remove_old_ugc_downloads</code></summary>

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>cl_righthand</code></summary>

Use right-handed view models.

default: `"1"`  
flags: `0x8080`  
</details>
<details>
<summary><code>cl_rumblescale</code></summary>

Scale sensitivity of rumble effects (0 to 1.0)

default: `"1.0"`  
flags: `0x8080`  
</details>
<details>
<summary><code>cl_sanitize_player_names</code></summary>

Replace names of other players with something non-offensive.

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>cl_scalecrosshair</code></summary>

Enable crosshair scaling (deprecated)

default: `"1"`  
flags: `0x8088`  
</details>
<details>
<summary><code>cl_scoreboard_mouse_enable_binding</code></summary>

Name of the binding to enable mouse selection in the scoreboard

default: `"+attack2"`  
flags: `0x80`  
</details>
<details>
<summary><code>cl_scoreboard_survivors_always_on</code></summary>

default: `"0"`  
flags: `0x80080`  
</details>
<details>
<summary><code>cl_server_graphic1_enable</code></summary>

When enabled, 360x60 (<16kb) image file will be displayed to on-server spectators.

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>cl_server_graphic2_enable</code></summary>

When enabled, 220x45 (<16kb) image file will be displayed to on-server spectators.

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>cl_shadowtextureoverlaysize</code></summary>

default: `"256"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_show_clan_in_death_notice</code></summary>

Is set, the clan name will show next to player names in the death notices.

default: `"1"`  
flags: `0x80088`  
</details>
<details>
<summary><code>cl_showanimstate</code></summary>

Show the (client) animation state for the specified entity (-1 for none).

default: `"-1"`  
flags: `0x4002`  
</details>
<details>
<summary><code>cl_showanimstate_activities</code></summary>

Show activities in the (client) animation state display.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_showanimstate_log</code></summary>

1 to output cl_showanimstate to Msg(). 2 to store in AnimStateClient.log. 3 for both.

default: `"0"`  
flags: `0x4002`  
</details>
<details>
<summary><code>cl_showerror</code></summary>

Show prediction errors, 2 for above plus detailed field deltas.

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>cl_showfps</code></summary>

Draw fps meter (1 = fps, 2 = smooth, 3 = server, 4 = Show+LogToFile, 5 = Thread and wait times +10 = detailed )

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>cl_showhelp</code></summary>

Set to 0 to not show on-screen help

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>cl_showpos</code></summary>

Draw current position at top of screen

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>cl_sidespeed</code></summary>

default: `"450"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_simdbones</code></summary>

Use SIMD bone setup.

default: `"0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>cl_skipfastpath</code></summary>

Set to 1 to stop all models that go through the model fast path from rendering

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_skipslowpath</code></summary>

Set to 1 to skip any models that don't go through the model fast path

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_spec_follow_grenade_key</code></summary>

0 = LALT, 1 = LSHIFT, 2 = +reload

default: `"0"`  
flags: `0x80088`  
</details>
<details>
<summary><code>cl_spec_mode</code></summary>

Saves the last viewed spectator mode for use next time we start to spectate

default: `"0"`  
flags: `0x10008288`  
</details>
<details>
<summary><code>cl_spec_show_bindings</code></summary>

Toggle the visibility of the spectator bindings.

default: `"1"`  
flags: `0x40080000`  
</details>
<details>
<summary><code>cl_spec_stats</code></summary>

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>cl_spec_use_tournament_content_standards</code></summary>

default: `"0.0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>cl_sporeclipdistance</code></summary>

default: `"512"`  
flags: `0x4008`  
</details>
<details>
<summary><code>cl_sun_decay_rate</code></summary>

default: `"0.05"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_sun_in_reflection_h_scale</code></summary>

default: `"2.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_sun_in_reflection_v_scale</code></summary>

default: `"2.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_sunlight_ortho_size</code></summary>

Set to values greater than 0 for ortho view render projections.

default: `"0.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_tablet_mapmode</code></summary>

default: `"1"`  
flags: `0x80080`  
</details>
<details>
<summary><code>cl_teamid_overhead_maxdist</code></summary>

max distance at which the overhead team id icons will show

default: `"3000"`  
flags: `0xc000`  
</details>
<details>
<summary><code>cl_teamid_overhead_maxdist_spec</code></summary>

max distance at which the overhead team id icons will show when a spectator

default: `"2000"`  
flags: `0xc000`  
</details>
<details>
<summary><code>cl_teammate_colors_show</code></summary>

In competitive, 1 = show teammates as separate colors in the radar, scoreboard, etc., 2 = show colors and letters

default: `"1"`  
flags: `0x80088`  
</details>
<details>
<summary><code>cl_threaded_bone_setup</code></summary>

Enable parallel processing of bones

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>cl_upspeed</code></summary>

default: `"320"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_use_new_headbob</code></summary>

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_use_opens_buy_menu</code></summary>

Pressing the +use key will open the buy menu if in a buy zone (just as if you pressed the 'buy' key).

default: `"1"`  
flags: `0x1008280`  
</details>
<details>
<summary><code>cl_use_simd_bones</code></summary>

1 use SIMD bones 0 use scalar bones.

default: `"1"`  
flags: `0x2000`  
</details>
<details>
<summary><code>cl_viewmodel_shift_left_amt</code></summary>

The amount the viewmodel shifts to the left when shooting accuracy increases.

default: `"1.5"`  
flags: `0x80`  
min value: `0.5`  
max value: `2`  
</details>
<details>
<summary><code>cl_viewmodel_shift_right_amt</code></summary>

The amount the viewmodel shifts to the right when shooting accuracy decreases.

default: `"0.75"`  
flags: `0x80`  
min value: `0.25`  
max value: `2`  
</details>
<details>
<summary><code>cl_weapon_clip_thinwalls</code></summary>

Enable outline selecting victim in hltv replay: 0 - none; 1 - ouline YOU; 2 - outline YOU, with red ragdoll outline; 3 - normal spectator outlines

default: `"1"`  
flags: `0x6000`  
</details>
<details>
<summary><code>cl_weapon_clip_thinwalls_debug</code></summary>

default: `"0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>cl_weapon_clip_thinwalls_lock</code></summary>

default: `"0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>cl_weapon_debug_print_accuracy</code></summary>

default: `"0"`  
flags: `0x84000`  
</details>
<details>
<summary><code>cl_weapon_debug_show_accuracy</code></summary>

Draws a circle representing the effective range with every shot.

default: `"0"`  
flags: `0x84000`  
</details>
<details>
<summary><code>cl_weapon_debug_show_accuracy_duration</code></summary>

default: `"10"`  
flags: `0x84000`  
</details>
<details>
<summary><code>cl_winddir</code></summary>

Weather effects wind direction angle

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_windspeed</code></summary>

Weather effects wind speed scalar

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cl_wpn_sway_scale</code></summary>

default: `"1.6"`  
flags: `0x4008`  
</details>
<details>
<summary><code>closecaption</code></summary>

Enable close captioning.

default: `"0"`  
flags: `0x1000080`  
</details>
<details>
<summary><code>closeonbuy</code></summary>

Set non-zero to close the buy menu after buying something

default: `"0"`  
flags: `0x8080`  
</details>
<details>
<summary><code>cloth_windage_multiplier</code></summary>

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>commentary_firstrun</code></summary>

Connect string for demo UI

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>crosshair</code></summary>

Shows all player names (including bots) as 16 W's.

default: `"1"`  
flags: `0x8080`  
</details>
<details>
<summary><code>custom_bot_difficulty</code></summary>

Bot difficulty for offline play.

default: `"0"`  
flags: `0x8200c`  
</details>
<details>
<summary><code>debug_aim_angle</code></summary>

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>debug_entity_outline_highlight</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>default_fov</code></summary>

Faster way of placing icons on the mini map.

default: `"90"`  
flags: `0x4000`  
</details>
<details>
<summary><code>developer</code></summary>

Set developer message level

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>enable_skeleton_draw</code></summary>

Render skeletons in wireframe

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>ff_damage_bullet_penetration</code></summary>

If friendly fire is off, this will scale the penetration power and damage a bullet does when penetrating another friendly player

default: `"0"`  
flags: `0x82000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>ff_damage_reduction_bullets</code></summary>

How much to reduce damage done to teammates when shot.  Range is from 0 - 1 (with 1 being damage equal to what is done to an enemy)

default: `"0.1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ff_damage_reduction_grenade</code></summary>

How much to reduce damage done to teammates by a thrown grenade.  Range is from 0 - 1 (with 1 being damage equal to what is done to an enemy)

default: `"0.25"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ff_damage_reduction_grenade_self</code></summary>

How much to damage a player does to himself with his own grenade.  Range is from 0 - 1 (with 1 being damage equal to what is done to an enemy)

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ff_damage_reduction_other</code></summary>

How much to reduce damage done to teammates by things other than bullets and grenades.  Range is from 0 - 1 (with 1 being damage equal to what is done to an enemy)

default: `"0.25"`  
flags: `0x82000`  
</details>
<details>
<summary><code>fish_debug</code></summary>

Show debug info for fish

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>fog_color</code></summary>

default: `"-1 -1 -1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>fog_colorskybox</code></summary>

default: `"-1 -1 -1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>fog_enable</code></summary>

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>fog_enableskybox</code></summary>

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>fog_end</code></summary>

default: `"-1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>fog_endskybox</code></summary>

default: `"-1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>fog_hdrcolorscale</code></summary>

default: `"-1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>fog_hdrcolorscaleskybox</code></summary>

default: `"-1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>fog_maxdensity</code></summary>

default: `"-1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>fog_maxdensityskybox</code></summary>

default: `"-1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>fog_start</code></summary>

default: `"-1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>fog_startskybox</code></summary>

default: `"-1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>fov_cs_debug</code></summary>

Sets the view fov if cheats are on.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>func_break_max_pieces</code></summary>

Maximum downwards speed of shattered glass particles

default: `"15"`  
flags: `0x2080`  
</details>
<details>
<summary><code>g15_update_msec</code></summary>

Logitech G-15 Keyboard update interval.

default: `"250"`  
flags: `0x80`  
</details>
<details>
<summary><code>g_Language</code></summary>

default: `"0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>g_debug_ragdoll_removal</code></summary>

default: `"0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>g_debug_ragdoll_visualize</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>g_ragdoll_important_maxcount</code></summary>

default: `"2"`  
flags: `0x2000`  
</details>
<details>
<summary><code>g_ragdoll_maxcount</code></summary>

After this many seconds of being basically stationary, the ragdoll will go to sleep.

default: `"8"`  
flags: `0x2000`  
</details>
<details>
<summary><code>game_mode</code></summary>

The current game mode (based on game type). See GameModes.txt.

default: `"0"`  
flags: `0x8200c`  
</details>
<details>
<summary><code>game_online</code></summary>

The current game is online.

default: `"1"`  
flags: `0x201c`  
</details>
<details>
<summary><code>game_public</code></summary>

The current game is public.

default: `"1"`  
flags: `0x201c`  
</details>
<details>
<summary><code>game_type</code></summary>

The current game type. See GameModes.txt.

default: `"0"`  
flags: `0x8200c`  
</details>
<details>
<summary><code>gameinstructor_find_errors</code></summary>

Set to 1 and the game instructor will run EVERY scripted command to uncover errors.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>gameinstructor_save_restore_lessons</code></summary>

Set to 0 to disable save/load of open lesson opportunities in single player.

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>gameinstructor_verbose</code></summary>

Set to 1 for standard debugging or 2 (in combo with gameinstructor_verbose_lesson) to show update actions.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>gameinstructor_verbose_lesson</code></summary>

Display more verbose information for lessons have this name.

default: `""`  
flags: `0x4000`  
</details>
<details>
<summary><code>gl_clear_randomcolor</code></summary>

Clear the back buffer to random colors every frame. Helps spot open seams in geometry.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>glow_muzzle_debug</code></summary>

Show muzzle glow shapes outside of the glow pass.

default: `"0"`  
flags: `0x4002`  
</details>
<details>
<summary><code>glow_outline_effect_enable</code></summary>

Enable entity outline glow effects.

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>glow_outline_width</code></summary>

Width of glow outline effect in screen space.

default: `"6.0f"`  
flags: `0x4000`  
</details>
<details>
<summary><code>gotv_theater_container</code></summary>

Enables GOTV theater mode for the specified container, setting it to 'live' will play top live matches

default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>healthshot_allow_use_at_full</code></summary>

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>healthshot_health</code></summary>

default: `"50"`  
flags: `0x82000`  
</details>
<details>
<summary><code>healthshot_healthboost_speed_multiplier</code></summary>

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>healthshot_healthboost_time</code></summary>

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>hidehud</code></summary>

default: `"0"`  
flags: `0xc000`  
</details>
<details>
<summary><code>hl2_episodic</code></summary>

default: `"0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>hostage_feetyawrate</code></summary>

How many degrees per second that hostages can turn their feet or upper body.

default: `"720"`  
flags: `0x2002`  
</details>
<details>
<summary><code>hud_fastswitch</code></summary>

default: `"1"`  
flags: `0x8000`  
</details>
<details>
<summary><code>hud_scaling</code></summary>

Scales hud elements

default: `"0.85"`  
flags: `0x80`  
min value: `0.5`  
max value: `0.95`  
</details>
<details>
<summary><code>hud_showtargetid</code></summary>

Enables display of target names

default: `"1"`  
flags: `0x8080`  
</details>
<details>
<summary><code>hud_takesshots</code></summary>

Auto-save a scoreboard screenshot at the end of a map.

default: `"0"`  
flags: `0x88`  
</details>
<details>
<summary><code>in_forceuser</code></summary>

Force user input to this split screen player.

default: `"0"`  
flags: `0x4002`  
</details>
<details>
<summary><code>inferno_dlight_spacing</code></summary>

Inferno dlights are at least this far apart

default: `"200"`  
flags: `0x4000`  
</details>
<details>
<summary><code>joy_accelmax</code></summary>

default: `"1.0"`  
flags: `0x80`  
</details>
<details>
<summary><code>joy_accelscale</code></summary>

default: `"3.5"`  
flags: `0x80`  
</details>
<details>
<summary><code>joy_accelscalepoly</code></summary>

default: `"0.4"`  
flags: `0x80`  
</details>
<details>
<summary><code>joy_advanced</code></summary>

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>joy_advaxisr</code></summary>

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>joy_advaxisu</code></summary>

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>joy_advaxisv</code></summary>

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>joy_advaxisx</code></summary>

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>joy_advaxisy</code></summary>

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>joy_advaxisz</code></summary>

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>joy_autoAimDampenMethod</code></summary>

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>joy_autoaimdampen</code></summary>

How much to scale user stick input when the gun is pointing at a valid target.

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>joy_autoaimdampenrange</code></summary>

The stick range where autoaim dampening is applied. 0 = off

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>joy_cfg_preset</code></summary>

default: `"1"`  
flags: `0x1008080`  
</details>
<details>
<summary><code>joy_circle_correct</code></summary>

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>joy_curvepoint_1</code></summary>



default: `"0.001"`  
flags: `0x80`  
min value: `0.001`  
max value: `5`  
</details>
<details>
<summary><code>joy_curvepoint_2</code></summary>



default: `"0.4"`  
flags: `0x80`  
min value: `0.001`  
max value: `5`  
</details>
<details>
<summary><code>joy_curvepoint_3</code></summary>



default: `"0.75"`  
flags: `0x80`  
min value: `0.001`  
max value: `5`  
</details>
<details>
<summary><code>joy_curvepoint_4</code></summary>



default: `"1"`  
flags: `0x80`  
min value: `0.001`  
max value: `5`  
</details>
<details>
<summary><code>joy_curvepoint_end</code></summary>



default: `"2"`  
flags: `0x80`  
min value: `0.001`  
max value: `5`  
</details>
<details>
<summary><code>joy_diagonalpov</code></summary>

POV manipulator operates on diagonal axes, too.

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>joy_display_input</code></summary>

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>joy_forwardsensitivity</code></summary>

default: `"-1"`  
flags: `0x80`  
</details>
<details>
<summary><code>joy_forwardthreshold</code></summary>

default: `"0.15"`  
flags: `0x80`  
</details>
<details>
<summary><code>joy_gamma</code></summary>

default: `"0.2"`  
flags: `0x80`  
</details>
<details>
<summary><code>joy_inverty</code></summary>

Whether to invert the Y axis of the joystick for looking.

default: `"0"`  
flags: `0x1008080`  
</details>
<details>
<summary><code>joy_lowend</code></summary>

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>joy_lowend_linear</code></summary>

default: `"0.55"`  
flags: `0x80`  
</details>
<details>
<summary><code>joy_lowmap</code></summary>

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>joy_name</code></summary>

default: `"joystick"`  
flags: `0x80`  
</details>
<details>
<summary><code>joy_no_accel_jump</code></summary>

If 0, the 360controller.cfg file will be executed on startup & option changes.

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>joy_pitchsensitivity</code></summary>

joystick pitch sensitivity

default: `"-1"`  
flags: `0x1008080`  
min value: `-5`  
max value: `-0.1`  
</details>
<details>
<summary><code>joy_pitchthreshold</code></summary>

default: `"0.15"`  
flags: `0x80`  
</details>
<details>
<summary><code>joy_response_look</code></summary>

'Look' stick response mode: 0=Default, 1=Acceleration Promotion

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>joy_response_look_pitch</code></summary>

'Look' stick response mode for pitch: 0=Default, 1=Acceleration Promotion

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>joy_response_move</code></summary>

'Movement' stick response mode: 0=Linear, 1=quadratic, 2=cubic, 3=quadratic extreme, 4=power function(i.e., pow(x,1/sensitivity)), 5=two-stage

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>joy_sensitive_step0</code></summary>

default: `"0.1"`  
flags: `0x80`  
</details>
<details>
<summary><code>joy_sensitive_step1</code></summary>

default: `"0.4"`  
flags: `0x80`  
</details>
<details>
<summary><code>joy_sensitive_step2</code></summary>

default: `"0.90"`  
flags: `0x80`  
</details>
<details>
<summary><code>joy_sidesensitivity</code></summary>

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>joy_sidethreshold</code></summary>

default: `"0.15"`  
flags: `0x80`  
</details>
<details>
<summary><code>joy_wingmanwarrior_turnhack</code></summary>

Wingman warrior hack related to turn axes.

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>joy_yawsensitivity</code></summary>

joystick yaw sensitivity

default: `"-1"`  
flags: `0x1008080`  
min value: `-5`  
max value: `-0.1`  
</details>
<details>
<summary><code>joy_yawthreshold</code></summary>

default: `"0.15"`  
flags: `0x80`  
</details>
<details>
<summary><code>joystick_force_disabled</code></summary>

Prevents any and all joystick input for cases where a piece of hardware is incorrectly identified as a joystick an sends bad signals.

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>key_bind_version</code></summary>

default: `"0"`  
flags: `0x80098`  
</details>
<details>
<summary><code>locator_split_len</code></summary>

default: `"0.5f"`  
flags: `0x4000`  
</details>
<details>
<summary><code>locator_split_maxwide_percent</code></summary>

Percentage of the lower half of the screen that the locator will draw at when at its reseting position on the hud.

default: `"0.80f"`  
flags: `0x4000`  
</details>
<details>
<summary><code>lockMoveControllerRet</code></summary>

default: `"0"`  
flags: `0x80`  
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
<summary><code>m_customaccel</code></summary>

Custom mouse acceleration:
0: custom accelaration disabled
1: mouse_acceleration = min(m_customaccel_max, pow(raw_mouse_delta, m_customaccel_exponent) * m_customaccel_scale + sensitivity)
2: Same as 1, with but x and y sensitivity are scaled by m_pitch and m_yaw respectively.
3: mouse_acceleration = pow(raw_mouse_delta, m_customaccel_exponent - 1) * sensitivity

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>m_customaccel_exponent</code></summary>

Mouse move is raised to this power before being scaled by scale factor.

default: `"1.05"`  
flags: `0x80`  
min value: `0.0001`  
max value: `10`  
</details>
<details>
<summary><code>m_customaccel_max</code></summary>

Max mouse move scale factor, 0 for no limit

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>m_customaccel_scale</code></summary>

Custom mouse acceleration value.

default: `"0.04"`  
flags: `0x80`  
min value: `0`  
max value: `10`  
</details>
<details>
<summary><code>m_forward</code></summary>

Mouse forward factor.

default: `"1"`  
flags: `0x80`  
min value: `0.0001`  
max value: `1000`  
</details>
<details>
<summary><code>m_mouseaccel1</code></summary>

Windows mouse acceleration initial threshold (2x movement).

default: `"0"`  
flags: `0x80`  
min value: `0`  
</details>
<details>
<summary><code>m_mouseaccel2</code></summary>

Windows mouse acceleration secondary threshold (4x movement).

default: `"0"`  
flags: `0x80`  
min value: `0`  
</details>
<details>
<summary><code>m_mousespeed</code></summary>

Windows mouse acceleration (0 to disable, 1 to enable [Windows 2000: enable initial threshold], 2 to enable secondary threshold [Windows 2000 only]).

default: `"1"`  
flags: `0x80`  
min value: `0`  
max value: `2`  
</details>
<details>
<summary><code>m_rawinput</code></summary>

Use Raw Input for mouse input.

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>m_side</code></summary>

Mouse side factor.

default: `"0.8"`  
flags: `0x80`  
min value: `0.0001`  
max value: `1000`  
</details>
<details>
<summary><code>m_yaw</code></summary>

Mouse yaw factor.

default: `"0.022"`  
flags: `0x80`  
min value: `0.0001`  
max value: `1000`  
</details>
<details>
<summary><code>mapcycledisabled</code></summary>

repeats the same map after each match instead of using the map cycle

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mapoverview_allow_client_draw</code></summary>

Allow a client to draw on the map overview

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>mapoverview_allow_grid_usage</code></summary>

When set to 1, allows turning on the (experimental) grid for drawing.

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>mapoverview_icon_scale</code></summary>

Sets the icon scale multiplier for the overview map. Valid values are 0.5 to 3.0.

default: `"1.0"`  
flags: `0x80080`  
min value: `0.5`  
max value: `3`  
</details>
<details>
<summary><code>mat_accelerate_adjust_exposure_down</code></summary>

default: `"40.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_autoexposure_max</code></summary>

default: `"2"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_autoexposure_max_multiplier</code></summary>

default: `"1.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_autoexposure_min</code></summary>

default: `"0.5"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_bloom_scalefactor_scalar</code></summary>

default: `"1.0"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mat_bloomamount_rate</code></summary>

default: `"0.05f"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_bloomscale</code></summary>

default: `"1"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mat_camerarendertargetoverlaysize</code></summary>

default: `"128"`  
flags: `0x4000`  
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
flags: `0x4000`  
</details>
<details>
<summary><code>mat_debug_bloom</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_debug_postprocessing_effects</code></summary>

0 = off, 1 = show post-processing passes in quadrants of the screen, 2 = only apply post-processing to the centre of the screen

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_disable_bloom</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_draw_zone_highlight</code></summary>

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_draw_zone_projection_mode</code></summary>

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_drawwater</code></summary>

Enable for optimization to water - considers view in leaf under water for purposes of culling

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_dynamic_tonemapping</code></summary>

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_exposure_center_region_x</code></summary>

default: `"0.9"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_exposure_center_region_y</code></summary>

default: `"0.85"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_force_bloom</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_force_tonemap_min_avglum</code></summary>

Override. Old default was 3.0

default: `"-1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_force_tonemap_percent_bright_pixels</code></summary>

Override. Old value was 2.0

default: `"-1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_force_tonemap_percent_target</code></summary>

Override. Old default was 60.

default: `"-1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_force_tonemap_scale</code></summary>

default: `"0.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_fullbright</code></summary>

Alpha for rope antialiasing effect

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_fullbright</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_fullbright</code></summary>

enable threading of detail prop drawing

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_hdr_uncapexposure</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_hsv</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_lpreview_mode</code></summary>

Threaded BuildWorldList and BuildRenderables list

default: `"-1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_non_hdr_bloom_scalefactor</code></summary>

default: `".3"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mat_normals</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_normals</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_postprocess_enable</code></summary>

Trims the algorithm from processing darks: (0.0312 - visible limit, slower), (0.0625 - high quality, faster), (0.0833 - upper limit, the start of fisible unfiltered edges). Special note: when using FXAA_GREEN_AS_LUMA, likely want to set this to zero

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_preview</code></summary>

Limit how much the view model follows the pointer in looking up.

default: `""`  
flags: `0x4008`  
</details>
<details>
<summary><code>mat_show_histogram</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_showcamerarendertarget</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_showframebuffertexture</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_showwatertextures</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_softwareskin</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_softwareskin</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_stub</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_tonemap_algorithm</code></summary>

0 = Original Algorithm 1 = New Algorithm

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_viewportscale</code></summary>

Scale down the main viewport (to reduce GPU impact on CPU profiling)

default: `"1.0"`  
flags: `0x4000`  
min value: `0.0015625`  
max value: `1`  
</details>
<details>
<summary><code>mat_viewportupscale</code></summary>

Scale the viewport back up

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_wireframe</code></summary>



default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_yuv</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mc_accel_band_size</code></summary>

Percentage of half the screen width or height.

default: `"0.5"`  
flags: `0x80`  
min value: `0.01`  
max value: `2`  
</details>
<details>
<summary><code>mc_dead_zone_radius</code></summary>

0 to 0.9. 0 being just around the center of the screen and 1 being the edges of the screen.

default: `"0.06"`  
flags: `0x80`  
min value: `0`  
max value: `0.9`  
</details>
<details>
<summary><code>mc_max_pitchrate</code></summary>

(degrees/sec)

default: `"100.0"`  
flags: `0x80`  
min value: `10`  
max value: `720`  
</details>
<details>
<summary><code>mc_max_yawrate</code></summary>

(degrees/sec)

default: `"230.0"`  
flags: `0x80`  
min value: `10`  
max value: `720`  
</details>
<details>
<summary><code>molotov_throw_detonate_time</code></summary>

default: `"2.0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_afterroundmoney</code></summary>

amount of money awared to every player after each round

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_allowspectators</code></summary>

toggles whether the server allows spectator mode or not

default: `"1.0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>mp_anyone_can_pickup_c4</code></summary>

If set, everyone can pick up the c4, not just Ts.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_blockstyle</code></summary>

Sets the style of capture point blocking used. 0 = Blocks break captures completely. 1 = Blocks only pause captures.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>mp_bonusroundtime</code></summary>

Time after round win until round restarts

default: `"15"`  
flags: `0x2000`  
min value: `5`  
max value: `15`  
</details>
<details>
<summary><code>mp_buy_allow_grenades</code></summary>

Whether players can purchase grenades from the buy menu or not.

default: `"1"`  
flags: `0x82000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>mp_buy_allow_guns</code></summary>

Whether players can purchase guns: pistols (1), SMGs (2), rifles (4), shotguns (8), sniper rifles (16), heavy MGs (32).

default: `"255"`  
flags: `0x82000`  
min value: `0`  
max value: `255`  
</details>
<details>
<summary><code>mp_buy_anywhere</code></summary>

When set, players can buy anywhere, not only in buyzones. 0 = default. 1 = both teams. 2 = Terrorists. 3 = Counter-Terrorists.

default: `"0"`  
flags: `0x82100`  
</details>
<details>
<summary><code>mp_buy_during_immunity</code></summary>

When set, players can buy when immune, ignoring buytime. 0 = default. 1 = both teams. 2 = Terrorists. 3 = Counter-Terrorists.

default: `"0"`  
flags: `0x82100`  
</details>
<details>
<summary><code>mp_buytime</code></summary>

How many seconds after round start players can buy items for.

default: `"90"`  
flags: `0x82000`  
min value: `0`  
</details>
<details>
<summary><code>mp_c4_cannot_be_defused</code></summary>

If set, the planted c4 cannot be defused.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_c4timer</code></summary>

how long from when the C4 is armed until it blows

default: `"40"`  
flags: `0x82100`  
min value: `10`  
</details>
<details>
<summary><code>mp_capdeteriorate_time</code></summary>

Time it takes for a full capture point to deteriorate.

default: `"90.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>mp_capstyle</code></summary>

Sets the style of capture points used. 0 = Fixed players required to cap. 1 = More players cap faster, but longer cap times.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>mp_coop_force_join_ct</code></summary>

If set, real players will auto join CT on join.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_coopmission_mission_number</code></summary>

Which mission the map should run after it loads.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_ct_default_grenades</code></summary>

The default grenades that the CTs will spawn with.  To give multiple grenades, separate each weapon class with a space like this: 'weapon_molotov weapon_hegrenade'

default: `""`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_ct_default_melee</code></summary>

The default melee weapon that the CTs will spawn with.  Even if this is blank, a knife will be given.  To give a taser, it should look like this: 'weapon_knife weapon_taser'.  Remember to set mp_weapons_allow_zeus to 1 if you want to give a taser!

default: `"weapon_knife"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_ct_default_primary</code></summary>

The default primary (rifle) weapon that the CTs will spawn with

default: `""`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_ct_default_secondary</code></summary>

The default secondary (pistol) weapon that the CTs will spawn with

default: `"weapon_hkp2000"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_death_drop_breachcharge</code></summary>

Drop breachcharge on player death

default: `"1"`  
flags: `0x82000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>mp_death_drop_c4</code></summary>

Whether c4 is droppable

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_death_drop_defuser</code></summary>

Drop defuser on player death

default: `"1"`  
flags: `0x82000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>mp_death_drop_grenade</code></summary>

Which grenade to drop on player death: 0=none, 1=best, 2=current or best, 3=all grenades

default: `"2"`  
flags: `0x82000`  
min value: `0`  
max value: `3`  
</details>
<details>
<summary><code>mp_death_drop_gun</code></summary>

Which gun to drop on player death: 0=none, 1=best, 2=current or best

default: `"1"`  
flags: `0x82000`  
min value: `0`  
max value: `2`  
</details>
<details>
<summary><code>mp_death_drop_healthshot</code></summary>

Drop healthshot on player death

default: `"1"`  
flags: `0x82000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>mp_death_drop_taser</code></summary>

Drop taser on player death

default: `"1"`  
flags: `0x82000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>mp_default_team_winner_no_objective</code></summary>

If the map doesn't define an objective (bomb, hostage, etc), the value of this convar will declare the winner when the time runs out in the round.

default: `"-1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_defuser_allocation</code></summary>

How to allocate defusers to CTs at start or round: 0=none, 1=random, 2=everyone

default: `"0"`  
flags: `0x82000`  
min value: `0`  
max value: `2`  
</details>
<details>
<summary><code>mp_disable_respawn_times</code></summary>

default: `"0"`  
flags: `0x2100`  
</details>
<details>
<summary><code>mp_display_kill_assists</code></summary>

Whether to display and score player assists

default: `"1"`  
flags: `0x82000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>mp_dm_bonus_percent</code></summary>

Percent of points additionally awarded when someone gets a kill with the bonus weapon during the bonus period.

default: `"50"`  
flags: `0x82000`  
min value: `0`  
</details>
<details>
<summary><code>mp_dm_bonus_respawn</code></summary>

When attempting to get the bonus weapon in deathmatch, whether we should respawn you with it or just give it to you directly

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_dm_bonusweapon_dogtags</code></summary>

Additional dogtags to drop when making a kill with the bonus weapon

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_dm_dogtag_score</code></summary>

Points to award for picking up a dogtag in deathmatch.

default: `"0"`  
flags: `0x82000`  
min value: `0`  
</details>
<details>
<summary><code>mp_dm_kill_base_score</code></summary>

Number of base points to award for a kill in deathmatch.  Cheaper weapons award 1 or 2 additional points.

default: `"10"`  
flags: `0x82000`  
min value: `0`  
</details>
<details>
<summary><code>mp_dm_teammode</code></summary>

In deathmatch, enables team DM visuals & scoring

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_dm_teammode_bonus_score</code></summary>

Team deathmatch victory points to award for kill with bonus weapon

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_dm_teammode_dogtag_score</code></summary>

Team deathmatch victory points to award for collecting enemy dogtags

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_dm_teammode_kill_score</code></summary>

Team deathmatch victory points to award for enemy kill

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_do_warmup_offine</code></summary>

Whether or not to do a warmup period at the start of a match in an offline (bot) match.

default: `"0"`  
flags: `0x82000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>mp_do_warmup_period</code></summary>

Whether or not to do a warmup period at the start of a match.

default: `"1"`  
flags: `0x82000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>mp_economy_reset_rounds</code></summary>

Reset all player money every N rounds (0 for never)

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_endmatch_votenextleveltime</code></summary>

If mp_endmatch_votenextmap is set, players have this much time to vote on the next map at match end.

default: `"20"`  
flags: `0x80000`  
</details>
<details>
<summary><code>mp_endmatch_votenextmap</code></summary>

Whether or not players vote for the next map at the end of the match when the final scoreboard comes up

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_endmatch_votenextmap_keepcurrent</code></summary>

If set, keeps the current map in the list of voting options.  If not set, the current map will not appear in the list of voting options.

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_endwarmup_player_count</code></summary>

Number of players required to be connected to end warmup early. 0 to require maximum players for mode.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_equipment_reset_rounds</code></summary>

Reset all player equipment every N rounds (0 for never)

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_facefronttime</code></summary>

After this amount of time of standing in place but aiming to one side, go ahead and move feet to face upper body.

default: `"2"`  
flags: `0x2002`  
</details>
<details>
<summary><code>mp_feetyawrate</code></summary>

How many degrees per second that we can turn our feet or upper body.

default: `"400"`  
flags: `0x2002`  
</details>
<details>
<summary><code>mp_force_assign_teams</code></summary>

Players don't get to choose what team they are on, it is auto assinged.

default: `"0"`  
flags: `0x82000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>mp_force_pick_time</code></summary>

The amount of time a player has on the team screen to make a selection before being auto-teamed

default: `"15"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_forcecamera</code></summary>

Restricts spectator modes for dead players. 0 = Any team. 1 = Only own team. 2 = No one; fade to black on death (previously mp_fadetoblack).

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_fraglimit</code></summary>

The number of kills at which the map ends

default: `"0"`  
flags: `0x2100`  
</details>
<details>
<summary><code>mp_free_armor</code></summary>

Determines whether kevlar (1+) and/or helmet (2+) are given automatically.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_friendlyfire</code></summary>

Allows team members to injure other members of their team

default: `"0"`  
flags: `0x82100`  
</details>
<details>
<summary><code>mp_ggprogressive_random_weapon_kills_needed</code></summary>

If mp_ggprogressive_use_random_weapons is set, this is the number of kills needed with each weapon

default: `"2"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_ggprogressive_round_restart_delay</code></summary>

Number of seconds to delay before restarting a round after a win in gungame progessive

default: `"15.0"`  
flags: `0x82000`  
min value: `0`  
max value: `90`  
</details>
<details>
<summary><code>mp_ggprogressive_use_random_weapons</code></summary>

If set, selects random weapons from set categories for the progression order

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_ggtr_always_upgrade</code></summary>

Award this many upgrade points every round in demolition mode

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_ggtr_bomb_defuse_bonus</code></summary>

Number of bonus upgrades to award the CTs when they defuse a gun game bomb

default: `"1"`  
flags: `0x82000`  
min value: `1`  
max value: `10`  
</details>
<details>
<summary><code>mp_ggtr_bomb_detonation_bonus</code></summary>

Number of bonus upgrades to award the Ts when they detonate a gun game bomb

default: `"1"`  
flags: `0x82000`  
min value: `1`  
max value: `10`  
</details>
<details>
<summary><code>mp_ggtr_bomb_pts_for_flash</code></summary>

Kill points required in a round to get a bonus flash grenade

default: `"4"`  
flags: `0x82000`  
min value: `1`  
max value: `5`  
</details>
<details>
<summary><code>mp_ggtr_bomb_pts_for_he</code></summary>

Kill points required in a round to get a bonus HE grenade

default: `"3"`  
flags: `0x82000`  
min value: `1`  
max value: `5`  
</details>
<details>
<summary><code>mp_ggtr_bomb_pts_for_molotov</code></summary>

Kill points required in a round to get a bonus molotov cocktail

default: `"5"`  
flags: `0x82000`  
min value: `1`  
max value: `5`  
</details>
<details>
<summary><code>mp_ggtr_bomb_pts_for_upgrade</code></summary>

Kill points required to upgrade a player's weapon

default: `"2.0"`  
flags: `0x82000`  
min value: `1`  
max value: `10`  
</details>
<details>
<summary><code>mp_ggtr_bomb_respawn_delay</code></summary>

Number of seconds to delay before making the bomb available to a respawner in gun game

default: `"0.0"`  
flags: `0x82000`  
min value: `0`  
max value: `30`  
</details>
<details>
<summary><code>mp_ggtr_end_round_kill_bonus</code></summary>

Number of bonus points awarded in Demolition Mode when knife kill ends round

default: `"1"`  
flags: `0x82000`  
min value: `0`  
max value: `10`  
</details>
<details>
<summary><code>mp_ggtr_halftime_delay</code></summary>

Number of seconds to delay during TR Mode halftime

default: `"0.0"`  
flags: `0x82000`  
min value: `0`  
max value: `30`  
</details>
<details>
<summary><code>mp_ggtr_last_weapon_kill_ends_half</code></summary>

End the half and give a team round point when a player makes a kill using the final weapon

default: `"0"`  
flags: `0x82000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>mp_ggtr_num_rounds_autoprogress</code></summary>

Upgrade the player's weapon after this number of rounds without upgrading

default: `"3"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_give_player_c4</code></summary>

Whether this map should spawn a c4 bomb for a player or not.

default: `"1"`  
flags: `0x82000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>mp_halftime</code></summary>

Determines whether the match switches sides in a halftime event.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_halftime_duration</code></summary>

Number of seconds that halftime lasts

default: `"15.0"`  
flags: `0x82000`  
min value: `0`  
max value: `300`  
</details>
<details>
<summary><code>mp_halftime_pausematch</code></summary>

Set to 1 to pause match after halftime countdown elapses. Match must be resumed by vote or admin.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_halftime_pausetimer</code></summary>

Set to 1 to stay in halftime indefinitely. Set to 0 to resume the timer.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_heavyassaultsuit_aimpunch</code></summary>

How much EXTRA aim punch will happen when a player wearing the assault suit gets when shot

default: `"1.0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_heavyassaultsuit_cooldown</code></summary>

Determines cooldown of purchase.

default: `"5"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_heavyassaultsuit_deploy_timescale</code></summary>

How fast a player wearing the heavy assault suit will draw their weapon (1.0 = normal speed, 0.5 = half speed)

default: `"0.8"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_heavyassaultsuit_speed</code></summary>

The max speed of a player when they are wearing the heavy assault suit

default: `"130"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_heavybot_damage_reduction_scale</code></summary>

How much damage should scale when the player wearing the heavy assault suit is shot (1.0 = no change, 0.5 = half damage)

default: `"1.0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_hostages_rescuetime</code></summary>

Additional time added to round time if a hostage is reached by a CT.

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_hostages_rescuetowin</code></summary>

0 == all alive, any other number is the number the CT's need to rescue to win the round.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>mp_hostages_takedamage</code></summary>

Whether or not hostages can be hurt.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_ik</code></summary>

Use IK on in-place turns.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>mp_join_grace_time</code></summary>

Number of seconds after round start to allow a player to join a game

default: `"0.0"`  
flags: `0x82000`  
min value: `0`  
max value: `30`  
</details>
<details>
<summary><code>mp_match_can_clinch</code></summary>

Can a team clinch and end the match by being so far ahead that the other team has no way to catching up?

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_match_end_changelevel</code></summary>

At the end of the match, perform a changelevel even if next map is the same

default: `"0"`  
flags: `0x82000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>mp_match_end_restart</code></summary>

At the end of the match, perform a restart instead of loading a new map

default: `"0"`  
flags: `0x82000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>mp_match_restart_delay</code></summary>

Time (in seconds) until a match restarts.

default: `"15"`  
flags: `0x82000`  
min value: `1`  
max value: `9999`  
</details>
<details>
<summary><code>mp_max_armor</code></summary>

Determines the highest level of armor allowed to be purchased. (0) None, (1) Kevlar, (2) Helmet

default: `"2"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_maxmoney</code></summary>

maximum amount of money allowed in a player's account

default: `"16000"`  
flags: `0x82000`  
min value: `0`  
</details>
<details>
<summary><code>mp_maxrounds</code></summary>

max number of rounds to play before server changes maps

default: `"0"`  
flags: `0x82100`  
min value: `0`  
</details>
<details>
<summary><code>mp_molotovusedelay</code></summary>

Number of seconds to delay before the molotov can be used after acquiring it

default: `"15.0"`  
flags: `0x82000`  
min value: `0`  
max value: `30`  
</details>
<details>
<summary><code>mp_overtime_enable</code></summary>

If a match ends in a tie, use overtime rules to determine winner

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_overtime_halftime_pausetimer</code></summary>

If set to 1 will set mp_halftime_pausetimer to 1 before every half of overtime. Set mp_halftime_pausetimer to 0 to resume the timer.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_overtime_maxrounds</code></summary>

When overtime is enabled play additional rounds to determine winner

default: `"6"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_overtime_startmoney</code></summary>

Money assigned to all players at start of every overtime half

default: `"10000"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_plant_c4_anywhere</code></summary>



default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_playercashawards</code></summary>

Players can earn money by performing in-game actions

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_playerid</code></summary>

Controls what information player see in the status bar: 0 all names; 1 team names; 2 no names

default: `"0"`  
flags: `0x82000`  
min value: `0`  
max value: `2`  
</details>
<details>
<summary><code>mp_playerid_delay</code></summary>

Number of seconds to delay showing information in the status bar

default: `"0.4"`  
flags: `0x82000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>mp_playerid_hold</code></summary>

Number of seconds to keep showing old information in the status bar

default: `"0.2"`  
flags: `0x82000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>mp_radar_showall</code></summary>

Determines who should see all. 0 = default. 1 = both teams. 2 = Terrorists. 3 = Counter-Terrorists.

default: `"0"`  
flags: `0x82000`  
min value: `0`  
max value: `3`  
</details>
<details>
<summary><code>mp_randomspawn</code></summary>

Determines whether players are to spawn. 0 = default; 1 = both teams; 2 = Terrorists; 3 = CTs.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_randomspawn_dist</code></summary>

If using mp_randomspawn, determines whether to test distance when selecting this spot.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_randomspawn_los</code></summary>

If using mp_randomspawn, determines whether to test Line of Sight when spawning.

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_respawn_immunitytime</code></summary>

How many seconds after respawn immunity lasts.

default: `"4.0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_respawn_on_death_ct</code></summary>

When set to 1, counter-terrorists will respawn after dying.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_respawn_on_death_t</code></summary>

When set to 1, terrorists will respawn after dying.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_respawnwavetime</code></summary>

Time between respawn waves.

default: `"10.0"`  
flags: `0x2100`  
</details>
<details>
<summary><code>mp_round_restart_delay</code></summary>

Number of seconds to delay before restarting a round after a win

default: `"7.0"`  
flags: `0x82000`  
min value: `0`  
max value: `14`  
</details>
<details>
<summary><code>mp_shield_speed_deployed</code></summary>

The max speed of a player when they have a shield deployed

default: `"170"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_shield_speed_holstered</code></summary>

The max speed of a player when they have a shield holstered

default: `"200"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_solid_teammates</code></summary>

How solid are teammates: 0 = transparent; 1 = fully solid; 2 = can stand on top of heads

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_spec_swapplayersides</code></summary>

Toggle set the player names and team names to the opposite side in which they are are on the spectator panel.

default: `"0"`  
flags: `0x82000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>mp_spectators_max</code></summary>

How many spectators are allowed in a match.

default: `"2"`  
flags: `0x82000`  
min value: `0`  
</details>
<details>
<summary><code>mp_startmoney</code></summary>

amount of money each player gets when they reset

default: `"800"`  
flags: `0x82000`  
min value: `0`  
</details>
<details>
<summary><code>mp_t_default_grenades</code></summary>

The default grenades that the Ts will spawn with.  To give multiple grenades, separate each weapon class with a space like this: 'weapon_molotov weapon_hegrenade'

default: `""`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_t_default_melee</code></summary>

The default melee weapon that the Ts will spawn with

default: `"weapon_knife"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_t_default_primary</code></summary>

The default primary (rifle) weapon that the Ts will spawn with

default: `""`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_t_default_secondary</code></summary>

The default secondary (pistol) weapon that the Ts will spawn with

default: `"weapon_glock"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_taser_recharge_time</code></summary>

Determines recharge time for taser. -1 = disabled.

default: `"-1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_team_timeout_max</code></summary>

Number of timeouts each team gets per match.

default: `"1"`  
flags: `0x82004`  
</details>
<details>
<summary><code>mp_team_timeout_time</code></summary>

Duration of each timeout.

default: `"60"`  
flags: `0x82004`  
</details>
<details>
<summary><code>mp_teamcashawards</code></summary>

Teams can earn money by performing in-game actions

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_teammates_are_enemies</code></summary>

When set, your teammates act as enemies and all players are valid targets.

default: `"0"`  
flags: `0x82100`  
</details>
<details>
<summary><code>mp_teams_unbalance_limit</code></summary>

Teams are unbalanced when one team has this many more players than the other team. (0 disables check)

default: `"1"`  
flags: `0x2100`  
min value: `0`  
max value: `30`  
</details>
<details>
<summary><code>mp_timelimit</code></summary>

game time per map in minutes

default: `"5"`  
flags: `0x82100`  
</details>
<details>
<summary><code>mp_tournament</code></summary>

default: `"0"`  
flags: `0x2100`  
</details>
<details>
<summary><code>mp_use_respawn_waves</code></summary>

When set to 1, and that player's team is set to respawn, they will respawn in waves. If set to 2, teams will respawn when the whole team is dead.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_verbose_changelevel_spew</code></summary>

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>mp_warmup_pausetimer</code></summary>

Set to 1 to stay in warmup indefinitely. Set to 0 to resume the timer.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_warmuptime_all_players_connected</code></summary>

Warmup time to use when all players have connected. 0 to disable.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_weapons_allow_heavy</code></summary>

Determines which team, if any, can purchase Heavy guns. -1 = any; 0 = non; 2 = Ts; 3 = CTs.

default: `"-1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_weapons_allow_map_placed</code></summary>

If this convar is set, when a match starts, the game will not delete weapons placed in the map.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_weapons_allow_pistols</code></summary>

Determines which team, if any, can purchase Pistols. -1 = any; 0 = non; 2 = Ts; 3 = CTs.

default: `"-1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_weapons_allow_rifles</code></summary>

Determines which team, if any, can purchase Rifles. -1 = any; 0 = non; 2 = Ts; 3 = CTs.

default: `"-1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_weapons_allow_smgs</code></summary>

Determines which team, if any, can purchase SMGs. -1 = any; 0 = non; 2 = Ts; 3 = CTs.

default: `"-1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_weapons_allow_typecount</code></summary>

Determines how many purchases of each weapon type allowed per player per round (0 to disallow purchasing, -1 to have no limit).

default: `"5"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_weapons_allow_zeus</code></summary>

Determines how many Zeus purchases a player can make per round (0 to disallow, -1 to have no limit).

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_weapons_glow_on_ground</code></summary>

If this convar is set, weapons on the ground will have a glow around them.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_weapons_max_gun_purchases_per_weapon_per_match</code></summary>

Max number of times a player may purchase any weapon per match

default: `"-1"`  
flags: `0x82004`  
min value: `-1`  
max value: `1`  
</details>
<details>
<summary><code>mp_win_panel_display_time</code></summary>

The amount of time to show the win panel between matches / halfs

default: `"3"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_winlimit</code></summary>

Max score one team can reach before server changes maps

default: `"0"`  
flags: `0x2100`  
min value: `0`  
</details>
<details>
<summary><code>muzzleflash_light</code></summary>

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>net_client_steamdatagram_enable_override</code></summary>

0: Use connect method requested by GC.  >0: Always use SDR if possible.  <0: Always use direct UDP if possible

default: `"0"`  
flags: `0x80008`  
</details>
<details>
<summary><code>net_graphholdsvframerate</code></summary>

Hold worst case in server framerate line.

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>net_graphipc</code></summary>

Show IPCs on netgraph panel

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>net_graphmsecs</code></summary>

The latency graph represents this many milliseconds.

default: `"400"`  
flags: `0x80`  
</details>
<details>
<summary><code>net_graphpos</code></summary>

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>net_graphshowinterp</code></summary>

Draw the interpolation graph.

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>net_graphshowlatency</code></summary>

Draw the ping/packet loss graph.

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>net_graphshowsvframerate</code></summary>

Draw the server framerate graph.

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>net_graphsolid</code></summary>

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>net_graphtext</code></summary>

Draw text fields

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>net_scale</code></summary>

Show the 'Paused' image when game is paused.

default: `"5"`  
flags: `0x80`  
</details>
<details>
<summary><code>nextlevel</code></summary>

If set to a valid map name, will trigger a changelevel to the specified map at the end of the round

default: `""`  
flags: `0x82100`  
</details>
<details>
<summary><code>nextmode</code></summary>

Sets the game mode to be played when the next level loads

default: `""`  
flags: `0x82100`  
</details>
<details>
<summary><code>old_radiusdamage</code></summary>

default: `"0.0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>option_duck_method</code></summary>

Player index of other player to check for position errors.

default: `"0"`  
flags: `0x8080`  
</details>
<details>
<summary><code>option_speed_method</code></summary>

default: `"0"`  
flags: `0x8080`  
</details>
<details>
<summary><code>panel_test_title_safe</code></summary>

Test vgui panel positioning with title safe indentation

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>particle_simulateoverflow</code></summary>

Used for stress-testing particle systems. Randomly denies creation of particles.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>phys_debug_check_contacts</code></summary>

Sets the max number of physics ticks allowed for client-side physics (ragdolls)

default: `"0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>phys_pushscale</code></summary>

1 - show hitches , 2 - show stalls

default: `"1"`  
flags: `0x2000`  
</details>
<details>
<summary><code>player_botdifflast_s</code></summary>

default: `"2"`  
flags: `0x80080`  
</details>
<details>
<summary><code>player_nevershow_communityservermessage</code></summary>

default: `"0"`  
flags: `0x1008080`  
</details>
<details>
<summary><code>player_teamplayedlast</code></summary>

default: `"3"`  
flags: `0x1008080`  
</details>
<details>
<summary><code>post_jump_crouch</code></summary>

This determines how long the third person player character will crouch for after landing a jump.  This only affects the third person animation visuals.

default: `"0.2f"`  
flags: `0x6000`  
</details>
<details>
<summary><code>props_break_max_pieces</code></summary>

Maximum prop breakable piece count (-1 = model default)

default: `"-1"`  
flags: `0x2000`  
</details>
<details>
<summary><code>props_break_max_pieces_perframe</code></summary>

Maximum prop breakable piece count per frame (-1 = model default)

default: `"-1"`  
flags: `0x2000`  
</details>
<details>
<summary><code>pwatchent</code></summary>

Entity to watch for prediction system changes.

default: `"-1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>pwatchvar</code></summary>

Entity variable to watch in prediction system for changes.

default: `""`  
flags: `0x4000`  
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
<summary><code>r_DrawModelLightOrigin</code></summary>

Prevent jiggle bones from pointing directly away from their target in case of numerical instability.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_DrawRain</code></summary>

Enable/disable rain rendering.

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_JeepViewBlendTo</code></summary>

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_JeepViewBlendToScale</code></summary>

default: `"0.03"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_JeepViewBlendToTime</code></summary>

default: `"1.5"`  
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
<summary><code>r_JeepViewZHeight</code></summary>

default: `"10.0"`  
flags: `0x6100`  
</details>
<details>
<summary><code>r_PortalTestEnts</code></summary>

Clip entities against portal frustums.

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_RainCheck</code></summary>

Enable/disable IsInAir() check for rain drops?

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_RainDebugDuration</code></summary>

Shows rain tracelines for this many seconds (0 disables)

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_RainHack</code></summary>

Allows rain in splitscreen

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_RainProfile</code></summary>

Enable/disable rain profiling.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_RainRadius</code></summary>

default: `"1500"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_RainSideVel</code></summary>

How much sideways velocity rain gets.

default: `"130"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_RainSimulate</code></summary>

Enable/disable rain simulation.

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_RainSplashPercentage</code></summary>

default: `"20"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_SnowDebugBox</code></summary>

Snow Debug Boxes.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_SnowEnable</code></summary>

Snow Enable

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_SnowEndAlpha</code></summary>

Snow.

default: `"255"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_SnowEndSize</code></summary>

Snow.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_SnowFallSpeed</code></summary>

Snow fall speed scale.

default: `"1.5"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_SnowInsideRadius</code></summary>

Snow.

default: `"256"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_SnowOutsideRadius</code></summary>

Snow.

default: `"1024"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_SnowParticles</code></summary>

Snow.

default: `"500"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_SnowPosScale</code></summary>

Snow.

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_SnowRayEnable</code></summary>

Snow.

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_SnowRayLength</code></summary>

Snow.

default: `"8192.0f"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_SnowRayRadius</code></summary>

Snow.

default: `"256"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_SnowSpeedScale</code></summary>

Snow.

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_SnowStartAlpha</code></summary>

Snow.

default: `"25"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_SnowStartSize</code></summary>

Snow.

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_SnowWindScale</code></summary>

Snow.

default: `"0.0035"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_SnowZoomOffset</code></summary>

Snow.

default: `"384.0f"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_SnowZoomRadius</code></summary>

Snow.

default: `"512.0f"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_VehicleViewClamp</code></summary>

Draw impact dust effects.

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_VehicleViewDampen</code></summary>

default: `"1"`  
flags: `0x6100`  
</details>
<details>
<summary><code>r_alphafade_usefov</code></summary>

Account for FOV when computing an entity's distance-based alpha fade

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_debugcheapwater</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_depthoverlay</code></summary>

Replaces opaque objects with their grayscaled depth values. r_showz_power scales the output.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_disable_distance_fade_on_big_props</code></summary>

Completely disable distance fading on large props

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_disable_distance_fade_on_big_props_thresh</code></summary>

Distance prop fade disable threshold size

default: `"48000"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_disable_update_shadow</code></summary>

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
<summary><code>r_drawbrushmodels</code></summary>

Render brush models. 0=Off, 1=Normal, 2=Wireframe

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawentities</code></summary>

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawentities</code></summary>

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawmodelnames</code></summary>

default: `"0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>r_drawmodelstatsoverlay</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawopaquedetailprops</code></summary>

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawopaquedetailprops_csm</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawopaquedetailprops_reflect</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawopaquedetailprops_refract</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawopaquerenderables</code></summary>

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawopaqueworld</code></summary>

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawothermodels</code></summary>

0=Off, 1=Normal, 2=Wireframe

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
<summary><code>r_drawplayers</code></summary>

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawrenderboxes</code></summary>

(0 - off) (1 - Draws the bounding box of entities) (2 - Draws the axis aligned bounding box used for culling) (3 - draws both bounding boxes)

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawropes</code></summary>

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawscreenoverlay</code></summary>

default: `"1"`  
flags: `0x10004000`  
</details>
<details>
<summary><code>r_drawshieldstencil</code></summary>

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawshieldstencil_debug</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawsprites</code></summary>

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawtracers</code></summary>

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawtracers_firstperson</code></summary>

Toggle visibility of first person weapon tracers

default: `"1"`  
flags: `0x80080`  
</details>
<details>
<summary><code>r_drawtracers_movetonotintersect</code></summary>



default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawtranslucentrenderables</code></summary>

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawtranslucentworld</code></summary>

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_drawunderwatercap</code></summary>

default: `"1"`  
flags: `0x10004000`  
</details>
<details>
<summary><code>r_drawunderwateroverlay</code></summary>

default: `"1"`  
flags: `0x10004000`  
</details>
<details>
<summary><code>r_drawviewmodel</code></summary>

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_eyegloss</code></summary>

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>r_eyegloss</code></summary>

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>r_eyemove</code></summary>

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>r_eyemove</code></summary>

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>r_eyeshift_x</code></summary>

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>r_eyeshift_x</code></summary>

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>r_eyeshift_y</code></summary>

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>r_eyeshift_y</code></summary>

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>r_eyeshift_z</code></summary>

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>r_eyeshift_z</code></summary>

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>r_eyesize</code></summary>

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>r_eyesize</code></summary>

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>r_eyewaterepsilon</code></summary>

default: `"7.0f"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_farz</code></summary>

Override the far clipping plane. -1 means to use the value in env_fog_controller.

default: `"-1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_flashlightambient</code></summary>

default: `"0.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_flashlightbacktraceoffset</code></summary>

default: `"0.4"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_flashlightconstant</code></summary>

default: `"0.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_flashlightfar</code></summary>

default: `"750.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_flashlightfov</code></summary>

default: `"53.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_flashlightladderdist</code></summary>

default: `"40.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_flashlightlinear</code></summary>

default: `"100.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_flashlightlockposition</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_flashlightmuzzleflashfov</code></summary>

default: `"120"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_flashlightnear</code></summary>

default: `"4.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_flashlightnearoffsetscale</code></summary>

default: `"1.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_flashlightoffsetforward</code></summary>

default: `"0.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_flashlightoffsetright</code></summary>

default: `"5.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_flashlightoffsetup</code></summary>

default: `"-5.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_flashlightquadratic</code></summary>

default: `"0.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_flashlightshadowatten</code></summary>

default: `"0.35"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_flashlightvisualizetrace</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_mapextents</code></summary>

Set the max dimension for the map.  This determines the far clipping plane

default: `"16384"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_modelwireframedecal</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_modelwireframedecal</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_nohw</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_nohw</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_nosw</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_nosw</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_particle_demo</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_portalsopenall</code></summary>

Open all portals

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_rainalpha</code></summary>

default: `"0.4"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_rainalphapow</code></summary>

default: `"0.8"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_raindensity</code></summary>

default: `"0.001"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_rainlength</code></summary>

default: `"0.1f"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_rainspeed</code></summary>

default: `"600.0f"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_rainwidth</code></summary>

default: `"0.5"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_replay_post_effect</code></summary>

Killer Replay - replay from victim's point of view (1); the default is killer's (0). Experimental.

default: `"-1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_shadow_debug_spew</code></summary>

Enable frustum culling of flashlights

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_shadowfromanyworldlight</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_shadowfromworldlights_debug</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_shadows_gamecontrol</code></summary>

Information about currently enabled flashlights

default: `"-1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_showenvcubemap</code></summary>

Trims the algorithm from processing darks: (0.0312 - visible limit, slower), (0.0625 - high quality, faster), (0.0833 - upper limit, the start of visible unfiltered edges). Special note: when using FXAA_GREEN_AS_LUMA, likely want to set this to zero

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_showenvcubemap</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_skin</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_skin</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_skybox</code></summary>

Enable the rendering of sky boxes

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_swingflashlight</code></summary>

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_underwateroverlay_drain_speed</code></summary>

0=Off, 1=Normal, 2=Wireframe

default: `"0.4"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_updaterefracttexture</code></summary>

Displays a list of the active screen shakes.

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_visocclusion</code></summary>

0-255, but 0 has errors at the moment

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_visualizetraces</code></summary>

Shows the incoming user messages for this client and dumps them out the type and size of the messages to the console. Setting this to 2 will display message contents as well

default: `"0"`  
flags: `0x4000`  
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
<summary><code>rope_subdiv</code></summary>

Rope subdivision amount

default: `"2"`  
flags: `0x800000`  
min value: `0`  
max value: `8`  
</details>
<details>
<summary><code>safezonex</code></summary>

The percentage of the screen width that is considered safe from overscan

default: `"1.0"`  
flags: `0x1000080`  
min value: `0.85`  
max value: `1`  
</details>
<details>
<summary><code>safezoney</code></summary>

The percentage of the screen height that is considered safe from overscan

default: `"1.0"`  
flags: `0x1000080`  
min value: `0.85`  
max value: `1`  
</details>
<details>
<summary><code>sc_enable</code></summary>

Enable SteamController

default: `"1.0"`  
flags: `0x8080`  
</details>
<details>
<summary><code>sc_pitch_sensitivity</code></summary>

SteamController pitch factor.

default: `"1.0"`  
flags: `0x8080`  
</details>
<details>
<summary><code>sc_yaw_sensitivity</code></summary>

SteamController yaw factor.

default: `"1.0"`  
flags: `0x8080`  
</details>
<details>
<summary><code>scene_clientflex</code></summary>

Do client side flex animation.

default: `"1"`  
flags: `0x2000`  
</details>
<details>
<summary><code>scene_print</code></summary>

When playing back a scene, print timing and event info to console.

default: `"0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>sensitivity</code></summary>

Mouse sensitivity.

default: `"2.5"`  
flags: `0x80`  
min value: `0.0001`  
max value: `1000`  
</details>
<details>
<summary><code>sk_autoaim_mode</code></summary>

default: `"1"`  
flags: `0x2080`  
</details>
<details>
<summary><code>skybox_disablereflection</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>smoothstairs</code></summary>

Smooth player eye z coordinate when traversing stairs.

default: `"1"`  
flags: `0x2000`  
</details>
<details>
<summary><code>snd_mainmenu_music_break_time_max</code></summary>

Minimum amount of time to pause between playing main menu music

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_mainmenu_music_break_time_min</code></summary>

Minimum amount of time to pause between playing main menu music

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>snd_music_boost</code></summary>

Specifies an amount to boost music volume by

default: `"0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>snd_music_selection</code></summary>

Tracking rotating music for players with no music packs equipped.

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>snd_mute_mvp_music_live_players</code></summary>

If set, MVP music is muted if players from both teams are still alive.

default: `"0"`  
flags: `0x80080`  
</details>
<details>
<summary><code>snd_prevent_ss_duplicates</code></summary>

switch to en/disable the prevention of splitscreen audio file duplicates


default: `"0"`  
flags: `0x6002`  
</details>
<details>
<summary><code>snd_sos_show_client_xmit</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>soundpatch_captionlength</code></summary>

How long looping soundpatch captions should display for.

default: `"2.0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>soundscape_fadetime</code></summary>

Time to crossfade sound effects between soundscapes

default: `"3.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>soundscape_radius_debug</code></summary>

Prints current volume of radius sounds

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>spec_autodirector</code></summary>

Auto-director chooses best view modes while spectating

default: `"1"`  
flags: `0x40000008`  
</details>
<details>
<summary><code>spec_autodirector_pausetime</code></summary>

Auto-director will pause for this long if a player is selected.

default: `"10"`  
flags: `0x40000008`  
</details>
<details>
<summary><code>spec_cameraman_disable_with_user_control</code></summary>

Disable cameraman UI control when user controls camera.

default: `"0"`  
flags: `0x40000008`  
</details>
<details>
<summary><code>spec_cameraman_ui</code></summary>

If a cameraman is active then use their UI commands (scoreboard, overview, etc.)

default: `"0"`  
flags: `0x40000008`  
</details>
<details>
<summary><code>spec_cameraman_xray</code></summary>

If a cameraman is active then use their Xray state.

default: `"0"`  
flags: `0x40000008`  
</details>
<details>
<summary><code>spec_dz_group_teams</code></summary>

If set, will group players into their teams for spectating, if 0, spectating numbers will be the default individual players

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>spec_freeze_cinematiclight_b</code></summary>

default: `"1.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>spec_freeze_cinematiclight_g</code></summary>

default: `"1.2"`  
flags: `0x4000`  
</details>
<details>
<summary><code>spec_freeze_cinematiclight_r</code></summary>

default: `"1.5"`  
flags: `0x4000`  
</details>
<details>
<summary><code>spec_freeze_cinematiclight_scale</code></summary>

default: `"2.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>spec_freeze_deathanim_time</code></summary>

The time that the death cam will spend watching the player's ragdoll before going into the freeze death cam.

default: `"0.8"`  
flags: `0x82000`  
</details>
<details>
<summary><code>spec_freeze_distance_max</code></summary>

Maximum random distance from the target to stop when framing them in observer freeze cam.

default: `"80"`  
flags: `0x4000`  
</details>
<details>
<summary><code>spec_freeze_distance_min</code></summary>

Minimum random distance from the target to stop when framing them in observer freeze cam.

default: `"60"`  
flags: `0x4000`  
</details>
<details>
<summary><code>spec_freeze_panel_extended_time</code></summary>

Time spent with the freeze panel still up after observer freeze cam is done.

default: `"0.0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>spec_freeze_target_fov</code></summary>

The target FOV that the deathcam should use.

default: `"42"`  
flags: `0x6000`  
</details>
<details>
<summary><code>spec_freeze_target_fov_long</code></summary>

The target FOV that the deathcam should use when the cam zoom far away on the target.

default: `"90"`  
flags: `0x6000`  
</details>
<details>
<summary><code>spec_freeze_time</code></summary>

Time spend frozen in observer freeze cam.

default: `"3.0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>spec_freeze_traveltime</code></summary>

Time taken to zoom in to frame a target in observer freeze cam.

default: `"0.3"`  
flags: `0x82000`  
min value: `0.01`  
</details>
<details>
<summary><code>spec_freeze_traveltime_long</code></summary>

Time taken to zoom in to frame a target in observer freeze cam when they are far away.

default: `"0.45"`  
flags: `0x6000`  
min value: `0.01`  
</details>
<details>
<summary><code>spec_glow_decay_time</code></summary>

Time to decay glow from 1.0 to spec_glow_silent_factor after spec_glow_full_time.

default: `"2.0"`  
flags: `0x80008`  
min value: `0`  
</details>
<details>
<summary><code>spec_glow_full_time</code></summary>

Noisy players stay at full brightness for this long.

default: `"1.0"`  
flags: `0x80008`  
min value: `0`  
</details>
<details>
<summary><code>spec_glow_silent_factor</code></summary>

Lurking player xray glow scaling.

default: `"0.6"`  
flags: `0x80008`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>spec_glow_spike_factor</code></summary>

Noisy player xray glow scaling (pop when noise is made).  Make >1 to add a 'spike' to noise-making players

default: `"1.2"`  
flags: `0x80008`  
min value: `1`  
max value: `3`  
</details>
<details>
<summary><code>spec_glow_spike_time</code></summary>

Time for noisy player glow 'spike' to show that they made noise very recently.

default: `"0.0"`  
flags: `0x80008`  
min value: `0`  
</details>
<details>
<summary><code>spec_hide_players</code></summary>

Toggle the visibility of scoreboard players.

default: `"0"`  
flags: `0x40080000`  
</details>
<details>
<summary><code>spec_lock_to_accountid</code></summary>

As an observer, lock the spectate target to the given accountid.

default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>spec_overwatch_skip_idle_ticks</code></summary>

Auto-director in overwatch mode will be skipping ticks when no subject observations are played.

default: `"10"`  
flags: `0x40000008`  
</details>
<details>
<summary><code>spec_replay_autostart</code></summary>

Auto-start Killer Replay when available

default: `"1"`  
flags: `0x88`  
</details>
<details>
<summary><code>spec_usenumberkeys_nobinds</code></summary>

If set to 1, map voting and spectator view use the raw number keys instead of the weapon binds (slot1, slot2, etc).

default: `"1"`  
flags: `0x88`  
</details>
<details>
<summary><code>ss_debug_draw_player</code></summary>

default: `"-1"`  
flags: `0x4002`  
</details>
<details>
<summary><code>ss_enable</code></summary>

Enables Split Screen support. Play Single Player now launches into split screen mode. NO ONLINE SUPPORT

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>ss_mimic</code></summary>

Split screen users mimic base player's CUserCmds

default: `"0"`  
flags: `0x4002`  
</details>
<details>
<summary><code>steam_controller_haptics</code></summary>

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>steamworks_sessionid_client</code></summary>

The client session ID for the new steamworks gamestats.

default: `"0"`  
flags: `0x210`  
</details>
<details>
<summary><code>steamworks_sessionid_server</code></summary>

The server session ID for the new steamworks gamestats.

default: `"0"`  
flags: `0x2010`  
</details>
<details>
<summary><code>store_version</code></summary>

Which version of the store to display.

default: `"1"`  
flags: `0x98`  
</details>
<details>
<summary><code>sv_accelerate</code></summary>

Linear acceleration amount (old value is 5.6)

default: `"5.5"`  
flags: `0x82100`  
</details>
<details>
<summary><code>sv_accelerate_debug_speed</code></summary>

default: `"0"`  
flags: `0x82100`  
</details>
<details>
<summary><code>sv_accelerate_use_weapon_speed</code></summary>

default: `"1"`  
flags: `0x82100`  
</details>
<details>
<summary><code>sv_air_max_horizontal_parachute_ratio</code></summary>

default: `"0.87"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_air_max_horizontal_parachute_speed</code></summary>

default: `"240"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_air_max_wishspeed</code></summary>

default: `"30"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_air_pushaway_dist</code></summary>

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_airaccelerate</code></summary>

default: `"12"`  
flags: `0x82100`  
</details>
<details>
<summary><code>sv_airaccelerate_parachute</code></summary>

default: `"2.6"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_airaccelerate_rappel</code></summary>

default: `"2.2"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_allow_thirdperson</code></summary>

Allows the server set players in third person mode without the client slamming it back (if cheats are on, all clients can set thirdperson without this convar being set)

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_alltalk</code></summary>

Deprecated. Replaced with sv_talk_enemy_dead and sv_talk_enemy_living.

default: `"0"`  
flags: `0x82100`  
</details>
<details>
<summary><code>sv_autobunnyhopping</code></summary>

Players automatically re-jump while holding jump button

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_backspeed</code></summary>

How much to slow down backwards motion

default: `"0.6"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_bot_difficulty_gamepad</code></summary>

Bot difficulty while playing with Gamepad device

default: `"0"`  
flags: `0x2010`  
</details>
<details>
<summary><code>sv_bot_difficulty_hydra</code></summary>

Bot difficulty while playing with Hydra device

default: `"0"`  
flags: `0x2010`  
</details>
<details>
<summary><code>sv_bot_difficulty_kbm</code></summary>

Bot difficulty while playing with Keyboard/Mouse device

default: `"0"`  
flags: `0x2010`  
</details>
<details>
<summary><code>sv_bot_difficulty_ps3move</code></summary>

Bot difficulty while playing with PS3Move device

default: `"0"`  
flags: `0x2010`  
</details>
<details>
<summary><code>sv_bot_difficulty_sharpshooter</code></summary>

Bot difficulty while playing with SharpShooter device

default: `"0"`  
flags: `0x2010`  
</details>
<details>
<summary><code>sv_bounce</code></summary>

Bounce multiplier for when physically simulated objects collide with other objects.

default: `"0"`  
flags: `0x82100`  
</details>
<details>
<summary><code>sv_breachcharge_arm_delay</code></summary>

default: `"0.3"`  
flags: `0x80004`  
</details>
<details>
<summary><code>sv_breachcharge_delay_max</code></summary>

default: `"0.8"`  
flags: `0x80004`  
</details>
<details>
<summary><code>sv_breachcharge_delay_min</code></summary>

default: `"0"`  
flags: `0x80004`  
</details>
<details>
<summary><code>sv_breachcharge_distance_max</code></summary>

default: `"1200"`  
flags: `0x80004`  
</details>
<details>
<summary><code>sv_breachcharge_distance_min</code></summary>

default: `"600"`  
flags: `0x80004`  
</details>
<details>
<summary><code>sv_breachcharge_fuse_max</code></summary>

default: `"1.0"`  
flags: `0x80004`  
</details>
<details>
<summary><code>sv_breachcharge_fuse_min</code></summary>

default: `"0.7"`  
flags: `0x80004`  
</details>
<details>
<summary><code>sv_bumpmine_arm_delay</code></summary>

default: `"0.3"`  
flags: `0x80004`  
</details>
<details>
<summary><code>sv_bumpmine_detonate_delay</code></summary>

default: `"0.25"`  
flags: `0x80004`  
</details>
<details>
<summary><code>sv_chat_proximity</code></summary>

default: `"-1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_clamp_unsafe_velocities</code></summary>

Whether the server will attempt to clamp velocities that could cause physics bugs or crashes.

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_clip_penetration_traces_to_players</code></summary>

default: `"1"`  
flags: `0x2000`  
</details>
<details>
<summary><code>sv_coach_comm_unrestricted</code></summary>

When set, ignores coach communication restrictions.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_coaching_enabled</code></summary>

Allows spectating and communicating with a team ( 'coach t' or 'coach ct' )

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_competitive_official_5v5</code></summary>

Enable to force the server to show 5v5 scoreboards and allows spectators to see characters through walls.

default: `"0"`  
flags: `0x82100`  
</details>
<details>
<summary><code>sv_compute_per_bot_difficulty</code></summary>

0 = compute all bot difficulties equally, 1 = compute unique bot difficulty for each bot 

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_cs_player_speed_has_hostage</code></summary>



default: `"200"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_deadtalk</code></summary>

Dead players can speak (voice, text) to the living

default: `"0"`  
flags: `0x82100`  
</details>
<details>
<summary><code>sv_debug_player_use</code></summary>

Visualizes +use logic. Green cross=trace success, Red cross=trace too far, Green box=radius success

default: `"0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>sv_disable_immunity_alpha</code></summary>

If set, clients won't slam the player model render settings each frame for immunity [mod authors use this]

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_disable_motd</code></summary>

Prevent the motd from showing.

default: `"1"`  
flags: `0x6002`  
</details>
<details>
<summary><code>sv_disable_observer_interpolation</code></summary>

Disallow interpolating between observer targets on this server.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_disable_radar</code></summary>

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_dz_enable_respawn</code></summary>

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_dz_hostage_rescue_reward</code></summary>

Number of cash bundles to award for rescuing a hostage

default: `"18"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_dz_squad_wipe_reward</code></summary>

Number of cash bundles to award for eliminating a squad

default: `"2"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_dz_team_count</code></summary>

Number of players per team

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_dz_zone_bombdrop_money_reward</code></summary>

How many money stacks players are rewarded each danger zone wave

default: `"15"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_dz_zone_bombdrop_money_reward_bonus</code></summary>

How many bonus money stacks players are rewarded each danger zone wave when they have the bonus item

default: `"5"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_dz_zone_hex_radius</code></summary>



default: `"2200"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_enablebunnyhopping</code></summary>

Allow player speed to exceed maximum running speed

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_endmatch_item_drop_interval</code></summary>

The time between drops on the end match scoreboard 

default: `"1.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_endmatch_item_drop_interval_ancient</code></summary>

The time between drops on the end match scoreboard for ancient items 

default: `"3.5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_endmatch_item_drop_interval_legendary</code></summary>

The time between drops on the end match scoreboard for legendary items 

default: `"2.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_endmatch_item_drop_interval_mythical</code></summary>

The time between drops on the end match scoreboard for mythical items 

default: `"1.25"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_endmatch_item_drop_interval_rare</code></summary>

The time between drops on the end match scoreboard for rare items 

default: `"1.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_exojump_jumpbonus_forward</code></summary>

ExoJump forwards velocity bonus when duck jumping

default: `"0.4"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_exojump_jumpbonus_up</code></summary>

ExoJump upwards bonus when holding the jump button

default: `"0.58"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_exojump_soundramp</code></summary>



default: `"20.0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>sv_exostaminajumpcost</code></summary>

Stamina penalty for jumping with exo legs

default: `".040"`  
flags: `0x82000`  
min value: `0`  
</details>
<details>
<summary><code>sv_exostaminalandcost</code></summary>

Stamina penalty for landing with exo legs

default: `".015"`  
flags: `0x82000`  
min value: `0`  
</details>
<details>
<summary><code>sv_extract_ammo_from_dropped_weapons</code></summary>

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_extreme_strafe_accuracy_fishtail</code></summary>

Number of degrees of aim 'fishtail' when making an extreme strafe direction change

default: `"0"`  
flags: `0x2002`  
min value: `-5`  
max value: `5`  
</details>
<details>
<summary><code>sv_falldamage_exojump_multiplier</code></summary>

ExoJump fall damage multiplier

default: `"0.4"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_falldamage_scale</code></summary>

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_falldamage_to_below_player_multiplier</code></summary>

Scale damage when distributed across two players

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_falldamage_to_below_player_ratio</code></summary>

Landing on a another player's head gives them this ratio of the damage.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_fistpoint_delay</code></summary>

default: `"1.8"`  
flags: `0x2000`  
</details>
<details>
<summary><code>sv_fistpunch_blocked_damage</code></summary>

default: `"25"`  
flags: `0x2000`  
</details>
<details>
<summary><code>sv_fistpunch_damage</code></summary>

default: `"10"`  
flags: `0x2000`  
</details>
<details>
<summary><code>sv_fistpunch_damage_hard</code></summary>

default: `"20"`  
flags: `0x2000`  
</details>
<details>
<summary><code>sv_fistpunch_damage_to_player_multiplier</code></summary>

default: `"1.5"`  
flags: `0x2000`  
</details>
<details>
<summary><code>sv_fistpunch_impact_sounds</code></summary>

default: `"1"`  
flags: `0x2000`  
</details>
<details>
<summary><code>sv_fistpunch_viewmove</code></summary>

default: `"40"`  
flags: `0x2000`  
</details>
<details>
<summary><code>sv_footstep_sound_frequency</code></summary>

How frequent to hear the player's step sound or how fast they appear to be running from first person.

default: `"0.97"`  
flags: `0x6000`  
</details>
<details>
<summary><code>sv_footsteps</code></summary>

Play footstep sound for players

default: `"1"`  
flags: `0x2102`  
</details>
<details>
<summary><code>sv_force_reflections</code></summary>

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_friction</code></summary>

World friction.

default: `"5.2"`  
flags: `0x82100`  
</details>
<details>
<summary><code>sv_full_alltalk</code></summary>

Any player (including Spectator team) can speak to any other player

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_grassburn</code></summary>

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_grenade_trajectory</code></summary>

Shows grenade trajectory visualization in-game.

default: `"0"`  
flags: `0x86000`  
</details>
<details>
<summary><code>sv_grenade_trajectory_dash</code></summary>

Dot-dash style grenade trajectory arc

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_grenade_trajectory_thickness</code></summary>

Visible thickness of grenade trajectory arc

default: `"0.2"`  
flags: `0x82000`  
min value: `0.1`  
max value: `1`  
</details>
<details>
<summary><code>sv_grenade_trajectory_time</code></summary>

Length of time grenade trajectory remains visible.

default: `"20"`  
flags: `0x82000`  
min value: `0.1`  
max value: `20`  
</details>
<details>
<summary><code>sv_grenade_trajectory_time_spectator</code></summary>

Length of time grenade trajectory remains visible as a spectator.

default: `"4"`  
flags: `0x82000`  
min value: `0`  
max value: `8`  
</details>
<details>
<summary><code>sv_highlight_distance</code></summary>



default: `"500"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_highlight_duration</code></summary>



default: `"3.5"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_holiday_mode</code></summary>

0 = OFF, 1 = Halloween, 2 = Winter

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_hudhint_sound</code></summary>

How often a sentence can repeat.

default: `"1"`  
flags: `0x2000`  
</details>
<details>
<summary><code>sv_infinite_ammo</code></summary>

Player's active weapon will never run out of ammo. If set to 2 then player has infinite total ammo but still has to reload the magazine.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_jump_impulse</code></summary>

Initial upward velocity for player jumps; sqrt(2*gravity*height).

default: `"301.993377"`  
flags: `0x82000`  
min value: `0`  
</details>
<details>
<summary><code>sv_jump_impulse_exojump_multiplier</code></summary>

ExoJump impulse multiplier

default: `"1.05"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_kick_ban_duration</code></summary>

How long should a kick ban from the server should last (in minutes)

default: `"15"`  
flags: `0x82100`  
</details>
<details>
<summary><code>sv_knife_attack_extend_from_player_aabb</code></summary>

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_ladder_angle</code></summary>

Cos of angle of incidence to ladder perpendicular for applying ladder_dampen

default: `"-0.707"`  
flags: `0x2000`  
min value: `-1`  
max value: `1`  
</details>
<details>
<summary><code>sv_ladder_dampen</code></summary>

Amount to dampen perpendicular movement on a ladder

default: `"0.2"`  
flags: `0x2000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>sv_ladder_scale_speed</code></summary>

Scale top speed on ladders

default: `"0.78"`  
flags: `0x82000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>sv_ledge_mantle_helper</code></summary>

1=Only improves success of jump+ducks to windows or vents (jump+duck to duck), 2=Improves success of all jump+ducks to ledges, 3=if you can get your eyes above it, you'll pull yourself up

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_ledge_mantle_helper_debug</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_ledge_mantle_helper_dzonly</code></summary>

1=only does the feature if running in game mode Danger Zone, 0=Doesn't check game mode to run

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_matchpause_auto_5v5</code></summary>

When enabled will automatically pause the match at next freeze time if less than 5 players are connected on each team.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_max_distance_transmit_footsteps</code></summary>

Maximum distance to transmit footstep sound effects.

default: `"1250.0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>sv_maxspeed</code></summary>

default: `"320"`  
flags: `0x82100`  
</details>
<details>
<summary><code>sv_maxvelocity</code></summary>

Maximum speed any ballistically moving object is allowed to attain per axis.

default: `"3500"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_min_jump_landing_sound</code></summary>

default: `"260.0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_mumble_positionalaudio</code></summary>

Allows players using Mumble to have support for positional audio.

default: `"1"`  
flags: `0x2000`  
</details>
<details>
<summary><code>sv_noclipaccelerate</code></summary>

default: `"5"`  
flags: `0x2180`  
</details>
<details>
<summary><code>sv_noclipduringpause</code></summary>

If cheats are enabled, then you can noclip with the game paused (for doing screenshots, etc.).

default: `"0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>sv_noclipspeed</code></summary>

default: `"5"`  
flags: `0x2180`  
</details>
<details>
<summary><code>sv_optimizedmovement</code></summary>

Perform the additional 'stuck' traces on the client side during prediction.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_outofammo_indicator</code></summary>

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_party_mode</code></summary>

Party!!

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_penetration_type</code></summary>

What type of penertration to use. 0 = old CS, 1 = new penetration

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_pushaway_clientside</code></summary>

Clientside physics push away (0=off, 1=only localplayer, 1=all players)

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_pushaway_clientside_size</code></summary>

Minimum size of pushback objects

default: `"15"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_pushaway_force</code></summary>

How hard physics objects are pushed away from the players on the server.

default: `"30000"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_pushaway_max_force</code></summary>

Maximum amount of force applied to physics objects by players.

default: `"1000"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_pushaway_max_player_force</code></summary>

Maximum of how hard the player is pushed away from physics objects.

default: `"10000"`  
flags: `0x6002`  
</details>
<details>
<summary><code>sv_pushaway_min_player_speed</code></summary>

If a player is moving slower than this, don't push away physics objects (enables ducking behind things).

default: `"75"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_pushaway_player_force</code></summary>

How hard the player is pushed away from physics objects (falls off with inverse square of distance).

default: `"200000"`  
flags: `0x6002`  
</details>
<details>
<summary><code>sv_reward_drop_delay</code></summary>

Delay between the end match scoreboard being shown and the beginning of item drops.

default: `"3.0"`  
flags: `0x2000`  
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
<summary><code>sv_server_graphic1</code></summary>

A 360x60 (<16kb) image file in /csgo/ that will be displayed to spectators.

default: `""`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_server_graphic2</code></summary>

A 220x45 (<16kb) image file in /csgo/ that will be displayed to spectators.

default: `""`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_server_verify_blood_on_player</code></summary>

default: `"1"`  
flags: `0x6000`  
</details>
<details>
<summary><code>sv_shield_explosive_damage_cap</code></summary>

default: `"99"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_shield_explosive_damage_crouch_bonus</code></summary>

default: `"10"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_shield_explosive_damage_mindist</code></summary>

default: `"250"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_shield_explosive_damage_mult</code></summary>

default: `"4"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_shield_explosive_damage_scale</code></summary>

default: `"0.5"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_shield_hitpoints</code></summary>

default: `"650"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_show_bot_difficulty_in_name</code></summary>

0 = hide bot difficulty in bot name, 1 = show bot difficulty in bot name

default: `"0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>sv_show_ragdoll_playernames</code></summary>

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_show_team_equipment_force_on</code></summary>

Force on if not prohibited

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_show_team_equipment_prohibit</code></summary>

Determines whether +cl_show_team_equipment is prohibited.

default: `"0"`  
flags: `0x82100`  
</details>
<details>
<summary><code>sv_showbullethits</code></summary>

1=show hits and near misses, 2=show hits only

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_showimpacts</code></summary>

Shows client (red) and server (blue) bullet impact point (1=both, 2=client-only, 3=server-only)

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_showimpacts_penetration</code></summary>

Shows extra data when bullets penetrate. (use sv_showimpacts_time to increase time shown)

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_showimpacts_time</code></summary>

Duration bullet impact indicators remain before disappearing

default: `"4"`  
flags: `0x82000`  
min value: `0`  
max value: `10`  
</details>
<details>
<summary><code>sv_showplayerhitboxes</code></summary>

Show lag compensated hitboxes for the specified player index whenever a player fires.

default: `"0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>sv_skirmish_id</code></summary>

Dedicated server skirmish id to run

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_skyname</code></summary>

Current name of the skybox texture

default: `"sky_urb01"`  
flags: `0x2080`  
</details>
<details>
<summary><code>sv_soundemitter_trace</code></summary>

Show all EmitSound calls including their symbolic name and the actual wave file they resolved to. (-1 = for nobody, 0 = for everybody, n = for one entity)


default: `"-1"`  
flags: `0x2000`  
</details>
<details>
<summary><code>sv_soundemitter_version</code></summary>

specfies what version of soundemitter system to use


default: `"2"`  
flags: `0x6002`  
</details>
<details>
<summary><code>sv_spec_hear</code></summary>

Determines who spectators can hear: 0: only spectators; 1: all players; 2: spectated team; 3: self only; 4: nobody

default: `"1"`  
flags: `0x82100`  
</details>
<details>
<summary><code>sv_spec_use_tournament_content_standards</code></summary>

default: `"0.0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_specaccelerate</code></summary>

default: `"5"`  
flags: `0x2180`  
</details>
<details>
<summary><code>sv_specnoclip</code></summary>

default: `"1"`  
flags: `0x2180`  
</details>
<details>
<summary><code>sv_specspeed</code></summary>

default: `"3"`  
flags: `0x2180`  
</details>
<details>
<summary><code>sv_staminajumpcost</code></summary>

Stamina penalty for jumping

default: `".080"`  
flags: `0x82000`  
min value: `0`  
</details>
<details>
<summary><code>sv_staminalandcost</code></summary>

Stamina penalty for landing

default: `".050"`  
flags: `0x82000`  
min value: `0`  
</details>
<details>
<summary><code>sv_staminamax</code></summary>

Maximum stamina penalty

default: `"80"`  
flags: `0x82000`  
min value: `0`  
max value: `100`  
</details>
<details>
<summary><code>sv_staminarecoveryrate</code></summary>

Rate at which stamina recovers (units/sec)

default: `"60"`  
flags: `0x82000`  
min value: `0`  
</details>
<details>
<summary><code>sv_standable_normal</code></summary>

default: `"0.7"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_stepsize</code></summary>

default: `"18"`  
flags: `0x2102`  
</details>
<details>
<summary><code>sv_stopspeed</code></summary>

Minimum stopping speed when on ground.

default: `"80"`  
flags: `0x82100`  
</details>
<details>
<summary><code>sv_suppress_viewpunch</code></summary>

default: `"0"`  
flags: `0x6002`  
</details>
<details>
<summary><code>sv_tablet_show_path_to_nearest_resq</code></summary>

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_talk_after_dying_time</code></summary>

The number of seconds a player can continue talking after dying as if they were still alive

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_talk_enemy_dead</code></summary>

Dead players can hear all dead enemy communication (voice, chat)

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_talk_enemy_living</code></summary>

Living players can hear all living enemy communication (voice, chat)

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_teamid_overhead</code></summary>

Shows teamID over player's heads.  0 = off, 1 = on

default: `"1"`  
flags: `0x82100`  
</details>
<details>
<summary><code>sv_teamid_overhead_always_prohibit</code></summary>

Determines whether cl_teamid_overhead_always is prohibited.

default: `"0"`  
flags: `0x82100`  
</details>
<details>
<summary><code>sv_teamid_overhead_maxdist</code></summary>

If >0, server will override cl_teamid_overhead_maxdist

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_teamid_overhead_maxdist_spec</code></summary>

If >0, server will override cl_teamid_overhead_maxdist_spec

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_timebetweenducks</code></summary>

Minimum time before recognizing consecutive duck key

default: `"0.4"`  
flags: `0x82000`  
min value: `0`  
max value: `2`  
</details>
<details>
<summary><code>sv_turbophysics</code></summary>

Turns on turbo physics

default: `"0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>sv_turning_inaccuracy_angle_min</code></summary>

default: `"4"`  
flags: `0x86000`  
</details>
<details>
<summary><code>sv_turning_inaccuracy_decay</code></summary>

default: `"0.8"`  
flags: `0x86000`  
</details>
<details>
<summary><code>sv_turning_inaccuracy_enabled</code></summary>

default: `"0"`  
flags: `0x86000`  
</details>
<details>
<summary><code>sv_walkable_normal</code></summary>

default: `"0.7"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_water_movespeed_multiplier</code></summary>

default: `"0.8"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_water_swim_mode</code></summary>

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_wateraccelerate</code></summary>

default: `"10"`  
flags: `0x82102`  
</details>
<details>
<summary><code>sv_waterdist</code></summary>

Vertical view fixup when eyes are near water plane.

default: `"12"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_waterfriction</code></summary>

default: `"1"`  
flags: `0x82102`  
</details>
<details>
<summary><code>sv_weapon_encumbrance_per_item</code></summary>

default: `"0.85"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_weapon_encumbrance_scale</code></summary>

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>tablet_c4_dist_max</code></summary>

default: `"3000"`  
flags: `0x82000`  
</details>
<details>
<summary><code>tablet_c4_dist_min</code></summary>

default: `"400"`  
flags: `0x82000`  
</details>
<details>
<summary><code>test_convar</code></summary>

Skips the prompt when saving a buy favorite in the buy menu

default: `"0"`  
flags: `0x40000080`  
</details>
<details>
<summary><code>test_convar</code></summary>

Skips the prompt when saving a buy favorite in the buy menu

default: `"500"`  
flags: `0x40000080`  
</details>
<details>
<summary><code>think_limit</code></summary>

Maximum think time in milliseconds, warning is printed if this is exceeded.

default: `"10"`  
flags: `0x82000`  
</details>
<details>
<summary><code>thirdperson_lockcamera</code></summary>

default: `"0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>tv_spectator_port_offset</code></summary>

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>ui_inventorysettings_recently_acknowledged</code></summary>

default: `""`  
flags: `0x80080`  
</details>
<details>
<summary><code>ui_lobby_draft_enabled</code></summary>

default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>ui_nearbylobbies_filter</code></summary>

Lobby voip stream audio volume

default: `"survival"`  
flags: `0x80080`  
</details>
<details>
<summary><code>ui_playsettings_maps_listen_casual</code></summary>

default: `"random_classic"`  
flags: `0x80080`  
</details>
<details>
<summary><code>ui_playsettings_maps_listen_competitive</code></summary>

default: `"random_classic"`  
flags: `0x80080`  
</details>
<details>
<summary><code>ui_playsettings_maps_listen_deathmatch</code></summary>

default: `"random_classic"`  
flags: `0x80080`  
</details>
<details>
<summary><code>ui_playsettings_maps_listen_scrimcomp2v2</code></summary>

default: `"mg_de_inferno"`  
flags: `0x80080`  
</details>
<details>
<summary><code>ui_playsettings_maps_listen_skirmish</code></summary>

default: `"mg_skirmish_flyingscoutsman"`  
flags: `0x80080`  
</details>
<details>
<summary><code>ui_playsettings_maps_official_casual</code></summary>

default: `"mg_casualsigma"`  
flags: `0x80080`  
</details>
<details>
<summary><code>ui_playsettings_maps_official_deathmatch</code></summary>

default: `"mg_casualsigma"`  
flags: `0x80080`  
</details>
<details>
<summary><code>ui_playsettings_maps_official_dzsirocco</code></summary>

default: `"mg_dz_sirocco"`  
flags: `0x80080`  
</details>
<details>
<summary><code>ui_playsettings_maps_workshop</code></summary>

default: `""`  
flags: `0x80080`  
</details>
<details>
<summary><code>ui_playsettings_mode_listen</code></summary>

default: `"casual"`  
flags: `0x80080`  
</details>
<details>
<summary><code>ui_playsettings_mode_official_dz</code></summary>

default: `"survival"`  
flags: `0x80080`  
</details>
<details>
<summary><code>ui_playsettings_survival_solo</code></summary>

default: `"0"`  
flags: `0x80080`  
</details>
<details>
<summary><code>ui_playsettings_warmup_map_name</code></summary>

default: `"de_mirage"`  
flags: `0x80080`  
</details>
<details>
<summary><code>ui_popup_weaponupdate_version</code></summary>

default: `"0"`  
flags: `0x80080`  
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
<summary><code>ui_vanitysetting_itemid</code></summary>

default: `""`  
flags: `0x80080`  
</details>
<details>
<summary><code>ui_vanitysetting_loadoutslot</code></summary>

default: `""`  
flags: `0x80080`  
</details>
<details>
<summary><code>ui_vanitysetting_team</code></summary>

default: `""`  
flags: `0x80080`  
</details>
<details>
<summary><code>vgui_message_dialog_modal</code></summary>

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>view_punch_decay</code></summary>

Decay factor exponent for view punch

default: `"18"`  
flags: `0x86000`  
</details>
<details>
<summary><code>view_recoil_tracking</code></summary>

How closely the view tracks with the aim punch from weapon recoil

default: `"0.45"`  
flags: `0x86000`  
</details>
<details>
<summary><code>viewmodel_fov</code></summary>

default: `"54"`  
flags: `0x80`  
</details>
<details>
<summary><code>viewmodel_offset_randomize</code></summary>

randomly change viewmodel offsets to visualize range

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>viewmodel_offset_x</code></summary>

default: `"0.0"`  
flags: `0x80`  
</details>
<details>
<summary><code>viewmodel_offset_y</code></summary>

default: `"0.0"`  
flags: `0x80`  
</details>
<details>
<summary><code>viewmodel_offset_z</code></summary>

default: `"0.0"`  
flags: `0x80`  
</details>
<details>
<summary><code>viewmodel_recoil</code></summary>

Amount of weapon recoil/aimpunch to display on viewmodel

default: `"1.0"`  
flags: `0x80`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>vm_debug</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>vm_draw_always</code></summary>

1 - Always draw view models, 2 - Never draw view models.  Should be done before map launches.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>voice_modenable</code></summary>

Enable/disable voice in this mod.

default: `"1"`  
flags: `0x40000080`  
</details>
<details>
<summary><code>weapon_accuracy_forcespread</code></summary>

Force spread to the specified value.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>weapon_accuracy_logging</code></summary>

Damage BELOW this value is considered light damage

default: `"0"`  
flags: `0x2082`  
</details>
<details>
<summary><code>weapon_accuracy_nospread</code></summary>

Disable weapon inaccuracy spread

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>weapon_accuracy_shotgun_spread_patterns</code></summary>

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>weapon_air_spread_scale</code></summary>

Scale factor for jumping inaccuracy, set to 0 to make jumping accuracy equal to standing

default: `"1.0"`  
flags: `0x82000`  
min value: `0`  
</details>
<details>
<summary><code>weapon_auto_cleanup_time</code></summary>

If set to non-zero, weapons will delete themselves after the specified time (in seconds) if no players are near.

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>weapon_debug_inaccuracy_only_up</code></summary>

Force weapon inaccuracy to be in exactly the up direction

default: `"0"`  
flags: `0x6002`  
</details>
<details>
<summary><code>weapon_debug_max_inaccuracy</code></summary>

Force all shots to have maximum inaccuracy

default: `"0"`  
flags: `0x6002`  
</details>
<details>
<summary><code>weapon_debug_spread_gap</code></summary>

default: `"0.67"`  
flags: `0xc008`  
</details>
<details>
<summary><code>weapon_debug_spread_show</code></summary>

Enables display of weapon accuracy; 1: show accuracy box, 3: show accuracy with dynamic crosshair

default: `"0"`  
flags: `0xc008`  
</details>
<details>
<summary><code>weapon_land_dip_amt</code></summary>

The amount the gun should dip when the player lands after a jump.

default: `"20.0"`  
flags: `0x6002`  
</details>
<details>
<summary><code>weapon_max_before_cleanup</code></summary>

If set to non-zero, will remove the oldest dropped weapon to maintain the specified number of dropped weapons in the world.

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>weapon_near_empty_sound</code></summary>

default: `"1"`  
flags: `0x6000`  
</details>
<details>
<summary><code>weapon_recoil_cooldown</code></summary>

DEPRECATED. Recoil now decays using weapon_recoil_decay_coefficient

default: `"0.55"`  
flags: `0x86000`  
</details>
<details>
<summary><code>weapon_recoil_decay1_exp</code></summary>

Decay factor exponent for weapon recoil

default: `"3.5"`  
flags: `0x86000`  
</details>
<details>
<summary><code>weapon_recoil_decay2_exp</code></summary>

Decay factor exponent for weapon recoil

default: `"8"`  
flags: `0x86000`  
</details>
<details>
<summary><code>weapon_recoil_decay2_lin</code></summary>

Decay factor (linear term) for weapon recoil

default: `"18"`  
flags: `0x86000`  
</details>
<details>
<summary><code>weapon_recoil_decay_coefficient</code></summary>



default: `"2.0"`  
flags: `0x86000`  
</details>
<details>
<summary><code>weapon_recoil_scale</code></summary>

Overall scale factor for recoil. Used to reduce recoil on specific platforms

default: `"2.0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>weapon_recoil_scale_motion_controller</code></summary>

Overall scale factor for recoil. Used to reduce recoil.  Only for motion controllers

default: `"1.0"`  
flags: `0x86000`  
</details>
<details>
<summary><code>weapon_recoil_suppression_factor</code></summary>

Initial recoil suppression factor (first suppressed shot will use this factor * standard recoil, lerping to 1 for later shots

default: `"0.75"`  
flags: `0x86000`  
</details>
<details>
<summary><code>weapon_recoil_suppression_shots</code></summary>

Number of shots before weapon uses full recoil

default: `"4"`  
flags: `0x86000`  
</details>
<details>
<summary><code>weapon_recoil_variance</code></summary>

Amount of variance per recoil impulse

default: `"0.55"`  
flags: `0x86000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>weapon_recoil_vel_decay</code></summary>

Decay factor for weapon recoil velocity

default: `"4.5"`  
flags: `0x86000`  
</details>
<details>
<summary><code>weapon_recoil_view_punch_extra</code></summary>

Additional (non-aim) punch added to view from recoil

default: `"0.055"`  
flags: `0x86000`  
</details>
<details>
<summary><code>weapon_reticle_knife_show</code></summary>

When enabled will show knife reticle on clients. Used for game modes requiring target id display when holding a knife.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>weapon_sound_falloff_multiplier</code></summary>

Scaling for falloff of weapon firing sounds

default: `"1.0"`  
flags: `0x86000`  
</details>
<details>
<summary><code>zoom_sensitivity_ratio_joystick</code></summary>

Additional controller sensitivity scale factor applied when FOV is zoomed in.

default: `"1.0"`  
flags: `0x8080`  
</details>
<details>
<summary><code>zoom_sensitivity_ratio_mouse</code></summary>

Additional mouse sensitivity scale factor applied when FOV is zoomed in.

default: `"1.0"`  
flags: `0x1008080`  
</details>

#### Addresses

```
client_panorama.dll!0x00d10ad0 ConVar BlendBonesMode
client_panorama.dll!0x00ce3060 ConVar achievement_debug
client_panorama.dll!0x00ce30b8 ConVar achievement_disable
client_panorama.dll!0x00ce3568 ConVar ai_debug_shoot_positions
client_panorama.dll!0x00ce3510 ConVar ai_shot_bias_max
client_panorama.dll!0x00ce34b8 ConVar ai_shot_bias_min
client_panorama.dll!0x00cffa78 ConVar ammo_338mag_headshot_mult
client_panorama.dll!0x00cff708 ConVar ammo_338mag_impulse
client_panorama.dll!0x00cff028 ConVar ammo_338mag_max
client_panorama.dll!0x00cffbd8 ConVar ammo_357sig_headshot_mult
client_panorama.dll!0x00cff868 ConVar ammo_357sig_impulse
client_panorama.dll!0x00cff188 ConVar ammo_357sig_max
client_panorama.dll!0x00cff290 ConVar ammo_357sig_min_max
client_panorama.dll!0x00cff1e0 ConVar ammo_357sig_p250_max
client_panorama.dll!0x00cff238 ConVar ammo_357sig_small_max
client_panorama.dll!0x00cffb80 ConVar ammo_45acp_headshot_mult
client_panorama.dll!0x00cff810 ConVar ammo_45acp_impulse
client_panorama.dll!0x00cff130 ConVar ammo_45acp_max
client_panorama.dll!0x00cff918 ConVar ammo_50AE_headshot_mult
client_panorama.dll!0x00cff5a8 ConVar ammo_50AE_impulse
client_panorama.dll!0x00cfee70 ConVar ammo_50AE_max
client_panorama.dll!0x00cffa20 ConVar ammo_556mm_box_headshot_mult
client_panorama.dll!0x00cff6b0 ConVar ammo_556mm_box_impulse
client_panorama.dll!0x00cfefd0 ConVar ammo_556mm_box_max
client_panorama.dll!0x00cff9c8 ConVar ammo_556mm_headshot_mult
client_panorama.dll!0x00cff658 ConVar ammo_556mm_impulse
client_panorama.dll!0x00cfef20 ConVar ammo_556mm_max
client_panorama.dll!0x00cfef78 ConVar ammo_556mm_small_max
client_panorama.dll!0x00cffc30 ConVar ammo_57mm_headshot_mult
client_panorama.dll!0x00cff8c0 ConVar ammo_57mm_impulse
client_panorama.dll!0x00cff2e8 ConVar ammo_57mm_max
client_panorama.dll!0x00cff970 ConVar ammo_762mm_headshot_mult
client_panorama.dll!0x00cff600 ConVar ammo_762mm_impulse
client_panorama.dll!0x00cfeec8 ConVar ammo_762mm_max
client_panorama.dll!0x00cffad0 ConVar ammo_9mm_headshot_mult
client_panorama.dll!0x00cff760 ConVar ammo_9mm_impulse
client_panorama.dll!0x00cff080 ConVar ammo_9mm_max
client_panorama.dll!0x00cffb28 ConVar ammo_buckshot_headshot_mult
client_panorama.dll!0x00cff7b8 ConVar ammo_buckshot_impulse
client_panorama.dll!0x00cff0d8 ConVar ammo_buckshot_max
client_panorama.dll!0x00cff4f8 ConVar ammo_grenade_limit_breachcharge
client_panorama.dll!0x00cff550 ConVar ammo_grenade_limit_bumpmine
client_panorama.dll!0x00cff340 ConVar ammo_grenade_limit_default
client_panorama.dll!0x00cff398 ConVar ammo_grenade_limit_flashbang
client_panorama.dll!0x00cff448 ConVar ammo_grenade_limit_snowballs
client_panorama.dll!0x00cff3f0 ConVar ammo_grenade_limit_total
client_panorama.dll!0x00cff4a0 ConVar ammo_item_limit_healthshot
client_panorama.dll!0x00d10b28 ConVar anim_3wayblend
client_panorama.dll!0x00d10b80 ConVar anim_twistbones_enabled
client_panorama.dll!0x00d01b78 ConVar bot_autodifficulty_threshold_high
client_panorama.dll!0x00d01b20 ConVar bot_autodifficulty_threshold_low
client_panorama.dll!0x00ceda38 ConVar c_maxdistance
client_panorama.dll!0x00ced8d8 ConVar c_maxpitch
client_panorama.dll!0x00ced988 ConVar c_maxyaw
client_panorama.dll!0x00ceda90 ConVar c_mindistance
client_panorama.dll!0x00ced930 ConVar c_minpitch
client_panorama.dll!0x00ced9e0 ConVar c_minyaw
client_panorama.dll!0x00cedb40 ConVar c_orthoheight
client_panorama.dll!0x00cedae8 ConVar c_orthowidth
client_panorama.dll!0x00cedb98 ConVar c_thirdpersonshoulder
client_panorama.dll!0x00cedcf8 ConVar c_thirdpersonshoulderaimdist
client_panorama.dll!0x00cedc48 ConVar c_thirdpersonshoulderdist
client_panorama.dll!0x00cedca0 ConVar c_thirdpersonshoulderheight
client_panorama.dll!0x00cedbf0 ConVar c_thirdpersonshoulderoffset
client_panorama.dll!0x00d09640 ConVar cachedvalue_count_partybrowser
client_panorama.dll!0x00d0a1d8 ConVar cachedvalue_count_teammates
client_panorama.dll!0x00ced828 ConVar cam_collision
client_panorama.dll!0x00ced720 ConVar cam_idealdelta
client_panorama.dll!0x00cee160 ConVar cam_idealdist
client_panorama.dll!0x00cee108 ConVar cam_idealdistright
client_panorama.dll!0x00cee0b0 ConVar cam_idealdistup
client_panorama.dll!0x00ced6c8 ConVar cam_ideallag
client_panorama.dll!0x00ced7d0 ConVar cam_idealpitch
client_panorama.dll!0x00ced778 ConVar cam_idealyaw
client_panorama.dll!0x00ced880 ConVar cam_showangles
client_panorama.dll!0x00ced670 ConVar cam_snapto
client_panorama.dll!0x00d032d8 ConVar cash_player_bomb_defused
client_panorama.dll!0x00d03280 ConVar cash_player_bomb_planted
client_panorama.dll!0x00d033e0 ConVar cash_player_damage_hostage
client_panorama.dll!0x00d034e8 ConVar cash_player_get_killed
client_panorama.dll!0x00d03388 ConVar cash_player_interact_with_hostage
client_panorama.dll!0x00d03228 ConVar cash_player_killed_enemy_default
client_panorama.dll!0x00d031d0 ConVar cash_player_killed_enemy_factor
client_panorama.dll!0x00d03438 ConVar cash_player_killed_hostage
client_panorama.dll!0x00d03178 ConVar cash_player_killed_teammate
client_panorama.dll!0x00d03330 ConVar cash_player_rescued_hostage
client_panorama.dll!0x00d03490 ConVar cash_player_respawn_amount
client_panorama.dll!0x00d02d00 ConVar cash_team_elimination_bomb_map
client_panorama.dll!0x00d02ca8 ConVar cash_team_elimination_hostage_map_ct
client_panorama.dll!0x00d02c50 ConVar cash_team_elimination_hostage_map_t
client_panorama.dll!0x00d03070 ConVar cash_team_hostage_alive
client_panorama.dll!0x00d03120 ConVar cash_team_hostage_interaction
client_panorama.dll!0x00d02f10 ConVar cash_team_loser_bonus
client_panorama.dll!0x00d02f68 ConVar cash_team_loser_bonus_consecutive_rounds
client_panorama.dll!0x00d030c8 ConVar cash_team_planted_bomb_but_defused
client_panorama.dll!0x00d03018 ConVar cash_team_rescued_hostage
client_panorama.dll!0x00d02d58 ConVar cash_team_survive_guardian_wave
client_panorama.dll!0x00d02bf8 ConVar cash_team_terrorist_win_bomb
client_panorama.dll!0x00d02e60 ConVar cash_team_win_by_defusing_bomb
client_panorama.dll!0x00d02eb8 ConVar cash_team_win_by_hostage_rescue
client_panorama.dll!0x00d02e08 ConVar cash_team_win_by_time_running_out_bomb
client_panorama.dll!0x00d02db0 ConVar cash_team_win_by_time_running_out_hostage
client_panorama.dll!0x00d02fc0 ConVar cash_team_winner_bonus_consecutive_rounds
client_panorama.dll!0x00cec868 ConVar cc_linger_time
client_panorama.dll!0x00cec8c0 ConVar cc_predisplay_time
client_panorama.dll!0x00cf4b40 ConVar cc_showmissing
client_panorama.dll!0x00cec970 ConVar cc_subtitles
client_panorama.dll!0x00d10bd8 ConVar choreo_spew_filter
client_panorama.dll!0x00cfdbf0 ConVar cl_autobuy
client_panorama.dll!0x00d02b48 ConVar cl_autohelp
client_panorama.dll!0x00d02a98 ConVar cl_autowepswitch
client_panorama.dll!0x00cefde8 ConVar cl_backspeed
client_panorama.dll!0x00d0b858 ConVar cl_bob_lower_amt
client_panorama.dll!0x00d0b6a0 ConVar cl_bob_version
client_panorama.dll!0x00d0b800 ConVar cl_bobamt_lat
client_panorama.dll!0x00d0b7a8 ConVar cl_bobamt_vert
client_panorama.dll!0x00d0b6f8 ConVar cl_bobcycle
client_panorama.dll!0x00d0b750 ConVar cl_bobup
client_panorama.dll!0x00cfa2a0 ConVar cl_brushfastpath
client_panorama.dll!0x00d0b648 ConVar cl_cam_driver_compensation_scale
client_panorama.dll!0x00cf6560 ConVar cl_camera_follow_bone_index
client_panorama.dll!0x00cfc270 ConVar cl_camera_height_restriction_debug
client_panorama.dll!0x00cec800 ConVar cl_chatfilter_version
client_panorama.dll!0x00cec7a8 ConVar cl_chatfilters
client_panorama.dll!0x00d09ca8 ConVar cl_clanid
client_panorama.dll!0x00d078c8 ConVar cl_compass_enabled
client_panorama.dll!0x00cfced0 ConVar cl_connection_trouble_show
client_panorama.dll!0x00cf58e0 ConVar cl_countbones
client_panorama.dll!0x00d0b3e0 ConVar cl_crosshair_drawoutline
client_panorama.dll!0x00d0b598 ConVar cl_crosshair_dynamic_maxdist_splitratio
client_panorama.dll!0x00d0b4e8 ConVar cl_crosshair_dynamic_splitalpha_innermod
client_panorama.dll!0x00d0b540 ConVar cl_crosshair_dynamic_splitalpha_outermod
client_panorama.dll!0x00d0b490 ConVar cl_crosshair_dynamic_splitdist
client_panorama.dll!0x00d0b438 ConVar cl_crosshair_outlinethickness
client_panorama.dll!0x00d0ae88 ConVar cl_crosshair_recoil
client_panorama.dll!0x00d07558 ConVar cl_crosshair_sniper_show_normal_inaccuracy
client_panorama.dll!0x00cfb928 ConVar cl_crosshair_sniper_width
client_panorama.dll!0x00d0b5f0 ConVar cl_crosshair_t
client_panorama.dll!0x00d0b120 ConVar cl_crosshairalpha
client_panorama.dll!0x00d0b018 ConVar cl_crosshaircolor
client_panorama.dll!0x00d0bd28 ConVar cl_crosshaircolor_b
client_panorama.dll!0x00d0bcd0 ConVar cl_crosshaircolor_g
client_panorama.dll!0x00d0bc78 ConVar cl_crosshaircolor_r
client_panorama.dll!0x00d0b330 ConVar cl_crosshairdot
client_panorama.dll!0x00d0b1d0 ConVar cl_crosshairgap
client_panorama.dll!0x00d0b228 ConVar cl_crosshairgap_useweaponvalue
client_panorama.dll!0x00d0b0c8 ConVar cl_crosshairscale
client_panorama.dll!0x00d0b280 ConVar cl_crosshairsize
client_panorama.dll!0x00d0bd80 ConVar cl_crosshairstyle
client_panorama.dll!0x00d0b2d8 ConVar cl_crosshairthickness
client_panorama.dll!0x00d0b178 ConVar cl_crosshairusealpha
client_panorama.dll!0x00cf73c0 ConVar cl_custommaterial_debug_graph
client_panorama.dll!0x00cfc320 ConVar cl_dangerzone_approaching_sound_radius
client_panorama.dll!0x00cfc3d0 ConVar cl_dangerzone_moving_sound_volume
client_panorama.dll!0x00cfc378 ConVar cl_dangerzone_sound_volume
client_panorama.dll!0x00ce7d30 ConVar cl_debugrumble
client_panorama.dll!0x00cea690 ConVar cl_detail_avoid_force
client_panorama.dll!0x00cea638 ConVar cl_detail_avoid_radius
client_panorama.dll!0x00cea6e8 ConVar cl_detail_avoid_recover_speed
client_panorama.dll!0x00cea5e0 ConVar cl_detail_max_sway
client_panorama.dll!0x00cea868 ConVar cl_detail_multiplier
client_panorama.dll!0x00ce3ec8 ConVar cl_disable_ragdolls
client_panorama.dll!0x00cfdb40 ConVar cl_disablefreezecam
client_panorama.dll!0x00d0d498 ConVar cl_disablehtmlmotd
client_panorama.dll!0x00cfbea8 ConVar cl_dm_buyrandomweapons
client_panorama.dll!0x00cfd568 ConVar cl_draw_only_deathnotices
client_panorama.dll!0x00ce8fb0 ConVar cl_drawhud
client_panorama.dll!0x00cfd388 ConVar cl_drawhud_force_deathnotices
client_panorama.dll!0x00cfd778 ConVar cl_drawhud_force_radar
client_panorama.dll!0x00cfd2d8 ConVar cl_drawhud_force_teamid_overhead
client_panorama.dll!0x00d08840 ConVar cl_drawhud_specvote
client_panorama.dll!0x00ce8ba0 ConVar cl_drawleaf
client_panorama.dll!0x00cf6de8 ConVar cl_drawmaterial
client_panorama.dll!0x00cf7440 ConVar cl_drawshadowtexture
client_panorama.dll!0x00d07e38 ConVar cl_dz_playagain_auto_spectate
client_panorama.dll!0x00d08db0 ConVar cl_embedded_stream_audio_volume_xmaster
client_panorama.dll!0x00ce44d8 ConVar cl_extrapolate
client_panorama.dll!0x00cf1740 ConVar cl_extrapolate_amount
client_panorama.dll!0x00cea8c0 ConVar cl_fastdetailsprites
client_panorama.dll!0x00cfd1d0 ConVar cl_fixedcrosshairgap
client_panorama.dll!0x00cfb5b8 ConVar cl_foot_contact_shadows
client_panorama.dll!0x00cf0c40 ConVar cl_forwardspeed
client_panorama.dll!0x00cfc218 ConVar cl_freezecameffects_showholiday
client_panorama.dll!0x00cfd438 ConVar cl_freezecampanel_position_dynamic
client_panorama.dll!0x00d05ef0 ConVar cl_http_log_enable
client_panorama.dll!0x00cfc110 ConVar cl_hud_background_alpha
client_panorama.dll!0x00cfc0b8 ConVar cl_hud_bomb_under_radar
client_panorama.dll!0x00cfc008 ConVar cl_hud_color
client_panorama.dll!0x00cfc168 ConVar cl_hud_healthammo_style
client_panorama.dll!0x00cfbf58 ConVar cl_hud_playercount_pos
client_panorama.dll!0x00cfbfb0 ConVar cl_hud_playercount_showcount
client_panorama.dll!0x00cf3d28 ConVar cl_idealpitchscale
client_panorama.dll!0x00ce47f0 ConVar cl_interpolate
client_panorama.dll!0x00d08fc8 ConVar cl_inventory_debug_tooltip
client_panorama.dll!0x00d08f18 ConVar cl_inventory_saved_filter2
client_panorama.dll!0x00d08f70 ConVar cl_inventory_saved_sort2
client_panorama.dll!0x00cf1848 ConVar cl_jiggle_bone_debug
client_panorama.dll!0x00cf1798 ConVar cl_jiggle_bone_debug_pitch_constraints
client_panorama.dll!0x00cf18f8 ConVar cl_jiggle_bone_debug_yaw_constraints
client_panorama.dll!0x00cf17f0 ConVar cl_jiggle_bone_invert
client_panorama.dll!0x00ce87a8 ConVar cl_join_advertise
client_panorama.dll!0x00cf3cd0 ConVar cl_lagcompensation
client_panorama.dll!0x00ce8d00 ConVar cl_leafsystemvis
client_panorama.dll!0x00cf6968 ConVar cl_leveloverview
client_panorama.dll!0x00d0d3f8 ConVar cl_leveloverviewmarker
client_panorama.dll!0x00cf6718 ConVar cl_lock_camera
client_panorama.dll!0x00d08ce0 ConVar cl_mainmenu_show_datagraph
client_panorama.dll!0x00cf8d58 ConVar cl_maxrenderable_dist
client_panorama.dll!0x00ce3fd0 ConVar cl_minimal_rtt_shadows
client_panorama.dll!0x00cf13d0 ConVar cl_mouselook
client_panorama.dll!0x00cfa8d0 ConVar cl_mute_all_but_friends_and_party
client_panorama.dll!0x00cfa878 ConVar cl_mute_enemy_team
client_panorama.dll!0x00cfcb38 ConVar cl_obs_interp_enable
client_panorama.dll!0x00cfd178 ConVar cl_observercrosshair
client_panorama.dll!0x00cf7100 ConVar cl_overdraw_test
client_panorama.dll!0x00cf2fd8 ConVar cl_particle_retire_cost
client_panorama.dll!0x00cf2d70 ConVar cl_particles_show_bbox
client_panorama.dll!0x00cf3160 ConVar cl_particles_show_controlpoints
client_panorama.dll!0x00cf3e30 ConVar cl_pclass
client_panorama.dll!0x00cf3dd8 ConVar cl_pdump
client_panorama.dll!0x00cf3628 ConVar cl_phys_show_active
client_panorama.dll!0x00cf3348 ConVar cl_phys_timescale
client_panorama.dll!0x00cf0e50 ConVar cl_pitchdown
client_panorama.dll!0x00cf0ea8 ConVar cl_pitchup
client_panorama.dll!0x00cfb560 ConVar cl_player_ping_mute
client_panorama.dll!0x00cfc958 ConVar cl_player_proximity_debug
client_panorama.dll!0x00d106d8 ConVar cl_playerspray_auto_apply
client_panorama.dll!0x00ce5658 ConVar cl_portal_use_new_dissolve
client_panorama.dll!0x00cf3d80 ConVar cl_predictionlist
client_panorama.dll!0x00cf3c78 ConVar cl_predictweapons
client_panorama.dll!0x00d07bc0 ConVar cl_quickinventory_lastinv
client_panorama.dll!0x00d07c18 ConVar cl_quickinventory_line_update_speed
client_panorama.dll!0x00cfd6c8 ConVar cl_radar_always_centered
client_panorama.dll!0x00cfd720 ConVar cl_radar_icon_scale_min
client_panorama.dll!0x00cfd618 ConVar cl_radar_rotate
client_panorama.dll!0x00cfd670 ConVar cl_radar_scale
client_panorama.dll!0x00cfd5c0 ConVar cl_radar_square_with_scoreboard
client_panorama.dll!0x00d07c80 ConVar cl_radial_radio_tab
client_panorama.dll!0x00d07cf0 ConVar cl_radialmenu_deadzone_size
client_panorama.dll!0x00cfc590 ConVar cl_ragdoll_workaround_threshold
client_panorama.dll!0x00d08a38 ConVar cl_rappel_tilt
client_panorama.dll!0x00cfdb98 ConVar cl_rebuy
client_panorama.dll!0x00cf53a0 ConVar cl_remove_old_ugc_downloads
client_panorama.dll!0x00ce5308 ConVar cl_righthand
client_panorama.dll!0x00ce7cd8 ConVar cl_rumblescale
client_panorama.dll!0x00ce72b8 ConVar cl_sanitize_player_names
client_panorama.dll!0x00d0b070 ConVar cl_scalecrosshair
client_panorama.dll!0x00cfda48 ConVar cl_scoreboard_mouse_enable_binding
client_panorama.dll!0x00d07d88 ConVar cl_scoreboard_survivors_always_on
client_panorama.dll!0x00d09388 ConVar cl_server_graphic1_enable
client_panorama.dll!0x00d09330 ConVar cl_server_graphic2_enable
client_panorama.dll!0x00cf7310 ConVar cl_shadowtextureoverlaysize
client_panorama.dll!0x00cfbe50 ConVar cl_show_clan_in_death_notice
client_panorama.dll!0x00ce3138 ConVar cl_showanimstate
client_panorama.dll!0x00ce31e8 ConVar cl_showanimstate_activities
client_panorama.dll!0x00ce3190 ConVar cl_showanimstate_log
client_panorama.dll!0x00cf3f90 ConVar cl_showerror
client_panorama.dll!0x00cf5720 ConVar cl_showfps
client_panorama.dll!0x00cec358 ConVar cl_showhelp
client_panorama.dll!0x00cf5778 ConVar cl_showpos
client_panorama.dll!0x00cf0f00 ConVar cl_sidespeed
client_panorama.dll!0x00d10a78 ConVar cl_simdbones
client_panorama.dll!0x00cf1ab0 ConVar cl_skipfastpath
client_panorama.dll!0x00cfa3a8 ConVar cl_skipslowpath
client_panorama.dll!0x00cfc1c0 ConVar cl_spec_follow_grenade_key
client_panorama.dll!0x00cfd510 ConVar cl_spec_mode
client_panorama.dll!0x00cfbae0 ConVar cl_spec_show_bindings
client_panorama.dll!0x00d092b0 ConVar cl_spec_stats
client_panorama.dll!0x00d053b8 ConVar cl_spec_use_tournament_content_standards
client_panorama.dll!0x00d0fc30 ConVar cl_sporeclipdistance
client_panorama.dll!0x00cec180 ConVar cl_sun_decay_rate
client_panorama.dll!0x00cec1d8 ConVar cl_sun_in_reflection_h_scale
client_panorama.dll!0x00cec230 ConVar cl_sun_in_reflection_v_scale
client_panorama.dll!0x00ce81f8 ConVar cl_sunlight_ortho_size
client_panorama.dll!0x00d0ca88 ConVar cl_tablet_mapmode
client_panorama.dll!0x00cfbd48 ConVar cl_teamid_overhead_maxdist
client_panorama.dll!0x00cfbda0 ConVar cl_teamid_overhead_maxdist_spec
client_panorama.dll!0x00cfbf00 ConVar cl_teammate_colors_show
client_panorama.dll!0x00ce42b0 ConVar cl_threaded_bone_setup
client_panorama.dll!0x00cefd90 ConVar cl_upspeed
client_panorama.dll!0x00d0bdd8 ConVar cl_use_new_headbob
client_panorama.dll!0x00d02af0 ConVar cl_use_opens_buy_menu
client_panorama.dll!0x00d10a20 ConVar cl_use_simd_bones
client_panorama.dll!0x00d0b8b0 ConVar cl_viewmodel_shift_left_amt
client_panorama.dll!0x00d0b908 ConVar cl_viewmodel_shift_right_amt
client_panorama.dll!0x00cfc680 ConVar cl_weapon_clip_thinwalls
client_panorama.dll!0x00cfc6d8 ConVar cl_weapon_clip_thinwalls_debug
client_panorama.dll!0x00cfc730 ConVar cl_weapon_clip_thinwalls_lock
client_panorama.dll!0x00d0bb18 ConVar cl_weapon_debug_print_accuracy
client_panorama.dll!0x00d0ba68 ConVar cl_weapon_debug_show_accuracy
client_panorama.dll!0x00d0bac0 ConVar cl_weapon_debug_show_accuracy_duration
client_panorama.dll!0x00d0ec00 ConVar cl_winddir
client_panorama.dll!0x00d0ec58 ConVar cl_windspeed
client_panorama.dll!0x00cf3ac0 ConVar cl_wpn_sway_scale
client_panorama.dll!0x00cecbe0 ConVar closecaption
client_panorama.dll!0x00d02ba0 ConVar closeonbuy
client_panorama.dll!0x00d10ed0 ConVar cloth_windage_multiplier
client_panorama.dll!0x00d06d00 ConVar commentary_firstrun
client_panorama.dll!0x00cfdca0 ConVar crosshair
client_panorama.dll!0x00d064f0 ConVar custom_bot_difficulty
client_panorama.dll!0x00d06310 ConVar debug_aim_angle
client_panorama.dll!0x00cfba30 ConVar debug_entity_outline_highlight
client_panorama.dll!0x00cfdc48 ConVar default_fov
client_panorama.dll!0x00cf5660 ConVar developer
client_panorama.dll!0x00ce4130 ConVar enable_skeleton_draw
client_panorama.dll!0x00d05130 ConVar ff_damage_bullet_penetration
client_panorama.dll!0x00d05080 ConVar ff_damage_reduction_bullets
client_panorama.dll!0x00d04fd0 ConVar ff_damage_reduction_grenade
client_panorama.dll!0x00d05028 ConVar ff_damage_reduction_grenade_self
client_panorama.dll!0x00d050d8 ConVar ff_damage_reduction_other
client_panorama.dll!0x00ce68a8 ConVar fish_debug
client_panorama.dll!0x00cf9540 ConVar fog_color
client_panorama.dll!0x00cf96f8 ConVar fog_colorskybox
client_panorama.dll!0x00cf9598 ConVar fog_enable
client_panorama.dll!0x00cf9750 ConVar fog_enableskybox
client_panorama.dll!0x00cf94e8 ConVar fog_end
client_panorama.dll!0x00cf9648 ConVar fog_endskybox
client_panorama.dll!0x00cf9800 ConVar fog_hdrcolorscale
client_panorama.dll!0x00cf9858 ConVar fog_hdrcolorscaleskybox
client_panorama.dll!0x00cf97a8 ConVar fog_maxdensity
client_panorama.dll!0x00cf96a0 ConVar fog_maxdensityskybox
client_panorama.dll!0x00cf9490 ConVar fog_start
client_panorama.dll!0x00cf95f0 ConVar fog_startskybox
client_panorama.dll!0x00cfcf28 ConVar fov_cs_debug
client_panorama.dll!0x00d104d8 ConVar func_break_max_pieces
client_panorama.dll!0x00cecd58 ConVar g15_update_msec
client_panorama.dll!0x00cebc10 ConVar g_Language
client_panorama.dll!0x00cf4440 ConVar g_debug_ragdoll_removal
client_panorama.dll!0x00ce41a8 ConVar g_debug_ragdoll_visualize
client_panorama.dll!0x00cf4498 ConVar g_ragdoll_important_maxcount
client_panorama.dll!0x00cf43e8 ConVar g_ragdoll_maxcount
client_panorama.dll!0x00d065a0 ConVar game_mode
client_panorama.dll!0x00d06548 ConVar game_online
client_panorama.dll!0x00d06498 ConVar game_public
client_panorama.dll!0x00d065f8 ConVar game_type
client_panorama.dll!0x00ce6d98 ConVar gameinstructor_find_errors
client_panorama.dll!0x00ce6b08 ConVar gameinstructor_save_restore_lessons
client_panorama.dll!0x00ce6ce8 ConVar gameinstructor_verbose
client_panorama.dll!0x00ce6d40 ConVar gameinstructor_verbose_lesson
client_panorama.dll!0x00cf6610 ConVar gl_clear_randomcolor
client_panorama.dll!0x00cec0c0 ConVar glow_muzzle_debug
client_panorama.dll!0x00cec010 ConVar glow_outline_effect_enable
client_panorama.dll!0x00cec068 ConVar glow_outline_width
client_panorama.dll!0x00d09258 ConVar gotv_theater_container
client_panorama.dll!0x00d0a4b0 ConVar healthshot_allow_use_at_full
client_panorama.dll!0x00d0a400 ConVar healthshot_health
client_panorama.dll!0x00d0a508 ConVar healthshot_healthboost_speed_multiplier
client_panorama.dll!0x00d0a458 ConVar healthshot_healthboost_time
client_panorama.dll!0x00cec578 ConVar hidehud
client_panorama.dll!0x00ce3408 ConVar hl2_episodic
client_panorama.dll!0x00d05518 ConVar hostage_feetyawrate
client_panorama.dll!0x00cfaaa0 ConVar hud_fastswitch
client_panorama.dll!0x00d084c0 ConVar hud_scaling
client_panorama.dll!0x00cfd280 ConVar hud_showtargetid
client_panorama.dll!0x00ce9008 ConVar hud_takesshots
client_panorama.dll!0x00cf0050 ConVar in_forceuser
client_panorama.dll!0x00d066f8 ConVar inferno_dlight_spacing
client_panorama.dll!0x00ceeb58 ConVar joy_accelmax
client_panorama.dll!0x00ceeaa8 ConVar joy_accelscale
client_panorama.dll!0x00ceeb00 ConVar joy_accelscalepoly
client_panorama.dll!0x00cee2c0 ConVar joy_advanced
client_panorama.dll!0x00cee420 ConVar joy_advaxisr
client_panorama.dll!0x00cee478 ConVar joy_advaxisu
client_panorama.dll!0x00cee4d0 ConVar joy_advaxisv
client_panorama.dll!0x00cee318 ConVar joy_advaxisx
client_panorama.dll!0x00cee370 ConVar joy_advaxisy
client_panorama.dll!0x00cee3c8 ConVar joy_advaxisz
client_panorama.dll!0x00ceebb0 ConVar joy_autoAimDampenMethod
client_panorama.dll!0x00ceec60 ConVar joy_autoaimdampen
client_panorama.dll!0x00ceec08 ConVar joy_autoaimdampenrange
client_panorama.dll!0x00cef398 ConVar joy_cfg_preset
client_panorama.dll!0x00cef080 ConVar joy_circle_correct
client_panorama.dll!0x00ceed10 ConVar joy_curvepoint_1
client_panorama.dll!0x00ceed68 ConVar joy_curvepoint_2
client_panorama.dll!0x00ceedc0 ConVar joy_curvepoint_3
client_panorama.dll!0x00ceee18 ConVar joy_curvepoint_4
client_panorama.dll!0x00ceee70 ConVar joy_curvepoint_end
client_panorama.dll!0x00cef0d8 ConVar joy_diagonalpov
client_panorama.dll!0x00cef130 ConVar joy_display_input
client_panorama.dll!0x00cee688 ConVar joy_forwardsensitivity
client_panorama.dll!0x00cee528 ConVar joy_forwardthreshold
client_panorama.dll!0x00ceea50 ConVar joy_gamma
client_panorama.dll!0x00cef1e0 ConVar joy_inverty
client_panorama.dll!0x00cee948 ConVar joy_lowend
client_panorama.dll!0x00cee9a0 ConVar joy_lowend_linear
client_panorama.dll!0x00cee9f8 ConVar joy_lowmap
client_panorama.dll!0x00cee268 ConVar joy_name
client_panorama.dll!0x00cef4a0 ConVar joy_no_accel_jump
client_panorama.dll!0x00cee738 ConVar joy_pitchsensitivity
client_panorama.dll!0x00cee5d8 ConVar joy_pitchthreshold
client_panorama.dll!0x00cee898 ConVar joy_response_look
client_panorama.dll!0x00cee8f0 ConVar joy_response_look_pitch
client_panorama.dll!0x00cee7e8 ConVar joy_response_move
client_panorama.dll!0x00ceef78 ConVar joy_sensitive_step0
client_panorama.dll!0x00ceefd0 ConVar joy_sensitive_step1
client_panorama.dll!0x00cef028 ConVar joy_sensitive_step2
client_panorama.dll!0x00cee6e0 ConVar joy_sidesensitivity
client_panorama.dll!0x00cee580 ConVar joy_sidethreshold
client_panorama.dll!0x00cef188 ConVar joy_wingmanwarrior_turnhack
client_panorama.dll!0x00cee790 ConVar joy_yawsensitivity
client_panorama.dll!0x00cee630 ConVar joy_yawthreshold
client_panorama.dll!0x00cee210 ConVar joystick_force_disabled
client_panorama.dll!0x00d09d00 ConVar key_bind_version
client_panorama.dll!0x00ced4f0 ConVar locator_split_len
client_panorama.dll!0x00ced498 ConVar locator_split_maxwide_percent
client_panorama.dll!0x00cfd228 ConVar lockMoveControllerRet
client_panorama.dll!0x00cf0df8 ConVar lookspring
client_panorama.dll!0x00cf0cf0 ConVar lookstrafe
client_panorama.dll!0x00cf1008 ConVar m_customaccel
client_panorama.dll!0x00cf1270 ConVar m_customaccel_exponent
client_panorama.dll!0x00cf1218 ConVar m_customaccel_max
client_panorama.dll!0x00cf12c8 ConVar m_customaccel_scale
client_panorama.dll!0x00cf1480 ConVar m_forward
client_panorama.dll!0x00cf11c0 ConVar m_mouseaccel1
client_panorama.dll!0x00cf1428 ConVar m_mouseaccel2
client_panorama.dll!0x00cf1530 ConVar m_mousespeed
client_panorama.dll!0x00cf1060 ConVar m_rawinput
client_panorama.dll!0x00cf1320 ConVar m_side
client_panorama.dll!0x00cf1168 ConVar m_yaw
client_panorama.dll!0x00cf26e8 ConVar mapcycledisabled
client_panorama.dll!0x00ce9218 ConVar mapoverview_allow_client_draw
client_panorama.dll!0x00ce9270 ConVar mapoverview_allow_grid_usage
client_panorama.dll!0x00ce92c8 ConVar mapoverview_icon_scale
client_panorama.dll!0x00cf7a18 ConVar mat_accelerate_adjust_exposure_down
client_panorama.dll!0x00cf7700 ConVar mat_autoexposure_max
client_panorama.dll!0x00cf7758 ConVar mat_autoexposure_max_multiplier
client_panorama.dll!0x00cf77b0 ConVar mat_autoexposure_min
client_panorama.dll!0x00cf7ac8 ConVar mat_bloom_scalefactor_scalar
client_panorama.dll!0x00cf75a0 ConVar mat_bloomamount_rate
client_panorama.dll!0x00cf74f0 ConVar mat_bloomscale
client_panorama.dll!0x00cf6ff8 ConVar mat_camerarendertargetoverlaysize
client_panorama.dll!0x00ce55a8 ConVar mat_colcorrection_forceentitiesclientside
client_panorama.dll!0x00cf79c0 ConVar mat_colorcorrection
client_panorama.dll!0x00cf7968 ConVar mat_debug_bloom
client_panorama.dll!0x00cf75f8 ConVar mat_debug_postprocessing_effects
client_panorama.dll!0x00cf7910 ConVar mat_disable_bloom
client_panorama.dll!0x00cfa4b0 ConVar mat_draw_zone_highlight
client_panorama.dll!0x00cfa458 ConVar mat_draw_zone_projection_mode
client_panorama.dll!0x00cf9ac0 ConVar mat_drawwater
client_panorama.dll!0x00cf7650 ConVar mat_dynamic_tonemapping
client_panorama.dll!0x00cf7b20 ConVar mat_exposure_center_region_x
client_panorama.dll!0x00cf7b78 ConVar mat_exposure_center_region_y
client_panorama.dll!0x00cf78b8 ConVar mat_force_bloom
client_panorama.dll!0x00cf7cd8 ConVar mat_force_tonemap_min_avglum
client_panorama.dll!0x00cf7c80 ConVar mat_force_tonemap_percent_bright_pixels
client_panorama.dll!0x00cf7c28 ConVar mat_force_tonemap_percent_target
client_panorama.dll!0x00cf7d30 ConVar mat_force_tonemap_scale
client_panorama.dll!0x00ce77c0 ConVar mat_fullbright
client_panorama.dll!0x00cf7d88 ConVar mat_fullbright
client_panorama.dll!0x00cea7f0 ConVar mat_fullbright
client_panorama.dll!0x00cf7860 ConVar mat_hdr_uncapexposure
client_panorama.dll!0x00cf7050 ConVar mat_hsv
client_panorama.dll!0x00cfa188 ConVar mat_lpreview_mode
client_panorama.dll!0x00cf7a70 ConVar mat_non_hdr_bloom_scalefactor
client_panorama.dll!0x00d114a8 ConVar mat_normals
client_panorama.dll!0x00d0e910 ConVar mat_normals
client_panorama.dll!0x00cf85a0 ConVar mat_postprocess_enable
client_panorama.dll!0x00ce54c0 ConVar mat_preview
client_panorama.dll!0x00cf7808 ConVar mat_show_histogram
client_panorama.dll!0x00cf6fa0 ConVar mat_showcamerarendertarget
client_panorama.dll!0x00cf6ef0 ConVar mat_showframebuffertexture
client_panorama.dll!0x00cf6e40 ConVar mat_showwatertextures
client_panorama.dll!0x00d0e5a0 ConVar mat_softwareskin
client_panorama.dll!0x00d11240 ConVar mat_softwareskin
client_panorama.dll!0x00ce8ae0 ConVar mat_stub
client_panorama.dll!0x00cf7bd0 ConVar mat_tonemap_algorithm
client_panorama.dll!0x00cf64b0 ConVar mat_viewportscale
client_panorama.dll!0x00cf6508 ConVar mat_viewportupscale
client_panorama.dll!0x00cf7498 ConVar mat_wireframe
client_panorama.dll!0x00cf70a8 ConVar mat_yuv
client_panorama.dll!0x00cef550 ConVar mc_accel_band_size
client_panorama.dll!0x00cef4f8 ConVar mc_dead_zone_radius
client_panorama.dll!0x00cef600 ConVar mc_max_pitchrate
client_panorama.dll!0x00cef5a8 ConVar mc_max_yawrate
client_panorama.dll!0x00d0a560 ConVar molotov_throw_detonate_time
client_panorama.dll!0x00d00050 ConVar mp_afterroundmoney
client_panorama.dll!0x00cebe00 ConVar mp_allowspectators
client_panorama.dll!0x00d00368 ConVar mp_anyone_can_pickup_c4
client_panorama.dll!0x00cf5010 ConVar mp_blockstyle
client_panorama.dll!0x00cf5278 ConVar mp_bonusroundtime
client_panorama.dll!0x00cffe40 ConVar mp_buy_allow_grenades
client_panorama.dll!0x00cffe98 ConVar mp_buy_allow_guns
client_panorama.dll!0x00cebf08 ConVar mp_buy_anywhere
client_panorama.dll!0x00cebf60 ConVar mp_buy_during_immunity
client_panorama.dll!0x00cffde8 ConVar mp_buytime
client_panorama.dll!0x00d003c0 ConVar mp_c4_cannot_be_defused
client_panorama.dll!0x00d04060 ConVar mp_c4timer
client_panorama.dll!0x00cf5328 ConVar mp_capdeteriorate_time
client_panorama.dll!0x00cf52d0 ConVar mp_capstyle
client_panorama.dll!0x00d01a18 ConVar mp_coop_force_join_ct
client_panorama.dll!0x00d01a70 ConVar mp_coopmission_mission_number
client_panorama.dll!0x00d00d60 ConVar mp_ct_default_grenades
client_panorama.dll!0x00d00c58 ConVar mp_ct_default_melee
client_panorama.dll!0x00d00d08 ConVar mp_ct_default_primary
client_panorama.dll!0x00d00cb0 ConVar mp_ct_default_secondary
client_panorama.dll!0x00d01968 ConVar mp_death_drop_breachcharge
client_panorama.dll!0x00d01808 ConVar mp_death_drop_c4
client_panorama.dll!0x00d018b8 ConVar mp_death_drop_defuser
client_panorama.dll!0x00d01860 ConVar mp_death_drop_grenade
client_panorama.dll!0x00d017b0 ConVar mp_death_drop_gun
client_panorama.dll!0x00d019c0 ConVar mp_death_drop_healthshot
client_panorama.dll!0x00d01910 ConVar mp_death_drop_taser
client_panorama.dll!0x00d01f98 ConVar mp_default_team_winner_no_objective
client_panorama.dll!0x00d01700 ConVar mp_defuser_allocation
client_panorama.dll!0x00cf5220 ConVar mp_disable_respawn_times
client_panorama.dll!0x00d015f8 ConVar mp_display_kill_assists
client_panorama.dll!0x00d01338 ConVar mp_dm_bonus_percent
client_panorama.dll!0x00d01390 ConVar mp_dm_bonus_respawn
client_panorama.dll!0x00d015a0 ConVar mp_dm_bonusweapon_dogtags
client_panorama.dll!0x00d013e8 ConVar mp_dm_dogtag_score
client_panorama.dll!0x00d012e0 ConVar mp_dm_kill_base_score
client_panorama.dll!0x00d01440 ConVar mp_dm_teammode
client_panorama.dll!0x00d014f0 ConVar mp_dm_teammode_bonus_score
client_panorama.dll!0x00d01548 ConVar mp_dm_teammode_dogtag_score
client_panorama.dll!0x00d01498 ConVar mp_dm_teammode_kill_score
client_panorama.dll!0x00cfff48 ConVar mp_do_warmup_offine
client_panorama.dll!0x00cffef0 ConVar mp_do_warmup_period
client_panorama.dll!0x00d01c28 ConVar mp_economy_reset_rounds
client_panorama.dll!0x00d02200 ConVar mp_endmatch_votenextleveltime
client_panorama.dll!0x00d02150 ConVar mp_endmatch_votenextmap
client_panorama.dll!0x00d021a8 ConVar mp_endmatch_votenextmap_keepcurrent
client_panorama.dll!0x00d006d8 ConVar mp_endwarmup_player_count
client_panorama.dll!0x00d01bd0 ConVar mp_equipment_reset_rounds
client_panorama.dll!0x00ce3298 ConVar mp_facefronttime
client_panorama.dll!0x00ce3240 ConVar mp_feetyawrate
client_panorama.dll!0x00cffce0 ConVar mp_force_assign_teams
client_panorama.dll!0x00d01ac8 ConVar mp_force_pick_time
client_panorama.dll!0x00cebfb8 ConVar mp_forcecamera
client_panorama.dll!0x00cf2798 ConVar mp_fraglimit
client_panorama.dll!0x00d03d48 ConVar mp_free_armor
client_panorama.dll!0x00cebe58 ConVar mp_friendlyfire
client_panorama.dll!0x00d00ba8 ConVar mp_ggprogressive_random_weapon_kills_needed
client_panorama.dll!0x00d00af8 ConVar mp_ggprogressive_round_restart_delay
client_panorama.dll!0x00d00b50 ConVar mp_ggprogressive_use_random_weapons
client_panorama.dll!0x00d009f0 ConVar mp_ggtr_always_upgrade
client_panorama.dll!0x00d01230 ConVar mp_ggtr_bomb_defuse_bonus
client_panorama.dll!0x00d01288 ConVar mp_ggtr_bomb_detonation_bonus
client_panorama.dll!0x00d01078 ConVar mp_ggtr_bomb_pts_for_flash
client_panorama.dll!0x00d01020 ConVar mp_ggtr_bomb_pts_for_he
client_panorama.dll!0x00d010d0 ConVar mp_ggtr_bomb_pts_for_molotov
client_panorama.dll!0x00d00fc8 ConVar mp_ggtr_bomb_pts_for_upgrade
client_panorama.dll!0x00d011d8 ConVar mp_ggtr_bomb_respawn_delay
client_panorama.dll!0x00d00a48 ConVar mp_ggtr_end_round_kill_bonus
client_panorama.dll!0x00d01180 ConVar mp_ggtr_halftime_delay
client_panorama.dll!0x00d00aa0 ConVar mp_ggtr_last_weapon_kill_ends_half
client_panorama.dll!0x00d00c00 ConVar mp_ggtr_num_rounds_autoprogress
client_panorama.dll!0x00d01758 ConVar mp_give_player_c4
client_panorama.dll!0x00d03df8 ConVar mp_halftime
client_panorama.dll!0x00d00940 ConVar mp_halftime_duration
client_panorama.dll!0x00d007e0 ConVar mp_halftime_pausematch
client_panorama.dll!0x00d00788 ConVar mp_halftime_pausetimer
client_panorama.dll!0x00cfed68 ConVar mp_heavyassaultsuit_aimpunch
client_panorama.dll!0x00d01e90 ConVar mp_heavyassaultsuit_cooldown
client_panorama.dll!0x00cfed10 ConVar mp_heavyassaultsuit_deploy_timescale
client_panorama.dll!0x00cfec60 ConVar mp_heavyassaultsuit_speed
client_panorama.dll!0x00cfecb8 ConVar mp_heavybot_damage_reduction_scale
client_panorama.dll!0x00d00310 ConVar mp_hostages_rescuetime
client_panorama.dll!0x00d002b8 ConVar mp_hostages_rescuetowin
client_panorama.dll!0x00d00260 ConVar mp_hostages_takedamage
client_panorama.dll!0x00ce32f0 ConVar mp_ik
client_panorama.dll!0x00d00f18 ConVar mp_join_grace_time
client_panorama.dll!0x00d00998 ConVar mp_match_can_clinch
client_panorama.dll!0x00d016a8 ConVar mp_match_end_changelevel
client_panorama.dll!0x00d01650 ConVar mp_match_end_restart
client_panorama.dll!0x00cf2690 ConVar mp_match_restart_delay
client_panorama.dll!0x00d03da0 ConVar mp_max_armor
client_panorama.dll!0x00cffff8 ConVar mp_maxmoney
client_panorama.dll!0x00cf5170 ConVar mp_maxrounds
client_panorama.dll!0x00d01128 ConVar mp_molotovusedelay
client_panorama.dll!0x00d00158 ConVar mp_overtime_enable
client_panorama.dll!0x00d00838 ConVar mp_overtime_halftime_pausetimer
client_panorama.dll!0x00d001b0 ConVar mp_overtime_maxrounds
client_panorama.dll!0x00d00208 ConVar mp_overtime_startmoney
client_panorama.dll!0x00d0aa38 ConVar mp_plant_c4_anywhere
client_panorama.dll!0x00d000a8 ConVar mp_playercashawards
client_panorama.dll!0x00d041c0 ConVar mp_playerid
client_panorama.dll!0x00d04218 ConVar mp_playerid_delay
client_panorama.dll!0x00d04270 ConVar mp_playerid_hold
client_panorama.dll!0x00cebda8 ConVar mp_radar_showall
client_panorama.dll!0x00d03e50 ConVar mp_randomspawn
client_panorama.dll!0x00d03f00 ConVar mp_randomspawn_dist
client_panorama.dll!0x00d03ea8 ConVar mp_randomspawn_los
client_panorama.dll!0x00d00890 ConVar mp_respawn_immunitytime
client_panorama.dll!0x00d020a0 ConVar mp_respawn_on_death_ct
client_panorama.dll!0x00d02048 ConVar mp_respawn_on_death_t
client_panorama.dll!0x00cf5068 ConVar mp_respawnwavetime
client_panorama.dll!0x00d008e8 ConVar mp_round_restart_delay
client_panorama.dll!0x00cfedc0 ConVar mp_shield_speed_deployed
client_panorama.dll!0x00cfee18 ConVar mp_shield_speed_holstered
client_panorama.dll!0x00d043d0 ConVar mp_solid_teammates
client_panorama.dll!0x00cffc88 ConVar mp_spec_swapplayersides
client_panorama.dll!0x00cffd90 ConVar mp_spectators_max
client_panorama.dll!0x00cfffa0 ConVar mp_startmoney
client_panorama.dll!0x00d00ec0 ConVar mp_t_default_grenades
client_panorama.dll!0x00d00db8 ConVar mp_t_default_melee
client_panorama.dll!0x00d00e68 ConVar mp_t_default_primary
client_panorama.dll!0x00d00e10 ConVar mp_t_default_secondary
client_panorama.dll!0x00d0cae0 ConVar mp_taser_recharge_time
client_panorama.dll!0x00cfea98 ConVar mp_team_timeout_max
client_panorama.dll!0x00cfea40 ConVar mp_team_timeout_time
client_panorama.dll!0x00d00100 ConVar mp_teamcashawards
client_panorama.dll!0x00cebeb0 ConVar mp_teammates_are_enemies
client_panorama.dll!0x00cf5118 ConVar mp_teams_unbalance_limit
client_panorama.dll!0x00cf2740 ConVar mp_timelimit
client_panorama.dll!0x00cf50c0 ConVar mp_tournament
client_panorama.dll!0x00d04168 ConVar mp_use_respawn_waves
client_panorama.dll!0x00d005d0 ConVar mp_verbose_changelevel_spew
client_panorama.dll!0x00d00730 ConVar mp_warmup_pausetimer
client_panorama.dll!0x00d00680 ConVar mp_warmuptime_all_players_connected
client_panorama.dll!0x00d01d88 ConVar mp_weapons_allow_heavy
client_panorama.dll!0x00d01f40 ConVar mp_weapons_allow_map_placed
client_panorama.dll!0x00d01cd8 ConVar mp_weapons_allow_pistols
client_panorama.dll!0x00d01de0 ConVar mp_weapons_allow_rifles
client_panorama.dll!0x00d01d30 ConVar mp_weapons_allow_smgs
client_panorama.dll!0x00d01ee8 ConVar mp_weapons_allow_typecount
client_panorama.dll!0x00d01c80 ConVar mp_weapons_allow_zeus
client_panorama.dll!0x00d01ff0 ConVar mp_weapons_glow_on_ground
client_panorama.dll!0x00cfeaf0 ConVar mp_weapons_max_gun_purchases_per_weapon_per_match
client_panorama.dll!0x00d00f70 ConVar mp_win_panel_display_time
client_panorama.dll!0x00cf51c8 ConVar mp_winlimit
client_panorama.dll!0x00d108c8 ConVar muzzleflash_light
client_panorama.dll!0x00d04a58 ConVar net_client_steamdatagram_enable_override
client_panorama.dll!0x00cf6018 ConVar net_graphholdsvframerate
client_panorama.dll!0x00cf60c8 ConVar net_graphipc
client_panorama.dll!0x00cf5eb8 ConVar net_graphmsecs
client_panorama.dll!0x00cf5db0 ConVar net_graphpos
client_panorama.dll!0x00cf5f68 ConVar net_graphshowinterp
client_panorama.dll!0x00cf5f10 ConVar net_graphshowlatency
client_panorama.dll!0x00cf5fc0 ConVar net_graphshowsvframerate
client_panorama.dll!0x00cf5e08 ConVar net_graphsolid
client_panorama.dll!0x00cf5e60 ConVar net_graphtext
client_panorama.dll!0x00cf5d58 ConVar net_scale
client_panorama.dll!0x00cf27f0 ConVar nextlevel
client_panorama.dll!0x00cf2848 ConVar nextmode
client_panorama.dll!0x00cebd48 ConVar old_radiusdamage
client_panorama.dll!0x00cefef0 ConVar option_duck_method
client_panorama.dll!0x00ceff48 ConVar option_speed_method
client_panorama.dll!0x00d143a0 ConVar panel_test_title_safe
client_panorama.dll!0x00cf2d18 ConVar particle_simulateoverflow
client_panorama.dll!0x00cf3450 ConVar phys_debug_check_contacts
client_panorama.dll!0x00cf4f98 ConVar phys_pushscale
client_panorama.dll!0x00d09bf8 ConVar player_botdifflast_s
client_panorama.dll!0x00d06f58 ConVar player_nevershow_communityservermessage
client_panorama.dll!0x00cfd880 ConVar player_teamplayedlast
client_panorama.dll!0x00d054c0 ConVar post_jump_crouch
client_panorama.dll!0x00cf41b0 ConVar props_break_max_pieces
client_panorama.dll!0x00cf4208 ConVar props_break_max_pieces_perframe
client_panorama.dll!0x00cf4040 ConVar pwatchent
client_panorama.dll!0x00cf4098 ConVar pwatchvar
client_panorama.dll!0x00cf1e40 ConVar r_AirboatViewDampenDamp
client_panorama.dll!0x00cf1de8 ConVar r_AirboatViewDampenFreq
client_panorama.dll!0x00cf1e98 ConVar r_AirboatViewZHeight
client_panorama.dll!0x00cf6ae8 ConVar r_DrawBeams
client_panorama.dll!0x00cf1a00 ConVar r_DrawModelLightOrigin
client_panorama.dll!0x00d0ef70 ConVar r_DrawRain
client_panorama.dll!0x00ce83f8 ConVar r_JeepViewBlendTo
client_panorama.dll!0x00ce8450 ConVar r_JeepViewBlendToScale
client_panorama.dll!0x00ce84a8 ConVar r_JeepViewBlendToTime
client_panorama.dll!0x00cf2368 ConVar r_JeepViewDampenDamp
client_panorama.dll!0x00cf23c0 ConVar r_JeepViewDampenFreq
client_panorama.dll!0x00cf2418 ConVar r_JeepViewZHeight
client_panorama.dll!0x00ce8bf8 ConVar r_PortalTestEnts
client_panorama.dll!0x00d0eec0 ConVar r_RainCheck
client_panorama.dll!0x00d0f020 ConVar r_RainDebugDuration
client_panorama.dll!0x00d0edb8 ConVar r_RainHack
client_panorama.dll!0x00d0efc8 ConVar r_RainProfile
client_panorama.dll!0x00d0ee10 ConVar r_RainRadius
client_panorama.dll!0x00d0ee68 ConVar r_RainSideVel
client_panorama.dll!0x00d0ef18 ConVar r_RainSimulate
client_panorama.dll!0x00d0ecb0 ConVar r_RainSplashPercentage
client_panorama.dll!0x00d0f380 ConVar r_SnowDebugBox
client_panorama.dll!0x00d0f0c0 ConVar r_SnowEnable
client_panorama.dll!0x00d0f4e0 ConVar r_SnowEndAlpha
client_panorama.dll!0x00d0f590 ConVar r_SnowEndSize
client_panorama.dll!0x00d0f2d0 ConVar r_SnowFallSpeed
client_panorama.dll!0x00d0f170 ConVar r_SnowInsideRadius
client_panorama.dll!0x00d0f1c8 ConVar r_SnowOutsideRadius
client_panorama.dll!0x00d0f118 ConVar r_SnowParticles
client_panorama.dll!0x00d0f278 ConVar r_SnowPosScale
client_panorama.dll!0x00d0f698 ConVar r_SnowRayEnable
client_panorama.dll!0x00d0f5e8 ConVar r_SnowRayLength
client_panorama.dll!0x00d0f640 ConVar r_SnowRayRadius
client_panorama.dll!0x00d0f220 ConVar r_SnowSpeedScale
client_panorama.dll!0x00d0f488 ConVar r_SnowStartAlpha
client_panorama.dll!0x00d0f538 ConVar r_SnowStartSize
client_panorama.dll!0x00d0f328 ConVar r_SnowWindScale
client_panorama.dll!0x00d0f3d8 ConVar r_SnowZoomOffset
client_panorama.dll!0x00d0f430 ConVar r_SnowZoomRadius
client_panorama.dll!0x00d0fb70 ConVar r_VehicleViewClamp
client_panorama.dll!0x00cf1d90 ConVar r_VehicleViewDampen
client_panorama.dll!0x00ce8d58 ConVar r_alphafade_usefov
client_panorama.dll!0x00cf98b0 ConVar r_debugcheapwater
client_panorama.dll!0x00cf6c60 ConVar r_depthoverlay
client_panorama.dll!0x00ce8df8 ConVar r_disable_distance_fade_on_big_props
client_panorama.dll!0x00ce8e50 ConVar r_disable_distance_fade_on_big_props_thresh
client_panorama.dll!0x00ce9488 ConVar r_disable_update_shadow
client_panorama.dll!0x00ce8ea8 ConVar r_drawallrenderables
client_panorama.dll!0x00cfa2f8 ConVar r_drawbrushmodels
client_panorama.dll!0x00d110e0 ConVar r_drawentities
client_panorama.dll!0x00d0e700 ConVar r_drawentities
client_panorama.dll!0x00ce4360 ConVar r_drawmodelnames
client_panorama.dll!0x00ce43b8 ConVar r_drawmodelstatsoverlay
client_panorama.dll!0x00cf92d8 ConVar r_drawopaquedetailprops
client_panorama.dll!0x00cf9330 ConVar r_drawopaquedetailprops_csm
client_panorama.dll!0x00cf9388 ConVar r_drawopaquedetailprops_reflect
client_panorama.dll!0x00cf93e0 ConVar r_drawopaquedetailprops_refract
client_panorama.dll!0x00cf9070 ConVar r_drawopaquerenderables
client_panorama.dll!0x00cf8e60 ConVar r_drawopaqueworld
client_panorama.dll!0x00ce4308 ConVar r_drawothermodels
client_panorama.dll!0x00cf2cc0 ConVar r_drawparticles
client_panorama.dll!0x00cfa400 ConVar r_drawplayers
client_panorama.dll!0x00ce4638 ConVar r_drawrenderboxes
client_panorama.dll!0x00ce7818 ConVar r_drawropes
client_panorama.dll!0x00cf9228 ConVar r_drawscreenoverlay
client_panorama.dll!0x00cfc880 ConVar r_drawshieldstencil
client_panorama.dll!0x00cfc8d8 ConVar r_drawshieldstencil_debug
client_panorama.dll!0x00ce8168 ConVar r_drawsprites
client_panorama.dll!0x00ceb470 ConVar r_drawtracers
client_panorama.dll!0x00ceb4c8 ConVar r_drawtracers_firstperson
client_panorama.dll!0x00ceb520 ConVar r_drawtracers_movetonotintersect
client_panorama.dll!0x00cf9018 ConVar r_drawtranslucentrenderables
client_panorama.dll!0x00cf8eb8 ConVar r_drawtranslucentworld
client_panorama.dll!0x00cf9280 ConVar r_drawunderwatercap
client_panorama.dll!0x00cf91d0 ConVar r_drawunderwateroverlay
client_panorama.dll!0x00cfa668 ConVar r_drawviewmodel
client_panorama.dll!0x00d0e390 ConVar r_eyegloss
client_panorama.dll!0x00d11558 ConVar r_eyegloss
client_panorama.dll!0x00d0e3e8 ConVar r_eyemove
client_panorama.dll!0x00d11138 ConVar r_eyemove
client_panorama.dll!0x00d111e8 ConVar r_eyeshift_x
client_panorama.dll!0x00d0e440 ConVar r_eyeshift_x
client_panorama.dll!0x00d0e498 ConVar r_eyeshift_y
client_panorama.dll!0x00d11030 ConVar r_eyeshift_y
client_panorama.dll!0x00d0e4f0 ConVar r_eyeshift_z
client_panorama.dll!0x00d11348 ConVar r_eyeshift_z
client_panorama.dll!0x00d0e548 ConVar r_eyesize
client_panorama.dll!0x00d11088 ConVar r_eyesize
client_panorama.dll!0x00cf9bc8 ConVar r_eyewaterepsilon
client_panorama.dll!0x00cf6668 ConVar r_farz
client_panorama.dll!0x00ceae30 ConVar r_flashlightambient
client_panorama.dll!0x00ceb040 ConVar r_flashlightbacktraceoffset
client_panorama.dll!0x00ceacd0 ConVar r_flashlightconstant
client_panorama.dll!0x00ceac78 ConVar r_flashlightfar
client_panorama.dll!0x00ceaac0 ConVar r_flashlightfov
client_panorama.dll!0x00ceaee0 ConVar r_flashlightladderdist
client_panorama.dll!0x00cead28 ConVar r_flashlightlinear
client_panorama.dll!0x00ceaa68 ConVar r_flashlightlockposition
client_panorama.dll!0x00ceb098 ConVar r_flashlightmuzzleflashfov
client_panorama.dll!0x00ceac20 ConVar r_flashlightnear
client_panorama.dll!0x00ceaf90 ConVar r_flashlightnearoffsetscale
client_panorama.dll!0x00ceabc8 ConVar r_flashlightoffsetforward
client_panorama.dll!0x00ceab18 ConVar r_flashlightoffsetright
client_panorama.dll!0x00ceab70 ConVar r_flashlightoffsetup
client_panorama.dll!0x00cead80 ConVar r_flashlightquadratic
client_panorama.dll!0x00ceae88 ConVar r_flashlightshadowatten
client_panorama.dll!0x00ceadd8 ConVar r_flashlightvisualizetrace
client_panorama.dll!0x00cf69c0 ConVar r_mapextents
client_panorama.dll!0x00d113a0 ConVar r_modelwireframedecal
client_panorama.dll!0x00d0e8b8 ConVar r_modelwireframedecal
client_panorama.dll!0x00d10fd8 ConVar r_nohw
client_panorama.dll!0x00d0e5f8 ConVar r_nohw
client_panorama.dll!0x00d11450 ConVar r_nosw
client_panorama.dll!0x00d0e650 ConVar r_nosw
client_panorama.dll!0x00cfa238 ConVar r_particle_demo
client_panorama.dll!0x00ce8c50 ConVar r_portalsopenall
client_panorama.dll!0x00d0f850 ConVar r_rainalpha
client_panorama.dll!0x00d0f8a8 ConVar r_rainalphapow
client_panorama.dll!0x00d0f6f0 ConVar r_raindensity
client_panorama.dll!0x00d0f7a0 ConVar r_rainlength
client_panorama.dll!0x00d0f7f8 ConVar r_rainspeed
client_panorama.dll!0x00d0f748 ConVar r_rainwidth
client_panorama.dll!0x00d07500 ConVar r_replay_post_effect
client_panorama.dll!0x00ce9900 ConVar r_shadow_debug_spew
client_panorama.dll!0x00ce96f0 ConVar r_shadowfromanyworldlight
client_panorama.dll!0x00ce9698 ConVar r_shadowfromworldlights_debug
client_panorama.dll!0x00ce9b10 ConVar r_shadows_gamecontrol
client_panorama.dll!0x00d0e338 ConVar r_showenvcubemap
client_panorama.dll!0x00d10f80 ConVar r_showenvcubemap
client_panorama.dll!0x00d0e808 ConVar r_skin
client_panorama.dll!0x00d11608 ConVar r_skin
client_panorama.dll!0x00cf8fc0 ConVar r_skybox
client_panorama.dll!0x00ceaa10 ConVar r_swingflashlight
client_panorama.dll!0x00cf9178 ConVar r_underwateroverlay_drain_speed
client_panorama.dll!0x00cf6cb8 ConVar r_updaterefracttexture
client_panorama.dll!0x00cf8d00 ConVar r_visocclusion
client_panorama.dll!0x00cf5608 ConVar r_visualizetraces
client_panorama.dll!0x00ce4480 ConVar report_cliententitysim
client_panorama.dll!0x00ce8b38 ConVar report_clientthinklist
client_panorama.dll!0x00ce7500 ConVar rope_subdiv
client_panorama.dll!0x00d08410 ConVar safezonex
client_panorama.dll!0x00d08468 ConVar safezoney
client_panorama.dll!0x00cf1638 ConVar sc_enable
client_panorama.dll!0x00cf15e0 ConVar sc_pitch_sensitivity
client_panorama.dll!0x00cf1690 ConVar sc_yaw_sensitivity
client_panorama.dll!0x00cf4968 ConVar scene_clientflex
client_panorama.dll!0x00cf4508 ConVar scene_print
client_panorama.dll!0x00cf1378 ConVar sensitivity
client_panorama.dll!0x00cebc68 ConVar sk_autoaim_mode
client_panorama.dll!0x00cfa5b8 ConVar skybox_disablereflection
client_panorama.dll!0x00ce39c0 ConVar smoothstairs
client_panorama.dll!0x00d06918 ConVar snd_mainmenu_music_break_time_max
client_panorama.dll!0x00d068b8 ConVar snd_mainmenu_music_break_time_min
client_panorama.dll!0x00d02258 ConVar snd_music_boost
client_panorama.dll!0x00d022b0 ConVar snd_music_selection
client_panorama.dll!0x00cfdaa0 ConVar snd_mute_mvp_music_live_players
client_panorama.dll!0x00cf4a38 ConVar snd_prevent_ss_duplicates
client_panorama.dll!0x00cf4a90 ConVar snd_sos_show_client_xmit
client_panorama.dll!0x00cf4c50 ConVar soundpatch_captionlength
client_panorama.dll!0x00ce7e98 ConVar soundscape_fadetime
client_panorama.dll!0x00ce7f48 ConVar soundscape_radius_debug
client_panorama.dll!0x00cec3d8 ConVar spec_autodirector
client_panorama.dll!0x00cec430 ConVar spec_autodirector_pausetime
client_panorama.dll!0x00ce91c0 ConVar spec_cameraman_disable_with_user_control
client_panorama.dll!0x00ce9110 ConVar spec_cameraman_ui
client_panorama.dll!0x00ce9168 ConVar spec_cameraman_xray
client_panorama.dll!0x00ce9060 ConVar spec_dz_group_teams
client_panorama.dll!0x00cfb6c0 ConVar spec_freeze_cinematiclight_b
client_panorama.dll!0x00cfb668 ConVar spec_freeze_cinematiclight_g
client_panorama.dll!0x00cfb610 ConVar spec_freeze_cinematiclight_r
client_panorama.dll!0x00cfb718 ConVar spec_freeze_cinematiclight_scale
client_panorama.dll!0x00ce4fc8 ConVar spec_freeze_deathanim_time
client_panorama.dll!0x00ce4f18 ConVar spec_freeze_distance_max
client_panorama.dll!0x00ce4ec0 ConVar spec_freeze_distance_min
client_panorama.dll!0x00ce4f70 ConVar spec_freeze_panel_extended_time
client_panorama.dll!0x00ce5078 ConVar spec_freeze_target_fov
client_panorama.dll!0x00ce5020 ConVar spec_freeze_target_fov_long
client_panorama.dll!0x00ce4db8 ConVar spec_freeze_time
client_panorama.dll!0x00ce4e10 ConVar spec_freeze_traveltime
client_panorama.dll!0x00ce4e68 ConVar spec_freeze_traveltime_long
client_panorama.dll!0x00cfb878 ConVar spec_glow_decay_time
client_panorama.dll!0x00cfb820 ConVar spec_glow_full_time
client_panorama.dll!0x00cfb770 ConVar spec_glow_silent_factor
client_panorama.dll!0x00cfb7c8 ConVar spec_glow_spike_factor
client_panorama.dll!0x00cfb8d0 ConVar spec_glow_spike_time
client_panorama.dll!0x00cfcfd8 ConVar spec_hide_players
client_panorama.dll!0x00ce5128 ConVar spec_lock_to_accountid
client_panorama.dll!0x00cec4e0 ConVar spec_overwatch_skip_idle_ticks
client_panorama.dll!0x00d073f8 ConVar spec_replay_autostart
client_panorama.dll!0x00ce90b8 ConVar spec_usenumberkeys_nobinds
client_panorama.dll!0x00cf6798 ConVar ss_debug_draw_player
client_panorama.dll!0x00cf5ba8 ConVar ss_enable
client_panorama.dll!0x00cf00a8 ConVar ss_mimic
client_panorama.dll!0x00d06158 ConVar steam_controller_haptics
client_panorama.dll!0x00d06650 ConVar steamworks_sessionid_client
client_panorama.dll!0x00cf4d88 ConVar steamworks_sessionid_server
client_panorama.dll!0x00d0d180 ConVar store_version
client_panorama.dll!0x00cf1f48 ConVar sv_accelerate
client_panorama.dll!0x00cf1b80 ConVar sv_accelerate_debug_speed
client_panorama.dll!0x00cf1b28 ConVar sv_accelerate_use_weapon_speed
client_panorama.dll!0x00ceb858 ConVar sv_air_max_horizontal_parachute_ratio
client_panorama.dll!0x00ceb800 ConVar sv_air_max_horizontal_parachute_speed
client_panorama.dll!0x00ceb7a8 ConVar sv_air_max_wishspeed
client_panorama.dll!0x00ceb8b0 ConVar sv_air_pushaway_dist
client_panorama.dll!0x00cf20a8 ConVar sv_airaccelerate
client_panorama.dll!0x00cf2050 ConVar sv_airaccelerate_parachute
client_panorama.dll!0x00cf1ff8 ConVar sv_airaccelerate_rappel
client_panorama.dll!0x00d00470 ConVar sv_allow_thirdperson
client_panorama.dll!0x00d028e0 ConVar sv_alltalk
client_panorama.dll!0x00cfe440 ConVar sv_autobunnyhopping
client_panorama.dll!0x00cf1ce0 ConVar sv_backspeed
client_panorama.dll!0x00d02570 ConVar sv_bot_difficulty_gamepad
client_panorama.dll!0x00d02620 ConVar sv_bot_difficulty_hydra
client_panorama.dll!0x00d02518 ConVar sv_bot_difficulty_kbm
client_panorama.dll!0x00d025c8 ConVar sv_bot_difficulty_ps3move
client_panorama.dll!0x00d02678 ConVar sv_bot_difficulty_sharpshooter
client_panorama.dll!0x00cf2260 ConVar sv_bounce
client_panorama.dll!0x00d0a8a0 ConVar sv_breachcharge_arm_delay
client_panorama.dll!0x00d0a798 ConVar sv_breachcharge_delay_max
client_panorama.dll!0x00d0a740 ConVar sv_breachcharge_delay_min
client_panorama.dll!0x00d0a6e8 ConVar sv_breachcharge_distance_max
client_panorama.dll!0x00d0a690 ConVar sv_breachcharge_distance_min
client_panorama.dll!0x00d0a848 ConVar sv_breachcharge_fuse_max
client_panorama.dll!0x00d0a7f0 ConVar sv_breachcharge_fuse_min
client_panorama.dll!0x00d0a940 ConVar sv_bumpmine_arm_delay
client_panorama.dll!0x00d0a998 ConVar sv_bumpmine_detonate_delay
client_panorama.dll!0x00cfe9e8 ConVar sv_chat_proximity
client_panorama.dll!0x00ce3460 ConVar sv_clamp_unsafe_velocities
client_panorama.dll!0x00d05238 ConVar sv_clip_penetration_traces_to_players
client_panorama.dll!0x00d00418 ConVar sv_coach_comm_unrestricted
client_panorama.dll!0x00d04588 ConVar sv_coaching_enabled
client_panorama.dll!0x00d044d8 ConVar sv_competitive_official_5v5
client_panorama.dll!0x00d024c0 ConVar sv_compute_per_bot_difficulty
client_panorama.dll!0x00d051e0 ConVar sv_cs_player_speed_has_hostage
client_panorama.dll!0x00d02938 ConVar sv_deadtalk
client_panorama.dll!0x00ce3910 ConVar sv_debug_player_use
client_panorama.dll!0x00d04110 ConVar sv_disable_immunity_alpha
client_panorama.dll!0x00cfeb48 ConVar sv_disable_motd
client_panorama.dll!0x00d042c8 ConVar sv_disable_observer_interpolation
client_panorama.dll!0x00cfe830 ConVar sv_disable_radar
client_panorama.dll!0x00d08ad8 ConVar sv_dz_enable_respawn
client_panorama.dll!0x00d00520 ConVar sv_dz_hostage_rescue_reward
client_panorama.dll!0x00d00578 ConVar sv_dz_squad_wipe_reward
client_panorama.dll!0x00d08b30 ConVar sv_dz_team_count
client_panorama.dll!0x00cea4f8 ConVar sv_dz_zone_bombdrop_money_reward
client_panorama.dll!0x00cea550 ConVar sv_dz_zone_bombdrop_money_reward_bonus
client_panorama.dll!0x00cea4a0 ConVar sv_dz_zone_hex_radius
client_panorama.dll!0x00cfe3e8 ConVar sv_enablebunnyhopping
client_panorama.dll!0x00d02308 ConVar sv_endmatch_item_drop_interval
client_panorama.dll!0x00d02468 ConVar sv_endmatch_item_drop_interval_ancient
client_panorama.dll!0x00d02410 ConVar sv_endmatch_item_drop_interval_legendary
client_panorama.dll!0x00d023b8 ConVar sv_endmatch_item_drop_interval_mythical
client_panorama.dll!0x00d02360 ConVar sv_endmatch_item_drop_interval_rare
client_panorama.dll!0x00cfe220 ConVar sv_exojump_jumpbonus_forward
client_panorama.dll!0x00cfe1c8 ConVar sv_exojump_jumpbonus_up
client_panorama.dll!0x00cfe278 ConVar sv_exojump_soundramp
client_panorama.dll!0x00cfdeb0 ConVar sv_exostaminajumpcost
client_panorama.dll!0x00cfdf08 ConVar sv_exostaminalandcost
client_panorama.dll!0x00ce3700 ConVar sv_extract_ammo_from_dropped_weapons
client_panorama.dll!0x00cfe068 ConVar sv_extreme_strafe_accuracy_fishtail
client_panorama.dll!0x00cfe170 ConVar sv_falldamage_exojump_multiplier
client_panorama.dll!0x00cfe8e0 ConVar sv_falldamage_scale
client_panorama.dll!0x00cfe990 ConVar sv_falldamage_to_below_player_multiplier
client_panorama.dll!0x00cfe938 ConVar sv_falldamage_to_below_player_ratio
client_panorama.dll!0x00d0c3e8 ConVar sv_fistpoint_delay
client_panorama.dll!0x00d0c390 ConVar sv_fistpunch_blocked_damage
client_panorama.dll!0x00d0c230 ConVar sv_fistpunch_damage
client_panorama.dll!0x00d0c2e0 ConVar sv_fistpunch_damage_hard
client_panorama.dll!0x00d0c288 ConVar sv_fistpunch_damage_to_player_multiplier
client_panorama.dll!0x00d0c440 ConVar sv_fistpunch_impact_sounds
client_panorama.dll!0x00d0c338 ConVar sv_fistpunch_viewmove
client_panorama.dll!0x00ce36a8 ConVar sv_footstep_sound_frequency
client_panorama.dll!0x00cf2520 ConVar sv_footsteps
client_panorama.dll!0x00d08b88 ConVar sv_force_reflections
client_panorama.dll!0x00cf1ef0 ConVar sv_friction
client_panorama.dll!0x00d02990 ConVar sv_full_alltalk
client_panorama.dll!0x00cec2e0 ConVar sv_grassburn
client_panorama.dll!0x00cf36e0 ConVar sv_grenade_trajectory
client_panorama.dll!0x00cf3840 ConVar sv_grenade_trajectory_dash
client_panorama.dll!0x00cf37e8 ConVar sv_grenade_trajectory_thickness
client_panorama.dll!0x00cf3738 ConVar sv_grenade_trajectory_time
client_panorama.dll!0x00cf3790 ConVar sv_grenade_trajectory_time_spectator
client_panorama.dll!0x00ce51d8 ConVar sv_highlight_distance
client_panorama.dll!0x00ce5180 ConVar sv_highlight_duration
client_panorama.dll!0x00cfe620 ConVar sv_holiday_mode
client_panorama.dll!0x00cecc90 ConVar sv_hudhint_sound
client_panorama.dll!0x00ce3a70 ConVar sv_infinite_ammo
client_panorama.dll!0x00cfe0c0 ConVar sv_jump_impulse
client_panorama.dll!0x00cfe118 ConVar sv_jump_impulse_exojump_multiplier
client_panorama.dll!0x00d026d0 ConVar sv_kick_ban_duration
client_panorama.dll!0x00d0c500 ConVar sv_knife_attack_extend_from_player_aabb
client_panorama.dll!0x00cebab0 ConVar sv_ladder_angle
client_panorama.dll!0x00ceba58 ConVar sv_ladder_dampen
client_panorama.dll!0x00cebb08 ConVar sv_ladder_scale_speed
client_panorama.dll!0x00cfe2d0 ConVar sv_ledge_mantle_helper
client_panorama.dll!0x00cfe380 ConVar sv_ledge_mantle_helper_debug
client_panorama.dll!0x00cfe328 ConVar sv_ledge_mantle_helper_dzonly
client_panorama.dll!0x00cffd38 ConVar sv_matchpause_auto_5v5
client_panorama.dll!0x00ce38b8 ConVar sv_max_distance_transmit_footsteps
client_panorama.dll!0x00cf24c8 ConVar sv_maxspeed
client_panorama.dll!0x00cf2208 ConVar sv_maxvelocity
client_panorama.dll!0x00d05188 ConVar sv_min_jump_landing_sound
client_panorama.dll!0x00cf28e0 ConVar sv_mumble_positionalaudio
client_panorama.dll!0x00cf22b8 ConVar sv_noclipaccelerate
client_panorama.dll!0x00cefe40 ConVar sv_noclipduringpause
client_panorama.dll!0x00cf2310 ConVar sv_noclipspeed
client_panorama.dll!0x00cebbb8 ConVar sv_optimizedmovement
client_panorama.dll!0x00cfe888 ConVar sv_outofammo_indicator
client_panorama.dll!0x00d004c8 ConVar sv_party_mode
client_panorama.dll!0x00d04e18 ConVar sv_penetration_type
client_panorama.dll!0x00cf2b08 ConVar sv_pushaway_clientside
client_panorama.dll!0x00cf4158 ConVar sv_pushaway_clientside_size
client_panorama.dll!0x00cf2a00 ConVar sv_pushaway_force
client_panorama.dll!0x00cf2ab0 ConVar sv_pushaway_max_force
client_panorama.dll!0x00cf2bb8 ConVar sv_pushaway_max_player_force
client_panorama.dll!0x00cf2a58 ConVar sv_pushaway_min_player_speed
client_panorama.dll!0x00cf2b60 ConVar sv_pushaway_player_force
client_panorama.dll!0x00cfe5c8 ConVar sv_reward_drop_delay
client_panorama.dll!0x00cf25d0 ConVar sv_rollangle
client_panorama.dll!0x00cf2578 ConVar sv_rollspeed
client_panorama.dll!0x00cfe518 ConVar sv_server_graphic1
client_panorama.dll!0x00cfe570 ConVar sv_server_graphic2
client_panorama.dll!0x00d052e8 ConVar sv_server_verify_blood_on_player
client_panorama.dll!0x00d0c788 ConVar sv_shield_explosive_damage_cap
client_panorama.dll!0x00d0c838 ConVar sv_shield_explosive_damage_crouch_bonus
client_panorama.dll!0x00d0c7e0 ConVar sv_shield_explosive_damage_mindist
client_panorama.dll!0x00d0c730 ConVar sv_shield_explosive_damage_mult
client_panorama.dll!0x00d0c6d8 ConVar sv_shield_explosive_damage_scale
client_panorama.dll!0x00d0c8c0 ConVar sv_shield_hitpoints
client_panorama.dll!0x00d040b8 ConVar sv_show_bot_difficulty_in_name
client_panorama.dll!0x00cfb980 ConVar sv_show_ragdoll_playernames
client_panorama.dll!0x00cfe728 ConVar sv_show_team_equipment_force_on
client_panorama.dll!0x00cfe6d0 ConVar sv_show_team_equipment_prohibit
client_panorama.dll!0x00d05468 ConVar sv_showbullethits
client_panorama.dll!0x00d04ec8 ConVar sv_showimpacts
client_panorama.dll!0x00d04e70 ConVar sv_showimpacts_penetration
client_panorama.dll!0x00d04f20 ConVar sv_showimpacts_time
client_panorama.dll!0x00d04f78 ConVar sv_showplayerhitboxes
client_panorama.dll!0x00d04530 ConVar sv_skirmish_id
client_panorama.dll!0x00cf1c88 ConVar sv_skyname
client_panorama.dll!0x00cf4ae8 ConVar sv_soundemitter_trace
client_panorama.dll!0x00cf49e0 ConVar sv_soundemitter_version
client_panorama.dll!0x00d02a40 ConVar sv_spec_hear
client_panorama.dll!0x00d05410 ConVar sv_spec_use_tournament_content_standards
client_panorama.dll!0x00cf2158 ConVar sv_specaccelerate
client_panorama.dll!0x00cf2100 ConVar sv_specnoclip
client_panorama.dll!0x00cf21b0 ConVar sv_specspeed
client_panorama.dll!0x00cfde00 ConVar sv_staminajumpcost
client_panorama.dll!0x00cfde58 ConVar sv_staminalandcost
client_panorama.dll!0x00cfdfb8 ConVar sv_staminamax
client_panorama.dll!0x00cfdf60 ConVar sv_staminarecoveryrate
client_panorama.dll!0x00ceb960 ConVar sv_standable_normal
client_panorama.dll!0x00cf2628 ConVar sv_stepsize
client_panorama.dll!0x00cf1fa0 ConVar sv_stopspeed
client_panorama.dll!0x00ce3968 ConVar sv_suppress_viewpunch
client_panorama.dll!0x00d0ca30 ConVar sv_tablet_show_path_to_nearest_resq
client_panorama.dll!0x00d029e8 ConVar sv_talk_after_dying_time
client_panorama.dll!0x00d04480 ConVar sv_talk_enemy_dead
client_panorama.dll!0x00d04428 ConVar sv_talk_enemy_living
client_panorama.dll!0x00d04320 ConVar sv_teamid_overhead
client_panorama.dll!0x00cfe678 ConVar sv_teamid_overhead_always_prohibit
client_panorama.dll!0x00cfe7d8 ConVar sv_teamid_overhead_maxdist
client_panorama.dll!0x00cfe780 ConVar sv_teamid_overhead_maxdist_spec
client_panorama.dll!0x00cfe010 ConVar sv_timebetweenducks
client_panorama.dll!0x00cf2c10 ConVar sv_turbophysics
client_panorama.dll!0x00d0af38 ConVar sv_turning_inaccuracy_angle_min
client_panorama.dll!0x00d0af90 ConVar sv_turning_inaccuracy_decay
client_panorama.dll!0x00d0aee0 ConVar sv_turning_inaccuracy_enabled
client_panorama.dll!0x00ceb908 ConVar sv_walkable_normal
client_panorama.dll!0x00ceb6f8 ConVar sv_water_movespeed_multiplier
client_panorama.dll!0x00ceb750 ConVar sv_water_swim_mode
client_panorama.dll!0x00cf1bd8 ConVar sv_wateraccelerate
client_panorama.dll!0x00cf1d38 ConVar sv_waterdist
client_panorama.dll!0x00cf1c30 ConVar sv_waterfriction
client_panorama.dll!0x00d05290 ConVar sv_weapon_encumbrance_per_item
client_panorama.dll!0x00cfe498 ConVar sv_weapon_encumbrance_scale
client_panorama.dll!0x00d0c9a8 ConVar tablet_c4_dist_max
client_panorama.dll!0x00d0c950 ConVar tablet_c4_dist_min
client_panorama.dll!0x00d08958 ConVar test_convar
client_panorama.dll!0x00d08900 ConVar test_convar
client_panorama.dll!0x00cf3898 ConVar think_limit
client_panorama.dll!0x00cfc788 ConVar thirdperson_lockcamera
client_panorama.dll!0x00d04c58 ConVar tv_spectator_port_offset
client_panorama.dll!0x00d09ba0 ConVar ui_inventorysettings_recently_acknowledged
client_panorama.dll!0x00d09160 ConVar ui_lobby_draft_enabled
client_panorama.dll!0x00d09728 ConVar ui_nearbylobbies_filter
client_panorama.dll!0x00d09990 ConVar ui_playsettings_maps_listen_casual
client_panorama.dll!0x00d09888 ConVar ui_playsettings_maps_listen_competitive
client_panorama.dll!0x00d099e8 ConVar ui_playsettings_maps_listen_deathmatch
client_panorama.dll!0x00d098e0 ConVar ui_playsettings_maps_listen_scrimcomp2v2
client_panorama.dll!0x00d09938 ConVar ui_playsettings_maps_listen_skirmish
client_panorama.dll!0x00d09780 ConVar ui_playsettings_maps_official_casual
client_panorama.dll!0x00d097d8 ConVar ui_playsettings_maps_official_deathmatch
client_panorama.dll!0x00d09830 ConVar ui_playsettings_maps_official_dzsirocco
client_panorama.dll!0x00d09a40 ConVar ui_playsettings_maps_workshop
client_panorama.dll!0x00d09e08 ConVar ui_playsettings_mode_listen
client_panorama.dll!0x00d09db0 ConVar ui_playsettings_mode_official_dz
client_panorama.dll!0x00d09e60 ConVar ui_playsettings_survival_solo
client_panorama.dll!0x00d09d58 ConVar ui_playsettings_warmup_map_name
client_panorama.dll!0x00d09c50 ConVar ui_popup_weaponupdate_version
client_panorama.dll!0x00cf39e8 ConVar ui_posedebug_fade_in_time
client_panorama.dll!0x00cf3990 ConVar ui_posedebug_fade_out_time
client_panorama.dll!0x00d09a98 ConVar ui_vanitysetting_itemid
client_panorama.dll!0x00d09b48 ConVar ui_vanitysetting_loadoutslot
client_panorama.dll!0x00d09af0 ConVar ui_vanitysetting_team
client_panorama.dll!0x00d06e60 ConVar vgui_message_dialog_modal
client_panorama.dll!0x00ce35f8 ConVar view_punch_decay
client_panorama.dll!0x00ce3650 ConVar view_recoil_tracking
client_panorama.dll!0x00cf6910 ConVar viewmodel_fov
client_panorama.dll!0x00cf6a18 ConVar viewmodel_offset_randomize
client_panorama.dll!0x00ce3ac8 ConVar viewmodel_offset_x
client_panorama.dll!0x00ce3b20 ConVar viewmodel_offset_y
client_panorama.dll!0x00ce3b78 ConVar viewmodel_offset_z
client_panorama.dll!0x00ce3bd0 ConVar viewmodel_recoil
client_panorama.dll!0x00ce53b8 ConVar vm_debug
client_panorama.dll!0x00ce5410 ConVar vm_draw_always
client_panorama.dll!0x00cfa6c0 ConVar voice_modenable
client_panorama.dll!0x00d0abc8 ConVar weapon_accuracy_forcespread
client_panorama.dll!0x00d06100 ConVar weapon_accuracy_logging
client_panorama.dll!0x00d0ac20 ConVar weapon_accuracy_nospread
client_panorama.dll!0x00d06368 ConVar weapon_accuracy_shotgun_spread_patterns
client_panorama.dll!0x00d0acd0 ConVar weapon_air_spread_scale
client_panorama.dll!0x00d0ad80 ConVar weapon_auto_cleanup_time
client_panorama.dll!0x00d06260 ConVar weapon_debug_inaccuracy_only_up
client_panorama.dll!0x00d06208 ConVar weapon_debug_max_inaccuracy
client_panorama.dll!0x00d0b388 ConVar weapon_debug_spread_gap
client_panorama.dll!0x00d0b960 ConVar weapon_debug_spread_show
client_panorama.dll!0x00d0aac0 ConVar weapon_land_dip_amt
client_panorama.dll!0x00d0add8 ConVar weapon_max_before_cleanup
client_panorama.dll!0x00d061b0 ConVar weapon_near_empty_sound
client_panorama.dll!0x00d0ac78 ConVar weapon_recoil_cooldown
client_panorama.dll!0x00d0ab18 ConVar weapon_recoil_decay1_exp
client_panorama.dll!0x00d0bbc8 ConVar weapon_recoil_decay2_exp
client_panorama.dll!0x00d0bb70 ConVar weapon_recoil_decay2_lin
client_panorama.dll!0x00d0ab70 ConVar weapon_recoil_decay_coefficient
client_panorama.dll!0x00d0ba10 ConVar weapon_recoil_scale
client_panorama.dll!0x00d0b9b8 ConVar weapon_recoil_scale_motion_controller
client_panorama.dll!0x00d05750 ConVar weapon_recoil_suppression_factor
client_panorama.dll!0x00d056f8 ConVar weapon_recoil_suppression_shots
client_panorama.dll!0x00d057a8 ConVar weapon_recoil_variance
client_panorama.dll!0x00d0bc20 ConVar weapon_recoil_vel_decay
client_panorama.dll!0x00d05340 ConVar weapon_recoil_view_punch_extra
client_panorama.dll!0x00d0ad28 ConVar weapon_reticle_knife_show
client_panorama.dll!0x00d0ae30 ConVar weapon_sound_falloff_multiplier
client_panorama.dll!0x00cf6350 ConVar zoom_sensitivity_ratio_joystick
client_panorama.dll!0x00cf63a8 ConVar zoom_sensitivity_ratio_mouse
```

### ConCommands

<details>
<summary><code>+alt1</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+alt2</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+attack</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+attack2</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+back</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+break</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+camdistance</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+camin</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+cammousemove</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+camout</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+campitchdown</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+campitchup</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+camyawleft</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+camyawright</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+cl_show_team_equipment</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+commandermousemove</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+csm_rot_x_neg</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+csm_rot_x_plus</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+csm_rot_y_neg</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+csm_rot_y_plus</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+duck</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+forward</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+graph</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+grenade1</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+grenade2</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+jlook</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+jump</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+klook</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+left</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+lookdown</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+lookspin</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+lookup</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+movedown</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+moveleft</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+moveright</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+moveup</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+quickinv</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+reload</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+right</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+score</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+showscores</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+speed</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+spray_menu</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+strafe</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+use</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+walk</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+zoom</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+zoom_in</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>+zoom_out</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-alt1</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-alt2</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-attack</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-attack2</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-back</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-break</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-camdistance</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-camin</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-cammousemove</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-camout</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-campitchdown</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-campitchup</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-camyawleft</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-camyawright</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-cl_show_team_equipment</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-commandermousemove</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-csm_rot_x_neg</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-csm_rot_x_plus</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-csm_rot_y_neg</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-csm_rot_y_plus</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-duck</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-forward</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-graph</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-grenade1</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-grenade2</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-jlook</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-jump</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-klook</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-left</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-lookdown</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-lookspin</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-lookup</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-movedown</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-moveleft</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-moveright</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-moveup</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-quickinv</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-reload</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-right</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-score</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-showscores</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-speed</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-spray_menu</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-strafe</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-use</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-walk</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-zoom</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-zoom_in</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>-zoom_out</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>CreateHairball</code></summary>



flags: `0x4002`  
</details>
<details>
<summary><code>ShowSteamStatsSessionID</code></summary>

Prints out the game stats session ID's (developer convar must be set to non-zero).

flags: `0x2`  
</details>
<details>
<summary><code>Test_ProxyToggle_EnsureValue</code></summary>

Test_ProxyToggle_EnsureValue

flags: `0x4000`  
</details>
<details>
<summary><code>autobuy</code></summary>

Attempt to purchase items with the order listed in cl_autobuy

flags: `0x40000000`  
</details>
<details>
<summary><code>bench_showstatsdialog</code></summary>

Shows a dialog displaying the most recent benchmark results.

flags: `0x4000`  
</details>
<details>
<summary><code>buy_stamps</code></summary>

Temporary solution for Pinion to kick back to community map makers.

flags: `0x0`  
</details>
<details>
<summary><code>buymenu</code></summary>

Show or hide main buy menu

flags: `0x10000000`  
</details>
<details>
<summary><code>cam_command</code></summary>

Tells camera to change modes

flags: `0x4000`  
</details>
<details>
<summary><code>camortho</code></summary>

Switch to orthographic camera.

flags: `0x4002`  
</details>
<details>
<summary><code>cancelselect</code></summary>



flags: `0x50000000`  
</details>
<details>
<summary><code>cc_emit</code></summary>

Emits a closed caption

flags: `0x0`  
</details>
<details>
<summary><code>cc_findsound</code></summary>

Searches for soundname which emits specified text.

flags: `0x0`  
</details>
<details>
<summary><code>cc_flush</code></summary>

Flushes async'd captions.

flags: `0x0`  
</details>
<details>
<summary><code>cc_random</code></summary>

Emits a random caption

flags: `0x0`  
</details>
<details>
<summary><code>cc_showblocks</code></summary>

Toggles showing which blocks are pending/loaded async.

flags: `0x0`  
</details>
<details>
<summary><code>centerview</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>cl_animationinfo</code></summary>

Hud element to examine.

flags: `0x0`  
</details>
<details>
<summary><code>cl_avatar_convert_rgb</code></summary>

Converts all png avatars in the avatars directory to rgb

flags: `0x84000`  
</details>
<details>
<summary><code>cl_clearhinthistory</code></summary>

Clear memory of client side hints displayed to the player.

flags: `0x0`  
</details>
<details>
<summary><code>cl_cs_dump_econ_item_stringtable</code></summary>

cl_cs_dump_econ_item_stringtable

flags: `0x0`  
</details>
<details>
<summary><code>cl_csm_status</code></summary>

Usage:
   cl_csm_status


flags: `0x0`  
</details>
<details>
<summary><code>cl_dev_decaltrace_blood</code></summary>

Shoot out a decal spray that shoots blood.

flags: `0x4002`  
</details>
<details>
<summary><code>cl_dump_particle_stats</code></summary>

dump particle profiling info to particle_profile.csv

flags: `0x0`  
</details>
<details>
<summary><code>cl_dumpplayer</code></summary>

Dumps info about a player

flags: `0x4000`  
</details>
<details>
<summary><code>cl_dumpsplithacks</code></summary>

Dump split screen workarounds.

flags: `0x0`  
</details>
<details>
<summary><code>cl_ent_absbox</code></summary>

Displays the client's absbox for the entity under the crosshair.

flags: `0x4000`  
</details>
<details>
<summary><code>cl_ent_bbox</code></summary>

Displays the client's bounding box for the entity under the crosshair.

flags: `0x4000`  
</details>
<details>
<summary><code>cl_ent_rbox</code></summary>

Displays the client's render box for the entity under the crosshair.

flags: `0x4000`  
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
<summary><code>cl_game_mode_convars</code></summary>

Display the values of the convars for the current game_mode.

flags: `0x0`  
</details>
<details>
<summary><code>cl_mainmenu_blog_file</code></summary>

Load blog file

flags: `0x40000010`  
</details>
<details>
<summary><code>cl_mainmenu_hide_blog</code></summary>

Show the news panel and hide blog

flags: `0x40000010`  
</details>
<details>
<summary><code>cl_mainmenu_show_blog</code></summary>

Show the blog and hide news panel

flags: `0x40000010`  
</details>
<details>
<summary><code>cl_matchstats_print_own_data</code></summary>

cl_matchstats_print_own_data RANGENAME

flags: `0x0`  
</details>
<details>
<summary><code>cl_modemanager_reload</code></summary>

Reloads the panel metaclasses for vgui screens.

flags: `0x0`  
</details>
<details>
<summary><code>cl_panelanimation</code></summary>

Shows panel animation variables: <panelname | blank for all panels>.

flags: `0x0`  
</details>
<details>
<summary><code>cl_particles_dump_effects</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>cl_particles_dumplist</code></summary>

Dump all new particles, optional name substring.

flags: `0x4000`  
</details>
<details>
<summary><code>cl_player_rank_events_spew</code></summary>

Spews the contents of all events this round that could be displayed to the player, as well as the player's current ranks.

flags: `0x2`  
</details>
<details>
<summary><code>cl_pred_track</code></summary>

<entindex> <fieldname>:  Track changes to entity index entindex, for field fieldname.

flags: `0x0`  
</details>
<details>
<summary><code>cl_predictioncopy_describe</code></summary>

Describe datamap_t for entindex

flags: `0x0`  
</details>
<details>
<summary><code>cl_quest_events_print</code></summary>

cl_quest_events_print

flags: `0x0`  
</details>
<details>
<summary><code>cl_quest_schedule_print</code></summary>

cl_quest_schedule_print

flags: `0x0`  
</details>
<details>
<summary><code>cl_reload_hud</code></summary>

Reloads the hud scale and resets scale and borders

flags: `0x0`  
</details>
<details>
<summary><code>cl_reloadpostprocessparams</code></summary>



flags: `0x4000`  
</details>
<details>
<summary><code>cl_remove_all_workshop_maps</code></summary>

Removes all maps from the workshop directory.

flags: `0x0`  
</details>
<details>
<summary><code>cl_removedecals</code></summary>

Remove the decals from the entity under the crosshair.

flags: `0x4000`  
</details>
<details>
<summary><code>cl_report_soundpatch</code></summary>

reports client-side sound patch count

flags: `0x0`  
</details>
<details>
<summary><code>cl_sos_test_get_opvar</code></summary>



flags: `0x4000`  
</details>
<details>
<summary><code>cl_sos_test_set_opvar</code></summary>



flags: `0x4000`  
</details>
<details>
<summary><code>cl_soundemitter_flush</code></summary>

Flushes the sounds.txt system (server only)

flags: `0x0`  
</details>
<details>
<summary><code>cl_soundemitter_reload</code></summary>

Flushes the sounds.txt system

flags: `0x0`  
</details>
<details>
<summary><code>cl_soundscape_flush</code></summary>

Flushes the client side soundscapes

flags: `0x10004000`  
</details>
<details>
<summary><code>cl_soundscape_printdebuginfo</code></summary>

print soundscapes

flags: `0x0`  
</details>
<details>
<summary><code>cl_ss_origin</code></summary>

print origin in script format

flags: `0x0`  
</details>
<details>
<summary><code>cl_steamscreenshots</code></summary>

Enable/disable saving screenshots to Steam

flags: `0x0`  
</details>
<details>
<summary><code>cl_tree_sway_dir</code></summary>

sets tree sway wind direction and strength

flags: `0x0`  
</details>
<details>
<summary><code>cl_updatevisibility</code></summary>

Updates visibility bits.

flags: `0x0`  
</details>
<details>
<summary><code>commentary_showmodelviewer</code></summary>

Display the commentary model viewer. Usage: commentary_showmodelviewer <model name> <optional attached model name>

flags: `0x0`  
</details>
<details>
<summary><code>commentary_testfirstrun</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>condump</code></summary>

dump the text currently in the console to condumpXX.log

flags: `0x0`  
</details>
<details>
<summary><code>confirm_abandon_match</code></summary>

Confirm that we wish to abandon match

flags: `0x40000010`  
</details>
<details>
<summary><code>confirm_activate_itemid_now</code></summary>

Confirm item activation by item id

flags: `0x40000010`  
</details>
<details>
<summary><code>confirm_join_friend_session_exit_current</code></summary>

Confirm that we wish to join a friend session, destroying a previous session

flags: `0x40000010`  
</details>
<details>
<summary><code>confirm_join_new_session_exit_current</code></summary>

Confirm that we wish to join a new session, destroying a previous session

flags: `0x40000010`  
</details>
<details>
<summary><code>confirm_join_party_session_exit_current</code></summary>

Confirm that we wish to join a party session, destroying a previous session

flags: `0x40000010`  
</details>
<details>
<summary><code>confirm_purchase_item_def_now</code></summary>

Confirm item purchase

flags: `0x40000010`  
</details>
<details>
<summary><code>confirm_watch_friend_session_exit_current</code></summary>

Confirm that we wish to watch a friend session, destroying a previous session

flags: `0x40000010`  
</details>
<details>
<summary><code>csgo_download_match</code></summary>

Downloads a match via serial code and starts playback

flags: `0x40020000`  
</details>
<details>
<summary><code>csgo_econ_action_preview</code></summary>

Preview an economy item

flags: `0x40020010`  
</details>
<details>
<summary><code>debug_purchase_defidx</code></summary>

Purchase an item by defindex

flags: `0x40080000`  
</details>
<details>
<summary><code>dlight_debug</code></summary>

Creates a dlight in front of the player

flags: `0x4000`  
</details>
<details>
<summary><code>dm_togglerandomweapons</code></summary>

Turns random weapons in deathmatch on/off

flags: `0x50000000`  
</details>
<details>
<summary><code>drawoverviewmap</code></summary>

Draws the overview map

flags: `0x0`  
</details>
<details>
<summary><code>drawradar</code></summary>

Draws HUD radar

flags: `0x0`  
</details>
<details>
<summary><code>dump_particlemanifest</code></summary>

Dump the list of particles loaded.

flags: `0x4000`  
</details>
<details>
<summary><code>econ_build_pinboard_images_from_collection_name</code></summary>

Renders and saves images for all models in a collection.

flags: `0x0`  
</details>
<details>
<summary><code>econ_show_items_with_tag</code></summary>

Lists the item definitions that have a specified tag.

flags: `0x8`  
</details>
<details>
<summary><code>endmatch_votenextmap</code></summary>

Votes for the next map at the end of the match

flags: `0x40000000`  
</details>
<details>
<summary><code>error_message_explain_pure</code></summary>

Take user to Steam support article

flags: `0x40000010`  
</details>
<details>
<summary><code>error_message_explain_vac</code></summary>

Take user to Steam support article

flags: `0x40000010`  
</details>
<details>
<summary><code>firstperson</code></summary>

Switch to firstperson camera.

flags: `0x10000000`  
</details>
<details>
<summary><code>force_centerview</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>g15_dumpplayer</code></summary>

Spew player data.

flags: `0x0`  
</details>
<details>
<summary><code>g15_reload</code></summary>

Reloads the Logitech G-15 Keyboard configs.

flags: `0x0`  
</details>
<details>
<summary><code>gameinstructor_dump_open_lessons</code></summary>

Gives a list of all currently open lessons.

flags: `0x4000`  
</details>
<details>
<summary><code>gameinstructor_reload_lessons</code></summary>

Shuts down all open lessons and reloads them from the script file.

flags: `0x4000`  
</details>
<details>
<summary><code>gameinstructor_reset_counts</code></summary>

Resets all display and success counts to zero.

flags: `0x0`  
</details>
<details>
<summary><code>gamemenucommand</code></summary>

Issue game menu command.

flags: `0x0`  
</details>
<details>
<summary><code>gamepadslot1</code></summary>



flags: `0x10000000`  
</details>
<details>
<summary><code>gamepadslot2</code></summary>



flags: `0x10000000`  
</details>
<details>
<summary><code>gamepadslot3</code></summary>



flags: `0x10000000`  
</details>
<details>
<summary><code>gamepadslot4</code></summary>



flags: `0x10000000`  
</details>
<details>
<summary><code>gamepadslot5</code></summary>



flags: `0x10000000`  
</details>
<details>
<summary><code>gamepadslot6</code></summary>



flags: `0x10000000`  
</details>
<details>
<summary><code>gcmd</code></summary>

Generate a command

flags: `0x40000010`  
</details>
<details>
<summary><code>getpos</code></summary>

dump position and angles to the console

flags: `0x0`  
</details>
<details>
<summary><code>getpos_exact</code></summary>

dump origin and angles to the console

flags: `0x0`  
</details>
<details>
<summary><code>hideoverviewmap</code></summary>

Hides the overview map

flags: `0x0`  
</details>
<details>
<summary><code>hidepanel</code></summary>

Hides a viewport panel <name>

flags: `0x0`  
</details>
<details>
<summary><code>hideradar</code></summary>

Hides HUD radar

flags: `0x0`  
</details>
<details>
<summary><code>hidescores</code></summary>

Forcibly hide score panel

flags: `0x40000000`  
</details>
<details>
<summary><code>highlight_sticker</code></summary>



flags: `0x4002`  
</details>
<details>
<summary><code>hud_reloadscheme</code></summary>

Reloads hud layout and animation scripts.

flags: `0x0`  
</details>
<details>
<summary><code>hud_subtitles</code></summary>

Plays the Subtitles: <filename>

flags: `0x0`  
</details>
<details>
<summary><code>impulse</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>invnext</code></summary>



flags: `0x10000000`  
</details>
<details>
<summary><code>invnextgrenade</code></summary>



flags: `0x10000000`  
</details>
<details>
<summary><code>invnextitem</code></summary>



flags: `0x10000000`  
</details>
<details>
<summary><code>invnextnongrenade</code></summary>



flags: `0x10000000`  
</details>
<details>
<summary><code>invprev</code></summary>



flags: `0x10000000`  
</details>
<details>
<summary><code>joyadvancedupdate</code></summary>



flags: `0x40000000`  
</details>
<details>
<summary><code>lastinv</code></summary>



flags: `0x10000000`  
</details>
<details>
<summary><code>launch_warmup_map</code></summary>

Launches warmup map

flags: `0x40020000`  
</details>
<details>
<summary><code>list_active_casters</code></summary>

List currently active casters.

flags: `0x80018`  
</details>
<details>
<summary><code>loadcommentary</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>localization_quest_item_string_printout</code></summary>

localization_quest_item_string_printout

flags: `0x0`  
</details>
<details>
<summary><code>mat_reloadwearablecustommaterials</code></summary>

Reloads wearable custom materials for the local player

flags: `0x400a`  
</details>
<details>
<summary><code>matchdraft_debug_sendlog</code></summary>

Print debug draft into HTTP log

flags: `0x400a0010`  
</details>
<details>
<summary><code>menuselect</code></summary>

menuselect

flags: `0x40000000`  
</details>
<details>
<summary><code>mm_queue_draft_show</code></summary>

Display current draft

flags: `0x40000010`  
</details>
<details>
<summary><code>mm_queue_show_stats</code></summary>

Display global server stats

flags: `0x40000000`  
</details>
<details>
<summary><code>modelpanel_set_sticker</code></summary>

[Slot] [Id] Adds a sticker to the 3d weapon preview model

flags: `0x4002`  
</details>
<details>
<summary><code>peel_sticker</code></summary>



flags: `0x4002`  
</details>
<details>
<summary><code>perfectworld_replenish_funds</code></summary>

Opens Perfect World funds replenishment page for account.

flags: `0x40000010`  
</details>
<details>
<summary><code>perfvisualbenchmark</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>perfvisualbenchmark_abort</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>pick_hint</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>pixelvis_debug</code></summary>

Dump debug info

flags: `0x0`  
</details>
<details>
<summary><code>playgamesound</code></summary>

Play a sound from the game sounds txt file

flags: `0x50004000`  
</details>
<details>
<summary><code>playsoundscape</code></summary>

Forces a soundscape to play

flags: `0x4000`  
</details>
<details>
<summary><code>playvideo</code></summary>

Plays a video: <filename> [width height]

flags: `0x0`  
</details>
<details>
<summary><code>playvideo_end_level_transition</code></summary>

Plays a video fullscreen without ability to skip (unless dev 1) and fades in: <filename> <time>

flags: `0x0`  
</details>
<details>
<summary><code>playvideo_exitcommand</code></summary>

Plays a video and fires and exit command when it is stopped or finishes: <filename> <exit command>

flags: `0x0`  
</details>
<details>
<summary><code>playvideo_exitcommand_nointerrupt</code></summary>

Plays a video (without interruption) and fires and exit command when it is stopped or finishes: <filename> <exit command>

flags: `0x0`  
</details>
<details>
<summary><code>playvideo_nointerrupt</code></summary>

Plays a video without ability to skip: <filename> [width height]

flags: `0x0`  
</details>
<details>
<summary><code>print_achievement_categories</code></summary>

Spews achievements for each category

flags: `0x2`  
</details>
<details>
<summary><code>print_mapgroup</code></summary>

Prints the current mapgroup and the contained maps

flags: `0x80000`  
</details>
<details>
<summary><code>quit_prompt</code></summary>

Exit the engine.

flags: `0x0`  
</details>
<details>
<summary><code>r_cheapwaterend</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>r_cheapwaterstart</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>r_ropes_holiday_light_color</code></summary>

Set each light's color: [light0-3] [r0-255] [g0-255] [b0-255]

flags: `0x0`  
</details>
<details>
<summary><code>r_screenoverlay</code></summary>

Draw specified material as an overlay

flags: `0x10004000`  
</details>
<details>
<summary><code>r_shadowangles</code></summary>

Set shadow angles

flags: `0x4000`  
</details>
<details>
<summary><code>r_shadowblobbycutoff</code></summary>

some shadow stuff

flags: `0x4000`  
</details>
<details>
<summary><code>r_shadowcolor</code></summary>

Set shadow color

flags: `0x4000`  
</details>
<details>
<summary><code>r_shadowdir</code></summary>

Set shadow direction

flags: `0x4000`  
</details>
<details>
<summary><code>r_shadowdist</code></summary>

Set shadow distance

flags: `0x4000`  
</details>
<details>
<summary><code>radio</code></summary>

Opens a radio menu

flags: `0x0`  
</details>
<details>
<summary><code>radio1</code></summary>

Opens a radio menu

flags: `0x0`  
</details>
<details>
<summary><code>radio2</code></summary>

Opens a radio menu

flags: `0x0`  
</details>
<details>
<summary><code>radio3</code></summary>

Opens a radio menu

flags: `0x0`  
</details>
<details>
<summary><code>rangefinder</code></summary>

rangefinder

flags: `0x4000`  
</details>
<details>
<summary><code>rebuy</code></summary>

Attempt to repurchase items with the order listed in cl_rebuy

flags: `0x40000000`  
</details>
<details>
<summary><code>reload_store_config</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>script_client</code></summary>

Run the text as a script

flags: `0x0`  
</details>
<details>
<summary><code>script_debug_client</code></summary>

Connect the vscript VM to the script debugger

flags: `0x0`  
</details>
<details>
<summary><code>script_dump_all_client</code></summary>

Dump the state of the VM to the console

flags: `0x0`  
</details>
<details>
<summary><code>script_execute_client</code></summary>

Run a vscript file

flags: `0x0`  
</details>
<details>
<summary><code>script_help_client</code></summary>

Output help for script functions, optionally with a search string

flags: `0x0`  
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
<summary><code>show_loadout_toggle</code></summary>

Toggles loadout display

flags: `0x40000000`  
</details>
<details>
<summary><code>showinfo</code></summary>

Shows a info panel: <type> <title> <message> [<command number>]

flags: `0x0`  
</details>
<details>
<summary><code>showpanel</code></summary>

Shows a viewport panel <name>

flags: `0x0`  
</details>
<details>
<summary><code>slot0</code></summary>



flags: `0x10000000`  
</details>
<details>
<summary><code>slot1</code></summary>



flags: `0x10000000`  
</details>
<details>
<summary><code>slot10</code></summary>



flags: `0x10000000`  
</details>
<details>
<summary><code>slot11</code></summary>



flags: `0x10000000`  
</details>
<details>
<summary><code>slot12</code></summary>



flags: `0x10000000`  
</details>
<details>
<summary><code>slot13</code></summary>



flags: `0x10000000`  
</details>
<details>
<summary><code>slot2</code></summary>



flags: `0x10000000`  
</details>
<details>
<summary><code>slot3</code></summary>



flags: `0x10000000`  
</details>
<details>
<summary><code>slot4</code></summary>



flags: `0x10000000`  
</details>
<details>
<summary><code>slot5</code></summary>



flags: `0x10000000`  
</details>
<details>
<summary><code>slot6</code></summary>



flags: `0x10000000`  
</details>
<details>
<summary><code>slot7</code></summary>



flags: `0x10000000`  
</details>
<details>
<summary><code>slot8</code></summary>



flags: `0x10000000`  
</details>
<details>
<summary><code>slot9</code></summary>



flags: `0x10000000`  
</details>
<details>
<summary><code>snapto</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>snd_playsounds</code></summary>

Play sounds from the game sounds txt file at a given location

flags: `0x50004000`  
</details>
<details>
<summary><code>snd_setsoundparam</code></summary>

Set a sound paramater

flags: `0x50000000`  
</details>
<details>
<summary><code>soundscape_dumpclient</code></summary>

Dumps the client's soundscape data.


flags: `0x4000`  
</details>
<details>
<summary><code>spec_cameraman_set_xray</code></summary>

Client command to change the whether the spectator is using the cameraman's X-ray state, if they are active, or let the spectator choose.

flags: `0x40000000`  
</details>
<details>
<summary><code>spec_goto</code></summary>

Move spectator to specified origin and eyes to specified pitch yaw.

flags: `0x40000000`  
</details>
<details>
<summary><code>spec_gui</code></summary>

Shows or hides the spectator bar

flags: `0x40000000`  
</details>
<details>
<summary><code>spec_lerpto</code></summary>

Lerp the spectator camera to specified origin and eyes to specified pitch yaw.

flags: `0x40000000`  
</details>
<details>
<summary><code>spec_menu</code></summary>

Activates spectator menu

flags: `0x40000000`  
</details>
<details>
<summary><code>spec_mode</code></summary>

Set spectator mode

flags: `0x40000000`  
</details>
<details>
<summary><code>spec_next</code></summary>

Spectate next player

flags: `0x40000000`  
</details>
<details>
<summary><code>spec_player</code></summary>

Spectate player by index

flags: `0x40000000`  
</details>
<details>
<summary><code>spec_player_by_accountid</code></summary>

Spectate player by Steam account ID

flags: `0x40000000`  
</details>
<details>
<summary><code>spec_player_by_name</code></summary>

Spectate player by name

flags: `0x40000000`  
</details>
<details>
<summary><code>spec_pos</code></summary>

dump position and angles to the console

flags: `0x0`  
</details>
<details>
<summary><code>spec_prev</code></summary>

Spectate previous player ( valid values are 3 to 6 )

flags: `0x40000000`  
</details>
<details>
<summary><code>ss_reloadletterbox</code></summary>

ss_reloadletterbox

flags: `0x0`  
</details>
<details>
<summary><code>ss_teleport</code></summary>

Teleport other splitscreen player to my location.

flags: `0x4002`  
</details>
<details>
<summary><code>stop_transition_videos_fadeout</code></summary>

Fades out all transition videos playing to the screen: <time>

flags: `0x0`  
</details>
<details>
<summary><code>stopsoundscape</code></summary>

Stops all soundscape processing and fades current looping sounds

flags: `0x4000`  
</details>
<details>
<summary><code>stopvideos</code></summary>

Stops all videos playing to the screen

flags: `0x0`  
</details>
<details>
<summary><code>stopvideos_fadeout</code></summary>

Fades out all videos playing to the screen: <time>

flags: `0x0`  
</details>
<details>
<summary><code>teammenu</code></summary>

Show team selection window

flags: `0x10000000`  
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


flags: `0x4000`  
</details>
<details>
<summary><code>thirdperson</code></summary>

Switch to thirdperson camera.

flags: `0x10004000`  
</details>
<details>
<summary><code>thirdperson_mayamode</code></summary>

Switch to thirdperson Maya-like camera controls.

flags: `0x4000`  
</details>
<details>
<summary><code>thirdpersonoverview</code></summary>

Switch to thirdperson-overview camera.

flags: `0x4002`  
</details>
<details>
<summary><code>thirdpersonshoulder</code></summary>

Switch to thirdperson-shoulder camera.

flags: `0x4002`  
</details>
<details>
<summary><code>toggleRdrOpt</code></summary>



flags: `0x2`  
</details>
<details>
<summary><code>toggleThreadedBuildRWList</code></summary>

toggleThreadedBuildRWList

flags: `0x2`  
</details>
<details>
<summary><code>toggle_duck</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>togglescores</code></summary>

Toggles score panel

flags: `0x40000000`  
</details>
<details>
<summary><code>ui_reloadscheme</code></summary>

Reloads the resource files for the active UI window

flags: `0x0`  
</details>
<details>
<summary><code>viewanim_addkeyframe</code></summary>



flags: `0x4000`  
</details>
<details>
<summary><code>viewanim_create</code></summary>

viewanim_create

flags: `0x0`  
</details>
<details>
<summary><code>viewanim_load</code></summary>

load animation from file

flags: `0x0`  
</details>
<details>
<summary><code>viewanim_reset</code></summary>

reset view angles!

flags: `0x4000`  
</details>
<details>
<summary><code>viewanim_save</code></summary>

Save current animation to file

flags: `0x0`  
</details>
<details>
<summary><code>viewanim_test</code></summary>

test view animation

flags: `0x0`  
</details>
<details>
<summary><code>voice_status_test_toggle</code></summary>

Test voice and status notices

flags: `0x2`  
</details>
<details>
<summary><code>workshop_publish</code></summary>

Bring up the Workshop Publish dialog.

flags: `0xa0000`  
</details>
<details>
<summary><code>workshop_workbench</code></summary>

Bring up the Workshop workbench dialog.

flags: `0xa0000`  
</details>
<details>
<summary><code>xlook</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>xmove</code></summary>



flags: `0x0`  
</details>

#### Addresses

```
client_panorama.dll!0x00cf082c ConCommand +alt1
client_panorama.dll!0x00cf0874 ConCommand +alt2
client_panorama.dll!0x00cf05c8 ConCommand +attack
client_panorama.dll!0x00cf0610 ConCommand +attack2
client_panorama.dll!0x00cf0388 ConCommand +back
client_panorama.dll!0x00cf0994 ConCommand +break
client_panorama.dll!0x00cee044 ConCommand +camdistance
client_panorama.dll!0x00cede94 ConCommand +camin
client_panorama.dll!0x00cedffc ConCommand +cammousemove
client_panorama.dll!0x00cededc ConCommand +camout
client_panorama.dll!0x00ceddbc ConCommand +campitchdown
client_panorama.dll!0x00cedd74 ConCommand +campitchup
client_panorama.dll!0x00cede04 ConCommand +camyawleft
client_panorama.dll!0x00cede4c ConCommand +camyawright
client_panorama.dll!0x00d086d8 ConCommand +cl_show_team_equipment
client_panorama.dll!0x00cf01d8 ConCommand +commandermousemove
client_panorama.dll!0x00ce6684 ConCommand +csm_rot_x_neg
client_panorama.dll!0x00ce663c ConCommand +csm_rot_x_plus
client_panorama.dll!0x00ce6714 ConCommand +csm_rot_y_neg
client_panorama.dll!0x00ce66cc ConCommand +csm_rot_y_plus
client_panorama.dll!0x00cf079c ConCommand +duck
client_panorama.dll!0x00cf0340 ConCommand +forward
client_panorama.dll!0x00cf094c ConCommand +graph
client_panorama.dll!0x00cf0b20 ConCommand +grenade1
client_panorama.dll!0x00cf0b68 ConCommand +grenade2
client_panorama.dll!0x00cf0754 ConCommand +jlook
client_panorama.dll!0x00cf06a0 ConCommand +jump
client_panorama.dll!0x00cf070c ConCommand +klook
client_panorama.dll!0x00cf02b0 ConCommand +left
client_panorama.dll!0x00cf0418 ConCommand +lookdown
client_panorama.dll!0x00cf0b8c ConCommand +lookspin
client_panorama.dll!0x00cf03d0 ConCommand +lookup
client_panorama.dll!0x00cf0268 ConCommand +movedown
client_panorama.dll!0x00cf04a8 ConCommand +moveleft
client_panorama.dll!0x00cf04f0 ConCommand +moveright
client_panorama.dll!0x00cf0220 ConCommand +moveup
client_panorama.dll!0x00d07b78 ConCommand +quickinv
client_panorama.dll!0x00cf07e4 ConCommand +reload
client_panorama.dll!0x00cf02f8 ConCommand +right
client_panorama.dll!0x00cf08bc ConCommand +score
client_panorama.dll!0x00cf0904 ConCommand +showscores
client_panorama.dll!0x00cf0538 ConCommand +speed
client_panorama.dll!0x00d0873c ConCommand +spray_menu
client_panorama.dll!0x00cf0460 ConCommand +strafe
client_panorama.dll!0x00cf0658 ConCommand +use
client_panorama.dll!0x00cf0580 ConCommand +walk
client_panorama.dll!0x00cf0a24 ConCommand +zoom
client_panorama.dll!0x00cf0a6c ConCommand +zoom_in
client_panorama.dll!0x00cf0ab4 ConCommand +zoom_out
client_panorama.dll!0x00cf0850 ConCommand -alt1
client_panorama.dll!0x00cf0898 ConCommand -alt2
client_panorama.dll!0x00cf05ec ConCommand -attack
client_panorama.dll!0x00cf0634 ConCommand -attack2
client_panorama.dll!0x00cf03ac ConCommand -back
client_panorama.dll!0x00cf09b8 ConCommand -break
client_panorama.dll!0x00cee068 ConCommand -camdistance
client_panorama.dll!0x00cedeb8 ConCommand -camin
client_panorama.dll!0x00cee020 ConCommand -cammousemove
client_panorama.dll!0x00cedf00 ConCommand -camout
client_panorama.dll!0x00cedde0 ConCommand -campitchdown
client_panorama.dll!0x00cedd98 ConCommand -campitchup
client_panorama.dll!0x00cede28 ConCommand -camyawleft
client_panorama.dll!0x00cede70 ConCommand -camyawright
client_panorama.dll!0x00d086fc ConCommand -cl_show_team_equipment
client_panorama.dll!0x00cf01fc ConCommand -commandermousemove
client_panorama.dll!0x00ce66a8 ConCommand -csm_rot_x_neg
client_panorama.dll!0x00ce6660 ConCommand -csm_rot_x_plus
client_panorama.dll!0x00ce6738 ConCommand -csm_rot_y_neg
client_panorama.dll!0x00ce66f0 ConCommand -csm_rot_y_plus
client_panorama.dll!0x00cf07c0 ConCommand -duck
client_panorama.dll!0x00cf0364 ConCommand -forward
client_panorama.dll!0x00cf0970 ConCommand -graph
client_panorama.dll!0x00cf0afc ConCommand -grenade1
client_panorama.dll!0x00cf0b44 ConCommand -grenade2
client_panorama.dll!0x00cf0778 ConCommand -jlook
client_panorama.dll!0x00cf06c4 ConCommand -jump
client_panorama.dll!0x00cf0730 ConCommand -klook
client_panorama.dll!0x00cf02d4 ConCommand -left
client_panorama.dll!0x00cf043c ConCommand -lookdown
client_panorama.dll!0x00cf0bb0 ConCommand -lookspin
client_panorama.dll!0x00cf03f4 ConCommand -lookup
client_panorama.dll!0x00cf028c ConCommand -movedown
client_panorama.dll!0x00cf04cc ConCommand -moveleft
client_panorama.dll!0x00cf0514 ConCommand -moveright
client_panorama.dll!0x00cf0244 ConCommand -moveup
client_panorama.dll!0x00d07b9c ConCommand -quickinv
client_panorama.dll!0x00cf0808 ConCommand -reload
client_panorama.dll!0x00cf031c ConCommand -right
client_panorama.dll!0x00cf08e0 ConCommand -score
client_panorama.dll!0x00cf0928 ConCommand -showscores
client_panorama.dll!0x00cf055c ConCommand -speed
client_panorama.dll!0x00d08760 ConCommand -spray_menu
client_panorama.dll!0x00cf0484 ConCommand -strafe
client_panorama.dll!0x00cf067c ConCommand -use
client_panorama.dll!0x00cf05a4 ConCommand -walk
client_panorama.dll!0x00cf0a48 ConCommand -zoom
client_panorama.dll!0x00cf0a90 ConCommand -zoom_in
client_panorama.dll!0x00cf0ad8 ConCommand -zoom_out
client_panorama.dll!0x00ce6df0 ConCommand CreateHairball
client_panorama.dll!0x00d066a8 ConCommand ShowSteamStatsSessionID
client_panorama.dll!0x00ce8334 ConCommand Test_ProxyToggle_EnsureValue
client_panorama.dll!0x00cfc7f0 ConCommand autobuy
client_panorama.dll!0x00d06cdc ConCommand bench_showstatsdialog
client_panorama.dll!0x00d0d2e4 ConCommand buy_stamps
client_panorama.dll!0x00d0a268 ConCommand buymenu
client_panorama.dll!0x00cedd50 ConCommand cam_command
client_panorama.dll!0x00cedfd8 ConCommand camortho
client_panorama.dll!0x00cfacf0 ConCommand cancelselect
client_panorama.dll!0x00ceca78 ConCommand cc_emit
client_panorama.dll!0x00cecb60 ConCommand cc_findsound
client_panorama.dll!0x00cecac0 ConCommand cc_flush
client_panorama.dll!0x00ceca9c ConCommand cc_random
client_panorama.dll!0x00cecae4 ConCommand cc_showblocks
client_panorama.dll!0x00cf6770 ConCommand centerview
client_panorama.dll!0x00cec678 ConCommand cl_animationinfo
client_panorama.dll!0x00d07008 ConCommand cl_avatar_convert_rgb
client_panorama.dll!0x00cec3b0 ConCommand cl_clearhinthistory
client_panorama.dll!0x00d0cbf4 ConCommand cl_cs_dump_econ_item_stringtable
client_panorama.dll!0x00ce65c0 ConCommand cl_csm_status
client_panorama.dll!0x00cfcea8 ConCommand cl_dev_decaltrace_blood
client_panorama.dll!0x00cf3210 ConCommand cl_dump_particle_stats
client_panorama.dll!0x00cecce8 ConCommand cl_dumpplayer
client_panorama.dll!0x00ce8824 ConCommand cl_dumpsplithacks
client_panorama.dll!0x00ce48b4 ConCommand cl_ent_absbox
client_panorama.dll!0x00ce4890 ConCommand cl_ent_bbox
client_panorama.dll!0x00ce48d8 ConCommand cl_ent_rbox
client_panorama.dll!0x00ce48fc ConCommand cl_find_ent
client_panorama.dll!0x00ce4920 ConCommand cl_find_ent_index
client_panorama.dll!0x00d06474 ConCommand cl_game_mode_convars
client_panorama.dll!0x00d09540 ConCommand cl_mainmenu_blog_file
client_panorama.dll!0x00d08c70 ConCommand cl_mainmenu_hide_blog
client_panorama.dll!0x00d08c4c ConCommand cl_mainmenu_show_blog
client_panorama.dll!0x00d09308 ConCommand cl_matchstats_print_own_data
client_panorama.dll!0x00ce88e8 ConCommand cl_modemanager_reload
client_panorama.dll!0x00cf5cb8 ConCommand cl_panelanimation
client_panorama.dll!0x00cf3138 ConCommand cl_particles_dump_effects
client_panorama.dll!0x00cf3234 ConCommand cl_particles_dumplist
client_panorama.dll!0x00d04dd0 ConCommand cl_player_rank_events_spew
client_panorama.dll!0x00cf4114 ConCommand cl_pred_track
client_panorama.dll!0x00cf40f0 ConCommand cl_predictioncopy_describe
client_panorama.dll!0x00d0d040 ConCommand cl_quest_events_print
client_panorama.dll!0x00d0d01c ConCommand cl_quest_schedule_print
client_panorama.dll!0x00d08698 ConCommand cl_reload_hud
client_panorama.dll!0x00cfd934 ConCommand cl_reloadpostprocessparams
client_panorama.dll!0x00d05eac ConCommand cl_remove_all_workshop_maps
client_panorama.dll!0x00ce486c ConCommand cl_removedecals
client_panorama.dll!0x00cf4ca8 ConCommand cl_report_soundpatch
client_panorama.dll!0x00cfd97c ConCommand cl_sos_test_get_opvar
client_panorama.dll!0x00cfd958 ConCommand cl_sos_test_set_opvar
client_panorama.dll!0x00cf4bc0 ConCommand cl_soundemitter_flush
client_panorama.dll!0x00cf4b9c ConCommand cl_soundemitter_reload
client_panorama.dll!0x00ce8078 ConCommand cl_soundscape_flush
client_panorama.dll!0x00ce80e4 ConCommand cl_soundscape_printdebuginfo
client_panorama.dll!0x00ce8108 ConCommand cl_ss_origin
client_panorama.dll!0x00cfd9c8 ConCommand cl_steamscreenshots
client_panorama.dll!0x00cea9cc ConCommand cl_tree_sway_dir
client_panorama.dll!0x00ce46c4 ConCommand cl_updatevisibility
client_panorama.dll!0x00cea290 ConCommand commentary_showmodelviewer
client_panorama.dll!0x00d06d58 ConCommand commentary_testfirstrun
client_panorama.dll!0x00d06878 ConCommand condump
client_panorama.dll!0x00d08cbc ConCommand confirm_abandon_match
client_panorama.dll!0x00d09ec8 ConCommand confirm_activate_itemid_now
client_panorama.dll!0x00d08e78 ConCommand confirm_join_friend_session_exit_current
client_panorama.dll!0x00d06a8c ConCommand confirm_join_new_session_exit_current
client_panorama.dll!0x00d096a4 ConCommand confirm_join_party_session_exit_current
client_panorama.dll!0x00d09f10 ConCommand confirm_purchase_item_def_now
client_panorama.dll!0x00d08e9c ConCommand confirm_watch_friend_session_exit_current
client_panorama.dll!0x00d091fc ConCommand csgo_download_match
client_panorama.dll!0x00d06de8 ConCommand csgo_econ_action_preview
client_panorama.dll!0x00d09f34 ConCommand debug_purchase_defidx
client_panorama.dll!0x00ce4848 ConCommand dlight_debug
client_panorama.dll!0x00cfc838 ConCommand dm_togglerandomweapons
client_panorama.dll!0x00d07b0c ConCommand drawoverviewmap
client_panorama.dll!0x00d08650 ConCommand drawradar
client_panorama.dll!0x00d0cd68 ConCommand dump_particlemanifest
client_panorama.dll!0x00d0d138 ConCommand econ_build_pinboard_images_from_collection_name
client_panorama.dll!0x00d0d064 ConCommand econ_show_items_with_tag
client_panorama.dll!0x00cfc85c ConCommand endmatch_votenextmap
client_panorama.dll!0x00cfdb1c ConCommand error_message_explain_pure
client_panorama.dll!0x00cfdaf8 ConCommand error_message_explain_vac
client_panorama.dll!0x00cedf6c ConCommand firstperson
client_panorama.dll!0x00cf09dc ConCommand force_centerview
client_panorama.dll!0x00cecd30 ConCommand g15_dumpplayer
client_panorama.dll!0x00cecd0c ConCommand g15_reload
client_panorama.dll!0x00ce6cc0 ConCommand gameinstructor_dump_open_lessons
client_panorama.dll!0x00ce6c78 ConCommand gameinstructor_reload_lessons
client_panorama.dll!0x00ce6c9c ConCommand gameinstructor_reset_counts
client_panorama.dll!0x00d06f34 ConCommand gamemenucommand
client_panorama.dll!0x00cfadec ConCommand gamepadslot1
client_panorama.dll!0x00cfae10 ConCommand gamepadslot2
client_panorama.dll!0x00cfae34 ConCommand gamepadslot3
client_panorama.dll!0x00cfae58 ConCommand gamepadslot4
client_panorama.dll!0x00cfae7c ConCommand gamepadslot5
client_panorama.dll!0x00cfaea0 ConCommand gamepadslot6
client_panorama.dll!0x00d09564 ConCommand gcmd
client_panorama.dll!0x00cf6814 ConCommand getpos
client_panorama.dll!0x00cf6838 ConCommand getpos_exact
client_panorama.dll!0x00d07b30 ConCommand hideoverviewmap
client_panorama.dll!0x00d0d474 ConCommand hidepanel
client_panorama.dll!0x00d08674 ConCommand hideradar
client_panorama.dll!0x00d0a2f8 ConCommand hidescores
client_panorama.dll!0x00cf42b8 ConCommand highlight_sticker
client_panorama.dll!0x00ce9464 ConCommand hud_reloadscheme
client_panorama.dll!0x00ced608 ConCommand hud_subtitles
client_panorama.dll!0x00cf06e8 ConCommand impulse
client_panorama.dll!0x00cfad14 ConCommand invnext
client_panorama.dll!0x00cfad80 ConCommand invnextgrenade
client_panorama.dll!0x00cfada4 ConCommand invnextitem
client_panorama.dll!0x00cfadc8 ConCommand invnextnongrenade
client_panorama.dll!0x00cfad38 ConCommand invprev
client_panorama.dll!0x00cf0a00 ConCommand joyadvancedupdate
client_panorama.dll!0x00cfad5c ConCommand lastinv
client_panorama.dll!0x00d090e4 ConCommand launch_warmup_map
client_panorama.dll!0x00cec550 ConCommand list_active_casters
client_panorama.dll!0x00d06da0 ConCommand loadcommentary
client_panorama.dll!0x00d09098 ConCommand localization_quest_item_string_printout
client_panorama.dll!0x00cfc930 ConCommand mat_reloadwearablecustommaterials
client_panorama.dll!0x00d09118 ConCommand matchdraft_debug_sendlog
client_panorama.dll!0x00d07698 ConCommand menuselect
client_panorama.dll!0x00d0913c ConCommand mm_queue_draft_show
client_panorama.dll!0x00d04c10 ConCommand mm_queue_show_stats
client_panorama.dll!0x00d06dc4 ConCommand modelpanel_set_sticker
client_panorama.dll!0x00cf42dc ConCommand peel_sticker
client_panorama.dll!0x00d0d2c0 ConCommand perfectworld_replenish_funds
client_panorama.dll!0x00cf32fc ConCommand perfvisualbenchmark
client_panorama.dll!0x00cf3320 ConCommand perfvisualbenchmark_abort
client_panorama.dll!0x00cfe4f0 ConCommand pick_hint
client_panorama.dll!0x00ce7220 ConCommand pixelvis_debug
client_panorama.dll!0x00cf4be4 ConCommand playgamesound
client_panorama.dll!0x00ce809c ConCommand playsoundscape
client_panorama.dll!0x00cf6230 ConCommand playvideo
client_panorama.dll!0x00cf6278 ConCommand playvideo_end_level_transition
client_panorama.dll!0x00cf629c ConCommand playvideo_exitcommand
client_panorama.dll!0x00cf62c0 ConCommand playvideo_exitcommand_nointerrupt
client_panorama.dll!0x00cf6254 ConCommand playvideo_nointerrupt
client_panorama.dll!0x00d04df4 ConCommand print_achievement_categories
client_panorama.dll!0x00cfeba0 ConCommand print_mapgroup
client_panorama.dll!0x00d06f10 ConCommand quit_prompt
client_panorama.dll!0x00cfa138 ConCommand r_cheapwaterend
client_panorama.dll!0x00cfa114 ConCommand r_cheapwaterstart
client_panorama.dll!0x00d06058 ConCommand r_ropes_holiday_light_color
client_panorama.dll!0x00cf7418 ConCommand r_screenoverlay
client_panorama.dll!0x00ce9b8c ConCommand r_shadowangles
client_panorama.dll!0x00ce9bf8 ConCommand r_shadowblobbycutoff
client_panorama.dll!0x00ce9bb0 ConCommand r_shadowcolor
client_panorama.dll!0x00ce9b68 ConCommand r_shadowdir
client_panorama.dll!0x00ce9bd4 ConCommand r_shadowdist
client_panorama.dll!0x00d07608 ConCommand radio
client_panorama.dll!0x00d0762c ConCommand radio1
client_panorama.dll!0x00d07650 ConCommand radio2
client_panorama.dll!0x00d07674 ConCommand radio3
client_panorama.dll!0x00cfc9b0 ConCommand rangefinder
client_panorama.dll!0x00cfc814 ConCommand rebuy
client_panorama.dll!0x00d09eec ConCommand reload_store_config
client_panorama.dll!0x00cfa9d8 ConCommand script_client
client_panorama.dll!0x00cfaa20 ConCommand script_debug_client
client_panorama.dll!0x00cfaa68 ConCommand script_dump_all_client
client_panorama.dll!0x00cfa9fc ConCommand script_execute_client
client_panorama.dll!0x00cfaa44 ConCommand script_help_client
client_panorama.dll!0x00cf6c10 ConCommand shake_stop
client_panorama.dll!0x00cf6c34 ConCommand shake_testpunch
client_panorama.dll!0x00cfd4e8 ConCommand show_loadout_toggle
client_panorama.dll!0x00d0d548 ConCommand showinfo
client_panorama.dll!0x00d0d450 ConCommand showpanel
client_panorama.dll!0x00cfac3c ConCommand slot0
client_panorama.dll!0x00cfaaf8 ConCommand slot1
client_panorama.dll!0x00cfac60 ConCommand slot10
client_panorama.dll!0x00cfac84 ConCommand slot11
client_panorama.dll!0x00cfaca8 ConCommand slot12
client_panorama.dll!0x00cfaccc ConCommand slot13
client_panorama.dll!0x00cfab1c ConCommand slot2
client_panorama.dll!0x00cfab40 ConCommand slot3
client_panorama.dll!0x00cfab64 ConCommand slot4
client_panorama.dll!0x00cfab88 ConCommand slot5
client_panorama.dll!0x00cfabac ConCommand slot6
client_panorama.dll!0x00cfabd0 ConCommand slot7
client_panorama.dll!0x00cfabf4 ConCommand slot8
client_panorama.dll!0x00cfac18 ConCommand slot9
client_panorama.dll!0x00cee08c ConCommand snapto
client_panorama.dll!0x00cf4c08 ConCommand snd_playsounds
client_panorama.dll!0x00cf4c2c ConCommand snd_setsoundparam
client_panorama.dll!0x00ce524c ConCommand soundscape_dumpclient
client_panorama.dll!0x00ce9320 ConCommand spec_cameraman_set_xray
client_panorama.dll!0x00ce941c ConCommand spec_goto
client_panorama.dll!0x00d0a2b0 ConCommand spec_gui
client_panorama.dll!0x00ce9440 ConCommand spec_lerpto
client_panorama.dll!0x00d0a28c ConCommand spec_menu
client_panorama.dll!0x00ce938c ConCommand spec_mode
client_panorama.dll!0x00ce9344 ConCommand spec_next
client_panorama.dll!0x00ce93b0 ConCommand spec_player
client_panorama.dll!0x00ce93f8 ConCommand spec_player_by_accountid
client_panorama.dll!0x00ce93d4 ConCommand spec_player_by_name
client_panorama.dll!0x00cf67f0 ConCommand spec_pos
client_panorama.dll!0x00ce9368 ConCommand spec_prev
client_panorama.dll!0x00cf5c38 ConCommand ss_reloadletterbox
client_panorama.dll!0x00cf0100 ConCommand ss_teleport
client_panorama.dll!0x00cf632c ConCommand stop_transition_videos_fadeout
client_panorama.dll!0x00ce80c0 ConCommand stopsoundscape
client_panorama.dll!0x00cf62e4 ConCommand stopvideos
client_panorama.dll!0x00cf6308 ConCommand stopvideos_fadeout
client_panorama.dll!0x00d0a244 ConCommand teammenu
client_panorama.dll!0x00cf8cd8 ConCommand test_freezeframe
client_panorama.dll!0x00cec5d0 ConCommand testhudanim
client_panorama.dll!0x00cedf24 ConCommand thirdperson
client_panorama.dll!0x00cedf48 ConCommand thirdperson_mayamode
client_panorama.dll!0x00cedfb4 ConCommand thirdpersonoverview
client_panorama.dll!0x00cedf90 ConCommand thirdpersonshoulder
client_panorama.dll!0x00cfd9a0 ConCommand toggleRdrOpt
client_panorama.dll!0x00cfa0f0 ConCommand toggleThreadedBuildRWList
client_panorama.dll!0x00cf0bd4 ConCommand toggle_duck
client_panorama.dll!0x00d0a2d4 ConCommand togglescores
client_panorama.dll!0x00d06b84 ConCommand ui_reloadscheme
client_panorama.dll!0x00cf6d7c ConCommand viewanim_addkeyframe
client_panorama.dll!0x00cf6d10 ConCommand viewanim_create
client_panorama.dll!0x00cf6dc4 ConCommand viewanim_load
client_panorama.dll!0x00cf6d58 ConCommand viewanim_reset
client_panorama.dll!0x00cf6da0 ConCommand viewanim_save
client_panorama.dll!0x00cf6d34 ConCommand viewanim_test
client_panorama.dll!0x00d08800 ConCommand voice_status_test_toggle
client_panorama.dll!0x00d06d7c ConCommand workshop_publish
client_panorama.dll!0x00d06e3c ConCommand workshop_workbench
client_panorama.dll!0x00cf0c1c ConCommand xlook
client_panorama.dll!0x00cf0bf8 ConCommand xmove
```

### Buttons

```
client_panorama.dll!0x051a0b18 kbutton_t +alt1
client_panorama.dll!0x051a0b24 kbutton_t +alt2
client_panorama.dll!0x0312ec44 kbutton_t +attack
client_panorama.dll!0x0312ec68 kbutton_t +attack2
client_panorama.dll!0x0312ec98 kbutton_t +back
client_panorama.dll!0x051a0b3c kbutton_t +break
client_panorama.dll!0x051a0a78 kbutton_t +camin
client_panorama.dll!0x051a0a84 kbutton_t +camout
client_panorama.dll!0x051a0a54 kbutton_t +campitchdown
client_panorama.dll!0x051a0a48 kbutton_t +campitchup
client_panorama.dll!0x051a0a60 kbutton_t +camyawleft
client_panorama.dll!0x051a0a6c kbutton_t +camyawright
client_panorama.dll!0x0312ec80 kbutton_t +commandermousemove
client_panorama.dll!0x051a0b00 kbutton_t +duck
client_panorama.dll!0x0312eca4 kbutton_t +forward
client_panorama.dll!0x0312ec38 kbutton_t +graph
client_panorama.dll!0x051a0b48 kbutton_t +grenade1
client_panorama.dll!0x051a0b54 kbutton_t +grenade2
client_panorama.dll!0x0312ecc0 kbutton_t +jlook
client_panorama.dll!0x051a0adc kbutton_t +jump
client_panorama.dll!0x051a0a94 kbutton_t +klook
client_panorama.dll!0x051a0aa0 kbutton_t +left
client_panorama.dll!0x051a0ac4 kbutton_t +lookdown
client_panorama.dll!0x0312ecb4 kbutton_t +lookspin
client_panorama.dll!0x051a0ab8 kbutton_t +lookup
client_panorama.dll!0x051a0af4 kbutton_t +movedown
client_panorama.dll!0x0312ec20 kbutton_t +moveleft
client_panorama.dll!0x0312ec14 kbutton_t +moveright
client_panorama.dll!0x051a0ae8 kbutton_t +moveup
client_panorama.dll!0x051a0b0c kbutton_t +reload
client_panorama.dll!0x051a0aac kbutton_t +right
client_panorama.dll!0x051a0b30 kbutton_t +score
client_panorama.dll!0x051a0b30 kbutton_t +showscores
client_panorama.dll!0x0312ec74 kbutton_t +speed
client_panorama.dll!0x0312eccc kbutton_t +strafe
client_panorama.dll!0x051a0ad0 kbutton_t +use
client_panorama.dll!0x0312ec8c kbutton_t +walk
client_panorama.dll!0x0312ec5c kbutton_t +zoom
client_panorama.dll!0x051a0b48 kbutton_t +zoom_in
client_panorama.dll!0x051a0b54 kbutton_t +zoom_out
```

### ClientClasses

<details>
<summary><code>client_class CAI_BaseNPC</code></summary>

sizeof: `12080`  
</details>
<details>
<summary><code>client_class CAK47</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CBRC4Target</code></summary>

sizeof: `10640`  
</details>
<details>
<summary><code>client_class CBaseAnimating</code></summary>

sizeof: `10624`  
</details>
<details>
<summary><code>client_class CBaseAnimatingOverlay</code></summary>

sizeof: `10736`  
</details>
<details>
<summary><code>client_class CBaseAttributableItem</code></summary>

sizeof: `12816`  
</details>
<details>
<summary><code>client_class CBaseButton</code></summary>

sizeof: `2576`  
</details>
<details>
<summary><code>client_class CBaseCSGrenade</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CBaseCSGrenadeProjectile</code></summary>

sizeof: `10720`  
</details>
<details>
<summary><code>client_class CBaseCombatCharacter</code></summary>

sizeof: `12048`  
</details>
<details>
<summary><code>client_class CBaseCombatWeapon</code></summary>

sizeof: `13008`  
</details>
<details>
<summary><code>client_class CBaseDoor</code></summary>

sizeof: `2576`  
</details>
<details>
<summary><code>client_class CBaseEntity</code></summary>

sizeof: `2520`  
</details>
<details>
<summary><code>client_class CBaseFlex</code></summary>

sizeof: `11632`  
</details>
<details>
<summary><code>client_class CBaseGrenade</code></summary>

sizeof: `10672`  
</details>
<details>
<summary><code>client_class CBaseParticleEntity</code></summary>

sizeof: `2752`  
</details>
<details>
<summary><code>client_class CBasePlayer</code></summary>

sizeof: `14448`  
</details>
<details>
<summary><code>client_class CBasePropDoor</code></summary>

sizeof: `10736`  
</details>
<details>
<summary><code>client_class CBaseTeamObjectiveResource</code></summary>

sizeof: `7440`  
</details>
<details>
<summary><code>client_class CBaseTempEntity</code></summary>

sizeof: `4236510919`  
</details>
<details>
<summary><code>client_class CBaseToggle</code></summary>

sizeof: `2568`  
</details>
<details>
<summary><code>client_class CBaseTrigger</code></summary>

sizeof: `2576`  
</details>
<details>
<summary><code>client_class CBaseVPhysicsTrigger</code></summary>

sizeof: `2528`  
</details>
<details>
<summary><code>client_class CBaseViewModel</code></summary>

sizeof: `10896`  
</details>
<details>
<summary><code>client_class CBaseWeaponWorldModel</code></summary>

sizeof: `10768`  
</details>
<details>
<summary><code>client_class CBeam</code></summary>

sizeof: `2688`  
</details>
<details>
<summary><code>client_class CBeamSpotlight</code></summary>

sizeof: `2608`  
</details>
<details>
<summary><code>client_class CBoneFollower</code></summary>

sizeof: `2528`  
</details>
<details>
<summary><code>client_class CBreachCharge</code></summary>

sizeof: `13248`  
</details>
<details>
<summary><code>client_class CBreachChargeProjectile</code></summary>

sizeof: `10704`  
</details>
<details>
<summary><code>client_class CBreakableProp</code></summary>

sizeof: `10656`  
</details>
<details>
<summary><code>client_class CBreakableSurface</code></summary>

sizeof: `3592`  
</details>
<details>
<summary><code>client_class CBumpMine</code></summary>

sizeof: `13232`  
</details>
<details>
<summary><code>client_class CBumpMineProjectile</code></summary>

sizeof: `10720`  
</details>
<details>
<summary><code>client_class CC4</code></summary>

sizeof: `13296`  
</details>
<details>
<summary><code>client_class CCSGameRulesProxy</code></summary>

sizeof: `2520`  
</details>
<details>
<summary><code>client_class CCSPlayer</code></summary>

sizeof: `48176`  
</details>
<details>
<summary><code>client_class CCSPlayerResource</code></summary>

sizeof: `24936`  
</details>
<details>
<summary><code>client_class CCSRagdoll</code></summary>

sizeof: `10832`  
</details>
<details>
<summary><code>client_class CCSTeam</code></summary>

sizeof: `2936`  
</details>
<details>
<summary><code>client_class CCascadeLight</code></summary>

sizeof: `2560`  
</details>
<details>
<summary><code>client_class CChicken</code></summary>

sizeof: `10736`  
</details>
<details>
<summary><code>client_class CColorCorrection</code></summary>

sizeof: `2848`  
</details>
<details>
<summary><code>client_class CColorCorrectionVolume</code></summary>

sizeof: `2872`  
</details>
<details>
<summary><code>client_class CDEagle</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CDangerZone</code></summary>

sizeof: `2560`  
</details>
<details>
<summary><code>client_class CDangerZoneController</code></summary>

sizeof: `2752`  
</details>
<details>
<summary><code>client_class CDecoyGrenade</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CDecoyProjectile</code></summary>

sizeof: `10736`  
</details>
<details>
<summary><code>client_class CDrone</code></summary>

sizeof: `11216`  
</details>
<details>
<summary><code>client_class CDronegun</code></summary>

sizeof: `10720`  
</details>
<details>
<summary><code>client_class CDynamicLight</code></summary>

sizeof: `2552`  
</details>
<details>
<summary><code>client_class CDynamicProp</code></summary>

sizeof: `10720`  
</details>
<details>
<summary><code>client_class CEconEntity</code></summary>

sizeof: `12816`  
</details>
<details>
<summary><code>client_class CEconWearable</code></summary>

sizeof: `12832`  
</details>
<details>
<summary><code>client_class CEmbers</code></summary>

sizeof: `2552`  
</details>
<details>
<summary><code>client_class CEntityDissolve</code></summary>

sizeof: `2592`  
</details>
<details>
<summary><code>client_class CEntityFlame</code></summary>

sizeof: `2544`  
</details>
<details>
<summary><code>client_class CEntityFreezing</code></summary>

sizeof: `2760`  
</details>
<details>
<summary><code>client_class CEntityParticleTrail</code></summary>

sizeof: `2792`  
</details>
<details>
<summary><code>client_class CEnvAmbientLight</code></summary>

sizeof: `2872`  
</details>
<details>
<summary><code>client_class CEnvDOFController</code></summary>

sizeof: `2552`  
</details>
<details>
<summary><code>client_class CEnvDetailController</code></summary>

sizeof: `2528`  
</details>
<details>
<summary><code>client_class CEnvGasCanister</code></summary>

sizeof: `10816`  
</details>
<details>
<summary><code>client_class CEnvParticleScript</code></summary>

sizeof: `10864`  
</details>
<details>
<summary><code>client_class CEnvProjectedTexture</code></summary>

sizeof: `2904`  
</details>
<details>
<summary><code>client_class CEnvQuadraticBeam</code></summary>

sizeof: `2552`  
</details>
<details>
<summary><code>client_class CEnvScreenEffect</code></summary>

sizeof: `2528`  
</details>
<details>
<summary><code>client_class CEnvScreenOverlay</code></summary>

sizeof: `5136`  
</details>
<details>
<summary><code>client_class CEnvTonemapController</code></summary>

sizeof: `2568`  
</details>
<details>
<summary><code>client_class CEnvWind</code></summary>

sizeof: `3048`  
</details>
<details>
<summary><code>client_class CFEPlayerDecal</code></summary>

sizeof: `2736`  
</details>
<details>
<summary><code>client_class CFireCrackerBlast</code></summary>

sizeof: `10688`  
</details>
<details>
<summary><code>client_class CFireSmoke</code></summary>

sizeof: `23696`  
</details>
<details>
<summary><code>client_class CFireTrail</code></summary>

sizeof: `2832`  
</details>
<details>
<summary><code>client_class CFish</code></summary>

sizeof: `10832`  
</details>
<details>
<summary><code>client_class CFists</code></summary>

sizeof: `13248`  
</details>
<details>
<summary><code>client_class CFlashbang</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CFogController</code></summary>

sizeof: `2600`  
</details>
<details>
<summary><code>client_class CFootstepControl</code></summary>

sizeof: `2608`  
</details>
<details>
<summary><code>client_class CFuncAreaPortalWindow</code></summary>

sizeof: `2536`  
</details>
<details>
<summary><code>client_class CFuncBrush</code></summary>

sizeof: `2520`  
</details>
<details>
<summary><code>client_class CFuncConveyor</code></summary>

sizeof: `2528`  
</details>
<details>
<summary><code>client_class CFuncLadder</code></summary>

sizeof: `2584`  
</details>
<details>
<summary><code>client_class CFuncMonitor</code></summary>

sizeof: `2520`  
</details>
<details>
<summary><code>client_class CFuncMoveLinear</code></summary>

sizeof: `2568`  
</details>
<details>
<summary><code>client_class CFuncOccluder</code></summary>

sizeof: `2528`  
</details>
<details>
<summary><code>client_class CFuncReflectiveGlass</code></summary>

sizeof: `2528`  
</details>
<details>
<summary><code>client_class CFuncRotating</code></summary>

sizeof: `2520`  
</details>
<details>
<summary><code>client_class CFuncSmokeVolume</code></summary>

sizeof: `3144`  
</details>
<details>
<summary><code>client_class CFuncTrackTrain</code></summary>

sizeof: `2536`  
</details>
<details>
<summary><code>client_class CFunc_Dust</code></summary>

sizeof: `2832`  
</details>
<details>
<summary><code>client_class CFunc_LOD</code></summary>

sizeof: `2528`  
</details>
<details>
<summary><code>client_class CGameRulesProxy</code></summary>

sizeof: `2520`  
</details>
<details>
<summary><code>client_class CGrassBurn</code></summary>

sizeof: `2552`  
</details>
<details>
<summary><code>client_class CHEGrenade</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CHandleTest</code></summary>

sizeof: `2528`  
</details>
<details>
<summary><code>client_class CHostage</code></summary>

sizeof: `12224`  
</details>
<details>
<summary><code>client_class CHostageCarriableProp</code></summary>

sizeof: `10640`  
</details>
<details>
<summary><code>client_class CIncendiaryGrenade</code></summary>

sizeof: `13280`  
</details>
<details>
<summary><code>client_class CInferno</code></summary>

sizeof: `10688`  
</details>
<details>
<summary><code>client_class CInfoLadderDismount</code></summary>

sizeof: `2520`  
</details>
<details>
<summary><code>client_class CInfoMapRegion</code></summary>

sizeof: `2656`  
</details>
<details>
<summary><code>client_class CInfoOverlayAccessor</code></summary>

sizeof: `2528`  
</details>
<details>
<summary><code>client_class CItem</code></summary>

sizeof: `13344`  
</details>
<details>
<summary><code>client_class CItemAssaultSuitUseable</code></summary>

sizeof: `13360`  
</details>
<details>
<summary><code>client_class CItemCash</code></summary>

sizeof: `13344`  
</details>
<details>
<summary><code>client_class CItemDogtags</code></summary>

sizeof: `13360`  
</details>
<details>
<summary><code>client_class CItem_Healthshot</code></summary>

sizeof: `13248`  
</details>
<details>
<summary><code>client_class CKnife</code></summary>

sizeof: `13232`  
</details>
<details>
<summary><code>client_class CKnifeGG</code></summary>

sizeof: `13232`  
</details>
<details>
<summary><code>client_class CLightGlow</code></summary>

sizeof: `2760`  
</details>
<details>
<summary><code>client_class CMaterialModifyControl</code></summary>

sizeof: `3344`  
</details>
<details>
<summary><code>client_class CMelee</code></summary>

sizeof: `13248`  
</details>
<details>
<summary><code>client_class CMolotovGrenade</code></summary>

sizeof: `13280`  
</details>
<details>
<summary><code>client_class CMolotovProjectile</code></summary>

sizeof: `10736`  
</details>
<details>
<summary><code>client_class CMovieDisplay</code></summary>

sizeof: `2800`  
</details>
<details>
<summary><code>client_class CParadropChopper</code></summary>

sizeof: `10656`  
</details>
<details>
<summary><code>client_class CParticleFire</code></summary>

sizeof: `6384`  
</details>
<details>
<summary><code>client_class CParticlePerformanceMonitor</code></summary>

sizeof: `2528`  
</details>
<details>
<summary><code>client_class CParticleSystem</code></summary>

sizeof: `3184`  
</details>
<details>
<summary><code>client_class CPhysBox</code></summary>

sizeof: `2528`  
</details>
<details>
<summary><code>client_class CPhysBoxMultiplayer</code></summary>

sizeof: `2544`  
</details>
<details>
<summary><code>client_class CPhysMagnet</code></summary>

sizeof: `10672`  
</details>
<details>
<summary><code>client_class CPhysPropAmmoBox</code></summary>

sizeof: `10736`  
</details>
<details>
<summary><code>client_class CPhysPropLootCrate</code></summary>

sizeof: `10752`  
</details>
<details>
<summary><code>client_class CPhysPropRadarJammer</code></summary>

sizeof: `10752`  
</details>
<details>
<summary><code>client_class CPhysPropWeaponUpgrade</code></summary>

sizeof: `10736`  
</details>
<details>
<summary><code>client_class CPhysicsProp</code></summary>

sizeof: `10688`  
</details>
<details>
<summary><code>client_class CPhysicsPropMultiplayer</code></summary>

sizeof: `10736`  
</details>
<details>
<summary><code>client_class CPlantedC4</code></summary>

sizeof: `10720`  
</details>
<details>
<summary><code>client_class CPlasma</code></summary>

sizeof: `21104`  
</details>
<details>
<summary><code>client_class CPlayerPing</code></summary>

sizeof: `2544`  
</details>
<details>
<summary><code>client_class CPlayerResource</code></summary>

sizeof: `5656`  
</details>
<details>
<summary><code>client_class CPointCamera</code></summary>

sizeof: `2560`  
</details>
<details>
<summary><code>client_class CPointCommentaryNode</code></summary>

sizeof: `11440`  
</details>
<details>
<summary><code>client_class CPointWorldText</code></summary>

sizeof: `2840`  
</details>
<details>
<summary><code>client_class CPoseController</code></summary>

sizeof: `2608`  
</details>
<details>
<summary><code>client_class CPostProcessController</code></summary>

sizeof: `2568`  
</details>
<details>
<summary><code>client_class CPrecipitation</code></summary>

sizeof: `2728`  
</details>
<details>
<summary><code>client_class CPrecipitationBlocker</code></summary>

sizeof: `645224`  
</details>
<details>
<summary><code>client_class CPredictedViewModel</code></summary>

sizeof: `11024`  
</details>
<details>
<summary><code>client_class CPropCounter</code></summary>

sizeof: `10640`  
</details>
<details>
<summary><code>client_class CPropDoorRotating</code></summary>

sizeof: `10736`  
</details>
<details>
<summary><code>client_class CPropJeep</code></summary>

sizeof: `10960`  
</details>
<details>
<summary><code>client_class CPropVehicleChoreoGeneric</code></summary>

sizeof: `10944`  
</details>
<details>
<summary><code>client_class CPropVehicleDriveable</code></summary>

sizeof: `10896`  
</details>
<details>
<summary><code>client_class CProp_Hallucination</code></summary>

sizeof: `10672`  
</details>
<details>
<summary><code>client_class CRagdollManager</code></summary>

sizeof: `2528`  
</details>
<details>
<summary><code>client_class CRagdollProp</code></summary>

sizeof: `11408`  
</details>
<details>
<summary><code>client_class CRagdollPropAttached</code></summary>

sizeof: `11472`  
</details>
<details>
<summary><code>client_class CRopeKeyframe</code></summary>

sizeof: `3416`  
</details>
<details>
<summary><code>client_class CSCAR17</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CSceneEntity</code></summary>

sizeof: `2600`  
</details>
<details>
<summary><code>client_class CSensorGrenade</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CSensorGrenadeProjectile</code></summary>

sizeof: `10736`  
</details>
<details>
<summary><code>client_class CShadowControl</code></summary>

sizeof: `2544`  
</details>
<details>
<summary><code>client_class CSlideshowDisplay</code></summary>

sizeof: `2856`  
</details>
<details>
<summary><code>client_class CSmokeGrenade</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CSmokeGrenadeProjectile</code></summary>

sizeof: `10736`  
</details>
<details>
<summary><code>client_class CSmokeStack</code></summary>

sizeof: `3016`  
</details>
<details>
<summary><code>client_class CSnowball</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CSnowballPile</code></summary>

sizeof: `10640`  
</details>
<details>
<summary><code>client_class CSnowballProjectile</code></summary>

sizeof: `10736`  
</details>
<details>
<summary><code>client_class CSpatialEntity</code></summary>

sizeof: `2816`  
</details>
<details>
<summary><code>client_class CSpotlightEnd</code></summary>

sizeof: `2536`  
</details>
<details>
<summary><code>client_class CSprite</code></summary>

sizeof: `2624`  
</details>
<details>
<summary><code>client_class CSpriteOriented</code></summary>

sizeof: `2624`  
</details>
<details>
<summary><code>client_class CSpriteTrail</code></summary>

sizeof: `4264`  
</details>
<details>
<summary><code>client_class CStatueProp</code></summary>

sizeof: `10736`  
</details>
<details>
<summary><code>client_class CSteamJet</code></summary>

sizeof: `2904`  
</details>
<details>
<summary><code>client_class CSun</code></summary>

sizeof: `2912`  
</details>
<details>
<summary><code>client_class CSunlightShadowControl</code></summary>

sizeof: `2848`  
</details>
<details>
<summary><code>client_class CSurvivalSpawnChopper</code></summary>

sizeof: `10624`  
</details>
<details>
<summary><code>client_class CTEArmorRicochet</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTEBSPDecal</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTEBaseBeam</code></summary>

sizeof: `4291726102`  
</details>
<details>
<summary><code>client_class CTEBeamEntPoint</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTEBeamEnts</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTEBeamFollow</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTEBeamLaser</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTEBeamPoints</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTEBeamRing</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTEBeamRingPoint</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTEBeamSpline</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTEBloodSprite</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTEBloodStream</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTEBreakModel</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTEBubbleTrail</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTEBubbles</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTEClientProjectile</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTEDecal</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTEDust</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTEDynamicLight</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTEEffectDispatch</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTEEnergySplash</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTEExplosion</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTEFireBullets</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTEFizz</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTEFootprintDecal</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTEFoundryHelpers</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTEGaussExplosion</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTEGlowSprite</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTEImpact</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTEKillPlayerAttachments</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTELargeFunnel</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTEMetalSparks</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTEMuzzleFlash</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTEParticleSystem</code></summary>

sizeof: `3779593927`  
</details>
<details>
<summary><code>client_class CTEPhysicsProp</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTEPlantBomb</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTEPlayerAnimEvent</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTEPlayerDecal</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTEProjectedDecal</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTERadioIcon</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTEShatterSurface</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTEShowLine</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTESmoke</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTESparks</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTESprite</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTESpriteSpray</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTEWorldDecal</code></summary>

sizeof: `0`  
</details>
<details>
<summary><code>client_class CTablet</code></summary>

sizeof: `15280`  
</details>
<details>
<summary><code>client_class CTeam</code></summary>

sizeof: `2936`  
</details>
<details>
<summary><code>client_class CTeamplayRoundBasedRulesProxy</code></summary>

sizeof: `2520`  
</details>
<details>
<summary><code>client_class CTesla</code></summary>

sizeof: `2880`  
</details>
<details>
<summary><code>client_class CTestTraceline</code></summary>

sizeof: `2528`  
</details>
<details>
<summary><code>client_class CTest_ProxyToggle_Networkable</code></summary>

sizeof: `2528`  
</details>
<details>
<summary><code>client_class CTriggerPlayerMovement</code></summary>

sizeof: `2584`  
</details>
<details>
<summary><code>client_class CTriggerSoundOperator</code></summary>

sizeof: `2584`  
</details>
<details>
<summary><code>client_class CVGuiScreen</code></summary>

sizeof: `2672`  
</details>
<details>
<summary><code>client_class CVoteController</code></summary>

sizeof: `2576`  
</details>
<details>
<summary><code>client_class CWaterBullet</code></summary>

sizeof: `10640`  
</details>
<details>
<summary><code>client_class CWaterLODControl</code></summary>

sizeof: `2528`  
</details>
<details>
<summary><code>client_class CWeaponAWP</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CWeaponAug</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CWeaponBaseItem</code></summary>

sizeof: `13248`  
</details>
<details>
<summary><code>client_class CWeaponBizon</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CWeaponCSBase</code></summary>

sizeof: `13232`  
</details>
<details>
<summary><code>client_class CWeaponCSBaseGun</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CWeaponCubemap</code></summary>

sizeof: `13008`  
</details>
<details>
<summary><code>client_class CWeaponCycler</code></summary>

sizeof: `13008`  
</details>
<details>
<summary><code>client_class CWeaponElite</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CWeaponFamas</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CWeaponFiveSeven</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CWeaponG3SG1</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CWeaponGalil</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CWeaponGalilAR</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CWeaponGlock</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CWeaponHKP2000</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CWeaponM249</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CWeaponM3</code></summary>

sizeof: `13248`  
</details>
<details>
<summary><code>client_class CWeaponM4A1</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CWeaponMAC10</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CWeaponMP5Navy</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CWeaponMP7</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CWeaponMP9</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CWeaponMag7</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CWeaponNOVA</code></summary>

sizeof: `13248`  
</details>
<details>
<summary><code>client_class CWeaponNegev</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CWeaponP228</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CWeaponP250</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CWeaponP90</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CWeaponSCAR20</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CWeaponSG550</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CWeaponSG552</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CWeaponSG556</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CWeaponSSG08</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CWeaponSawedoff</code></summary>

sizeof: `13248`  
</details>
<details>
<summary><code>client_class CWeaponScout</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CWeaponShield</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CWeaponTMP</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CWeaponTaser</code></summary>

sizeof: `13280`  
</details>
<details>
<summary><code>client_class CWeaponTec9</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CWeaponUMP45</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CWeaponUSP</code></summary>

sizeof: `13264`  
</details>
<details>
<summary><code>client_class CWeaponXM1014</code></summary>

sizeof: `13248`  
</details>
<details>
<summary><code>client_class CWorld</code></summary>

sizeof: `3435971421`  
</details>
<details>
<summary><code>client_class CWorldVguiText</code></summary>

sizeof: `3368`  
</details>
<details>
<summary><code>client_class DustTrail</code></summary>

sizeof: `2912`  
</details>
<details>
<summary><code>client_class MovieExplosion</code></summary>

sizeof: `4576`  
</details>
<details>
<summary><code>client_class NextBotCombatCharacter</code></summary>

sizeof: `12096`  
</details>
<details>
<summary><code>client_class ParticleSmokeGrenade</code></summary>

sizeof: `11800`  
</details>
<details>
<summary><code>client_class RocketTrail</code></summary>

sizeof: `2880`  
</details>
<details>
<summary><code>client_class SmokeTrail</code></summary>

sizeof: `2872`  
</details>
<details>
<summary><code>client_class SporeExplosion</code></summary>

sizeof: `2808`  
</details>
<details>
<summary><code>client_class SporeTrail</code></summary>

sizeof: `2904`  
</details>

```
client_panorama.dll!0x00ce3cf0 ClientClass CAI_BaseNPC
client_panorama.dll!0x00d0be4c ClientClass CAK47
client_panorama.dll!0x00d089e0 ClientClass CBRC4Target
client_panorama.dll!0x00ce4188 ClientClass CBaseAnimating
client_panorama.dll!0x00ce442c ClientClass CBaseAnimatingOverlay
client_panorama.dll!0x00d0cd4c ClientClass CBaseAttributableItem
client_panorama.dll!0x00ce52e8 ClientClass CBaseButton
client_panorama.dll!0x00d0a654 ClientClass CBaseCSGrenade
client_panorama.dll!0x00cfb478 ClientClass CBaseCSGrenadeProjectile
client_panorama.dll!0x00ce4448 ClientClass CBaseCombatCharacter
client_panorama.dll!0x00ce33ec ClientClass CBaseCombatWeapon
client_panorama.dll!0x00ce4464 ClientClass CBaseDoor
client_panorama.dll!0x00ce46a8 ClientClass CBaseEntity
client_panorama.dll!0x00ce4b00 ClientClass CBaseFlex
client_panorama.dll!0x00ce35c0 ClientClass CBaseGrenade
client_panorama.dll!0x00ce35dc ClientClass CBaseParticleEntity
client_panorama.dll!0x00ce5230 ClientClass CBasePlayer
client_panorama.dll!0x00ce73ec ClientClass CBasePropDoor
client_panorama.dll!0x00ce82e0 ClientClass CBaseTeamObjectiveResource
client_panorama.dll!0x00d0ebe0 ClientClass CBaseTempEntity
client_panorama.dll!0x00ce5270 ClientClass CBaseToggle
client_panorama.dll!0x00ce8358 ClientClass CBaseTrigger
client_panorama.dll!0x00ce8398 ClientClass CBaseVPhysicsTrigger
client_panorama.dll!0x00ce3c28 ClientClass CBaseViewModel
client_panorama.dll!0x00ce33d0 ClientClass CBaseWeaponWorldModel
client_panorama.dll!0x00ce3c44 ClientClass CBeam
client_panorama.dll!0x00ce5518 ClientClass CBeamSpotlight
client_panorama.dll!0x00ce4410 ClientClass CBoneFollower
client_panorama.dll!0x00d0a914 ClientClass CBreachCharge
client_panorama.dll!0x00d0a8f8 ClientClass CBreachChargeProjectile
client_panorama.dll!0x00ce5534 ClientClass CBreakableProp
client_panorama.dll!0x00ce6938 ClientClass CBreakableSurface
client_panorama.dll!0x00d0aa0c ClientClass CBumpMine
client_panorama.dll!0x00d0a9f0 ClientClass CBumpMineProjectile
client_panorama.dll!0x00d0aa90 ClientClass CC4
client_panorama.dll!0x00cfec40 ClientClass CCSGameRulesProxy
client_panorama.dll!0x00cfc660 ClientClass CCSPlayer
client_panorama.dll!0x00cfd030 ClientClass CCSPlayerResource
client_panorama.dll!0x00cfc448 ClientClass CCSRagdoll
client_panorama.dll!0x00cfd0a8 ClientClass CCSTeam
client_panorama.dll!0x00ce65e4 ClientClass CCascadeLight
client_panorama.dll!0x00d066cc ClientClass CChicken
client_panorama.dll!0x00ce5600 ClientClass CColorCorrection
client_panorama.dll!0x00ce561c ClientClass CColorCorrectionVolume
client_panorama.dll!0x00d0c1cc ClientClass CDEagle
client_panorama.dll!0x00cea5c4 ClientClass CDangerZone
client_panorama.dll!0x00cea5a8 ClientClass CDangerZoneController
client_panorama.dll!0x00d0c1e8 ClientClass CDecoyGrenade
client_panorama.dll!0x00d0a38c ClientClass CDecoyProjectile
client_panorama.dll!0x00d05ed0 ClientClass CDrone
client_panorama.dll!0x00d08be0 ClientClass CDronegun
client_panorama.dll!0x00ce5638 ClientClass CDynamicLight
client_panorama.dll!0x00ce73d0 ClientClass CDynamicProp
client_panorama.dll!0x00d0cd30 ClientClass CEconEntity
client_panorama.dll!0x00d0d1d8 ClientClass CEconWearable
client_panorama.dll!0x00d0f920 ClientClass CEmbers
client_panorama.dll!0x00ce56c0 ClientClass CEntityDissolve
client_panorama.dll!0x00ce56dc ClientClass CEntityFlame
client_panorama.dll!0x00ce5750 ClientClass CEntityFreezing
client_panorama.dll!0x00ce576c ClientClass CEntityParticleTrail
client_panorama.dll!0x00ce57e0 ClientClass CEnvAmbientLight
client_panorama.dll!0x00ce6760 ClientClass CEnvDOFController
client_panorama.dll!0x00cea9b0 ClientClass CEnvDetailController
client_panorama.dll!0x00d08bfc ClientClass CEnvGasCanister
client_panorama.dll!0x00ce6798 ClientClass CEnvParticleScript
client_panorama.dll!0x00ce67b4 ClientClass CEnvProjectedTexture
client_panorama.dll!0x00d0f93c ClientClass CEnvQuadraticBeam
client_panorama.dll!0x00ce6844 ClientClass CEnvScreenEffect
client_panorama.dll!0x00ce6828 ClientClass CEnvScreenOverlay
client_panorama.dll!0x00ce6860 ClientClass CEnvTonemapController
client_panorama.dll!0x00d0f904 ClientClass CEnvWind
client_panorama.dll!0x00d106b8 ClientClass CFEPlayerDecal
client_panorama.dll!0x00d0681c ClientClass CFireCrackerBlast
client_panorama.dll!0x00ce688c ClientClass CFireSmoke
client_panorama.dll!0x00d0fcc0 ClientClass CFireTrail
client_panorama.dll!0x00ce6900 ClientClass CFish
client_panorama.dll!0x00d0c498 ClientClass CFists
client_panorama.dll!0x00d0c4c4 ClientClass CFlashbang
client_panorama.dll!0x00ce677c ClientClass CFogController
client_panorama.dll!0x00d05398 ClientClass CFootstepControl
client_panorama.dll!0x00ce691c ClientClass CFuncAreaPortalWindow
client_panorama.dll!0x00ce6954 ClientClass CFuncBrush
client_panorama.dll!0x00ce6970 ClientClass CFuncConveyor
client_panorama.dll!0x00ceb178 ClientClass CFuncLadder
client_panorama.dll!0x00ce69e0 ClientClass CFuncMonitor
client_panorama.dll!0x00ce69fc ClientClass CFuncMoveLinear
client_panorama.dll!0x00ce6a18 ClientClass CFuncOccluder
client_panorama.dll!0x00ce6a34 ClientClass CFuncReflectiveGlass
client_panorama.dll!0x00ce6a58 ClientClass CFuncRotating
client_panorama.dll!0x00ce6ac0 ClientClass CFuncSmokeVolume
client_panorama.dll!0x00ce6adc ClientClass CFuncTrackTrain
client_panorama.dll!0x00ce698c ClientClass CFunc_Dust
client_panorama.dll!0x00ce69c4 ClientClass CFunc_LOD
client_panorama.dll!0x00cebd2c ClientClass CGameRulesProxy
client_panorama.dll!0x00cec33c ClientClass CGrassBurn
client_panorama.dll!0x00d0c4e0 ClientClass CHEGrenade
client_panorama.dll!0x00cf5380 ClientClass CHandleTest
client_panorama.dll!0x00cfb52c ClientClass CHostage
client_panorama.dll!0x00cfb510 ClientClass CHostageCarriableProp
client_panorama.dll!0x00d0c63c ClientClass CIncendiaryGrenade
client_panorama.dll!0x00d06800 ClientClass CInferno
client_panorama.dll!0x00ceb194 ClientClass CInfoLadderDismount
client_panorama.dll!0x00d089bc ClientClass CInfoMapRegion
client_panorama.dll!0x00ce6e14 ClientClass CInfoOverlayAccessor
client_panorama.dll!0x00ce6e50 ClientClass CItem
client_panorama.dll!0x00ce6e6c ClientClass CItemAssaultSuitUseable
client_panorama.dll!0x00d08c28 ClientClass CItemCash
client_panorama.dll!0x00d0a3a8 ClientClass CItemDogtags
client_panorama.dll!0x00d0a3d4 ClientClass CItem_Healthshot
client_panorama.dll!0x00d0c570 ClientClass CKnife
client_panorama.dll!0x00d0c59c ClientClass CKnifeGG
client_panorama.dll!0x00ce6e88 ClientClass CLightGlow
client_panorama.dll!0x00ce6f00 ClientClass CMaterialModifyControl
client_panorama.dll!0x00d0c5d4 ClientClass CMelee
client_panorama.dll!0x00d0c600 ClientClass CMolotovGrenade
client_panorama.dll!0x00d0a5b8 ClientClass CMolotovProjectile
client_panorama.dll!0x00ce6f1c ClientClass CMovieDisplay
client_panorama.dll!0x00d08a18 ClientClass CParadropChopper
client_panorama.dll!0x00d0fb18 ClientClass CParticleFire
client_panorama.dll!0x00ce81c0 ClientClass CParticlePerformanceMonitor
client_panorama.dll!0x00ce6f38 ClientClass CParticleSystem
client_panorama.dll!0x00ce6f8c ClientClass CPhysBox
client_panorama.dll!0x00ce7424 ClientClass CPhysBoxMultiplayer
client_panorama.dll!0x00ce7090 ClientClass CPhysMagnet
client_panorama.dll!0x00d0a338 ClientClass CPhysPropAmmoBox
client_panorama.dll!0x00d0a31c ClientClass CPhysPropLootCrate
client_panorama.dll!0x00d0a370 ClientClass CPhysPropRadarJammer
client_panorama.dll!0x00d0a354 ClientClass CPhysPropWeaponUpgrade
client_panorama.dll!0x00ce6fc4 ClientClass CPhysicsProp
client_panorama.dll!0x00ce7440 ClientClass CPhysicsPropMultiplayer
client_panorama.dll!0x00cfd0c4 ClientClass CPlantedC4
client_panorama.dll!0x00ce7244 ClientClass CPlasma
client_panorama.dll!0x00cfc570 ClientClass CPlayerPing
client_panorama.dll!0x00ce7310 ClientClass CPlayerResource
client_panorama.dll!0x00ce732c ClientClass CPointCamera
client_panorama.dll!0x00ce7350 ClientClass CPointCommentaryNode
client_panorama.dll!0x00ce737c ClientClass CPointWorldText
client_panorama.dll!0x00cf3970 ClientClass CPoseController
client_panorama.dll!0x00ce7398 ClientClass CPostProcessController
client_panorama.dll!0x00d0f0a4 ClientClass CPrecipitation
client_panorama.dll!0x00d0f088 ClientClass CPrecipitationBlocker
client_panorama.dll!0x00cf3a4c ClientClass CPredictedViewModel
client_panorama.dll!0x00cf4138 ClientClass CPropCounter
client_panorama.dll!0x00ce7408 ClientClass CPropDoorRotating
client_panorama.dll!0x00ce8500 ClientClass CPropJeep
client_panorama.dll!0x00ce83d8 ClientClass CPropVehicleChoreoGeneric
client_panorama.dll!0x00d0fb50 ClientClass CPropVehicleDriveable
client_panorama.dll!0x00ce73b4 ClientClass CProp_Hallucination
client_panorama.dll!0x00ce745c ClientClass CRagdollManager
client_panorama.dll!0x00cf43b0 ClientClass CRagdollProp
client_panorama.dll!0x00cf43cc ClientClass CRagdollPropAttached
client_panorama.dll!0x00ce7488 ClientClass CRopeKeyframe
client_panorama.dll!0x00d0c098 ClientClass CSCAR17
client_panorama.dll!0x00ce7d88 ClientClass CSceneEntity
client_panorama.dll!0x00d0c6a0 ClientClass CSensorGrenade
client_panorama.dll!0x00d0a5d4 ClientClass CSensorGrenadeProjectile
client_panorama.dll!0x00ce7e5c ClientClass CShadowControl
client_panorama.dll!0x00ce7e78 ClientClass CSlideshowDisplay
client_panorama.dll!0x00d0c918 ClientClass CSmokeGrenade
client_panorama.dll!0x00d0a5f0 ClientClass CSmokeGrenadeProjectile
client_panorama.dll!0x00d0fd08 ClientClass CSmokeStack
client_panorama.dll!0x00d0c934 ClientClass CSnowball
client_panorama.dll!0x00d0a60c ClientClass CSnowballPile
client_panorama.dll!0x00d0a638 ClientClass CSnowballProjectile
client_panorama.dll!0x00ce812c ClientClass CSpatialEntity
client_panorama.dll!0x00ce8148 ClientClass CSpotlightEnd
client_panorama.dll!0x00cf4d30 ClientClass CSprite
client_panorama.dll!0x00cf4d4c ClientClass CSpriteOriented
client_panorama.dll!0x00cf4d68 ClientClass CSpriteTrail
client_panorama.dll!0x00ce6fa8 ClientClass CStatueProp
client_panorama.dll!0x00d0fd24 ClientClass CSteamJet
client_panorama.dll!0x00ce81dc ClientClass CSun
client_panorama.dll!0x00ce82a8 ClientClass CSunlightShadowControl
client_panorama.dll!0x00d089fc ClientClass CSurvivalSpawnChopper
client_panorama.dll!0x00d0fde4 ClientClass CTEArmorRicochet
client_panorama.dll!0x00d101d4 ClientClass CTEBSPDecal
client_panorama.dll!0x00d0fe00 ClientClass CTEBaseBeam
client_panorama.dll!0x00d0fe8c ClientClass CTEBeamEntPoint
client_panorama.dll!0x00d0fefc ClientClass CTEBeamEnts
client_panorama.dll!0x00d0ff68 ClientClass CTEBeamFollow
client_panorama.dll!0x00d0ffdc ClientClass CTEBeamLaser
client_panorama.dll!0x00d1005c ClientClass CTEBeamPoints
client_panorama.dll!0x00d100cc ClientClass CTEBeamRing
client_panorama.dll!0x00d10148 ClientClass CTEBeamRingPoint
client_panorama.dll!0x00d10164 ClientClass CTEBeamSpline
client_panorama.dll!0x00d10180 ClientClass CTEBloodSprite
client_panorama.dll!0x00d1019c ClientClass CTEBloodStream
client_panorama.dll!0x00d101b8 ClientClass CTEBreakModel
client_panorama.dll!0x00d1020c ClientClass CTEBubbleTrail
client_panorama.dll!0x00d101f0 ClientClass CTEBubbles
client_panorama.dll!0x00d10228 ClientClass CTEClientProjectile
client_panorama.dll!0x00d10244 ClientClass CTEDecal
client_panorama.dll!0x00ce69a8 ClientClass CTEDust
client_panorama.dll!0x00d10260 ClientClass CTEDynamicLight
client_panorama.dll!0x00d102f8 ClientClass CTEEffectDispatch
client_panorama.dll!0x00d10330 ClientClass CTEEnergySplash
client_panorama.dll!0x00d1034c ClientClass CTEExplosion
client_panorama.dll!0x00cfd110 ClientClass CTEFireBullets
client_panorama.dll!0x00d10388 ClientClass CTEFizz
client_panorama.dll!0x00d103a4 ClientClass CTEFootprintDecal
client_panorama.dll!0x00ceb15c ClientClass CTEFoundryHelpers
client_panorama.dll!0x00d0fa44 ClientClass CTEGaussExplosion
client_panorama.dll!0x00d10444 ClientClass CTEGlowSprite
client_panorama.dll!0x00d10460 ClientClass CTEImpact
client_panorama.dll!0x00d10490 ClientClass CTEKillPlayerAttachments
client_panorama.dll!0x00d104ac ClientClass CTELargeFunnel
client_panorama.dll!0x00d0fd9c ClientClass CTEMetalSparks
client_panorama.dll!0x00d105a8 ClientClass CTEMuzzleFlash
client_panorama.dll!0x00d105c4 ClientClass CTEParticleSystem
client_panorama.dll!0x00d105e0 ClientClass CTEPhysicsProp
client_panorama.dll!0x00cfd150 ClientClass CTEPlantBomb
client_panorama.dll!0x00cfc42c ClientClass CTEPlayerAnimEvent
client_panorama.dll!0x00d1069c ClientClass CTEPlayerDecal
client_panorama.dll!0x00d10730 ClientClass CTEProjectedDecal
client_panorama.dll!0x00cfd0f4 ClientClass CTERadioIcon
client_panorama.dll!0x00d10428 ClientClass CTEShatterSurface
client_panorama.dll!0x00d1074c ClientClass CTEShowLine
client_panorama.dll!0x00d10768 ClientClass CTESmoke
client_panorama.dll!0x00d10784 ClientClass CTESparks
client_panorama.dll!0x00d107a0 ClientClass CTESprite
client_panorama.dll!0x00d107bc ClientClass CTESpriteSpray
client_panorama.dll!0x00d107d8 ClientClass CTEWorldDecal
client_panorama.dll!0x00d0ca00 ClientClass CTablet
client_panorama.dll!0x00ce82c4 ClientClass CTeam
client_panorama.dll!0x00cf4ff0 ClientClass CTeamplayRoundBasedRulesProxy
client_panorama.dll!0x00ce82fc ClientClass CTesla
client_panorama.dll!0x00d107f4 ClientClass CTestTraceline
client_panorama.dll!0x00ce8318 ClientClass CTest_ProxyToggle_Networkable
client_panorama.dll!0x00ce8374 ClientClass CTriggerPlayerMovement
client_panorama.dll!0x00ce83b4 ClientClass CTriggerSoundOperator
client_panorama.dll!0x00ce85c4 ClientClass CVGuiScreen
client_panorama.dll!0x00ce85e0 ClientClass CVoteController
client_panorama.dll!0x00ce85fc ClientClass CWaterBullet
client_panorama.dll!0x00ce8618 ClientClass CWaterLODControl
client_panorama.dll!0x00d0be84 ClientClass CWeaponAWP
client_panorama.dll!0x00d0be68 ClientClass CWeaponAug
client_panorama.dll!0x00d0a670 ClientClass CWeaponBaseItem
client_panorama.dll!0x00d0bea0 ClientClass CWeaponBizon
client_panorama.dll!0x00d0afe8 ClientClass CWeaponCSBase
client_panorama.dll!0x00d0be30 ClientClass CWeaponCSBaseGun
client_panorama.dll!0x00cfd8f4 ClientClass CWeaponCubemap
client_panorama.dll!0x00cfd8d8 ClientClass CWeaponCycler
client_panorama.dll!0x00d0c204 ClientClass CWeaponElite
client_panorama.dll!0x00d0bebc ClientClass CWeaponFamas
client_panorama.dll!0x00d0bed8 ClientClass CWeaponFiveSeven
client_panorama.dll!0x00d0bef4 ClientClass CWeaponG3SG1
client_panorama.dll!0x00d0bf10 ClientClass CWeaponGalil
client_panorama.dll!0x00d0bf2c ClientClass CWeaponGalilAR
client_panorama.dll!0x00d0bf48 ClientClass CWeaponGlock
client_panorama.dll!0x00d0bf64 ClientClass CWeaponHKP2000
client_panorama.dll!0x00d0c194 ClientClass CWeaponM249
client_panorama.dll!0x00d0c5b8 ClientClass CWeaponM3
client_panorama.dll!0x00d0bf80 ClientClass CWeaponM4A1
client_panorama.dll!0x00d0bf9c ClientClass CWeaponMAC10
client_panorama.dll!0x00d0bfd4 ClientClass CWeaponMP5Navy
client_panorama.dll!0x00d0bff0 ClientClass CWeaponMP7
client_panorama.dll!0x00d0c00c ClientClass CWeaponMP9
client_panorama.dll!0x00d0bfb8 ClientClass CWeaponMag7
client_panorama.dll!0x00d0c668 ClientClass CWeaponNOVA
client_panorama.dll!0x00d0c028 ClientClass CWeaponNegev
client_panorama.dll!0x00d0c044 ClientClass CWeaponP228
client_panorama.dll!0x00d0c060 ClientClass CWeaponP250
client_panorama.dll!0x00d0c07c ClientClass CWeaponP90
client_panorama.dll!0x00d0c0b4 ClientClass CWeaponSCAR20
client_panorama.dll!0x00d0c0ec ClientClass CWeaponSG550
client_panorama.dll!0x00d0c6bc ClientClass CWeaponSG552
client_panorama.dll!0x00d0c108 ClientClass CWeaponSG556
client_panorama.dll!0x00d0c124 ClientClass CWeaponSSG08
client_panorama.dll!0x00d0c684 ClientClass CWeaponSawedoff
client_panorama.dll!0x00d0c0d0 ClientClass CWeaponScout
client_panorama.dll!0x00d0c890 ClientClass CWeaponShield
client_panorama.dll!0x00d0c15c ClientClass CWeaponTMP
client_panorama.dll!0x00d0cb38 ClientClass CWeaponTaser
client_panorama.dll!0x00d0c140 ClientClass CWeaponTec9
client_panorama.dll!0x00d0c178 ClientClass CWeaponUMP45
client_panorama.dll!0x00d0c1b0 ClientClass CWeaponUSP
client_panorama.dll!0x00d0cb64 ClientClass CWeaponXM1014
client_panorama.dll!0x00ce8634 ClientClass CWorld
client_panorama.dll!0x00ce8660 ClientClass CWorldVguiText
client_panorama.dll!0x00d0fcdc ClientClass DustTrail
client_panorama.dll!0x00d0fa60 ClientClass MovieExplosion
client_panorama.dll!0x00d0d5c8 ClientClass NextBotCombatCharacter
client_panorama.dll!0x00d0fb34 ClientClass ParticleSmokeGrenade
client_panorama.dll!0x00d0fbf8 ClientClass RocketTrail
client_panorama.dll!0x00d0fbdc ClientClass SmokeTrail
client_panorama.dll!0x00d0fc14 ClientClass SporeExplosion
client_panorama.dll!0x00d0fca4 ClientClass SporeTrail
```

### Datamaps

<details>
<summary><code>class CBaseCSGrenade extends CWeaponCSBase</code></summary>

```
{
	m_bRedraw: Boolean,
	m_bPinPulled: Boolean,
	m_flThrowStrength: Float,
}
```

#### Offsets

```
CBaseCSGrenade!0x33b0 m_bRedraw
CBaseCSGrenade!0x33b2 m_bPinPulled
CBaseCSGrenade!0x33bc m_flThrowStrength
```
</details>
<details>
<summary><code>class CBaseCombatWeapon extends C_BaseFlex</code></summary>

```
{
	m_nNextThinkTick: Integer,
	m_hOwner: EHandle,
	m_nViewModelIndex: Integer,
	m_flNextPrimaryAttack: Float,
	m_flNextSecondaryAttack: Float,
	m_iViewModelIndex: Integer,
	m_iWorldModelIndex: Integer,
	m_iWorldDroppedModelIndex: Integer,
	m_iWeaponModule: Integer,
	m_iNumEmptyAttacks: Integer,
	m_iState: Integer,
	m_iPrimaryAmmoType: Integer,
	m_iSecondaryAmmoType: Integer,
	m_iClip1: Integer,
	m_iClip2: Integer,
	m_iPrimaryReserveAmmoCount: Integer,
	m_iSecondaryReserveAmmoCount: Integer,
	m_flTimeWeaponIdle: Float,
	m_flNextEmptySoundTime: Float,
	m_fMinRange1: Float,
	m_fMinRange2: Float,
	m_fMaxRange1: Float,
	m_fMaxRange2: Float,
	m_fFireDuration: Float,
	m_Activity: Integer,
	m_iPrimaryAmmoCount: Integer,
	m_iSecondaryAmmoCount: Integer,
	m_iszName: Integer,
	m_bRemoveable: Boolean,
	m_bInReload: Boolean,
	m_bFireOnEmpty: Boolean,
	m_bFiresUnderwater: Boolean,
	m_bAltFiresUnderwater: Boolean,
	m_bReloadsSingly: Boolean,
}
```

#### Offsets

```
CBaseCombatWeapon!0x00fc m_nNextThinkTick
CBaseCombatWeapon!0x3210 m_hOwner
CBaseCombatWeapon!0x3214 m_nViewModelIndex
CBaseCombatWeapon!0x3218 m_flNextPrimaryAttack
CBaseCombatWeapon!0x321c m_flNextSecondaryAttack
CBaseCombatWeapon!0x3220 m_iViewModelIndex
CBaseCombatWeapon!0x3224 m_iWorldModelIndex
CBaseCombatWeapon!0x3228 m_iWorldDroppedModelIndex
CBaseCombatWeapon!0x322c m_iWeaponModule
CBaseCombatWeapon!0x3230 m_iNumEmptyAttacks
CBaseCombatWeapon!0x3238 m_iState
CBaseCombatWeapon!0x323c m_iPrimaryAmmoType
CBaseCombatWeapon!0x3240 m_iSecondaryAmmoType
CBaseCombatWeapon!0x3244 m_iClip1
CBaseCombatWeapon!0x3248 m_iClip2
CBaseCombatWeapon!0x324c m_iPrimaryReserveAmmoCount
CBaseCombatWeapon!0x3250 m_iSecondaryReserveAmmoCount
CBaseCombatWeapon!0x3254 m_flTimeWeaponIdle
CBaseCombatWeapon!0x3258 m_flNextEmptySoundTime
CBaseCombatWeapon!0x325c m_fMinRange1
CBaseCombatWeapon!0x3260 m_fMinRange2
CBaseCombatWeapon!0x3264 m_fMaxRange1
CBaseCombatWeapon!0x3268 m_fMaxRange2
CBaseCombatWeapon!0x326c m_fFireDuration
CBaseCombatWeapon!0x3274 m_Activity
CBaseCombatWeapon!0x3278 m_iPrimaryAmmoCount
CBaseCombatWeapon!0x327c m_iSecondaryAmmoCount
CBaseCombatWeapon!0x3280 m_iszName
CBaseCombatWeapon!0x3284 m_bRemoveable
CBaseCombatWeapon!0x3285 m_bInReload
CBaseCombatWeapon!0x3286 m_bFireOnEmpty
CBaseCombatWeapon!0x3287 m_bFiresUnderwater
CBaseCombatWeapon!0x3288 m_bAltFiresUnderwater
CBaseCombatWeapon!0x3289 m_bReloadsSingly
```
</details>
<details>
<summary><code>class CBaseGrenade extends C_BaseAnimating</code></summary>

```
{
	m_vecVelocity: Vector,
	m_bIsLive: Boolean,
	m_DmgRadius: Boolean,
	m_flNextAttack: Float,
	m_flDamage: Float,
	m_hThrower: EHandle,
}
```

#### Offsets

```
CBaseGrenade!0x0114 m_vecVelocity
CBaseGrenade!0x2981 m_bIsLive
CBaseGrenade!0x2984 m_DmgRadius
CBaseGrenade!0x2988 m_flNextAttack
CBaseGrenade!0x2998 m_flDamage
CBaseGrenade!0x29a0 m_hThrower
```
</details>
<details>
<summary><code>class CBaseViewModel extends C_BaseAnimating</code></summary>

```
{
	m_fEffects: Integer,
	m_nModelIndex: Short,
	m_flAnimTime: Float,
	m_flCycle: Float,
	m_flPlaybackRate: Float,
	m_nSkin: Integer,
	m_nBody: Integer,
	m_nSequence: Integer,
	m_nViewModelIndex: Integer,
	m_nAnimationParity: Integer,
	m_hWeapon: EHandle,
	m_hOwner: EHandle,
	m_flTimeWeaponIdle: Float,
	m_Activity: Integer,
}
```

#### Offsets

```
CBaseViewModel!0x00f0 m_fEffects
CBaseViewModel!0x0258 m_nModelIndex
CBaseViewModel!0x0260 m_flAnimTime
CBaseViewModel!0x0a14 m_flCycle
CBaseViewModel!0x0a18 m_flPlaybackRate
CBaseViewModel!0x0a1c m_nSkin
CBaseViewModel!0x0a20 m_nBody
CBaseViewModel!0x28bc m_nSequence
CBaseViewModel!0x29c0 m_nViewModelIndex
CBaseViewModel!0x29c4 m_nAnimationParity
CBaseViewModel!0x29c8 m_hWeapon
CBaseViewModel!0x29cc m_hOwner
CBaseViewModel!0x29d0 m_flTimeWeaponIdle
CBaseViewModel!0x29d4 m_Activity
```
</details>
<details>
<summary><code>class CBaseWeaponWorldModel extends C_BaseAnimatingOverlay</code></summary>

```
{
	m_fEffects: Integer,
	m_nModelIndex: Short,
	m_nBody: Integer,
	m_hCombatWeaponParent: EHandle,
}
```

#### Offsets

```
CBaseWeaponWorldModel!0x00f0 m_fEffects
CBaseWeaponWorldModel!0x0258 m_nModelIndex
CBaseWeaponWorldModel!0x0a20 m_nBody
CBaseWeaponWorldModel!0x29f0 m_hCombatWeaponParent
```
</details>
<details>
<summary><code>class CBeam extends C_BaseEntity</code></summary>

```
{
	m_clrRender: Integer,
	m_vecOrigin: Vector,
	m_nModelIndex: Integer,
	m_nRenderFX: Integer,
	m_nRenderMode: Integer,
	m_flFrameRate: Float,
	m_nNumBeamEnts: Integer,
	m_nHaloIndex: Integer,
	m_nBeamType: Integer,
	m_hAttachEntity: EHandle,
	m_nAttachIndex: Integer,
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

#### Offsets

```
CBeam!0x0070 m_clrRender
CBeam!0x00ac m_vecOrigin
CBeam!0x0258 m_nModelIndex
CBeam!0x025a m_nRenderFX
CBeam!0x025b m_nRenderMode
CBeam!0x09d8 m_flFrameRate
CBeam!0x09e8 m_nNumBeamEnts
CBeam!0x09f0 m_nHaloIndex
CBeam!0x09f4 m_nBeamType
CBeam!0x09fc m_hAttachEntity
CBeam!0x0a24 m_nAttachIndex
CBeam!0x0a4c m_fWidth
CBeam!0x0a50 m_fEndWidth
CBeam!0x0a54 m_fFadeLength
CBeam!0x0a58 m_fHaloScale
CBeam!0x0a5c m_fAmplitude
CBeam!0x0a60 m_fStartFrame
CBeam!0x0a64 m_fSpeed
CBeam!0x0a68 m_flFrame
CBeam!0x0a70 m_vecEndPos
```
</details>
<details>
<summary><code>class CC4 extends CWeaponCSBase</code></summary>

```
{
	m_bStartedArming: Integer,
	m_fArmedTime: Float,
	m_bBombPlacedAnimation: Integer,
	m_bShowC4LED: Integer,
	m_bIsPlantingViaUse: Integer,
}
```

#### Offsets

```
CC4!0x33d0 m_bStartedArming
CC4!0x33d4 m_fArmedTime
CC4!0x33d8 m_bBombPlacedAnimation
CC4!0x33d9 m_bShowC4LED
CC4!0x33da m_bIsPlantingViaUse
```
</details>
<details>
<summary><code>class CCollisionProperty</code></summary>

```
{
	m_vecMins: Vector,
	m_vecMaxs: Vector,
	m_usSolidFlags: Short,
	m_nSolidType: Char,
	m_triggerBloat: Char,
}
```

#### Offsets

```
CCollisionProperty!0x0008 m_vecMins
CCollisionProperty!0x0014 m_vecMaxs
CCollisionProperty!0x0020 m_usSolidFlags
CCollisionProperty!0x0022 m_nSolidType
CCollisionProperty!0x0023 m_triggerBloat
```
</details>
<details>
<summary><code>class CMelee extends CWeaponCSBase</code></summary>

```
{
	m_flThrowAt: Float,
}
```

#### Offsets

```
CMelee!0x33b0 m_flThrowAt
```
</details>
<details>
<summary><code>class CPlayerLocalData</code></summary>

```
{
	m_nStepside: Integer,
	m_nOldButtons: Integer,
	m_flFOVRate: Float,
	m_iHideHUD: Integer,
	m_nDuckTimeMsecs: Integer,
	m_nDuckJumpTimeMsecs: Integer,
	m_nJumpTimeMsecs: Integer,
	m_flFallVelocity: Float,
	m_flStepSize: Float,
	m_viewPunchAngle: Vector,
	m_aimPunchAngle: Vector,
	m_aimPunchAngleVel: Vector,
	m_bDucked: Boolean,
	m_bDucking: Boolean,
	m_flLastDuckTime: Float,
	m_bInDuckJump: Boolean,
	m_bDrawViewmodel: Boolean,
	m_bWearingSuit: Boolean,
	m_bPoisoned: Boolean,
	m_bAllowAutoMovement: Boolean,
}
```

#### Offsets

```
CPlayerLocalData!0x003c m_nStepside
CPlayerLocalData!0x0040 m_nOldButtons
CPlayerLocalData!0x0044 m_flFOVRate
CPlayerLocalData!0x0048 m_iHideHUD
CPlayerLocalData!0x004c m_nDuckTimeMsecs
CPlayerLocalData!0x0050 m_nDuckJumpTimeMsecs
CPlayerLocalData!0x0054 m_nJumpTimeMsecs
CPlayerLocalData!0x0058 m_flFallVelocity
CPlayerLocalData!0x0060 m_flStepSize
CPlayerLocalData!0x0064 m_viewPunchAngle
CPlayerLocalData!0x0070 m_aimPunchAngle
CPlayerLocalData!0x007c m_aimPunchAngleVel
CPlayerLocalData!0x0088 m_bDucked
CPlayerLocalData!0x0089 m_bDucking
CPlayerLocalData!0x008c m_flLastDuckTime
CPlayerLocalData!0x0090 m_bInDuckJump
CPlayerLocalData!0x0091 m_bDrawViewmodel
CPlayerLocalData!0x0092 m_bWearingSuit
CPlayerLocalData!0x0093 m_bPoisoned
CPlayerLocalData!0x0094 m_bAllowAutoMovement
```
</details>
<details>
<summary><code>class CPlayerState</code></summary>

```
{
	deadflag: Boolean,
}
```

#### Offsets

```
CPlayerState!0x0004 deadflag
```
</details>
<details>
<summary><code>class CSprite extends C_BaseEntity</code></summary>

```
{
	m_hAttachedToEntity: EHandle,
	m_nAttachment: Integer,
	m_flSpriteFramerate: Float,
	m_flFrame: Float,
	m_flDieTime: Float,
	m_nBrightness: Integer,
	m_flBrightnessTime: Float,
	m_flSpriteScale: Float,
	m_flScaleTime: Float,
	m_flLastTime: Float,
	m_flMaxFrame: Float,
}
```

#### Offsets

```
CSprite!0x09e8 m_hAttachedToEntity
CSprite!0x09ec m_nAttachment
CSprite!0x09f0 m_flSpriteFramerate
CSprite!0x09f4 m_flFrame
CSprite!0x09f8 m_flDieTime
CSprite!0x09fc m_nBrightness
CSprite!0x0a00 m_flBrightnessTime
CSprite!0x0a04 m_flSpriteScale
CSprite!0x0a08 m_flScaleTime
CSprite!0x0a18 m_flLastTime
CSprite!0x0a1c m_flMaxFrame
```
</details>
<details>
<summary><code>class CWeaponBaseItem extends CWeaponCSBase</code></summary>

```
{
	m_bRedraw: Boolean,
}
```

#### Offsets

```
CWeaponBaseItem!0x33bc m_bRedraw
```
</details>
<details>
<summary><code>class CWeaponCSBase extends CBaseCombatWeapon</code></summary>

```
{
	m_flNextPrimaryAttack: Float,
	m_flNextSecondaryAttack: Float,
	m_flTimeWeaponIdle: Float,
	m_weaponMode: Integer,
	m_fAccuracyPenalty: Float,
	m_iRecoilIndex: Integer,
	m_flRecoilIndex: Float,
	m_flPostponeFireReadyTime: Float,
	m_bReloadVisuallyComplete: Boolean,
	m_fLastShotTime: Float,
	m_iIronSightMode: Integer,
}
```

#### Offsets

```
CWeaponCSBase!0x3218 m_flNextPrimaryAttack
CWeaponCSBase!0x321c m_flNextSecondaryAttack
CWeaponCSBase!0x3254 m_flTimeWeaponIdle
CWeaponCSBase!0x32f8 m_weaponMode
CWeaponCSBase!0x3310 m_fAccuracyPenalty
CWeaponCSBase!0x331c m_iRecoilIndex
CWeaponCSBase!0x3320 m_flRecoilIndex
CWeaponCSBase!0x3328 m_flPostponeFireReadyTime
CWeaponCSBase!0x332c m_bReloadVisuallyComplete
CWeaponCSBase!0x3384 m_fLastShotTime
CWeaponCSBase!0x33a0 m_iIronSightMode
```
</details>
<details>
<summary><code>class CWeaponCSBaseGun extends CWeaponCSBase</code></summary>

```
{
	m_zoomLevel: Integer,
	m_iBurstShotsRemaining: Integer,
	m_fNextBurstShot: Float,
}
```

#### Offsets

```
CWeaponCSBaseGun!0x33b0 m_zoomLevel
CWeaponCSBaseGun!0x33b4 m_iBurstShotsRemaining
CWeaponCSBaseGun!0x33b8 m_fNextBurstShot
```
</details>
<details>
<summary><code>class CWeaponM3 extends CWeaponCSBase</code></summary>

```
{
	m_reloadState: Integer,
}
```

#### Offsets

```
CWeaponM3!0x33b4 m_reloadState
```
</details>
<details>
<summary><code>class CWeaponNOVA extends CWeaponCSBase</code></summary>

```
{
	m_reloadState: Integer,
}
```

#### Offsets

```
CWeaponNOVA!0x33b4 m_reloadState
```
</details>
<details>
<summary><code>class CWeaponSawedoff extends CWeaponCSBase</code></summary>

```
{
	m_reloadState: Integer,
}
```

#### Offsets

```
CWeaponSawedoff!0x33b4 m_reloadState
```
</details>
<details>
<summary><code>class CWeaponTaser extends CWeaponCSBaseGun</code></summary>

```
{
	m_fFireTime: Float,
}
```

#### Offsets

```
CWeaponTaser!0x33d0 m_fFireTime
```
</details>
<details>
<summary><code>class CWeaponXM1014 extends CWeaponCSBase</code></summary>

```
{
	m_reloadState: Integer,
}
```

#### Offsets

```
CWeaponXM1014!0x33b4 m_reloadState
```
</details>
<details>
<summary><code>class C_BaseAnimating extends C_BaseEntity</code></summary>

```
{
	m_flPlaybackRate: Float,
	m_nSkin: Integer,
	m_nBody: Integer,
	m_nNewSequenceParity: Integer,
	m_nResetEventsParity: Integer,
	m_flEncodedController: Float,
	m_nMuzzleFlashParity: Char,
}
```

#### Offsets

```
C_BaseAnimating!0x0a18 m_flPlaybackRate
C_BaseAnimating!0x0a1c m_nSkin
C_BaseAnimating!0x0a20 m_nBody
C_BaseAnimating!0x0a44 m_nNewSequenceParity
C_BaseAnimating!0x0a48 m_nResetEventsParity
C_BaseAnimating!0x0a54 m_flEncodedController
C_BaseAnimating!0x0a64 m_nMuzzleFlashParity
```
</details>
<details>
<summary><code>class C_BaseCombatCharacter extends C_BaseFlex</code></summary>

```
{
	m_flNextAttack: Float,
	m_iAmmo: Integer,
	m_hMyWeapons: EHandle,
	m_hActiveWeapon: EHandle,
}
```

#### Offsets

```
C_BaseCombatCharacter!0x2d70 m_flNextAttack
C_BaseCombatCharacter!0x2d78 m_iAmmo
C_BaseCombatCharacter!0x2df8 m_hMyWeapons
C_BaseCombatCharacter!0x2ef8 m_hActiveWeapon
```
</details>
<details>
<summary><code>class C_BaseEntity</code></summary>

```
{
	m_vecAbsVelocity: Vector,
	m_vecAbsOrigin: Vector,
	m_vecOrigin: Vector,
	m_vecAngVelocity: Vector,
	m_angAbsRotation: Vector,
	m_angRotation: Vector,
	m_flMaxFallVelocity: Float,
	m_flGravity: Float,
	m_flProxyRandomValue: Float,
	m_iEFlags: Integer,
	m_nWaterType: Char,
	m_fEffects: Integer,
	m_iTeamNum: Integer,
	m_iPendingTeamNum: Integer,
	m_iHealth: Integer,
	m_fFlags: Integer,
	m_vecViewOffset: Vector,
	m_vecVelocity: Vector,
	m_vecBaseVelocity: Vector,
	m_angNetworkAngles: Vector,
	m_vecNetworkOrigin: Vector,
	m_flFriction: Float,
	m_hNetworkMoveParent: EHandle,
	m_hOwnerEntity: EHandle,
	m_hGroundEntity: EHandle,
	m_nModelIndex: Short,
	m_nRenderFX: Char,
	m_nRenderMode: Char,
	m_MoveType: Char,
	m_MoveCollide: Char,
	m_nWaterLevel: Char,
	m_flUseLookAtAngle: Float,
	m_Collision: CCollisionProperty,
	m_bEverHadPredictionErrorsForThisCommand: Boolean,
}
```

#### Offsets

```
C_BaseEntity!0x0094 m_vecAbsVelocity
C_BaseEntity!0x00a0 m_vecAbsOrigin
C_BaseEntity!0x00ac m_vecOrigin
C_BaseEntity!0x00b8 m_vecAngVelocity
C_BaseEntity!0x00c4 m_angAbsRotation
C_BaseEntity!0x00d0 m_angRotation
C_BaseEntity!0x00dc m_flMaxFallVelocity
C_BaseEntity!0x00e0 m_flGravity
C_BaseEntity!0x00e4 m_flProxyRandomValue
C_BaseEntity!0x00e8 m_iEFlags
C_BaseEntity!0x00ec m_nWaterType
C_BaseEntity!0x00f0 m_fEffects
C_BaseEntity!0x00f4 m_iTeamNum
C_BaseEntity!0x00f8 m_iPendingTeamNum
C_BaseEntity!0x0100 m_iHealth
C_BaseEntity!0x0104 m_fFlags
C_BaseEntity!0x0108 m_vecViewOffset
C_BaseEntity!0x0114 m_vecVelocity
C_BaseEntity!0x0120 m_vecBaseVelocity
C_BaseEntity!0x012c m_angNetworkAngles
C_BaseEntity!0x0138 m_vecNetworkOrigin
C_BaseEntity!0x0144 m_flFriction
C_BaseEntity!0x0148 m_hNetworkMoveParent
C_BaseEntity!0x014c m_hOwnerEntity
C_BaseEntity!0x0150 m_hGroundEntity
C_BaseEntity!0x0258 m_nModelIndex
C_BaseEntity!0x025a m_nRenderFX
C_BaseEntity!0x025b m_nRenderMode
C_BaseEntity!0x025c m_MoveType
C_BaseEntity!0x025d m_MoveCollide
C_BaseEntity!0x025e m_nWaterLevel
C_BaseEntity!0x02cc m_flUseLookAtAngle
C_BaseEntity!0x031c m_Collision
C_BaseEntity!0x0938 m_bEverHadPredictionErrorsForThisCommand
```
</details>
<details>
<summary><code>class C_BasePlayer extends C_BaseCombatCharacter</code></summary>

```
{
	m_nNextThinkTick: Integer,
	m_iHealth: Integer,
	m_vecBaseVelocity: Vector,
	m_hGroundEntity: EHandle,
	m_nWaterLevel: Char,
	m_lifeState: Char,
	m_flDuckAmount: Float,
	m_flDuckSpeed: Float,
	m_Local: CPlayerLocalData,
	pl: CPlayerState,
	m_iFOV: Integer,
	m_iFOVStart: Integer,
	m_afButtonLast: Integer,
	m_afButtonPressed: Integer,
	m_afButtonReleased: Integer,
	m_nButtons: Integer,
	m_nImpulse: Integer,
	m_ladderSurfaceProps: Integer,
	m_flPhysics: Integer,
	m_flFOVTime: Float,
	m_flWaterJumpTime: Float,
	m_flSwimSoundTime: Float,
	m_ignoreLadderJumpTime: Float,
	m_bHasWalkMovedSinceLastJump: Boolean,
	m_flStepSoundTime: Float,
	m_surfaceFriction: Float,
	m_vecLadderNormal: Vector,
	m_iBonusProgress: Integer,
	m_iBonusChallenge: Integer,
	m_flMaxspeed: Float,
	m_hZoomOwner: EHandle,
	m_vphysicsCollisionState: Integer,
	m_oldOrigin: Vector,
	m_bTouchedPhysObject: Boolean,
	m_bPhysicsWasFrozen: Boolean,
	m_vNewVPhysicsPosition: Vector,
	m_vNewVPhysicsVelocity: Vector,
	m_afPhysicsFlags: Integer,
	m_hVehicle: EHandle,
	m_hLastWeapon: EHandle,
	m_hViewModel: EHandle,
	m_fOnTarget: Boolean,
	m_nTickBase: Integer,
	m_vecPreviouslyPredictedOrigin: Vector,
}
```

#### Offsets

```
C_BasePlayer!0x00fc m_nNextThinkTick
C_BasePlayer!0x0100 m_iHealth
C_BasePlayer!0x0120 m_vecBaseVelocity
C_BasePlayer!0x0150 m_hGroundEntity
C_BasePlayer!0x025e m_nWaterLevel
C_BasePlayer!0x025f m_lifeState
C_BasePlayer!0x2fac m_flDuckAmount
C_BasePlayer!0x2fb0 m_flDuckSpeed
C_BasePlayer!0x2fbc m_Local
C_BasePlayer!0x31d0 pl
C_BasePlayer!0x31e4 m_iFOV
C_BasePlayer!0x31e8 m_iFOVStart
C_BasePlayer!0x31ec m_afButtonLast
C_BasePlayer!0x31f0 m_afButtonPressed
C_BasePlayer!0x31f4 m_afButtonReleased
C_BasePlayer!0x31f8 m_nButtons
C_BasePlayer!0x31fc m_nImpulse
C_BasePlayer!0x3200 m_ladderSurfaceProps
C_BasePlayer!0x3204 m_flPhysics
C_BasePlayer!0x3208 m_flFOVTime
C_BasePlayer!0x320c m_flWaterJumpTime
C_BasePlayer!0x3210 m_flSwimSoundTime
C_BasePlayer!0x3214 m_ignoreLadderJumpTime
C_BasePlayer!0x3218 m_bHasWalkMovedSinceLastJump
C_BasePlayer!0x321c m_flStepSoundTime
C_BasePlayer!0x322c m_surfaceFriction
C_BasePlayer!0x3230 m_vecLadderNormal
C_BasePlayer!0x3240 m_iBonusProgress
C_BasePlayer!0x3244 m_iBonusChallenge
C_BasePlayer!0x3248 m_flMaxspeed
C_BasePlayer!0x324c m_hZoomOwner
C_BasePlayer!0x325c m_vphysicsCollisionState
C_BasePlayer!0x3260 m_oldOrigin
C_BasePlayer!0x326c m_bTouchedPhysObject
C_BasePlayer!0x326d m_bPhysicsWasFrozen
C_BasePlayer!0x3270 m_vNewVPhysicsPosition
C_BasePlayer!0x327c m_vNewVPhysicsVelocity
C_BasePlayer!0x32ec m_afPhysicsFlags
C_BasePlayer!0x32f0 m_hVehicle
C_BasePlayer!0x32f4 m_hLastWeapon
C_BasePlayer!0x32f8 m_hViewModel
C_BasePlayer!0x3324 m_fOnTarget
C_BasePlayer!0x342c m_nTickBase
C_BasePlayer!0x35a4 m_vecPreviouslyPredictedOrigin
```
</details>
<details>
<summary><code>class C_CSPlayer extends C_BasePlayer</code></summary>

```
{
	m_flCycle: Float,
	m_bIsScoped: Boolean,
	m_bIsWalking: Boolean,
	m_bResumeZoom: Boolean,
	m_nIsAutoMounting: Integer,
	m_flStamina: Float,
	m_iDirection: Integer,
	m_iShotsFired: Integer,
	m_nNumFastDucks: Integer,
	m_bDuckOverride: Boolean,
}
```

#### Offsets

```
C_CSPlayer!0x0a14 m_flCycle
C_CSPlayer!0x3910 m_bIsScoped
C_CSPlayer!0x3911 m_bIsWalking
C_CSPlayer!0x3912 m_bResumeZoom
C_CSPlayer!0x3a80 m_nIsAutoMounting
C_CSPlayer!0xa378 m_flStamina
C_CSPlayer!0xa37c m_iDirection
C_CSPlayer!0xa380 m_iShotsFired
C_CSPlayer!0xa384 m_nNumFastDucks
C_CSPlayer!0xa388 m_bDuckOverride
```
</details>
<details>
<summary><code>class C_ColorCorrectionVolume extends C_BaseToggle</code></summary>

```
{
	m_Weight: Float,
}
```

#### Offsets

```
C_ColorCorrectionVolume!0x0a2c m_Weight
```
</details>
<details>
<summary><code>class C_PlayerResource extends C_BaseEntity</code></summary>

```
{
	m_szName: String,
	m_bConnected: Boolean,
	m_iPing: Integer,
	m_iKills: Integer,
	m_iAssists: Integer,
	m_iDeaths: Integer,
	m_iTeam: Integer,
	m_iPendingTeam: Integer,
	m_bAlive: Boolean,
	m_iHealth: Integer,
	m_iCoachingTeam: Integer,
}
```

#### Offsets

```
C_PlayerResource!0x09e0 m_szName
C_PlayerResource!0x0ae4 m_bConnected
C_PlayerResource!0x0b28 m_iPing
C_PlayerResource!0x0c2c m_iKills
C_PlayerResource!0x0d30 m_iAssists
C_PlayerResource!0x0e34 m_iDeaths
C_PlayerResource!0x0f38 m_iTeam
C_PlayerResource!0x103c m_iPendingTeam
C_PlayerResource!0x1140 m_bAlive
C_PlayerResource!0x1184 m_iHealth
C_PlayerResource!0x1288 m_iCoachingTeam
```
</details>
<details>
<summary><code>class C_Team extends C_BaseEntity</code></summary>

```
{
	m_szTeamname: Char,
	m_szClanTeamname: Char,
	m_szTeamFlagImage: Char,
	m_szTeamLogoImage: Char,
	m_szTeamMatchStat: Char,
	m_scoreTotal: Integer,
	m_scoreFirstHalf: Integer,
	m_scoreSecondHalf: Integer,
	m_scoreOvertime: Integer,
	m_iClanID: Integer,
	m_iDeaths: Integer,
	m_iPing: Integer,
	m_iPacketloss: Integer,
	m_iTeamNum: Integer,
	m_bSurrendered: Integer,
}
```

#### Offsets

```
C_Team!0x09ec m_szTeamname
C_Team!0x0a0c m_szClanTeamname
C_Team!0x0a2c m_szTeamFlagImage
C_Team!0x0a34 m_szTeamLogoImage
C_Team!0x0a3c m_szTeamMatchStat
C_Team!0x0b40 m_scoreTotal
C_Team!0x0b44 m_scoreFirstHalf
C_Team!0x0b48 m_scoreSecondHalf
C_Team!0x0b4c m_scoreOvertime
C_Team!0x0b58 m_iClanID
C_Team!0x0b5c m_iDeaths
C_Team!0x0b60 m_iPing
C_Team!0x0b64 m_iPacketloss
C_Team!0x0b68 m_iTeamNum
C_Team!0x0b6c m_bSurrendered
```
</details>
<details>
<summary><code>class CountdownTimer</code></summary>

```
{
	m_duration: Float,
	m_timestamp: Float,
}
```

#### Offsets

```
CountdownTimer!0x0004 m_duration
CountdownTimer!0x0008 m_timestamp
```
</details>
<details>
<summary><code>class IntervalTimer</code></summary>

```
{
	m_timestamp: Float,
}
```

#### Offsets

```
IntervalTimer!0x0004 m_timestamp
```
</details>

### Recvtables

<details>
<summary><code>class DT_AI_BaseNPC</code></summary>

```
{
	m_lifeState: Int,
	m_flTimePingEffect: Float,
	m_iDeathPose: Int,
	m_iDeathFrame: Int,
	m_iSpeedModRadius: Int,
	m_iSpeedModSpeed: Int,
	m_bPerformAvoidance: Int,
	m_bIsMoving: Int,
	m_bFadeCorpse: Int,
	m_bSpeedModActive: Int,
	m_bImportanRagdoll: Int,
}
```

#### Offsets

```
DT_AI_BaseNPC!0x025f m_lifeState
DT_AI_BaseNPC!0x2f10 m_flTimePingEffect
DT_AI_BaseNPC!0x2f14 m_iDeathPose
DT_AI_BaseNPC!0x2f18 m_iDeathFrame
DT_AI_BaseNPC!0x2f1c m_iSpeedModRadius
DT_AI_BaseNPC!0x2f20 m_iSpeedModSpeed
DT_AI_BaseNPC!0x2f24 m_bPerformAvoidance
DT_AI_BaseNPC!0x2f25 m_bIsMoving
DT_AI_BaseNPC!0x2f26 m_bFadeCorpse
DT_AI_BaseNPC!0x2f27 m_bSpeedModActive
DT_AI_BaseNPC!0x2f28 m_bImportanRagdoll
```
</details>
<details>
<summary><code>class DT_AttributeContainer</code></summary>

```
{
	m_iReapplyProvisionParity: Int,
	m_hOuter: Int,
	m_ProviderType: Int,
	m_Item: DataTable,
}
```

#### Offsets

```
DT_AttributeContainer!0x0018 m_iReapplyProvisionParity
DT_AttributeContainer!0x001c m_hOuter
DT_AttributeContainer!0x0024 m_ProviderType
DT_AttributeContainer!0x0040 m_Item
```
</details>
<details>
<summary><code>class DT_AttributeContainerPlayer</code></summary>

```
{
	m_iReapplyProvisionParity: Int,
	m_hOuter: Int,
	m_ProviderType: Int,
	m_hPlayer: Int,
}
```

#### Offsets

```
DT_AttributeContainerPlayer!0x0018 m_iReapplyProvisionParity
DT_AttributeContainerPlayer!0x001c m_hOuter
DT_AttributeContainerPlayer!0x0024 m_ProviderType
DT_AttributeContainerPlayer!0x0040 m_hPlayer
```
</details>
<details>
<summary><code>class DT_AttributeList</code></summary>

```
{
}
```

#### Offsets

```
```
</details>
<details>
<summary><code>class DT_BaseAnimating</code></summary>

```
{
	serveranimdata: DataTable,
	m_bClientSideRagdoll: Int,
	m_nHitboxSet: Int,
	m_flPlaybackRate: Float,
	m_nSkin: Int,
	m_nBody: Int,
	m_nHighlightColorR: Int,
	m_nHighlightColorG: Int,
	m_nHighlightColorB: Int,
	m_nNewSequenceParity: Int,
	m_nResetEventsParity: Int,
	m_nMuzzleFlashParity: Int,
	m_vecForce: Vector,
	m_nForceBone: Int,
	m_bClientSideFrameReset: Int,
	m_flFrozen: Float,
	m_flModelScale: Float,
	m_ScaleType: Int,
	m_bClientSideAnimation: Int,
	m_nSequence: Int,
	m_bSuppressAnimSounds: Int,
}
```

#### Offsets

```
DT_BaseAnimating!0x0000 serveranimdata
DT_BaseAnimating!0x0279 m_bClientSideRagdoll
DT_BaseAnimating!0x09fc m_nHitboxSet
DT_BaseAnimating!0x0a18 m_flPlaybackRate
DT_BaseAnimating!0x0a1c m_nSkin
DT_BaseAnimating!0x0a20 m_nBody
DT_BaseAnimating!0x0a38 m_nHighlightColorR
DT_BaseAnimating!0x0a3c m_nHighlightColorG
DT_BaseAnimating!0x0a40 m_nHighlightColorB
DT_BaseAnimating!0x0a44 m_nNewSequenceParity
DT_BaseAnimating!0x0a48 m_nResetEventsParity
DT_BaseAnimating!0x0a64 m_nMuzzleFlashParity
DT_BaseAnimating!0x2680 m_vecForce
DT_BaseAnimating!0x268c m_nForceBone
DT_BaseAnimating!0x26c0 m_bClientSideFrameReset
DT_BaseAnimating!0x26f8 m_flFrozen
DT_BaseAnimating!0x2748 m_flModelScale
DT_BaseAnimating!0x274c m_ScaleType
DT_BaseAnimating!0x289c m_bClientSideAnimation
DT_BaseAnimating!0x28bc m_nSequence
DT_BaseAnimating!0x294a m_bSuppressAnimSounds
```
</details>
<details>
<summary><code>class DT_BaseAttributableItem</code></summary>

```
{
	m_AttributeManager: DataTable,
	m_OriginalOwnerXuidLow: Int,
	m_OriginalOwnerXuidHigh: Int,
	m_nFallbackPaintKit: Int,
	m_nFallbackSeed: Int,
	m_flFallbackWear: Float,
	m_nFallbackStatTrak: Int,
}
```

#### Offsets

```
DT_BaseAttributableItem!0x2d80 m_AttributeManager
DT_BaseAttributableItem!0x31b0 m_OriginalOwnerXuidLow
DT_BaseAttributableItem!0x31b4 m_OriginalOwnerXuidHigh
DT_BaseAttributableItem!0x31b8 m_nFallbackPaintKit
DT_BaseAttributableItem!0x31bc m_nFallbackSeed
DT_BaseAttributableItem!0x31c0 m_flFallbackWear
DT_BaseAttributableItem!0x31c4 m_nFallbackStatTrak
```
</details>
<details>
<summary><code>class DT_BaseFlex</code></summary>

```
{
	m_viewtarget: Vector,
	m_blinktoggle: Int,
}
```

#### Offsets

```
DT_BaseFlex!0x29f0 m_viewtarget
DT_BaseFlex!0x2bd4 m_blinktoggle
```
</details>
<details>
<summary><code>class DT_BaseTempEntity</code></summary>

```
{
}
```

#### Offsets

```
```
</details>
<details>
<summary><code>class DT_BaseToggle</code></summary>

```
{
	m_vecFinalDest: Vector,
	m_movementType: Int,
	m_flMoveTargetTime: Float,
}
```

#### Offsets

```
DT_BaseToggle!0x09ec m_vecFinalDest
DT_BaseToggle!0x09f8 m_movementType
DT_BaseToggle!0x09fc m_flMoveTargetTime
```
</details>
<details>
<summary><code>class DT_Beam</code></summary>

```
{
	m_clrRender: Int,
	m_vecOrigin: Vector,
	moveparent: Int,
	m_nModelIndex: Int,
	m_nRenderFX: Int,
	m_nRenderMode: Int,
	m_flFrameRate: Float,
	m_flHDRColorScale: Float,
	m_nNumBeamEnts: Int,
	m_nHaloIndex: Int,
	m_nBeamType: Int,
	m_nBeamFlags: Int,
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

#### Offsets

```
DT_Beam!0x0070 m_clrRender
DT_Beam!0x0138 m_vecOrigin
DT_Beam!0x0148 moveparent
DT_Beam!0x0258 m_nModelIndex
DT_Beam!0x025a m_nRenderFX
DT_Beam!0x025b m_nRenderMode
DT_Beam!0x09d8 m_flFrameRate
DT_Beam!0x09dc m_flHDRColorScale
DT_Beam!0x09e8 m_nNumBeamEnts
DT_Beam!0x09f0 m_nHaloIndex
DT_Beam!0x09f4 m_nBeamType
DT_Beam!0x09f8 m_nBeamFlags
DT_Beam!0x0a4c m_fWidth
DT_Beam!0x0a50 m_fEndWidth
DT_Beam!0x0a54 m_fFadeLength
DT_Beam!0x0a58 m_fHaloScale
DT_Beam!0x0a5c m_fAmplitude
DT_Beam!0x0a60 m_fStartFrame
DT_Beam!0x0a64 m_fSpeed
DT_Beam!0x0a68 m_flFrame
DT_Beam!0x0a6c m_nClipStyle
DT_Beam!0x0a70 m_vecEndPos
```
</details>
<details>
<summary><code>class DT_BumpMineProjectile</code></summary>

```
{
	m_nParentBoneIndex: Int,
	m_vecParentBonePos: Vector,
	m_bArmed: Int,
}
```

#### Offsets

```
DT_BumpMineProjectile!0x29b4 m_nParentBoneIndex
DT_BumpMineProjectile!0x29b8 m_vecParentBonePos
DT_BumpMineProjectile!0x29c4 m_bArmed
```
</details>
<details>
<summary><code>class DT_CFish</code></summary>

```
{
	m_nModelIndex: Int,
	m_lifeState: Int,
	m_x: Float,
	m_y: Float,
	m_z: Float,
	m_angle: Float,
	m_poolOrigin: Vector,
	m_waterLevel: Float,
}
```

#### Offsets

```
DT_CFish!0x0258 m_nModelIndex
DT_CFish!0x025f m_lifeState
DT_CFish!0x29c8 m_x
DT_CFish!0x29cc m_y
DT_CFish!0x29d0 m_z
DT_CFish!0x29d8 m_angle
DT_CFish!0x29e0 m_poolOrigin
DT_CFish!0x29ec m_waterLevel
```
</details>
<details>
<summary><code>class DT_CSGameRules</code></summary>

```
{
	m_bFreezePeriod: Int,
	m_bWarmupPeriod: Int,
	m_fWarmupPeriodEnd: Float,
	m_fWarmupPeriodStart: Float,
	m_bTerroristTimeOutActive: Int,
	m_bCTTimeOutActive: Int,
	m_flTerroristTimeOutRemaining: Float,
	m_flCTTimeOutRemaining: Float,
	m_nTerroristTimeOuts: Int,
	m_nCTTimeOuts: Int,
	m_bMatchWaitingForResume: Int,
	m_iRoundTime: Int,
	m_fMatchStartTime: Float,
	m_fRoundStartTime: Float,
	m_flRestartRoundTime: Float,
	m_bGameRestart: Int,
	m_flGameStartTime: Float,
	m_timeUntilNextPhaseStarts: Float,
	m_gamePhase: Int,
	m_totalRoundsPlayed: Int,
	m_nOvertimePlaying: Int,
	m_iHostagesRemaining: Int,
	m_bAnyHostageReached: Int,
	m_bMapHasBombTarget: Int,
	m_bMapHasRescueZone: Int,
	m_bMapHasBuyZone: Int,
	m_bIsQueuedMatchmaking: Int,
	m_bIsValveDS: Int,
	m_bLogoMap: Int,
	m_bPlayAllStepSoundsOnServer: Int,
	m_iNumGunGameProgressiveWeaponsCT: Int,
	m_iNumGunGameProgressiveWeaponsT: Int,
	m_iSpectatorSlotCount: Int,
	m_MatchDevice: Int,
	m_bHasMatchStarted: Int,
	m_flDMBonusStartTime: Float,
	m_flDMBonusTimeLength: Float,
	m_unDMBonusWeaponLoadoutSlot: Int,
	m_bDMBonusActive: Int,
	m_nNextMapInMapgroup: Int,
	m_szTournamentEventName: String,
	m_szTournamentEventStage: String,
	m_szMatchStatTxt: String,
	m_szTournamentPredictionsTxt: String,
	m_nTournamentPredictionsPct: Int,
	m_flCMMItemDropRevealStartTime: Float,
	m_flCMMItemDropRevealEndTime: Float,
	m_bIsDroppingItems: Int,
	m_bIsQuestEligible: Int,
	m_nGuardianModeWaveNumber: Int,
	m_nGuardianModeSpecialKillsRemaining: Int,
	m_nGuardianModeSpecialWeaponNeeded: Int,
	m_numGlobalGiftsGiven: Int,
	m_numGlobalGifters: Int,
	m_numGlobalGiftsPeriodSeconds: Int,
	m_numBestOfMaps: Int,
	m_nHalloweenMaskListSeed: Int,
	m_bBombDropped: Int,
	m_bBombPlanted: Int,
	m_iRoundWinStatus: Int,
	m_eRoundWinReason: Int,
	m_bTCantBuy: Int,
	m_bCTCantBuy: Int,
	m_flGuardianBuyUntilTime: Float,
	m_iActiveAssassinationTargetMissionID: Int,
	m_nEndMatchMapVoteWinner: Int,
	m_iNumConsecutiveCTLoses: Int,
	m_iNumConsecutiveTerroristLoses: Int,
	m_SurvivalRules: DataTable,
}
```

#### Offsets

```
DT_CSGameRules!0x0020 m_bFreezePeriod
DT_CSGameRules!0x0021 m_bWarmupPeriod
DT_CSGameRules!0x0024 m_fWarmupPeriodEnd
DT_CSGameRules!0x0028 m_fWarmupPeriodStart
DT_CSGameRules!0x002c m_bTerroristTimeOutActive
DT_CSGameRules!0x002d m_bCTTimeOutActive
DT_CSGameRules!0x0030 m_flTerroristTimeOutRemaining
DT_CSGameRules!0x0034 m_flCTTimeOutRemaining
DT_CSGameRules!0x0038 m_nTerroristTimeOuts
DT_CSGameRules!0x003c m_nCTTimeOuts
DT_CSGameRules!0x0040 m_bMatchWaitingForResume
DT_CSGameRules!0x0044 m_iRoundTime
DT_CSGameRules!0x0048 m_fMatchStartTime
DT_CSGameRules!0x004c m_fRoundStartTime
DT_CSGameRules!0x0050 m_flRestartRoundTime
DT_CSGameRules!0x0054 m_bGameRestart
DT_CSGameRules!0x0058 m_flGameStartTime
DT_CSGameRules!0x005c m_timeUntilNextPhaseStarts
DT_CSGameRules!0x0060 m_gamePhase
DT_CSGameRules!0x0064 m_totalRoundsPlayed
DT_CSGameRules!0x0068 m_nOvertimePlaying
DT_CSGameRules!0x006c m_iHostagesRemaining
DT_CSGameRules!0x0070 m_bAnyHostageReached
DT_CSGameRules!0x0071 m_bMapHasBombTarget
DT_CSGameRules!0x0072 m_bMapHasRescueZone
DT_CSGameRules!0x0073 m_bMapHasBuyZone
DT_CSGameRules!0x0074 m_bIsQueuedMatchmaking
DT_CSGameRules!0x0075 m_bIsValveDS
DT_CSGameRules!0x0076 m_bLogoMap
DT_CSGameRules!0x0077 m_bPlayAllStepSoundsOnServer
DT_CSGameRules!0x0078 m_iNumGunGameProgressiveWeaponsCT
DT_CSGameRules!0x007c m_iNumGunGameProgressiveWeaponsT
DT_CSGameRules!0x0080 m_iSpectatorSlotCount
DT_CSGameRules!0x0444 m_MatchDevice
DT_CSGameRules!0x0448 m_bHasMatchStarted
DT_CSGameRules!0x044c m_flDMBonusStartTime
DT_CSGameRules!0x0450 m_flDMBonusTimeLength
DT_CSGameRules!0x0454 m_unDMBonusWeaponLoadoutSlot
DT_CSGameRules!0x0456 m_bDMBonusActive
DT_CSGameRules!0x0458 m_nNextMapInMapgroup
DT_CSGameRules!0x045c m_szTournamentEventName
DT_CSGameRules!0x0560 m_szTournamentEventStage
DT_CSGameRules!0x0664 m_szMatchStatTxt
DT_CSGameRules!0x0768 m_szTournamentPredictionsTxt
DT_CSGameRules!0x086c m_nTournamentPredictionsPct
DT_CSGameRules!0x0870 m_flCMMItemDropRevealStartTime
DT_CSGameRules!0x0874 m_flCMMItemDropRevealEndTime
DT_CSGameRules!0x0878 m_bIsDroppingItems
DT_CSGameRules!0x0879 m_bIsQuestEligible
DT_CSGameRules!0x087c m_nGuardianModeWaveNumber
DT_CSGameRules!0x0880 m_nGuardianModeSpecialKillsRemaining
DT_CSGameRules!0x0884 m_nGuardianModeSpecialWeaponNeeded
DT_CSGameRules!0x0890 m_numGlobalGiftsGiven
DT_CSGameRules!0x0894 m_numGlobalGifters
DT_CSGameRules!0x0898 m_numGlobalGiftsPeriodSeconds
DT_CSGameRules!0x0994 m_numBestOfMaps
DT_CSGameRules!0x0998 m_nHalloweenMaskListSeed
DT_CSGameRules!0x099c m_bBombDropped
DT_CSGameRules!0x099d m_bBombPlanted
DT_CSGameRules!0x09a0 m_iRoundWinStatus
DT_CSGameRules!0x09a4 m_eRoundWinReason
DT_CSGameRules!0x09a8 m_bTCantBuy
DT_CSGameRules!0x09a9 m_bCTCantBuy
DT_CSGameRules!0x09ac m_flGuardianBuyUntilTime
DT_CSGameRules!0x0c18 m_iActiveAssassinationTargetMissionID
DT_CSGameRules!0x0c70 m_nEndMatchMapVoteWinner
DT_CSGameRules!0x0c74 m_iNumConsecutiveCTLoses
DT_CSGameRules!0x0c78 m_iNumConsecutiveTerroristLoses
DT_CSGameRules!0x0cf8 m_SurvivalRules
```
</details>
<details>
<summary><code>class DT_CascadeLight</code></summary>

```
{
	m_shadowDirection: Vector,
	m_envLightShadowDirection: Vector,
	m_bEnabled: Int,
	m_bUseLightEnvAngles: Int,
	m_LightColor: Int,
	m_LightColorScale: Int,
	m_flMaxShadowDist: Float,
}
```

#### Offsets

```
DT_CascadeLight!0x09d8 m_shadowDirection
DT_CascadeLight!0x09e4 m_envLightShadowDirection
DT_CascadeLight!0x09f0 m_bEnabled
DT_CascadeLight!0x09f1 m_bUseLightEnvAngles
DT_CascadeLight!0x09f2 m_LightColor
DT_CascadeLight!0x09f8 m_LightColorScale
DT_CascadeLight!0x09fc m_flMaxShadowDist
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
	m_nSurroundType: Int,
	m_vecSpecifiedSurroundingMins: Vector,
	m_vecSpecifiedSurroundingMaxs: Vector,
}
```

#### Offsets

```
DT_CollisionProperty!0x0008 m_vecMins
DT_CollisionProperty!0x0014 m_vecMaxs
DT_CollisionProperty!0x0020 m_usSolidFlags
DT_CollisionProperty!0x0022 m_nSolidType
DT_CollisionProperty!0x0023 m_triggerBloat
DT_CollisionProperty!0x002a m_nSurroundType
DT_CollisionProperty!0x002c m_vecSpecifiedSurroundingMins
DT_CollisionProperty!0x0038 m_vecSpecifiedSurroundingMaxs
```
</details>
<details>
<summary><code>class DT_ColorCorrectionVolume</code></summary>

```
{
	m_bEnabled: Int,
	m_MaxWeight: Float,
	m_FadeDuration: Float,
	m_Weight: Float,
	m_lookupFilename: String,
}
```

#### Offsets

```
DT_ColorCorrectionVolume!0x0a20 m_bEnabled
DT_ColorCorrectionVolume!0x0a24 m_MaxWeight
DT_ColorCorrectionVolume!0x0a28 m_FadeDuration
DT_ColorCorrectionVolume!0x0a2c m_Weight
DT_ColorCorrectionVolume!0x0a30 m_lookupFilename
```
</details>
<details>
<summary><code>class DT_DangerZone</code></summary>

```
{
	m_vecDangerZoneOriginStartedAt: Vector,
	m_flBombLaunchTime: Float,
	m_flExtraRadius: Float,
	m_flExtraRadiusStartTime: Float,
	m_flExtraRadiusTotalLerpTime: Float,
	m_nDropOrder: Int,
	m_iWave: Int,
}
```

#### Offsets

```
DT_DangerZone!0x09d8 m_vecDangerZoneOriginStartedAt
DT_DangerZone!0x09e4 m_flBombLaunchTime
DT_DangerZone!0x09e8 m_flExtraRadius
DT_DangerZone!0x09ec m_flExtraRadiusStartTime
DT_DangerZone!0x09f0 m_flExtraRadiusTotalLerpTime
DT_DangerZone!0x09f4 m_nDropOrder
DT_DangerZone!0x09f8 m_iWave
```
</details>
<details>
<summary><code>class DT_DynamicLight</code></summary>

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

#### Offsets

```
DT_DynamicLight!0x09d8 m_Flags
DT_DynamicLight!0x09d9 m_LightStyle
DT_DynamicLight!0x09dc m_Radius
DT_DynamicLight!0x09e0 m_Exponent
DT_DynamicLight!0x09e4 m_InnerAngle
DT_DynamicLight!0x09e8 m_OuterAngle
DT_DynamicLight!0x09ec m_SpotRadius
```
</details>
<details>
<summary><code>class DT_EconEntity</code></summary>

```
{
	m_AttributeManager: DataTable,
	m_OriginalOwnerXuidLow: Int,
	m_OriginalOwnerXuidHigh: Int,
	m_nFallbackPaintKit: Int,
	m_nFallbackSeed: Int,
	m_flFallbackWear: Float,
	m_nFallbackStatTrak: Int,
}
```

#### Offsets

```
DT_EconEntity!0x2d80 m_AttributeManager
DT_EconEntity!0x31b0 m_OriginalOwnerXuidLow
DT_EconEntity!0x31b4 m_OriginalOwnerXuidHigh
DT_EconEntity!0x31b8 m_nFallbackPaintKit
DT_EconEntity!0x31bc m_nFallbackSeed
DT_EconEntity!0x31c0 m_flFallbackWear
DT_EconEntity!0x31c4 m_nFallbackStatTrak
```
</details>
<details>
<summary><code>class DT_EntityFreezing</code></summary>

```
{
	m_vFreezingOrigin: Vector,
	m_flFrozen: Float,
	m_bFinishFreezing: Int,
}
```

#### Offsets

```
DT_EntityFreezing!0x09d8 m_vFreezingOrigin
DT_EntityFreezing!0x0aac m_flFrozen
DT_EntityFreezing!0x0ab0 m_bFinishFreezing
```
</details>
<details>
<summary><code>class DT_EntityParticleTrail</code></summary>

```
{
	m_iMaterialName: Int,
	m_Info: DataTable,
	m_hConstraintEntity: Int,
}
```

#### Offsets

```
DT_EntityParticleTrail!0x0ac0 m_iMaterialName
DT_EntityParticleTrail!0x0ac4 m_Info
DT_EntityParticleTrail!0x0ad8 m_hConstraintEntity
```
</details>
<details>
<summary><code>class DT_EnvDOFController</code></summary>

```
{
	m_bDOFEnabled: Int,
	m_flNearBlurDepth: Float,
	m_flNearFocusDepth: Float,
	m_flFarFocusDepth: Float,
	m_flFarBlurDepth: Float,
	m_flNearBlurRadius: Float,
	m_flFarBlurRadius: Float,
}
```

#### Offsets

```
DT_EnvDOFController!0x09d8 m_bDOFEnabled
DT_EnvDOFController!0x09dc m_flNearBlurDepth
DT_EnvDOFController!0x09e0 m_flNearFocusDepth
DT_EnvDOFController!0x09e4 m_flFarFocusDepth
DT_EnvDOFController!0x09e8 m_flFarBlurDepth
DT_EnvDOFController!0x09ec m_flNearBlurRadius
DT_EnvDOFController!0x09f0 m_flFarBlurRadius
```
</details>
<details>
<summary><code>class DT_EnvGasCanister</code></summary>

```
{
	m_vecOrigin: VectorXY,
	m_vecOrigin[2]: Float,
	m_bLanded: Int,
	m_hSkyboxCopy: Int,
	m_vecImpactPosition: Vector,
	m_vecStartPosition: Vector,
	m_vecEnterWorldPosition: Vector,
	m_vecDirection: Vector,
	m_vecStartAngles: Vector,
	m_flFlightTime: Float,
	m_flFlightSpeed: Float,
	m_flLaunchTime: Float,
	m_flInitialZSpeed: Float,
	m_flZAcceleration: Float,
	m_flHorizSpeed: Float,
	m_bLaunchedFromWithinWorld: Int,
	m_vecParabolaDirection: Vector,
	m_flWorldEnterTime: Float,
	m_vecSkyboxOrigin: Vector,
	m_flSkyboxScale: Float,
	m_bInSkybox: Int,
	m_bDoImpactEffects: Int,
	m_nMyZoneIndex: Int,
}
```

#### Offsets

```
DT_EnvGasCanister!0x0138 m_vecOrigin
DT_EnvGasCanister!0x0140 m_vecOrigin[2]
DT_EnvGasCanister!0x2980 m_bLanded
DT_EnvGasCanister!0x29a8 m_hSkyboxCopy
DT_EnvGasCanister!0x29b4 m_vecImpactPosition
DT_EnvGasCanister!0x29c0 m_vecStartPosition
DT_EnvGasCanister!0x29cc m_vecEnterWorldPosition
DT_EnvGasCanister!0x29d8 m_vecDirection
DT_EnvGasCanister!0x29e4 m_vecStartAngles
DT_EnvGasCanister!0x29f0 m_flFlightTime
DT_EnvGasCanister!0x29f4 m_flFlightSpeed
DT_EnvGasCanister!0x29f8 m_flLaunchTime
DT_EnvGasCanister!0x29fc m_flInitialZSpeed
DT_EnvGasCanister!0x2a00 m_flZAcceleration
DT_EnvGasCanister!0x2a04 m_flHorizSpeed
DT_EnvGasCanister!0x2a08 m_bLaunchedFromWithinWorld
DT_EnvGasCanister!0x2a0c m_vecParabolaDirection
DT_EnvGasCanister!0x2a18 m_flWorldEnterTime
DT_EnvGasCanister!0x2a1c m_vecSkyboxOrigin
DT_EnvGasCanister!0x2a28 m_flSkyboxScale
DT_EnvGasCanister!0x2a2c m_bInSkybox
DT_EnvGasCanister!0x2a2d m_bDoImpactEffects
DT_EnvGasCanister!0x2a30 m_nMyZoneIndex
```
</details>
<details>
<summary><code>class DT_EnvProjectedTexture</code></summary>

```
{
	m_hTargetEntity: Int,
	m_bState: Int,
	m_bAlwaysUpdate: Int,
	m_flLightFOV: Float,
	m_bEnableShadows: Int,
	m_bSimpleProjection: Int,
	m_bLightOnlyTarget: Int,
	m_bLightWorld: Int,
	m_bCameraSpace: Int,
	m_flBrightnessScale: Float,
	m_LightColor: Int,
	m_flColorTransitionTime: Float,
	m_flAmbient: Float,
	m_flNearZ: Float,
	m_flFarZ: Float,
	m_SpotlightTextureName: String,
	m_nSpotlightTextureFrame: Int,
	m_nShadowQuality: Int,
	m_iStyle: Int,
	m_flProjectionSize: Float,
	m_flRotation: Float,
}
```

#### Offsets

```
DT_EnvProjectedTexture!0x09dc m_hTargetEntity
DT_EnvProjectedTexture!0x09e0 m_bState
DT_EnvProjectedTexture!0x09e1 m_bAlwaysUpdate
DT_EnvProjectedTexture!0x09e4 m_flLightFOV
DT_EnvProjectedTexture!0x09e8 m_bEnableShadows
DT_EnvProjectedTexture!0x09e9 m_bSimpleProjection
DT_EnvProjectedTexture!0x09ea m_bLightOnlyTarget
DT_EnvProjectedTexture!0x09eb m_bLightWorld
DT_EnvProjectedTexture!0x09ec m_bCameraSpace
DT_EnvProjectedTexture!0x09f0 m_flBrightnessScale
DT_EnvProjectedTexture!0x09f4 m_LightColor
DT_EnvProjectedTexture!0x0a08 m_flColorTransitionTime
DT_EnvProjectedTexture!0x0a0c m_flAmbient
DT_EnvProjectedTexture!0x0a10 m_flNearZ
DT_EnvProjectedTexture!0x0a14 m_flFarZ
DT_EnvProjectedTexture!0x0a18 m_SpotlightTextureName
DT_EnvProjectedTexture!0x0b24 m_nSpotlightTextureFrame
DT_EnvProjectedTexture!0x0b28 m_nShadowQuality
DT_EnvProjectedTexture!0x0b2c m_iStyle
DT_EnvProjectedTexture!0x0b38 m_flProjectionSize
DT_EnvProjectedTexture!0x0b3c m_flRotation
```
</details>
<details>
<summary><code>class DT_EnvWindShared</code></summary>

```
{
	m_flStartTime: Float,
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

#### Offsets

```
DT_EnvWindShared!0x0004 m_flStartTime
DT_EnvWindShared!0x0008 m_iWindSeed
DT_EnvWindShared!0x000c m_iMinWind
DT_EnvWindShared!0x0010 m_iMaxWind
DT_EnvWindShared!0x0018 m_iMinGust
DT_EnvWindShared!0x001c m_iMaxGust
DT_EnvWindShared!0x0020 m_flMinGustDelay
DT_EnvWindShared!0x0024 m_flMaxGustDelay
DT_EnvWindShared!0x0028 m_flGustDuration
DT_EnvWindShared!0x002c m_iGustDirChange
DT_EnvWindShared!0x006c m_iInitialWindDir
DT_EnvWindShared!0x0070 m_flInitialWindSpeed
```
</details>
<details>
<summary><code>class DT_GameRulesProxy</code></summary>

```
{
}
```

#### Offsets

```
```
</details>
<details>
<summary><code>class DT_LightGlow</code></summary>

```
{
	HDRColorScale: Float,
	m_clrRender: Int,
	m_angRotation: Vector,
	m_vecOrigin: Vector,
	moveparent: Int,
	m_nHorizontalSize: Int,
	m_nVerticalSize: Int,
	m_nMinDist: Int,
	m_nMaxDist: Int,
	m_nOuterMaxDist: Int,
	m_spawnflags: Int,
	m_flGlowProxySize: Float,
}
```

#### Offsets

```
DT_LightGlow!0x0000 HDRColorScale
DT_LightGlow!0x0070 m_clrRender
DT_LightGlow!0x012c m_angRotation
DT_LightGlow!0x0138 m_vecOrigin
DT_LightGlow!0x0148 moveparent
DT_LightGlow!0x09d8 m_nHorizontalSize
DT_LightGlow!0x09dc m_nVerticalSize
DT_LightGlow!0x09e0 m_nMinDist
DT_LightGlow!0x09e4 m_nMaxDist
DT_LightGlow!0x09e8 m_nOuterMaxDist
DT_LightGlow!0x09ec m_spawnflags
DT_LightGlow!0x0ac4 m_flGlowProxySize
```
</details>
<details>
<summary><code>class DT_LocalActiveWeaponData</code></summary>

```
{
	m_nNextThinkTick: Int,
	m_flNextPrimaryAttack: Float,
	m_flNextSecondaryAttack: Float,
	m_flTimeWeaponIdle: Float,
}
```

#### Offsets

```
DT_LocalActiveWeaponData!0x00fc m_nNextThinkTick
DT_LocalActiveWeaponData!0x3218 m_flNextPrimaryAttack
DT_LocalActiveWeaponData!0x321c m_flNextSecondaryAttack
DT_LocalActiveWeaponData!0x3254 m_flTimeWeaponIdle
```
</details>
<details>
<summary><code>class DT_LocalPlayerExclusive</code></summary>

```
{
	m_nNextThinkTick: Int,
	m_vecViewOffset[0]: Float,
	m_vecViewOffset[1]: Float,
	m_vecViewOffset[2]: Float,
	m_vecVelocity[0]: Float,
	m_vecVelocity[1]: Float,
	m_vecVelocity[2]: Float,
	m_vecBaseVelocity: Vector,
	m_flFriction: Float,
	m_Local: DataTable,
	m_hTonemapController: Int,
	m_hLastWeapon: Int,
	m_fOnTarget: Int,
	m_hConstraintEntity: Int,
	m_vecConstraintCenter: Vector,
	m_flConstraintRadius: Float,
	m_flConstraintWidth: Float,
	m_flConstraintSpeedFactor: Float,
	m_bConstraintPastRadius: Int,
	m_flDeathTime: Float,
	m_flNextDecalTime: Float,
	m_fForceTeam: Float,
	m_nTickBase: Int,
	m_flLaggedMovementValue: Float,
}
```

#### Offsets

```
DT_LocalPlayerExclusive!0x00fc m_nNextThinkTick
DT_LocalPlayerExclusive!0x0108 m_vecViewOffset[0]
DT_LocalPlayerExclusive!0x010c m_vecViewOffset[1]
DT_LocalPlayerExclusive!0x0110 m_vecViewOffset[2]
DT_LocalPlayerExclusive!0x0114 m_vecVelocity[0]
DT_LocalPlayerExclusive!0x0118 m_vecVelocity[1]
DT_LocalPlayerExclusive!0x011c m_vecVelocity[2]
DT_LocalPlayerExclusive!0x0120 m_vecBaseVelocity
DT_LocalPlayerExclusive!0x0144 m_flFriction
DT_LocalPlayerExclusive!0x2fbc m_Local
DT_LocalPlayerExclusive!0x31cc m_hTonemapController
DT_LocalPlayerExclusive!0x32f4 m_hLastWeapon
DT_LocalPlayerExclusive!0x3324 m_fOnTarget
DT_LocalPlayerExclusive!0x3340 m_hConstraintEntity
DT_LocalPlayerExclusive!0x3344 m_vecConstraintCenter
DT_LocalPlayerExclusive!0x3350 m_flConstraintRadius
DT_LocalPlayerExclusive!0x3354 m_flConstraintWidth
DT_LocalPlayerExclusive!0x3358 m_flConstraintSpeedFactor
DT_LocalPlayerExclusive!0x335c m_bConstraintPastRadius
DT_LocalPlayerExclusive!0x33c0 m_flDeathTime
DT_LocalPlayerExclusive!0x33c4 m_flNextDecalTime
DT_LocalPlayerExclusive!0x33c8 m_fForceTeam
DT_LocalPlayerExclusive!0x342c m_nTickBase
DT_LocalPlayerExclusive!0x3590 m_flLaggedMovementValue
```
</details>
<details>
<summary><code>class DT_MaterialModifyControl</code></summary>

```
{
	m_szMaterialName: String,
	m_szMaterialVar: String,
	m_szMaterialVarValue: String,
	m_iFrameStart: Int,
	m_iFrameEnd: Int,
	m_bWrap: Int,
	m_flFramerate: Float,
	m_bNewAnimCommandsSemaphore: Int,
	m_flFloatLerpStartValue: Float,
	m_flFloatLerpEndValue: Float,
	m_flFloatLerpTransitionTime: Float,
	m_bFloatLerpWrap: Int,
	m_nModifyMode: Int,
}
```

#### Offsets

```
DT_MaterialModifyControl!0x09d8 m_szMaterialName
DT_MaterialModifyControl!0x0ad7 m_szMaterialVar
DT_MaterialModifyControl!0x0bd6 m_szMaterialVarValue
DT_MaterialModifyControl!0x0ce0 m_iFrameStart
DT_MaterialModifyControl!0x0ce4 m_iFrameEnd
DT_MaterialModifyControl!0x0ce8 m_bWrap
DT_MaterialModifyControl!0x0cec m_flFramerate
DT_MaterialModifyControl!0x0cf0 m_bNewAnimCommandsSemaphore
DT_MaterialModifyControl!0x0cf4 m_flFloatLerpStartValue
DT_MaterialModifyControl!0x0cf8 m_flFloatLerpEndValue
DT_MaterialModifyControl!0x0cfc m_flFloatLerpTransitionTime
DT_MaterialModifyControl!0x0d00 m_bFloatLerpWrap
DT_MaterialModifyControl!0x0d08 m_nModifyMode
```
</details>
<details>
<summary><code>class DT_MovieDisplay</code></summary>

```
{
	m_bEnabled: Int,
	m_bLooping: Int,
	m_szMovieFilename: String,
	m_szGroupName: String,
	m_bStretchToFill: Int,
	m_bForcedSlave: Int,
	m_bUseCustomUVs: Int,
	m_flUMin: Float,
	m_flUMax: Float,
	m_flVMin: Float,
	m_flVMax: Float,
}
```

#### Offsets

```
DT_MovieDisplay!0x09d8 m_bEnabled
DT_MovieDisplay!0x09d9 m_bLooping
DT_MovieDisplay!0x09da m_szMovieFilename
DT_MovieDisplay!0x0a5a m_szGroupName
DT_MovieDisplay!0x0ada m_bStretchToFill
DT_MovieDisplay!0x0adb m_bForcedSlave
DT_MovieDisplay!0x0adc m_bUseCustomUVs
DT_MovieDisplay!0x0ae0 m_flUMin
DT_MovieDisplay!0x0ae4 m_flUMax
DT_MovieDisplay!0x0ae8 m_flVMin
DT_MovieDisplay!0x0aec m_flVMax
```
</details>
<details>
<summary><code>class DT_ParadropChopper</code></summary>

```
{
	m_vecOrigin: VectorXY,
	m_vecOrigin[2]: Float,
	m_hCallingPlayer: Int,
}
```

#### Offsets

```
DT_ParadropChopper!0x0138 m_vecOrigin
DT_ParadropChopper!0x0140 m_vecOrigin[2]
DT_ParadropChopper!0x2990 m_hCallingPlayer
```
</details>
<details>
<summary><code>class DT_ParticleProperty</code></summary>

```
{
}
```

#### Offsets

```
```
</details>
<details>
<summary><code>class DT_Plasma</code></summary>

```
{
	m_flStartScale: Float,
	m_flScale: Float,
	m_flScaleTime: Float,
	m_nFlags: Int,
	m_nPlasmaModelIndex: Int,
	m_nPlasmaModelIndex2: Int,
	m_nGlowModelIndex: Int,
}
```

#### Offsets

```
DT_Plasma!0x09d8 m_flStartScale
DT_Plasma!0x09dc m_flScale
DT_Plasma!0x09e0 m_flScaleTime
DT_Plasma!0x09e4 m_nFlags
DT_Plasma!0x09e8 m_nPlasmaModelIndex
DT_Plasma!0x09ec m_nPlasmaModelIndex2
DT_Plasma!0x09f0 m_nGlowModelIndex
```
</details>
<details>
<summary><code>class DT_PlayerPing</code></summary>

```
{
	m_hPlayer: Int,
	m_hPingedEntity: Int,
	m_iType: Int,
}
```

#### Offsets

```
DT_PlayerPing!0x09e4 m_hPlayer
DT_PlayerPing!0x09e8 m_hPingedEntity
DT_PlayerPing!0x09ec m_iType
```
</details>
<details>
<summary><code>class DT_PlayerResource</code></summary>

```
{
}
```

#### Offsets

```
```
</details>
<details>
<summary><code>class DT_PostProcessController</code></summary>

```
{
	m_bMaster: Int,
}
```

#### Offsets

```
DT_PostProcessController!0x0a04 m_bMaster
```
</details>
<details>
<summary><code>class DT_PropVehicleChoreoGeneric</code></summary>

```
{
	m_hPlayer: Int,
	m_bEnterAnimOn: Int,
	m_bExitAnimOn: Int,
	m_vecEyeExitEndpoint: Vector,
	m_bForceEyesToAttachment: Int,
	m_vehicleView.bClampEyeAngles: Int,
	m_vehicleView.flPitchCurveZero: Float,
	m_vehicleView.flPitchCurveLinear: Float,
	m_vehicleView.flRollCurveZero: Float,
	m_vehicleView.flRollCurveLinear: Float,
	m_vehicleView.flFOV: Float,
	m_vehicleView.flYawMin: Float,
	m_vehicleView.flYawMax: Float,
	m_vehicleView.flPitchMin: Float,
	m_vehicleView.flPitchMax: Float,
}
```

#### Offsets

```
DT_PropVehicleChoreoGeneric!0x29f4 m_hPlayer
DT_PropVehicleChoreoGeneric!0x29fc m_bEnterAnimOn
DT_PropVehicleChoreoGeneric!0x29fd m_bExitAnimOn
DT_PropVehicleChoreoGeneric!0x2a00 m_vecEyeExitEndpoint
DT_PropVehicleChoreoGeneric!0x2a0c m_bForceEyesToAttachment
DT_PropVehicleChoreoGeneric!0x2a90 m_vehicleView.bClampEyeAngles
DT_PropVehicleChoreoGeneric!0x2a94 m_vehicleView.flPitchCurveZero
DT_PropVehicleChoreoGeneric!0x2a98 m_vehicleView.flPitchCurveLinear
DT_PropVehicleChoreoGeneric!0x2a9c m_vehicleView.flRollCurveZero
DT_PropVehicleChoreoGeneric!0x2aa0 m_vehicleView.flRollCurveLinear
DT_PropVehicleChoreoGeneric!0x2aa4 m_vehicleView.flFOV
DT_PropVehicleChoreoGeneric!0x2aa8 m_vehicleView.flYawMin
DT_PropVehicleChoreoGeneric!0x2aac m_vehicleView.flYawMax
DT_PropVehicleChoreoGeneric!0x2ab0 m_vehicleView.flPitchMin
DT_PropVehicleChoreoGeneric!0x2ab4 m_vehicleView.flPitchMax
```
</details>
<details>
<summary><code>class DT_Prop_Hallucination</code></summary>

```
{
	m_bEnabled: Int,
	m_fVisibleTime: Float,
	m_fRechargeTime: Float,
}
```

#### Offsets

```
DT_Prop_Hallucination!0x2999 m_bEnabled
DT_Prop_Hallucination!0x299c m_fVisibleTime
DT_Prop_Hallucination!0x29a0 m_fRechargeTime
```
</details>
<details>
<summary><code>class DT_Ragdoll</code></summary>

```
{
	m_ragAngles: Array,
	m_ragPos: Array,
	m_ragPos[0]: Vector,
	m_ragAngles[0]: Vector,
	m_hUnragdoll: Int,
	m_flBlendWeight: Float,
	m_nOverlaySequence: Int,
}
```

#### Offsets

```
DT_Ragdoll!0x0000 m_ragAngles
DT_Ragdoll!0x0000 m_ragPos
DT_Ragdoll!0x2980 m_ragPos[0]
DT_Ragdoll!0x2aa0 m_ragAngles[0]
DT_Ragdoll!0x2c7c m_hUnragdoll
DT_Ragdoll!0x2c80 m_flBlendWeight
DT_Ragdoll!0x2c88 m_nOverlaySequence
```
</details>
<details>
<summary><code>class DT_RocketTrail</code></summary>

```
{
	m_SpawnRate: Float,
	m_StartColor: Vector,
	m_EndColor: Vector,
	m_Opacity: Float,
	m_ParticleLifetime: Float,
	m_StopEmitTime: Float,
	m_MinSpeed: Float,
	m_MaxSpeed: Float,
	m_StartSize: Float,
	m_EndSize: Float,
	m_SpawnRadius: Float,
	m_bEmit: Int,
	m_bDamaged: Int,
	m_nAttachment: Int,
	m_flFlareScale: Float,
}
```

#### Offsets

```
DT_RocketTrail!0x0ac4 m_SpawnRate
DT_RocketTrail!0x0ac8 m_StartColor
DT_RocketTrail!0x0ad4 m_EndColor
DT_RocketTrail!0x0ae0 m_Opacity
DT_RocketTrail!0x0ae4 m_ParticleLifetime
DT_RocketTrail!0x0ae8 m_StopEmitTime
DT_RocketTrail!0x0aec m_MinSpeed
DT_RocketTrail!0x0af0 m_MaxSpeed
DT_RocketTrail!0x0af4 m_StartSize
DT_RocketTrail!0x0af8 m_EndSize
DT_RocketTrail!0x0afc m_SpawnRadius
DT_RocketTrail!0x0b0c m_bEmit
DT_RocketTrail!0x0b0d m_bDamaged
DT_RocketTrail!0x0b10 m_nAttachment
DT_RocketTrail!0x0b20 m_flFlareScale
```
</details>
<details>
<summary><code>class DT_RopeKeyframe</code></summary>

```
{
	m_vecOrigin: Vector,
	moveparent: Int,
	m_iParentAttachment: Int,
	m_nMinCPULevel: Int,
	m_nMaxCPULevel: Int,
	m_nMinGPULevel: Int,
	m_nMaxGPULevel: Int,
	m_flScrollSpeed: Float,
	m_RopeFlags: Int,
	m_iRopeMaterialModelIndex: Int,
	m_iDefaultRopeMaterialModelIndex: Int,
	m_nSegments: Int,
	m_hStartPoint: Int,
	m_hEndPoint: Int,
	m_iStartAttachment: Int,
	m_iEndAttachment: Int,
	m_Subdiv: Int,
	m_RopeLength: Int,
	m_Slack: Int,
	m_TextureScale: Float,
	m_fLockedPoints: Int,
	m_nChangeCount: Int,
	m_Width: Float,
	m_bConstrainBetweenEndpoints: Int,
}
```

#### Offsets

```
DT_RopeKeyframe!0x0138 m_vecOrigin
DT_RopeKeyframe!0x0148 moveparent
DT_RopeKeyframe!0x02ec m_iParentAttachment
DT_RopeKeyframe!0x0988 m_nMinCPULevel
DT_RopeKeyframe!0x0989 m_nMaxCPULevel
DT_RopeKeyframe!0x098a m_nMinGPULevel
DT_RopeKeyframe!0x098b m_nMaxGPULevel
DT_RopeKeyframe!0x0a0c m_flScrollSpeed
DT_RopeKeyframe!0x0a10 m_RopeFlags
DT_RopeKeyframe!0x0a14 m_iRopeMaterialModelIndex
DT_RopeKeyframe!0x0a18 m_iDefaultRopeMaterialModelIndex
DT_RopeKeyframe!0x0ca8 m_nSegments
DT_RopeKeyframe!0x0cac m_hStartPoint
DT_RopeKeyframe!0x0cb0 m_hEndPoint
DT_RopeKeyframe!0x0cb4 m_iStartAttachment
DT_RopeKeyframe!0x0cb6 m_iEndAttachment
DT_RopeKeyframe!0x0cb8 m_Subdiv
DT_RopeKeyframe!0x0cbc m_RopeLength
DT_RopeKeyframe!0x0cc0 m_Slack
DT_RopeKeyframe!0x0cc4 m_TextureScale
DT_RopeKeyframe!0x0cc8 m_fLockedPoints
DT_RopeKeyframe!0x0ccc m_nChangeCount
DT_RopeKeyframe!0x0cd0 m_Width
DT_RopeKeyframe!0x0d50 m_bConstrainBetweenEndpoints
```
</details>
<details>
<summary><code>class DT_SceneEntity</code></summary>

```
{
	m_bIsPlayingBack: Int,
	m_bPaused: Int,
	m_bMultiplayer: Int,
	m_flForceClientTime: Float,
	m_nSceneStringIndex: Int,
}
```

#### Offsets

```
DT_SceneEntity!0x09dc m_bIsPlayingBack
DT_SceneEntity!0x09dd m_bPaused
DT_SceneEntity!0x09de m_bMultiplayer
DT_SceneEntity!0x09e4 m_flForceClientTime
DT_SceneEntity!0x09e8 m_nSceneStringIndex
```
</details>
<details>
<summary><code>class DT_SlideshowDisplay</code></summary>

```
{
	m_bEnabled: Int,
	m_szDisplayText: String,
	m_szSlideshowDirectory: String,
	m_fMinSlideTime: Float,
	m_fMaxSlideTime: Float,
	m_iCycleType: Int,
	m_bNoListRepeats: Int,
}
```

#### Offsets

```
DT_SlideshowDisplay!0x09d8 m_bEnabled
DT_SlideshowDisplay!0x09d9 m_szDisplayText
DT_SlideshowDisplay!0x0a59 m_szSlideshowDirectory
DT_SlideshowDisplay!0x0b08 m_fMinSlideTime
DT_SlideshowDisplay!0x0b0c m_fMaxSlideTime
DT_SlideshowDisplay!0x0b14 m_iCycleType
DT_SlideshowDisplay!0x0b18 m_bNoListRepeats
```
</details>
<details>
<summary><code>class DT_SmokeTrail</code></summary>

```
{
	m_SpawnRate: Float,
	m_StartColor: Vector,
	m_EndColor: Vector,
	m_Opacity: Float,
	m_ParticleLifetime: Float,
	m_StopEmitTime: Float,
	m_MinSpeed: Float,
	m_MaxSpeed: Float,
	m_MinDirectedSpeed: Float,
	m_MaxDirectedSpeed: Float,
	m_StartSize: Float,
	m_EndSize: Float,
	m_SpawnRadius: Float,
	m_bEmit: Int,
	m_nAttachment: Int,
}
```

#### Offsets

```
DT_SmokeTrail!0x0ac4 m_SpawnRate
DT_SmokeTrail!0x0ac8 m_StartColor
DT_SmokeTrail!0x0ad4 m_EndColor
DT_SmokeTrail!0x0ae0 m_Opacity
DT_SmokeTrail!0x0ae4 m_ParticleLifetime
DT_SmokeTrail!0x0ae8 m_StopEmitTime
DT_SmokeTrail!0x0aec m_MinSpeed
DT_SmokeTrail!0x0af0 m_MaxSpeed
DT_SmokeTrail!0x0af4 m_MinDirectedSpeed
DT_SmokeTrail!0x0af8 m_MaxDirectedSpeed
DT_SmokeTrail!0x0afc m_StartSize
DT_SmokeTrail!0x0b00 m_EndSize
DT_SmokeTrail!0x0b04 m_SpawnRadius
DT_SmokeTrail!0x0b14 m_bEmit
DT_SmokeTrail!0x0b18 m_nAttachment
```
</details>
<details>
<summary><code>class DT_SporeExplosion</code></summary>

```
{
	m_flSpawnRate: Float,
	m_flParticleLifetime: Float,
	m_flStartSize: Float,
	m_flEndSize: Float,
	m_flSpawnRadius: Float,
	m_bEmit: Int,
	m_bDontRemove: Int,
}
```

#### Offsets

```
DT_SporeExplosion!0x0ac4 m_flSpawnRate
DT_SporeExplosion!0x0ac8 m_flParticleLifetime
DT_SporeExplosion!0x0acc m_flStartSize
DT_SporeExplosion!0x0ad0 m_flEndSize
DT_SporeExplosion!0x0ad4 m_flSpawnRadius
DT_SporeExplosion!0x0adc m_bEmit
DT_SporeExplosion!0x0add m_bDontRemove
```
</details>
<details>
<summary><code>class DT_SporeTrail</code></summary>

```
{
	m_vecEndColor: Vector,
	m_flSpawnRate: Float,
	m_flParticleLifetime: Float,
	m_flStartSize: Float,
	m_flEndSize: Float,
	m_flSpawnRadius: Float,
	m_bEmit: Int,
}
```

#### Offsets

```
DT_SporeTrail!0x0ac0 m_vecEndColor
DT_SporeTrail!0x0acc m_flSpawnRate
DT_SporeTrail!0x0ad0 m_flParticleLifetime
DT_SporeTrail!0x0ad4 m_flStartSize
DT_SporeTrail!0x0ad8 m_flEndSize
DT_SporeTrail!0x0adc m_flSpawnRadius
DT_SporeTrail!0x0aec m_bEmit
```
</details>
<details>
<summary><code>class DT_Sprite</code></summary>

```
{
	m_hAttachedToEntity: Int,
	m_nAttachment: Int,
	m_flSpriteFramerate: Float,
	m_flFrame: Float,
	m_nBrightness: Int,
	m_flBrightnessTime: Float,
	m_flSpriteScale: Float,
	m_flScaleTime: Float,
	m_bWorldSpaceScale: Int,
	m_flGlowProxySize: Float,
	m_flHDRColorScale: Float,
}
```

#### Offsets

```
DT_Sprite!0x09e8 m_hAttachedToEntity
DT_Sprite!0x09ec m_nAttachment
DT_Sprite!0x09f0 m_flSpriteFramerate
DT_Sprite!0x09f4 m_flFrame
DT_Sprite!0x09fc m_nBrightness
DT_Sprite!0x0a00 m_flBrightnessTime
DT_Sprite!0x0a04 m_flSpriteScale
DT_Sprite!0x0a08 m_flScaleTime
DT_Sprite!0x0a0c m_bWorldSpaceScale
DT_Sprite!0x0a10 m_flGlowProxySize
DT_Sprite!0x0a14 m_flHDRColorScale
```
</details>
<details>
<summary><code>class DT_SteamJet</code></summary>

```
{
	m_SpreadSpeed: Float,
	m_Speed: Float,
	m_StartSize: Float,
	m_EndSize: Float,
	m_Rate: Float,
	m_JetLength: Float,
	m_bEmit: Int,
	m_nType: Int,
	m_bFaceLeft: Int,
	m_spawnflags: Int,
	m_flRollSpeed: Float,
}
```

#### Offsets

```
DT_SteamJet!0x0ac4 m_SpreadSpeed
DT_SteamJet!0x0ac8 m_Speed
DT_SteamJet!0x0acc m_StartSize
DT_SteamJet!0x0ad0 m_EndSize
DT_SteamJet!0x0ad4 m_Rate
DT_SteamJet!0x0ad8 m_JetLength
DT_SteamJet!0x0adc m_bEmit
DT_SteamJet!0x0ae0 m_nType
DT_SteamJet!0x0ae4 m_bFaceLeft
DT_SteamJet!0x0ae8 m_spawnflags
DT_SteamJet!0x0aec m_flRollSpeed
```
</details>
<details>
<summary><code>class DT_TEBSPDecal</code></summary>

```
{
	m_vecOrigin: Vector,
	m_nEntity: Int,
	m_nIndex: Int,
}
```

#### Offsets

```
DT_TEBSPDecal!0x0010 m_vecOrigin
DT_TEBSPDecal!0x001c m_nEntity
DT_TEBSPDecal!0x0020 m_nIndex
```
</details>
<details>
<summary><code>class DT_TEBeamRingPoint</code></summary>

```
{
	m_vecCenter: Vector,
	m_flStartRadius: Float,
	m_flEndRadius: Float,
}
```

#### Offsets

```
DT_TEBeamRingPoint!0x004c m_vecCenter
DT_TEBeamRingPoint!0x0058 m_flStartRadius
DT_TEBeamRingPoint!0x005c m_flEndRadius
```
</details>
<details>
<summary><code>class DT_TEBreakModel</code></summary>

```
{
	m_vecOrigin: Vector,
	m_angRotation[0]: Float,
	m_angRotation[1]: Float,
	m_angRotation[2]: Float,
	m_vecSize: Vector,
	m_vecVelocity: Vector,
	m_nRandomization: Int,
	m_nModelIndex: Int,
	m_nCount: Int,
	m_fTime: Float,
	m_nFlags: Int,
}
```

#### Offsets

```
DT_TEBreakModel!0x0010 m_vecOrigin
DT_TEBreakModel!0x001c m_angRotation[0]
DT_TEBreakModel!0x0020 m_angRotation[1]
DT_TEBreakModel!0x0024 m_angRotation[2]
DT_TEBreakModel!0x0028 m_vecSize
DT_TEBreakModel!0x0034 m_vecVelocity
DT_TEBreakModel!0x0040 m_nRandomization
DT_TEBreakModel!0x0044 m_nModelIndex
DT_TEBreakModel!0x0048 m_nCount
DT_TEBreakModel!0x004c m_fTime
DT_TEBreakModel!0x0050 m_nFlags
```
</details>
<details>
<summary><code>class DT_TEDust</code></summary>

```
{
	m_flSize: Float,
	m_flSpeed: Float,
	m_vecDirection: Vector,
}
```

#### Offsets

```
DT_TEDust!0x001c m_flSize
DT_TEDust!0x0020 m_flSpeed
DT_TEDust!0x0024 m_vecDirection
```
</details>
<details>
<summary><code>class DT_TEParticleSystem</code></summary>

```
{
	m_vecOrigin[0]: Float,
	m_vecOrigin[1]: Float,
	m_vecOrigin[2]: Float,
}
```

#### Offsets

```
DT_TEParticleSystem!0x0010 m_vecOrigin[0]
DT_TEParticleSystem!0x0014 m_vecOrigin[1]
DT_TEParticleSystem!0x0018 m_vecOrigin[2]
```
</details>
<details>
<summary><code>class DT_TESparks</code></summary>

```
{
	m_nMagnitude: Int,
	m_nTrailLength: Int,
	m_vecDir: Vector,
}
```

#### Offsets

```
DT_TESparks!0x001c m_nMagnitude
DT_TESparks!0x0020 m_nTrailLength
DT_TESparks!0x0024 m_vecDir
```
</details>
<details>
<summary><code>class DT_VGuiScreen</code></summary>

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

#### Offsets

```
DT_VGuiScreen!0x09e0 m_flWidth
DT_VGuiScreen!0x09e4 m_flHeight
DT_VGuiScreen!0x09e8 m_nPanelName
DT_VGuiScreen!0x0a04 m_nAttachmentIndex
DT_VGuiScreen!0x0a08 m_nOverlayMaterial
DT_VGuiScreen!0x0a0c m_fScreenFlags
DT_VGuiScreen!0x0a68 m_hPlayerOwner
```
</details>
<details>
<summary><code>class DT_VoteController</code></summary>

```
{
	m_iActiveIssueIndex: Int,
	m_iOnlyTeamToVote: Int,
	m_nPotentialVotes: Int,
	m_bIsYesNoVote: Int,
}
```

#### Offsets

```
DT_VoteController!0x09e4 m_iActiveIssueIndex
DT_VoteController!0x09e8 m_iOnlyTeamToVote
DT_VoteController!0x0a04 m_nPotentialVotes
DT_VoteController!0x0a0a m_bIsYesNoVote
```
</details>

### WeaponData

```
class WeaponInfo {
	primary clip size: Int,
	secondary clip size: Int,
	primary default clip size: Int,
	secondary default clip size: Int,
	primary reserve ammo max: Int,
	secondary reserve ammo max: Int,
	allow hand flipping: Bool,
	model right handed: Bool,
	is melee weapon: Bool,
	weapon weight: Int,
	rumble effect: Int,
}
class CSWeaponInfo extends WeaponInfo {
	in game price: Int,
	kill award: Int,
	cycletime: Float,
	cycletime alt: Float,
	time to idle: Float,
	idle interval: Float,
	is full auto: Bool,
	damage: Int,
	armor ratio: Float,
	bullets: Int,
	penetration: Float,
	flinch velocity modifier large: Float,
	flinch velocity modifier small: Float,
	range: Float,
	range modifier: Float,
	throw velocity: Float,
	has silencer: Int,
	silencer model: Int,
	crosshair min distance: Int,
	crosshair delta distance: Int,
	max player speed: Float,
	max player speed alt: Float,
	attack movespeed factor: Float,
	spread: Float,
	spread alt: Float,
	inaccuracy crouch: Float,
	inaccuracy crouch alt: Float,
	inaccuracy stand: Float,
	inaccuracy stand alt: Float,
	inaccuracy jump initial: Float,
	inaccuracy jump: Float,
	inaccuracy jump alt: Float,
	inaccuracy land: Float,
	inaccuracy land alt: Float,
	inaccuracy ladder: Float,
	inaccuracy ladder alt: Float,
	inaccuracy fire: Float,
	inaccuracy fire alt: Float,
	inaccuracy move: Float,
	inaccuracy move alt: Float,
	inaccuracy reload: Float,
	recoil seed: Int,
	recoil angle: Float,
	recoil angle alt: Float,
	recoil angle variance: Float,
	recoil angle variance alt: Float,
	recoil magnitude: Float,
	recoil magnitude alt: Float,
	recoil magnitude variance: Float,
	recoil magnitude variance alt: Float,
	spread seed: Int,
	recovery time crouch: Float,
	recovery time stand: Float,
	recovery time crouch final: Float,
	recovery time stand final: Float,
	recovery transition start bullet: Int,
	recovery transition end bullet: Int,
	unzoom after shot: Bool,
	hide view model zoomed: Bool,
	zoom levels: Int,
	zoom fov 1: Int,
	zoom fov 2: Int,
	zoom time 0: Float,
	zoom time 1: Float,
	zoom time 2: Float,
	addon scale: Float,
	tracer frequency: Int,
	tracer frequency alt: Int,
	heat per shot: Float,
	inaccuracy pitch shift: Float,
	inaccuracy alt sound threshold: Float,
	bot audible range: Float,
	wrong team msg: Int,
	has burst mode: Bool,
	is revolver: Bool,
	cannot shoot underwater: Bool,
}
```

#### Offsets

```
WeaponInfo!0x0014 primary clip size
WeaponInfo!0x0018 secondary clip size
WeaponInfo!0x001c primary default clip size
WeaponInfo!0x0020 secondary default clip size
WeaponInfo!0x0024 primary reserve ammo max
WeaponInfo!0x0028 secondary reserve ammo max
WeaponInfo!0x0090 allow hand flipping
WeaponInfo!0x0091 model right handed
WeaponInfo!0x0092 is melee weapon
WeaponInfo!0x009c weapon weight
WeaponInfo!0x00c0 rumble effect
CSWeaponInfo!0x00d0 in game price
CSWeaponInfo!0x00d4 kill award
CSWeaponInfo!0x00dc cycletime
CSWeaponInfo!0x00e0 cycletime alt
CSWeaponInfo!0x00e4 time to idle
CSWeaponInfo!0x00e8 idle interval
CSWeaponInfo!0x00ec is full auto
CSWeaponInfo!0x00f0 damage
CSWeaponInfo!0x00f4 armor ratio
CSWeaponInfo!0x00f8 bullets
CSWeaponInfo!0x00fc penetration
CSWeaponInfo!0x0100 flinch velocity modifier large
CSWeaponInfo!0x0104 flinch velocity modifier small
CSWeaponInfo!0x0108 range
CSWeaponInfo!0x010c range modifier
CSWeaponInfo!0x0110 throw velocity
CSWeaponInfo!0x0120 has silencer
CSWeaponInfo!0x0124 silencer model
CSWeaponInfo!0x0128 crosshair min distance
CSWeaponInfo!0x012c crosshair delta distance
CSWeaponInfo!0x0130 max player speed
CSWeaponInfo!0x0134 max player speed alt
CSWeaponInfo!0x0138 attack movespeed factor
CSWeaponInfo!0x013c spread
CSWeaponInfo!0x0140 spread alt
CSWeaponInfo!0x0144 inaccuracy crouch
CSWeaponInfo!0x0148 inaccuracy crouch alt
CSWeaponInfo!0x014c inaccuracy stand
CSWeaponInfo!0x0150 inaccuracy stand alt
CSWeaponInfo!0x0154 inaccuracy jump initial
CSWeaponInfo!0x0158 inaccuracy jump
CSWeaponInfo!0x015c inaccuracy jump alt
CSWeaponInfo!0x0160 inaccuracy land
CSWeaponInfo!0x0164 inaccuracy land alt
CSWeaponInfo!0x0168 inaccuracy ladder
CSWeaponInfo!0x016c inaccuracy ladder alt
CSWeaponInfo!0x0170 inaccuracy fire
CSWeaponInfo!0x0174 inaccuracy fire alt
CSWeaponInfo!0x0178 inaccuracy move
CSWeaponInfo!0x017c inaccuracy move alt
CSWeaponInfo!0x0180 inaccuracy reload
CSWeaponInfo!0x0184 recoil seed
CSWeaponInfo!0x0188 recoil angle
CSWeaponInfo!0x018c recoil angle alt
CSWeaponInfo!0x0190 recoil angle variance
CSWeaponInfo!0x0194 recoil angle variance alt
CSWeaponInfo!0x0198 recoil magnitude
CSWeaponInfo!0x019c recoil magnitude alt
CSWeaponInfo!0x01a0 recoil magnitude variance
CSWeaponInfo!0x01a4 recoil magnitude variance alt
CSWeaponInfo!0x01a8 spread seed
CSWeaponInfo!0x01ac recovery time crouch
CSWeaponInfo!0x01b0 recovery time stand
CSWeaponInfo!0x01b4 recovery time crouch final
CSWeaponInfo!0x01b8 recovery time stand final
CSWeaponInfo!0x01bc recovery transition start bullet
CSWeaponInfo!0x01c0 recovery transition end bullet
CSWeaponInfo!0x01c4 unzoom after shot
CSWeaponInfo!0x01c5 hide view model zoomed
CSWeaponInfo!0x01c8 zoom levels
CSWeaponInfo!0x01cc zoom fov 1
CSWeaponInfo!0x01d0 zoom fov 2
CSWeaponInfo!0x01d4 zoom time 0
CSWeaponInfo!0x01d8 zoom time 1
CSWeaponInfo!0x01dc zoom time 2
CSWeaponInfo!0x01e8 addon scale
CSWeaponInfo!0x01f4 tracer frequency
CSWeaponInfo!0x01f8 tracer frequency alt
CSWeaponInfo!0x0210 heat per shot
CSWeaponInfo!0x021c inaccuracy pitch shift
CSWeaponInfo!0x0220 inaccuracy alt sound threshold
CSWeaponInfo!0x0224 bot audible range
CSWeaponInfo!0x0230 wrong team msg
CSWeaponInfo!0x0234 has burst mode
CSWeaponInfo!0x0235 is revolver
CSWeaponInfo!0x0236 cannot shoot underwater
```

## Server.dll

### Interfaces

```
server.dll!0x009a6fac BotManager001
server.dll!0x00a5fb68 GameMovement001
server.dll!0x00aa8668 HLTVDirector001
server.dll!0x0099d4d8 IEffects001
server.dll!0x009a6fb0 PlayerInfoManager001
server.dll!0x009a6fa8 PlayerInfoManager002
server.dll!0x009a6fd8 PluginHelpersCheck001
server.dll!0x009a2e20 ServerGameClients004
server.dll!0x009a2ef0 ServerGameDLL005
server.dll!0x009a2db8 ServerGameEnts001
server.dll!0x009b8f8c ServerGameTags001
server.dll!0x009b9e18 ServerGameTags001
server.dll!0x009a3db8 ServerGameTags001
server.dll!0x009a5e70 ServerGameTags001
server.dll!0x009a7fe8 ServerGameTags001
server.dll!0x009a8194 ServerGameTags001
server.dll!0x009a8700 ServerGameTags001
server.dll!0x009a8910 ServerGameTags001
server.dll!0x009a8954 ServerGameTags001
server.dll!0x009a9204 ServerGameTags001
server.dll!0x009a9ac8 ServerGameTags001
server.dll!0x009aa1cc ServerGameTags001
server.dll!0x009aa1ec ServerGameTags001
server.dll!0x009aa420 ServerGameTags001
server.dll!0x0099ab20 ServerGameTags001
server.dll!0x009aad48 ServerGameTags001
server.dll!0x009b3da0 ServerGameTags001
server.dll!0x0099c5bc ServerGameTags001
server.dll!0x009b4340 ServerGameTags001
server.dll!0x009b53b8 ServerGameTags001
server.dll!0x009b5438 ServerGameTags001
server.dll!0x009bd524 ServerGameTags001
server.dll!0x009b8d6c ServerGameTags001
server.dll!0x009b8e68 ServerGameTags001
server.dll!0x009b8ec4 ServerGameTags001
server.dll!0x009b8f30 ServerGameTags001
server.dll!0x009b8f4c ServerGameTags001
server.dll!0x009b8f5c ServerGameTags001
server.dll!0x009b8f6c ServerGameTags001
server.dll!0x009b8f7c ServerGameTags001
server.dll!0x0099c53c ServerGameTags001
server.dll!0x009b8f9c ServerGameTags001
server.dll!0x009b9ab0 ServerGameTags001
server.dll!0x009b9ac0 ServerGameTags001
server.dll!0x009b9e08 ServerGameTags001
server.dll!0x009a2934 ServerGameTags001
server.dll!0x009b9e28 ServerGameTags001
server.dll!0x009b9e38 ServerGameTags001
server.dll!0x009b9e48 ServerGameTags001
server.dll!0x009b9e58 ServerGameTags001
server.dll!0x009b9f70 ServerGameTags001
server.dll!0x009b9f80 ServerGameTags001
server.dll!0x009b9f90 ServerGameTags001
server.dll!0x009b9fa0 ServerGameTags001
server.dll!0x009b9fb0 ServerGameTags001
server.dll!0x009b9fc0 ServerGameTags001
server.dll!0x009b9fd0 ServerGameTags001
server.dll!0x009b9fe0 ServerGameTags001
server.dll!0x009b9ff0 ServerGameTags001
server.dll!0x009ba000 ServerGameTags001
server.dll!0x009ba380 ServerGameTags001
server.dll!0x009ba390 ServerGameTags001
server.dll!0x009ba3a0 ServerGameTags001
server.dll!0x009ba3b0 ServerGameTags001
server.dll!0x009ba3c0 ServerGameTags001
server.dll!0x009ba3d0 ServerGameTags001
server.dll!0x009ba3e0 ServerGameTags001
server.dll!0x009ba3f0 ServerGameTags001
server.dll!0x009ba400 ServerGameTags001
server.dll!0x009ba468 ServerGameTags001
server.dll!0x009ba478 ServerGameTags001
server.dll!0x009ba488 ServerGameTags001
server.dll!0x009ba498 ServerGameTags001
server.dll!0x009ba4a8 ServerGameTags001
server.dll!0x009ba4b8 ServerGameTags001
server.dll!0x009ba4c8 ServerGameTags001
server.dll!0x009bac28 ServerGameTags001
server.dll!0x009bbff0 ServerGameTags001
server.dll!0x009bc574 ServerGameTags001
server.dll!0x00aa9040 VENGINE_GAMETYPES_VERSION002
server.dll!0x009bfbe4 VSERVERTOOLS001
server.dll!0x00a34eac VServerDllSharedAppSystems001
```

### ConVars

<details>
<summary><code>BlendBonesMode</code></summary>

default: `"2"`  
flags: `0x2000`  
</details>
<details>
<summary><code>CS_WarnFriendlyDamageInterval</code></summary>

Defines how frequently the server notifies clients that a player damaged a friend

default: `"3.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>achievement_debug</code></summary>

Turn on achievement debug msgs.

default: `"0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>achievement_disable</code></summary>

Turn off achievements.

default: `"0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>ai_LOS_mode</code></summary>

Tests for scripted sequences that are embedded in the world. Run through your map with this set to check for NPCs falling through the world.

default: `"0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ai_debug_shoot_positions</code></summary>

default: `"0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>ai_drawbattlelines</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>ai_report_task_timings_on_limit</code></summary>

frametime limit for min efficiency AIE_NORMAL (in sec's).

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>ai_shot_bias_max</code></summary>

default: `"1.0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ai_shot_bias_min</code></summary>

default: `"-1.0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ai_think_limit_label</code></summary>

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>ai_vehicle_avoidance</code></summary>

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>ammo_338mag_headshot_mult</code></summary>

You must enable tweaking via tweak_ammo_impulses to use this value.

default: `"1.0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_338mag_impulse</code></summary>

You must enable tweaking via tweak_ammo_impulses to use this value.

default: `"2800"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_338mag_max</code></summary>

default: `"30"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ammo_357sig_headshot_mult</code></summary>

You must enable tweaking via tweak_ammo_impulses to use this value.

default: `"1.0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_357sig_impulse</code></summary>

You must enable tweaking via tweak_ammo_impulses to use this value.

default: `"2000"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_357sig_max</code></summary>

default: `"52"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ammo_357sig_min_max</code></summary>

default: `"12"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ammo_357sig_p250_max</code></summary>

default: `"26"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ammo_357sig_small_max</code></summary>

default: `"24"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ammo_45acp_headshot_mult</code></summary>

You must enable tweaking via tweak_ammo_impulses to use this value.

default: `"1.0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_45acp_impulse</code></summary>

You must enable tweaking via tweak_ammo_impulses to use this value.

default: `"2100"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_45acp_max</code></summary>

default: `"100"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ammo_50AE_headshot_mult</code></summary>

You must enable tweaking via tweak_ammo_impulses to use this value.

default: `"1.0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_50AE_impulse</code></summary>

You must enable tweaking via tweak_ammo_impulses to use this value.

default: `"2400"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_50AE_max</code></summary>

default: `"35"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ammo_556mm_box_headshot_mult</code></summary>

You must enable tweaking via tweak_ammo_impulses to use this value.

default: `"1.0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_556mm_box_impulse</code></summary>

You must enable tweaking via tweak_ammo_impulses to use this value.

default: `"2400"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_556mm_box_max</code></summary>

default: `"200"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ammo_556mm_headshot_mult</code></summary>

You must enable tweaking via tweak_ammo_impulses to use this value.

default: `"1.0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_556mm_impulse</code></summary>

You must enable tweaking via tweak_ammo_impulses to use this value.

default: `"2400"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_556mm_max</code></summary>

default: `"90"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ammo_556mm_small_max</code></summary>

default: `"40"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ammo_57mm_headshot_mult</code></summary>

You must enable tweaking via tweak_ammo_impulses to use this value.

default: `"1.0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_57mm_impulse</code></summary>

You must enable tweaking via tweak_ammo_impulses to use this value.

default: `"2000"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_57mm_max</code></summary>

default: `"100"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ammo_762mm_headshot_mult</code></summary>

You must enable tweaking via tweak_ammo_impulses to use this value.

default: `"1.0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_762mm_impulse</code></summary>

You must enable tweaking via tweak_ammo_impulses to use this value.

default: `"2400"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_762mm_max</code></summary>

default: `"90"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ammo_9mm_headshot_mult</code></summary>

You must enable tweaking via tweak_ammo_impulses to use this value.

default: `"1.0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_9mm_impulse</code></summary>

You must enable tweaking via tweak_ammo_impulses to use this value.

default: `"2000"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_9mm_max</code></summary>

default: `"120"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ammo_buckshot_headshot_mult</code></summary>

You must enable tweaking via tweak_ammo_impulses to use this value.

default: `"1.0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_buckshot_impulse</code></summary>

You must enable tweaking via tweak_ammo_impulses to use this value.

default: `"600"`  
flags: `0x2000`  
</details>
<details>
<summary><code>ammo_buckshot_max</code></summary>

default: `"32"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ammo_grenade_limit_breachcharge</code></summary>

default: `"3"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ammo_grenade_limit_bumpmine</code></summary>

default: `"3"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ammo_grenade_limit_default</code></summary>

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ammo_grenade_limit_flashbang</code></summary>

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ammo_grenade_limit_snowballs</code></summary>

default: `"3"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ammo_grenade_limit_total</code></summary>

default: `"3"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ammo_item_limit_healthshot</code></summary>

default: `"4"`  
flags: `0x82000`  
</details>
<details>
<summary><code>anim_3wayblend</code></summary>

Toggle the 3-way animation blending code.

default: `"1"`  
flags: `0x2000`  
</details>
<details>
<summary><code>anim_twistbones_enabled</code></summary>

Enable procedural twist bones.

default: `"0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>bot_allow_grenades</code></summary>

If nonzero, bots may use grenades.

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>bot_allow_machine_guns</code></summary>

If nonzero, bots may use the machine gun.

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>bot_allow_pistols</code></summary>

If nonzero, bots may use pistols.

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>bot_allow_rifles</code></summary>

If nonzero, bots may use rifles.

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>bot_allow_rogues</code></summary>

If nonzero, bots may occasionally go 'rogue'. Rogue bots do not obey radio commands, nor pursue scenario goals.

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>bot_allow_shotguns</code></summary>

If nonzero, bots may use shotguns.

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>bot_allow_snipers</code></summary>

If nonzero, bots may use sniper rifles.

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>bot_allow_sub_machine_guns</code></summary>

If nonzero, bots may use sub-machine guns.

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>bot_auto_follow</code></summary>

If nonzero, bots with high co-op may automatically follow a nearby human player.

default: `"0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>bot_auto_vacate</code></summary>

If nonzero, bots will automatically leave to make room for human players.

default: `"1"`  
flags: `0x2000`  
</details>
<details>
<summary><code>bot_autodifficulty_threshold_high</code></summary>

Upper bound above Average Human Contribution Score that a bot must be above to change its difficulty

default: `"5.0"`  
flags: `0x82000`  
min value: `-20`  
max value: `20`  
</details>
<details>
<summary><code>bot_autodifficulty_threshold_low</code></summary>

Lower bound below Average Human Contribution Score that a bot must be below to change its difficulty

default: `"-2.0"`  
flags: `0x82000`  
min value: `-20`  
max value: `20`  
</details>
<details>
<summary><code>bot_chatter</code></summary>

Control how bots talk. Allowed values: 'off', 'radio', 'minimal', or 'normal'.

default: `"normal"`  
flags: `0x82000`  
</details>
<details>
<summary><code>bot_controllable</code></summary>

Determines whether bots can be controlled by players

default: `"1"`  
flags: `0x2000`  
</details>
<details>
<summary><code>bot_coop_force_throw_grenade_chance</code></summary>

default: `"0.3"`  
flags: `0x4000`  
</details>
<details>
<summary><code>bot_coop_idle_max_vision_distance</code></summary>

Max distance bots can see targets (in coop) when they are idle, dormant, hiding or asleep.

default: `"1400"`  
flags: `0x6000`  
min value: `-1`  
</details>
<details>
<summary><code>bot_crouch</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>bot_debug</code></summary>

For internal testing purposes.

default: `"0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>bot_debug_target</code></summary>

For internal testing purposes.

default: `"0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>bot_defer_to_human_goals</code></summary>

If nonzero and there is a human on the team, the bots will not do the scenario tasks.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>bot_defer_to_human_items</code></summary>

If nonzero and there is a human on the team, the bots will not get scenario items.

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>bot_difficulty</code></summary>

Defines the skill of bots joining the game.  Values are: 0=easy, 1=normal, 2=hard, 3=expert.

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>bot_dont_shoot</code></summary>

If nonzero, bots will not fire weapons (for debugging).

default: `"0"`  
flags: `0x86000`  
</details>
<details>
<summary><code>bot_eco_limit</code></summary>

If nonzero, bots will not buy if their money falls below this amount.

default: `"2000"`  
flags: `0x2000`  
</details>
<details>
<summary><code>bot_flipout</code></summary>

If nonzero, bots use no CPU for AI. Instead, they run around randomly.

default: `"0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>bot_freeze</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>bot_ignore_players</code></summary>

Bots will not see non-bot players.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>bot_join_after_player</code></summary>

If nonzero, bots wait until a player joins before entering the game.

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>bot_join_team</code></summary>

Determines the team bots will join into. Allowed values: 'any', 'T', or 'CT'.

default: `"any"`  
flags: `0x82000`  
</details>
<details>
<summary><code>bot_loadout</code></summary>

bots are given these items at round start

default: `""`  
flags: `0x4000`  
</details>
<details>
<summary><code>bot_max_vision_distance_override</code></summary>

Max distance bots can see targets.

default: `"-1"`  
flags: `0x6000`  
min value: `-1`  
</details>
<details>
<summary><code>bot_mimic</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>bot_mimic_yaw_offset</code></summary>

default: `"180"`  
flags: `0x4000`  
</details>
<details>
<summary><code>bot_profile_db</code></summary>

The filename from which bot profiles will be read.

default: `"BotProfile.db"`  
flags: `0x2000`  
</details>
<details>
<summary><code>bot_quota</code></summary>

Determines the total number of bots in the game.

default: `"10"`  
flags: `0x82000`  
</details>
<details>
<summary><code>bot_quota_mode</code></summary>

Determines the type of quota.
Allowed values: 'normal', 'fill', and 'match'.
If 'fill', the server will adjust bots to keep N players in the game, where N is bot_quota.
If 'match', the server will maintain a 1:N ratio of humans to bots, where N is bot_quota.

default: `"normal"`  
flags: `0x82000`  
</details>
<details>
<summary><code>bot_randombuy</code></summary>

should bots ignore their prefered weapons and just buy weapons at random?

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>bot_show_battlefront</code></summary>

Show areas where rushing players will initially meet.

default: `"0"`  
flags: `0x4004`  
</details>
<details>
<summary><code>bot_show_nav</code></summary>

For internal testing purposes.

default: `"0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>bot_show_occupy_time</code></summary>

Show when each nav area can first be reached by each team.

default: `"0"`  
flags: `0x4004`  
</details>
<details>
<summary><code>bot_stop</code></summary>

If nonzero, immediately stops all bot processing.

default: `"0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>bot_traceview</code></summary>

For internal testing purposes.

default: `"0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>bot_walk</code></summary>

If nonzero, bots can only walk, not run.

default: `"0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>bot_zombie</code></summary>

If nonzero, bots will stay in idle mode and not attack.

default: `"0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>cash_player_bomb_defused</code></summary>

default: `"300"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_player_bomb_planted</code></summary>

default: `"300"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_player_damage_hostage</code></summary>

default: `"-30"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_player_get_killed</code></summary>

default: `"0"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_player_interact_with_hostage</code></summary>

default: `"150"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_player_killed_enemy_default</code></summary>

default: `"300"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_player_killed_enemy_factor</code></summary>

default: `"1"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_player_killed_hostage</code></summary>

default: `"-1000"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_player_killed_teammate</code></summary>

default: `"-300"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_player_rescued_hostage</code></summary>

default: `"1000"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_player_respawn_amount</code></summary>

default: `"0"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_team_elimination_bomb_map</code></summary>

default: `"3250"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_team_elimination_hostage_map_ct</code></summary>

default: `"2000"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_team_elimination_hostage_map_t</code></summary>

default: `"1000"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_team_hostage_alive</code></summary>

default: `"0"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_team_hostage_interaction</code></summary>

default: `"500"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_team_loser_bonus</code></summary>

default: `"1400"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_team_loser_bonus_consecutive_rounds</code></summary>

default: `"500"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_team_planted_bomb_but_defused</code></summary>

default: `"800"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_team_rescued_hostage</code></summary>

default: `"0"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_team_survive_guardian_wave</code></summary>

default: `"1000"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_team_terrorist_win_bomb</code></summary>

default: `"3500"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_team_win_by_defusing_bomb</code></summary>

default: `"3250"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_team_win_by_hostage_rescue</code></summary>

default: `"3500"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_team_win_by_time_running_out_bomb</code></summary>

default: `"3250"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_team_win_by_time_running_out_hostage</code></summary>

default: `"3250"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cash_team_winner_bonus_consecutive_rounds</code></summary>

default: `"0"`  
flags: `0x82100`  
</details>
<details>
<summary><code>cc_showmissing</code></summary>

Show missing closecaption entries.

default: `"0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>chet_debug_idle</code></summary>

If set one, many debug prints to help track down the TLK_IDLE issue. Set two for super verbose info

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>choreo_spew_filter</code></summary>

Spew choreo. Use a sub-string or * to display all events.

default: `""`  
flags: `0x2000`  
</details>
<details>
<summary><code>cl_remove_old_ugc_downloads</code></summary>

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>cl_simdbones</code></summary>

Use SIMD bone setup.

default: `"0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>cl_use_simd_bones</code></summary>

1 use SIMD bones 0 use scalar bones.

default: `"1"`  
flags: `0x2000`  
</details>
<details>
<summary><code>contributionscore_assist</code></summary>

amount of contribution score added for an assist

default: `"1"`  
flags: `0x80004`  
</details>
<details>
<summary><code>contributionscore_bomb_defuse_major</code></summary>

amount of contribution score for defusing a bomb while at least one enemy remains alive

default: `"3"`  
flags: `0x80004`  
</details>
<details>
<summary><code>contributionscore_bomb_defuse_minor</code></summary>

amount of contribution score for defusing a bomb after eliminating enemy team

default: `"1"`  
flags: `0x80004`  
</details>
<details>
<summary><code>contributionscore_bomb_exploded</code></summary>

amount of contribution score awarded to bomb planter and terrorists remaining alive if bomb explosion wins the round

default: `"1"`  
flags: `0x80004`  
</details>
<details>
<summary><code>contributionscore_bomb_planted</code></summary>

amount of contribution score for planting a bomb

default: `"2"`  
flags: `0x80004`  
</details>
<details>
<summary><code>contributionscore_cash_bundle</code></summary>

amount of contribution score for picking up a cash bundle

default: `"0"`  
flags: `0x80004`  
</details>
<details>
<summary><code>contributionscore_crate_break</code></summary>

amount of contribution score for breaking an item crate

default: `"0"`  
flags: `0x80004`  
</details>
<details>
<summary><code>contributionscore_hostage_kill</code></summary>

amount of contribution score for killing a hostage, normally negative

default: `"-2"`  
flags: `0x80004`  
</details>
<details>
<summary><code>contributionscore_hostage_rescue_major</code></summary>

amount of contribution score added to rescuer per hostage rescued

default: `"3"`  
flags: `0x80004`  
</details>
<details>
<summary><code>contributionscore_hostage_rescue_minor</code></summary>

amount of contribution score added to all alive CTs per hostage rescued

default: `"1"`  
flags: `0x80004`  
</details>
<details>
<summary><code>contributionscore_kill</code></summary>

amount of contribution score added for a kill

default: `"2"`  
flags: `0x80004`  
</details>
<details>
<summary><code>contributionscore_kill_factor</code></summary>

percentage of victim's contribution score to award to their killer as a bonus

default: `"0"`  
flags: `0x80004`  
</details>
<details>
<summary><code>contributionscore_objective_kill</code></summary>

amount of contribution score added for an objective related kill

default: `"3"`  
flags: `0x80004`  
</details>
<details>
<summary><code>contributionscore_suicide</code></summary>

amount of contribution score for a suicide, normally negative

default: `"-2"`  
flags: `0x80004`  
</details>
<details>
<summary><code>contributionscore_team_kill</code></summary>

amount of contribution score for a team kill, normally negative

default: `"-2"`  
flags: `0x80004`  
</details>
<details>
<summary><code>cs_ShowStateTransitions</code></summary>

cs_ShowStateTransitions <ent index or -1 for all>. Show player state transitions.

default: `"-2"`  
flags: `0x4000`  
</details>
<details>
<summary><code>cs_enable_player_physics_box</code></summary>

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>cs_hostage_near_rescue_music_distance</code></summary>

default: `"2000"`  
flags: `0x4000`  
</details>
<details>
<summary><code>custom_bot_difficulty</code></summary>

Bot difficulty for offline play.

default: `"0"`  
flags: `0x8200c`  
</details>
<details>
<summary><code>debug_aim_angle</code></summary>

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>debug_visibility_monitor</code></summary>

Automatically set by the game when a commentary file is available for the current map.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>dev_reportmoneychanges</code></summary>

Displays money account changes for players in the console

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
<summary><code>ent_messages_draw</code></summary>

Visualizes all entity input/output activity.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>ff_damage_bullet_penetration</code></summary>

If friendly fire is off, this will scale the penetration power and damage a bullet does when penetrating another friendly player

default: `"0"`  
flags: `0x82000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>ff_damage_reduction_bullets</code></summary>

How much to reduce damage done to teammates when shot.  Range is from 0 - 1 (with 1 being damage equal to what is done to an enemy)

default: `"0.1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ff_damage_reduction_grenade</code></summary>

How much to reduce damage done to teammates by a thrown grenade.  Range is from 0 - 1 (with 1 being damage equal to what is done to an enemy)

default: `"0.25"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ff_damage_reduction_grenade_self</code></summary>

How much to damage a player does to himself with his own grenade.  Range is from 0 - 1 (with 1 being damage equal to what is done to an enemy)

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>ff_damage_reduction_other</code></summary>

How much to reduce damage done to teammates by things other than bullets and grenades.  Range is from 0 - 1 (with 1 being damage equal to what is done to an enemy)

default: `"0.25"`  
flags: `0x82000`  
</details>
<details>
<summary><code>fish_dormant</code></summary>

Turns off interactive fish behavior. Fish become immobile and unresponsive.

default: `"0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>func_break_max_pieces</code></summary>

default: `"15"`  
flags: `0x2080`  
</details>
<details>
<summary><code>fx_new_sparks</code></summary>

Use new style sparks.


default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>g_Language</code></summary>

default: `"0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>g_debug_angularsensor</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>g_debug_constraint_sounds</code></summary>

Enable debug printing about constraint sounds.

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
<summary><code>g_debug_vehiclebase</code></summary>

Show alien gib entities

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>g_debug_vehicledriver</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>g_debug_vehicleexit</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>g_debug_vehiclesound</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>g_jeepexitspeed</code></summary>

default: `"100"`  
flags: `0x4000`  
</details>
<details>
<summary><code>g_ragdoll_important_maxcount</code></summary>

default: `"2"`  
flags: `0x2000`  
</details>
<details>
<summary><code>g_ragdoll_maxcount</code></summary>

default: `"8"`  
flags: `0x2000`  
</details>
<details>
<summary><code>game_mode</code></summary>

The current game mode (based on game type). See GameModes.txt.

default: `"0"`  
flags: `0x8200c`  
</details>
<details>
<summary><code>game_online</code></summary>

The current game is online.

default: `"1"`  
flags: `0x201c`  
</details>
<details>
<summary><code>game_public</code></summary>

The current game is public.

default: `"1"`  
flags: `0x201c`  
</details>
<details>
<summary><code>game_type</code></summary>

The current game type. See GameModes.txt.

default: `"0"`  
flags: `0x8200c`  
</details>
<details>
<summary><code>gg_knife_kill_demotes</code></summary>

0 = knife kill in gungame has no effect on player level, 1 = knife kill demotes player by one level

default: `"1"`  
flags: `0x2000`  
</details>
<details>
<summary><code>global_event_log_enabled</code></summary>

Enables the global event log system

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>healthshot_allow_use_at_full</code></summary>

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>healthshot_health</code></summary>

default: `"50"`  
flags: `0x82000`  
</details>
<details>
<summary><code>healthshot_healthboost_damage_multiplier</code></summary>

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>healthshot_healthboost_speed_multiplier</code></summary>

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>healthshot_healthboost_time</code></summary>

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>hl2_episodic</code></summary>

default: `"0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>hostage_debug</code></summary>

Show hostage AI debug information

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>hostage_feetyawrate</code></summary>

How many degrees per second that hostages can turn their feet or upper body.

default: `"720"`  
flags: `0x2002`  
</details>
<details>
<summary><code>hostage_is_silent</code></summary>

When set, the hostage won't play any code driven response rules lines

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>inferno_child_spawn_interval_multiplier</code></summary>

Amount spawn interval increases for each child

default: `"0.1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>inferno_child_spawn_max_depth</code></summary>

default: `"4"`  
flags: `0x82000`  
</details>
<details>
<summary><code>inferno_damage</code></summary>

Damage per second

default: `"40"`  
flags: `0x4000`  
</details>
<details>
<summary><code>inferno_debug</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>inferno_flame_lifetime</code></summary>

Average lifetime of each flame in seconds

default: `"7"`  
flags: `0x82000`  
</details>
<details>
<summary><code>inferno_flame_spacing</code></summary>

Minimum distance between separate flame spawns

default: `"42"`  
flags: `0x4000`  
</details>
<details>
<summary><code>inferno_forward_reduction_factor</code></summary>

default: `"0.9"`  
flags: `0x4000`  
</details>
<details>
<summary><code>inferno_friendly_fire_duration</code></summary>

For this long, FF is credited back to the thrower.

default: `"6"`  
flags: `0x4000`  
</details>
<details>
<summary><code>inferno_initial_spawn_interval</code></summary>

Time between spawning flames for first fire

default: `"0.02"`  
flags: `0x4000`  
</details>
<details>
<summary><code>inferno_max_child_spawn_interval</code></summary>

Largest time interval for child flame spawning

default: `"0.5"`  
flags: `0x4000`  
</details>
<details>
<summary><code>inferno_max_flames</code></summary>

Maximum number of flames that can be created

default: `"16"`  
flags: `0x82000`  
</details>
<details>
<summary><code>inferno_max_range</code></summary>

Maximum distance flames can spread from their initial ignition point

default: `"150"`  
flags: `0x82000`  
</details>
<details>
<summary><code>inferno_per_flame_spawn_duration</code></summary>

Duration each new flame will attempt to spawn new flames

default: `"3"`  
flags: `0x4000`  
</details>
<details>
<summary><code>inferno_scorch_decals</code></summary>

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>inferno_spawn_angle</code></summary>

Angular change from parent

default: `"45"`  
flags: `0x4000`  
</details>
<details>
<summary><code>inferno_surface_offset</code></summary>

default: `"20"`  
flags: `0x4000`  
</details>
<details>
<summary><code>inferno_velocity_decay_factor</code></summary>

default: `"0.2"`  
flags: `0x4000`  
</details>
<details>
<summary><code>inferno_velocity_factor</code></summary>

default: `"0.003"`  
flags: `0x4000`  
</details>
<details>
<summary><code>inferno_velocity_normal_factor</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>loopsingleplayermaps</code></summary>

Show bbox and dismount points for all ladders (must be set before level load.)


default: `"0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>mapcycledisabled</code></summary>

repeats the same map after each match instead of using the map cycle

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>molotov_throw_detonate_time</code></summary>

default: `"2.0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_afterroundmoney</code></summary>

amount of money awared to every player after each round

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_allowNPCs</code></summary>

default: `"1"`  
flags: `0x100`  
</details>
<details>
<summary><code>mp_allowspectators</code></summary>

toggles whether the server allows spectator mode or not

default: `"1.0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>mp_anyone_can_pickup_c4</code></summary>

If set, everyone can pick up the c4, not just Ts.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_autocrosshair</code></summary>

default: `"1"`  
flags: `0x100`  
</details>
<details>
<summary><code>mp_autokick</code></summary>

Kick idle/team-killing/team-damaging players

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_autoteambalance</code></summary>

default: `"1"`  
flags: `0x80100`  
</details>
<details>
<summary><code>mp_backup_restore_load_autopause</code></summary>

Whether to automatically pause the match after restoring round data from backup

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>mp_backup_round_auto</code></summary>

If enabled will keep in-memory backups to handle reconnecting players even if the backup files aren't written to disk

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>mp_backup_round_file</code></summary>

If set then server will save all played rounds information to files filename_date_time_team1_team2_mapname_roundnum_score1_score2.txt

default: `"backup"`  
flags: `0x80000`  
</details>
<details>
<summary><code>mp_backup_round_file_last</code></summary>

Every time a backup file is written the value of this convar gets updated to hold the name of the backup file.

default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>mp_backup_round_file_pattern</code></summary>

If set then server will save all played rounds information to files named by this pattern, e.g.'%prefix%_%date%_%time%_%team1%_%team2%_%map%_round%round%_score_%score1%_%score2%.txt'

default: `"%prefix%_round%round%.txt"`  
flags: `0x80000`  
</details>
<details>
<summary><code>mp_blockstyle</code></summary>

Sets the style of capture point blocking used. 0 = Blocks break captures completely. 1 = Blocks only pause captures.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>mp_bonusroundtime</code></summary>

Time after round win until round restarts

default: `"15"`  
flags: `0x2000`  
min value: `5`  
max value: `15`  
</details>
<details>
<summary><code>mp_buy_allow_grenades</code></summary>

Whether players can purchase grenades from the buy menu or not.

default: `"1"`  
flags: `0x82000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>mp_buy_allow_guns</code></summary>

Whether players can purchase guns: pistols (1), SMGs (2), rifles (4), shotguns (8), sniper rifles (16), heavy MGs (32).

default: `"255"`  
flags: `0x82000`  
min value: `0`  
max value: `255`  
</details>
<details>
<summary><code>mp_buy_anywhere</code></summary>

When set, players can buy anywhere, not only in buyzones. 0 = default. 1 = both teams. 2 = Terrorists. 3 = Counter-Terrorists.

default: `"0"`  
flags: `0x82100`  
</details>
<details>
<summary><code>mp_buy_during_immunity</code></summary>

When set, players can buy when immune, ignoring buytime. 0 = default. 1 = both teams. 2 = Terrorists. 3 = Counter-Terrorists.

default: `"0"`  
flags: `0x82100`  
</details>
<details>
<summary><code>mp_buytime</code></summary>

How many seconds after round start players can buy items for.

default: `"90"`  
flags: `0x82000`  
min value: `0`  
</details>
<details>
<summary><code>mp_c4_cannot_be_defused</code></summary>

If set, the planted c4 cannot be defused.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_c4timer</code></summary>

how long from when the C4 is armed until it blows

default: `"40"`  
flags: `0x82100`  
min value: `10`  
</details>
<details>
<summary><code>mp_capdeteriorate_time</code></summary>

Time it takes for a full capture point to deteriorate.

default: `"90.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>mp_capstyle</code></summary>

Sets the style of capture points used. 0 = Fixed players required to cap. 1 = More players cap faster, but longer cap times.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>mp_competitive_endofmatch_extra_time</code></summary>

After a competitive match finishes rematch voting extra time is given for rankings.

default: `"15"`  
flags: `0x80000`  
</details>
<details>
<summary><code>mp_consecutive_loss_aversion</code></summary>

How loss streak is affected with round win: 0 = win fully resets loss bonus, 1 = first win steps down loss bonus, 2 = first win holds loss bonus and step down starting with second win

default: `"1"`  
flags: `0x82000`  
min value: `0`  
</details>
<details>
<summary><code>mp_consecutive_loss_max</code></summary>



default: `"4"`  
flags: `0x82000`  
min value: `0`  
</details>
<details>
<summary><code>mp_coop_force_join_ct</code></summary>

If set, real players will auto join CT on join.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_coopmission_bot_difficulty_offset</code></summary>

The difficulty offset modifier for bots during coop missions.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_coopmission_mission_number</code></summary>

Which mission the map should run after it loads.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_ct_default_grenades</code></summary>

The default grenades that the CTs will spawn with.  To give multiple grenades, separate each weapon class with a space like this: 'weapon_molotov weapon_hegrenade'

default: `""`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_ct_default_melee</code></summary>

The default melee weapon that the CTs will spawn with.  Even if this is blank, a knife will be given.  To give a taser, it should look like this: 'weapon_knife weapon_taser'.  Remember to set mp_weapons_allow_zeus to 1 if you want to give a taser!

default: `"weapon_knife"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_ct_default_primary</code></summary>

The default primary (rifle) weapon that the CTs will spawn with

default: `""`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_ct_default_secondary</code></summary>

The default secondary (pistol) weapon that the CTs will spawn with

default: `"weapon_hkp2000"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_damage_headshot_only</code></summary>

Determines whether non-headshot hits do any damage.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_damage_scale_ct_body</code></summary>

Scales the damage a CT player takes by this much when they take damage in the body. (1 == 100%, 0.5 == 50%)

default: `"1.0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_damage_scale_ct_head</code></summary>

Scales the damage a CT player takes by this much when they take damage in the head (1 == 100%, 0.5 == 50%).  REMEMBER! headshots do 4x the damage of the body before this scaler is applied.

default: `"1.0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_damage_scale_t_body</code></summary>

Scales the damage a T player takes by this much when they take damage in the body. (1 == 100%, 0.5 == 50%)

default: `"1.0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_damage_scale_t_head</code></summary>

Scales the damage a T player takes by this much when they take damage in the head (1 == 100%, 0.5 == 50%).  REMEMBER! headshots do 4x the damage of the body before this scaler is applied.

default: `"1.0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_damage_vampiric_amount</code></summary>

If Set to non-0, will determine the fraction of damage dealt that will be given to attacker.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_death_drop_breachcharge</code></summary>

Drop breachcharge on player death

default: `"1"`  
flags: `0x82000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>mp_death_drop_c4</code></summary>

Whether c4 is droppable

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_death_drop_defuser</code></summary>

Drop defuser on player death

default: `"1"`  
flags: `0x82000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>mp_death_drop_grenade</code></summary>

Which grenade to drop on player death: 0=none, 1=best, 2=current or best, 3=all grenades

default: `"2"`  
flags: `0x82000`  
min value: `0`  
max value: `3`  
</details>
<details>
<summary><code>mp_death_drop_gun</code></summary>

Which gun to drop on player death: 0=none, 1=best, 2=current or best

default: `"1"`  
flags: `0x82000`  
min value: `0`  
max value: `2`  
</details>
<details>
<summary><code>mp_death_drop_healthshot</code></summary>

Drop healthshot on player death

default: `"1"`  
flags: `0x82000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>mp_death_drop_taser</code></summary>

Drop taser on player death

default: `"1"`  
flags: `0x82000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>mp_deathcam_skippable</code></summary>

Determines whether a player can early-out of the deathcam.

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_default_team_winner_no_objective</code></summary>

If the map doesn't define an objective (bomb, hostage, etc), the value of this convar will declare the winner when the time runs out in the round.

default: `"-1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_defuser_allocation</code></summary>

How to allocate defusers to CTs at start or round: 0=none, 1=random, 2=everyone

default: `"0"`  
flags: `0x82000`  
min value: `0`  
max value: `2`  
</details>
<details>
<summary><code>mp_disable_respawn_times</code></summary>

default: `"0"`  
flags: `0x2100`  
</details>
<details>
<summary><code>mp_disconnect_kills_bots</code></summary>

When a bot disconnects, kill them first.  Requires mp_disconnect_kills_players.

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>mp_disconnect_kills_players</code></summary>

When a player disconnects, kill them first (triggering item drops, stats, etc.)

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>mp_display_kill_assists</code></summary>

Whether to display and score player assists

default: `"1"`  
flags: `0x82000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>mp_dm_bonus_length_max</code></summary>

Maximum time the bonus time will last (in seconds)

default: `"30"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_dm_bonus_length_min</code></summary>

Minimum time the bonus time will last (in seconds)

default: `"30"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_dm_bonus_percent</code></summary>

Percent of points additionally awarded when someone gets a kill with the bonus weapon during the bonus period.

default: `"50"`  
flags: `0x82000`  
min value: `0`  
</details>
<details>
<summary><code>mp_dm_bonus_respawn</code></summary>

When attempting to get the bonus weapon in deathmatch, whether we should respawn you with it or just give it to you directly

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_dm_bonusweapon_dogtags</code></summary>

Additional dogtags to drop when making a kill with the bonus weapon

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_dm_dogtag_score</code></summary>

Points to award for picking up a dogtag in deathmatch.

default: `"0"`  
flags: `0x82000`  
min value: `0`  
</details>
<details>
<summary><code>mp_dm_kill_base_score</code></summary>

Number of base points to award for a kill in deathmatch.  Cheaper weapons award 1 or 2 additional points.

default: `"10"`  
flags: `0x82000`  
min value: `0`  
</details>
<details>
<summary><code>mp_dm_teammode</code></summary>

In deathmatch, enables team DM visuals & scoring

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_dm_teammode_bonus_score</code></summary>

Team deathmatch victory points to award for kill with bonus weapon

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_dm_teammode_dogtag_score</code></summary>

Team deathmatch victory points to award for collecting enemy dogtags

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_dm_teammode_kill_score</code></summary>

Team deathmatch victory points to award for enemy kill

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_dm_time_between_bonus_max</code></summary>

Maximum time a bonus time will start after the round start or after the last bonus (in seconds)

default: `"40"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_dm_time_between_bonus_min</code></summary>

Minimum time a bonus time will start after the round start or after the last bonus (in seconds)

default: `"30"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_do_warmup_offine</code></summary>

Whether or not to do a warmup period at the start of a match in an offline (bot) match.

default: `"0"`  
flags: `0x82000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>mp_do_warmup_period</code></summary>

Whether or not to do a warmup period at the start of a match.

default: `"1"`  
flags: `0x82000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>mp_dogtag_despawn_on_killer_death</code></summary>

Whether dogtags should despawn when their killer dies

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_dogtag_despawn_time</code></summary>

How many seconds dogtags should stay around before despawning automatically (0 = infinite)

default: `"120"`  
flags: `0x82000`  
min value: `0`  
</details>
<details>
<summary><code>mp_dogtag_pickup_rule</code></summary>

Who is eligible to pick up a dogtag (0 = killer only, 1 = killer's team, 2 = victim's team, 3 = killer & victim's team, 4 = anyone)

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_drop_grenade_enable</code></summary>

Allows players to drop grenades.

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>mp_drop_knife_enable</code></summary>

Allows players to drop knives.

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>mp_economy_reset_rounds</code></summary>

Reset all player money every N rounds (0 for never)

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_enableroundwaittime</code></summary>

Enable timers to wait between rounds.

default: `"1"`  
flags: `0x2000`  
</details>
<details>
<summary><code>mp_endmatch_votenextleveltime</code></summary>

If mp_endmatch_votenextmap is set, players have this much time to vote on the next map at match end.

default: `"20"`  
flags: `0x80000`  
</details>
<details>
<summary><code>mp_endmatch_votenextmap</code></summary>

Whether or not players vote for the next map at the end of the match when the final scoreboard comes up

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_endmatch_votenextmap_keepcurrent</code></summary>

If set, keeps the current map in the list of voting options.  If not set, the current map will not appear in the list of voting options.

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_endmatch_votenextmap_wargames_modes</code></summary>

Modes available for endmatch voting during War Games. Separate names with spaces.

default: `"armsrace demolition flyingscoutsman"`  
flags: `0x80000`  
</details>
<details>
<summary><code>mp_endmatch_votenextmap_wargames_nummaps</code></summary>

Maximum number of maps to include in endmatch voting during War Games

default: `"3"`  
flags: `0x80000`  
</details>
<details>
<summary><code>mp_endmatch_votenextmap_wargames_nummodes</code></summary>

Maximum number of other War Games to include in endmatch voting during War Games

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>mp_endwarmup_player_count</code></summary>

Number of players required to be connected to end warmup early. 0 to require maximum players for mode.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_equipment_reset_rounds</code></summary>

Reset all player equipment every N rounds (0 for never)

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_facefronttime</code></summary>

After this amount of time of standing in place but aiming to one side, go ahead and move feet to face upper body.

default: `"2"`  
flags: `0x2002`  
</details>
<details>
<summary><code>mp_falldamage</code></summary>

default: `"0"`  
flags: `0x100`  
</details>
<details>
<summary><code>mp_feetyawrate</code></summary>

How many degrees per second that we can turn our feet or upper body.

default: `"400"`  
flags: `0x2002`  
</details>
<details>
<summary><code>mp_flashlight</code></summary>

default: `"0"`  
flags: `0x100`  
</details>
<details>
<summary><code>mp_flinch_punch_scale</code></summary>

Scalar for first person view punch when getting hit.

default: `"3"`  
flags: `0x6002`  
</details>
<details>
<summary><code>mp_footsteps</code></summary>

default: `"1"`  
flags: `0x100`  
</details>
<details>
<summary><code>mp_force_assign_teams</code></summary>

Players don't get to choose what team they are on, it is auto assinged.

default: `"0"`  
flags: `0x82000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>mp_force_pick_time</code></summary>

The amount of time a player has on the team screen to make a selection before being auto-teamed

default: `"15"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_forcerespawn</code></summary>

default: `"1"`  
flags: `0x100`  
</details>
<details>
<summary><code>mp_fraglimit</code></summary>

The number of kills at which the map ends

default: `"0"`  
flags: `0x2100`  
</details>
<details>
<summary><code>mp_free_armor</code></summary>

Determines whether kevlar (1+) and/or helmet (2+) are given automatically.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_freezetime</code></summary>

how many seconds to keep players frozen when the round starts

default: `"6"`  
flags: `0x82100`  
min value: `0`  
max value: `60`  
</details>
<details>
<summary><code>mp_friendlyfire</code></summary>

Allows team members to injure other members of their team

default: `"0"`  
flags: `0x82100`  
</details>
<details>
<summary><code>mp_ggprogressive_random_weapon_kills_needed</code></summary>

If mp_ggprogressive_use_random_weapons is set, this is the number of kills needed with each weapon

default: `"2"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_ggprogressive_round_restart_delay</code></summary>

Number of seconds to delay before restarting a round after a win in gungame progessive

default: `"15.0"`  
flags: `0x82000`  
min value: `0`  
max value: `90`  
</details>
<details>
<summary><code>mp_ggprogressive_use_random_weapons</code></summary>

If set, selects random weapons from set categories for the progression order

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_ggtr_always_upgrade</code></summary>

Award this many upgrade points every round in demolition mode

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_ggtr_bomb_defuse_bonus</code></summary>

Number of bonus upgrades to award the CTs when they defuse a gun game bomb

default: `"1"`  
flags: `0x82000`  
min value: `1`  
max value: `10`  
</details>
<details>
<summary><code>mp_ggtr_bomb_detonation_bonus</code></summary>

Number of bonus upgrades to award the Ts when they detonate a gun game bomb

default: `"1"`  
flags: `0x82000`  
min value: `1`  
max value: `10`  
</details>
<details>
<summary><code>mp_ggtr_bomb_pts_for_flash</code></summary>

Kill points required in a round to get a bonus flash grenade

default: `"4"`  
flags: `0x82000`  
min value: `1`  
max value: `5`  
</details>
<details>
<summary><code>mp_ggtr_bomb_pts_for_he</code></summary>

Kill points required in a round to get a bonus HE grenade

default: `"3"`  
flags: `0x82000`  
min value: `1`  
max value: `5`  
</details>
<details>
<summary><code>mp_ggtr_bomb_pts_for_molotov</code></summary>

Kill points required in a round to get a bonus molotov cocktail

default: `"5"`  
flags: `0x82000`  
min value: `1`  
max value: `5`  
</details>
<details>
<summary><code>mp_ggtr_bomb_pts_for_upgrade</code></summary>

Kill points required to upgrade a player's weapon

default: `"2.0"`  
flags: `0x82000`  
min value: `1`  
max value: `10`  
</details>
<details>
<summary><code>mp_ggtr_bomb_respawn_delay</code></summary>

Number of seconds to delay before making the bomb available to a respawner in gun game

default: `"0.0"`  
flags: `0x82000`  
min value: `0`  
max value: `30`  
</details>
<details>
<summary><code>mp_ggtr_end_round_kill_bonus</code></summary>

Number of bonus points awarded in Demolition Mode when knife kill ends round

default: `"1"`  
flags: `0x82000`  
min value: `0`  
max value: `10`  
</details>
<details>
<summary><code>mp_ggtr_halftime_delay</code></summary>

Number of seconds to delay during TR Mode halftime

default: `"0.0"`  
flags: `0x82000`  
min value: `0`  
max value: `30`  
</details>
<details>
<summary><code>mp_ggtr_last_weapon_kill_ends_half</code></summary>

End the half and give a team round point when a player makes a kill using the final weapon

default: `"0"`  
flags: `0x82000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>mp_ggtr_num_rounds_autoprogress</code></summary>

Upgrade the player's weapon after this number of rounds without upgrading

default: `"3"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_give_player_c4</code></summary>

Whether this map should spawn a c4 bomb for a player or not.

default: `"1"`  
flags: `0x82000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>mp_global_damage_per_second</code></summary>

If above 0, deal non-lethal damage to players over time.

default: `"0.0"`  
flags: `0x82000`  
min value: `0`  
</details>
<details>
<summary><code>mp_guardian_bot_money_per_wave</code></summary>

The amount of money bots get time each wave the players complete.  This # is absolute and not additive, the money is set to (this)x(wave#) for each bot on each wave.

default: `"800"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_guardian_loc_adjective</code></summary>

(If set) kill condition token (#quest_hud_guardian_adjective_<name>)

default: `""`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_guardian_loc_condition</code></summary>

(If set) kill condition token (#quest_hud_guardian_condition_<name>)

default: `""`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_guardian_loc_icon</code></summary>

(If set) icon to override for guardian mission

default: `""`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_guardian_loc_mission</code></summary>

Token to use to generate guardian mission (#quest_hud_guardian_mission_<name>)

default: `"default"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_guardian_loc_override</code></summary>

Token to use to display guardian mission (#quest_hud_guardian_override_<name>) (if exists)

default: `""`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_guardian_loc_weapon</code></summary>

(If set) weapon name token (#SFUI_WPNHUD_<name>)

default: `""`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_guardian_player_dist_max</code></summary>

The maximum distance a player is allowed to get from the bombsite before they're killed.

default: `"2000"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_guardian_player_dist_min</code></summary>

The distance at which we start to warn a player when they are too far from the guarded bombsite.

default: `"1300"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_guardian_special_kills_needed</code></summary>

The number of kills needed with a specific weapon.

default: `"10"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_guardian_special_weapon_needed</code></summary>

The weapon that needs to be used to increment the kills needed to complete the mission.

default: `"awp"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_guardian_target_site</code></summary>

If set to the index of a bombsite, will cause random spawns to be only created near that site.

default: `"-1"`  
flags: `0x80004`  
</details>
<details>
<summary><code>mp_halftime</code></summary>

Determines whether the match switches sides in a halftime event.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_halftime_duration</code></summary>

Number of seconds that halftime lasts

default: `"15.0"`  
flags: `0x82000`  
min value: `0`  
max value: `300`  
</details>
<details>
<summary><code>mp_halftime_pausematch</code></summary>

Set to 1 to pause match after halftime countdown elapses. Match must be resumed by vote or admin.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_halftime_pausetimer</code></summary>

Set to 1 to stay in halftime indefinitely. Set to 0 to resume the timer.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_heavyassaultsuit_aimpunch</code></summary>

How much EXTRA aim punch will happen when a player wearing the assault suit gets when shot

default: `"1.0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_heavyassaultsuit_cooldown</code></summary>

Determines cooldown of purchase.

default: `"5"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_heavyassaultsuit_deploy_timescale</code></summary>

How fast a player wearing the heavy assault suit will draw their weapon (1.0 = normal speed, 0.5 = half speed)

default: `"0.8"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_heavyassaultsuit_speed</code></summary>

The max speed of a player when they are wearing the heavy assault suit

default: `"130"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_heavybot_damage_reduction_scale</code></summary>

How much damage should scale when the player wearing the heavy assault suit is shot (1.0 = no change, 0.5 = half damage)

default: `"1.0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_hostagepenalty</code></summary>

Terrorist are kicked for killing too much hostages

default: `"10"`  
flags: `0x100`  
</details>
<details>
<summary><code>mp_hostages_max</code></summary>

Maximum number of hostages to spawn.

default: `"2"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_hostages_rescuetime</code></summary>

Additional time added to round time if a hostage is reached by a CT.

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_hostages_rescuetowin</code></summary>

0 == all alive, any other number is the number the CT's need to rescue to win the round.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>mp_hostages_run_speed_modifier</code></summary>

Default is 1.0, slow down hostages by setting this to < 1.0.

default: `"1.0"`  
flags: `0x82000`  
min value: `0.1`  
max value: `1.5`  
</details>
<details>
<summary><code>mp_hostages_spawn_farthest</code></summary>

When enabled will consistently force the farthest hostages to spawn.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_hostages_spawn_force_positions</code></summary>

Comma separated list of zero based indices to force spawn positions, e.g. '0,2' or '1,6'

default: `""`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_hostages_spawn_same_every_round</code></summary>

0 = spawn hostages randomly every round, 1 = same spawns for entire match.

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_hostages_takedamage</code></summary>

Whether or not hostages can be hurt.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_humanteam</code></summary>

Restricts human players to a single team {any, CT, T}

default: `"any"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_ignore_round_win_conditions</code></summary>

Ignore conditions which would end the current round

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_ik</code></summary>

Use IK on in-place turns.

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>mp_join_grace_time</code></summary>

Number of seconds after round start to allow a player to join a game

default: `"0.0"`  
flags: `0x82000`  
min value: `0`  
max value: `30`  
</details>
<details>
<summary><code>mp_limitteams</code></summary>

Max # of players 1 team can have over another (0 disables check)

default: `"2"`  
flags: `0x82100`  
min value: `0`  
max value: `30`  
</details>
<details>
<summary><code>mp_logdetail</code></summary>

Logs attacks.  Values are: 0=off, 1=enemy, 2=teammate, 3=both)

default: `"0"`  
flags: `0x80000`  
min value: `0`  
max value: `3`  
</details>
<details>
<summary><code>mp_logdetail_items</code></summary>

Logs a line any time a player acquires or loses an item.

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>mp_logdistance_2d</code></summary>

Enables distance logging every so many units

default: `"250"`  
flags: `0x80000`  
</details>
<details>
<summary><code>mp_logdistance_sec</code></summary>

Enables distance logging every so many seconds

default: `"15"`  
flags: `0x80000`  
</details>
<details>
<summary><code>mp_logloadouts</code></summary>

Enables distance logging with full loadouts

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>mp_logmoney</code></summary>

Enables money logging.  Values are: 0=off, 1=on

default: `"0"`  
flags: `0x80000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>mp_match_can_clinch</code></summary>

Can a team clinch and end the match by being so far ahead that the other team has no way to catching up?

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_match_end_changelevel</code></summary>

At the end of the match, perform a changelevel even if next map is the same

default: `"0"`  
flags: `0x82000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>mp_match_end_restart</code></summary>

At the end of the match, perform a restart instead of loading a new map

default: `"0"`  
flags: `0x82000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>mp_match_restart_delay</code></summary>

Time (in seconds) until a match restarts.

default: `"15"`  
flags: `0x82000`  
min value: `1`  
max value: `9999`  
</details>
<details>
<summary><code>mp_max_armor</code></summary>

Determines the highest level of armor allowed to be purchased. (0) None, (1) Kevlar, (2) Helmet

default: `"2"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_maxmoney</code></summary>

maximum amount of money allowed in a player's account

default: `"16000"`  
flags: `0x82000`  
min value: `0`  
</details>
<details>
<summary><code>mp_maxrounds</code></summary>

max number of rounds to play before server changes maps

default: `"0"`  
flags: `0x82100`  
min value: `0`  
</details>
<details>
<summary><code>mp_molotovusedelay</code></summary>

Number of seconds to delay before the molotov can be used after acquiring it

default: `"15.0"`  
flags: `0x82000`  
min value: `0`  
max value: `30`  
</details>
<details>
<summary><code>mp_only_cts_rescue_hostages</code></summary>



default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_overtime_enable</code></summary>

If a match ends in a tie, use overtime rules to determine winner

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_overtime_halftime_pausetimer</code></summary>

If set to 1 will set mp_halftime_pausetimer to 1 before every half of overtime. Set mp_halftime_pausetimer to 0 to resume the timer.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_overtime_maxrounds</code></summary>

When overtime is enabled play additional rounds to determine winner

default: `"6"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_overtime_startmoney</code></summary>

Money assigned to all players at start of every overtime half

default: `"10000"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_plant_c4_anywhere</code></summary>



default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_player_healthbuffer_decay_rate</code></summary>

When a player has buffer health, this is how fast it ticks down.

default: `"0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>mp_playercashawards</code></summary>

Players can earn money by performing in-game actions

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_playerid</code></summary>

Controls what information player see in the status bar: 0 all names; 1 team names; 2 no names

default: `"0"`  
flags: `0x82000`  
min value: `0`  
max value: `2`  
</details>
<details>
<summary><code>mp_playerid_delay</code></summary>

Number of seconds to delay showing information in the status bar

default: `"0.4"`  
flags: `0x82000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>mp_playerid_hold</code></summary>

Number of seconds to keep showing old information in the status bar

default: `"0.2"`  
flags: `0x82000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>mp_radar_showall</code></summary>

Determines who should see all. 0 = default. 1 = both teams. 2 = Terrorists. 3 = Counter-Terrorists.

default: `"0"`  
flags: `0x82000`  
min value: `0`  
max value: `3`  
</details>
<details>
<summary><code>mp_randomspawn</code></summary>

Determines whether players are to spawn. 0 = default; 1 = both teams; 2 = Terrorists; 3 = CTs.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_randomspawn_dist</code></summary>

If using mp_randomspawn, determines whether to test distance when selecting this spot.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_randomspawn_los</code></summary>

If using mp_randomspawn, determines whether to test Line of Sight when spawning.

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_require_gun_use_to_acquire</code></summary>

Whether guns must be +used to acquire or default is touch-to-pickup

default: `"0"`  
flags: `0x80000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>mp_respawn_immunitytime</code></summary>

How many seconds after respawn immunity lasts.

default: `"4.0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_respawn_on_death_ct</code></summary>

When set to 1, counter-terrorists will respawn after dying.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_respawn_on_death_t</code></summary>

When set to 1, terrorists will respawn after dying.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_respawnwavetime</code></summary>

Time between respawn waves.

default: `"10.0"`  
flags: `0x2100`  
</details>
<details>
<summary><code>mp_restartgame</code></summary>

If non-zero, game will restart in the specified number of seconds

default: `"0"`  
flags: `0x80004`  
</details>
<details>
<summary><code>mp_round_restart_delay</code></summary>

Number of seconds to delay before restarting a round after a win

default: `"7.0"`  
flags: `0x82000`  
min value: `0`  
max value: `14`  
</details>
<details>
<summary><code>mp_roundtime</code></summary>

How many minutes each round takes.

default: `"5"`  
flags: `0x82100`  
min value: `1`  
max value: `60`  
</details>
<details>
<summary><code>mp_roundtime_defuse</code></summary>

How many minutes each round of Bomb Defuse takes. If 0 then use mp_roundtime instead.

default: `"0"`  
flags: `0x82100`  
min value: `0`  
max value: `60`  
</details>
<details>
<summary><code>mp_roundtime_deployment</code></summary>

How many minutes deployment for coop mission takes.

default: `"5"`  
flags: `0x80000`  
min value: `1`  
max value: `15`  
</details>
<details>
<summary><code>mp_roundtime_hostage</code></summary>

How many minutes each round of Hostage Rescue takes. If 0 then use mp_roundtime instead.

default: `"0"`  
flags: `0x82100`  
min value: `0`  
max value: `60`  
</details>
<details>
<summary><code>mp_shield_speed_deployed</code></summary>

The max speed of a player when they have a shield deployed

default: `"170"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_shield_speed_holstered</code></summary>

The max speed of a player when they have a shield holstered

default: `"200"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_showcleanedupents</code></summary>

Show entities that are removed on round respawn.

default: `"0"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mp_showroundtransitions</code></summary>

Show gamestate round transitions.

default: `"0"`  
flags: `0x4002`  
</details>
<details>
<summary><code>mp_solid_teammates</code></summary>

How solid are teammates: 0 = transparent; 1 = fully solid; 2 = can stand on top of heads

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_spawnprotectiontime</code></summary>

Kick players who team-kill within this many seconds of a round restart.

default: `"5"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_spec_swapplayersides</code></summary>

Toggle set the player names and team names to the opposite side in which they are are on the spectator panel.

default: `"0"`  
flags: `0x82000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>mp_spectators_max</code></summary>

How many spectators are allowed in a match.

default: `"2"`  
flags: `0x82000`  
min value: `0`  
</details>
<details>
<summary><code>mp_stalemate_at_timelimit</code></summary>

Allow the match to end when mp_timelimit hits instead of waiting for the end of the current round.

default: `"0"`  
flags: `0x100`  
</details>
<details>
<summary><code>mp_stalemate_enable</code></summary>

Enable/Disable stalemate mode.

default: `"0"`  
flags: `0x100`  
</details>
<details>
<summary><code>mp_stalemate_timelimit</code></summary>

Timelimit (in seconds) of the stalemate round.

default: `"240"`  
flags: `0x2000`  
</details>
<details>
<summary><code>mp_starting_losses</code></summary>

Determines what the initial loss streak is.

default: `"0"`  
flags: `0x82000`  
min value: `0`  
</details>
<details>
<summary><code>mp_startmoney</code></summary>

amount of money each player gets when they reset

default: `"800"`  
flags: `0x82000`  
min value: `0`  
</details>
<details>
<summary><code>mp_suicide_time</code></summary>

Specifies number of seconds required to wait before another suicide.

default: `"5"`  
flags: `0x6000`  
</details>
<details>
<summary><code>mp_t_default_grenades</code></summary>

The default grenades that the Ts will spawn with.  To give multiple grenades, separate each weapon class with a space like this: 'weapon_molotov weapon_hegrenade'

default: `""`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_t_default_melee</code></summary>

The default melee weapon that the Ts will spawn with

default: `"weapon_knife"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_t_default_primary</code></summary>

The default primary (rifle) weapon that the Ts will spawn with

default: `""`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_t_default_secondary</code></summary>

The default secondary (pistol) weapon that the Ts will spawn with

default: `"weapon_glock"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_tagging_scale</code></summary>

Scalar for player tagging modifier when hit. Lower values for greater tagging.

default: `"1.0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_taser_recharge_time</code></summary>

Determines recharge time for taser. -1 = disabled.

default: `"-1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_td_dmgtokick</code></summary>

The damage threshhold players have to exceed in a match to get kicked.

default: `"300"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_td_dmgtowarn</code></summary>

The damage threshhold players have to exceed in a match to get warned that they are about to be kicked.

default: `"200"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_td_spawndmgthreshold</code></summary>

The damage threshold players have to exceed at the start of the round to be warned/kick.

default: `"50"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_team_timeout_max</code></summary>

Number of timeouts each team gets per match.

default: `"1"`  
flags: `0x82004`  
</details>
<details>
<summary><code>mp_team_timeout_time</code></summary>

Duration of each timeout.

default: `"60"`  
flags: `0x82004`  
</details>
<details>
<summary><code>mp_teamcashawards</code></summary>

Teams can earn money by performing in-game actions

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_teamflag_1</code></summary>

Enter a country's alpha 2 code to show that flag next to team 1's name in the spectator scoreboard.

default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>mp_teamflag_2</code></summary>

Enter a country's alpha 2 code to show that flag next to team 2's name in the spectator scoreboard.

default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>mp_teamlist</code></summary>

default: `"hgrunt;scientist"`  
flags: `0x100`  
</details>
<details>
<summary><code>mp_teamlogo_1</code></summary>

Enter a team's shorthand image name to display their logo. Images can be found here: 'resource/flash/econ/tournaments/teams'

default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>mp_teamlogo_2</code></summary>

Enter a team's shorthand image name to display their logo. Images can be found here: 'resource/flash/econ/tournaments/teams'

default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>mp_teammatchstat_1</code></summary>

A non-empty string sets first team's match stat.

default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>mp_teammatchstat_2</code></summary>

A non-empty string sets second team's match stat.

default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>mp_teammatchstat_cycletime</code></summary>

Cycle match stats after so many seconds

default: `"45"`  
flags: `0x80000`  
</details>
<details>
<summary><code>mp_teammatchstat_holdtime</code></summary>

Decide on a match stat and hold it additionally for at least so many seconds

default: `"5"`  
flags: `0x80000`  
</details>
<details>
<summary><code>mp_teammatchstat_txt</code></summary>

A non-empty string sets the match stat description, e.g. 'Match 2 of 3'.

default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>mp_teammates_are_enemies</code></summary>

When set, your teammates act as enemies and all players are valid targets.

default: `"0"`  
flags: `0x82100`  
</details>
<details>
<summary><code>mp_teamname_1</code></summary>

A non-empty string overrides the first team's name.

default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>mp_teamname_2</code></summary>

A non-empty string overrides the second team's name.

default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>mp_teamplay</code></summary>

default: `"0"`  
flags: `0x100`  
</details>
<details>
<summary><code>mp_teamprediction_pct</code></summary>

A value between 1 and 99 will show predictions in favor of CT team.

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>mp_teamprediction_txt</code></summary>

A value between 1 and 99 will set predictions in favor of first team.

default: `"#SFUIHUD_Spectate_Predictions"`  
flags: `0x80000`  
</details>
<details>
<summary><code>mp_teams_unbalance_limit</code></summary>

Teams are unbalanced when one team has this many more players than the other team. (0 disables check)

default: `"1"`  
flags: `0x2100`  
min value: `0`  
max value: `30`  
</details>
<details>
<summary><code>mp_teamscore_1</code></summary>

A non-empty string for best-of-N maps won by the first team.

default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>mp_teamscore_2</code></summary>

A non-empty string for best-of-N maps won by the second team.

default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>mp_tkpunish</code></summary>

Will TK'ers and team damagers be punished in the next round?  {0=no,  1=yes}

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_tournament</code></summary>

default: `"0"`  
flags: `0x2100`  
</details>
<details>
<summary><code>mp_use_respawn_waves</code></summary>

When set to 1, and that player's team is set to respawn, they will respawn in waves. If set to 2, teams will respawn when the whole team is dead.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_verbose_changelevel_spew</code></summary>

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>mp_warmup_pausetimer</code></summary>

Set to 1 to stay in warmup indefinitely. Set to 0 to resume the timer.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_warmuptime_all_players_connected</code></summary>

Warmup time to use when all players have connected. 0 to disable.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_weapon_melee_touch_time_after_hit</code></summary>

default: `"5.0"`  
flags: `0x84000`  
</details>
<details>
<summary><code>mp_weapon_next_owner_touch_time</code></summary>

default: `"1.3"`  
flags: `0x84000`  
</details>
<details>
<summary><code>mp_weapon_prev_owner_touch_time</code></summary>

default: `"1.5"`  
flags: `0x84000`  
</details>
<details>
<summary><code>mp_weapon_self_inflict_amount</code></summary>

If Set to non-0, will hurt the attacker by the specified fraction of max damage if they miss.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_weapons_allow_heavy</code></summary>

Determines which team, if any, can purchase Heavy guns. -1 = any; 0 = non; 2 = Ts; 3 = CTs.

default: `"-1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_weapons_allow_map_placed</code></summary>

If this convar is set, when a match starts, the game will not delete weapons placed in the map.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_weapons_allow_pistols</code></summary>

Determines which team, if any, can purchase Pistols. -1 = any; 0 = non; 2 = Ts; 3 = CTs.

default: `"-1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_weapons_allow_rifles</code></summary>

Determines which team, if any, can purchase Rifles. -1 = any; 0 = non; 2 = Ts; 3 = CTs.

default: `"-1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_weapons_allow_smgs</code></summary>

Determines which team, if any, can purchase SMGs. -1 = any; 0 = non; 2 = Ts; 3 = CTs.

default: `"-1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_weapons_allow_typecount</code></summary>

Determines how many purchases of each weapon type allowed per player per round (0 to disallow purchasing, -1 to have no limit).

default: `"5"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_weapons_allow_zeus</code></summary>

Determines how many Zeus purchases a player can make per round (0 to disallow, -1 to have no limit).

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_weapons_glow_on_ground</code></summary>

If this convar is set, weapons on the ground will have a glow around them.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_weapons_max_gun_purchases_per_weapon_per_match</code></summary>

Max number of times a player may purchase any weapon per match

default: `"-1"`  
flags: `0x82004`  
min value: `-1`  
max value: `1`  
</details>
<details>
<summary><code>mp_weaponstay</code></summary>

default: `"0"`  
flags: `0x100`  
</details>
<details>
<summary><code>mp_win_panel_display_time</code></summary>

The amount of time to show the win panel between matches / halfs

default: `"3"`  
flags: `0x82000`  
</details>
<details>
<summary><code>mp_winlimit</code></summary>

Max score one team can reach before server changes maps

default: `"0"`  
flags: `0x2100`  
min value: `0`  
</details>
<details>
<summary><code>nav_area_bgcolor</code></summary>

RGBA color to draw as the background color for nav areas while editing.

default: `"0 0 0 30"`  
flags: `0x4000`  
</details>
<details>
<summary><code>nav_area_max_size</code></summary>

Max area size created in nav generation

default: `"50"`  
flags: `0x4000`  
</details>
<details>
<summary><code>nav_coplanar_slope_limit</code></summary>

Specifies the item whitelist file to use.

default: `"0.99"`  
flags: `0x4000`  
</details>
<details>
<summary><code>nav_coplanar_slope_limit_displacement</code></summary>

default: `"0.7"`  
flags: `0x4000`  
</details>
<details>
<summary><code>nav_corner_adjust_adjacent</code></summary>

radius used to raise/lower corners in nearby areas when raising/lowering corners.

default: `"18"`  
flags: `0x4000`  
</details>
<details>
<summary><code>nav_create_area_at_feet</code></summary>

Anchor nav_begin_area Z to editing player's feet

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>nav_create_place_on_ground</code></summary>

If true, nav areas will be placed flush with the ground when created by hand.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>nav_debug_blocked</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>nav_displacement_test</code></summary>

Checks for nodes embedded in displacements (useful for in-development maps)

default: `"10000"`  
flags: `0x4000`  
</details>
<details>
<summary><code>nav_drag_selection_volume_zmax_offset</code></summary>

The offset of the nav drag volume top from center

default: `"32"`  
flags: `0x2000`  
</details>
<details>
<summary><code>nav_drag_selection_volume_zmin_offset</code></summary>

The offset of the nav drag volume bottom from center

default: `"32"`  
flags: `0x2000`  
</details>
<details>
<summary><code>nav_draw_limit</code></summary>

The maximum number of areas to draw in edit mode

default: `"500"`  
flags: `0x4000`  
</details>
<details>
<summary><code>nav_edit</code></summary>

Set to one to interactively edit the Navigation Mesh. Set to zero to leave edit mode.

default: `"0"`  
flags: `0x4004`  
</details>
<details>
<summary><code>nav_generate_fencetops</code></summary>

Autogenerate nav areas on fence and obstacle tops

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>nav_generate_fixup_jump_areas</code></summary>

Convert obsolete jump areas into 2-way connections

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>nav_generate_incremental_range</code></summary>

default: `"2000"`  
flags: `0x4000`  
</details>
<details>
<summary><code>nav_generate_incremental_tolerance</code></summary>

Z tolerance for adding new nav areas.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>nav_max_view_distance</code></summary>

Maximum range for precomputed nav mesh visibility (0 = default 1500 units)

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>nav_max_vis_delta_list_length</code></summary>

default: `"64"`  
flags: `0x4000`  
</details>
<details>
<summary><code>nav_potentially_visible_dot_tolerance</code></summary>

default: `"0.98"`  
flags: `0x4000`  
</details>
<details>
<summary><code>nav_quicksave</code></summary>

Set to one to skip the time consuming phases of the analysis.  Useful for data collection and testing.

default: `"0"`  
flags: `0x4004`  
</details>
<details>
<summary><code>nav_show_approach_points</code></summary>

Show Approach Points in the Navigation Mesh.

default: `"0"`  
flags: `0x4004`  
</details>
<details>
<summary><code>nav_show_area_info</code></summary>

Duration in seconds to show nav area ID and attributes while editing

default: `"0.5"`  
flags: `0x4000`  
</details>
<details>
<summary><code>nav_show_compass</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>nav_show_continguous</code></summary>

Highlight non-contiguous connections

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>nav_show_danger</code></summary>

Show current 'danger' levels.

default: `"0"`  
flags: `0x4004`  
</details>
<details>
<summary><code>nav_show_light_intensity</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>nav_show_node_grid</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>nav_show_node_id</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>nav_show_nodes</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>nav_show_player_counts</code></summary>

Show current player counts in each area.

default: `"0"`  
flags: `0x4004`  
</details>
<details>
<summary><code>nav_show_potentially_visible</code></summary>

Show areas that are potentially visible from the current nav area

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>nav_slope_limit</code></summary>

The ground unit normal's Z component must be greater than this for nav areas to be generated.

default: `"0.7"`  
flags: `0x4000`  
</details>
<details>
<summary><code>nav_slope_tolerance</code></summary>

The ground unit normal's Z component must be this close to the nav area's Z component to be generated.

default: `"0.1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>nav_snap_to_grid</code></summary>

Snap to the nav generation grid when creating new nav areas

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>nav_solid_props</code></summary>

Make props solid to nav generation/editing

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>nav_split_place_on_ground</code></summary>

If true, nav areas will be placed flush with the ground when split.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>nav_test_node</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>nav_test_node_crouch</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>nav_test_node_crouch_dir</code></summary>

default: `"4"`  
flags: `0x4000`  
</details>
<details>
<summary><code>nav_update_visibility_on_edit</code></summary>

If nonzero editing the mesh will incrementally recompue visibility

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>nextlevel</code></summary>

If set to a valid map name, will trigger a changelevel to the specified map at the end of the round

default: `""`  
flags: `0x82100`  
</details>
<details>
<summary><code>nextmap_print_enabled</code></summary>

When enabled prints next map to clients

default: `"0"`  
flags: `0x80004`  
</details>
<details>
<summary><code>nextmode</code></summary>

Sets the game mode to be played when the next level loads

default: `""`  
flags: `0x82100`  
</details>
<details>
<summary><code>noclip_fixup</code></summary>

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>npc_ally_deathmessage</code></summary>

Set to 1 to see debug related to the Question & Answer system used to create conversations between allied NPCs.

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>npc_height_adjust</code></summary>

Enable test mode for ik height adjustment

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>occlusion_test_camera_margins</code></summary>

Amount by which the camera (viewer's eye) is expanded for occlusion test. This should be large enough to accommodate eye's movement within a frame or two

default: `"12"`  
flags: `0x80000`  
</details>
<details>
<summary><code>occlusion_test_jump_margin</code></summary>

Amount by which the player bounding box is expanded up for occlusion test to account for jumping. This margin should be large enough to accommodate player movement within a frame or two. Affects both camera box and player box.

default: `"12"`  
flags: `0x80000`  
</details>
<details>
<summary><code>occlusion_test_shadow_length</code></summary>

Max length of completely occluded shadow to consider a player for occlusion test. If shadow provably stops at this distance, the player may be considered for occlusion test. For longer shadows, we just don't do occlusion because we are not likely to find full occlusion when one of the boxes is expanded too much.

default: `"144"`  
flags: `0x80000`  
</details>
<details>
<summary><code>old_radiusdamage</code></summary>

default: `"0.0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>panel_test_title_safe</code></summary>

Test vgui panel positioning with title safe indentation

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
<summary><code>phys_debug_check_contacts</code></summary>

default: `"0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>phys_headshotscale</code></summary>

Modifier for the headshot impulse hits on players

default: `"1.3"`  
flags: `0x2000`  
</details>
<details>
<summary><code>phys_playerscale</code></summary>

This multiplies the bullet impact impuse on players for more dramatic results when players are shot.

default: `"10.0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>phys_pushscale</code></summary>

1 - show hitches , 2 - show stalls

default: `"1"`  
flags: `0x2000`  
</details>
<details>
<summary><code>phys_show_active</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>player_debug_print_damage</code></summary>

When true, print amount and type of all damage received by player to console.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>player_ping_throttle_decay</code></summary>

Decay for how fast the ping throttle delay will decay

default: `"0.58"`  
flags: `0x84000`  
</details>
<details>
<summary><code>post_jump_crouch</code></summary>

This determines how long the third person player character will crouch for after landing a jump.  This only affects the third person animation visuals.

default: `"0.2f"`  
flags: `0x6000`  
</details>
<details>
<summary><code>props_break_max_pieces</code></summary>

Maximum prop breakable piece count (-1 = model default)

default: `"-1"`  
flags: `0x2000`  
</details>
<details>
<summary><code>props_break_max_pieces_perframe</code></summary>

Maximum prop breakable piece count per frame (-1 = model default)

default: `"-1"`  
flags: `0x2000`  
</details>
<details>
<summary><code>pvs_min_player_distance</code></summary>

Min distance to player at which PVS is used. At closer distances, PVS assumes we can see a shadow or something else from the player, so it's safer to just always be "Visible"

default: `"1500"`  
flags: `0x80000`  
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
<summary><code>r_JeepFOV</code></summary>

default: `"90"`  
flags: `0x6000`  
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
<summary><code>r_JeepViewZHeight</code></summary>

default: `"10.0"`  
flags: `0x6100`  
</details>
<details>
<summary><code>r_VehicleViewDampen</code></summary>

default: `"1"`  
flags: `0x6100`  
</details>
<details>
<summary><code>r_vehicleBrakeRate</code></summary>

If enabled, prints diagnostic information about the current fog volume

default: `"1.5"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_visualizetraces</code></summary>

If set, notarget will cause entities to never think they are in the pvs

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>radarvisdistance</code></summary>

at this distance and beyond you need to be point right at someone to see them

default: `"1000.0f"`  
flags: `0x4000`  
min value: `10`  
</details>
<details>
<summary><code>radarvismaxdot</code></summary>

how closely you have to point at someone to see them beyond max distance

default: `".996"`  
flags: `0x4000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>radarvismethod</code></summary>

0 for traditional method, 1 for more realistic method

default: `"1"`  
flags: `0x4000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>radarvispow</code></summary>

the degree to which you can point away from a target, and still see them on radar.

default: `".4"`  
flags: `0x4000`  
</details>
<details>
<summary><code>rr_followup_maxdist</code></summary>

'then ANY' or 'then ALL' response followups will be dispatched only to characters within this distance.

default: `"1800"`  
flags: `0x4000`  
</details>
<details>
<summary><code>rr_remarkable_max_distance</code></summary>

AIs will not even consider remarkarbles that are more than this many units away.

default: `"1200"`  
flags: `0x4000`  
</details>
<details>
<summary><code>rr_remarkable_world_entities_replay_limit</code></summary>

TLK_REMARKs will be dispatched no more than this many times for any given info_remarkable

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>rr_remarkables_enabled</code></summary>

If 1, polling for info_remarkables and issuances of TLK_REMARK is enabled.

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>rr_thenany_score_slop</code></summary>

When computing respondents for a 'THEN ANY' rule, all rule-matching scores within this much of the best score will be considered.

default: `"0.0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>scene_clientflex</code></summary>

Do client side flex animation.

default: `"1"`  
flags: `0x2000`  
</details>
<details>
<summary><code>scene_print</code></summary>

When playing back a scene, print timing and event info to console.

default: `"0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>scene_showfaceto</code></summary>

When playing back, show the directions of faceto events.

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>scene_showlook</code></summary>

When playing back, show the directions of look events.

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>scene_showmoveto</code></summary>

When moving, show the end location.

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>scene_showunlock</code></summary>

Show when a vcd is playing but normal AI is running.

default: `"0"`  
flags: `0x80`  
</details>
<details>
<summary><code>servercfgfile</code></summary>

default: `"server.cfg"`  
flags: `0x80000`  
</details>
<details>
<summary><code>showtriggers</code></summary>

Shows trigger brushes

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>sk_autoaim_mode</code></summary>

default: `"1"`  
flags: `0x2080`  
</details>
<details>
<summary><code>skill</code></summary>

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>smoothstairs</code></summary>

Smooth player eye z coordinate when traversing stairs.

default: `"1"`  
flags: `0x2000`  
</details>
<details>
<summary><code>snd_music_boost</code></summary>

Specifies an amount to boost music volume by

default: `"0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>snd_prevent_ss_duplicates</code></summary>

switch to en/disable the prevention of splitscreen audio file duplicates


default: `"0"`  
flags: `0x6002`  
</details>
<details>
<summary><code>snd_sos_show_server_xmit</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>soundpatch_captionlength</code></summary>

How long looping soundpatch captions should display for.

default: `"2.0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>soundscape_debug</code></summary>

When on, draws lines to all env_soundscape entities. Green lines show the active soundscape, red lines show soundscapes that aren't in range, and white lines show soundscapes that are in range, but not the active soundscape.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>spec_allow_roaming</code></summary>

If nonzero, allow free-roaming spectator camera.

default: `"0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>spec_freeze_deathanim_time</code></summary>

The time that the death cam will spend watching the player's ragdoll before going into the freeze death cam.

default: `"0.8"`  
flags: `0x82000`  
</details>
<details>
<summary><code>spec_freeze_panel_extended_time</code></summary>

Time spent with the freeze panel still up after observer freeze cam is done.

default: `"0.0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>spec_freeze_target_fov</code></summary>

The target FOV that the deathcam should use.

default: `"42"`  
flags: `0x6000`  
</details>
<details>
<summary><code>spec_freeze_target_fov_long</code></summary>

The target FOV that the deathcam should use when the cam zoom far away on the target.

default: `"90"`  
flags: `0x6000`  
</details>
<details>
<summary><code>spec_freeze_time</code></summary>

Time spend frozen in observer freeze cam.

default: `"3.0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>spec_freeze_time_lock</code></summary>

Time players are prevented from skipping the freeze cam

default: `"1.0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>spec_freeze_traveltime</code></summary>

Time taken to zoom in to frame a target in observer freeze cam.

default: `"0.3"`  
flags: `0x82000`  
min value: `0.01`  
</details>
<details>
<summary><code>spec_replay_bot</code></summary>

Enable Spectator Hltv Replay when killed by bot

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>spec_replay_cam_delay</code></summary>

Hltv Replay delay in seconds

default: `"5"`  
flags: `0x80000`  
</details>
<details>
<summary><code>spec_replay_cam_options</code></summary>

Debug options for replay cam

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>spec_replay_round_delay</code></summary>

Round can be delayed by this much due to someone watching a replay; must be at least 3-4 seconds, otherwise the last replay will always be interrupted by round start, assuming normal pause between round_end and round_start events (7 seconds) and freezecam delay (2 seconds) and 7.4 second full replay (5.4 second pre-death and ~2 seconds post-death) and replay in/out switching (up to a second)

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>spec_replay_winddown_time</code></summary>

The trailing time, in seconds, of replay past the event, including fade-out

default: `"2"`  
flags: `0x80000`  
</details>
<details>
<summary><code>steam_controller_haptics</code></summary>

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>steamworks_sessionid_server</code></summary>

The server session ID for the new steamworks gamestats.

default: `"0"`  
flags: `0x2010`  
</details>
<details>
<summary><code>suitvolume</code></summary>

default: `"0.25"`  
flags: `0x80`  
</details>
<details>
<summary><code>sv_accelerate</code></summary>

Linear acceleration amount (old value is 5.6)

default: `"5.5"`  
flags: `0x82100`  
</details>
<details>
<summary><code>sv_accelerate_debug_speed</code></summary>

default: `"0"`  
flags: `0x82100`  
</details>
<details>
<summary><code>sv_accelerate_use_weapon_speed</code></summary>

default: `"1"`  
flags: `0x82100`  
</details>
<details>
<summary><code>sv_air_max_horizontal_parachute_ratio</code></summary>

default: `"0.87"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_air_max_horizontal_parachute_speed</code></summary>

default: `"240"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_air_max_wishspeed</code></summary>

default: `"30"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_air_pushaway_dist</code></summary>

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_airaccelerate</code></summary>

default: `"12"`  
flags: `0x82100`  
</details>
<details>
<summary><code>sv_airaccelerate_parachute</code></summary>

default: `"2.6"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_airaccelerate_rappel</code></summary>

default: `"2.2"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_allchat</code></summary>

Players can receive all other players' text chat, team restrictions apply

default: `"1"`  
flags: `0x80100`  
</details>
<details>
<summary><code>sv_allow_thirdperson</code></summary>

Allows the server set players in third person mode without the client slamming it back (if cheats are on, all clients can set thirdperson without this convar being set)

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_allow_votes</code></summary>

Allow voting?

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_alltalk</code></summary>

Deprecated. Replaced with sv_talk_enemy_dead and sv_talk_enemy_living.

default: `"0"`  
flags: `0x82100`  
</details>
<details>
<summary><code>sv_arms_race_vote_to_restart_disallowed_after</code></summary>

Arms Race gun level after which vote to restart is disallowed

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_auto_adjust_bot_difficulty</code></summary>

Adjust the difficulty of bots each round based on contribution score.

default: `"1"`  
flags: `0x80004`  
</details>
<details>
<summary><code>sv_auto_full_alltalk_during_warmup_half_end</code></summary>

When enabled will automatically turn on full all talk mode in warmup, at halftime and at the end of the match

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_autobunnyhopping</code></summary>

Players automatically re-jump while holding jump button

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_autobuyammo</code></summary>

Enable automatic ammo purchase when inside buy zones during buy periods

default: `"0"`  
flags: `0x82100`  
</details>
<details>
<summary><code>sv_backspeed</code></summary>

How much to slow down backwards motion

default: `"0.6"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_bonus_challenge</code></summary>

Set to values other than 0 to select a bonus map challenge type.

default: `"0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>sv_bot_buy_decoy_weight</code></summary>

Given a bot will buy a grenade, controls the odds of the grenade type. Proportional to all other sv_bot_buy_*_weight convars.

default: `"1"`  
flags: `0x80004`  
min value: `0`  
</details>
<details>
<summary><code>sv_bot_buy_flash_weight</code></summary>

Given a bot will buy a grenade, controls the odds of the grenade type. Proportional to all other sv_bot_buy_*_weight convars.

default: `"1"`  
flags: `0x80004`  
min value: `0`  
</details>
<details>
<summary><code>sv_bot_buy_grenade_chance</code></summary>

Chance bots will buy a grenade with leftover money (after prim, sec and armor). Input as percent (0-100.0)

default: `"33"`  
flags: `0x80004`  
min value: `0`  
max value: `100`  
</details>
<details>
<summary><code>sv_bot_buy_hegrenade_weight</code></summary>

Given a bot will buy a grenade, controls the odds of the grenade type. Proportional to all other sv_bot_buy_*_weight convars.

default: `"6"`  
flags: `0x80004`  
min value: `0`  
</details>
<details>
<summary><code>sv_bot_buy_molotov_weight</code></summary>

Given a bot will buy a grenade, controls the odds of the grenade type. Proportional to all other sv_bot_buy_*_weight convars.

default: `"1"`  
flags: `0x80004`  
min value: `0`  
</details>
<details>
<summary><code>sv_bot_buy_smoke_weight</code></summary>

Given a bot will buy a grenade, controls the odds of the grenade type. Proportional to all other sv_bot_buy_*_weight convars.

default: `"1"`  
flags: `0x80004`  
min value: `0`  
</details>
<details>
<summary><code>sv_bot_difficulty_gamepad</code></summary>

Bot difficulty while playing with Gamepad device

default: `"0"`  
flags: `0x2010`  
</details>
<details>
<summary><code>sv_bot_difficulty_hydra</code></summary>

Bot difficulty while playing with Hydra device

default: `"0"`  
flags: `0x2010`  
</details>
<details>
<summary><code>sv_bot_difficulty_kbm</code></summary>

Bot difficulty while playing with Keyboard/Mouse device

default: `"0"`  
flags: `0x2010`  
</details>
<details>
<summary><code>sv_bot_difficulty_ps3move</code></summary>

Bot difficulty while playing with PS3Move device

default: `"0"`  
flags: `0x2010`  
</details>
<details>
<summary><code>sv_bot_difficulty_sharpshooter</code></summary>

Bot difficulty while playing with SharpShooter device

default: `"0"`  
flags: `0x2010`  
</details>
<details>
<summary><code>sv_bots_force_rebuy_every_round</code></summary>

If set, this strips the bots of their weapons every round and forces them to rebuy.

default: `"0"`  
flags: `0x80004`  
</details>
<details>
<summary><code>sv_bots_get_easier_each_win</code></summary>

If > 0, some # of bots will lower thier difficulty each time they win. The argument defines how many will lower their difficulty each time.

default: `"0"`  
flags: `0x80004`  
</details>
<details>
<summary><code>sv_bots_get_harder_after_each_wave</code></summary>

If > 0, some # of bots will raise thier difficulty each time CTs beat a Guardian wave. The argument defines how many will raise their difficulty each time

default: `"0"`  
flags: `0x80004`  
</details>
<details>
<summary><code>sv_bounce</code></summary>

Bounce multiplier for when physically simulated objects collide with other objects.

default: `"0"`  
flags: `0x82100`  
</details>
<details>
<summary><code>sv_breachcharge_arm_delay</code></summary>

default: `"0.3"`  
flags: `0x80004`  
</details>
<details>
<summary><code>sv_breachcharge_delay_max</code></summary>

default: `"0.8"`  
flags: `0x80004`  
</details>
<details>
<summary><code>sv_breachcharge_delay_min</code></summary>

default: `"0"`  
flags: `0x80004`  
</details>
<details>
<summary><code>sv_breachcharge_distance_max</code></summary>

default: `"1200"`  
flags: `0x80004`  
</details>
<details>
<summary><code>sv_breachcharge_distance_min</code></summary>

default: `"600"`  
flags: `0x80004`  
</details>
<details>
<summary><code>sv_breachcharge_fuse_max</code></summary>

default: `"1.0"`  
flags: `0x80004`  
</details>
<details>
<summary><code>sv_breachcharge_fuse_min</code></summary>

default: `"0.7"`  
flags: `0x80004`  
</details>
<details>
<summary><code>sv_broadcast_ugc_download_progress_interval</code></summary>

default: `"8"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_broadcast_ugc_downloads</code></summary>

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_bumpmine_arm_delay</code></summary>

default: `"0.3"`  
flags: `0x80004`  
</details>
<details>
<summary><code>sv_bumpmine_detonate_delay</code></summary>

default: `"0.25"`  
flags: `0x80004`  
</details>
<details>
<summary><code>sv_buy_status_override</code></summary>

Override for buy status map info. 0 = everyone can buy, 1 = ct only, 2 = t only 3 = nobody

default: `"-1"`  
flags: `0x82004`  
</details>
<details>
<summary><code>sv_chat_proximity</code></summary>

default: `"-1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_clamp_unsafe_velocities</code></summary>

Whether the server will attempt to clamp velocities that could cause physics bugs or crashes.

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_clip_penetration_traces_to_players</code></summary>

default: `"1"`  
flags: `0x2000`  
</details>
<details>
<summary><code>sv_clockcorrection_msecs</code></summary>

The server tries to keep each player's m_nTickBase withing this many msecs of the server absolute tickcount

default: `"30"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_coach_comm_unrestricted</code></summary>

When set, ignores coach communication restrictions.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_coaching_enabled</code></summary>

Allows spectating and communicating with a team ( 'coach t' or 'coach ct' )

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_competitive_minspec</code></summary>

Enable to force certain client convars to minimum/maximum values to help prevent competitive advantages.

default: `"1"`  
flags: `0x82100`  
</details>
<details>
<summary><code>sv_competitive_official_5v5</code></summary>

Enable to force the server to show 5v5 scoreboards and allows spectators to see characters through walls.

default: `"0"`  
flags: `0x82100`  
</details>
<details>
<summary><code>sv_compute_per_bot_difficulty</code></summary>

0 = compute all bot difficulties equally, 1 = compute unique bot difficulty for each bot 

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_cs_player_speed_has_hostage</code></summary>



default: `"200"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_ct_spawn_on_bombsite</code></summary>

Force cts to spawn on a bombsite

default: `"-1"`  
flags: `0x80004`  
</details>
<details>
<summary><code>sv_damage_print_enable</code></summary>

Turn this off to disable the player's damage feed in the console after getting killed.

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_dc_friends_reqd</code></summary>

Set this to 0 to allow direct connects to a game in progress even if no presents are present

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_deadtalk</code></summary>

Dead players can speak (voice, text) to the living

default: `"0"`  
flags: `0x82100`  
</details>
<details>
<summary><code>sv_debug_player_use</code></summary>

Visualizes +use logic. Green cross=trace success, Red cross=trace too far, Green box=radius success

default: `"0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>sv_debug_ugc_downloads</code></summary>

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_disable_immunity_alpha</code></summary>

If set, clients won't slam the player model render settings each frame for immunity [mod authors use this]

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_disable_motd</code></summary>

Prevent the motd from showing.

default: `"1"`  
flags: `0x6002`  
</details>
<details>
<summary><code>sv_disable_observer_interpolation</code></summary>

Disallow interpolating between observer targets on this server.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_disable_pas</code></summary>

default: `"1"`  
flags: `0x86000`  
</details>
<details>
<summary><code>sv_disable_querycache</code></summary>

debug - disable trace query cache

default: `"0"`  
flags: `0x6002`  
</details>
<details>
<summary><code>sv_disable_radar</code></summary>

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_disablefreezecam</code></summary>

Turn on/off freezecam on server

default: `"0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>sv_drowning_damage_initial</code></summary>

default: `"2"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_drowning_damage_max</code></summary>

default: `"5"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_dz_autojointeam</code></summary>

Whether players are automatically assigned a DZ team

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_dz_cash_bundle_size</code></summary>

Size of a cash bundle

default: `"50"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_dz_contractkill_reward</code></summary>

Cash bundles to award for a successful contract kill

default: `"10"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_dz_enable_respawn</code></summary>

Missile wave number to prevent respawns, >5 will allow respawns on all waves

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_dz_exploration_payment_amount</code></summary>

Number of cash bundles to award for exploring a new sector

default: `"2"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_dz_exploration_payment_amount_bonus</code></summary>

Number of BONUS cash bundles to award for exploring (if the player has the item/upgrade)

default: `"2"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_dz_hostage_rescue_reward</code></summary>

Number of cash bundles to award for rescuing a hostage

default: `"18"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_dz_jointeam_allowed</code></summary>

Whether non-server admins are allowed to use the dz_jointeam command

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_dz_parachute_reuse</code></summary>



default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_dz_player_max_health</code></summary>

default: `"120"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_dz_player_spawn_armor</code></summary>

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_dz_player_spawn_health</code></summary>

default: `"120"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_dz_show_enemy_name_scope_range</code></summary>

default: `"800"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_dz_squad_wipe_reward</code></summary>

Number of cash bundles to award for eliminating a squad

default: `"2"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_dz_team_count</code></summary>

Number of players per team

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_dz_warmup_tablet</code></summary>



default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_dz_warmup_weapon</code></summary>



default: `"weapon_glock"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_dz_zone_bombdrop_money_reward</code></summary>

How many money stacks players are rewarded each danger zone wave

default: `"15"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_dz_zone_bombdrop_money_reward_bonus</code></summary>

How many bonus money stacks players are rewarded each danger zone wave when they have the bonus item

default: `"5"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_dz_zone_damage</code></summary>

default: `"1"`  
flags: `0x84000`  
</details>
<details>
<summary><code>sv_dz_zone_hex_radius</code></summary>

default: `"2200"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_enablebunnyhopping</code></summary>

Allow player speed to exceed maximum running speed

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_endmatch_item_drop_interval</code></summary>

The time between drops on the end match scoreboard 

default: `"1.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_endmatch_item_drop_interval_ancient</code></summary>

The time between drops on the end match scoreboard for ancient items 

default: `"3.5"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_endmatch_item_drop_interval_legendary</code></summary>

The time between drops on the end match scoreboard for legendary items 

default: `"2.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_endmatch_item_drop_interval_mythical</code></summary>

The time between drops on the end match scoreboard for mythical items 

default: `"1.25"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_endmatch_item_drop_interval_rare</code></summary>

The time between drops on the end match scoreboard for rare items 

default: `"1.0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_env_entity_makers_enabled</code></summary>



default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_exojump_jumpbonus_forward</code></summary>

ExoJump forwards velocity bonus when duck jumping

default: `"0.4"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_exojump_jumpbonus_up</code></summary>

ExoJump upwards bonus when holding the jump button

default: `"0.58"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_exojump_soundramp</code></summary>



default: `"20.0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>sv_exostaminajumpcost</code></summary>

Stamina penalty for jumping with exo legs

default: `".040"`  
flags: `0x82000`  
min value: `0`  
</details>
<details>
<summary><code>sv_exostaminalandcost</code></summary>

Stamina penalty for landing with exo legs

default: `".015"`  
flags: `0x82000`  
min value: `0`  
</details>
<details>
<summary><code>sv_extract_ammo_from_dropped_weapons</code></summary>

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_extreme_strafe_accuracy_fishtail</code></summary>

Number of degrees of aim 'fishtail' when making an extreme strafe direction change

default: `"0"`  
flags: `0x2002`  
min value: `-5`  
max value: `5`  
</details>
<details>
<summary><code>sv_falldamage_exojump_multiplier</code></summary>

ExoJump fall damage multiplier

default: `"0.4"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_falldamage_scale</code></summary>

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_falldamage_to_below_player_multiplier</code></summary>

Scale damage when distributed across two players

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_falldamage_to_below_player_ratio</code></summary>

Landing on a another player's head gives them this ratio of the damage.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_fistpoint_delay</code></summary>

default: `"1.8"`  
flags: `0x2000`  
</details>
<details>
<summary><code>sv_fistpunch_blocked_damage</code></summary>

default: `"25"`  
flags: `0x2000`  
</details>
<details>
<summary><code>sv_fistpunch_damage</code></summary>

default: `"10"`  
flags: `0x2000`  
</details>
<details>
<summary><code>sv_fistpunch_damage_hard</code></summary>

default: `"20"`  
flags: `0x2000`  
</details>
<details>
<summary><code>sv_fistpunch_damage_to_player_multiplier</code></summary>

default: `"1.5"`  
flags: `0x2000`  
</details>
<details>
<summary><code>sv_fistpunch_impact_sounds</code></summary>

default: `"1"`  
flags: `0x2000`  
</details>
<details>
<summary><code>sv_fistpunch_viewmove</code></summary>

default: `"40"`  
flags: `0x2000`  
</details>
<details>
<summary><code>sv_flashbang_strength</code></summary>

Flashbang strength

default: `"3.55"`  
flags: `0x2000`  
min value: `2`  
max value: `8`  
</details>
<details>
<summary><code>sv_footstep_sound_frequency</code></summary>

How frequent to hear the player's step sound or how fast they appear to be running from first person.

default: `"0.97"`  
flags: `0x6000`  
</details>
<details>
<summary><code>sv_footsteps</code></summary>

Play footstep sound for players

default: `"1"`  
flags: `0x2102`  
</details>
<details>
<summary><code>sv_force_reflections</code></summary>

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_force_transmit_ents</code></summary>

Will transmit all entities to client, regardless of PVS conditions (will still skip based on transmit flags, however).

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_force_transmit_players</code></summary>

Will transmit players to all clients regardless of PVS checks.

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_friction</code></summary>

World friction.

default: `"5.2"`  
flags: `0x82100`  
</details>
<details>
<summary><code>sv_full_alltalk</code></summary>

Any player (including Spectator team) can speak to any other player

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_gameinstructor_disable</code></summary>

Force all clients to disable their game instructors.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_grassburn</code></summary>

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_grenade_trajectory</code></summary>

Shows grenade trajectory visualization in-game.

default: `"0"`  
flags: `0x86000`  
</details>
<details>
<summary><code>sv_grenade_trajectory_dash</code></summary>

Dot-dash style grenade trajectory arc

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_grenade_trajectory_thickness</code></summary>

Visible thickness of grenade trajectory arc

default: `"0.2"`  
flags: `0x82000`  
min value: `0.1`  
max value: `1`  
</details>
<details>
<summary><code>sv_grenade_trajectory_time</code></summary>

Length of time grenade trajectory remains visible.

default: `"20"`  
flags: `0x82000`  
min value: `0.1`  
max value: `20`  
</details>
<details>
<summary><code>sv_grenade_trajectory_time_spectator</code></summary>

Length of time grenade trajectory remains visible as a spectator.

default: `"4"`  
flags: `0x82000`  
min value: `0`  
max value: `8`  
</details>
<details>
<summary><code>sv_guardian_heavy_all</code></summary>

default: `"0"`  
flags: `0x80004`  
</details>
<details>
<summary><code>sv_guardian_heavy_count</code></summary>

default: `"0"`  
flags: `0x80004`  
</details>
<details>
<summary><code>sv_guardian_max_wave_for_heavy</code></summary>

default: `"0"`  
flags: `0x80004`  
</details>
<details>
<summary><code>sv_guardian_min_wave_for_heavy</code></summary>

default: `"0"`  
flags: `0x80004`  
</details>
<details>
<summary><code>sv_health_approach_enabled</code></summary>

Shows that bones that are setup every think

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_health_approach_speed</code></summary>

default: `"10"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_hegrenade_damage_multiplier</code></summary>

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_hegrenade_radius_multiplier</code></summary>

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_holiday_mode</code></summary>

0 = OFF, 1 = Halloween, 2 = Winter

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_ignoregrenaderadio</code></summary>

Turn off Fire in the hole messages

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_infinite_ammo</code></summary>

Player's active weapon will never run out of ammo. If set to 2 then player has infinite total ammo but still has to reload the magazine.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_jump_impulse</code></summary>

Initial upward velocity for player jumps; sqrt(2*gravity*height).

default: `"301.993377"`  
flags: `0x82000`  
min value: `0`  
</details>
<details>
<summary><code>sv_jump_impulse_exojump_multiplier</code></summary>

ExoJump impulse multiplier

default: `"1.05"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_kick_ban_duration</code></summary>

How long should a kick ban from the server should last (in minutes)

default: `"15"`  
flags: `0x82100`  
</details>
<details>
<summary><code>sv_kick_players_with_cooldown</code></summary>

(0: do not kick on insecure servers; 1: kick players with Untrusted status or convicted by Overwatch; 2: kick players with any cooldown)

default: `"1"`  
flags: `0x82004`  
</details>
<details>
<summary><code>sv_knife_attack_extend_from_player_aabb</code></summary>

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_ladder_angle</code></summary>

Cos of angle of incidence to ladder perpendicular for applying ladder_dampen

default: `"-0.707"`  
flags: `0x2000`  
min value: `-1`  
max value: `1`  
</details>
<details>
<summary><code>sv_ladder_dampen</code></summary>

Amount to dampen perpendicular movement on a ladder

default: `"0.2"`  
flags: `0x2000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>sv_ladder_scale_speed</code></summary>

Scale top speed on ladders

default: `"0.78"`  
flags: `0x82000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>sv_lagcompensateself</code></summary>

Player can lag compensate themselves.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>sv_lagcompensationforcerestore</code></summary>

Don't test validity of a lag comp restore, just do it.

default: `"1"`  
flags: `0x4000`  
</details>
<details>
<summary><code>sv_ledge_mantle_helper</code></summary>

1=Only improves success of jump+ducks to windows or vents (jump+duck to duck), 2=Improves success of all jump+ducks to ledges, 3=if you can get your eyes above it, you'll pull yourself up

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_ledge_mantle_helper_debug</code></summary>



default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_ledge_mantle_helper_dzonly</code></summary>

1=only does the feature if running in game mode Danger Zone, 0=Doesn't check game mode to run

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_matchend_drops_enabled</code></summary>

Rewards gameplay time is always accumulated for players, but drops at the end of the match can be prevented

default: `"1"`  
flags: `0x80004`  
</details>
<details>
<summary><code>sv_matchpause_auto_5v5</code></summary>

When enabled will automatically pause the match at next freeze time if less than 5 players are connected on each team.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_max_distance_transmit_footsteps</code></summary>

Maximum distance to transmit footstep sound effects.

default: `"1250.0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>sv_maxspeed</code></summary>

default: `"320"`  
flags: `0x82100`  
</details>
<details>
<summary><code>sv_maxunlag</code></summary>

Maximum lag compensation in seconds

default: `"0.200"`  
flags: `0x80000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>sv_maxusrcmdprocessticks</code></summary>

Maximum number of client-issued usrcmd ticks that can be replayed in packet loss conditions, 0 to allow no restrictions

default: `"16"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_maxusrcmdprocessticks_holdaim</code></summary>

Hold client aim for multiple server sim ticks when client-issued usrcmd contains multiple actions (0: off; 1: hold this server tick; 2+: hold multiple ticks)

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_maxusrcmdprocessticks_warning</code></summary>

Print a warning when user commands get dropped due to insufficient usrcmd ticks allocated, number of seconds to throttle, negative disabled

default: `"-1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_maxvelocity</code></summary>

Maximum speed any ballistically moving object is allowed to attain per axis.

default: `"3500"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_min_jump_landing_sound</code></summary>

default: `"260.0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_netvisdist</code></summary>

Test networking visibility distance

default: `"10000"`  
flags: `0x4002`  
</details>
<details>
<summary><code>sv_noclipaccelerate</code></summary>

default: `"5"`  
flags: `0x2180`  
</details>
<details>
<summary><code>sv_noclipduringpause</code></summary>

If cheats are enabled, then you can noclip with the game paused (for doing screenshots, etc.).

default: `"0"`  
flags: `0x6000`  
</details>
<details>
<summary><code>sv_noclipspeed</code></summary>

default: `"5"`  
flags: `0x2180`  
</details>
<details>
<summary><code>sv_occlude_players</code></summary>

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_optimizedmovement</code></summary>

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_outofammo_indicator</code></summary>

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_party_mode</code></summary>

Party!!

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_penetration_type</code></summary>

What type of penertration to use. 0 = old CS, 1 = new penetration

default: `"1"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_player_parachute_velocity</code></summary>

default: `"-200"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_prime_accounts_only</code></summary>

When this setting is enabled only prime users can connect to this game server.

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_prop_door_open_speed_scale</code></summary>

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_pushaway_clientside</code></summary>

Clientside physics push away (0=off, 1=only localplayer, 1=all players)

default: `"0"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_pushaway_clientside_size</code></summary>

Minimum size of pushback objects

default: `"15"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_pushaway_force</code></summary>

How hard physics objects are pushed away from the players on the server.

default: `"30000"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_pushaway_hostage_force</code></summary>

How hard the hostage is pushed away from physics objects (falls off with inverse square of distance).

default: `"20000"`  
flags: `0x6000`  
</details>
<details>
<summary><code>sv_pushaway_max_force</code></summary>

Maximum amount of force applied to physics objects by players.

default: `"1000"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_pushaway_max_hostage_force</code></summary>

Maximum of how hard the hostage is pushed away from physics objects.

default: `"1000"`  
flags: `0x6000`  
</details>
<details>
<summary><code>sv_pushaway_max_player_force</code></summary>

Maximum of how hard the player is pushed away from physics objects.

default: `"10000"`  
flags: `0x6002`  
</details>
<details>
<summary><code>sv_pushaway_min_player_speed</code></summary>

If a player is moving slower than this, don't push away physics objects (enables ducking behind things).

default: `"75"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_pushaway_player_force</code></summary>

How hard the player is pushed away from physics objects (falls off with inverse square of distance).

default: `"200000"`  
flags: `0x6002`  
</details>
<details>
<summary><code>sv_pvsskipanimation</code></summary>

Skips SetupBones when npc's are outside the PVS

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>sv_regeneration_wait_time</code></summary>

default: `"1.0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>sv_remove_old_ugc_downloads</code></summary>

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_reward_drop_delay</code></summary>

Delay between the end match scoreboard being shown and the beginning of item drops.

default: `"3.0"`  
flags: `0x2000`  
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
<summary><code>sv_server_graphic1</code></summary>

A 360x60 (<16kb) image file in /csgo/ that will be displayed to spectators.

default: `""`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_server_graphic2</code></summary>

A 220x45 (<16kb) image file in /csgo/ that will be displayed to spectators.

default: `""`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_server_verify_blood_on_player</code></summary>

default: `"1"`  
flags: `0x6000`  
</details>
<details>
<summary><code>sv_shield_explosive_damage_cap</code></summary>

default: `"99"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_shield_explosive_damage_crouch_bonus</code></summary>

default: `"10"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_shield_explosive_damage_mindist</code></summary>

default: `"250"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_shield_explosive_damage_mult</code></summary>

default: `"4"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_shield_explosive_damage_scale</code></summary>

default: `"0.5"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_shield_hitpoints</code></summary>

default: `"650"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_show_bot_difficulty_in_name</code></summary>

0 = hide bot difficulty in bot name, 1 = show bot difficulty in bot name

default: `"0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>sv_show_team_equipment_force_on</code></summary>

Force on if not prohibited

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_show_team_equipment_prohibit</code></summary>

Determines whether +cl_show_team_equipment is prohibited.

default: `"0"`  
flags: `0x82100`  
</details>
<details>
<summary><code>sv_show_voip_indicator_for_enemies</code></summary>

Makes it so the voip icon is shown over enemies as well as allies when they are talking

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_showanimstate</code></summary>

Show the (server) animation state for the specified entity (-1 for none).

default: `"-1"`  
flags: `0x4002`  
</details>
<details>
<summary><code>sv_showanimstate_activities</code></summary>

Show activities in the (server) animation state display.

default: `"0"`  
flags: `0x4002`  
</details>
<details>
<summary><code>sv_showanimstate_log</code></summary>

1 to output sv_showanimstate to Msg(). 2 to store in AnimStateServer.log. 3 for both.

default: `"0"`  
flags: `0x4002`  
</details>
<details>
<summary><code>sv_showbullethits</code></summary>

1=show hits and near misses, 2=show hits only

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_showimpacts</code></summary>

Shows client (red) and server (blue) bullet impact point (1=both, 2=client-only, 3=server-only)

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_showimpacts_penetration</code></summary>

Shows extra data when bullets penetrate. (use sv_showimpacts_time to increase time shown)

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_showimpacts_time</code></summary>

Duration bullet impact indicators remain before disappearing

default: `"4"`  
flags: `0x82000`  
min value: `0`  
max value: `10`  
</details>
<details>
<summary><code>sv_showlagcompensation</code></summary>

Show lag compensated hitboxes whenever a player is lag compensated.

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>sv_showlagcompensation_duration</code></summary>

Duration to show lag-compensated hitboxes

default: `"4.0"`  
flags: `0x4000`  
min value: `0`  
max value: `10`  
</details>
<details>
<summary><code>sv_showplayerhitboxes</code></summary>

Show lag compensated hitboxes for the specified player index whenever a player fires.

default: `"0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>sv_skirmish_id</code></summary>

Dedicated server skirmish id to run

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_skyname</code></summary>

Current name of the skybox texture

default: `"sky_urb01"`  
flags: `0x2080`  
</details>
<details>
<summary><code>sv_snowball_strength</code></summary>

Snowball strength

default: `"12.0"`  
flags: `0x2000`  
min value: `2`  
max value: `64`  
</details>
<details>
<summary><code>sv_soundemitter_trace</code></summary>

Show all EmitSound calls including their symbolic name and the actual wave file they resolved to. (-1 = for nobody, 0 = for everybody, n = for one entity)


default: `"-1"`  
flags: `0x2000`  
</details>
<details>
<summary><code>sv_soundemitter_version</code></summary>

specfies what version of soundemitter system to use


default: `"2"`  
flags: `0x6002`  
</details>
<details>
<summary><code>sv_spawn_afk_bomb_drop_time</code></summary>

Players that have never moved since they spawned will drop the bomb after this amount of time.

default: `"15"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_spawn_rappel_min_duration</code></summary>



default: `"8.0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_spawn_rappel_min_duration_with_chute</code></summary>



default: `"2.5"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_spec_hear</code></summary>

Determines who spectators can hear: 0: only spectators; 1: all players; 2: spectated team; 3: self only; 4: nobody

default: `"1"`  
flags: `0x82100`  
</details>
<details>
<summary><code>sv_spec_post_death_additional_time</code></summary>



default: `"0"`  
flags: `0x82000`  
min value: `0`  
max value: `60`  
</details>
<details>
<summary><code>sv_spec_use_tournament_content_standards</code></summary>

default: `"0.0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_specaccelerate</code></summary>

default: `"5"`  
flags: `0x2180`  
</details>
<details>
<summary><code>sv_specnoclip</code></summary>

default: `"1"`  
flags: `0x2180`  
</details>
<details>
<summary><code>sv_specspeed</code></summary>

default: `"3"`  
flags: `0x2180`  
</details>
<details>
<summary><code>sv_staminajumpcost</code></summary>

Stamina penalty for jumping

default: `".080"`  
flags: `0x82000`  
min value: `0`  
</details>
<details>
<summary><code>sv_staminalandcost</code></summary>

Stamina penalty for landing

default: `".050"`  
flags: `0x82000`  
min value: `0`  
</details>
<details>
<summary><code>sv_staminamax</code></summary>

Maximum stamina penalty

default: `"80"`  
flags: `0x82000`  
min value: `0`  
max value: `100`  
</details>
<details>
<summary><code>sv_staminarecoveryrate</code></summary>

Rate at which stamina recovers (units/sec)

default: `"60"`  
flags: `0x82000`  
min value: `0`  
</details>
<details>
<summary><code>sv_standable_normal</code></summary>

default: `"0.7"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_stepsize</code></summary>

default: `"18"`  
flags: `0x2102`  
</details>
<details>
<summary><code>sv_stopspeed</code></summary>

Minimum stopping speed when on ground.

default: `"80"`  
flags: `0x82100`  
</details>
<details>
<summary><code>sv_suppress_viewpunch</code></summary>

default: `"0"`  
flags: `0x6002`  
</details>
<details>
<summary><code>sv_tablet_show_path_to_nearest_resq</code></summary>

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_talk_after_dying_time</code></summary>

The number of seconds a player can continue talking after dying as if they were still alive

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_talk_enemy_dead</code></summary>

Dead players can hear all dead enemy communication (voice, chat)

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_talk_enemy_living</code></summary>

Living players can hear all living enemy communication (voice, chat)

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_teamid_overhead</code></summary>

Shows teamID over player's heads.  0 = off, 1 = on

default: `"1"`  
flags: `0x82100`  
</details>
<details>
<summary><code>sv_teamid_overhead_always_prohibit</code></summary>

Determines whether cl_teamid_overhead_always is prohibited.

default: `"0"`  
flags: `0x82100`  
</details>
<details>
<summary><code>sv_teamid_overhead_maxdist</code></summary>

If >0, server will override cl_teamid_overhead_maxdist

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_teamid_overhead_maxdist_spec</code></summary>

If >0, server will override cl_teamid_overhead_maxdist_spec

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_timebetweenducks</code></summary>

Minimum time before recognizing consecutive duck key

default: `"0.4"`  
flags: `0x82000`  
min value: `0`  
max value: `2`  
</details>
<details>
<summary><code>sv_turbophysics</code></summary>

Turns on turbo physics

default: `"0"`  
flags: `0x2000`  
</details>
<details>
<summary><code>sv_turning_inaccuracy_angle_min</code></summary>

default: `"4"`  
flags: `0x86000`  
</details>
<details>
<summary><code>sv_turning_inaccuracy_decay</code></summary>

default: `"0.8"`  
flags: `0x86000`  
</details>
<details>
<summary><code>sv_turning_inaccuracy_enabled</code></summary>

default: `"0"`  
flags: `0x86000`  
</details>
<details>
<summary><code>sv_ugc_manager_max_new_file_check_interval_secs</code></summary>

default: `"1000"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_unlockedchapters</code></summary>

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>sv_usercmd_custom_random_seed</code></summary>

When enabled server will populate an additional random seed independent of the client

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_voice_proximity</code></summary>

default: `"-1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_voice_proximity_positional</code></summary>

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_vote_allow_in_warmup</code></summary>

Allow voting during warmup?

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_vote_allow_spectators</code></summary>

Allow spectators to initiate votes?

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_vote_command_delay</code></summary>

How long after a vote passes until the action happens

default: `"2"`  
flags: `0x80000`  
max value: `4.5`  
</details>
<details>
<summary><code>sv_vote_count_spectator_votes</code></summary>

Allow spectators to vote on issues?

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_vote_creation_timer</code></summary>

How often someone can individually call a vote.

default: `"120"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_vote_disallow_kick_on_match_point</code></summary>

Disallow vote kicking on the match point round.

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_vote_failure_timer</code></summary>

A vote that fails cannot be re-submitted for this long

default: `"300"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_vote_issue_kick_allowed</code></summary>

Can people hold votes to kick players from the server?

default: `"1"`  
flags: `0x82100`  
</details>
<details>
<summary><code>sv_vote_issue_loadbackup_allowed</code></summary>

Can people hold votes to load match from backup?

default: `"1"`  
flags: `0x82100`  
</details>
<details>
<summary><code>sv_vote_issue_loadbackup_spec_authoritative</code></summary>

When enabled, admins load match from backup without players vote

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_vote_issue_loadbackup_spec_only</code></summary>

When enabled, only admins load match from backup

default: `"0"`  
flags: `0x82100`  
</details>
<details>
<summary><code>sv_vote_issue_loadbackup_spec_safe</code></summary>

When enabled, admins load match from backup in safe time of the round only

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_vote_issue_pause_match_spec_only</code></summary>

When enabled, only admins start technical pause

default: `"0"`  
flags: `0x82100`  
</details>
<details>
<summary><code>sv_vote_issue_restart_game_allowed</code></summary>

Can people hold votes to restart the game?

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_vote_kick_ban_duration</code></summary>

How long should a kick vote ban someone from the server? (in minutes)

default: `"15"`  
flags: `0x82100`  
</details>
<details>
<summary><code>sv_vote_quorum_ratio</code></summary>

The minimum ratio of players needed to vote on an issue to resolve it.

default: `"0.501"`  
flags: `0x80000`  
min value: `0.01`  
max value: `1`  
</details>
<details>
<summary><code>sv_vote_timer_duration</code></summary>

How long to allow voting on an issue

default: `"15"`  
flags: `0x80000`  
</details>
<details>
<summary><code>sv_vote_to_changelevel_before_match_point</code></summary>

Restricts vote to change level to rounds prior to match point (default 0, vote is never disallowed)

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_walkable_normal</code></summary>

default: `"0.7"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_water_movespeed_multiplier</code></summary>

Maximum move magnitude that can be requested by client.

default: `"0.8"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_water_swim_mode</code></summary>

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_wateraccelerate</code></summary>

default: `"10"`  
flags: `0x82102`  
</details>
<details>
<summary><code>sv_waterdist</code></summary>

Vertical view fixup when eyes are near water plane.

default: `"12"`  
flags: `0x2002`  
</details>
<details>
<summary><code>sv_waterfriction</code></summary>

default: `"1"`  
flags: `0x82102`  
</details>
<details>
<summary><code>sv_weapon_encumbrance_per_item</code></summary>

default: `"0.85"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_weapon_encumbrance_scale</code></summary>

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_weapon_require_use_grace_period</code></summary>

default: `"1"`  
flags: `0x80004`  
</details>
<details>
<summary><code>sv_workshop_allow_other_maps</code></summary>

When hosting a workshop collection, users can play other workshop map on this server when it is empty and then mapcycle into this server collection.

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>tablet_c4_dist_max</code></summary>

default: `"3000"`  
flags: `0x82000`  
</details>
<details>
<summary><code>tablet_c4_dist_min</code></summary>

default: `"400"`  
flags: `0x82000`  
</details>
<details>
<summary><code>think_limit</code></summary>

Maximum think time in milliseconds, warning is printed if this is exceeded.

default: `"10"`  
flags: `0x82000`  
</details>
<details>
<summary><code>tr_best_course_time</code></summary>

The player's best time for the timed obstacle course

default: `"0"`  
flags: `0x1008082`  
</details>
<details>
<summary><code>tr_completed_training</code></summary>

Whether the local player has completed the initial training portion of the training map

default: `"0"`  
flags: `0x1008082`  
</details>
<details>
<summary><code>tv_allow_autorecording_index</code></summary>

When >=0 restricts autorecording only to the specified TV index

default: `"-1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>tv_allow_camera_man_steamid</code></summary>

Allows tournament production cameraman to run csgo.exe -interactivecaster on SteamID 7650123456XXX and be the camera man.

default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>tv_allow_camera_man_steamid2</code></summary>

Allows tournament production tv cameraman to run csgo.exe -interactivecaster on SteamID 7650123456XXX and be the tv camera man.

default: `""`  
flags: `0x80000`  
</details>
<details>
<summary><code>tv_allow_static_shots</code></summary>

Auto director uses fixed level cameras for shots

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>tv_delay</code></summary>

GOTV broadcast delay in seconds

default: `"10"`  
flags: `0x80000`  
min value: `0`  
max value: `120`  
</details>
<details>
<summary><code>tv_delay1</code></summary>

GOTV[instance 1] broadcast delay in seconds

default: `"15"`  
flags: `0x80000`  
min value: `0`  
max value: `120`  
</details>
<details>
<summary><code>tv_delaymapchange</code></summary>

Delays map change until broadcast is complete

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>tv_relayradio</code></summary>

Relay team radio commands to TV: 0=off, 1=on

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>tv_relaytextchat</code></summary>

Relay text chat data: 0=off, 1=say, 2=say+say_team

default: `"1"`  
flags: `0x80000`  
</details>
<details>
<summary><code>view_punch_decay</code></summary>

Decay factor exponent for view punch

default: `"18"`  
flags: `0x86000`  
</details>
<details>
<summary><code>view_recoil_tracking</code></summary>

How closely the view tracks with the aim punch from weapon recoil

default: `"0.45"`  
flags: `0x86000`  
</details>
<details>
<summary><code>vis_force</code></summary>

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>vismon_poll_frequency</code></summary>

default: `".5"`  
flags: `0x4000`  
</details>
<details>
<summary><code>vismon_trace_limit</code></summary>

default: `"12"`  
flags: `0x4000`  
</details>
<details>
<summary><code>voice_player_speaking_delay_threshold</code></summary>

default: `"0.5f"`  
flags: `0x4000`  
</details>
<details>
<summary><code>weapon_accuracy_forcespread</code></summary>

Force spread to the specified value.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>weapon_accuracy_logging</code></summary>

default: `"0"`  
flags: `0x2082`  
</details>
<details>
<summary><code>weapon_accuracy_nospread</code></summary>

Disable weapon inaccuracy spread

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>weapon_accuracy_shotgun_spread_patterns</code></summary>

default: `"1"`  
flags: `0x82000`  
</details>
<details>
<summary><code>weapon_air_spread_scale</code></summary>

Scale factor for jumping inaccuracy, set to 0 to make jumping accuracy equal to standing

default: `"1.0"`  
flags: `0x82000`  
min value: `0`  
</details>
<details>
<summary><code>weapon_auto_cleanup_time</code></summary>

If set to non-zero, weapons will delete themselves after the specified time (in seconds) if no players are near.

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>weapon_debug_inaccuracy_only_up</code></summary>

Force weapon inaccuracy to be in exactly the up direction

default: `"0"`  
flags: `0x6002`  
</details>
<details>
<summary><code>weapon_debug_max_inaccuracy</code></summary>

Force all shots to have maximum inaccuracy

default: `"0"`  
flags: `0x6002`  
</details>
<details>
<summary><code>weapon_land_dip_amt</code></summary>

The amount the gun should dip when the player lands after a jump.

default: `"20.0"`  
flags: `0x6002`  
</details>
<details>
<summary><code>weapon_max_before_cleanup</code></summary>

If set to non-zero, will remove the oldest dropped weapon to maintain the specified number of dropped weapons in the world.

default: `"0"`  
flags: `0x80000`  
</details>
<details>
<summary><code>weapon_molotov_maxdetonateslope</code></summary>

Maximum angle of slope on which the molotov will detonate

default: `"30.0"`  
flags: `0x2000`  
min value: `0`  
max value: `90`  
</details>
<details>
<summary><code>weapon_near_empty_sound</code></summary>

default: `"1"`  
flags: `0x6000`  
</details>
<details>
<summary><code>weapon_recoil_cooldown</code></summary>

DEPRECATED. Recoil now decays using weapon_recoil_decay_coefficient

default: `"0.55"`  
flags: `0x86000`  
</details>
<details>
<summary><code>weapon_recoil_decay1_exp</code></summary>

Decay factor exponent for weapon recoil

default: `"3.5"`  
flags: `0x86000`  
</details>
<details>
<summary><code>weapon_recoil_decay2_exp</code></summary>

Decay factor exponent for weapon recoil

default: `"8"`  
flags: `0x86000`  
</details>
<details>
<summary><code>weapon_recoil_decay2_lin</code></summary>

Decay factor (linear term) for weapon recoil

default: `"18"`  
flags: `0x86000`  
</details>
<details>
<summary><code>weapon_recoil_decay_coefficient</code></summary>



default: `"2.0"`  
flags: `0x86000`  
</details>
<details>
<summary><code>weapon_recoil_scale</code></summary>

Overall scale factor for recoil. Used to reduce recoil on specific platforms

default: `"2.0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>weapon_recoil_scale_motion_controller</code></summary>

Overall scale factor for recoil. Used to reduce recoil.  Only for motion controllers

default: `"1.0"`  
flags: `0x86000`  
</details>
<details>
<summary><code>weapon_recoil_suppression_factor</code></summary>

Initial recoil suppression factor (first suppressed shot will use this factor * standard recoil, lerping to 1 for later shots

default: `"0.75"`  
flags: `0x86000`  
</details>
<details>
<summary><code>weapon_recoil_suppression_shots</code></summary>

Number of shots before weapon uses full recoil

default: `"4"`  
flags: `0x86000`  
</details>
<details>
<summary><code>weapon_recoil_variance</code></summary>

Amount of variance per recoil impulse

default: `"0.55"`  
flags: `0x86000`  
min value: `0`  
max value: `1`  
</details>
<details>
<summary><code>weapon_recoil_vel_decay</code></summary>

Decay factor for weapon recoil velocity

default: `"4.5"`  
flags: `0x86000`  
</details>
<details>
<summary><code>weapon_recoil_view_punch_extra</code></summary>

Additional (non-aim) punch added to view from recoil

default: `"0.055"`  
flags: `0x86000`  
</details>
<details>
<summary><code>weapon_reticle_knife_show</code></summary>

When enabled will show knife reticle on clients. Used for game modes requiring target id display when holding a knife.

default: `"0"`  
flags: `0x82000`  
</details>
<details>
<summary><code>weapon_sound_falloff_multiplier</code></summary>

Scaling for falloff of weapon firing sounds

default: `"1.0"`  
flags: `0x86000`  
</details>
<details>
<summary><code>xbox_autothrottle</code></summary>

default: `"1"`  
flags: `0x80`  
</details>
<details>
<summary><code>xbox_throttlebias</code></summary>

default: `"100"`  
flags: `0x80`  
</details>
<details>
<summary><code>xbox_throttlespoof</code></summary>

default: `"200"`  
flags: `0x80`  
</details>

#### Addresses

```
server.dll!0x009bfcc8 ConVar BlendBonesMode
server.dll!0x009b4820 ConVar CS_WarnFriendlyDamageInterval
server.dll!0x00998218 ConVar achievement_debug
server.dll!0x00998270 ConVar achievement_disable
server.dll!0x009993b0 ConVar ai_LOS_mode
server.dll!0x0099c140 ConVar ai_debug_shoot_positions
server.dll!0x00999740 ConVar ai_drawbattlelines
server.dll!0x00999290 ConVar ai_report_task_timings_on_limit
server.dll!0x0099c1f0 ConVar ai_shot_bias_max
server.dll!0x0099c198 ConVar ai_shot_bias_min
server.dll!0x009992e8 ConVar ai_think_limit_label
server.dll!0x0099a778 ConVar ai_vehicle_avoidance
server.dll!0x009aced0 ConVar ammo_338mag_headshot_mult
server.dll!0x009acb60 ConVar ammo_338mag_impulse
server.dll!0x009ac480 ConVar ammo_338mag_max
server.dll!0x009ad030 ConVar ammo_357sig_headshot_mult
server.dll!0x009accc0 ConVar ammo_357sig_impulse
server.dll!0x009ac5e0 ConVar ammo_357sig_max
server.dll!0x009ac6e8 ConVar ammo_357sig_min_max
server.dll!0x009ac638 ConVar ammo_357sig_p250_max
server.dll!0x009ac690 ConVar ammo_357sig_small_max
server.dll!0x009acfd8 ConVar ammo_45acp_headshot_mult
server.dll!0x009acc68 ConVar ammo_45acp_impulse
server.dll!0x009ac588 ConVar ammo_45acp_max
server.dll!0x009acd70 ConVar ammo_50AE_headshot_mult
server.dll!0x009aca00 ConVar ammo_50AE_impulse
server.dll!0x009ac2c8 ConVar ammo_50AE_max
server.dll!0x009ace78 ConVar ammo_556mm_box_headshot_mult
server.dll!0x009acb08 ConVar ammo_556mm_box_impulse
server.dll!0x009ac428 ConVar ammo_556mm_box_max
server.dll!0x009ace20 ConVar ammo_556mm_headshot_mult
server.dll!0x009acab0 ConVar ammo_556mm_impulse
server.dll!0x009ac378 ConVar ammo_556mm_max
server.dll!0x009ac3d0 ConVar ammo_556mm_small_max
server.dll!0x009ad088 ConVar ammo_57mm_headshot_mult
server.dll!0x009acd18 ConVar ammo_57mm_impulse
server.dll!0x009ac740 ConVar ammo_57mm_max
server.dll!0x009acdc8 ConVar ammo_762mm_headshot_mult
server.dll!0x009aca58 ConVar ammo_762mm_impulse
server.dll!0x009ac320 ConVar ammo_762mm_max
server.dll!0x009acf28 ConVar ammo_9mm_headshot_mult
server.dll!0x009acbb8 ConVar ammo_9mm_impulse
server.dll!0x009ac4d8 ConVar ammo_9mm_max
server.dll!0x009acf80 ConVar ammo_buckshot_headshot_mult
server.dll!0x009acc10 ConVar ammo_buckshot_impulse
server.dll!0x009ac530 ConVar ammo_buckshot_max
server.dll!0x009ac950 ConVar ammo_grenade_limit_breachcharge
server.dll!0x009ac9a8 ConVar ammo_grenade_limit_bumpmine
server.dll!0x009ac798 ConVar ammo_grenade_limit_default
server.dll!0x009ac7f0 ConVar ammo_grenade_limit_flashbang
server.dll!0x009ac8a0 ConVar ammo_grenade_limit_snowballs
server.dll!0x009ac848 ConVar ammo_grenade_limit_total
server.dll!0x009ac8f8 ConVar ammo_item_limit_healthshot
server.dll!0x009bfd20 ConVar anim_3wayblend
server.dll!0x009bfd78 ConVar anim_twistbones_enabled
server.dll!0x009b9378 ConVar bot_allow_grenades
server.dll!0x009b9320 ConVar bot_allow_machine_guns
server.dll!0x009b91c0 ConVar bot_allow_pistols
server.dll!0x009b92c8 ConVar bot_allow_rifles
server.dll!0x009b9690 ConVar bot_allow_rogues
server.dll!0x009b9218 ConVar bot_allow_shotguns
server.dll!0x009b93d0 ConVar bot_allow_snipers
server.dll!0x009b9270 ConVar bot_allow_sub_machine_guns
server.dll!0x009b9588 ConVar bot_auto_follow
server.dll!0x009b97f0 ConVar bot_auto_vacate
server.dll!0x009af2c8 ConVar bot_autodifficulty_threshold_high
server.dll!0x009af270 ConVar bot_autodifficulty_threshold_low
server.dll!0x009b9638 ConVar bot_chatter
server.dll!0x009b9848 ConVar bot_controllable
server.dll!0x009ba410 ConVar bot_coop_force_throw_grenade_chance
server.dll!0x009b9f18 ConVar bot_coop_idle_max_vision_distance
server.dll!0x009b4be8 ConVar bot_crouch
server.dll!0x009b9740 ConVar bot_debug
server.dll!0x009b9798 ConVar bot_debug_target
server.dll!0x009b96e8 ConVar bot_defer_to_human_goals
server.dll!0x009b98f8 ConVar bot_defer_to_human_items
server.dll!0x009b98a0 ConVar bot_difficulty
server.dll!0x009b94d8 ConVar bot_dont_shoot
server.dll!0x009b9530 ConVar bot_eco_limit
server.dll!0x009b95e0 ConVar bot_flipout
server.dll!0x009b4b90 ConVar bot_freeze
server.dll!0x009b9ec0 ConVar bot_ignore_players
server.dll!0x009b9a00 ConVar bot_join_after_player
server.dll!0x009b9a58 ConVar bot_join_team
server.dll!0x009ba010 ConVar bot_loadout
server.dll!0x009b9e68 ConVar bot_max_vision_distance_override
server.dll!0x009b4b38 ConVar bot_mimic
server.dll!0x009b4c40 ConVar bot_mimic_yaw_offset
server.dll!0x009b9480 ConVar bot_profile_db
server.dll!0x009b9950 ConVar bot_quota
server.dll!0x009b99a8 ConVar bot_quota_mode
server.dll!0x009ba068 ConVar bot_randombuy
server.dll!0x009b9b28 ConVar bot_show_battlefront
server.dll!0x009b90b8 ConVar bot_show_nav
server.dll!0x009b9ad0 ConVar bot_show_occupy_time
server.dll!0x009b9060 ConVar bot_stop
server.dll!0x009b9008 ConVar bot_traceview
server.dll!0x009b9110 ConVar bot_walk
server.dll!0x009b9428 ConVar bot_zombie
server.dll!0x009b2c60 ConVar cash_player_bomb_defused
server.dll!0x009b2c08 ConVar cash_player_bomb_planted
server.dll!0x009b2d68 ConVar cash_player_damage_hostage
server.dll!0x009b2e70 ConVar cash_player_get_killed
server.dll!0x009b2d10 ConVar cash_player_interact_with_hostage
server.dll!0x009b2bb0 ConVar cash_player_killed_enemy_default
server.dll!0x009b2b58 ConVar cash_player_killed_enemy_factor
server.dll!0x009b2dc0 ConVar cash_player_killed_hostage
server.dll!0x009b2b00 ConVar cash_player_killed_teammate
server.dll!0x009b2cb8 ConVar cash_player_rescued_hostage
server.dll!0x009b2e18 ConVar cash_player_respawn_amount
server.dll!0x009b2688 ConVar cash_team_elimination_bomb_map
server.dll!0x009b2630 ConVar cash_team_elimination_hostage_map_ct
server.dll!0x009b25d8 ConVar cash_team_elimination_hostage_map_t
server.dll!0x009b29f8 ConVar cash_team_hostage_alive
server.dll!0x009b2aa8 ConVar cash_team_hostage_interaction
server.dll!0x009b2898 ConVar cash_team_loser_bonus
server.dll!0x009b28f0 ConVar cash_team_loser_bonus_consecutive_rounds
server.dll!0x009b2a50 ConVar cash_team_planted_bomb_but_defused
server.dll!0x009b29a0 ConVar cash_team_rescued_hostage
server.dll!0x009b26e0 ConVar cash_team_survive_guardian_wave
server.dll!0x009b2580 ConVar cash_team_terrorist_win_bomb
server.dll!0x009b27e8 ConVar cash_team_win_by_defusing_bomb
server.dll!0x009b2840 ConVar cash_team_win_by_hostage_rescue
server.dll!0x009b2790 ConVar cash_team_win_by_time_running_out_bomb
server.dll!0x009b2738 ConVar cash_team_win_by_time_running_out_hostage
server.dll!0x009b2948 ConVar cash_team_winner_bonus_consecutive_rounds
server.dll!0x009a8390 ConVar cc_showmissing
server.dll!0x0099a158 ConVar chet_debug_idle
server.dll!0x009bfdd0 ConVar choreo_spew_filter
server.dll!0x009a9348 ConVar cl_remove_old_ugc_downloads
server.dll!0x009bfc70 ConVar cl_simdbones
server.dll!0x009bfc18 ConVar cl_use_simd_bones
server.dll!0x009b1e80 ConVar contributionscore_assist
server.dll!0x009b2090 ConVar contributionscore_bomb_defuse_major
server.dll!0x009b2038 ConVar contributionscore_bomb_defuse_minor
server.dll!0x009b2140 ConVar contributionscore_bomb_exploded
server.dll!0x009b20e8 ConVar contributionscore_bomb_planted
server.dll!0x009b22a0 ConVar contributionscore_cash_bundle
server.dll!0x009b22f8 ConVar contributionscore_crate_break
server.dll!0x009b2248 ConVar contributionscore_hostage_kill
server.dll!0x009b1fe0 ConVar contributionscore_hostage_rescue_major
server.dll!0x009b1f88 ConVar contributionscore_hostage_rescue_minor
server.dll!0x009b1ed8 ConVar contributionscore_kill
server.dll!0x009b2350 ConVar contributionscore_kill_factor
server.dll!0x009b1f30 ConVar contributionscore_objective_kill
server.dll!0x009b2198 ConVar contributionscore_suicide
server.dll!0x009b21f0 ConVar contributionscore_team_kill
server.dll!0x009b4350 ConVar cs_ShowStateTransitions
server.dll!0x009b4df8 ConVar cs_enable_player_physics_box
server.dll!0x009b4da0 ConVar cs_hostage_near_rescue_music_distance
server.dll!0x009b8a08 ConVar custom_bot_difficulty
server.dll!0x009b8980 ConVar debug_aim_angle
server.dll!0x0099d148 ConVar debug_visibility_monitor
server.dll!0x009b0400 ConVar dev_reportmoneychanges
server.dll!0x009a96d0 ConVar developer
server.dll!0x0099bbd0 ConVar ent_messages_draw
server.dll!0x009b5600 ConVar ff_damage_bullet_penetration
server.dll!0x009b5a90 ConVar ff_damage_reduction_bullets
server.dll!0x009b59e0 ConVar ff_damage_reduction_grenade
server.dll!0x009b5988 ConVar ff_damage_reduction_grenade_self
server.dll!0x009b5a38 ConVar ff_damage_reduction_other
server.dll!0x009a1d30 ConVar fish_dormant
server.dll!0x009a20c8 ConVar func_break_max_pieces
server.dll!0x009a1950 ConVar fx_new_sparks
server.dll!0x009a35f8 ConVar g_Language
server.dll!0x009a7040 ConVar g_debug_angularsensor
server.dll!0x009a5398 ConVar g_debug_constraint_sounds
server.dll!0x009a7788 ConVar g_debug_ragdoll_removal
server.dll!0x0099b0b8 ConVar g_debug_trackpather
server.dll!0x009a9728 ConVar g_debug_vehiclebase
server.dll!0x009a4f60 ConVar g_debug_vehicledriver
server.dll!0x009a9868 ConVar g_debug_vehicleexit
server.dll!0x009a9810 ConVar g_debug_vehiclesound
server.dll!0x009b7230 ConVar g_jeepexitspeed
server.dll!0x009a77e0 ConVar g_ragdoll_important_maxcount
server.dll!0x009a7730 ConVar g_ragdoll_maxcount
server.dll!0x009b8b10 ConVar game_mode
server.dll!0x009b8ab8 ConVar game_online
server.dll!0x009b8a60 ConVar game_public
server.dll!0x009b8b68 ConVar game_type
server.dll!0x009b4d48 ConVar gg_knife_kill_demotes
server.dll!0x009a3aa0 ConVar global_event_log_enabled
server.dll!0x009bbb30 ConVar healthshot_allow_use_at_full
server.dll!0x009bba80 ConVar healthshot_health
server.dll!0x009b4f00 ConVar healthshot_healthboost_damage_multiplier
server.dll!0x009bbb88 ConVar healthshot_healthboost_speed_multiplier
server.dll!0x009bbad8 ConVar healthshot_healthboost_time
server.dll!0x0099c248 ConVar hl2_episodic
server.dll!0x009baf00 ConVar hostage_debug
server.dll!0x009b5b98 ConVar hostage_feetyawrate
server.dll!0x009bace8 ConVar hostage_is_silent
server.dll!0x009ba5f8 ConVar inferno_child_spawn_interval_multiplier
server.dll!0x009baac8 ConVar inferno_child_spawn_max_depth
server.dll!0x009ba8b8 ConVar inferno_damage
server.dll!0x009ba860 ConVar inferno_debug
server.dll!0x009ba7b0 ConVar inferno_flame_lifetime
server.dll!0x009ba758 ConVar inferno_flame_spacing
server.dll!0x009babd0 ConVar inferno_forward_reduction_factor
server.dll!0x009ba808 ConVar inferno_friendly_fire_duration
server.dll!0x009ba5a0 ConVar inferno_initial_spawn_interval
server.dll!0x009ba650 ConVar inferno_max_child_spawn_interval
server.dll!0x009ba700 ConVar inferno_max_flames
server.dll!0x009ba910 ConVar inferno_max_range
server.dll!0x009ba548 ConVar inferno_per_flame_spawn_duration
server.dll!0x009bab20 ConVar inferno_scorch_decals
server.dll!0x009ba6a8 ConVar inferno_spawn_angle
server.dll!0x009baa70 ConVar inferno_surface_offset
server.dll!0x009ba9c0 ConVar inferno_velocity_decay_factor
server.dll!0x009ba968 ConVar inferno_velocity_factor
server.dll!0x009baa18 ConVar inferno_velocity_normal_factor
server.dll!0x009a2408 ConVar loopsingleplayermaps
server.dll!0x009a4a40 ConVar mapcycledisabled
server.dll!0x009b8c98 ConVar molotov_throw_detonate_time
server.dll!0x009ad558 ConVar mp_afterroundmoney
server.dll!0x009a2880 ConVar mp_allowNPCs
server.dll!0x009a37f8 ConVar mp_allowspectators
server.dll!0x009ad870 ConVar mp_anyone_can_pickup_c4
server.dll!0x009a2720 ConVar mp_autocrosshair
server.dll!0x009b06c0 ConVar mp_autokick
server.dll!0x009a8ea0 ConVar mp_autoteambalance
server.dll!0x009ac248 ConVar mp_backup_restore_load_autopause
server.dll!0x009ac0a0 ConVar mp_backup_round_auto
server.dll!0x009ac0f8 ConVar mp_backup_round_file
server.dll!0x009ac1a8 ConVar mp_backup_round_file_last
server.dll!0x009ac150 ConVar mp_backup_round_file_pattern
server.dll!0x009a89d0 ConVar mp_blockstyle
server.dll!0x009a8c90 ConVar mp_bonusroundtime
server.dll!0x009ad348 ConVar mp_buy_allow_grenades
server.dll!0x009ad3a0 ConVar mp_buy_allow_guns
server.dll!0x009a3900 ConVar mp_buy_anywhere
server.dll!0x009a3958 ConVar mp_buy_during_immunity
server.dll!0x009ad2f0 ConVar mp_buytime
server.dll!0x009ad8c8 ConVar mp_c4_cannot_be_defused
server.dll!0x009b03a0 ConVar mp_c4timer
server.dll!0x009a8a80 ConVar mp_capdeteriorate_time
server.dll!0x009a8978 ConVar mp_capstyle
server.dll!0x009afa58 ConVar mp_competitive_endofmatch_extra_time
server.dll!0x009b12c8 ConVar mp_consecutive_loss_aversion
server.dll!0x009b1320 ConVar mp_consecutive_loss_max
server.dll!0x009af168 ConVar mp_coop_force_join_ct
server.dll!0x009b8fb0 ConVar mp_coopmission_bot_difficulty_offset
server.dll!0x009af1c0 ConVar mp_coopmission_mission_number
server.dll!0x009ae458 ConVar mp_ct_default_grenades
server.dll!0x009ae350 ConVar mp_ct_default_melee
server.dll!0x009ae400 ConVar mp_ct_default_primary
server.dll!0x009ae3a8 ConVar mp_ct_default_secondary
server.dll!0x009b1110 ConVar mp_damage_headshot_only
server.dll!0x009b0f58 ConVar mp_damage_scale_ct_body
server.dll!0x009b0fb0 ConVar mp_damage_scale_ct_head
server.dll!0x009b1008 ConVar mp_damage_scale_t_body
server.dll!0x009b1060 ConVar mp_damage_scale_t_head
server.dll!0x009b11c0 ConVar mp_damage_vampiric_amount
server.dll!0x009af0b8 ConVar mp_death_drop_breachcharge
server.dll!0x009aef58 ConVar mp_death_drop_c4
server.dll!0x009af008 ConVar mp_death_drop_defuser
server.dll!0x009aefb0 ConVar mp_death_drop_grenade
server.dll!0x009aef00 ConVar mp_death_drop_gun
server.dll!0x009af110 ConVar mp_death_drop_healthshot
server.dll!0x009af060 ConVar mp_death_drop_taser
server.dll!0x009b4e50 ConVar mp_deathcam_skippable
server.dll!0x009af690 ConVar mp_default_team_winner_no_objective
server.dll!0x009aedf8 ConVar mp_defuser_allocation
server.dll!0x009a8c38 ConVar mp_disable_respawn_times
server.dll!0x009adb30 ConVar mp_disconnect_kills_bots
server.dll!0x009adad8 ConVar mp_disconnect_kills_players
server.dll!0x009aecf0 ConVar mp_display_kill_assists
server.dll!0x009b0df8 ConVar mp_dm_bonus_length_max
server.dll!0x009b0da0 ConVar mp_dm_bonus_length_min
server.dll!0x009aea30 ConVar mp_dm_bonus_percent
server.dll!0x009aea88 ConVar mp_dm_bonus_respawn
server.dll!0x009aec98 ConVar mp_dm_bonusweapon_dogtags
server.dll!0x009aeae0 ConVar mp_dm_dogtag_score
server.dll!0x009ae9d8 ConVar mp_dm_kill_base_score
server.dll!0x009aeb38 ConVar mp_dm_teammode
server.dll!0x009aebe8 ConVar mp_dm_teammode_bonus_score
server.dll!0x009aec40 ConVar mp_dm_teammode_dogtag_score
server.dll!0x009aeb90 ConVar mp_dm_teammode_kill_score
server.dll!0x009b0d48 ConVar mp_dm_time_between_bonus_max
server.dll!0x009b0cf0 ConVar mp_dm_time_between_bonus_min
server.dll!0x009ad450 ConVar mp_do_warmup_offine
server.dll!0x009ad3f8 ConVar mp_do_warmup_period
server.dll!0x009b0ea8 ConVar mp_dogtag_despawn_on_killer_death
server.dll!0x009b0f00 ConVar mp_dogtag_despawn_time
server.dll!0x009b0e50 ConVar mp_dogtag_pickup_rule
server.dll!0x009b4770 ConVar mp_drop_grenade_enable
server.dll!0x009b4718 ConVar mp_drop_knife_enable
server.dll!0x009af378 ConVar mp_economy_reset_rounds
server.dll!0x009a8d40 ConVar mp_enableroundwaittime
server.dll!0x009afb60 ConVar mp_endmatch_votenextleveltime
server.dll!0x009afab0 ConVar mp_endmatch_votenextmap
server.dll!0x009afb08 ConVar mp_endmatch_votenextmap_keepcurrent
server.dll!0x009b2478 ConVar mp_endmatch_votenextmap_wargames_modes
server.dll!0x009b24d0 ConVar mp_endmatch_votenextmap_wargames_nummaps
server.dll!0x009b2528 ConVar mp_endmatch_votenextmap_wargames_nummodes
server.dll!0x009adcc8 ConVar mp_endwarmup_player_count
server.dll!0x009af320 ConVar mp_equipment_reset_rounds
server.dll!0x0099b2f8 ConVar mp_facefronttime
server.dll!0x009a2568 ConVar mp_falldamage
server.dll!0x0099b2a0 ConVar mp_feetyawrate
server.dll!0x009a26c8 ConVar mp_flashlight
server.dll!0x009b5130 ConVar mp_flinch_punch_scale
server.dll!0x009a2670 ConVar mp_footsteps
server.dll!0x009ad138 ConVar mp_force_assign_teams
server.dll!0x009af218 ConVar mp_force_pick_time
server.dll!0x009a2618 ConVar mp_forcerespawn
server.dll!0x009a4af0 ConVar mp_fraglimit
server.dll!0x009b3740 ConVar mp_free_armor
server.dll!0x009b05b8 ConVar mp_freezetime
server.dll!0x009a3850 ConVar mp_friendlyfire
server.dll!0x009ae2a0 ConVar mp_ggprogressive_random_weapon_kills_needed
server.dll!0x009ae1f0 ConVar mp_ggprogressive_round_restart_delay
server.dll!0x009ae248 ConVar mp_ggprogressive_use_random_weapons
server.dll!0x009ae0e8 ConVar mp_ggtr_always_upgrade
server.dll!0x009ae928 ConVar mp_ggtr_bomb_defuse_bonus
server.dll!0x009ae980 ConVar mp_ggtr_bomb_detonation_bonus
server.dll!0x009ae770 ConVar mp_ggtr_bomb_pts_for_flash
server.dll!0x009ae718 ConVar mp_ggtr_bomb_pts_for_he
server.dll!0x009ae7c8 ConVar mp_ggtr_bomb_pts_for_molotov
server.dll!0x009ae6c0 ConVar mp_ggtr_bomb_pts_for_upgrade
server.dll!0x009ae8d0 ConVar mp_ggtr_bomb_respawn_delay
server.dll!0x009ae140 ConVar mp_ggtr_end_round_kill_bonus
server.dll!0x009ae878 ConVar mp_ggtr_halftime_delay
server.dll!0x009ae198 ConVar mp_ggtr_last_weapon_kill_ends_half
server.dll!0x009ae2f8 ConVar mp_ggtr_num_rounds_autoprogress
server.dll!0x009aee50 ConVar mp_give_player_c4
server.dll!0x009b1218 ConVar mp_global_damage_per_second
server.dll!0x009b0a30 ConVar mp_guardian_bot_money_per_wave
server.dll!0x009b0b90 ConVar mp_guardian_loc_adjective
server.dll!0x009b0be8 ConVar mp_guardian_loc_condition
server.dll!0x009b0c40 ConVar mp_guardian_loc_icon
server.dll!0x009b0ae0 ConVar mp_guardian_loc_mission
server.dll!0x009b0a88 ConVar mp_guardian_loc_override
server.dll!0x009b0b38 ConVar mp_guardian_loc_weapon
server.dll!0x009b09d8 ConVar mp_guardian_player_dist_max
server.dll!0x009b0980 ConVar mp_guardian_player_dist_min
server.dll!0x009b08d0 ConVar mp_guardian_special_kills_needed
server.dll!0x009b0928 ConVar mp_guardian_special_weapon_needed
server.dll!0x009bb138 ConVar mp_guardian_target_site
server.dll!0x009b37f0 ConVar mp_halftime
server.dll!0x009ae038 ConVar mp_halftime_duration
server.dll!0x009addd0 ConVar mp_halftime_pausematch
server.dll!0x009add78 ConVar mp_halftime_pausetimer
server.dll!0x009abf98 ConVar mp_heavyassaultsuit_aimpunch
server.dll!0x009af5e0 ConVar mp_heavyassaultsuit_cooldown
server.dll!0x009abf40 ConVar mp_heavyassaultsuit_deploy_timescale
server.dll!0x009abe90 ConVar mp_heavyassaultsuit_speed
server.dll!0x009abee8 ConVar mp_heavybot_damage_reduction_scale
server.dll!0x009bac38 ConVar mp_hostagepenalty
server.dll!0x009baf58 ConVar mp_hostages_max
server.dll!0x009ad818 ConVar mp_hostages_rescuetime
server.dll!0x009ad7c0 ConVar mp_hostages_rescuetowin
server.dll!0x009badf0 ConVar mp_hostages_run_speed_modifier
server.dll!0x009bb060 ConVar mp_hostages_spawn_farthest
server.dll!0x009bb008 ConVar mp_hostages_spawn_force_positions
server.dll!0x009bafb0 ConVar mp_hostages_spawn_same_every_round
server.dll!0x009ad768 ConVar mp_hostages_takedamage
server.dll!0x009b0878 ConVar mp_humanteam
server.dll!0x009b0c98 ConVar mp_ignore_round_win_conditions
server.dll!0x0099b350 ConVar mp_ik
server.dll!0x009ae610 ConVar mp_join_grace_time
server.dll!0x009b0610 ConVar mp_limitteams
server.dll!0x009aa360 ConVar mp_logdetail
server.dll!0x009aa3b8 ConVar mp_logdetail_items
server.dll!0x009b4458 ConVar mp_logdistance_2d
server.dll!0x009b44b0 ConVar mp_logdistance_sec
server.dll!0x009b4508 ConVar mp_logloadouts
server.dll!0x009b4400 ConVar mp_logmoney
server.dll!0x009ae090 ConVar mp_match_can_clinch
server.dll!0x009aeda0 ConVar mp_match_end_changelevel
server.dll!0x009aed48 ConVar mp_match_end_restart
server.dll!0x009a49e8 ConVar mp_match_restart_delay
server.dll!0x009b3798 ConVar mp_max_armor
server.dll!0x009ad500 ConVar mp_maxmoney
server.dll!0x009a8b88 ConVar mp_maxrounds
server.dll!0x009ae820 ConVar mp_molotovusedelay
server.dll!0x009bae48 ConVar mp_only_cts_rescue_hostages
server.dll!0x009ad660 ConVar mp_overtime_enable
server.dll!0x009ade28 ConVar mp_overtime_halftime_pausetimer
server.dll!0x009ad6b8 ConVar mp_overtime_maxrounds
server.dll!0x009ad710 ConVar mp_overtime_startmoney
server.dll!0x009bc000 ConVar mp_plant_c4_anywhere
server.dll!0x009b10b8 ConVar mp_player_healthbuffer_decay_rate
server.dll!0x009ad5b0 ConVar mp_playercashawards
server.dll!0x009aded8 ConVar mp_playerid
server.dll!0x009adf30 ConVar mp_playerid_delay
server.dll!0x009adf88 ConVar mp_playerid_hold
server.dll!0x009a37a0 ConVar mp_radar_showall
server.dll!0x009b3848 ConVar mp_randomspawn
server.dll!0x009b38f8 ConVar mp_randomspawn_dist
server.dll!0x009b38a0 ConVar mp_randomspawn_los
server.dll!0x009aeea8 ConVar mp_require_gun_use_to_acquire
server.dll!0x009ade80 ConVar mp_respawn_immunitytime
server.dll!0x009af798 ConVar mp_respawn_on_death_ct
server.dll!0x009af740 ConVar mp_respawn_on_death_t
server.dll!0x009a8a28 ConVar mp_respawnwavetime
server.dll!0x009a4ba0 ConVar mp_restartgame
server.dll!0x009adfe0 ConVar mp_round_restart_delay
server.dll!0x009b0458 ConVar mp_roundtime
server.dll!0x009b0560 ConVar mp_roundtime_defuse
server.dll!0x009b04b0 ConVar mp_roundtime_deployment
server.dll!0x009b0508 ConVar mp_roundtime_hostage
server.dll!0x009abff0 ConVar mp_shield_speed_deployed
server.dll!0x009ac048 ConVar mp_shield_speed_holstered
server.dll!0x009a8d98 ConVar mp_showcleanedupents
server.dll!0x009a8ce8 ConVar mp_showroundtransitions
server.dll!0x009b3a30 ConVar mp_solid_teammates
server.dll!0x009b0718 ConVar mp_spawnprotectiontime
server.dll!0x009ad0e0 ConVar mp_spec_swapplayersides
server.dll!0x009ad298 ConVar mp_spectators_max
server.dll!0x009a8f50 ConVar mp_stalemate_at_timelimit
server.dll!0x009a8ef8 ConVar mp_stalemate_enable
server.dll!0x009a8e48 ConVar mp_stalemate_timelimit
server.dll!0x009b1270 ConVar mp_starting_losses
server.dll!0x009ad4a8 ConVar mp_startmoney
server.dll!0x009a6508 ConVar mp_suicide_time
server.dll!0x009ae5b8 ConVar mp_t_default_grenades
server.dll!0x009ae4b0 ConVar mp_t_default_melee
server.dll!0x009ae560 ConVar mp_t_default_primary
server.dll!0x009ae508 ConVar mp_t_default_secondary
server.dll!0x009b50b0 ConVar mp_tagging_scale
server.dll!0x009bd478 ConVar mp_taser_recharge_time
server.dll!0x009b0820 ConVar mp_td_dmgtokick
server.dll!0x009b07c8 ConVar mp_td_dmgtowarn
server.dll!0x009b0770 ConVar mp_td_spawndmgthreshold
server.dll!0x009ab6c8 ConVar mp_team_timeout_max
server.dll!0x009ab670 ConVar mp_team_timeout_time
server.dll!0x009ad608 ConVar mp_teamcashawards
server.dll!0x009ab9c0 ConVar mp_teamflag_1
server.dll!0x009aba18 ConVar mp_teamflag_2
server.dll!0x009a2778 ConVar mp_teamlist
server.dll!0x009aba70 ConVar mp_teamlogo_1
server.dll!0x009abac8 ConVar mp_teamlogo_2
server.dll!0x009abc28 ConVar mp_teammatchstat_1
server.dll!0x009abc80 ConVar mp_teammatchstat_2
server.dll!0x009abe38 ConVar mp_teammatchstat_cycletime
server.dll!0x009abde0 ConVar mp_teammatchstat_holdtime
server.dll!0x009abbd0 ConVar mp_teammatchstat_txt
server.dll!0x009a38a8 ConVar mp_teammates_are_enemies
server.dll!0x009ab910 ConVar mp_teamname_1
server.dll!0x009ab968 ConVar mp_teamname_2
server.dll!0x009a2510 ConVar mp_teamplay
server.dll!0x009abb78 ConVar mp_teamprediction_pct
server.dll!0x009abb20 ConVar mp_teamprediction_txt
server.dll!0x009a8b30 ConVar mp_teams_unbalance_limit
server.dll!0x009abcd8 ConVar mp_teamscore_1
server.dll!0x009abd30 ConVar mp_teamscore_2
server.dll!0x009b0668 ConVar mp_tkpunish
server.dll!0x009a8ad8 ConVar mp_tournament
server.dll!0x009af7f0 ConVar mp_use_respawn_waves
server.dll!0x009b3ae0 ConVar mp_verbose_changelevel_spew
server.dll!0x009add20 ConVar mp_warmup_pausetimer
server.dll!0x009adc70 ConVar mp_warmuptime_all_players_connected
server.dll!0x009bcf30 ConVar mp_weapon_melee_touch_time_after_hit
server.dll!0x009bc5e0 ConVar mp_weapon_next_owner_touch_time
server.dll!0x009bc588 ConVar mp_weapon_prev_owner_touch_time
server.dll!0x009b1168 ConVar mp_weapon_self_inflict_amount
server.dll!0x009af4d8 ConVar mp_weapons_allow_heavy
server.dll!0x009b3be8 ConVar mp_weapons_allow_map_placed
server.dll!0x009af428 ConVar mp_weapons_allow_pistols
server.dll!0x009af530 ConVar mp_weapons_allow_rifles
server.dll!0x009af480 ConVar mp_weapons_allow_smgs
server.dll!0x009af638 ConVar mp_weapons_allow_typecount
server.dll!0x009af3d0 ConVar mp_weapons_allow_zeus
server.dll!0x009af6e8 ConVar mp_weapons_glow_on_ground
server.dll!0x009ab720 ConVar mp_weapons_max_gun_purchases_per_weapon_per_match
server.dll!0x009a25c0 ConVar mp_weaponstay
server.dll!0x009ae668 ConVar mp_win_panel_display_time
server.dll!0x009a8be0 ConVar mp_winlimit
server.dll!0x009bd950 ConVar nav_area_bgcolor
server.dll!0x009be4c0 ConVar nav_area_max_size
server.dll!0x009bd848 ConVar nav_coplanar_slope_limit
server.dll!0x009bd8a0 ConVar nav_coplanar_slope_limit_displacement
server.dll!0x009bd9a8 ConVar nav_corner_adjust_adjacent
server.dll!0x009bdfd8 ConVar nav_create_area_at_feet
server.dll!0x009bdf28 ConVar nav_create_place_on_ground
server.dll!0x009bda58 ConVar nav_debug_blocked
server.dll!0x009be308 ConVar nav_displacement_test
server.dll!0x009be030 ConVar nav_drag_selection_volume_zmax_offset
server.dll!0x009be088 ConVar nav_drag_selection_volume_zmin_offset
server.dll!0x009bdf80 ConVar nav_draw_limit
server.dll!0x009bf3d8 ConVar nav_edit
server.dll!0x009be360 ConVar nav_generate_fencetops
server.dll!0x009be3b8 ConVar nav_generate_fixup_jump_areas
server.dll!0x009be410 ConVar nav_generate_incremental_range
server.dll!0x009be468 ConVar nav_generate_incremental_tolerance
server.dll!0x009bdb08 ConVar nav_max_view_distance
server.dll!0x009be710 ConVar nav_max_vis_delta_list_length
server.dll!0x009bdbb8 ConVar nav_potentially_visible_dot_tolerance
server.dll!0x009bf380 ConVar nav_quicksave
server.dll!0x009be608 ConVar nav_show_approach_points
server.dll!0x009bde78 ConVar nav_show_area_info
server.dll!0x009be0f8 ConVar nav_show_compass
server.dll!0x009bdab0 ConVar nav_show_continguous
server.dll!0x009be660 ConVar nav_show_danger
server.dll!0x009bda00 ConVar nav_show_light_intensity
server.dll!0x009bf598 ConVar nav_show_node_grid
server.dll!0x009bf438 ConVar nav_show_node_id
server.dll!0x009bf5f0 ConVar nav_show_nodes
server.dll!0x009be6b8 ConVar nav_show_player_counts
server.dll!0x009bdc10 ConVar nav_show_potentially_visible
server.dll!0x009be258 ConVar nav_slope_limit
server.dll!0x009be2b0 ConVar nav_slope_tolerance
server.dll!0x009bded0 ConVar nav_snap_to_grid
server.dll!0x009be1c0 ConVar nav_solid_props
server.dll!0x009bd8f8 ConVar nav_split_place_on_ground
server.dll!0x009bf490 ConVar nav_test_node
server.dll!0x009bf4e8 ConVar nav_test_node_crouch
server.dll!0x009bf540 ConVar nav_test_node_crouch_dir
server.dll!0x009bdb60 ConVar nav_update_visibility_on_edit
server.dll!0x009a4e30 ConVar nextlevel
server.dll!0x009a4dd8 ConVar nextmap_print_enabled
server.dll!0x009a4e88 ConVar nextmode
server.dll!0x0099cd08 ConVar noclip_fixup
server.dll!0x0099ac90 ConVar npc_ally_deathmessage
server.dll!0x0099b4d0 ConVar npc_height_adjust
server.dll!0x009a29a0 ConVar occlusion_test_camera_margins
server.dll!0x009a29f8 ConVar occlusion_test_jump_margin
server.dll!0x009a2948 ConVar occlusion_test_shadow_length
server.dll!0x009a3548 ConVar old_radiusdamage
server.dll!0x009c1510 ConVar panel_test_title_safe
server.dll!0x009a52e0 ConVar particle_test_attach_attachment
server.dll!0x009a5288 ConVar particle_test_attach_mode
server.dll!0x009a5230 ConVar particle_test_file
server.dll!0x009a54a8 ConVar phys_debug_check_contacts
server.dll!0x009b4610 ConVar phys_headshotscale
server.dll!0x009b45b8 ConVar phys_playerscale
server.dll!0x009a88b8 ConVar phys_pushscale
server.dll!0x009a5450 ConVar phys_show_active
server.dll!0x009a6718 ConVar player_debug_print_damage
server.dll!0x009b4560 ConVar player_ping_throttle_decay
server.dll!0x009b5b40 ConVar post_jump_crouch
server.dll!0x009a75f0 ConVar props_break_max_pieces
server.dll!0x009a7648 ConVar props_break_max_pieces_perframe
server.dll!0x009a2a50 ConVar pvs_min_player_distance
server.dll!0x009a4290 ConVar r_AirboatViewDampenDamp
server.dll!0x009a4238 ConVar r_AirboatViewDampenFreq
server.dll!0x009a42e8 ConVar r_AirboatViewZHeight
server.dll!0x009a97b8 ConVar r_JeepFOV
server.dll!0x009a4188 ConVar r_JeepViewDampenDamp
server.dll!0x009a4130 ConVar r_JeepViewDampenFreq
server.dll!0x009a41e0 ConVar r_JeepViewZHeight
server.dll!0x009a40d8 ConVar r_VehicleViewDampen
server.dll!0x009a1ee0 ConVar r_vehicleBrakeRate
server.dll!0x009a9678 ConVar r_visualizetraces
server.dll!0x009aa2b0 ConVar radarvisdistance
server.dll!0x009aa308 ConVar radarvismaxdot
server.dll!0x009aa200 ConVar radarvismethod
server.dll!0x009aa258 ConVar radarvispow
server.dll!0x0099af58 ConVar rr_followup_maxdist
server.dll!0x009986b0 ConVar rr_remarkable_max_distance
server.dll!0x00998600 ConVar rr_remarkable_world_entities_replay_limit
server.dll!0x00998658 ConVar rr_remarkables_enabled
server.dll!0x0099afb0 ConVar rr_thenany_score_slop
server.dll!0x009a7ed8 ConVar scene_clientflex
server.dll!0x009a7f30 ConVar scene_print
server.dll!0x00998550 ConVar scene_showfaceto
server.dll!0x0099c2a0 ConVar scene_showlook
server.dll!0x0099c2f8 ConVar scene_showmoveto
server.dll!0x0099c350 ConVar scene_showunlock
server.dll!0x009a2460 ConVar servercfgfile
server.dll!0x009a92f0 ConVar showtriggers
server.dll!0x009a3470 ConVar sk_autoaim_mode
server.dll!0x009a35a0 ConVar skill
server.dll!0x0099c890 ConVar smoothstairs
server.dll!0x009afbb8 ConVar snd_music_boost
server.dll!0x009a8288 ConVar snd_prevent_ss_duplicates
server.dll!0x009a82e0 ConVar snd_sos_show_server_xmit
server.dll!0x009a8510 ConVar soundpatch_captionlength
server.dll!0x009a85f0 ConVar soundscape_debug
server.dll!0x009a62a0 ConVar spec_allow_roaming
server.dll!0x009a6198 ConVar spec_freeze_deathanim_time
server.dll!0x009a6140 ConVar spec_freeze_panel_extended_time
server.dll!0x009a6248 ConVar spec_freeze_target_fov
server.dll!0x009a61f0 ConVar spec_freeze_target_fov_long
server.dll!0x009a6038 ConVar spec_freeze_time
server.dll!0x009a6090 ConVar spec_freeze_time_lock
server.dll!0x009a60e8 ConVar spec_freeze_traveltime
server.dll!0x009a4ee0 ConVar spec_replay_bot
server.dll!0x009a5f30 ConVar spec_replay_cam_delay
server.dll!0x009a5f88 ConVar spec_replay_cam_options
server.dll!0x009aaee0 ConVar spec_replay_round_delay
server.dll!0x009a5fe0 ConVar spec_replay_winddown_time
server.dll!0x009b8770 ConVar steam_controller_haptics
server.dll!0x009a8710 ConVar steamworks_sessionid_server
server.dll!0x009a28d8 ConVar suitvolume
server.dll!0x009a4398 ConVar sv_accelerate
server.dll!0x009a3ec8 ConVar sv_accelerate_debug_speed
server.dll!0x009a3e70 ConVar sv_accelerate_use_weapon_speed
server.dll!0x009a3110 ConVar sv_air_max_horizontal_parachute_ratio
server.dll!0x009a30b8 ConVar sv_air_max_horizontal_parachute_speed
server.dll!0x009a3060 ConVar sv_air_max_wishspeed
server.dll!0x009a3168 ConVar sv_air_pushaway_dist
server.dll!0x009a44f8 ConVar sv_airaccelerate
server.dll!0x009a44a0 ConVar sv_airaccelerate_parachute
server.dll!0x009a4448 ConVar sv_airaccelerate_rappel
server.dll!0x0099c550 ConVar sv_allchat
server.dll!0x009ad978 ConVar sv_allow_thirdperson
server.dll!0x009a9ba0 ConVar sv_allow_votes
server.dll!0x009b0138 ConVar sv_alltalk
server.dll!0x009b72f8 ConVar sv_arms_race_vote_to_restart_disallowed_after
server.dll!0x009ab510 ConVar sv_auto_adjust_bot_difficulty
server.dll!0x009b02f0 ConVar sv_auto_full_alltalk_during_warmup_half_end
server.dll!0x009aac40 ConVar sv_autobunnyhopping
server.dll!0x009b4cf0 ConVar sv_autobuyammo
server.dll!0x009a4028 ConVar sv_backspeed
server.dll!0x009a62f8 ConVar sv_bonus_challenge
server.dll!0x009ba278 ConVar sv_bot_buy_decoy_weight
server.dll!0x009ba220 ConVar sv_bot_buy_flash_weight
server.dll!0x009ba170 ConVar sv_bot_buy_grenade_chance
server.dll!0x009ba328 ConVar sv_bot_buy_hegrenade_weight
server.dll!0x009ba2d0 ConVar sv_bot_buy_molotov_weight
server.dll!0x009ba1c8 ConVar sv_bot_buy_smoke_weight
server.dll!0x009afed0 ConVar sv_bot_difficulty_gamepad
server.dll!0x009aff80 ConVar sv_bot_difficulty_hydra
server.dll!0x009afe78 ConVar sv_bot_difficulty_kbm
server.dll!0x009aff28 ConVar sv_bot_difficulty_ps3move
server.dll!0x009affd8 ConVar sv_bot_difficulty_sharpshooter
server.dll!0x009ab618 ConVar sv_bots_force_rebuy_every_round
server.dll!0x009ab568 ConVar sv_bots_get_easier_each_win
server.dll!0x009ab5c0 ConVar sv_bots_get_harder_after_each_wave
server.dll!0x009a46b0 ConVar sv_bounce
server.dll!0x009bbe78 ConVar sv_breachcharge_arm_delay
server.dll!0x009bbd70 ConVar sv_breachcharge_delay_max
server.dll!0x009bbd18 ConVar sv_breachcharge_delay_min
server.dll!0x009bbcc0 ConVar sv_breachcharge_distance_max
server.dll!0x009bbc68 ConVar sv_breachcharge_distance_min
server.dll!0x009bbe20 ConVar sv_breachcharge_fuse_max
server.dll!0x009bbdc8 ConVar sv_breachcharge_fuse_min
server.dll!0x009b8260 ConVar sv_broadcast_ugc_download_progress_interval
server.dll!0x009b8208 ConVar sv_broadcast_ugc_downloads
server.dll!0x009bbf08 ConVar sv_bumpmine_arm_delay
server.dll!0x009bbf60 ConVar sv_bumpmine_detonate_delay
server.dll!0x009ab460 ConVar sv_buy_status_override
server.dll!0x009ab358 ConVar sv_chat_proximity
server.dll!0x0099c0e8 ConVar sv_clamp_unsafe_velocities
server.dll!0x009b5810 ConVar sv_clip_penetration_traces_to_players
server.dll!0x009a68f8 ConVar sv_clockcorrection_msecs
server.dll!0x009ad920 ConVar sv_coach_comm_unrestricted
server.dll!0x009b3d48 ConVar sv_coaching_enabled
server.dll!0x009b3950 ConVar sv_competitive_minspec
server.dll!0x009b0030 ConVar sv_competitive_official_5v5
server.dll!0x009afdc8 ConVar sv_compute_per_bot_difficulty
server.dll!0x009b5708 ConVar sv_cs_player_speed_has_hostage
server.dll!0x009ab4b8 ConVar sv_ct_spawn_on_bombsite
server.dll!0x009b4668 ConVar sv_damage_print_enable
server.dll!0x009a2c40 ConVar sv_dc_friends_reqd
server.dll!0x009b3a88 ConVar sv_deadtalk
server.dll!0x0099c7e0 ConVar sv_debug_player_use
server.dll!0x009b81b0 ConVar sv_debug_ugc_downloads
server.dll!0x009b00e0 ConVar sv_disable_immunity_alpha
server.dll!0x009ab7d0 ConVar sv_disable_motd
server.dll!0x009aaf38 ConVar sv_disable_observer_interpolation
server.dll!0x009a7850 ConVar sv_disable_pas
server.dll!0x009a76a0 ConVar sv_disable_querycache
server.dll!0x009ab250 ConVar sv_disable_radar
server.dll!0x009b4a30 ConVar sv_disablefreezecam
server.dll!0x009a67f0 ConVar sv_drowning_damage_initial
server.dll!0x009a6848 ConVar sv_drowning_damage_max
server.dll!0x009bb2f8 ConVar sv_dz_autojointeam
server.dll!0x009b5360 ConVar sv_dz_cash_bundle_size
server.dll!0x009b4ea8 ConVar sv_dz_contractkill_reward
server.dll!0x009bb610 ConVar sv_dz_enable_respawn
server.dll!0x009bd3a0 ConVar sv_dz_exploration_payment_amount
server.dll!0x009bd3f8 ConVar sv_dz_exploration_payment_amount_bonus
server.dll!0x009ada28 ConVar sv_dz_hostage_rescue_reward
server.dll!0x009bb2a0 ConVar sv_dz_jointeam_allowed
server.dll!0x009b4980 ConVar sv_dz_parachute_reuse
server.dll!0x009bb248 ConVar sv_dz_player_max_health
server.dll!0x009bb1f0 ConVar sv_dz_player_spawn_armor
server.dll!0x009bb198 ConVar sv_dz_player_spawn_health
server.dll!0x009bb458 ConVar sv_dz_show_enemy_name_scope_range
server.dll!0x009ada80 ConVar sv_dz_squad_wipe_reward
server.dll!0x009bb830 ConVar sv_dz_team_count
server.dll!0x009bb400 ConVar sv_dz_warmup_tablet
server.dll!0x009bb3a8 ConVar sv_dz_warmup_weapon
server.dll!0x0099d2b0 ConVar sv_dz_zone_bombdrop_money_reward
server.dll!0x0099d308 ConVar sv_dz_zone_bombdrop_money_reward_bonus
server.dll!0x0099d360 ConVar sv_dz_zone_damage
server.dll!0x0099d258 ConVar sv_dz_zone_hex_radius
server.dll!0x009aabe8 ConVar sv_enablebunnyhopping
server.dll!0x009afc10 ConVar sv_endmatch_item_drop_interval
server.dll!0x009afd70 ConVar sv_endmatch_item_drop_interval_ancient
server.dll!0x009afd18 ConVar sv_endmatch_item_drop_interval_legendary
server.dll!0x009afcc0 ConVar sv_endmatch_item_drop_interval_mythical
server.dll!0x009afc68 ConVar sv_endmatch_item_drop_interval_rare
server.dll!0x009a17a0 ConVar sv_env_entity_makers_enabled
server.dll!0x009aaa20 ConVar sv_exojump_jumpbonus_forward
server.dll!0x009aa9c8 ConVar sv_exojump_jumpbonus_up
server.dll!0x009aaa78 ConVar sv_exojump_soundramp
server.dll!0x009aa708 ConVar sv_exostaminajumpcost
server.dll!0x009aa760 ConVar sv_exostaminalandcost
server.dll!0x0099c6d8 ConVar sv_extract_ammo_from_dropped_weapons
server.dll!0x009aa8c0 ConVar sv_extreme_strafe_accuracy_fishtail
server.dll!0x009aacf0 ConVar sv_falldamage_exojump_multiplier
server.dll!0x009ab300 ConVar sv_falldamage_scale
server.dll!0x009b3b90 ConVar sv_falldamage_to_below_player_multiplier
server.dll!0x009b3b38 ConVar sv_falldamage_to_below_player_ratio
server.dll!0x009bcc88 ConVar sv_fistpoint_delay
server.dll!0x009bcc30 ConVar sv_fistpunch_blocked_damage
server.dll!0x009bcb28 ConVar sv_fistpunch_damage
server.dll!0x009bcd60 ConVar sv_fistpunch_damage_hard
server.dll!0x009bcb80 ConVar sv_fistpunch_damage_to_player_multiplier
server.dll!0x009bcce0 ConVar sv_fistpunch_impact_sounds
server.dll!0x009bcbd8 ConVar sv_fistpunch_viewmove
server.dll!0x009b8460 ConVar sv_flashbang_strength
server.dll!0x0099c680 ConVar sv_footstep_sound_frequency
server.dll!0x009a4810 ConVar sv_footsteps
server.dll!0x009bb350 ConVar sv_force_reflections
server.dll!0x009a2b38 ConVar sv_force_transmit_ents
server.dll!0x009a6350 ConVar sv_force_transmit_players
server.dll!0x009a4340 ConVar sv_friction
server.dll!0x009b0190 ConVar sv_full_alltalk
server.dll!0x009ad190 ConVar sv_gameinstructor_disable
server.dll!0x009a3b48 ConVar sv_grassburn
server.dll!0x009a5b20 ConVar sv_grenade_trajectory
server.dll!0x009a5c80 ConVar sv_grenade_trajectory_dash
server.dll!0x009a5c28 ConVar sv_grenade_trajectory_thickness
server.dll!0x009a5b78 ConVar sv_grenade_trajectory_time
server.dll!0x009a5bd0 ConVar sv_grenade_trajectory_time_spectator
server.dll!0x009b4928 ConVar sv_guardian_heavy_all
server.dll!0x009b5308 ConVar sv_guardian_heavy_count
server.dll!0x009b48d0 ConVar sv_guardian_max_wave_for_heavy
server.dll!0x009b4878 ConVar sv_guardian_min_wave_for_heavy
server.dll!0x0099b5f0 ConVar sv_health_approach_enabled
server.dll!0x0099b648 ConVar sv_health_approach_speed
server.dll!0x009b8bc0 ConVar sv_hegrenade_damage_multiplier
server.dll!0x009b8c18 ConVar sv_hegrenade_radius_multiplier
server.dll!0x009aafe8 ConVar sv_holiday_mode
server.dll!0x009bbbf8 ConVar sv_ignoregrenaderadio
server.dll!0x0099c8e8 ConVar sv_infinite_ammo
server.dll!0x009aa918 ConVar sv_jump_impulse
server.dll!0x009aa970 ConVar sv_jump_impulse_exojump_multiplier
server.dll!0x009b0088 ConVar sv_kick_ban_duration
server.dll!0x009ab3b0 ConVar sv_kick_players_with_cooldown
server.dll!0x009bcde0 ConVar sv_knife_attack_extend_from_player_aabb
server.dll!0x009a3368 ConVar sv_ladder_angle
server.dll!0x009a3310 ConVar sv_ladder_dampen
server.dll!0x009a33c0 ConVar sv_ladder_scale_speed
server.dll!0x009a6e30 ConVar sv_lagcompensateself
server.dll!0x009a6d28 ConVar sv_lagcompensationforcerestore
server.dll!0x009aaad0 ConVar sv_ledge_mantle_helper
server.dll!0x009aab80 ConVar sv_ledge_mantle_helper_debug
server.dll!0x009aab28 ConVar sv_ledge_mantle_helper_dzonly
server.dll!0x009ab408 ConVar sv_matchend_drops_enabled
server.dll!0x009ad240 ConVar sv_matchpause_auto_5v5
server.dll!0x0099c788 ConVar sv_max_distance_transmit_footsteps
server.dll!0x009a47b8 ConVar sv_maxspeed
server.dll!0x009a6ee0 ConVar sv_maxunlag
server.dll!0x009a6400 ConVar sv_maxusrcmdprocessticks
server.dll!0x009a6b70 ConVar sv_maxusrcmdprocessticks_holdaim
server.dll!0x009a6b18 ConVar sv_maxusrcmdprocessticks_warning
server.dll!0x009a4658 ConVar sv_maxvelocity
server.dll!0x009b56b0 ConVar sv_min_jump_landing_sound
server.dll!0x0099b998 ConVar sv_netvisdist
server.dll!0x009a4708 ConVar sv_noclipaccelerate
server.dll!0x009a64b0 ConVar sv_noclipduringpause
server.dll!0x009a4760 ConVar sv_noclipspeed
server.dll!0x009a2dc8 ConVar sv_occlude_players
server.dll!0x009a3418 ConVar sv_optimizedmovement
server.dll!0x009ab2a8 ConVar sv_outofammo_indicator
server.dll!0x009ad9d0 ConVar sv_party_mode
server.dll!0x009b5ae8 ConVar sv_penetration_type
server.dll!0x009b5058 ConVar sv_player_parachute_velocity
server.dll!0x009aa538 ConVar sv_prime_accounts_only
server.dll!0x009a72e0 ConVar sv_prop_door_open_speed_scale
server.dll!0x009a5078 ConVar sv_pushaway_clientside
server.dll!0x009a7598 ConVar sv_pushaway_clientside_size
server.dll!0x009a5180 ConVar sv_pushaway_force
server.dll!0x009bad40 ConVar sv_pushaway_hostage_force
server.dll!0x009a51d8 ConVar sv_pushaway_max_force
server.dll!0x009bad98 ConVar sv_pushaway_max_hostage_force
server.dll!0x009a5128 ConVar sv_pushaway_max_player_force
server.dll!0x009a5020 ConVar sv_pushaway_min_player_speed
server.dll!0x009a50d0 ConVar sv_pushaway_player_force
server.dll!0x0099b3c0 ConVar sv_pvsskipanimation
server.dll!0x009a63a8 ConVar sv_regeneration_wait_time
server.dll!0x009b8310 ConVar sv_remove_old_ugc_downloads
server.dll!0x009aaf90 ConVar sv_reward_drop_delay
server.dll!0x009a48c0 ConVar sv_rollangle
server.dll!0x009a4868 ConVar sv_rollspeed
server.dll!0x009b3c98 ConVar sv_server_graphic1
server.dll!0x009b3cf0 ConVar sv_server_graphic2
server.dll!0x009b58c0 ConVar sv_server_verify_blood_on_player
server.dll!0x009bd0d0 ConVar sv_shield_explosive_damage_cap
server.dll!0x009bd180 ConVar sv_shield_explosive_damage_crouch_bonus
server.dll!0x009bd128 ConVar sv_shield_explosive_damage_mindist
server.dll!0x009bd078 ConVar sv_shield_explosive_damage_mult
server.dll!0x009bd020 ConVar sv_shield_explosive_damage_scale
server.dll!0x009bd200 ConVar sv_shield_hitpoints
server.dll!0x009afe20 ConVar sv_show_bot_difficulty_in_name
server.dll!0x009ab0f0 ConVar sv_show_team_equipment_force_on
server.dll!0x009ab098 ConVar sv_show_team_equipment_prohibit
server.dll!0x009b4ae0 ConVar sv_show_voip_indicator_for_enemies
server.dll!0x0099b198 ConVar sv_showanimstate
server.dll!0x0099b248 ConVar sv_showanimstate_activities
server.dll!0x0099b1f0 ConVar sv_showanimstate_log
server.dll!0x009b54a0 ConVar sv_showbullethits
server.dll!0x009b54f8 ConVar sv_showimpacts
server.dll!0x009b5448 ConVar sv_showimpacts_penetration
server.dll!0x009b5550 ConVar sv_showimpacts_time
server.dll!0x009a6c78 ConVar sv_showlagcompensation
server.dll!0x009a6cd0 ConVar sv_showlagcompensation_duration
server.dll!0x009b55a8 ConVar sv_showplayerhitboxes
server.dll!0x009b3c40 ConVar sv_skirmish_id
server.dll!0x009a3fd0 ConVar sv_skyname
server.dll!0x009b8e10 ConVar sv_snowball_strength
server.dll!0x009a8338 ConVar sv_soundemitter_trace
server.dll!0x009a8230 ConVar sv_soundemitter_version
server.dll!0x009b46c0 ConVar sv_spawn_afk_bomb_drop_time
server.dll!0x009b5760 ConVar sv_spawn_rappel_min_duration
server.dll!0x009b57b8 ConVar sv_spawn_rappel_min_duration_with_chute
server.dll!0x009b0348 ConVar sv_spec_hear
server.dll!0x009a68a0 ConVar sv_spec_post_death_additional_time
server.dll!0x009b5658 ConVar sv_spec_use_tournament_content_standards
server.dll!0x009a45a8 ConVar sv_specaccelerate
server.dll!0x009a4550 ConVar sv_specnoclip
server.dll!0x009a4600 ConVar sv_specspeed
server.dll!0x009aa658 ConVar sv_staminajumpcost
server.dll!0x009aa6b0 ConVar sv_staminalandcost
server.dll!0x009aa810 ConVar sv_staminamax
server.dll!0x009aa7b8 ConVar sv_staminarecoveryrate
server.dll!0x009a3218 ConVar sv_standable_normal
server.dll!0x009a4970 ConVar sv_stepsize
server.dll!0x009a43f0 ConVar sv_stopspeed
server.dll!0x0099c838 ConVar sv_suppress_viewpunch
server.dll!0x009bd348 ConVar sv_tablet_show_path_to_nearest_resq
server.dll!0x009b0298 ConVar sv_talk_after_dying_time
server.dll!0x009b01e8 ConVar sv_talk_enemy_dead
server.dll!0x009b0240 ConVar sv_talk_enemy_living
server.dll!0x009ab1f8 ConVar sv_teamid_overhead
server.dll!0x009ab040 ConVar sv_teamid_overhead_always_prohibit
server.dll!0x009ab1a0 ConVar sv_teamid_overhead_maxdist
server.dll!0x009ab148 ConVar sv_teamid_overhead_maxdist_spec
server.dll!0x009aa868 ConVar sv_timebetweenducks
server.dll!0x009a7540 ConVar sv_turbophysics
server.dll!0x009bc4a0 ConVar sv_turning_inaccuracy_angle_min
server.dll!0x009bc4f8 ConVar sv_turning_inaccuracy_decay
server.dll!0x009bc448 ConVar sv_turning_inaccuracy_enabled
server.dll!0x009b82b8 ConVar sv_ugc_manager_max_new_file_check_interval_secs
server.dll!0x009a2e98 ConVar sv_unlockedchapters
server.dll!0x009a69a8 ConVar sv_usercmd_custom_random_seed
server.dll!0x009a99f8 ConVar sv_voice_proximity
server.dll!0x009a9a50 ConVar sv_voice_proximity_positional
server.dll!0x009a9db0 ConVar sv_vote_allow_in_warmup
server.dll!0x009a9d00 ConVar sv_vote_allow_spectators
server.dll!0x009a9b48 ConVar sv_vote_command_delay
server.dll!0x009a9d58 ConVar sv_vote_count_spectator_votes
server.dll!0x009a9c50 ConVar sv_vote_creation_timer
server.dll!0x009a9e08 ConVar sv_vote_disallow_kick_on_match_point
server.dll!0x009a9bf8 ConVar sv_vote_failure_timer
server.dll!0x009b7350 ConVar sv_vote_issue_kick_allowed
server.dll!0x009b7400 ConVar sv_vote_issue_loadbackup_allowed
server.dll!0x009b7508 ConVar sv_vote_issue_loadbackup_spec_authoritative
server.dll!0x009b7458 ConVar sv_vote_issue_loadbackup_spec_only
server.dll!0x009b74b0 ConVar sv_vote_issue_loadbackup_spec_safe
server.dll!0x009b7820 ConVar sv_vote_issue_pause_match_spec_only
server.dll!0x009b72a0 ConVar sv_vote_issue_restart_game_allowed
server.dll!0x009b73a8 ConVar sv_vote_kick_ban_duration
server.dll!0x009a9ca8 ConVar sv_vote_quorum_ratio
server.dll!0x009a9af0 ConVar sv_vote_timer_duration
server.dll!0x009b75b8 ConVar sv_vote_to_changelevel_before_match_point
server.dll!0x009a31c0 ConVar sv_walkable_normal
server.dll!0x009a2fb0 ConVar sv_water_movespeed_multiplier
server.dll!0x009a3008 ConVar sv_water_swim_mode
server.dll!0x009a3f20 ConVar sv_wateraccelerate
server.dll!0x009a4080 ConVar sv_waterdist
server.dll!0x009a3f78 ConVar sv_waterfriction
server.dll!0x009b5868 ConVar sv_weapon_encumbrance_per_item
server.dll!0x009aac98 ConVar sv_weapon_encumbrance_scale
server.dll!0x009b51b0 ConVar sv_weapon_require_use_grace_period
server.dll!0x009aa430 ConVar sv_workshop_allow_other_maps
server.dll!0x009bd2d8 ConVar tablet_c4_dist_max
server.dll!0x009bd280 ConVar tablet_c4_dist_min
server.dll!0x009a5d30 ConVar think_limit
server.dll!0x009af9a8 ConVar tr_best_course_time
server.dll!0x009af950 ConVar tr_completed_training
server.dll!0x009a3d00 ConVar tv_allow_autorecording_index
server.dll!0x009aa488 ConVar tv_allow_camera_man_steamid
server.dll!0x009aa4e0 ConVar tv_allow_camera_man_steamid2
server.dll!0x009a3ca8 ConVar tv_allow_static_shots
server.dll!0x009a3bf8 ConVar tv_delay
server.dll!0x009a3c50 ConVar tv_delay1
server.dll!0x009a4b48 ConVar tv_delaymapchange
server.dll!0x009b47c8 ConVar tv_relayradio
server.dll!0x0099ca80 ConVar tv_relaytextchat
server.dll!0x0099c5d0 ConVar view_punch_decay
server.dll!0x0099c628 ConVar view_recoil_tracking
server.dll!0x009a6798 ConVar vis_force
server.dll!0x0099d1a0 ConVar vismon_poll_frequency
server.dll!0x0099d1f8 ConVar vismon_trace_limit
server.dll!0x009a6f50 ConVar voice_player_speaking_delay_threshold
server.dll!0x009bc1e0 ConVar weapon_accuracy_forcespread
server.dll!0x009b8718 ConVar weapon_accuracy_logging
server.dll!0x009bc238 ConVar weapon_accuracy_nospread
server.dll!0x009b8928 ConVar weapon_accuracy_shotgun_spread_patterns
server.dll!0x009bc2e8 ConVar weapon_air_spread_scale
server.dll!0x009bc398 ConVar weapon_auto_cleanup_time
server.dll!0x009b8878 ConVar weapon_debug_inaccuracy_only_up
server.dll!0x009b8820 ConVar weapon_debug_max_inaccuracy
server.dll!0x009bc0d8 ConVar weapon_land_dip_amt
server.dll!0x009bc6e8 ConVar weapon_max_before_cleanup
server.dll!0x009b8cf0 ConVar weapon_molotov_maxdetonateslope
server.dll!0x009b87c8 ConVar weapon_near_empty_sound
server.dll!0x009bc290 ConVar weapon_recoil_cooldown
server.dll!0x009bc130 ConVar weapon_recoil_decay1_exp
server.dll!0x009bc798 ConVar weapon_recoil_decay2_exp
server.dll!0x009bc740 ConVar weapon_recoil_decay2_lin
server.dll!0x009bc188 ConVar weapon_recoil_decay_coefficient
server.dll!0x009bc690 ConVar weapon_recoil_scale
server.dll!0x009bc638 ConVar weapon_recoil_scale_motion_controller
server.dll!0x009b7a30 ConVar weapon_recoil_suppression_factor
server.dll!0x009b79d8 ConVar weapon_recoil_suppression_shots
server.dll!0x009b7a88 ConVar weapon_recoil_variance
server.dll!0x009bc7f0 ConVar weapon_recoil_vel_decay
server.dll!0x009b5918 ConVar weapon_recoil_view_punch_extra
server.dll!0x009bc340 ConVar weapon_reticle_knife_show
server.dll!0x009bc3f0 ConVar weapon_sound_falloff_multiplier
server.dll!0x009a1fe8 ConVar xbox_autothrottle
server.dll!0x009a1f38 ConVar xbox_throttlebias
server.dll!0x009a1f90 ConVar xbox_throttlespoof
```

### ConCommands

<details>
<summary><code>CreatePredictionError</code></summary>

Create a prediction error

flags: `0x4000`  
</details>
<details>
<summary><code>Test_EHandle</code></summary>



flags: `0x4000`  
</details>
<details>
<summary><code>Test_InitRandomEntitySpawner</code></summary>



flags: `0x4000`  
</details>
<details>
<summary><code>Test_ProxyToggle_EnableProxy</code></summary>



flags: `0x4000`  
</details>
<details>
<summary><code>Test_ProxyToggle_SetValue</code></summary>



flags: `0x4000`  
</details>
<details>
<summary><code>Test_RandomizeInPVS</code></summary>



flags: `0x4000`  
</details>
<details>
<summary><code>Test_RemoveAllRandomEntities</code></summary>



flags: `0x4000`  
</details>
<details>
<summary><code>Test_SpawnRandomEntities</code></summary>



flags: `0x4000`  
</details>
<details>
<summary><code>_resetgamestats</code></summary>

Erases current game stats and writes out a blank stats file

flags: `0x0`  
</details>
<details>
<summary><code>ai_clear_bad_links</code></summary>

Clears bits set on nav links indicating link is unusable 

flags: `0x0`  
</details>
<details>
<summary><code>ai_debug_node_connect</code></summary>

Debug the attempted connection between two nodes

flags: `0x0`  
</details>
<details>
<summary><code>ai_disable</code></summary>

Bi-passes all AI logic routines and puts all NPCs into their idle animations.  Can be used to get NPCs out of your way and to test effect of AI logic routines on frame rate

flags: `0x4000`  
</details>
<details>
<summary><code>ai_drop_hint</code></summary>

Drop an ai_hint at the player's current eye position.

flags: `0x4000`  
</details>
<details>
<summary><code>ai_dump_hints</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>ai_hull</code></summary>

Controls which connections are shown when ai_show_hull or ai_show_connect commands are used
	Arguments:	NPC name or classname, <none>=NPC under crosshair

flags: `0x4000`  
</details>
<details>
<summary><code>ai_next_hull</code></summary>

Cycles through the various hull sizes.  Currently selected hull size is written to the screen.  Controls which connections are shown when ai_show_hull or ai_show_connect commands are used
	Arguments:	-none-

flags: `0x4000`  
</details>
<details>
<summary><code>ai_nodes</code></summary>

Toggles node display.  First call displays the nodes for the given network as green objects.  Second call  displays the nodes and their IDs.  Nodes are color coded as follows:
	Green		- ground node
	Cyan		- air node
	Magenta	- climb node
	Grey		- node not available for selected hull size
	Orange 	- node currently locked

flags: `0x4000`  
</details>
<details>
<summary><code>ai_resume</code></summary>

If NPC is stepping through tasks (see ai_step ) will resume normal processing.

flags: `0x4000`  
</details>
<details>
<summary><code>ai_set_move_height_epsilon</code></summary>

Set how high AI bumps up ground walkers when checking steps

flags: `0x0`  
</details>
<details>
<summary><code>ai_setenabled</code></summary>

Like ai_disable but you manually specify the state (with a 0 or 1) instead of toggling it.

flags: `0x4000`  
</details>
<details>
<summary><code>ai_show_connect</code></summary>

Displays the allowed connections between each node for the currently selected hull type.  Hulls are color code as follows:
	Green		- ground movement 
	Blue		- jumping movement
	Cyan		- flying movement
	
	Yellow		- crawling movement
	Magenta	- climbing movement
	Red		- connection disabled

flags: `0x4000`  
</details>
<details>
<summary><code>ai_show_connect_crawl</code></summary>

Displays the allowed connections between each node for the currently selected hull type.  Hulls are color code as follows:
	Green		- ground movement 
	Blue		- jumping movement
	Cyan		- flying movement
	Yellow		- crawling movement
	Magenta	- climbing movement
	Red		- connection disabled

flags: `0x4000`  
</details>
<details>
<summary><code>ai_show_connect_fly</code></summary>

Displays the allowed connections between each node for the currently selected hull type.  Hulls are color code as follows:
	Green		- ground movement 
	Blue		- jumping movement
	Cyan		- flying movement
	
	Yellow		- crawling movement
	Magenta	- climbing movement
	Red		- connection disabled

flags: `0x4000`  
</details>
<details>
<summary><code>ai_show_connect_jump</code></summary>

Displays the allowed connections between each node for the currently selected hull type.  Hulls are color code as follows:
	Green		- ground movement 
	Blue		- jumping movement
	Cyan		- flying movement
	
	Yellow		- crawling movement
	Magenta	- climbing movement
	Red		- connection disabled

flags: `0x4000`  
</details>
<details>
<summary><code>ai_show_graph_connect</code></summary>

Toggles graph connection display for the node that the player is looking at.  Nodes that are connected to the selected node by the net graph will be drawn in red with magenta lines connecting to the selected node.  Nodes that are not connected via the net graph from the selected node will be drawn in blue.

flags: `0x4000`  
</details>
<details>
<summary><code>ai_show_grid</code></summary>

Draw a grid on the floor where looking.

flags: `0x4000`  
</details>
<details>
<summary><code>ai_show_hints</code></summary>

Displays all hints as small boxes
	Blue		- hint is available for use
	Red		- hint is currently being used by an NPC
	Orange		- hint not being used by timed out
	Grey		- hint has been disabled

flags: `0x4000`  
</details>
<details>
<summary><code>ai_show_hull</code></summary>

Displays the allowed hulls between each node for the currently selected hull type.  Hulls are color code as follows:
	Green		- ground movement 
	Blue		- jumping movement
	Cyan		- flying movement
	
	Yellow		- crawling movement
	Magenta	- climbing movement
	Arguments: 	-none-

flags: `0x4000`  
</details>
<details>
<summary><code>ai_show_node</code></summary>

Highlight the specified node

flags: `0x4000`  
</details>
<details>
<summary><code>ai_show_visibility</code></summary>

Toggles visibility display for the node that the player is looking at.  Nodes that are visible from the selected node will be drawn in red with yellow lines connecting to the selected node.  Nodes that are not visible from the selected node will be drawn in blue.

flags: `0x4000`  
</details>
<details>
<summary><code>ai_step</code></summary>

NPCs will freeze after completing their current task.  To complete the next task, use 'ai_step' again.  To resume processing normally use 'ai_resume'

flags: `0x4000`  
</details>
<details>
<summary><code>ai_test_los</code></summary>

Test AI LOS from the player's POV

flags: `0x0`  
</details>
<details>
<summary><code>ainet_generate_report</code></summary>

Generate a report to the console.

flags: `0x0`  
</details>
<details>
<summary><code>ainet_generate_report_only</code></summary>

Generate a report to the console.

flags: `0x0`  
</details>
<details>
<summary><code>air_density</code></summary>

Changes the density of air for drag computations.

flags: `0x4000`  
</details>
<details>
<summary><code>bot_add</code></summary>

bot_add <t|ct> <type> <difficulty> <name> - Adds a bot matching the given criteria.

flags: `0x4`  
</details>
<details>
<summary><code>bot_add_ct</code></summary>

bot_add_ct <type> <difficulty> <name> - Adds a Counter-Terrorist bot matching the given criteria.

flags: `0x4`  
</details>
<details>
<summary><code>bot_add_t</code></summary>

bot_add_t <type> <difficulty> <name> - Adds a terrorist bot matching the given criteria.

flags: `0x4`  
</details>
<details>
<summary><code>bot_all_weapons</code></summary>

Allows the bots to use all weapons

flags: `0x4`  
</details>
<details>
<summary><code>bot_control_next_all_teams</code></summary>

Take control of the next bot regardless of team (development only).

flags: `0x40004002`  
</details>
<details>
<summary><code>bot_goto_mark</code></summary>

Sends a bot to the marked nav area (useful for testing navigation meshes)

flags: `0x4004`  
</details>
<details>
<summary><code>bot_goto_selected</code></summary>

Sends a bot to the selected nav area (useful for testing navigation meshes)

flags: `0x4004`  
</details>
<details>
<summary><code>bot_kick</code></summary>

bot_kick <all> <t|ct> <type> <difficulty> <name> - Kicks a specific bot, or all bots, matching the given criteria.

flags: `0x4`  
</details>
<details>
<summary><code>bot_kill</code></summary>

bot_kill <all> <t|ct> <type> <difficulty> <name> - Kills a specific bot, or all bots, matching the given criteria.

flags: `0x4004`  
</details>
<details>
<summary><code>bot_knives_only</code></summary>

Restricts the bots to only using knives

flags: `0x4`  
</details>
<details>
<summary><code>bot_pistols_only</code></summary>

Restricts the bots to only using pistols

flags: `0x4`  
</details>
<details>
<summary><code>bot_place</code></summary>

bot_place - Places a bot from the map at where the local player is pointing.

flags: `0x4004`  
</details>
<details>
<summary><code>bot_snipers_only</code></summary>

Restricts the bots to only using sniper rifles

flags: `0x4`  
</details>
<details>
<summary><code>buddha</code></summary>

Toggle.  Player takes damage but won't die. (Shows red cross when health is zero)

flags: `0x4000`  
</details>
<details>
<summary><code>buyrandom</code></summary>

Buy random primary and secondary. Primarily for deathmatch where cost is not an issue.

flags: `0x400`  
</details>
<details>
<summary><code>callvote</code></summary>

Start a vote on an issue.

flags: `0x400`  
</details>
<details>
<summary><code>cast_hull</code></summary>

Tests hull collision detection

flags: `0x4000`  
</details>
<details>
<summary><code>cast_ray</code></summary>

Tests collision detection

flags: `0x4000`  
</details>
<details>
<summary><code>ch_createairboat</code></summary>

Spawn airboat in front of the player.

flags: `0x4000`  
</details>
<details>
<summary><code>ch_createjeep</code></summary>

Spawn jeep in front of the player.

flags: `0x4000`  
</details>
<details>
<summary><code>cl_csm_server_status</code></summary>

Usage:
 cl_csm_server_status


flags: `0x0`  
</details>
<details>
<summary><code>clear_bombs</code></summary>



flags: `0x4000`  
</details>
<details>
<summary><code>clear_debug_overlays</code></summary>

clears debug overlays

flags: `0x0`  
</details>
<details>
<summary><code>collision_test</code></summary>

Tests collision system

flags: `0x4000`  
</details>
<details>
<summary><code>commentary_cvarsnotchanging</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>commentary_finishnode</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>create_flashlight</code></summary>



flags: `0x4000`  
</details>
<details>
<summary><code>creditsdone</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>cs_make_vip</code></summary>

Marks a player as the VIP

flags: `0x0`  
</details>
<details>
<summary><code>dbghist_addline</code></summary>

Add a line to the debug history. Format: <category id> <line>

flags: `0x0`  
</details>
<details>
<summary><code>dbghist_dump</code></summary>

Dump the debug history to the console. Format: <category id>
    Categories:
     0: Entity I/O
     1: AI Decisions
     2: Scene Print
     3: Alyx Blind
     4: Log of damage done to player

flags: `0x0`  
</details>
<details>
<summary><code>dm_reset_spawns</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>drawcross</code></summary>

Draws a cross at the given location
	Arguments: x y z

flags: `0x4000`  
</details>
<details>
<summary><code>drawline</code></summary>

Draws line between two 3D Points.
	Green if no collision
	Red is collides with something
	Arguments: x1 y1 z1 x2 y2 z2

flags: `0x4000`  
</details>
<details>
<summary><code>ds_get_newest_subscribed_files</code></summary>

Re-reads web api auth key and subscribed file lists from disk and downloads the latest updates of those files from steam

flags: `0x80000`  
</details>
<details>
<summary><code>dump_entity_sizes</code></summary>

Print sizeof(entclass)

flags: `0x0`  
</details>
<details>
<summary><code>dump_globals</code></summary>

Dump all global entities/states

flags: `0x0`  
</details>
<details>
<summary><code>dumpentityfactories</code></summary>

Lists all entity factory names.

flags: `0x4`  
</details>
<details>
<summary><code>dumpeventqueue</code></summary>

Dump the contents of the Entity I/O event queue to the console.

flags: `0x0`  
</details>
<details>
<summary><code>dumpgamestringtable</code></summary>

Dump the contents of the game string table to the console.

flags: `0x4000`  
</details>
<details>
<summary><code>dz_clearteams</code></summary>

Clear all DZ teams

flags: `0x4`  
</details>
<details>
<summary><code>dz_jointeam</code></summary>

dz_jointeam team# [userid#|name] - Join DZ team N (0 to leave your team).  Server admins can assign other players to teams.

flags: `0x400`  
</details>
<details>
<summary><code>dz_shuffle_teams</code></summary>

Shuffle all teams for Danger Zone

flags: `0x4`  
</details>
<details>
<summary><code>dz_spawnselect_choose_hex</code></summary>



flags: `0x404`  
</details>
<details>
<summary><code>endround</code></summary>

End the current round.

flags: `0x4000`  
</details>
<details>
<summary><code>ent_absbox</code></summary>

Displays the total bounding box for the given entity(s) in green.  Some entites will also display entity specific overlays.
	Arguments:   	{entity_name} / {class_name} / no argument picks what player is looking at 

flags: `0x4000`  
</details>
<details>
<summary><code>ent_attachments</code></summary>

Displays the attachment points on an entity.
	Arguments:   	{entity_name} / {class_name} / no argument picks what player is looking at 

flags: `0x4000`  
</details>
<details>
<summary><code>ent_autoaim</code></summary>

Displays the entity's autoaim radius.
	Arguments:   	{entity_name} / {class_name} / no argument picks what player is looking at

flags: `0x4000`  
</details>
<details>
<summary><code>ent_bbox</code></summary>

Displays the movement bounding box for the given entity(ies) in orange.  Some entites will also display entity specific overlays.
	Arguments:   	{entity_name} / {class_name} / no argument picks what player is looking at 

flags: `0x4000`  
</details>
<details>
<summary><code>ent_cancelpendingentfires</code></summary>

Cancels all ent_fire created outputs that are currently waiting for their delay to expire.

flags: `0x0`  
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


flags: `0x4000`  
</details>
<details>
<summary><code>ent_info</code></summary>

Usage:
   ent_info <class name>


flags: `0x4000`  
</details>
<details>
<summary><code>ent_keyvalue</code></summary>

Applies the comma delimited key=value pairs to the entity with the given Hammer ID.
	Format: ent_keyvalue <entity id> <key1>=<value1>,<key2>=<value2>,...,<keyN>=<valueN>


flags: `0x4000`  
</details>
<details>
<summary><code>ent_list_report</code></summary>

Reports all list of all entities in a map, one by one

flags: `0x0`  
</details>
<details>
<summary><code>ent_messages</code></summary>

Toggles input/output message display for the selected entity(ies).  The name of the entity will be displayed as well as any messages that it sends or receives.
	Arguments:   	{entity_name} / {class_name} / no argument picks what player is looking at

flags: `0x4000`  
</details>
<details>
<summary><code>ent_name</code></summary>



flags: `0x4000`  
</details>
<details>
<summary><code>ent_orient</code></summary>

Orient the specified entity to match the player's angles. By default, only orients target entity's YAW. Use the 'allangles' option to orient on all axis.
	Format: ent_orient <entity name> <optional: allangles>

flags: `0x4000`  
</details>
<details>
<summary><code>ent_pause</code></summary>

Toggles pausing of input/output message processing for entities.  When turned on processing of all message will stop.  Any messages displayed with 'ent_messages' will stop fading and be displayed indefinitely. To step through the messages one by one use 'ent_step'.

flags: `0x4000`  
</details>
<details>
<summary><code>ent_pivot</code></summary>

Displays the pivot for the given entity(ies).
	(y=up=green, z=forward=blue, x=left=red). 
	Arguments:   	{entity_name} / {class_name} / no argument picks what player is looking at 

flags: `0x4000`  
</details>
<details>
<summary><code>ent_rbox</code></summary>

Displays the total bounding box for the given entity(s) in green.  Some entites will also display entity specific overlays.
	Arguments:   	{entity_name} / {class_name} / no argument picks what player is looking at 

flags: `0x4000`  
</details>
<details>
<summary><code>ent_remove</code></summary>

Removes the given entity(s)
	Arguments:   	{entity_name} / {class_name} / no argument picks what player is looking at 

flags: `0x4000`  
</details>
<details>
<summary><code>ent_remove_all</code></summary>

Removes all entities of the specified type
	Arguments:   	{entity_name} / {class_name} 

flags: `0x4000`  
</details>
<details>
<summary><code>ent_rotate</code></summary>

Rotates an entity by a specified # of degrees

flags: `0x4000`  
</details>
<details>
<summary><code>ent_script_dump</code></summary>

Dumps the names and values of this entity's script scope to the console
	Arguments:   	{entity_name} / {class_name} / no argument picks what player is looking at 

flags: `0x4000`  
</details>
<details>
<summary><code>ent_setang</code></summary>

Set entity angles

flags: `0x4000`  
</details>
<details>
<summary><code>ent_setname</code></summary>

Sets the targetname of the given entity(s)
	Arguments:   	{new entity name} {entity_name} / {class_name} / no argument picks what player is looking at 

flags: `0x4000`  
</details>
<details>
<summary><code>ent_setpos</code></summary>

Move entity to position

flags: `0x4000`  
</details>
<details>
<summary><code>ent_show_response_criteria</code></summary>

Print, to the console, an entity's current criteria set used to select responses.
	Arguments:   	{entity_name} / {class_name} / no argument picks what player is looking at 

flags: `0x4000`  
</details>
<details>
<summary><code>ent_step</code></summary>

When 'ent_pause' is set this will step through one waiting input / output message at a time.

flags: `0x4000`  
</details>
<details>
<summary><code>ent_teleport</code></summary>

Teleport the specified entity to where the player is looking.
	Format: ent_teleport <entity name>

flags: `0x4000`  
</details>
<details>
<summary><code>ent_text</code></summary>

Displays text debugging information about the given entity(ies) on top of the entity (See Overlay Text)
	Arguments:   	{entity_name} / {class_name} / no argument picks what player is looking at 

flags: `0x4000`  
</details>
<details>
<summary><code>ent_viewoffset</code></summary>

Displays the eye position for the given entity(ies) in red.
	Arguments:   	{entity_name} / {class_name} / no argument picks what player is looking at 

flags: `0x4000`  
</details>
<details>
<summary><code>exojump</code></summary>

equips or removes exojump

flags: `0x4000`  
</details>
<details>
<summary><code>explode</code></summary>

Kills the player with explosive damage

flags: `0x400`  
</details>
<details>
<summary><code>explodevector</code></summary>

Kills a player applying an explosive force. Usage: explodevector <player> <x value> <y value> <z value>

flags: `0x400`  
</details>
<details>
<summary><code>fadein</code></summary>

fadein {time r g b}: Fades the screen in from black or from the specified color over the given number of seconds.

flags: `0x4000`  
</details>
<details>
<summary><code>fadeout</code></summary>

fadeout {time r g b}: Fades the screen to black or to the specified color over the given number of seconds.

flags: `0x4000`  
</details>
<details>
<summary><code>find_ent</code></summary>

Find and list all entities with classnames or targetnames that contain the specified substring.
Format: find_ent <substring>


flags: `0x4000`  
</details>
<details>
<summary><code>find_ent_index</code></summary>

Display data for entity matching specified index.
Format: find_ent_index <index>


flags: `0x4000`  
</details>
<details>
<summary><code>firetarget</code></summary>



flags: `0x4000`  
</details>
<details>
<summary><code>foundry_engine_get_mouse_control</code></summary>

Give the engine control of the mouse.

flags: `0x4000`  
</details>
<details>
<summary><code>foundry_engine_release_mouse_control</code></summary>

Give the control of the mouse back to Hammer.

flags: `0x4000`  
</details>
<details>
<summary><code>foundry_select_entity</code></summary>

Select the entity under the crosshair or select entities with the specified name.

flags: `0x4000`  
</details>
<details>
<summary><code>foundry_sync_hammer_view</code></summary>

Move Hammer's 3D view to the same position as the engine's 3D view.

flags: `0x4000`  
</details>
<details>
<summary><code>foundry_update_entity</code></summary>

Updates the entity's position/angles when in edit mode

flags: `0x4000`  
</details>
<details>
<summary><code>give</code></summary>

Give item to player.
	Arguments: <item_name>

flags: `0x400`  
</details>
<details>
<summary><code>givecurrentammo</code></summary>

Give a supply of ammo for current weapon..


flags: `0x4000`  
</details>
<details>
<summary><code>global_set</code></summary>

global_set <globalname> <state>: Sets the state of the given env_global (0 = OFF, 1 = ON, 2 = DEAD).

flags: `0x4000`  
</details>
<details>
<summary><code>god</code></summary>

Toggle. Player becomes invulnerable.

flags: `0x4000`  
</details>
<details>
<summary><code>gods</code></summary>

Toggle. All players become invulnerable.

flags: `0x4000`  
</details>
<details>
<summary><code>groundlist</code></summary>

Display ground entity list <index>

flags: `0x0`  
</details>
<details>
<summary><code>hammer_update_entity</code></summary>

Updates the entity's position/angles when in edit mode

flags: `0x0`  
</details>
<details>
<summary><code>hammer_update_safe_entities</code></summary>

Updates entities in the map that can safely be updated (don't have parents or are affected by constraints). Also excludes entities mentioned in any hammer_updateignorelist objects in this map.

flags: `0x0`  
</details>
<details>
<summary><code>host_workshop_collection</code></summary>

Get the latest version of maps in a workshop collection and host them as a maplist.

flags: `0x0`  
</details>
<details>
<summary><code>host_workshop_map</code></summary>

Get the latest version of the map and host it on this server.

flags: `0x0`  
</details>
<details>
<summary><code>hurtme</code></summary>

Hurts the player.
	Arguments: <health to lose>

flags: `0x4000`  
</details>
<details>
<summary><code>itemtimedata_dump_active</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>itemtimedata_dump_total</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>itemtimedata_print_and_reset</code></summary>

Outputs item time data to server log and clears data.

flags: `0x0`  
</details>
<details>
<summary><code>kdtree_test</code></summary>

Tests spatial partition for entities queries.

flags: `0x4000`  
</details>
<details>
<summary><code>kill</code></summary>

Kills the player with generic damage

flags: `0x400`  
</details>
<details>
<summary><code>killvector</code></summary>

Kills a player applying force. Usage: killvector <player> <x value> <y value> <z value>

flags: `0x400`  
</details>
<details>
<summary><code>listRecentNPCSpeech</code></summary>

Displays a list of the last 5 lines of speech from NPCs.

flags: `0x20004`  
</details>
<details>
<summary><code>listissues</code></summary>

List all the issues that can be voted on.

flags: `0x400`  
</details>
<details>
<summary><code>load_master_item_schema</code></summary>

Reloads the item master schema.

flags: `0x4016`  
</details>
<details>
<summary><code>logaddress_add_http</code></summary>

Set URI of a listener to receive logs via http post. Wrap URI in double quotes.

flags: `0x80800`  
</details>
<details>
<summary><code>logaddress_delall_http</code></summary>

Remove all http listeners from the dispatch list.

flags: `0x80800`  
</details>
<details>
<summary><code>logaddress_list_http</code></summary>

List all URIs currently receiving server logs

flags: `0x80800`  
</details>
<details>
<summary><code>map_setbombradius</code></summary>

Sets the bomb radius for the map.

flags: `0x4000`  
</details>
<details>
<summary><code>map_showbombradius</code></summary>

Shows bomb radius from the center of each bomb site and planted bomb.

flags: `0x4000`  
</details>
<details>
<summary><code>map_showspawnpoints</code></summary>

Shows player spawn points (red=invalid). Optionally pass in the duration.

flags: `0x0`  
</details>
<details>
<summary><code>mp_backup_restore_list_files</code></summary>

Lists recent backup round files matching the prefix, most recent files first, accepts a numeric parameter to limit the number of files displayed

flags: `0x80004`  
</details>
<details>
<summary><code>mp_backup_restore_load_file</code></summary>

Loads player cash, KDA, scores and team scores; resets to the next round after the backup

flags: `0x80004`  
</details>
<details>
<summary><code>mp_disable_autokick</code></summary>

Prevents a userid from being auto-kicked

flags: `0x0`  
</details>
<details>
<summary><code>mp_dump_timers</code></summary>

Prints round timers to the console for debugging

flags: `0x0`  
</details>
<details>
<summary><code>mp_forcerespawnplayers</code></summary>

Force all players to respawn.

flags: `0x4000`  
</details>
<details>
<summary><code>mp_forcewin</code></summary>

Forces team to win

flags: `0x4000`  
</details>
<details>
<summary><code>mp_pause_match</code></summary>

Pause the match in the next freeze time

flags: `0x0`  
</details>
<details>
<summary><code>mp_scrambleteams</code></summary>

Scramble the teams and restart the game

flags: `0x0`  
</details>
<details>
<summary><code>mp_swapteams</code></summary>

Swap the teams and restart the game

flags: `0x0`  
</details>
<details>
<summary><code>mp_switchteams</code></summary>

Switch teams and restart the game

flags: `0x0`  
</details>
<details>
<summary><code>mp_tournament_restart</code></summary>

Restart Tournament Mode on the current level.

flags: `0x0`  
</details>
<details>
<summary><code>mp_unpause_match</code></summary>

Resume the match

flags: `0x0`  
</details>
<details>
<summary><code>mp_warmup_end</code></summary>

End warmup immediately.

flags: `0x0`  
</details>
<details>
<summary><code>mp_warmup_start</code></summary>

Start warmup.

flags: `0x0`  
</details>
<details>
<summary><code>nav_add_to_selected_set</code></summary>

Add current area to the selected set.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_add_to_selected_set_by_id</code></summary>

Add specified area id to the selected set.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_analyze</code></summary>

Re-analyze the current Navigation Mesh and save it to disk.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_analyze_scripted</code></summary>

commandline hook to run a nav_analyze and then quit.

flags: `0x4014`  
</details>
<details>
<summary><code>nav_avoid</code></summary>

Toggles the 'avoid this area when possible' flag used by the AI system.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_begin_area</code></summary>

Defines a corner of a new Area or Ladder. To complete the Area or Ladder, drag the opposite corner to the desired location and issue a 'nav_end_area' command.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_begin_deselecting</code></summary>

Start continuously removing from the selected set.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_begin_drag_deselecting</code></summary>

Start dragging a selection area.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_begin_drag_selecting</code></summary>

Start dragging a selection area.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_begin_selecting</code></summary>

Start continuously adding to the selected set.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_begin_shift_xy</code></summary>

Begin shifting the Selected Set.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_build_ladder</code></summary>

Attempts to build a nav ladder on the climbable surface under the cursor.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_check_connectivity</code></summary>

Checks to be sure every (or just the marked) nav area can get to every goal area for the map (hostages or bomb site).

flags: `0x4000`  
</details>
<details>
<summary><code>nav_check_file_consistency</code></summary>

Scans the maps directory and reports any missing/out-of-date navigation files.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_check_floor</code></summary>

Updates the blocked/unblocked status for every nav area.

flags: `0x4`  
</details>
<details>
<summary><code>nav_check_stairs</code></summary>

Update the nav mesh STAIRS attribute

flags: `0x4000`  
</details>
<details>
<summary><code>nav_chop_selected</code></summary>

Chops all selected areas into their component 1x1 areas

flags: `0x4000`  
</details>
<details>
<summary><code>nav_clear_attribute</code></summary>

Remove given nav attribute from all areas in the selected set.

flags: `0x4000`  
</details>
<details>
<summary><code>nav_clear_selected_set</code></summary>

Clear the selected set.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_clear_walkable_marks</code></summary>

Erase any previously placed walkable positions.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_compress_id</code></summary>

Re-orders area and ladder ID's so they are continuous.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_connect</code></summary>

To connect two Areas, mark the first Area, highlight the second Area, then invoke the connect command. Note that this creates a ONE-WAY connection from the first to the second Area. To make a two-way connection, also connect the second area to the first.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_corner_lower</code></summary>

Lower the selected corner of the currently marked Area.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_corner_place_on_ground</code></summary>

Places the selected corner of the currently marked Area on the ground.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_corner_raise</code></summary>

Raise the selected corner of the currently marked Area.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_corner_select</code></summary>

Select a corner of the currently marked Area. Use multiple times to access all four corners.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_crouch</code></summary>

Toggles the 'must crouch in this area' flag used by the AI system.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_delete</code></summary>

Deletes the currently highlighted Area.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_delete_marked</code></summary>

Deletes the currently marked Area (if any).

flags: `0x4004`  
</details>
<details>
<summary><code>nav_disconnect</code></summary>

To disconnect two Areas, mark an Area, highlight a second Area, then invoke the disconnect command. This will remove all connections between the two Areas.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_dont_hide</code></summary>

Toggles the 'area is not suitable for hiding spots' flag used by the AI system.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_end_area</code></summary>

Defines the second corner of a new Area or Ladder and creates it.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_end_deselecting</code></summary>

Stop continuously removing from the selected set.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_end_drag_deselecting</code></summary>

Stop dragging a selection area.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_end_drag_selecting</code></summary>

Stop dragging a selection area.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_end_selecting</code></summary>

Stop continuously adding to the selected set.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_end_shift_xy</code></summary>

Finish shifting the Selected Set.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_flood_select</code></summary>

Selects the current Area and all Areas connected to it, recursively. To clear a selection, use this command again.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_gen_cliffs_approx</code></summary>

Mark cliff areas, post-processing approximation

flags: `0x4000`  
</details>
<details>
<summary><code>nav_generate</code></summary>

Generate a Navigation Mesh for the current map and save it to disk.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_generate_incremental</code></summary>

Generate a Navigation Mesh for the current map and save it to disk.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_jump</code></summary>

Toggles the 'traverse this area by jumping' flag used by the AI system.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_ladder_flip</code></summary>

Flips the selected ladder's direction.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_load</code></summary>

Loads the Navigation Mesh for the current map.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_lower_drag_volume_max</code></summary>

Lower the top of the drag select volume.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_lower_drag_volume_min</code></summary>

Lower the bottom of the drag select volume.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_make_sniper_spots</code></summary>

Chops the marked area into disconnected sub-areas suitable for sniper spots.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_mark</code></summary>

Marks the Area or Ladder under the cursor for manipulation by subsequent editing commands.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_mark_attribute</code></summary>

Set nav attribute for all areas in the selected set.

flags: `0x4000`  
</details>
<details>
<summary><code>nav_mark_unnamed</code></summary>

Mark an Area with no Place name. Useful for finding stray areas missed when Place Painting.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_mark_walkable</code></summary>

Mark the current location as a walkable position. These positions are used as seed locations when sampling the map to generate a Navigation Mesh.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_merge</code></summary>

To merge two Areas into one, mark the first Area, highlight the second by pointing your cursor at it, and invoke the merge command.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_merge_mesh</code></summary>

Merges a saved selected set into the current mesh.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_no_hostages</code></summary>

Toggles the 'hostages cannot use this area' flag used by the AI system.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_no_jump</code></summary>

Toggles the 'dont jump in this area' flag used by the AI system.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_place_floodfill</code></summary>

Sets the Place of the Area under the cursor to the curent Place, and 'flood-fills' the Place to all adjacent Areas. Flood-filling stops when it hits an Area with the same Place, or a different Place than that of the initial Area.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_place_list</code></summary>

Lists all place names used in the map.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_place_pick</code></summary>

Sets the current Place to the Place of the Area under the cursor.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_place_replace</code></summary>

Replaces all instances of the first place with the second place.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_place_set</code></summary>

Sets the Place of all selected areas to the current Place.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_precise</code></summary>

Toggles the 'dont avoid obstacles' flag used by the AI system.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_raise_drag_volume_max</code></summary>

Raise the top of the drag select volume.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_raise_drag_volume_min</code></summary>

Raise the bottom of the drag select volume.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_recall_selected_set</code></summary>

Re-selects the stored selected set.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_remove_from_selected_set</code></summary>

Remove current area from the selected set.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_remove_jump_areas</code></summary>

Removes legacy jump areas, replacing them with connections.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_run</code></summary>

Toggles the 'traverse this area by running' flag used by the AI system.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_save</code></summary>

Saves the current Navigation Mesh to disk.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_save_selected</code></summary>

Writes the selected set to disk for merging into another mesh via nav_merge_mesh.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_select_blocked_areas</code></summary>

Adds all blocked areas to the selected set

flags: `0x4000`  
</details>
<details>
<summary><code>nav_select_damaging_areas</code></summary>

Adds all damaging areas to the selected set

flags: `0x4000`  
</details>
<details>
<summary><code>nav_select_half_space</code></summary>

Selects any areas that intersect the given half-space.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_select_invalid_areas</code></summary>

Adds all invalid areas to the Selected Set.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_select_obstructed_areas</code></summary>

Adds all obstructed areas to the selected set

flags: `0x4000`  
</details>
<details>
<summary><code>nav_select_overlapping</code></summary>

Selects nav areas that are overlapping others.

flags: `0x4`  
</details>
<details>
<summary><code>nav_select_radius</code></summary>

Adds all areas in a radius to the selection set

flags: `0x4000`  
</details>
<details>
<summary><code>nav_select_stairs</code></summary>

Adds all stairway areas to the selected set

flags: `0x4000`  
</details>
<details>
<summary><code>nav_set_place_mode</code></summary>

Sets the editor into or out of Place mode. Place mode allows labelling of Area with Place names.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_shift</code></summary>

Shifts the selected areas by the specified amount

flags: `0x4000`  
</details>
<details>
<summary><code>nav_simplify_selected</code></summary>

Chops all selected areas into their component 1x1 areas and re-merges them together into larger areas

flags: `0x4000`  
</details>
<details>
<summary><code>nav_splice</code></summary>

To splice, mark an area, highlight a second area, then invoke the splice command to create a new, connected area between them.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_split</code></summary>

To split an Area into two, align the split line using your cursor and invoke the split command.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_stand</code></summary>

Toggles the 'stand while hiding' flag used by the AI system.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_stop</code></summary>

Toggles the 'must stop when entering this area' flag used by the AI system.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_store_selected_set</code></summary>

Stores the current selected set for later retrieval.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_strip</code></summary>

Strips all Hiding Spots, Approach Points, and Encounter Spots from the current Area.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_subdivide</code></summary>

Subdivides all selected areas.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_test_stairs</code></summary>

Test the selected set for being on stairs

flags: `0x4000`  
</details>
<details>
<summary><code>nav_toggle_deselecting</code></summary>

Start or stop continuously removing from the selected set.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_toggle_in_selected_set</code></summary>

Remove current area from the selected set.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_toggle_place_mode</code></summary>

Toggle the editor into and out of Place mode. Place mode allows labelling of Area with Place names.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_toggle_place_painting</code></summary>

Toggles Place Painting mode. When Place Painting, pointing at an Area will 'paint' it with the current Place.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_toggle_selected_set</code></summary>

Toggles all areas into/out of the selected set.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_toggle_selecting</code></summary>

Start or stop continuously adding to the selected set.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_transient</code></summary>

Toggles the 'area is transient and may become blocked' flag used by the AI system.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_unmark</code></summary>

Clears the marked Area or Ladder.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_update_blocked</code></summary>

Updates the blocked/unblocked status for every nav area.

flags: `0x4`  
</details>
<details>
<summary><code>nav_update_lighting</code></summary>

Recomputes lighting values

flags: `0x4000`  
</details>
<details>
<summary><code>nav_use_place</code></summary>

If used without arguments, all available Places will be listed. If a Place argument is given, the current Place is set.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_walk</code></summary>

Toggles the 'traverse this area by walking' flag used by the AI system.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_warp_to_mark</code></summary>

Warps the player to the marked area.

flags: `0x4004`  
</details>
<details>
<summary><code>nav_world_center</code></summary>

Centers the nav mesh in the world

flags: `0x4000`  
</details>
<details>
<summary><code>noclip</code></summary>

Toggle. Player becomes non-solid and flies.  Optional argument of 0 or 1 to force enable/disable

flags: `0x4000`  
</details>
<details>
<summary><code>notarget</code></summary>

Toggle. Player becomes hidden to NPCs.

flags: `0x4000`  
</details>
<details>
<summary><code>npc_ammo_deplete</code></summary>

Subtracts half of the target's ammo

flags: `0x0`  
</details>
<details>
<summary><code>npc_bipass</code></summary>

Displays the local movement attempts by the given NPC(s) (triangulation detours).  Failed bypass routes are displayed in red, successful bypasses are shown in green.
	Arguments:   	{entity_name} / {class_name} / no argument picks what player is looking at.

flags: `0x4000`  
</details>
<details>
<summary><code>npc_combat</code></summary>

Displays text debugging information about the squad and enemy of the selected NPC  (See Overlay Text)
	Arguments:   	{npc_name} / {npc class_name} / no argument picks what player is looking at

flags: `0x4000`  
</details>
<details>
<summary><code>npc_conditions</code></summary>

Displays all the current AI conditions that an NPC has in the overlay text.
	Arguments:   	{npc_name} / {npc class_name} / no argument picks what player is looking at

flags: `0x4000`  
</details>
<details>
<summary><code>npc_create</code></summary>

Creates an NPC of the given type where the player is looking (if the given NPC can actually stand at that location).  
	Arguments:	[npc_class_name] [name of npc (optional) ] [addon type (optional) ] [name of addon (optional) ]

flags: `0x4000`  
</details>
<details>
<summary><code>npc_create_aimed</code></summary>

Creates an NPC aimed away from the player of the given type where the player is looking (if the given NPC can actually stand at that location).  Note that this only works for npc classes that are already in the world.  You can not create an entity that doesn't have an instance in the level.
	Arguments:	{npc_class_name}

flags: `0x4000`  
</details>
<details>
<summary><code>npc_destroy</code></summary>

Removes the given NPC(s) from the universe
Arguments:   	{npc_name} / {npc_class_name} / no argument picks what player is looking at

flags: `0x4000`  
</details>
<details>
<summary><code>npc_destroy_unselected</code></summary>

Removes all NPCs from the universe that aren't currently selected

flags: `0x4000`  
</details>
<details>
<summary><code>npc_enemies</code></summary>

Shows memory of NPC.  Draws an X on top of each memory.
	Eluded entities drawn in blue (don't know where it went)
	Unreachable entities drawn in green (can't get to it)
	Current enemy drawn in red
	Current target entity drawn in magenta
	All other entities drawn in pink
	Arguments:   	{npc_name} / {npc class_name} / no argument picks what player is looking at

flags: `0x4000`  
</details>
<details>
<summary><code>npc_focus</code></summary>

Displays red line to NPC's enemy (if has one) and blue line to NPC's target entity (if has one)
	Arguments:   	{npc_name} / {npc class_name} / no argument picks what player is looking at

flags: `0x4000`  
</details>
<details>
<summary><code>npc_freeze</code></summary>

Selected NPC(s) will freeze in place (or unfreeze). If there are no selected NPCs, uses the NPC under the crosshair.
	Arguments:	-none-

flags: `0x4000`  
</details>
<details>
<summary><code>npc_freeze_unselected</code></summary>

Freeze all NPCs not selected

flags: `0x0`  
</details>
<details>
<summary><code>npc_go</code></summary>

Selected NPC(s) will go to the location that the player is looking (shown with a purple box)
	Arguments:	-none-

flags: `0x4000`  
</details>
<details>
<summary><code>npc_go_random</code></summary>

Sends all selected NPC(s) to a random node.
	Arguments:   	-none-

flags: `0x4000`  
</details>
<details>
<summary><code>npc_heal</code></summary>

Heals the target back to full health

flags: `0x0`  
</details>
<details>
<summary><code>npc_kill</code></summary>

Kills the given NPC(s)
Arguments:   	{npc_name} / {npc_class_name} / no argument picks what player is looking at

flags: `0x4000`  
</details>
<details>
<summary><code>npc_nearest</code></summary>

Draw's a while box around the NPC(s) nearest node
	Arguments:   	{entity_name} / {class_name} / no argument picks what player is looking at  

flags: `0x4000`  
</details>
<details>
<summary><code>npc_relationships</code></summary>

Displays the relationships between this NPC and all others.
	Arguments:   	{entity_name} / {class_name} / no argument picks what player is looking at

flags: `0x4000`  
</details>
<details>
<summary><code>npc_reset</code></summary>

Reloads schedules for all NPC's from their script files
	Arguments:	-none-

flags: `0x4000`  
</details>
<details>
<summary><code>npc_route</code></summary>

Displays the current route of the given NPC as a line on the screen.  Waypoints along the route are drawn as small cyan rectangles.  Line is color coded in the following manner:
	Blue	- path to a node
	Cyan	- detour around an object (triangulation)
	Red	- jump
	Maroon - path to final target position
	Arguments:   	{npc_name} / {npc_class_name} / no argument picks what player is looking at 

flags: `0x4000`  
</details>
<details>
<summary><code>npc_select</code></summary>

Select or deselects the given NPC(s) for later manipulation.  Selected NPC's are shown surrounded by a red translucent box
	Arguments:   	{entity_name} / {class_name} / no argument picks what player is looking at 

flags: `0x4000`  
</details>
<details>
<summary><code>npc_set_freeze</code></summary>

Selected NPC(s) will freeze in place (or unfreeze). If there are no selected NPCs, uses the NPC under the crosshair.
	Arguments:	-none-

flags: `0x4000`  
</details>
<details>
<summary><code>npc_set_freeze_unselected</code></summary>

Freeze all NPCs not selected

flags: `0x0`  
</details>
<details>
<summary><code>npc_squads</code></summary>

Obsolete.  Replaced by npc_combat

flags: `0x4000`  
</details>
<details>
<summary><code>npc_steering</code></summary>

Displays the steering obstructions of the NPC (used to perform local avoidance)
	Arguments:   	{entity_name} / {class_name} / no argument picks what player is looking at

flags: `0x4000`  
</details>
<details>
<summary><code>npc_steering_all</code></summary>

Displays the steering obstructions of all NPCs (used to perform local avoidance)


flags: `0x4000`  
</details>
<details>
<summary><code>npc_task_text</code></summary>

Outputs text debugging information to the console about the all the tasks + break conditions of the selected NPC current schedule
	Arguments:   	{npc_name} / {npc class_name} / no argument picks what player is looking at 

flags: `0x4000`  
</details>
<details>
<summary><code>npc_tasks</code></summary>

Displays detailed text debugging information about the all the tasks of the selected NPC current schedule (See Overlay Text)
	Arguments:   	{npc_name} / {npc class_name} / no argument picks what player is looking at 

flags: `0x4000`  
</details>
<details>
<summary><code>npc_teleport</code></summary>

Selected NPC will teleport to the location that the player is looking (shown with a purple box)
	Arguments:	-none-

flags: `0x4000`  
</details>
<details>
<summary><code>npc_thinknow</code></summary>

Trigger NPC to think

flags: `0x0`  
</details>
<details>
<summary><code>npc_viewcone</code></summary>

Displays the viewcone of the NPC (where they are currently looking and what the extents of there vision is)
	Arguments:   	{entity_name} / {class_name} / no argument picks what player is looking at

flags: `0x4000`  
</details>
<details>
<summary><code>observer_use</code></summary>



flags: `0x400`  
</details>
<details>
<summary><code>parachute</code></summary>

equips parachute

flags: `0x4000`  
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
<summary><code>physics_budget</code></summary>

Times the cost of each active object

flags: `0x0`  
</details>
<details>
<summary><code>physics_constraints</code></summary>

Highlights constraint system graph for an entity

flags: `0x0`  
</details>
<details>
<summary><code>physics_debug_entity</code></summary>

Dumps debug info for an entity

flags: `0x0`  
</details>
<details>
<summary><code>physics_highlight_active</code></summary>

Turns on the absbox for all active physics objects

flags: `0x0`  
</details>
<details>
<summary><code>physics_report_active</code></summary>

Lists all active physics objects

flags: `0x0`  
</details>
<details>
<summary><code>physics_select</code></summary>

Dumps debug info for an entity

flags: `0x0`  
</details>
<details>
<summary><code>picker</code></summary>

Toggles 'picker' mode.  When picker is on, the bounding box, pivot and debugging text is displayed for whatever entity the player is looking at.
	Arguments:	full - enables all debug information

flags: `0x4000`  
</details>
<details>
<summary><code>player_ping</code></summary>

Creates a ping notification where the player is looking.

flags: `0x400`  
</details>
<details>
<summary><code>print_mapgroup_sv</code></summary>

Prints the current mapgroup and the contained maps

flags: `0x80000`  
</details>
<details>
<summary><code>prop_debug</code></summary>

Toggle prop debug mode. If on, props will show colorcoded bounding boxes. Red means ignore all damage. White means respond physically to damage but never break. Green maps health in the range of 100 down to 1.

flags: `0x4000`  
</details>
<details>
<summary><code>prop_dynamic_create</code></summary>

Creates a dynamic prop with a specific .mdl aimed away from where the player is looking.
	Arguments: {.mdl name}

flags: `0x4000`  
</details>
<details>
<summary><code>prop_physics_create</code></summary>

Creates a physics prop with a specific .mdl aimed away from where the player is looking.
	Arguments: {.mdl name}

flags: `0x4000`  
</details>
<details>
<summary><code>replay_death</code></summary>

start hltv replay of last death

flags: `0x4000`  
</details>
<details>
<summary><code>replay_start</code></summary>

Start GOTV replay: replay_start <delay> [<player name or index>]

flags: `0x4000`  
</details>
<details>
<summary><code>replay_stop</code></summary>

stop hltv replay

flags: `0x400`  
</details>
<details>
<summary><code>report_entities</code></summary>

Lists all entities

flags: `0x0`  
</details>
<details>
<summary><code>report_simthinklist</code></summary>

Lists all simulating/thinking entities

flags: `0x0`  
</details>
<details>
<summary><code>report_soundpatch</code></summary>

reports sound patch count

flags: `0x0`  
</details>
<details>
<summary><code>report_touchlinks</code></summary>

Lists all touchlinks

flags: `0x0`  
</details>
<details>
<summary><code>reset_expo</code></summary>

Reset player scores, player controls, team scores, and end the round

flags: `0x4004`  
</details>
<details>
<summary><code>respawn_entities</code></summary>

Respawn all the entities in the map.

flags: `0x4040`  
</details>
<details>
<summary><code>rr_forceconcept</code></summary>

fire a response concept directly at a given character.
USAGE: rr_forceconcept <target> <concept> "criteria1:value1,criteria2:value2,..."
criteria values are optional.


flags: `0x4000`  
</details>
<details>
<summary><code>rr_reloadresponsesystems</code></summary>

Reload all response system scripts.

flags: `0x4000`  
</details>
<details>
<summary><code>say</code></summary>

Display player message

flags: `0x400`  
</details>
<details>
<summary><code>say_team</code></summary>

Display player message to team

flags: `0x400`  
</details>
<details>
<summary><code>scene_flush</code></summary>

Flush all .vcds from the cache and reload from disk.

flags: `0x0`  
</details>
<details>
<summary><code>scene_playvcd</code></summary>

Play the given VCD as an instanced scripted scene.

flags: `0x4000`  
</details>
<details>
<summary><code>script</code></summary>

Run the text as a script

flags: `0x0`  
</details>
<details>
<summary><code>script_debug</code></summary>

Connect the vscript VM to the script debugger

flags: `0x0`  
</details>
<details>
<summary><code>script_dump_all</code></summary>

Dump the state of the VM to the console

flags: `0x0`  
</details>
<details>
<summary><code>script_execute</code></summary>

Run a vscript file

flags: `0x0`  
</details>
<details>
<summary><code>script_help</code></summary>

Output help for script functions, optionally with a search string

flags: `0x0`  
</details>
<details>
<summary><code>script_reload_code</code></summary>

Execute a vscript file, replacing existing functions with the functions in the run script

flags: `0x0`  
</details>
<details>
<summary><code>script_reload_entity_code</code></summary>

Execute all of this entity's VScripts, replacing existing functions with the functions in the run scripts

flags: `0x0`  
</details>
<details>
<summary><code>script_reload_think</code></summary>

Execute an activation script, replacing existing functions with the functions in the run script

flags: `0x0`  
</details>
<details>
<summary><code>send_round_backup_file_list</code></summary>



flags: `0x80414`  
</details>
<details>
<summary><code>server_game_time</code></summary>

Gives the game time in seconds (server's curtime)

flags: `0x0`  
</details>
<details>
<summary><code>setang</code></summary>

Snap player eyes to specified pitch yaw <roll:optional> (must have sv_cheats).

flags: `0x4000`  
</details>
<details>
<summary><code>setang_exact</code></summary>

Snap player eyes and orientation to specified pitch yaw <roll:optional> (must have sv_cheats).

flags: `0x4000`  
</details>
<details>
<summary><code>setmodel</code></summary>

Changes's player's model

flags: `0x4000`  
</details>
<details>
<summary><code>setpos</code></summary>

Move player to specified origin (must have sv_cheats).

flags: `0x4000`  
</details>
<details>
<summary><code>setpos_exact</code></summary>

Move player to an exact specified origin (must have sv_cheats).

flags: `0x4000`  
</details>
<details>
<summary><code>setpos_player</code></summary>

Move specified player to specified origin (must have sv_cheats).

flags: `0x4000`  
</details>
<details>
<summary><code>shake</code></summary>

Shake the screen.

flags: `0x4000`  
</details>
<details>
<summary><code>showtriggers_toggle</code></summary>

Toggle show triggers

flags: `0x4000`  
</details>
<details>
<summary><code>skip_next_map</code></summary>

Skips the next map in the map rotation for the server.

flags: `0x0`  
</details>
<details>
<summary><code>soundscape_flush</code></summary>

Flushes the server & client side soundscapes

flags: `0x0`  
</details>
<details>
<summary><code>surfaceprop</code></summary>

Reports the surface properties at the cursor

flags: `0x4000`  
</details>
<details>
<summary><code>survival_check_num_possible_final_zone</code></summary>

print out a number of all possible final zone

flags: `0x0`  
</details>
<details>
<summary><code>sv_benchmark_force_start</code></summary>

Force start the benchmark. This is only for debugging. It's better to set sv_benchmark to 1 and restart the level.

flags: `0x0`  
</details>
<details>
<summary><code>sv_clearhinthistory</code></summary>

Clear memory of server side hints displayed to the player.

flags: `0x0`  
</details>
<details>
<summary><code>sv_cs_dump_econ_item_stringtable</code></summary>

sv_cs_dump_econ_item_stringtable

flags: `0x0`  
</details>
<details>
<summary><code>sv_dz_paradrop</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>sv_dz_reset_danger_zone</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>sv_findsoundname</code></summary>

Find sound names which reference the specified wave files.

flags: `0x2`  
</details>
<details>
<summary><code>sv_game_mode_convars</code></summary>

Display the values of the convars for the current game_mode.

flags: `0x0`  
</details>
<details>
<summary><code>sv_load_forced_client_names_file</code></summary>

Loads a file containing SteamID64 names for clients

flags: `0x80004`  
</details>
<details>
<summary><code>sv_load_random_client_names_file</code></summary>

Loads a file containing random name words for clients

flags: `0x80004`  
</details>
<details>
<summary><code>sv_querycache_stats</code></summary>

Display status of the query cache (client only)

flags: `0x0`  
</details>
<details>
<summary><code>sv_soundemitter_filecheck</code></summary>

Report missing wave files for sounds and game_sounds files.

flags: `0x2`  
</details>
<details>
<summary><code>sv_soundemitter_flush</code></summary>

Flushes the sounds.txt system (server only)

flags: `0x2`  
</details>
<details>
<summary><code>sv_soundemitter_reload</code></summary>

Flushes the sounds.txt system

flags: `0x0`  
</details>
<details>
<summary><code>sv_soundemitter_spew</code></summary>

Print details about a sound.

flags: `0x2`  
</details>
<details>
<summary><code>sv_soundscape_printdebuginfo</code></summary>

print soundscapes

flags: `0x4000`  
</details>
<details>
<summary><code>test_dispatcheffect</code></summary>

Test a clientside dispatch effect.
	Usage: test_dispatcheffect <effect name> <distance away> <flags> <magnitude> <scale>
	Defaults are: <distance 1024> <flags 0> <magnitude 0> <scale 0>


flags: `0x4000`  
</details>
<details>
<summary><code>test_entity_blocker</code></summary>

Test command that drops an entity blocker out in front of the player.

flags: `0x4000`  
</details>
<details>
<summary><code>test_outtro_stats</code></summary>



flags: `0x4000`  
</details>
<details>
<summary><code>timeleft</code></summary>

prints the time remaining in the match

flags: `0x400`  
</details>
<details>
<summary><code>timeout_ct_start</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>timeout_terrorist_start</code></summary>



flags: `0x0`  
</details>
<details>
<summary><code>traceattack</code></summary>

traceattack damage hitgroup

flags: `0x0`  
</details>
<details>
<summary><code>tv_time_remaining</code></summary>

Print remaining tv broadcast time

flags: `0x80404`  
</details>
<details>
<summary><code>tweak_ammo_impulses</code></summary>

Allow real-time tweaking of the ammo impulse values.

flags: `0x4004`  
</details>
<details>
<summary><code>use</code></summary>

Use a particular weapon	
Arguments: <weapon_name>

flags: `0x400`  
</details>
<details>
<summary><code>vehicle_flushscript</code></summary>

Flush and reload all vehicle scripts

flags: `0x0`  
</details>
<details>
<summary><code>voxeltree_box</code></summary>

View entities in the voxel-tree inside box <Vector(min), Vector(max)>.

flags: `0x4000`  
</details>
<details>
<summary><code>voxeltree_playerview</code></summary>

View entities in the voxel-tree at the player position.

flags: `0x4000`  
</details>
<details>
<summary><code>voxeltree_sphere</code></summary>

View entities in the voxel-tree inside sphere <Vector(center), float(radius)>.

flags: `0x4000`  
</details>
<details>
<summary><code>voxeltree_view</code></summary>

View entities in the voxel-tree.

flags: `0x4000`  
</details>
<details>
<summary><code>wc_air_edit_further</code></summary>

When in WC edit mode and editing air nodes,  moves position of air node crosshair and placement location further away from player

flags: `0x4000`  
</details>
<details>
<summary><code>wc_air_edit_nearer</code></summary>

When in WC edit mode and editing air nodes,  moves position of air node crosshair and placement location nearer to from player

flags: `0x4000`  
</details>
<details>
<summary><code>wc_air_node_edit</code></summary>

When in WC edit mode, toggles laying down or air nodes instead of ground nodes

flags: `0x4000`  
</details>
<details>
<summary><code>wc_create</code></summary>

When in WC edit mode, creates a node where the player is looking if a node is allowed at that location for the currently selected hull size (see ai_next_hull)

flags: `0x4000`  
</details>
<details>
<summary><code>wc_destroy</code></summary>

When in WC edit mode, destroys the node that the player is nearest to looking at.  (The node will be highlighted by a red box).

flags: `0x4000`  
</details>
<details>
<summary><code>wc_destroy_undo</code></summary>

When in WC edit mode restores the last deleted node

flags: `0x4000`  
</details>
<details>
<summary><code>wc_link_edit</code></summary>



flags: `0x4000`  
</details>
<details>
<summary><code>wipe_nav_attributes</code></summary>

Clear all nav attributes of selected area.

flags: `0x4000`  
</details>
<details>
<summary><code>workshop_start_map</code></summary>

Sets the first map to load once a workshop collection been hosted. Takes the file id of desired start map as a parameter.

flags: `0x0`  
</details>

#### Addresses

```
server.dll!0x009b4fdc ConCommand CreatePredictionError
server.dll!0x009a90c4 ConCommand Test_EHandle
server.dll!0x009a914c ConCommand Test_InitRandomEntitySpawner
server.dll!0x009a9104 ConCommand Test_ProxyToggle_EnableProxy
server.dll!0x009a9128 ConCommand Test_ProxyToggle_SetValue
server.dll!0x009a9194 ConCommand Test_RandomizeInPVS
server.dll!0x009a91b8 ConCommand Test_RemoveAllRandomEntities
server.dll!0x009a9170 ConCommand Test_SpawnRandomEntities
server.dll!0x009a3700 ConCommand _resetgamestats
server.dll!0x0099a06c ConCommand ai_clear_bad_links
server.dll!0x0099a9f0 ConCommand ai_debug_node_connect
server.dll!0x009998f8 ConCommand ai_disable
server.dll!0x0099a228 ConCommand ai_drop_hint
server.dll!0x0099a204 ConCommand ai_dump_hints
server.dll!0x00999aa8 ConCommand ai_hull
server.dll!0x00999a84 ConCommand ai_next_hull
server.dll!0x00999acc ConCommand ai_nodes
server.dll!0x00999a60 ConCommand ai_resume
server.dll!0x0099a750 ConCommand ai_set_move_height_epsilon
server.dll!0x0099991c ConCommand ai_setenabled
server.dll!0x00999988 ConCommand ai_show_connect
server.dll!0x009999f4 ConCommand ai_show_connect_crawl
server.dll!0x009999d0 ConCommand ai_show_connect_fly
server.dll!0x009999ac ConCommand ai_show_connect_jump
server.dll!0x00999b38 ConCommand ai_show_graph_connect
server.dll!0x00999a18 ConCommand ai_show_grid
server.dll!0x00999940 ConCommand ai_show_hints
server.dll!0x00999964 ConCommand ai_show_hull
server.dll!0x00999af0 ConCommand ai_show_node
server.dll!0x00999b14 ConCommand ai_show_visibility
server.dll!0x00999a3c ConCommand ai_step
server.dll!0x0099a090 ConCommand ai_test_los
server.dll!0x0099a0b4 ConCommand ainet_generate_report
server.dll!0x0099a0d8 ConCommand ainet_generate_report_only
server.dll!0x009a57c0 ConCommand air_density
server.dll!0x009b9bfc ConCommand bot_add
server.dll!0x009b9c44 ConCommand bot_add_ct
server.dll!0x009b9c20 ConCommand bot_add_t
server.dll!0x009b9d1c ConCommand bot_all_weapons
server.dll!0x009b52e0 ConCommand bot_control_next_all_teams
server.dll!0x009b9d40 ConCommand bot_goto_mark
server.dll!0x009b9d64 ConCommand bot_goto_selected
server.dll!0x009b9c8c ConCommand bot_kick
server.dll!0x009b9c68 ConCommand bot_kill
server.dll!0x009b9cb0 ConCommand bot_knives_only
server.dll!0x009b9cd4 ConCommand bot_pistols_only
server.dll!0x009b9bd8 ConCommand bot_place
server.dll!0x009b9cf8 ConCommand bot_snipers_only
server.dll!0x0099cc08 ConCommand buddha
server.dll!0x009b522c ConCommand buyrandom
server.dll!0x009a9e84 ConCommand callvote
server.dll!0x0099cb0c ConCommand cast_hull
server.dll!0x0099cae8 ConCommand cast_ray
server.dll!0x009a6a7c ConCommand ch_createairboat
server.dll!0x009a6a58 ConCommand ch_createjeep
server.dll!0x009a1704 ConCommand cl_csm_server_status
server.dll!0x009bc058 ConCommand clear_bombs
server.dll!0x009a4f38 ConCommand clear_debug_overlays
server.dll!0x009a94f4 ConCommand collision_test
server.dll!0x0099d0a0 ConCommand commentary_cvarsnotchanging
server.dll!0x0099d0c4 ConCommand commentary_finishnode
server.dll!0x009a1820 ConCommand create_flashlight
server.dll!0x009a18e4 ConCommand creditsdone
server.dll!0x009b5188 ConCommand cs_make_vip
server.dll!0x009a172c ConCommand dbghist_addline
server.dll!0x009a1750 ConCommand dbghist_dump
server.dll!0x009bb110 ConCommand dm_reset_spawns
server.dll!0x0099cb54 ConCommand drawcross
server.dll!0x0099cb30 ConCommand drawline
server.dll!0x009b842c ConCommand ds_get_newest_subscribed_files
server.dll!0x009a93c4 ConCommand dump_entity_sizes
server.dll!0x009a3b00 ConCommand dump_globals
server.dll!0x009a93a0 ConCommand dumpentityfactories
server.dll!0x0099ca24 ConCommand dumpeventqueue
server.dll!0x009a3724 ConCommand dumpgamestringtable
server.dll!0x009bb734 ConCommand dz_clearteams
server.dll!0x009bb710 ConCommand dz_jointeam
server.dll!0x009bb6ec ConCommand dz_shuffle_teams
server.dll!0x009bb808 ConCommand dz_spawnselect_choose_hex
server.dll!0x009b1378 ConCommand endround
server.dll!0x0099bcf0 ConCommand ent_absbox
server.dll!0x0099bd38 ConCommand ent_attachments
server.dll!0x0099bfa4 ConCommand ent_autoaim
server.dll!0x0099bccc ConCommand ent_bbox
server.dll!0x0099be84 ConCommand ent_cancelpendingentfires
server.dll!0x0099c020 ConCommand ent_create
server.dll!0x0099be34 ConCommand ent_dump
server.dll!0x0099bea8 ConCommand ent_info
server.dll!0x009bfbf4 ConCommand ent_keyvalue
server.dll!0x009b139c ConCommand ent_list_report
server.dll!0x0099becc ConCommand ent_messages
server.dll!0x0099bc60 ConCommand ent_name
server.dll!0x0099c068 ConCommand ent_orient
server.dll!0x0099bef0 ConCommand ent_pause
server.dll!0x0099bf38 ConCommand ent_pivot
server.dll!0x0099bd14 ConCommand ent_rbox
server.dll!0x0099bd80 ConCommand ent_remove
server.dll!0x0099bda4 ConCommand ent_remove_all
server.dll!0x009a73bc ConCommand ent_rotate
server.dll!0x0099bca8 ConCommand ent_script_dump
server.dll!0x0099cdf0 ConCommand ent_setang
server.dll!0x0099bdc8 ConCommand ent_setname
server.dll!0x0099cdcc ConCommand ent_setpos
server.dll!0x0099bf80 ConCommand ent_show_response_criteria
server.dll!0x0099bf5c ConCommand ent_step
server.dll!0x0099c044 ConCommand ent_teleport
server.dll!0x0099bc84 ConCommand ent_text
server.dll!0x0099bd5c ConCommand ent_viewoffset
server.dll!0x0099cf34 ConCommand exojump
server.dll!0x0099cb9c ConCommand explode
server.dll!0x0099cbe4 ConCommand explodevector
server.dll!0x009a18c0 ConCommand fadein
server.dll!0x009a189c ConCommand fadeout
server.dll!0x0099bdec ConCommand find_ent
server.dll!0x0099be10 ConCommand find_ent_index
server.dll!0x0099be58 ConCommand firetarget
server.dll!0x009a1e74 ConCommand foundry_engine_get_mouse_control
server.dll!0x009a1e98 ConCommand foundry_engine_release_mouse_control
server.dll!0x009a1ebc ConCommand foundry_select_entity
server.dll!0x009a1e50 ConCommand foundry_sync_hammer_view
server.dll!0x009a1e2c ConCommand foundry_update_entity
server.dll!0x0099cc74 ConCommand give
server.dll!0x009a6770 ConCommand givecurrentammo
server.dll!0x009a3dc8 ConCommand global_set
server.dll!0x0099cd84 ConCommand god
server.dll!0x0099cda8 ConCommand gods
server.dll!0x0099cf7c ConCommand groundlist
server.dll!0x009aa148 ConCommand hammer_update_entity
server.dll!0x009aa16c ConCommand hammer_update_safe_entities
server.dll!0x009b8408 ConCommand host_workshop_collection
server.dll!0x009b83e4 ConCommand host_workshop_map
server.dll!0x0099ceec ConCommand hurtme
server.dll!0x009b3eb8 ConCommand itemtimedata_dump_active
server.dll!0x009b3edc ConCommand itemtimedata_dump_total
server.dll!0x009b3f00 ConCommand itemtimedata_print_and_reset
server.dll!0x009a9440 ConCommand kdtree_test
server.dll!0x0099cb78 ConCommand kill
server.dll!0x0099cbc0 ConCommand killvector
server.dll!0x009a7a84 ConCommand listRecentNPCSpeech
server.dll!0x009a9e60 ConCommand listissues
server.dll!0x009bd7b0 ConCommand load_master_item_schema
server.dll!0x009a7ff8 ConCommand logaddress_add_http
server.dll!0x009a801c ConCommand logaddress_delall_http
server.dll!0x009a8040 ConCommand logaddress_list_http
server.dll!0x009b3718 ConCommand map_setbombradius
server.dll!0x009b36f4 ConCommand map_showbombradius
server.dll!0x009b36d0 ConCommand map_showspawnpoints
server.dll!0x009ac224 ConCommand mp_backup_restore_list_files
server.dll!0x009ac2a0 ConCommand mp_backup_restore_load_file
server.dll!0x009a6acc ConCommand mp_disable_autokick
server.dll!0x009b23c4 ConCommand mp_dump_timers
server.dll!0x009a8ff0 ConCommand mp_forcerespawnplayers
server.dll!0x009a8fcc ConCommand mp_forcewin
server.dll!0x009b2430 ConCommand mp_pause_match
server.dll!0x009b23e8 ConCommand mp_scrambleteams
server.dll!0x009b240c ConCommand mp_swapteams
server.dll!0x009a8fa8 ConCommand mp_switchteams
server.dll!0x009a9014 ConCommand mp_tournament_restart
server.dll!0x009b2454 ConCommand mp_unpause_match
server.dll!0x009adbf4 ConCommand mp_warmup_end
server.dll!0x009adbd0 ConCommand mp_warmup_start
server.dll!0x009be864 ConCommand nav_add_to_selected_set
server.dll!0x009be888 ConCommand nav_add_to_selected_set_by_id
server.dll!0x009bf23c ConCommand nav_analyze
server.dll!0x009bf260 ConCommand nav_analyze_scripted
server.dll!0x009bee94 ConCommand nav_avoid
server.dll!0x009bece4 ConCommand nav_begin_area
server.dll!0x009beaa4 ConCommand nav_begin_deselecting
server.dll!0x009be9a8 ConCommand nav_begin_drag_deselecting
server.dll!0x009be960 ConCommand nav_begin_drag_selecting
server.dll!0x009be918 ConCommand nav_begin_selecting
server.dll!0x009beb34 ConCommand nav_begin_shift_xy
server.dll!0x009bf2f0 ConCommand nav_build_ladder
server.dll!0x009b9d88 ConCommand nav_check_connectivity
server.dll!0x009be234 ConCommand nav_check_file_consistency
server.dll!0x009bdda0 ConCommand nav_check_floor
server.dll!0x009be530 ConCommand nav_check_stairs
server.dll!0x009bf648 ConCommand nav_chop_selected
server.dll!0x009bf338 ConCommand nav_clear_attribute
server.dll!0x009be8f4 ConCommand nav_clear_selected_set
server.dll!0x009bf2a8 ConCommand nav_clear_walkable_marks
server.dll!0x009bf2cc ConCommand nav_compress_id
server.dll!0x009bed2c ConCommand nav_connect
server.dll!0x009bf164 ConCommand nav_corner_lower
server.dll!0x009bf188 ConCommand nav_corner_place_on_ground
server.dll!0x009bf140 ConCommand nav_corner_raise
server.dll!0x009bf11c ConCommand nav_corner_select
server.dll!0x009bed98 ConCommand nav_crouch
server.dll!0x009be78c ConCommand nav_delete
server.dll!0x009be7b0 ConCommand nav_delete_marked
server.dll!0x009bed50 ConCommand nav_disconnect
server.dll!0x009beedc ConCommand nav_dont_hide
server.dll!0x009bed08 ConCommand nav_end_area
server.dll!0x009beac8 ConCommand nav_end_deselecting
server.dll!0x009be9cc ConCommand nav_end_drag_deselecting
server.dll!0x009be984 ConCommand nav_end_drag_selecting
server.dll!0x009be93c ConCommand nav_end_selecting
server.dll!0x009beb58 ConCommand nav_end_shift_xy
server.dll!0x009be7d4 ConCommand nav_flood_select
server.dll!0x009be59c ConCommand nav_gen_cliffs_approx
server.dll!0x009bf1f4 ConCommand nav_generate
server.dll!0x009bf218 ConCommand nav_generate_incremental
server.dll!0x009bede0 ConCommand nav_jump
server.dll!0x009bf1d0 ConCommand nav_ladder_flip
server.dll!0x009bef90 ConCommand nav_load
server.dll!0x009bea14 ConCommand nav_lower_drag_volume_max
server.dll!0x009bea5c ConCommand nav_lower_drag_volume_min
server.dll!0x009bec54 ConCommand nav_make_sniper_spots
server.dll!0x009bec9c ConCommand nav_mark
server.dll!0x009bf35c ConCommand nav_mark_attribute
server.dll!0x009bf0f8 ConCommand nav_mark_unnamed
server.dll!0x009bf284 ConCommand nav_mark_walkable
server.dll!0x009bec78 ConCommand nav_merge
server.dll!0x009be5e4 ConCommand nav_merge_mesh
server.dll!0x009bef24 ConCommand nav_no_hostages
server.dll!0x009bee04 ConCommand nav_no_jump
server.dll!0x009bf068 ConCommand nav_place_floodfill
server.dll!0x009beffc ConCommand nav_place_list
server.dll!0x009bf0b0 ConCommand nav_place_pick
server.dll!0x009befd8 ConCommand nav_place_replace
server.dll!0x009bf08c ConCommand nav_place_set
server.dll!0x009bedbc ConCommand nav_precise
server.dll!0x009be9f0 ConCommand nav_raise_drag_volume_max
server.dll!0x009bea38 ConCommand nav_raise_drag_volume_min
server.dll!0x009be840 ConCommand nav_recall_selected_set
server.dll!0x009be8ac ConCommand nav_remove_from_selected_set
server.dll!0x009be768 ConCommand nav_remove_jump_areas
server.dll!0x009bee70 ConCommand nav_run
server.dll!0x009bef6c ConCommand nav_save
server.dll!0x009be5c0 ConCommand nav_save_selected
server.dll!0x009beba0 ConCommand nav_select_blocked_areas
server.dll!0x009bebe8 ConCommand nav_select_damaging_areas
server.dll!0x009beb10 ConCommand nav_select_half_space
server.dll!0x009beb7c ConCommand nav_select_invalid_areas
server.dll!0x009bebc4 ConCommand nav_select_obstructed_areas
server.dll!0x009bddc4 ConCommand nav_select_overlapping
server.dll!0x009be198 ConCommand nav_select_radius
server.dll!0x009bec0c ConCommand nav_select_stairs
server.dll!0x009bf044 ConCommand nav_set_place_mode
server.dll!0x009be150 ConCommand nav_shift
server.dll!0x009bf66c ConCommand nav_simplify_selected
server.dll!0x009bed74 ConCommand nav_splice
server.dll!0x009bec30 ConCommand nav_split
server.dll!0x009bef00 ConCommand nav_stand
server.dll!0x009bee28 ConCommand nav_stop
server.dll!0x009be81c ConCommand nav_store_selected_set
server.dll!0x009bef48 ConCommand nav_strip
server.dll!0x009be578 ConCommand nav_subdivide
server.dll!0x009be554 ConCommand nav_test_stairs
server.dll!0x009beaec ConCommand nav_toggle_deselecting
server.dll!0x009be8d0 ConCommand nav_toggle_in_selected_set
server.dll!0x009bf020 ConCommand nav_toggle_place_mode
server.dll!0x009bf0d4 ConCommand nav_toggle_place_painting
server.dll!0x009be7f8 ConCommand nav_toggle_selected_set
server.dll!0x009bea80 ConCommand nav_toggle_selecting
server.dll!0x009beeb8 ConCommand nav_transient
server.dll!0x009becc0 ConCommand nav_unmark
server.dll!0x009bdd7c ConCommand nav_update_blocked
server.dll!0x009bdd58 ConCommand nav_update_lighting
server.dll!0x009befb4 ConCommand nav_use_place
server.dll!0x009bee4c ConCommand nav_walk
server.dll!0x009bf1ac ConCommand nav_warp_to_mark
server.dll!0x009be174 ConCommand nav_world_center
server.dll!0x0099cd60 ConCommand noclip
server.dll!0x0099cec8 ConCommand notarget
server.dll!0x0099a048 ConCommand npc_ammo_deplete
server.dll!0x00999b5c ConCommand npc_bipass
server.dll!0x00999ee0 ConCommand npc_combat
server.dll!0x00999f70 ConCommand npc_conditions
server.dll!0x00999c68 ConCommand npc_create
server.dll!0x00999c8c ConCommand npc_create_aimed
server.dll!0x00999b80 ConCommand npc_destroy
server.dll!0x00999cb0 ConCommand npc_destroy_unselected
server.dll!0x00999bc8 ConCommand npc_enemies
server.dll!0x00999bec ConCommand npc_focus
server.dll!0x00999cd4 ConCommand npc_freeze
server.dll!0x00999d1c ConCommand npc_freeze_unselected
server.dll!0x00999e08 ConCommand npc_go
server.dll!0x00999e2c ConCommand npc_go_random
server.dll!0x0099a024 ConCommand npc_heal
server.dll!0x00999ba4 ConCommand npc_kill
server.dll!0x00999e74 ConCommand npc_nearest
server.dll!0x00999fb8 ConCommand npc_relationships
server.dll!0x00999e50 ConCommand npc_reset
server.dll!0x00999e98 ConCommand npc_route
server.dll!0x00999ebc ConCommand npc_select
server.dll!0x00999cf8 ConCommand npc_set_freeze
server.dll!0x00999d40 ConCommand npc_set_freeze_unselected
server.dll!0x00999f04 ConCommand npc_squads
server.dll!0x00999fdc ConCommand npc_steering
server.dll!0x0099a000 ConCommand npc_steering_all
server.dll!0x00999f4c ConCommand npc_task_text
server.dll!0x00999f28 ConCommand npc_tasks
server.dll!0x00999d88 ConCommand npc_teleport
server.dll!0x00999d64 ConCommand npc_thinknow
server.dll!0x00999f94 ConCommand npc_viewcone
server.dll!0x009b52bc ConCommand observer_use
server.dll!0x0099cf10 ConCommand parachute
server.dll!0x009a5338 ConCommand particle_test_start
server.dll!0x009a535c ConCommand particle_test_stop
server.dll!0x009a5690 ConCommand physics_budget
server.dll!0x009a5624 ConCommand physics_constraints
server.dll!0x009a5648 ConCommand physics_debug_entity
server.dll!0x009a55b8 ConCommand physics_highlight_active
server.dll!0x009a55dc ConCommand physics_report_active
server.dll!0x009a566c ConCommand physics_select
server.dll!0x0099bf14 ConCommand picker
server.dll!0x0099cf58 ConCommand player_ping
server.dll!0x009ab828 ConCommand print_mapgroup_sv
server.dll!0x009a7250 ConCommand prop_debug
server.dll!0x009a7374 ConCommand prop_dynamic_create
server.dll!0x009a7398 ConCommand prop_physics_create
server.dll!0x009b5274 ConCommand replay_death
server.dll!0x009b5250 ConCommand replay_start
server.dll!0x009b5298 ConCommand replay_stop
server.dll!0x009a1600 ConCommand report_entities
server.dll!0x009a1648 ConCommand report_simthinklist
server.dll!0x009a8568 ConCommand report_soundpatch
server.dll!0x009a1624 ConCommand report_touchlinks
server.dll!0x009b13e4 ConCommand reset_expo
server.dll!0x009a15dc ConCommand respawn_entities
server.dll!0x0099a1b0 ConCommand rr_forceconcept
server.dll!0x0099acec ConCommand rr_reloadresponsesystems
server.dll!0x0099cc2c ConCommand say
server.dll!0x0099cc50 ConCommand say_team
server.dll!0x009a7aa8 ConCommand scene_flush
server.dll!0x009a7a60 ConCommand scene_playvcd
server.dll!0x009a9f70 ConCommand script
server.dll!0x009a9fb8 ConCommand script_debug
server.dll!0x009aa000 ConCommand script_dump_all
server.dll!0x009a9f94 ConCommand script_execute
server.dll!0x009a9fdc ConCommand script_help
server.dll!0x009a9f00 ConCommand script_reload_code
server.dll!0x009a9f24 ConCommand script_reload_entity_code
server.dll!0x009a9f48 ConCommand script_reload_think
server.dll!0x009ac200 ConCommand send_round_backup_file_list
server.dll!0x009a3b24 ConCommand server_game_time
server.dll!0x0099ce5c ConCommand setang
server.dll!0x0099cea4 ConCommand setang_exact
server.dll!0x0099cc98 ConCommand setmodel
server.dll!0x0099ce14 ConCommand setpos
server.dll!0x0099ce80 ConCommand setpos_exact
server.dll!0x0099ce38 ConCommand setpos_player
server.dll!0x009a192c ConCommand shake
server.dll!0x009a9270 ConCommand showtriggers_toggle
server.dll!0x009a4bf8 ConCommand skip_next_map
server.dll!0x009a8648 ConCommand soundscape_flush
server.dll!0x009a5600 ConCommand surfaceprop
server.dll!0x0099d3e0 ConCommand survival_check_num_possible_final_zone
server.dll!0x009a8170 ConCommand sv_benchmark_force_start
server.dll!0x009a3bd4 ConCommand sv_clearhinthistory
server.dll!0x009bd500 ConCommand sv_cs_dump_econ_item_stringtable
server.dll!0x009bb6a0 ConCommand sv_dz_paradrop
server.dll!0x009bb67c ConCommand sv_dz_reset_danger_zone
server.dll!0x009a84c8 ConCommand sv_findsoundname
server.dll!0x009b89e4 ConCommand sv_game_mode_convars
server.dll!0x009aa5b4 ConCommand sv_load_forced_client_names_file
server.dll!0x009aa634 ConCommand sv_load_random_client_names_file
server.dll!0x009a76f8 ConCommand sv_querycache_stats
server.dll!0x009a8480 ConCommand sv_soundemitter_filecheck
server.dll!0x009a845c ConCommand sv_soundemitter_flush
server.dll!0x009a84a4 ConCommand sv_soundemitter_reload
server.dll!0x009a84ec ConCommand sv_soundemitter_spew
server.dll!0x009a866c ConCommand sv_soundscape_printdebuginfo
server.dll!0x0099ccbc ConCommand test_dispatcheffect
server.dll!0x0099d4f8 ConCommand test_entity_blocker
server.dll!0x009a1908 ConCommand test_outtro_stats
server.dll!0x009b5208 ConCommand timeleft
server.dll!0x009adbac ConCommand timeout_ct_start
server.dll!0x009adb88 ConCommand timeout_terrorist_start
server.dll!0x009b5108 ConCommand traceattack
server.dll!0x009b13c0 ConCommand tv_time_remaining
server.dll!0x009b1408 ConCommand tweak_ammo_impulses
server.dll!0x0099cce0 ConCommand use
server.dll!0x009a9780 ConCommand vehicle_flushscript
server.dll!0x009a94ac ConCommand voxeltree_box
server.dll!0x009a9488 ConCommand voxeltree_playerview
server.dll!0x009a94d0 ConCommand voxeltree_sphere
server.dll!0x009a9464 ConCommand voxeltree_view
server.dll!0x009aa0dc ConCommand wc_air_edit_further
server.dll!0x009aa100 ConCommand wc_air_edit_nearer
server.dll!0x009aa0b8 ConCommand wc_air_node_edit
server.dll!0x009aa04c ConCommand wc_create
server.dll!0x009aa070 ConCommand wc_destroy
server.dll!0x009aa094 ConCommand wc_destroy_undo
server.dll!0x009aa124 ConCommand wc_link_edit
server.dll!0x009bf314 ConCommand wipe_nav_attributes
server.dll!0x009b83c0 ConCommand workshop_start_map
```

## Other

### Interfaces

```
inputsystem.dll!0x0002ed28 InputStackSystemVersion001
inputsystem.dll!0x00031188 InputSystemVersion001
```

### Interfaces

```
materialsystemd.dll!0x000bae10 COLORCORRECTION_VERSION_1
materialsystemd.dll!0x000bcafc ShaderSystem002
materialsystemd.dll!0x000df420 VMaterialSystem080
materialsystemd.dll!0x000bc468 VMaterialSystemConfig004
materialsystemd.dll!0x000bc960 VMaterialSystemStub001
```

### Interfaces

```
shaderapidx9.dll!0x000a5a90 DebugTextureInfo001
shaderapidx9.dll!0x00089488 MaterialSystemHardwareConfig013
shaderapidx9.dll!0x000a5a8c ShaderApi029
shaderapidx9.dll!0x000a5830 ShaderDevice001
shaderapidx9.dll!0x0008a7e8 ShaderDeviceMgr001
shaderapidx9.dll!0x0008a9a8 ShaderShadow010
shaderapidx9.dll!0x00089304 VBAllocTracker001
```

### Interfaces

```
vgui2.dll!0x0006c314 SchemeSurface001
vgui2.dll!0x0006afa8 VGUI_Input005
vgui2.dll!0x0006afa8 VGUI_InputInternal001
vgui2.dll!0x000608c4 VGUI_Panel009
vgui2.dll!0x0006c2d0 VGUI_Scheme010
vgui2.dll!0x0006c310 VGUI_Surface031
vgui2.dll!0x00060670 VGUI_System010
vgui2.dll!0x0006c498 VGUI_ivgui008
```

### Interfaces

```
vguimatsurface.dll!0x000f47c4 SchemeSurface001
vguimatsurface.dll!0x000f47c0 VGUI_Surface031
```

### Interfaces

```
vphysics.dll!0x0010aa20 VPhysics031
vphysics.dll!0x0010aa5c VPhysicsCollision007
vphysics.dll!0x00116018 VPhysicsSurfaceProps001
```

### Interfaces

```
vstdlib.dll!0x00038a10 EventSystem001
vstdlib.dll!0x0003c270 VEngineCvar007
vstdlib.dll!0x0003e608 VProcessUtils002
```

### Interfaces

```
matchmaking.dll!0x000831e0 MATCHFRAMEWORK_001
matchmaking.dll!0x00096378 VENGINE_GAMETYPES_VERSION002
```

