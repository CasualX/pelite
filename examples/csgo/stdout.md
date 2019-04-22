# Engine

## Interfaces

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

## ConVars

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

### Addresses

```
engine.dll!0x519be8 ConVar adsp_debug
engine.dll!0x5a0b28 ConVar budget_averages_window
engine.dll!0x5a0cf8 ConVar budget_background_alpha
engine.dll!0x5a0ad0 ConVar budget_bargraph_background_alpha
engine.dll!0x5a0bd8 ConVar budget_history_numsamplesvisible
engine.dll!0x5a0a20 ConVar budget_peaks_window
engine.dll!0x5a0b80 ConVar budget_show_averages
engine.dll!0x5a0c30 ConVar budget_show_history
engine.dll!0x5a0a78 ConVar budget_show_peaks
engine.dll!0x5954e0 ConVar bugreporter_uploadasync
engine.dll!0x595320 ConVar bugreporter_username
engine.dll!0x591b10 ConVar cl_allowdownload
engine.dll!0x59e568 ConVar cl_allowupload
engine.dll!0x5957f8 ConVar cl_clock_correction
engine.dll!0x595640 ConVar cl_clock_correction_adjustment_max_amount
engine.dll!0x595698 ConVar cl_clock_correction_adjustment_max_offset
engine.dll!0x5957a0 ConVar cl_clock_correction_adjustment_min_offset
engine.dll!0x5956f0 ConVar cl_clock_correction_force_server_tick
engine.dll!0x595748 ConVar cl_clock_showdebuginfo
engine.dll!0x595590 ConVar cl_clockdrift_max_ms
engine.dll!0x5955e8 ConVar cl_clockdrift_max_ms_threadmode
engine.dll!0x589c08 ConVar cl_color
engine.dll!0x58c0f0 ConVar cl_debug_ugc_downloads
engine.dll!0x589c60 ConVar cl_decryptdata_key
engine.dll!0x589cb8 ConVar cl_decryptdata_key_pub
engine.dll!0x58c098 ConVar cl_download_demoplayer
engine.dll!0x58c040 ConVar cl_downloadfilter
engine.dll!0x58b0c0 ConVar cl_entityreport
engine.dll!0x58b220 ConVar cl_flushentitypacket
engine.dll!0x58bfe8 ConVar cl_forcepreload
engine.dll!0x589d10 ConVar cl_hideserverip
engine.dll!0x58b350 ConVar cl_ignorepackets
engine.dll!0x58ac98 ConVar cl_interpolate
engine.dll!0x589b58 ConVar cl_interpolate
engine.dll!0x5899f8 ConVar cl_resend
engine.dll!0x589a50 ConVar cl_resend_timeout
engine.dll!0x58bba8 ConVar cl_showevents
engine.dll!0x58bc00 ConVar cl_showpluginmessages2
engine.dll!0x59e978 ConVar cl_skipslowpath
engine.dll!0x58bf90 ConVar cl_timeout
engine.dll!0x59d678 ConVar clientport
engine.dll!0x597fe8 ConVar closecaption
engine.dll!0x58c4d8 ConVar con_enable
engine.dll!0x58c530 ConVar con_filter_enable
engine.dll!0x58c588 ConVar con_filter_text
engine.dll!0x58c5e0 ConVar con_filter_text_out
engine.dll!0x58c3d0 ConVar con_notifytime
engine.dll!0x58c2c8 ConVar con_timestamp
engine.dll!0x58c378 ConVar con_trace
engine.dll!0x58c428 ConVar contimes
engine.dll!0x598670 ConVar coop
engine.dll!0x598618 ConVar deathmatch
engine.dll!0x595538 ConVar debug_map_crc
engine.dll!0x58a158 ConVar demo_recordcommands
engine.dll!0x58cf08 ConVar demo_strict_validation
engine.dll!0x598828 ConVar developer
engine.dll!0x596820 ConVar display_game_events
engine.dll!0x51aa48 ConVar dsp_automatic
engine.dll!0x519980 ConVar dsp_db_min
engine.dll!0x5199d8 ConVar dsp_db_mixdrop
engine.dll!0x519878 ConVar dsp_dist_max
engine.dll!0x519820 ConVar dsp_dist_min
engine.dll!0x51ad08 ConVar dsp_enhance_stereo
engine.dll!0x51a998 ConVar dsp_facingaway
engine.dll!0x519928 ConVar dsp_mix_max
engine.dll!0x5198d0 ConVar dsp_mix_min
engine.dll!0x51b258 ConVar dsp_off
engine.dll!0x51b360 ConVar dsp_room
engine.dll!0x51aaf8 ConVar dsp_slow_cpu
engine.dll!0x51b2b0 ConVar dsp_spatial
engine.dll!0x51a9f0 ConVar dsp_speaker
engine.dll!0x51acb0 ConVar dsp_vol_2ch
engine.dll!0x51ac58 ConVar dsp_vol_4ch
engine.dll!0x51ac00 ConVar dsp_vol_5ch
engine.dll!0x51aba8 ConVar dsp_volume
engine.dll!0x51a940 ConVar dsp_water
engine.dll!0x58ca18 ConVar enable_debug_overlays
engine.dll!0x59f838 ConVar engine_no_focus_sleep
engine.dll!0x58da40 ConVar fog_enable_water_fog
engine.dll!0x51c3b0 ConVar force_audio_english
engine.dll!0x59f708 ConVar fps_max_menu
engine.dll!0x58b7c0 ConVar fps_screenshot_frequency
engine.dll!0x58b768 ConVar fps_screenshot_threshold
engine.dll!0x592858 ConVar host_flush_threshold
engine.dll!0x5987d0 ConVar host_framerate
engine.dll!0x599170 ConVar host_info_show
engine.dll!0x599220 ConVar host_map
engine.dll!0x599118 ConVar host_name_store
engine.dll!0x5990c0 ConVar host_players_show
engine.dll!0x599068 ConVar host_rules_show
engine.dll!0x5979e8 ConVar host_sleep
engine.dll!0x598880 ConVar host_timescale
engine.dll!0x59d598 ConVar hostip
engine.dll!0x59d3e0 ConVar hostport
engine.dll!0x59c148 ConVar in_forceuser
engine.dll!0x59da00 ConVar ip
engine.dll!0x59dc00 ConVar ip_relay
engine.dll!0x59d490 ConVar ip_steam
engine.dll!0x59dc58 ConVar ip_tv
engine.dll!0x59d7c8 ConVar ip_tv1
engine.dll!0x59c6b0 ConVar lightcache_maxmiss
engine.dll!0x59c600 ConVar mat_ambient_light_b
engine.dll!0x59c308 ConVar mat_ambient_light_g
engine.dll!0x59c658 ConVar mat_ambient_light_r
engine.dll!0x58f478 ConVar mat_bumpbasis
engine.dll!0x58c170 ConVar mat_colorcorrection
engine.dll!0x58ef00 ConVar mat_debugalttab
engine.dll!0x58f218 ConVar mat_depthbias_normal
engine.dll!0x589de8 ConVar mat_dynamic_tonemapping
engine.dll!0x589ef0 ConVar mat_force_tonemap_scale
engine.dll!0x58d8e0 ConVar mat_forcedynamic
engine.dll!0x58f840 ConVar mat_fullbright
engine.dll!0x58ffd8 ConVar mat_loadtextures
engine.dll!0x58f790 ConVar mat_luxels
engine.dll!0x58efb0 ConVar mat_monitorgamma
engine.dll!0x58f110 ConVar mat_monitorgamma_tv_enabled
engine.dll!0x58f7e8 ConVar mat_norendering
engine.dll!0x58f6e0 ConVar mat_normals
engine.dll!0x58beb8 ConVar mat_show_texture_memory_usage
engine.dll!0x58e680 ConVar mat_softwareskin
engine.dll!0x58dd60 ConVar mat_surfaceid
engine.dll!0x58ddb8 ConVar mat_surfacemat
engine.dll!0x58bd10 ConVar mat_texture_list
engine.dll!0x58bdb0 ConVar mat_texture_list_all
engine.dll!0x58be60 ConVar mat_texture_list_view
engine.dll!0x58f738 ConVar mat_wireframe
engine.dll!0x597b60 ConVar mem_incremental_compact_rate
engine.dll!0x58fd10 ConVar mod_dynamicloadpause
engine.dll!0x58fdc0 ConVar mod_dynamicloadspew
engine.dll!0x58fd68 ConVar mod_dynamicloadthrottle
engine.dll!0x58fcb8 ConVar mod_dynamicunloadtex
engine.dll!0x58fc60 ConVar mod_dynamicunloadtime
engine.dll!0x59cb58 ConVar net_blockmsg
engine.dll!0x59d028 ConVar net_droponsendoverflow
engine.dll!0x59d5f0 ConVar net_droppackets
engine.dll!0x58b560 ConVar net_earliertempents
engine.dll!0x59d8f8 ConVar net_fakejitter
engine.dll!0x59d438 ConVar net_fakelag
engine.dll!0x59dba8 ConVar net_fakeloss
engine.dll!0x59dcb0 ConVar net_maxroutable
engine.dll!0x59d950 ConVar net_public_adr
engine.dll!0x59dd08 ConVar net_queue_trace
engine.dll!0x591870 ConVar net_showreliablesounds
engine.dll!0x59d540 ConVar net_showsplits
engine.dll!0x59cd68 ConVar net_showudp
engine.dll!0x59ca50 ConVar net_showudp_oob
engine.dll!0x59d080 ConVar net_showudp_remoteonly
engine.dll!0x59da58 ConVar net_splitrate
engine.dll!0x59d280 ConVar net_steamcnx_allowrelay
engine.dll!0x59d1a8 ConVar net_steamcnx_enabled
engine.dll!0x59d150 ConVar net_threaded_socket_burst_cap
engine.dll!0x59d200 ConVar net_threaded_socket_recovery_rate
engine.dll!0x59d2d8 ConVar net_threaded_socket_recovery_time
engine.dll!0x5981a0 ConVar next
engine.dll!0x595998 ConVar occlusion_old
engine.dll!0x596588 ConVar occlusion_test_async_jitter
engine.dll!0x596530 ConVar occlusion_test_async_move_tolerance
engine.dll!0x596690 ConVar occlusion_test_jump_margin
engine.dll!0x5965e0 ConVar occlusion_test_margins
engine.dll!0x596710 ConVar occlusion_test_shadow_max_distance
engine.dll!0x59dfd0 ConVar paint_alpha_offset_enabled
engine.dll!0x59df78 ConVar paint_max_surface_border_alpha
engine.dll!0x59e130 ConVar paint_min_valid_alpha_value
engine.dll!0x59e188 ConVar paintsplat_bias
engine.dll!0x59e0d8 ConVar paintsplat_max_alpha_noise
engine.dll!0x59e028 ConVar paintsplat_noise_enabled
engine.dll!0x5a3958 ConVar panel_test_title_safe
engine.dll!0x589b00 ConVar password
engine.dll!0x5904a8 ConVar r_ClipAreaFrustums
engine.dll!0x590450 ConVar r_ClipAreaPortals
engine.dll!0x595cd8 ConVar r_DispBuildable
engine.dll!0x595c80 ConVar r_DispWalkable
engine.dll!0x589e98 ConVar r_DrawBeams
engine.dll!0x595c28 ConVar r_DrawDisp
engine.dll!0x58dfd0 ConVar r_DrawModelLightOrigin
engine.dll!0x590660 ConVar r_DrawPortals
engine.dll!0x58eb90 ConVar r_ambientfraction
engine.dll!0x59c490 ConVar r_ambientlightingonly
engine.dll!0x59c7b8 ConVar r_avglight
engine.dll!0x58d338 ConVar r_avglightmap
engine.dll!0x5910b8 ConVar r_brush_queue_mode
engine.dll!0x59e920 ConVar r_colorstaticprops
engine.dll!0x58e2b8 ConVar r_debugrandomstaticlighting
engine.dll!0x59ebc8 ConVar r_disable_static_prop_loading
engine.dll!0x58d3e8 ConVar r_dlightsenable
engine.dll!0x58f580 ConVar r_drawbrushmodels
engine.dll!0x590978 ConVar r_drawdecals
engine.dll!0x58e7e0 ConVar r_drawentities
engine.dll!0x58d9e8 ConVar r_drawfuncdetail
engine.dll!0x58d938 ConVar r_drawleaf
engine.dll!0x58ec98 ConVar r_drawlightcache
engine.dll!0x59c438 ConVar r_drawlightcache
engine.dll!0x58d2e0 ConVar r_drawlightinfo
engine.dll!0x58d288 ConVar r_drawlights
engine.dll!0x58dec8 ConVar r_drawmodelstatsoverlay
engine.dll!0x58df20 ConVar r_drawmodelstatsoverlaydistance
engine.dll!0x58df78 ConVar r_drawmodelstatsoverlayfilter
engine.dll!0x58eda0 ConVar r_drawmodelstatsoverlaymax
engine.dll!0x58ed48 ConVar r_drawmodelstatsoverlaymin
engine.dll!0x58de70 ConVar r_drawskybox
engine.dll!0x59eb70 ConVar r_drawstaticprops
engine.dll!0x58d888 ConVar r_drawtranslucentworld
engine.dll!0x5a05b0 ConVar r_drawvgui
engine.dll!0x58d990 ConVar r_drawworld
engine.dll!0x590920 ConVar r_dscale_basefov
engine.dll!0x5908c8 ConVar r_dscale_fardist
engine.dll!0x590870 ConVar r_dscale_farscale
engine.dll!0x590818 ConVar r_dscale_neardist
engine.dll!0x5907c0 ConVar r_dscale_nearscale
engine.dll!0x58f688 ConVar r_dynamic
engine.dll!0x58d6d0 ConVar r_dynamiclighting
engine.dll!0x58e080 ConVar r_entity
engine.dll!0x58e4c8 ConVar r_eyemove
engine.dll!0x58e520 ConVar r_eyeshift_x
engine.dll!0x58e578 ConVar r_eyeshift_y
engine.dll!0x58e5d0 ConVar r_eyeshift_z
engine.dll!0x58e628 ConVar r_eyesize
engine.dll!0x590f98 ConVar r_flashlightclip
engine.dll!0x590ff0 ConVar r_flashlightdrawclip
engine.dll!0x591048 ConVar r_flashlightscissor
engine.dll!0x58ea48 ConVar r_ignoreStaticColorChecksum
engine.dll!0x58e368 ConVar r_itemblinkmax
engine.dll!0x58e3c0 ConVar r_itemblinkrate
engine.dll!0x59c2b0 ConVar r_lightcache_numambientsamples
engine.dll!0x59c540 ConVar r_lightcache_radiusfactor
engine.dll!0x59c3b8 ConVar r_lightcachecenter
engine.dll!0x58ec40 ConVar r_lightcachemodel
engine.dll!0x58e130 ConVar r_lightinterp
engine.dll!0x58f5d8 ConVar r_lightmap
engine.dll!0x58f630 ConVar r_lightstyle
engine.dll!0x59c8d0 ConVar r_lockpvs
engine.dll!0x58ecf0 ConVar r_modelAmbientMin
engine.dll!0x58e940 ConVar r_modelwireframedecal
engine.dll!0x58e6d8 ConVar r_nohw
engine.dll!0x58e730 ConVar r_nosw
engine.dll!0x59c820 ConVar r_novis
engine.dll!0x590088 ConVar r_occlusionspew
engine.dll!0x59c5a8 ConVar r_oldlightselection
engine.dll!0x59e828 ConVar r_partition_level
engine.dll!0x5905b0 ConVar r_portalsopenall
engine.dll!0x58e418 ConVar r_proplightingpooling
engine.dll!0x59c360 ConVar r_radiosity
engine.dll!0x59e8c8 ConVar r_shadow_deferred
engine.dll!0x590b68 ConVar r_shadowids
engine.dll!0x590ab8 ConVar r_shadows_gamecontrol
engine.dll!0x590b10 ConVar r_shadowwireframe
engine.dll!0x58e470 ConVar r_showenvcubemap
engine.dll!0x58e8e8 ConVar r_skin
engine.dll!0x58e9f0 ConVar r_slowpathwireframe
engine.dll!0x5901e8 ConVar r_visocclusion
engine.dll!0x58d5c8 ConVar r_visualizelighttraces
engine.dll!0x58d620 ConVar r_visualizelighttracesshowfulltrace
engine.dll!0x58d570 ConVar r_visualizetraces
engine.dll!0x591bc0 ConVar replay_debug
engine.dll!0x51aaa0 ConVar room_type
engine.dll!0x59fee0 ConVar rpt_vprof_time
engine.dll!0x5a1448 ConVar showbudget_texture
engine.dll!0x598360 ConVar singlestep
engine.dll!0x598778 ConVar skill
engine.dll!0x519310 ConVar snd_deathcamera_volume
engine.dll!0x51a6d8 ConVar snd_debug_panlaw
engine.dll!0x51b998 ConVar snd_disable_mixer_duck
engine.dll!0x51b940 ConVar snd_disable_mixer_solo
engine.dll!0x51a280 ConVar snd_duckerattacktime
engine.dll!0x51a2d8 ConVar snd_duckerreleasetime
engine.dll!0x51a330 ConVar snd_duckerthreshold
engine.dll!0x51a388 ConVar snd_ducking_off
engine.dll!0x51a228 ConVar snd_ducktovolume
engine.dll!0x519a88 ConVar snd_dvar_dist_max
engine.dll!0x519a30 ConVar snd_dvar_dist_min
engine.dll!0x519100 ConVar snd_dzmusic_volume
engine.dll!0x51a178 ConVar snd_filter
engine.dll!0x59e618 ConVar snd_foliage_db_loss
engine.dll!0x59e778 ConVar snd_gain
engine.dll!0x59e6c8 ConVar snd_gain_max
engine.dll!0x59e5c0 ConVar snd_gain_min
engine.dll!0x518a70 ConVar snd_hrtf_distance_behind
engine.dll!0x5187b0 ConVar snd_hrtf_lerp_max_distance
engine.dll!0x518758 ConVar snd_hrtf_lerp_min_distance
engine.dll!0x51c0f0 ConVar snd_hrtf_stereo_blend
engine.dll!0x51c148 ConVar snd_hrtf_voice_delay
engine.dll!0x51b470 ConVar snd_hrtf_volume
engine.dll!0x518700 ConVar snd_hwcompat
engine.dll!0x51b890 ConVar snd_list
engine.dll!0x519260 ConVar snd_mapobjective_volume
engine.dll!0x518b48 ConVar snd_max_same_sounds
engine.dll!0x518ba0 ConVar snd_max_same_weapon_sounds
engine.dll!0x519158 ConVar snd_menumusic_volume
engine.dll!0x5194c8 ConVar snd_mix_async
engine.dll!0x519520 ConVar snd_mix_async_onetime_reset
engine.dll!0x5193c0 ConVar snd_mixahead
engine.dll!0x51b7e0 ConVar snd_mixer_master_dsp
engine.dll!0x51b788 ConVar snd_mixer_master_level
engine.dll!0x519578 ConVar snd_music_volume_onetime_reset_2
engine.dll!0x519050 ConVar snd_musicvolume_multiplier_inoverlay
engine.dll!0x5195d0 ConVar snd_mute_losefocus
engine.dll!0x519368 ConVar snd_mvp_volume
engine.dll!0x519b90 ConVar snd_obscured_gain_dB
engine.dll!0x51c9e8 ConVar snd_occlusion_bounces
engine.dll!0x51c7d8 ConVar snd_occlusion_collide_min_distance
engine.dll!0x51c8e0 ConVar snd_occlusion_eq_high
engine.dll!0x51c830 ConVar snd_occlusion_eq_low
engine.dll!0x51c888 ConVar snd_occlusion_eq_mid
engine.dll!0x51c780 ConVar snd_occlusion_indirect_max
engine.dll!0x51c728 ConVar snd_occlusion_indirect_min
engine.dll!0x51c6d0 ConVar snd_occlusion_indirect_radius
engine.dll!0x51c5c8 ConVar snd_occlusion_material_override
engine.dll!0x51c938 ConVar snd_occlusion_no_eq_scale
engine.dll!0x51c570 ConVar snd_occlusion_pos_override
engine.dll!0x51c990 ConVar snd_occlusion_rays
engine.dll!0x51c620 ConVar snd_occlusion_visualize
engine.dll!0x51c678 ConVar snd_occlusion_visualize_filter
engine.dll!0x51c518 ConVar snd_op_test_convar
engine.dll!0x51b4c8 ConVar snd_pause_all
engine.dll!0x518fa0 ConVar snd_pitchquality
engine.dll!0x519b38 ConVar snd_pre_gain_dist_falloff
engine.dll!0x51c358 ConVar snd_prefetch_common
engine.dll!0x51ab50 ConVar snd_profile
engine.dll!0x518650 ConVar snd_rear_speaker_scale
engine.dll!0x59e720 ConVar snd_refdist
engine.dll!0x5189c0 ConVar snd_report_format_sound
engine.dll!0x518968 ConVar snd_report_loop_sound
engine.dll!0x5188b8 ConVar snd_report_start_sound
engine.dll!0x518910 ConVar snd_report_stop_sound
engine.dll!0x518a18 ConVar snd_report_verbose_error
engine.dll!0x519208 ConVar snd_roundend_volume
engine.dll!0x5191b0 ConVar snd_roundstart_volume
engine.dll!0x518de8 ConVar snd_show
engine.dll!0x518e98 ConVar snd_show_filter
engine.dll!0x51b838 ConVar snd_showclassname
engine.dll!0x51b8e8 ConVar snd_showmixer
engine.dll!0x519ae0 ConVar snd_showstart
engine.dll!0x51cc50 ConVar snd_sos_list_operator_updates
engine.dll!0x51a1d0 ConVar snd_sos_show_block_debug
engine.dll!0x5186a8 ConVar snd_sos_show_client_rcv
engine.dll!0x51cf08 ConVar snd_sos_show_operator_entry_filter
engine.dll!0x51cb48 ConVar snd_sos_show_operator_init
engine.dll!0x51cbf8 ConVar snd_sos_show_operator_parse
engine.dll!0x51ce58 ConVar snd_sos_show_operator_prestart
engine.dll!0x51ce00 ConVar snd_sos_show_operator_shutdown
engine.dll!0x51ceb0 ConVar snd_sos_show_operator_start
engine.dll!0x51ca98 ConVar snd_sos_show_operator_stop_entry
engine.dll!0x51cba0 ConVar snd_sos_show_operator_updates
engine.dll!0x51caf0 ConVar snd_sos_show_queuetotrack
engine.dll!0x51ccd0 ConVar snd_sos_show_startqueue
engine.dll!0x51a730 ConVar snd_surround_speakers
engine.dll!0x5192b8 ConVar snd_tensecondwarning_volume
engine.dll!0x518f48 ConVar snd_visualize
engine.dll!0x519628 ConVar sound_device_override
engine.dll!0x591660 ConVar spec_replay_enable
engine.dll!0x5973b8 ConVar spec_replay_leadup_time
engine.dll!0x5916b8 ConVar spec_replay_message_time
engine.dll!0x591920 ConVar spec_replay_on_death
engine.dll!0x591b68 ConVar spec_replay_rate_base
engine.dll!0x591710 ConVar spec_replay_rate_limit
engine.dll!0x5918c8 ConVar sv_allow_legacy_cmd_execution_from_client
engine.dll!0x594cc8 ConVar sv_allow_wait_command
engine.dll!0x591558 ConVar sv_allowdownload
engine.dll!0x5915b0 ConVar sv_allowupload
engine.dll!0x594e80 ConVar sv_alternateticks
engine.dll!0x592cf8 ConVar sv_client_cmdrate_difference
engine.dll!0x592da8 ConVar sv_client_max_interp_ratio
engine.dll!0x592d50 ConVar sv_client_min_interp_ratio
engine.dll!0x592e00 ConVar sv_client_predict
engine.dll!0x5932d8 ConVar sv_consistency
engine.dll!0x592ae8 ConVar sv_contact
engine.dll!0x591f90 ConVar sv_creationtickcheck
engine.dll!0x593578 ConVar sv_debugmanualmode
engine.dll!0x592bf0 ConVar sv_downloadurl
engine.dll!0x59dd60 ConVar sv_dumpstringtables
engine.dll!0x591138 ConVar sv_duplicate_playernames_ok
engine.dll!0x593520 ConVar sv_enable_delta_packing
engine.dll!0x5937f0 ConVar sv_forcepreload
engine.dll!0x592750 ConVar sv_hibernate_ms
engine.dll!0x5927a8 ConVar sv_hibernate_ms_vgui
engine.dll!0x592800 ConVar sv_hibernate_postgame_delay
engine.dll!0x5926f8 ConVar sv_hibernate_punt_tv_clients
engine.dll!0x595060 ConVar sv_hosting_lobby
engine.dll!0x592a38 ConVar sv_lan
engine.dll!0x592358 ConVar sv_log_onefile
engine.dll!0x5923b0 ConVar sv_logbans
engine.dll!0x591fe8 ConVar sv_logblocks
engine.dll!0x592300 ConVar sv_logecho
engine.dll!0x592250 ConVar sv_logfile
engine.dll!0x5922a8 ConVar sv_logflush
engine.dll!0x5921f8 ConVar sv_logsdir
engine.dll!0x592408 ConVar sv_logsecret
engine.dll!0x592460 ConVar sv_logsocket2
engine.dll!0x5924b8 ConVar sv_logsocket2_substr
engine.dll!0x5917c0 ConVar sv_max_dropped_packets_to_process
engine.dll!0x5921a0 ConVar sv_max_queries_sec
engine.dll!0x592098 ConVar sv_max_queries_sec_global
engine.dll!0x5920f0 ConVar sv_max_queries_tracked_ips_max
engine.dll!0x592040 ConVar sv_max_queries_tracked_ips_prune
engine.dll!0x592148 ConVar sv_max_queries_window
engine.dll!0x592ca0 ConVar sv_maxcmdrate
engine.dll!0x591348 ConVar sv_maxrate
engine.dll!0x5913f8 ConVar sv_maxupdaterate
engine.dll!0x593048 ConVar sv_maxuptimelimit
engine.dll!0x592f98 ConVar sv_memlimit
engine.dll!0x592c48 ConVar sv_mincmdrate
engine.dll!0x5913a0 ConVar sv_minrate
engine.dll!0x591450 ConVar sv_minupdaterate
engine.dll!0x592ff0 ConVar sv_minuptimelimit
engine.dll!0x593628 ConVar sv_parallel_packentities
engine.dll!0x5930c8 ConVar sv_parallel_sendsnapshot
engine.dll!0x592a90 ConVar sv_pausable
engine.dll!0x592988 ConVar sv_pure_consensus
engine.dll!0x5931d0 ConVar sv_pure_kick_clients
engine.dll!0x5929e0 ConVar sv_pure_retiretime
engine.dll!0x593228 ConVar sv_pure_trace
engine.dll!0x591818 ConVar sv_quota_stringcmdspersecond
engine.dll!0x593920 ConVar sv_rcon_whitelist_address
engine.dll!0x595168 ConVar sv_region
engine.dll!0x591298 ConVar sv_reliableavatardata
engine.dll!0x591500 ConVar sv_replaybots
engine.dll!0x594f30 ConVar sv_reservation_tickrate_adjustment
engine.dll!0x5950b8 ConVar sv_reservation_timeout
engine.dll!0x591cf0 ConVar sv_show_cull_props
engine.dll!0x593b30 ConVar sv_steamauth_enforce
engine.dll!0x594c70 ConVar sv_steamgroup_exclusive
engine.dll!0x5914a8 ConVar sv_stressbots
engine.dll!0x58b2a0 ConVar sv_unlockedchapters
engine.dll!0x5934c8 ConVar sv_validate_edict_change_infos
engine.dll!0x594dd0 ConVar sv_visiblemaxplayers
engine.dll!0x519ee8 ConVar sv_voice_proximity_minvolume
engine.dll!0x589468 ConVar sv_voice_proximity_use_falloff
engine.dll!0x593398 ConVar sv_voicecodec
engine.dll!0x592b98 ConVar sv_voiceenable
engine.dll!0x59f410 ConVar sys_minidumpspewlines
engine.dll!0x5a1740 ConVar texture_budget_background_alpha
engine.dll!0x59e7d0 ConVar think_trace_limit
engine.dll!0x5971b0 ConVar tv_advertise_watchable
engine.dll!0x5935d0 ConVar tv_allow_camera_man_override
engine.dll!0x597840 ConVar tv_autorecord
engine.dll!0x596ee0 ConVar tv_autoretry
engine.dll!0x596af0 ConVar tv_broadcast_drop_fragments
engine.dll!0x596c20 ConVar tv_broadcast_keyframe_interval
engine.dll!0x596a40 ConVar tv_broadcast_keyframe_interval1
engine.dll!0x596ba0 ConVar tv_broadcast_max_requests
engine.dll!0x596b48 ConVar tv_broadcast_max_requests1
engine.dll!0x596a98 ConVar tv_broadcast_origin_auth
engine.dll!0x5969e8 ConVar tv_broadcast_origin_auth1
engine.dll!0x596c78 ConVar tv_broadcast_startup_resend_interval
engine.dll!0x596990 ConVar tv_broadcast_terminate
engine.dll!0x5976b8 ConVar tv_broadcast_url
engine.dll!0x597738 ConVar tv_broadcast_url1
engine.dll!0x596cd0 ConVar tv_chatgroupsize
engine.dll!0x596d80 ConVar tv_chattimelimit
engine.dll!0x597790 ConVar tv_debug
engine.dll!0x597638 ConVar tv_deltacache
engine.dll!0x597158 ConVar tv_dispatchmode
engine.dll!0x597230 ConVar tv_dispatchweight
engine.dll!0x5974b0 ConVar tv_enable_delta_frames
engine.dll!0x596fb8 ConVar tv_encryptdata_key
engine.dll!0x597508 ConVar tv_encryptdata_key_pub
engine.dll!0x597898 ConVar tv_maxclients
engine.dll!0x5978f0 ConVar tv_maxclients_relayreserved
engine.dll!0x596dd8 ConVar tv_maxrate
engine.dll!0x58b2f8 ConVar tv_nochat
engine.dll!0x597288 ConVar tv_overridemaster
engine.dll!0x597588 ConVar tv_password
engine.dll!0x58d068 ConVar tv_playcast_delay_prediction
engine.dll!0x58a0d8 ConVar tv_playcast_delay_resync
engine.dll!0x58d118 ConVar tv_playcast_max_rcvage
engine.dll!0x58d010 ConVar tv_playcast_max_rtdelay
engine.dll!0x58cf60 ConVar tv_playcast_origin_auth
engine.dll!0x58cfb8 ConVar tv_playcast_retry_timeout
engine.dll!0x59d9a8 ConVar tv_port
engine.dll!0x59d878 ConVar tv_port1
engine.dll!0x596d28 ConVar tv_relaypassword
engine.dll!0x597080 ConVar tv_relayvoice
engine.dll!0x596f38 ConVar tv_snapshotrate
engine.dll!0x596e30 ConVar tv_snapshotrate1
engine.dll!0x596e88 ConVar tv_timeout
engine.dll!0x597948 ConVar tv_transmitall
engine.dll!0x5a0fb8 ConVar vgui_drawtree
engine.dll!0x51d328 ConVar voice_caster_enable
engine.dll!0x51d118 ConVar voice_caster_scale
engine.dll!0x51d2d0 ConVar voice_enable
engine.dll!0x51d3d8 ConVar voice_forcemicrecord
engine.dll!0x598b20 ConVar voice_inputfromfile
engine.dll!0x51cf60 ConVar voice_loopback
engine.dll!0x5893b8 ConVar voice_mixer_boost
engine.dll!0x589360 ConVar voice_mixer_mute
engine.dll!0x589410 ConVar voice_mixer_volume
engine.dll!0x5894c0 ConVar voice_positional
engine.dll!0x519f40 ConVar voice_positional_seconds_after_death
engine.dll!0x598c98 ConVar voice_recordtofile
engine.dll!0x51d0c0 ConVar voice_scale
engine.dll!0x51d380 ConVar voice_threshold
engine.dll!0x518ff8 ConVar volume
engine.dll!0x5a1848 ConVar vprof_graphheight
engine.dll!0x5a17f0 ConVar vprof_graphwidth
engine.dll!0x5a1988 ConVar vprof_unaccounted_limit
engine.dll!0x5a1930 ConVar vprof_verbose
engine.dll!0x5a19e0 ConVar vprof_warningmsec
engine.dll!0x5a0438 ConVar xbox_arcade_remaining_trial_time
```

## ConCommands

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

### Addresses

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

# Client

## Interfaces

```
client_panorama.dll!0x04d13564 ClientAlphaPropertyMgrV001
client_panorama.dll!0x04d13558 ClientLeafSystem002
client_panorama.dll!0x00cca1a0 ClientRenderTargets001
client_panorama.dll!0x00ccf648 GameClientExports001
client_panorama.dll!0x00cecfac GameConsole004
client_panorama.dll!0x0512d638 GameMovement001
client_panorama.dll!0x0512f9f8 GameUI011
client_panorama.dll!0x00cd16d8 IEffects001
client_panorama.dll!0x0511d658 RenderToRTHelper001
client_panorama.dll!0x00ced18c RunGameEngine005
client_panorama.dll!0x00ccf8c8 VCLIENTMATERIALSYSTEM001
client_panorama.dll!0x05173110 VCLIENTTOOLS001
client_panorama.dll!0x04cde1e0 VClient018
client_panorama.dll!0x04cde1c8 VClientDllSharedAppSystems001
client_panorama.dll!0x04d034dc VClientEntityList003
client_panorama.dll!0x0512e948 VClientPrediction001
client_panorama.dll!0x0512f308 VENGINE_GAMETYPES_VERSION002
client_panorama.dll!0x00ced3cc VGuiModuleLoader003
client_panorama.dll!0x00cda028 VParticleSystemQuery004
```

## ConVars

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
<summary><code>cl_quickinventory_deadzone_size</code></summary>

default: `"0.05"`  
flags: `0x80088`  
</details>
<details>
<summary><code>cl_quickinventory_lastinv</code></summary>

default: `"0"`  
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

enable threading of detail prop drawing

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>mat_fullbright</code></summary>

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

default: `"0"`  
flags: `0x4000`  
</details>
<details>
<summary><code>r_showenvcubemap</code></summary>

Trims the algorithm from processing darks: (0.0312 - visible limit, slower), (0.0625 - high quality, faster), (0.0833 - upper limit, the start of visible unfiltered edges). Special note: when using FXAA_GREEN_AS_LUMA, likely want to set this to zero

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
<summary><code>sv_dz_hostage_rescue_reward</code></summary>

Number of cash bundles to award for rescuing a hostage

default: `"10"`  
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

default: `"500"`  
flags: `0x40000080`  
</details>
<details>
<summary><code>test_convar</code></summary>

Skips the prompt when saving a buy favorite in the buy menu

default: `"0"`  
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
<summary><code>ui_playsettings_maps_official_dangerzone</code></summary>

default: `"mg_dz_blacksite"`  
flags: `0x80080`  
</details>
<details>
<summary><code>ui_playsettings_maps_official_deathmatch</code></summary>

default: `"mg_casualsigma"`  
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

### Addresses

```
client_panorama.dll!0xcf6ea0 ConVar BlendBonesMode
client_panorama.dll!0xcc9e60 ConVar achievement_debug
client_panorama.dll!0xcc9eb8 ConVar achievement_disable
client_panorama.dll!0xcca368 ConVar ai_debug_shoot_positions
client_panorama.dll!0xcca310 ConVar ai_shot_bias_max
client_panorama.dll!0xcca2b8 ConVar ai_shot_bias_min
client_panorama.dll!0xce6220 ConVar ammo_338mag_headshot_mult
client_panorama.dll!0xce5eb0 ConVar ammo_338mag_impulse
client_panorama.dll!0xce5828 ConVar ammo_338mag_max
client_panorama.dll!0xce6380 ConVar ammo_357sig_headshot_mult
client_panorama.dll!0xce6010 ConVar ammo_357sig_impulse
client_panorama.dll!0xce5988 ConVar ammo_357sig_max
client_panorama.dll!0xce5a90 ConVar ammo_357sig_min_max
client_panorama.dll!0xce59e0 ConVar ammo_357sig_p250_max
client_panorama.dll!0xce5a38 ConVar ammo_357sig_small_max
client_panorama.dll!0xce6328 ConVar ammo_45acp_headshot_mult
client_panorama.dll!0xce5fb8 ConVar ammo_45acp_impulse
client_panorama.dll!0xce5930 ConVar ammo_45acp_max
client_panorama.dll!0xce60c0 ConVar ammo_50AE_headshot_mult
client_panorama.dll!0xce5d50 ConVar ammo_50AE_impulse
client_panorama.dll!0xce5670 ConVar ammo_50AE_max
client_panorama.dll!0xce61c8 ConVar ammo_556mm_box_headshot_mult
client_panorama.dll!0xce5e58 ConVar ammo_556mm_box_impulse
client_panorama.dll!0xce57d0 ConVar ammo_556mm_box_max
client_panorama.dll!0xce6170 ConVar ammo_556mm_headshot_mult
client_panorama.dll!0xce5e00 ConVar ammo_556mm_impulse
client_panorama.dll!0xce5720 ConVar ammo_556mm_max
client_panorama.dll!0xce5778 ConVar ammo_556mm_small_max
client_panorama.dll!0xce63d8 ConVar ammo_57mm_headshot_mult
client_panorama.dll!0xce6068 ConVar ammo_57mm_impulse
client_panorama.dll!0xce5ae8 ConVar ammo_57mm_max
client_panorama.dll!0xce6118 ConVar ammo_762mm_headshot_mult
client_panorama.dll!0xce5da8 ConVar ammo_762mm_impulse
client_panorama.dll!0xce56c8 ConVar ammo_762mm_max
client_panorama.dll!0xce6278 ConVar ammo_9mm_headshot_mult
client_panorama.dll!0xce5f08 ConVar ammo_9mm_impulse
client_panorama.dll!0xce5880 ConVar ammo_9mm_max
client_panorama.dll!0xce62d0 ConVar ammo_buckshot_headshot_mult
client_panorama.dll!0xce5f60 ConVar ammo_buckshot_impulse
client_panorama.dll!0xce58d8 ConVar ammo_buckshot_max
client_panorama.dll!0xce5cf8 ConVar ammo_grenade_limit_breachcharge
client_panorama.dll!0xce5b40 ConVar ammo_grenade_limit_default
client_panorama.dll!0xce5b98 ConVar ammo_grenade_limit_flashbang
client_panorama.dll!0xce5c48 ConVar ammo_grenade_limit_snowballs
client_panorama.dll!0xce5bf0 ConVar ammo_grenade_limit_total
client_panorama.dll!0xce5ca0 ConVar ammo_item_limit_healthshot
client_panorama.dll!0xcf6ef8 ConVar anim_3wayblend
client_panorama.dll!0xcf6f50 ConVar anim_twistbones_enabled
client_panorama.dll!0xce82c8 ConVar bot_autodifficulty_threshold_high
client_panorama.dll!0xce8270 ConVar bot_autodifficulty_threshold_low
client_panorama.dll!0xcd47e0 ConVar c_maxdistance
client_panorama.dll!0xcd4680 ConVar c_maxpitch
client_panorama.dll!0xcd4730 ConVar c_maxyaw
client_panorama.dll!0xcd4838 ConVar c_mindistance
client_panorama.dll!0xcd46d8 ConVar c_minpitch
client_panorama.dll!0xcd4788 ConVar c_minyaw
client_panorama.dll!0xcd48e8 ConVar c_orthoheight
client_panorama.dll!0xcd4890 ConVar c_orthowidth
client_panorama.dll!0xcd4940 ConVar c_thirdpersonshoulder
client_panorama.dll!0xcd4aa0 ConVar c_thirdpersonshoulderaimdist
client_panorama.dll!0xcd49f0 ConVar c_thirdpersonshoulderdist
client_panorama.dll!0xcd4a48 ConVar c_thirdpersonshoulderheight
client_panorama.dll!0xcd4998 ConVar c_thirdpersonshoulderoffset
client_panorama.dll!0xcefbc8 ConVar cachedvalue_count_partybrowser
client_panorama.dll!0xcf0760 ConVar cachedvalue_count_teammates
client_panorama.dll!0xcd45d0 ConVar cam_collision
client_panorama.dll!0xcd44c8 ConVar cam_idealdelta
client_panorama.dll!0xcd4f08 ConVar cam_idealdist
client_panorama.dll!0xcd4eb0 ConVar cam_idealdistright
client_panorama.dll!0xcd4e58 ConVar cam_idealdistup
client_panorama.dll!0xcd4470 ConVar cam_ideallag
client_panorama.dll!0xcd4578 ConVar cam_idealpitch
client_panorama.dll!0xcd4520 ConVar cam_idealyaw
client_panorama.dll!0xcd4628 ConVar cam_showangles
client_panorama.dll!0xcd4418 ConVar cam_snapto
client_panorama.dll!0xce9a28 ConVar cash_player_bomb_defused
client_panorama.dll!0xce99d0 ConVar cash_player_bomb_planted
client_panorama.dll!0xce9b30 ConVar cash_player_damage_hostage
client_panorama.dll!0xce9c38 ConVar cash_player_get_killed
client_panorama.dll!0xce9ad8 ConVar cash_player_interact_with_hostage
client_panorama.dll!0xce9978 ConVar cash_player_killed_enemy_default
client_panorama.dll!0xce9920 ConVar cash_player_killed_enemy_factor
client_panorama.dll!0xce9b88 ConVar cash_player_killed_hostage
client_panorama.dll!0xce98c8 ConVar cash_player_killed_teammate
client_panorama.dll!0xce9a80 ConVar cash_player_rescued_hostage
client_panorama.dll!0xce9be0 ConVar cash_player_respawn_amount
client_panorama.dll!0xce9450 ConVar cash_team_elimination_bomb_map
client_panorama.dll!0xce93f8 ConVar cash_team_elimination_hostage_map_ct
client_panorama.dll!0xce93a0 ConVar cash_team_elimination_hostage_map_t
client_panorama.dll!0xce97c0 ConVar cash_team_hostage_alive
client_panorama.dll!0xce9870 ConVar cash_team_hostage_interaction
client_panorama.dll!0xce9660 ConVar cash_team_loser_bonus
client_panorama.dll!0xce96b8 ConVar cash_team_loser_bonus_consecutive_rounds
client_panorama.dll!0xce9818 ConVar cash_team_planted_bomb_but_defused
client_panorama.dll!0xce9768 ConVar cash_team_rescued_hostage
client_panorama.dll!0xce94a8 ConVar cash_team_survive_guardian_wave
client_panorama.dll!0xce9348 ConVar cash_team_terrorist_win_bomb
client_panorama.dll!0xce95b0 ConVar cash_team_win_by_defusing_bomb
client_panorama.dll!0xce9608 ConVar cash_team_win_by_hostage_rescue
client_panorama.dll!0xce9558 ConVar cash_team_win_by_time_running_out_bomb
client_panorama.dll!0xce9500 ConVar cash_team_win_by_time_running_out_hostage
client_panorama.dll!0xce9710 ConVar cash_team_winner_bonus_consecutive_rounds
client_panorama.dll!0xcd3610 ConVar cc_linger_time
client_panorama.dll!0xcd3668 ConVar cc_predisplay_time
client_panorama.dll!0xcdb8e8 ConVar cc_showmissing
client_panorama.dll!0xcd3718 ConVar cc_subtitles
client_panorama.dll!0xcf6fa8 ConVar choreo_spew_filter
client_panorama.dll!0xce4708 ConVar cl_autobuy
client_panorama.dll!0xce9298 ConVar cl_autohelp
client_panorama.dll!0xce91e8 ConVar cl_autowepswitch
client_panorama.dll!0xcd6b90 ConVar cl_backspeed
client_panorama.dll!0xcf1ce8 ConVar cl_bob_lower_amt
client_panorama.dll!0xcf1b30 ConVar cl_bob_version
client_panorama.dll!0xcf1c90 ConVar cl_bobamt_lat
client_panorama.dll!0xcf1c38 ConVar cl_bobamt_vert
client_panorama.dll!0xcf1b88 ConVar cl_bobcycle
client_panorama.dll!0xcf1be0 ConVar cl_bobup
client_panorama.dll!0xce0f98 ConVar cl_brushfastpath
client_panorama.dll!0xcf1ad8 ConVar cl_cam_driver_compensation_scale
client_panorama.dll!0xcdd308 ConVar cl_camera_follow_bone_index
client_panorama.dll!0xce2eb8 ConVar cl_camera_height_restriction_debug
client_panorama.dll!0xcd35a8 ConVar cl_chatfilter_version
client_panorama.dll!0xcd3550 ConVar cl_chatfilters
client_panorama.dll!0xcf0230 ConVar cl_clanid
client_panorama.dll!0xcedfd8 ConVar cl_compass_enabled
client_panorama.dll!0xce3a40 ConVar cl_connection_trouble_show
client_panorama.dll!0xcdc688 ConVar cl_countbones
client_panorama.dll!0xcf1870 ConVar cl_crosshair_drawoutline
client_panorama.dll!0xcf1a28 ConVar cl_crosshair_dynamic_maxdist_splitratio
client_panorama.dll!0xcf1978 ConVar cl_crosshair_dynamic_splitalpha_innermod
client_panorama.dll!0xcf19d0 ConVar cl_crosshair_dynamic_splitalpha_outermod
client_panorama.dll!0xcf1920 ConVar cl_crosshair_dynamic_splitdist
client_panorama.dll!0xcf18c8 ConVar cl_crosshair_outlinethickness
client_panorama.dll!0xcf1318 ConVar cl_crosshair_recoil
client_panorama.dll!0xcedc68 ConVar cl_crosshair_sniper_show_normal_inaccuracy
client_panorama.dll!0xce2570 ConVar cl_crosshair_sniper_width
client_panorama.dll!0xcf1a80 ConVar cl_crosshair_t
client_panorama.dll!0xcf15b0 ConVar cl_crosshairalpha
client_panorama.dll!0xcf14a8 ConVar cl_crosshaircolor
client_panorama.dll!0xcf21b8 ConVar cl_crosshaircolor_b
client_panorama.dll!0xcf2160 ConVar cl_crosshaircolor_g
client_panorama.dll!0xcf2108 ConVar cl_crosshaircolor_r
client_panorama.dll!0xcf17c0 ConVar cl_crosshairdot
client_panorama.dll!0xcf1660 ConVar cl_crosshairgap
client_panorama.dll!0xcf16b8 ConVar cl_crosshairgap_useweaponvalue
client_panorama.dll!0xcf1558 ConVar cl_crosshairscale
client_panorama.dll!0xcf1710 ConVar cl_crosshairsize
client_panorama.dll!0xcf2210 ConVar cl_crosshairstyle
client_panorama.dll!0xcf1768 ConVar cl_crosshairthickness
client_panorama.dll!0xcf1608 ConVar cl_crosshairusealpha
client_panorama.dll!0xcde110 ConVar cl_custommaterial_debug_graph
client_panorama.dll!0xce2f68 ConVar cl_dangerzone_approaching_sound_radius
client_panorama.dll!0xce3018 ConVar cl_dangerzone_moving_sound_volume
client_panorama.dll!0xce2fc0 ConVar cl_dangerzone_sound_volume
client_panorama.dll!0xcceb30 ConVar cl_debugrumble
client_panorama.dll!0xcd1438 ConVar cl_detail_avoid_force
client_panorama.dll!0xcd13e0 ConVar cl_detail_avoid_radius
client_panorama.dll!0xcd1490 ConVar cl_detail_avoid_recover_speed
client_panorama.dll!0xcd1388 ConVar cl_detail_max_sway
client_panorama.dll!0xcd1610 ConVar cl_detail_multiplier
client_panorama.dll!0xccacc8 ConVar cl_disable_ragdolls
client_panorama.dll!0xce4658 ConVar cl_disablefreezecam
client_panorama.dll!0xcf3868 ConVar cl_disablehtmlmotd
client_panorama.dll!0xce2af0 ConVar cl_dm_buyrandomweapons
client_panorama.dll!0xce40d8 ConVar cl_draw_only_deathnotices
client_panorama.dll!0xccfdb0 ConVar cl_drawhud
client_panorama.dll!0xce3ef8 ConVar cl_drawhud_force_deathnotices
client_panorama.dll!0xce42e8 ConVar cl_drawhud_force_radar
client_panorama.dll!0xce3e48 ConVar cl_drawhud_force_teamid_overhead
client_panorama.dll!0xceee20 ConVar cl_drawhud_specvote
client_panorama.dll!0xccf9a0 ConVar cl_drawleaf
client_panorama.dll!0xcddb38 ConVar cl_drawmaterial
client_panorama.dll!0xcde190 ConVar cl_drawshadowtexture
client_panorama.dll!0xcee430 ConVar cl_dz_playagain_auto_spectate
client_panorama.dll!0xcef338 ConVar cl_embedded_stream_audio_volume_xmaster
client_panorama.dll!0xccb2d8 ConVar cl_extrapolate
client_panorama.dll!0xcd84e8 ConVar cl_extrapolate_amount
client_panorama.dll!0xcd1668 ConVar cl_fastdetailsprites
client_panorama.dll!0xce3d40 ConVar cl_fixedcrosshairgap
client_panorama.dll!0xce2200 ConVar cl_foot_contact_shadows
client_panorama.dll!0xcd79e8 ConVar cl_forwardspeed
client_panorama.dll!0xce2e60 ConVar cl_freezecameffects_showholiday
client_panorama.dll!0xce3fa8 ConVar cl_freezecampanel_position_dynamic
client_panorama.dll!0xcec600 ConVar cl_http_log_enable
client_panorama.dll!0xce2d58 ConVar cl_hud_background_alpha
client_panorama.dll!0xce2d00 ConVar cl_hud_bomb_under_radar
client_panorama.dll!0xce2c50 ConVar cl_hud_color
client_panorama.dll!0xce2db0 ConVar cl_hud_healthammo_style
client_panorama.dll!0xce2ba0 ConVar cl_hud_playercount_pos
client_panorama.dll!0xce2bf8 ConVar cl_hud_playercount_showcount
client_panorama.dll!0xcdaad0 ConVar cl_idealpitchscale
client_panorama.dll!0xccb5f0 ConVar cl_interpolate
client_panorama.dll!0xcef550 ConVar cl_inventory_debug_tooltip
client_panorama.dll!0xcef4a0 ConVar cl_inventory_saved_filter2
client_panorama.dll!0xcef4f8 ConVar cl_inventory_saved_sort2
client_panorama.dll!0xcd85f0 ConVar cl_jiggle_bone_debug
client_panorama.dll!0xcd8540 ConVar cl_jiggle_bone_debug_pitch_constraints
client_panorama.dll!0xcd86a0 ConVar cl_jiggle_bone_debug_yaw_constraints
client_panorama.dll!0xcd8598 ConVar cl_jiggle_bone_invert
client_panorama.dll!0xccf5a8 ConVar cl_join_advertise
client_panorama.dll!0xcdaa78 ConVar cl_lagcompensation
client_panorama.dll!0xccfb00 ConVar cl_leafsystemvis
client_panorama.dll!0xcdd710 ConVar cl_leveloverview
client_panorama.dll!0xcf37c8 ConVar cl_leveloverviewmarker
client_panorama.dll!0xcdd4c0 ConVar cl_lock_camera
client_panorama.dll!0xcef268 ConVar cl_mainmenu_show_datagraph
client_panorama.dll!0xcdfaa8 ConVar cl_maxrenderable_dist
client_panorama.dll!0xccadd0 ConVar cl_minimal_rtt_shadows
client_panorama.dll!0xcd8178 ConVar cl_mouselook
client_panorama.dll!0xce1570 ConVar cl_mute_all_but_friends_and_party
client_panorama.dll!0xce1518 ConVar cl_mute_enemy_team
client_panorama.dll!0xce36a8 ConVar cl_obs_interp_enable
client_panorama.dll!0xce3ce8 ConVar cl_observercrosshair
client_panorama.dll!0xcdde50 ConVar cl_overdraw_test
client_panorama.dll!0xcd9d80 ConVar cl_particle_retire_cost
client_panorama.dll!0xcd9b18 ConVar cl_particles_show_bbox
client_panorama.dll!0xcd9f08 ConVar cl_particles_show_controlpoints
client_panorama.dll!0xcdabd8 ConVar cl_pclass
client_panorama.dll!0xcdab80 ConVar cl_pdump
client_panorama.dll!0xcda3d0 ConVar cl_phys_show_active
client_panorama.dll!0xcda0f0 ConVar cl_phys_timescale
client_panorama.dll!0xcd7bf8 ConVar cl_pitchdown
client_panorama.dll!0xcd7c50 ConVar cl_pitchup
client_panorama.dll!0xce34c8 ConVar cl_player_proximity_debug
client_panorama.dll!0xcf6aa8 ConVar cl_playerspray_auto_apply
client_panorama.dll!0xccc458 ConVar cl_portal_use_new_dissolve
client_panorama.dll!0xcdab28 ConVar cl_predictionlist
client_panorama.dll!0xcdaa20 ConVar cl_predictweapons
client_panorama.dll!0xcee280 ConVar cl_quickinventory_deadzone_size
client_panorama.dll!0xcee2d8 ConVar cl_quickinventory_lastinv
client_panorama.dll!0xce4238 ConVar cl_radar_always_centered
client_panorama.dll!0xce4290 ConVar cl_radar_icon_scale_min
client_panorama.dll!0xce4188 ConVar cl_radar_rotate
client_panorama.dll!0xce41e0 ConVar cl_radar_scale
client_panorama.dll!0xce4130 ConVar cl_radar_square_with_scoreboard
client_panorama.dll!0xce31b0 ConVar cl_ragdoll_workaround_threshold
client_panorama.dll!0xcef018 ConVar cl_rappel_tilt
client_panorama.dll!0xce46b0 ConVar cl_rebuy
client_panorama.dll!0xcdc148 ConVar cl_remove_old_ugc_downloads
client_panorama.dll!0xccc108 ConVar cl_righthand
client_panorama.dll!0xccead8 ConVar cl_rumblescale
client_panorama.dll!0xcce0b8 ConVar cl_sanitize_player_names
client_panorama.dll!0xcf1500 ConVar cl_scalecrosshair
client_panorama.dll!0xce45b8 ConVar cl_scoreboard_mouse_enable_binding
client_panorama.dll!0xcee380 ConVar cl_scoreboard_survivors_always_on
client_panorama.dll!0xcef910 ConVar cl_server_graphic1_enable
client_panorama.dll!0xcef8b8 ConVar cl_server_graphic2_enable
client_panorama.dll!0xcde060 ConVar cl_shadowtextureoverlaysize
client_panorama.dll!0xce2a98 ConVar cl_show_clan_in_death_notice
client_panorama.dll!0xcc9f38 ConVar cl_showanimstate
client_panorama.dll!0xcc9fe8 ConVar cl_showanimstate_activities
client_panorama.dll!0xcc9f90 ConVar cl_showanimstate_log
client_panorama.dll!0xcdad38 ConVar cl_showerror
client_panorama.dll!0xcdc4c8 ConVar cl_showfps
client_panorama.dll!0xcd3100 ConVar cl_showhelp
client_panorama.dll!0xcdc520 ConVar cl_showpos
client_panorama.dll!0xcd7ca8 ConVar cl_sidespeed
client_panorama.dll!0xcf6e48 ConVar cl_simdbones
client_panorama.dll!0xcd8858 ConVar cl_skipfastpath
client_panorama.dll!0xce10a0 ConVar cl_skipslowpath
client_panorama.dll!0xce2e08 ConVar cl_spec_follow_grenade_key
client_panorama.dll!0xce4080 ConVar cl_spec_mode
client_panorama.dll!0xce2728 ConVar cl_spec_show_bindings
client_panorama.dll!0xcef838 ConVar cl_spec_stats
client_panorama.dll!0xcebb08 ConVar cl_spec_use_tournament_content_standards
client_panorama.dll!0xcf6000 ConVar cl_sporeclipdistance
client_panorama.dll!0xcd2f28 ConVar cl_sun_decay_rate
client_panorama.dll!0xcd2f80 ConVar cl_sun_in_reflection_h_scale
client_panorama.dll!0xcd2fd8 ConVar cl_sun_in_reflection_v_scale
client_panorama.dll!0xcceff8 ConVar cl_sunlight_ortho_size
client_panorama.dll!0xcf2e58 ConVar cl_tablet_mapmode
client_panorama.dll!0xce2990 ConVar cl_teamid_overhead_maxdist
client_panorama.dll!0xce29e8 ConVar cl_teamid_overhead_maxdist_spec
client_panorama.dll!0xce2b48 ConVar cl_teammate_colors_show
client_panorama.dll!0xccb0b0 ConVar cl_threaded_bone_setup
client_panorama.dll!0xcd6b38 ConVar cl_upspeed
client_panorama.dll!0xcf2268 ConVar cl_use_new_headbob
client_panorama.dll!0xce9240 ConVar cl_use_opens_buy_menu
client_panorama.dll!0xcf6df0 ConVar cl_use_simd_bones
client_panorama.dll!0xcf1d40 ConVar cl_viewmodel_shift_left_amt
client_panorama.dll!0xcf1d98 ConVar cl_viewmodel_shift_right_amt
client_panorama.dll!0xce32a0 ConVar cl_weapon_clip_thinwalls
client_panorama.dll!0xce32f8 ConVar cl_weapon_clip_thinwalls_debug
client_panorama.dll!0xce3350 ConVar cl_weapon_clip_thinwalls_lock
client_panorama.dll!0xcf1fa8 ConVar cl_weapon_debug_print_accuracy
client_panorama.dll!0xcf1ef8 ConVar cl_weapon_debug_show_accuracy
client_panorama.dll!0xcf1f50 ConVar cl_weapon_debug_show_accuracy_duration
client_panorama.dll!0xcf4fd0 ConVar cl_winddir
client_panorama.dll!0xcf5028 ConVar cl_windspeed
client_panorama.dll!0xcda868 ConVar cl_wpn_sway_scale
client_panorama.dll!0xcd3988 ConVar closecaption
client_panorama.dll!0xce92f0 ConVar closeonbuy
client_panorama.dll!0xcf72a0 ConVar cloth_windage_multiplier
client_panorama.dll!0xced410 ConVar commentary_firstrun
client_panorama.dll!0xce47b8 ConVar crosshair
client_panorama.dll!0xcecc00 ConVar custom_bot_difficulty
client_panorama.dll!0xceca20 ConVar debug_aim_angle
client_panorama.dll!0xce2678 ConVar debug_entity_outline_highlight
client_panorama.dll!0xce4760 ConVar default_fov
client_panorama.dll!0xcdc408 ConVar developer
client_panorama.dll!0xccaf30 ConVar enable_skeleton_draw
client_panorama.dll!0xceb880 ConVar ff_damage_bullet_penetration
client_panorama.dll!0xceb7d0 ConVar ff_damage_reduction_bullets
client_panorama.dll!0xceb720 ConVar ff_damage_reduction_grenade
client_panorama.dll!0xceb778 ConVar ff_damage_reduction_grenade_self
client_panorama.dll!0xceb828 ConVar ff_damage_reduction_other
client_panorama.dll!0xccd6a8 ConVar fish_debug
client_panorama.dll!0xce0238 ConVar fog_color
client_panorama.dll!0xce03f0 ConVar fog_colorskybox
client_panorama.dll!0xce0290 ConVar fog_enable
client_panorama.dll!0xce0448 ConVar fog_enableskybox
client_panorama.dll!0xce01e0 ConVar fog_end
client_panorama.dll!0xce0340 ConVar fog_endskybox
client_panorama.dll!0xce04f8 ConVar fog_hdrcolorscale
client_panorama.dll!0xce0550 ConVar fog_hdrcolorscaleskybox
client_panorama.dll!0xce04a0 ConVar fog_maxdensity
client_panorama.dll!0xce0398 ConVar fog_maxdensityskybox
client_panorama.dll!0xce0188 ConVar fog_start
client_panorama.dll!0xce02e8 ConVar fog_startskybox
client_panorama.dll!0xce3a98 ConVar fov_cs_debug
client_panorama.dll!0xcf68a8 ConVar func_break_max_pieces
client_panorama.dll!0xcd3b00 ConVar g15_update_msec
client_panorama.dll!0xcd29b8 ConVar g_Language
client_panorama.dll!0xcdb1e8 ConVar g_debug_ragdoll_removal
client_panorama.dll!0xccafa8 ConVar g_debug_ragdoll_visualize
client_panorama.dll!0xcdb240 ConVar g_ragdoll_important_maxcount
client_panorama.dll!0xcdb190 ConVar g_ragdoll_maxcount
client_panorama.dll!0xceccb0 ConVar game_mode
client_panorama.dll!0xcecc58 ConVar game_online
client_panorama.dll!0xcecba8 ConVar game_public
client_panorama.dll!0xcecd08 ConVar game_type
client_panorama.dll!0xccdb98 ConVar gameinstructor_find_errors
client_panorama.dll!0xccd908 ConVar gameinstructor_save_restore_lessons
client_panorama.dll!0xccdae8 ConVar gameinstructor_verbose
client_panorama.dll!0xccdb40 ConVar gameinstructor_verbose_lesson
client_panorama.dll!0xcdd3b8 ConVar gl_clear_randomcolor
client_panorama.dll!0xcd2e68 ConVar glow_muzzle_debug
client_panorama.dll!0xcd2db8 ConVar glow_outline_effect_enable
client_panorama.dll!0xcd2e10 ConVar glow_outline_width
client_panorama.dll!0xcef7e0 ConVar gotv_theater_container
client_panorama.dll!0xcf0a38 ConVar healthshot_allow_use_at_full
client_panorama.dll!0xcf0988 ConVar healthshot_health
client_panorama.dll!0xcf0a90 ConVar healthshot_healthboost_speed_multiplier
client_panorama.dll!0xcf09e0 ConVar healthshot_healthboost_time
client_panorama.dll!0xcd3320 ConVar hidehud
client_panorama.dll!0xcca208 ConVar hl2_episodic
client_panorama.dll!0xcebc68 ConVar hostage_feetyawrate
client_panorama.dll!0xce1740 ConVar hud_fastswitch
client_panorama.dll!0xceeaa0 ConVar hud_scaling
client_panorama.dll!0xce3df0 ConVar hud_showtargetid
client_panorama.dll!0xccfe08 ConVar hud_takesshots
client_panorama.dll!0xcd6df8 ConVar in_forceuser
client_panorama.dll!0xcece08 ConVar inferno_dlight_spacing
client_panorama.dll!0xcd5900 ConVar joy_accelmax
client_panorama.dll!0xcd5850 ConVar joy_accelscale
client_panorama.dll!0xcd58a8 ConVar joy_accelscalepoly
client_panorama.dll!0xcd5068 ConVar joy_advanced
client_panorama.dll!0xcd51c8 ConVar joy_advaxisr
client_panorama.dll!0xcd5220 ConVar joy_advaxisu
client_panorama.dll!0xcd5278 ConVar joy_advaxisv
client_panorama.dll!0xcd50c0 ConVar joy_advaxisx
client_panorama.dll!0xcd5118 ConVar joy_advaxisy
client_panorama.dll!0xcd5170 ConVar joy_advaxisz
client_panorama.dll!0xcd5958 ConVar joy_autoAimDampenMethod
client_panorama.dll!0xcd5a08 ConVar joy_autoaimdampen
client_panorama.dll!0xcd59b0 ConVar joy_autoaimdampenrange
client_panorama.dll!0xcd6140 ConVar joy_cfg_preset
client_panorama.dll!0xcd5e28 ConVar joy_circle_correct
client_panorama.dll!0xcd5ab8 ConVar joy_curvepoint_1
client_panorama.dll!0xcd5b10 ConVar joy_curvepoint_2
client_panorama.dll!0xcd5b68 ConVar joy_curvepoint_3
client_panorama.dll!0xcd5bc0 ConVar joy_curvepoint_4
client_panorama.dll!0xcd5c18 ConVar joy_curvepoint_end
client_panorama.dll!0xcd5e80 ConVar joy_diagonalpov
client_panorama.dll!0xcd5ed8 ConVar joy_display_input
client_panorama.dll!0xcd5430 ConVar joy_forwardsensitivity
client_panorama.dll!0xcd52d0 ConVar joy_forwardthreshold
client_panorama.dll!0xcd57f8 ConVar joy_gamma
client_panorama.dll!0xcd5f88 ConVar joy_inverty
client_panorama.dll!0xcd56f0 ConVar joy_lowend
client_panorama.dll!0xcd5748 ConVar joy_lowend_linear
client_panorama.dll!0xcd57a0 ConVar joy_lowmap
client_panorama.dll!0xcd5010 ConVar joy_name
client_panorama.dll!0xcd6248 ConVar joy_no_accel_jump
client_panorama.dll!0xcd54e0 ConVar joy_pitchsensitivity
client_panorama.dll!0xcd5380 ConVar joy_pitchthreshold
client_panorama.dll!0xcd5640 ConVar joy_response_look
client_panorama.dll!0xcd5698 ConVar joy_response_look_pitch
client_panorama.dll!0xcd5590 ConVar joy_response_move
client_panorama.dll!0xcd5d20 ConVar joy_sensitive_step0
client_panorama.dll!0xcd5d78 ConVar joy_sensitive_step1
client_panorama.dll!0xcd5dd0 ConVar joy_sensitive_step2
client_panorama.dll!0xcd5488 ConVar joy_sidesensitivity
client_panorama.dll!0xcd5328 ConVar joy_sidethreshold
client_panorama.dll!0xcd5f30 ConVar joy_wingmanwarrior_turnhack
client_panorama.dll!0xcd5538 ConVar joy_yawsensitivity
client_panorama.dll!0xcd53d8 ConVar joy_yawthreshold
client_panorama.dll!0xcd4fb8 ConVar joystick_force_disabled
client_panorama.dll!0xcf0288 ConVar key_bind_version
client_panorama.dll!0xcd4298 ConVar locator_split_len
client_panorama.dll!0xcd4240 ConVar locator_split_maxwide_percent
client_panorama.dll!0xce3d98 ConVar lockMoveControllerRet
client_panorama.dll!0xcd7ba0 ConVar lookspring
client_panorama.dll!0xcd7a98 ConVar lookstrafe
client_panorama.dll!0xcd7db0 ConVar m_customaccel
client_panorama.dll!0xcd8018 ConVar m_customaccel_exponent
client_panorama.dll!0xcd7fc0 ConVar m_customaccel_max
client_panorama.dll!0xcd8070 ConVar m_customaccel_scale
client_panorama.dll!0xcd8228 ConVar m_forward
client_panorama.dll!0xcd7f68 ConVar m_mouseaccel1
client_panorama.dll!0xcd81d0 ConVar m_mouseaccel2
client_panorama.dll!0xcd82d8 ConVar m_mousespeed
client_panorama.dll!0xcd7e08 ConVar m_rawinput
client_panorama.dll!0xcd80c8 ConVar m_side
client_panorama.dll!0xcd7f10 ConVar m_yaw
client_panorama.dll!0xcd9490 ConVar mapcycledisabled
client_panorama.dll!0xcd0018 ConVar mapoverview_allow_client_draw
client_panorama.dll!0xcd0070 ConVar mapoverview_allow_grid_usage
client_panorama.dll!0xcd00c8 ConVar mapoverview_icon_scale
client_panorama.dll!0xcde768 ConVar mat_accelerate_adjust_exposure_down
client_panorama.dll!0xcde450 ConVar mat_autoexposure_max
client_panorama.dll!0xcde4a8 ConVar mat_autoexposure_max_multiplier
client_panorama.dll!0xcde500 ConVar mat_autoexposure_min
client_panorama.dll!0xcde818 ConVar mat_bloom_scalefactor_scalar
client_panorama.dll!0xcde2f0 ConVar mat_bloomamount_rate
client_panorama.dll!0xcde240 ConVar mat_bloomscale
client_panorama.dll!0xcddd48 ConVar mat_camerarendertargetoverlaysize
client_panorama.dll!0xccc3a8 ConVar mat_colcorrection_forceentitiesclientside
client_panorama.dll!0xcde710 ConVar mat_colorcorrection
client_panorama.dll!0xcde6b8 ConVar mat_debug_bloom
client_panorama.dll!0xcde348 ConVar mat_debug_postprocessing_effects
client_panorama.dll!0xcde660 ConVar mat_disable_bloom
client_panorama.dll!0xce1150 ConVar mat_draw_zone_highlight
client_panorama.dll!0xce10f8 ConVar mat_draw_zone_projection_mode
client_panorama.dll!0xce07b8 ConVar mat_drawwater
client_panorama.dll!0xcde3a0 ConVar mat_dynamic_tonemapping
client_panorama.dll!0xcde870 ConVar mat_exposure_center_region_x
client_panorama.dll!0xcde8c8 ConVar mat_exposure_center_region_y
client_panorama.dll!0xcde608 ConVar mat_force_bloom
client_panorama.dll!0xcdea28 ConVar mat_force_tonemap_min_avglum
client_panorama.dll!0xcde9d0 ConVar mat_force_tonemap_percent_bright_pixels
client_panorama.dll!0xcde978 ConVar mat_force_tonemap_percent_target
client_panorama.dll!0xcdea80 ConVar mat_force_tonemap_scale
client_panorama.dll!0xcce5c0 ConVar mat_fullbright
client_panorama.dll!0xcd1598 ConVar mat_fullbright
client_panorama.dll!0xcdead8 ConVar mat_fullbright
client_panorama.dll!0xcde5b0 ConVar mat_hdr_uncapexposure
client_panorama.dll!0xcddda0 ConVar mat_hsv
client_panorama.dll!0xce0e80 ConVar mat_lpreview_mode
client_panorama.dll!0xcde7c0 ConVar mat_non_hdr_bloom_scalefactor
client_panorama.dll!0xcf4ce0 ConVar mat_normals
client_panorama.dll!0xcf7878 ConVar mat_normals
client_panorama.dll!0xcdf2f0 ConVar mat_postprocess_enable
client_panorama.dll!0xccc2c0 ConVar mat_preview
client_panorama.dll!0xcde558 ConVar mat_show_histogram
client_panorama.dll!0xcddcf0 ConVar mat_showcamerarendertarget
client_panorama.dll!0xcddc40 ConVar mat_showframebuffertexture
client_panorama.dll!0xcddb90 ConVar mat_showwatertextures
client_panorama.dll!0xcf4970 ConVar mat_softwareskin
client_panorama.dll!0xcf7610 ConVar mat_softwareskin
client_panorama.dll!0xccf8e0 ConVar mat_stub
client_panorama.dll!0xcde920 ConVar mat_tonemap_algorithm
client_panorama.dll!0xcdd258 ConVar mat_viewportscale
client_panorama.dll!0xcdd2b0 ConVar mat_viewportupscale
client_panorama.dll!0xcde1e8 ConVar mat_wireframe
client_panorama.dll!0xcdddf8 ConVar mat_yuv
client_panorama.dll!0xcd62f8 ConVar mc_accel_band_size
client_panorama.dll!0xcd62a0 ConVar mc_dead_zone_radius
client_panorama.dll!0xcd63a8 ConVar mc_max_pitchrate
client_panorama.dll!0xcd6350 ConVar mc_max_yawrate
client_panorama.dll!0xcf0ae8 ConVar molotov_throw_detonate_time
client_panorama.dll!0xce67f8 ConVar mp_afterroundmoney
client_panorama.dll!0xcd2ba8 ConVar mp_allowspectators
client_panorama.dll!0xce6b10 ConVar mp_anyone_can_pickup_c4
client_panorama.dll!0xcdbdb8 ConVar mp_blockstyle
client_panorama.dll!0xcdc020 ConVar mp_bonusroundtime
client_panorama.dll!0xce65e8 ConVar mp_buy_allow_grenades
client_panorama.dll!0xce6640 ConVar mp_buy_allow_guns
client_panorama.dll!0xcd2cb0 ConVar mp_buy_anywhere
client_panorama.dll!0xcd2d08 ConVar mp_buy_during_immunity
client_panorama.dll!0xce6590 ConVar mp_buytime
client_panorama.dll!0xce6b68 ConVar mp_c4_cannot_be_defused
client_panorama.dll!0xcea7b0 ConVar mp_c4timer
client_panorama.dll!0xcdc0d0 ConVar mp_capdeteriorate_time
client_panorama.dll!0xcdc078 ConVar mp_capstyle
client_panorama.dll!0xce8168 ConVar mp_coop_force_join_ct
client_panorama.dll!0xce81c0 ConVar mp_coopmission_mission_number
client_panorama.dll!0xce74b0 ConVar mp_ct_default_grenades
client_panorama.dll!0xce73a8 ConVar mp_ct_default_melee
client_panorama.dll!0xce7458 ConVar mp_ct_default_primary
client_panorama.dll!0xce7400 ConVar mp_ct_default_secondary
client_panorama.dll!0xce80b8 ConVar mp_death_drop_breachcharge
client_panorama.dll!0xce7f58 ConVar mp_death_drop_c4
client_panorama.dll!0xce8008 ConVar mp_death_drop_defuser
client_panorama.dll!0xce7fb0 ConVar mp_death_drop_grenade
client_panorama.dll!0xce7f00 ConVar mp_death_drop_gun
client_panorama.dll!0xce8110 ConVar mp_death_drop_healthshot
client_panorama.dll!0xce8060 ConVar mp_death_drop_taser
client_panorama.dll!0xce86e8 ConVar mp_default_team_winner_no_objective
client_panorama.dll!0xce7e50 ConVar mp_defuser_allocation
client_panorama.dll!0xcdbfc8 ConVar mp_disable_respawn_times
client_panorama.dll!0xce7d48 ConVar mp_display_kill_assists
client_panorama.dll!0xce7a88 ConVar mp_dm_bonus_percent
client_panorama.dll!0xce7ae0 ConVar mp_dm_bonus_respawn
client_panorama.dll!0xce7cf0 ConVar mp_dm_bonusweapon_dogtags
client_panorama.dll!0xce7b38 ConVar mp_dm_dogtag_score
client_panorama.dll!0xce7a30 ConVar mp_dm_kill_base_score
client_panorama.dll!0xce7b90 ConVar mp_dm_teammode
client_panorama.dll!0xce7c40 ConVar mp_dm_teammode_bonus_score
client_panorama.dll!0xce7c98 ConVar mp_dm_teammode_dogtag_score
client_panorama.dll!0xce7be8 ConVar mp_dm_teammode_kill_score
client_panorama.dll!0xce66f0 ConVar mp_do_warmup_offine
client_panorama.dll!0xce6698 ConVar mp_do_warmup_period
client_panorama.dll!0xce8378 ConVar mp_economy_reset_rounds
client_panorama.dll!0xce8950 ConVar mp_endmatch_votenextleveltime
client_panorama.dll!0xce88a0 ConVar mp_endmatch_votenextmap
client_panorama.dll!0xce88f8 ConVar mp_endmatch_votenextmap_keepcurrent
client_panorama.dll!0xce6e28 ConVar mp_endwarmup_player_count
client_panorama.dll!0xce8320 ConVar mp_equipment_reset_rounds
client_panorama.dll!0xcca098 ConVar mp_facefronttime
client_panorama.dll!0xcca040 ConVar mp_feetyawrate
client_panorama.dll!0xce6488 ConVar mp_force_assign_teams
client_panorama.dll!0xce8218 ConVar mp_force_pick_time
client_panorama.dll!0xcd2d60 ConVar mp_forcecamera
client_panorama.dll!0xcd9540 ConVar mp_fraglimit
client_panorama.dll!0xcea498 ConVar mp_free_armor
client_panorama.dll!0xcd2c00 ConVar mp_friendlyfire
client_panorama.dll!0xce72f8 ConVar mp_ggprogressive_random_weapon_kills_needed
client_panorama.dll!0xce7248 ConVar mp_ggprogressive_round_restart_delay
client_panorama.dll!0xce72a0 ConVar mp_ggprogressive_use_random_weapons
client_panorama.dll!0xce7140 ConVar mp_ggtr_always_upgrade
client_panorama.dll!0xce7980 ConVar mp_ggtr_bomb_defuse_bonus
client_panorama.dll!0xce79d8 ConVar mp_ggtr_bomb_detonation_bonus
client_panorama.dll!0xce77c8 ConVar mp_ggtr_bomb_pts_for_flash
client_panorama.dll!0xce7770 ConVar mp_ggtr_bomb_pts_for_he
client_panorama.dll!0xce7820 ConVar mp_ggtr_bomb_pts_for_molotov
client_panorama.dll!0xce7718 ConVar mp_ggtr_bomb_pts_for_upgrade
client_panorama.dll!0xce7928 ConVar mp_ggtr_bomb_respawn_delay
client_panorama.dll!0xce7198 ConVar mp_ggtr_end_round_kill_bonus
client_panorama.dll!0xce78d0 ConVar mp_ggtr_halftime_delay
client_panorama.dll!0xce71f0 ConVar mp_ggtr_last_weapon_kill_ends_half
client_panorama.dll!0xce7350 ConVar mp_ggtr_num_rounds_autoprogress
client_panorama.dll!0xce7ea8 ConVar mp_give_player_c4
client_panorama.dll!0xcea548 ConVar mp_halftime
client_panorama.dll!0xce7090 ConVar mp_halftime_duration
client_panorama.dll!0xce6f30 ConVar mp_halftime_pausematch
client_panorama.dll!0xce6ed8 ConVar mp_halftime_pausetimer
client_panorama.dll!0xce5618 ConVar mp_heavyassaultsuit_aimpunch
client_panorama.dll!0xce85e0 ConVar mp_heavyassaultsuit_cooldown
client_panorama.dll!0xce55c0 ConVar mp_heavyassaultsuit_deploy_timescale
client_panorama.dll!0xce5510 ConVar mp_heavyassaultsuit_speed
client_panorama.dll!0xce5568 ConVar mp_heavybot_damage_reduction_scale
client_panorama.dll!0xce6ab8 ConVar mp_hostages_rescuetime
client_panorama.dll!0xce6a60 ConVar mp_hostages_rescuetowin
client_panorama.dll!0xce6a08 ConVar mp_hostages_takedamage
client_panorama.dll!0xcca0f0 ConVar mp_ik
client_panorama.dll!0xce7668 ConVar mp_join_grace_time
client_panorama.dll!0xce70e8 ConVar mp_match_can_clinch
client_panorama.dll!0xce7df8 ConVar mp_match_end_changelevel
client_panorama.dll!0xce7da0 ConVar mp_match_end_restart
client_panorama.dll!0xcd9438 ConVar mp_match_restart_delay
client_panorama.dll!0xcea4f0 ConVar mp_max_armor
client_panorama.dll!0xce67a0 ConVar mp_maxmoney
client_panorama.dll!0xcdbf18 ConVar mp_maxrounds
client_panorama.dll!0xce7878 ConVar mp_molotovusedelay
client_panorama.dll!0xce6900 ConVar mp_overtime_enable
client_panorama.dll!0xce6f88 ConVar mp_overtime_halftime_pausetimer
client_panorama.dll!0xce6958 ConVar mp_overtime_maxrounds
client_panorama.dll!0xce69b0 ConVar mp_overtime_startmoney
client_panorama.dll!0xcf0ec8 ConVar mp_plant_c4_anywhere
client_panorama.dll!0xce6850 ConVar mp_playercashawards
client_panorama.dll!0xcea910 ConVar mp_playerid
client_panorama.dll!0xcea968 ConVar mp_playerid_delay
client_panorama.dll!0xcea9c0 ConVar mp_playerid_hold
client_panorama.dll!0xcd2b50 ConVar mp_radar_showall
client_panorama.dll!0xcea5a0 ConVar mp_randomspawn
client_panorama.dll!0xcea650 ConVar mp_randomspawn_dist
client_panorama.dll!0xcea5f8 ConVar mp_randomspawn_los
client_panorama.dll!0xce6fe0 ConVar mp_respawn_immunitytime
client_panorama.dll!0xce87f0 ConVar mp_respawn_on_death_ct
client_panorama.dll!0xce8798 ConVar mp_respawn_on_death_t
client_panorama.dll!0xcdbe10 ConVar mp_respawnwavetime
client_panorama.dll!0xce7038 ConVar mp_round_restart_delay
client_panorama.dll!0xceab20 ConVar mp_solid_teammates
client_panorama.dll!0xce6430 ConVar mp_spec_swapplayersides
client_panorama.dll!0xce6538 ConVar mp_spectators_max
client_panorama.dll!0xce6748 ConVar mp_startmoney
client_panorama.dll!0xce7610 ConVar mp_t_default_grenades
client_panorama.dll!0xce7508 ConVar mp_t_default_melee
client_panorama.dll!0xce75b8 ConVar mp_t_default_primary
client_panorama.dll!0xce7560 ConVar mp_t_default_secondary
client_panorama.dll!0xcf2eb0 ConVar mp_taser_recharge_time
client_panorama.dll!0xce5348 ConVar mp_team_timeout_max
client_panorama.dll!0xce52f0 ConVar mp_team_timeout_time
client_panorama.dll!0xce68a8 ConVar mp_teamcashawards
client_panorama.dll!0xcd2c58 ConVar mp_teammates_are_enemies
client_panorama.dll!0xcdbec0 ConVar mp_teams_unbalance_limit
client_panorama.dll!0xcd94e8 ConVar mp_timelimit
client_panorama.dll!0xcdbe68 ConVar mp_tournament
client_panorama.dll!0xcea8b8 ConVar mp_use_respawn_waves
client_panorama.dll!0xce6d20 ConVar mp_verbose_changelevel_spew
client_panorama.dll!0xce6e80 ConVar mp_warmup_pausetimer
client_panorama.dll!0xce6dd0 ConVar mp_warmuptime_all_players_connected
client_panorama.dll!0xce84d8 ConVar mp_weapons_allow_heavy
client_panorama.dll!0xce8690 ConVar mp_weapons_allow_map_placed
client_panorama.dll!0xce8428 ConVar mp_weapons_allow_pistols
client_panorama.dll!0xce8530 ConVar mp_weapons_allow_rifles
client_panorama.dll!0xce8480 ConVar mp_weapons_allow_smgs
client_panorama.dll!0xce8638 ConVar mp_weapons_allow_typecount
client_panorama.dll!0xce83d0 ConVar mp_weapons_allow_zeus
client_panorama.dll!0xce8740 ConVar mp_weapons_glow_on_ground
client_panorama.dll!0xce53a0 ConVar mp_weapons_max_gun_purchases_per_weapon_per_match
client_panorama.dll!0xce76c0 ConVar mp_win_panel_display_time
client_panorama.dll!0xcdbf70 ConVar mp_winlimit
client_panorama.dll!0xcf6c98 ConVar muzzleflash_light
client_panorama.dll!0xceb1a8 ConVar net_client_steamdatagram_enable_override
client_panorama.dll!0xcdcdc0 ConVar net_graphholdsvframerate
client_panorama.dll!0xcdce70 ConVar net_graphipc
client_panorama.dll!0xcdcc60 ConVar net_graphmsecs
client_panorama.dll!0xcdcb58 ConVar net_graphpos
client_panorama.dll!0xcdcd10 ConVar net_graphshowinterp
client_panorama.dll!0xcdccb8 ConVar net_graphshowlatency
client_panorama.dll!0xcdcd68 ConVar net_graphshowsvframerate
client_panorama.dll!0xcdcbb0 ConVar net_graphsolid
client_panorama.dll!0xcdcc08 ConVar net_graphtext
client_panorama.dll!0xcdcb00 ConVar net_scale
client_panorama.dll!0xcd9598 ConVar nextlevel
client_panorama.dll!0xcd95f0 ConVar nextmode
client_panorama.dll!0xcd2af0 ConVar old_radiusdamage
client_panorama.dll!0xcd6c98 ConVar option_duck_method
client_panorama.dll!0xcd6cf0 ConVar option_speed_method
client_panorama.dll!0xcfa768 ConVar panel_test_title_safe
client_panorama.dll!0xcd9ac0 ConVar particle_simulateoverflow
client_panorama.dll!0xcda1f8 ConVar phys_debug_check_contacts
client_panorama.dll!0xcdbd40 ConVar phys_pushscale
client_panorama.dll!0xcf0180 ConVar player_botdifflast_s
client_panorama.dll!0xced668 ConVar player_nevershow_communityservermessage
client_panorama.dll!0xce43f0 ConVar player_teamplayedlast
client_panorama.dll!0xcebc10 ConVar post_jump_crouch
client_panorama.dll!0xcdaf58 ConVar props_break_max_pieces
client_panorama.dll!0xcdafb0 ConVar props_break_max_pieces_perframe
client_panorama.dll!0xcdade8 ConVar pwatchent
client_panorama.dll!0xcdae40 ConVar pwatchvar
client_panorama.dll!0xcd8be8 ConVar r_AirboatViewDampenDamp
client_panorama.dll!0xcd8b90 ConVar r_AirboatViewDampenFreq
client_panorama.dll!0xcd8c40 ConVar r_AirboatViewZHeight
client_panorama.dll!0xcdd838 ConVar r_DrawBeams
client_panorama.dll!0xcd87a8 ConVar r_DrawModelLightOrigin
client_panorama.dll!0xcf5340 ConVar r_DrawRain
client_panorama.dll!0xccf1f8 ConVar r_JeepViewBlendTo
client_panorama.dll!0xccf250 ConVar r_JeepViewBlendToScale
client_panorama.dll!0xccf2a8 ConVar r_JeepViewBlendToTime
client_panorama.dll!0xcd9110 ConVar r_JeepViewDampenDamp
client_panorama.dll!0xcd9168 ConVar r_JeepViewDampenFreq
client_panorama.dll!0xcd91c0 ConVar r_JeepViewZHeight
client_panorama.dll!0xccf9f8 ConVar r_PortalTestEnts
client_panorama.dll!0xcf5290 ConVar r_RainCheck
client_panorama.dll!0xcf53f0 ConVar r_RainDebugDuration
client_panorama.dll!0xcf5188 ConVar r_RainHack
client_panorama.dll!0xcf5398 ConVar r_RainProfile
client_panorama.dll!0xcf51e0 ConVar r_RainRadius
client_panorama.dll!0xcf5238 ConVar r_RainSideVel
client_panorama.dll!0xcf52e8 ConVar r_RainSimulate
client_panorama.dll!0xcf5080 ConVar r_RainSplashPercentage
client_panorama.dll!0xcf5750 ConVar r_SnowDebugBox
client_panorama.dll!0xcf5490 ConVar r_SnowEnable
client_panorama.dll!0xcf58b0 ConVar r_SnowEndAlpha
client_panorama.dll!0xcf5960 ConVar r_SnowEndSize
client_panorama.dll!0xcf56a0 ConVar r_SnowFallSpeed
client_panorama.dll!0xcf5540 ConVar r_SnowInsideRadius
client_panorama.dll!0xcf5598 ConVar r_SnowOutsideRadius
client_panorama.dll!0xcf54e8 ConVar r_SnowParticles
client_panorama.dll!0xcf5648 ConVar r_SnowPosScale
client_panorama.dll!0xcf5a68 ConVar r_SnowRayEnable
client_panorama.dll!0xcf59b8 ConVar r_SnowRayLength
client_panorama.dll!0xcf5a10 ConVar r_SnowRayRadius
client_panorama.dll!0xcf55f0 ConVar r_SnowSpeedScale
client_panorama.dll!0xcf5858 ConVar r_SnowStartAlpha
client_panorama.dll!0xcf5908 ConVar r_SnowStartSize
client_panorama.dll!0xcf56f8 ConVar r_SnowWindScale
client_panorama.dll!0xcf57a8 ConVar r_SnowZoomOffset
client_panorama.dll!0xcf5800 ConVar r_SnowZoomRadius
client_panorama.dll!0xcf5f40 ConVar r_VehicleViewClamp
client_panorama.dll!0xcd8b38 ConVar r_VehicleViewDampen
client_panorama.dll!0xccfb58 ConVar r_alphafade_usefov
client_panorama.dll!0xce05a8 ConVar r_debugcheapwater
client_panorama.dll!0xcdd9b0 ConVar r_depthoverlay
client_panorama.dll!0xccfbf8 ConVar r_disable_distance_fade_on_big_props
client_panorama.dll!0xccfc50 ConVar r_disable_distance_fade_on_big_props_thresh
client_panorama.dll!0xcd0288 ConVar r_disable_update_shadow
client_panorama.dll!0xccfca8 ConVar r_drawallrenderables
client_panorama.dll!0xce0ff0 ConVar r_drawbrushmodels
client_panorama.dll!0xcf4ad0 ConVar r_drawentities
client_panorama.dll!0xcf74b0 ConVar r_drawentities
client_panorama.dll!0xccb160 ConVar r_drawmodelnames
client_panorama.dll!0xccb1b8 ConVar r_drawmodelstatsoverlay
client_panorama.dll!0xcdffd0 ConVar r_drawopaquedetailprops
client_panorama.dll!0xce0028 ConVar r_drawopaquedetailprops_csm
client_panorama.dll!0xce0080 ConVar r_drawopaquedetailprops_reflect
client_panorama.dll!0xce00d8 ConVar r_drawopaquedetailprops_refract
client_panorama.dll!0xcdfdc0 ConVar r_drawopaquerenderables
client_panorama.dll!0xcdfbb0 ConVar r_drawopaqueworld
client_panorama.dll!0xccb108 ConVar r_drawothermodels
client_panorama.dll!0xcd9a68 ConVar r_drawparticles
client_panorama.dll!0xccb438 ConVar r_drawrenderboxes
client_panorama.dll!0xcce618 ConVar r_drawropes
client_panorama.dll!0xcdff78 ConVar r_drawscreenoverlay
client_panorama.dll!0xccef68 ConVar r_drawsprites
client_panorama.dll!0xcd2218 ConVar r_drawtracers
client_panorama.dll!0xcd2270 ConVar r_drawtracers_firstperson
client_panorama.dll!0xcd22c8 ConVar r_drawtracers_movetonotintersect
client_panorama.dll!0xcdfd68 ConVar r_drawtranslucentrenderables
client_panorama.dll!0xcdfc08 ConVar r_drawtranslucentworld
client_panorama.dll!0xcdff20 ConVar r_drawunderwateroverlay
client_panorama.dll!0xce1308 ConVar r_drawviewmodel
client_panorama.dll!0xcf4760 ConVar r_eyegloss
client_panorama.dll!0xcf7928 ConVar r_eyegloss
client_panorama.dll!0xcf47b8 ConVar r_eyemove
client_panorama.dll!0xcf7508 ConVar r_eyemove
client_panorama.dll!0xcf4810 ConVar r_eyeshift_x
client_panorama.dll!0xcf75b8 ConVar r_eyeshift_x
client_panorama.dll!0xcf7400 ConVar r_eyeshift_y
client_panorama.dll!0xcf4868 ConVar r_eyeshift_y
client_panorama.dll!0xcf48c0 ConVar r_eyeshift_z
client_panorama.dll!0xcf7718 ConVar r_eyeshift_z
client_panorama.dll!0xcf4918 ConVar r_eyesize
client_panorama.dll!0xcf7458 ConVar r_eyesize
client_panorama.dll!0xce08c0 ConVar r_eyewaterepsilon
client_panorama.dll!0xcdd410 ConVar r_farz
client_panorama.dll!0xcd1bd8 ConVar r_flashlightambient
client_panorama.dll!0xcd1de8 ConVar r_flashlightbacktraceoffset
client_panorama.dll!0xcd1a78 ConVar r_flashlightconstant
client_panorama.dll!0xcd1a20 ConVar r_flashlightfar
client_panorama.dll!0xcd1868 ConVar r_flashlightfov
client_panorama.dll!0xcd1c88 ConVar r_flashlightladderdist
client_panorama.dll!0xcd1ad0 ConVar r_flashlightlinear
client_panorama.dll!0xcd1810 ConVar r_flashlightlockposition
client_panorama.dll!0xcd1e40 ConVar r_flashlightmuzzleflashfov
client_panorama.dll!0xcd19c8 ConVar r_flashlightnear
client_panorama.dll!0xcd1d38 ConVar r_flashlightnearoffsetscale
client_panorama.dll!0xcd1970 ConVar r_flashlightoffsetforward
client_panorama.dll!0xcd18c0 ConVar r_flashlightoffsetright
client_panorama.dll!0xcd1918 ConVar r_flashlightoffsetup
client_panorama.dll!0xcd1b28 ConVar r_flashlightquadratic
client_panorama.dll!0xcd1c30 ConVar r_flashlightshadowatten
client_panorama.dll!0xcd1b80 ConVar r_flashlightvisualizetrace
client_panorama.dll!0xcdd768 ConVar r_mapextents
client_panorama.dll!0xcf7770 ConVar r_modelwireframedecal
client_panorama.dll!0xcf4c88 ConVar r_modelwireframedecal
client_panorama.dll!0xcf73a8 ConVar r_nohw
client_panorama.dll!0xcf49c8 ConVar r_nohw
client_panorama.dll!0xcf7820 ConVar r_nosw
client_panorama.dll!0xcf4a20 ConVar r_nosw
client_panorama.dll!0xce0f30 ConVar r_particle_demo
client_panorama.dll!0xccfa50 ConVar r_portalsopenall
client_panorama.dll!0xcf5c20 ConVar r_rainalpha
client_panorama.dll!0xcf5c78 ConVar r_rainalphapow
client_panorama.dll!0xcf5ac0 ConVar r_raindensity
client_panorama.dll!0xcf5b70 ConVar r_rainlength
client_panorama.dll!0xcf5bc8 ConVar r_rainspeed
client_panorama.dll!0xcf5b18 ConVar r_rainwidth
client_panorama.dll!0xcedc10 ConVar r_replay_post_effect
client_panorama.dll!0xcd0700 ConVar r_shadow_debug_spew
client_panorama.dll!0xcd04f0 ConVar r_shadowfromanyworldlight
client_panorama.dll!0xcd0498 ConVar r_shadowfromworldlights_debug
client_panorama.dll!0xcd0910 ConVar r_shadows_gamecontrol
client_panorama.dll!0xcf7350 ConVar r_showenvcubemap
client_panorama.dll!0xcf4708 ConVar r_showenvcubemap
client_panorama.dll!0xcf4bd8 ConVar r_skin
client_panorama.dll!0xcf79d8 ConVar r_skin
client_panorama.dll!0xcdfd10 ConVar r_skybox
client_panorama.dll!0xcd17b8 ConVar r_swingflashlight
client_panorama.dll!0xcdfec8 ConVar r_underwateroverlay_drain_speed
client_panorama.dll!0xcdda08 ConVar r_updaterefracttexture
client_panorama.dll!0xcdfa50 ConVar r_visocclusion
client_panorama.dll!0xcdc3b0 ConVar r_visualizetraces
client_panorama.dll!0xccb280 ConVar report_cliententitysim
client_panorama.dll!0xccf938 ConVar report_clientthinklist
client_panorama.dll!0xcce300 ConVar rope_subdiv
client_panorama.dll!0xcee9f0 ConVar safezonex
client_panorama.dll!0xceea48 ConVar safezoney
client_panorama.dll!0xcd83e0 ConVar sc_enable
client_panorama.dll!0xcd8388 ConVar sc_pitch_sensitivity
client_panorama.dll!0xcd8438 ConVar sc_yaw_sensitivity
client_panorama.dll!0xcdb710 ConVar scene_clientflex
client_panorama.dll!0xcdb2b0 ConVar scene_print
client_panorama.dll!0xcd8120 ConVar sensitivity
client_panorama.dll!0xcd2a10 ConVar sk_autoaim_mode
client_panorama.dll!0xce1258 ConVar skybox_disablereflection
client_panorama.dll!0xcca7c0 ConVar smoothstairs
client_panorama.dll!0xced028 ConVar snd_mainmenu_music_break_time_max
client_panorama.dll!0xcecfc8 ConVar snd_mainmenu_music_break_time_min
client_panorama.dll!0xce89a8 ConVar snd_music_boost
client_panorama.dll!0xce8a00 ConVar snd_music_selection
client_panorama.dll!0xcdb7e0 ConVar snd_prevent_ss_duplicates
client_panorama.dll!0xcdb838 ConVar snd_sos_show_client_xmit
client_panorama.dll!0xcdb9f8 ConVar soundpatch_captionlength
client_panorama.dll!0xccec98 ConVar soundscape_fadetime
client_panorama.dll!0xcced48 ConVar soundscape_radius_debug
client_panorama.dll!0xcd3180 ConVar spec_autodirector
client_panorama.dll!0xcd31d8 ConVar spec_autodirector_pausetime
client_panorama.dll!0xccffc0 ConVar spec_cameraman_disable_with_user_control
client_panorama.dll!0xccff10 ConVar spec_cameraman_ui
client_panorama.dll!0xccff68 ConVar spec_cameraman_xray
client_panorama.dll!0xccfe60 ConVar spec_dz_group_teams
client_panorama.dll!0xce2308 ConVar spec_freeze_cinematiclight_b
client_panorama.dll!0xce22b0 ConVar spec_freeze_cinematiclight_g
client_panorama.dll!0xce2258 ConVar spec_freeze_cinematiclight_r
client_panorama.dll!0xce2360 ConVar spec_freeze_cinematiclight_scale
client_panorama.dll!0xccbdc8 ConVar spec_freeze_deathanim_time
client_panorama.dll!0xccbd18 ConVar spec_freeze_distance_max
client_panorama.dll!0xccbcc0 ConVar spec_freeze_distance_min
client_panorama.dll!0xccbd70 ConVar spec_freeze_panel_extended_time
client_panorama.dll!0xccbe78 ConVar spec_freeze_target_fov
client_panorama.dll!0xccbe20 ConVar spec_freeze_target_fov_long
client_panorama.dll!0xccbbb8 ConVar spec_freeze_time
client_panorama.dll!0xccbc10 ConVar spec_freeze_traveltime
client_panorama.dll!0xccbc68 ConVar spec_freeze_traveltime_long
client_panorama.dll!0xce24c0 ConVar spec_glow_decay_time
client_panorama.dll!0xce2468 ConVar spec_glow_full_time
client_panorama.dll!0xce23b8 ConVar spec_glow_silent_factor
client_panorama.dll!0xce2410 ConVar spec_glow_spike_factor
client_panorama.dll!0xce2518 ConVar spec_glow_spike_time
client_panorama.dll!0xce3b48 ConVar spec_hide_players
client_panorama.dll!0xccbf28 ConVar spec_lock_to_accountid
client_panorama.dll!0xcd3288 ConVar spec_overwatch_skip_idle_ticks
client_panorama.dll!0xcedb08 ConVar spec_replay_autostart
client_panorama.dll!0xccfeb8 ConVar spec_usenumberkeys_nobinds
client_panorama.dll!0xcdd540 ConVar ss_debug_draw_player
client_panorama.dll!0xcdc950 ConVar ss_enable
client_panorama.dll!0xcd6e50 ConVar ss_mimic
client_panorama.dll!0xcec868 ConVar steam_controller_haptics
client_panorama.dll!0xcecd60 ConVar steamworks_sessionid_client
client_panorama.dll!0xcdbb30 ConVar steamworks_sessionid_server
client_panorama.dll!0xcf3550 ConVar store_version
client_panorama.dll!0xcd8cf0 ConVar sv_accelerate
client_panorama.dll!0xcd8928 ConVar sv_accelerate_debug_speed
client_panorama.dll!0xcd88d0 ConVar sv_accelerate_use_weapon_speed
client_panorama.dll!0xcd2600 ConVar sv_air_max_horizontal_parachute_ratio
client_panorama.dll!0xcd25a8 ConVar sv_air_max_horizontal_parachute_speed
client_panorama.dll!0xcd2550 ConVar sv_air_max_wishspeed
client_panorama.dll!0xcd2658 ConVar sv_air_pushaway_dist
client_panorama.dll!0xcd8e50 ConVar sv_airaccelerate
client_panorama.dll!0xcd8df8 ConVar sv_airaccelerate_parachute
client_panorama.dll!0xcd8da0 ConVar sv_airaccelerate_rappel
client_panorama.dll!0xce6c18 ConVar sv_allow_thirdperson
client_panorama.dll!0xce9030 ConVar sv_alltalk
client_panorama.dll!0xce4cf0 ConVar sv_autobunnyhopping
client_panorama.dll!0xcd8a88 ConVar sv_backspeed
client_panorama.dll!0xce8cc0 ConVar sv_bot_difficulty_gamepad
client_panorama.dll!0xce8d70 ConVar sv_bot_difficulty_hydra
client_panorama.dll!0xce8c68 ConVar sv_bot_difficulty_kbm
client_panorama.dll!0xce8d18 ConVar sv_bot_difficulty_ps3move
client_panorama.dll!0xce8dc8 ConVar sv_bot_difficulty_sharpshooter
client_panorama.dll!0xcd9008 ConVar sv_bounce
client_panorama.dll!0xcf0e28 ConVar sv_breachcharge_arm_delay
client_panorama.dll!0xcf0d20 ConVar sv_breachcharge_delay_max
client_panorama.dll!0xcf0cc8 ConVar sv_breachcharge_delay_min
client_panorama.dll!0xcf0c70 ConVar sv_breachcharge_distance_max
client_panorama.dll!0xcf0c18 ConVar sv_breachcharge_distance_min
client_panorama.dll!0xcf0dd0 ConVar sv_breachcharge_fuse_max
client_panorama.dll!0xcf0d78 ConVar sv_breachcharge_fuse_min
client_panorama.dll!0xce5298 ConVar sv_chat_proximity
client_panorama.dll!0xcca260 ConVar sv_clamp_unsafe_velocities
client_panorama.dll!0xceb988 ConVar sv_clip_penetration_traces_to_players
client_panorama.dll!0xce6bc0 ConVar sv_coach_comm_unrestricted
client_panorama.dll!0xceacd8 ConVar sv_coaching_enabled
client_panorama.dll!0xceac28 ConVar sv_competitive_official_5v5
client_panorama.dll!0xce8c10 ConVar sv_compute_per_bot_difficulty
client_panorama.dll!0xceb930 ConVar sv_cs_player_speed_has_hostage
client_panorama.dll!0xce9088 ConVar sv_deadtalk
client_panorama.dll!0xcca710 ConVar sv_debug_player_use
client_panorama.dll!0xcea860 ConVar sv_disable_immunity_alpha
client_panorama.dll!0xce53f8 ConVar sv_disable_motd
client_panorama.dll!0xceaa18 ConVar sv_disable_observer_interpolation
client_panorama.dll!0xce50e0 ConVar sv_disable_radar
client_panorama.dll!0xce6cc8 ConVar sv_dz_hostage_rescue_reward
client_panorama.dll!0xcef0b8 ConVar sv_dz_team_count
client_panorama.dll!0xcd12f8 ConVar sv_dz_zone_bombdrop_money_reward
client_panorama.dll!0xcd12a0 ConVar sv_dz_zone_hex_radius
client_panorama.dll!0xce4c98 ConVar sv_enablebunnyhopping
client_panorama.dll!0xce8a58 ConVar sv_endmatch_item_drop_interval
client_panorama.dll!0xce8bb8 ConVar sv_endmatch_item_drop_interval_ancient
client_panorama.dll!0xce8b60 ConVar sv_endmatch_item_drop_interval_legendary
client_panorama.dll!0xce8b08 ConVar sv_endmatch_item_drop_interval_mythical
client_panorama.dll!0xce8ab0 ConVar sv_endmatch_item_drop_interval_rare
client_panorama.dll!0xcca500 ConVar sv_extract_ammo_from_dropped_weapons
client_panorama.dll!0xce4ad0 ConVar sv_extreme_strafe_accuracy_fishtail
client_panorama.dll!0xce5190 ConVar sv_falldamage_scale
client_panorama.dll!0xce5240 ConVar sv_falldamage_to_below_player_multiplier
client_panorama.dll!0xce51e8 ConVar sv_falldamage_to_below_player_ratio
client_panorama.dll!0xcf2878 ConVar sv_fistpoint_delay
client_panorama.dll!0xcf2820 ConVar sv_fistpunch_blocked_damage
client_panorama.dll!0xcf26c0 ConVar sv_fistpunch_damage
client_panorama.dll!0xcf2770 ConVar sv_fistpunch_damage_hard
client_panorama.dll!0xcf2718 ConVar sv_fistpunch_damage_to_player_multiplier
client_panorama.dll!0xcf28d0 ConVar sv_fistpunch_impact_sounds
client_panorama.dll!0xcf27c8 ConVar sv_fistpunch_viewmove
client_panorama.dll!0xcca4a8 ConVar sv_footstep_sound_frequency
client_panorama.dll!0xcd92c8 ConVar sv_footsteps
client_panorama.dll!0xcef110 ConVar sv_force_reflections
client_panorama.dll!0xcd8c98 ConVar sv_friction
client_panorama.dll!0xce90e0 ConVar sv_full_alltalk
client_panorama.dll!0xcd3088 ConVar sv_grassburn
client_panorama.dll!0xcda488 ConVar sv_grenade_trajectory
client_panorama.dll!0xcda5e8 ConVar sv_grenade_trajectory_dash
client_panorama.dll!0xcda590 ConVar sv_grenade_trajectory_thickness
client_panorama.dll!0xcda4e0 ConVar sv_grenade_trajectory_time
client_panorama.dll!0xcda538 ConVar sv_grenade_trajectory_time_spectator
client_panorama.dll!0xccbfd8 ConVar sv_highlight_distance
client_panorama.dll!0xccbf80 ConVar sv_highlight_duration
client_panorama.dll!0xce4ed0 ConVar sv_holiday_mode
client_panorama.dll!0xcd3a38 ConVar sv_hudhint_sound
client_panorama.dll!0xcca870 ConVar sv_infinite_ammo
client_panorama.dll!0xce4b28 ConVar sv_jump_impulse
client_panorama.dll!0xce8e20 ConVar sv_kick_ban_duration
client_panorama.dll!0xcf2990 ConVar sv_knife_attack_extend_from_player_aabb
client_panorama.dll!0xcd2858 ConVar sv_ladder_angle
client_panorama.dll!0xcd2800 ConVar sv_ladder_dampen
client_panorama.dll!0xcd28b0 ConVar sv_ladder_scale_speed
client_panorama.dll!0xce4b80 ConVar sv_ledge_mantle_helper
client_panorama.dll!0xce4c30 ConVar sv_ledge_mantle_helper_debug
client_panorama.dll!0xce4bd8 ConVar sv_ledge_mantle_helper_dzonly
client_panorama.dll!0xce64e0 ConVar sv_matchpause_auto_5v5
client_panorama.dll!0xcca6b8 ConVar sv_max_distance_transmit_footsteps
client_panorama.dll!0xcd9270 ConVar sv_maxspeed
client_panorama.dll!0xcd8fb0 ConVar sv_maxvelocity
client_panorama.dll!0xceb8d8 ConVar sv_min_jump_landing_sound
client_panorama.dll!0xcd9688 ConVar sv_mumble_positionalaudio
client_panorama.dll!0xcd9060 ConVar sv_noclipaccelerate
client_panorama.dll!0xcd6be8 ConVar sv_noclipduringpause
client_panorama.dll!0xcd90b8 ConVar sv_noclipspeed
client_panorama.dll!0xcd2960 ConVar sv_optimizedmovement
client_panorama.dll!0xce5138 ConVar sv_outofammo_indicator
client_panorama.dll!0xce6c70 ConVar sv_party_mode
client_panorama.dll!0xceb568 ConVar sv_penetration_type
client_panorama.dll!0xcd98b0 ConVar sv_pushaway_clientside
client_panorama.dll!0xcdaf00 ConVar sv_pushaway_clientside_size
client_panorama.dll!0xcd97a8 ConVar sv_pushaway_force
client_panorama.dll!0xcd9858 ConVar sv_pushaway_max_force
client_panorama.dll!0xcd9960 ConVar sv_pushaway_max_player_force
client_panorama.dll!0xcd9800 ConVar sv_pushaway_min_player_speed
client_panorama.dll!0xcd9908 ConVar sv_pushaway_player_force
client_panorama.dll!0xce4e78 ConVar sv_reward_drop_delay
client_panorama.dll!0xcd9378 ConVar sv_rollangle
client_panorama.dll!0xcd9320 ConVar sv_rollspeed
client_panorama.dll!0xce4dc8 ConVar sv_server_graphic1
client_panorama.dll!0xce4e20 ConVar sv_server_graphic2
client_panorama.dll!0xceba38 ConVar sv_server_verify_blood_on_player
client_panorama.dll!0xcea808 ConVar sv_show_bot_difficulty_in_name
client_panorama.dll!0xce25c8 ConVar sv_show_ragdoll_playernames
client_panorama.dll!0xce4fd8 ConVar sv_show_team_equipment_force_on
client_panorama.dll!0xce4f80 ConVar sv_show_team_equipment_prohibit
client_panorama.dll!0xcebbb8 ConVar sv_showbullethits
client_panorama.dll!0xceb618 ConVar sv_showimpacts
client_panorama.dll!0xceb5c0 ConVar sv_showimpacts_penetration
client_panorama.dll!0xceb670 ConVar sv_showimpacts_time
client_panorama.dll!0xceb6c8 ConVar sv_showplayerhitboxes
client_panorama.dll!0xceac80 ConVar sv_skirmish_id
client_panorama.dll!0xcd8a30 ConVar sv_skyname
client_panorama.dll!0xcdb890 ConVar sv_soundemitter_trace
client_panorama.dll!0xcdb788 ConVar sv_soundemitter_version
client_panorama.dll!0xce9190 ConVar sv_spec_hear
client_panorama.dll!0xcebb60 ConVar sv_spec_use_tournament_content_standards
client_panorama.dll!0xcd8f00 ConVar sv_specaccelerate
client_panorama.dll!0xcd8ea8 ConVar sv_specnoclip
client_panorama.dll!0xcd8f58 ConVar sv_specspeed
client_panorama.dll!0xce4918 ConVar sv_staminajumpcost
client_panorama.dll!0xce4970 ConVar sv_staminalandcost
client_panorama.dll!0xce4a20 ConVar sv_staminamax
client_panorama.dll!0xce49c8 ConVar sv_staminarecoveryrate
client_panorama.dll!0xcd2708 ConVar sv_standable_normal
client_panorama.dll!0xcd93d0 ConVar sv_stepsize
client_panorama.dll!0xcd8d48 ConVar sv_stopspeed
client_panorama.dll!0xcca768 ConVar sv_suppress_viewpunch
client_panorama.dll!0xcf2e00 ConVar sv_tablet_show_path_to_nearest_resq
client_panorama.dll!0xce9138 ConVar sv_talk_after_dying_time
client_panorama.dll!0xceabd0 ConVar sv_talk_enemy_dead
client_panorama.dll!0xceab78 ConVar sv_talk_enemy_living
client_panorama.dll!0xceaa70 ConVar sv_teamid_overhead
client_panorama.dll!0xce4f28 ConVar sv_teamid_overhead_always_prohibit
client_panorama.dll!0xce5088 ConVar sv_teamid_overhead_maxdist
client_panorama.dll!0xce5030 ConVar sv_teamid_overhead_maxdist_spec
client_panorama.dll!0xce4a78 ConVar sv_timebetweenducks
client_panorama.dll!0xcd99b8 ConVar sv_turbophysics
client_panorama.dll!0xcf13c8 ConVar sv_turning_inaccuracy_angle_min
client_panorama.dll!0xcf1420 ConVar sv_turning_inaccuracy_decay
client_panorama.dll!0xcf1370 ConVar sv_turning_inaccuracy_enabled
client_panorama.dll!0xcd26b0 ConVar sv_walkable_normal
client_panorama.dll!0xcd24a0 ConVar sv_water_movespeed_multiplier
client_panorama.dll!0xcd24f8 ConVar sv_water_swim_mode
client_panorama.dll!0xcd8980 ConVar sv_wateraccelerate
client_panorama.dll!0xcd8ae0 ConVar sv_waterdist
client_panorama.dll!0xcd89d8 ConVar sv_waterfriction
client_panorama.dll!0xceb9e0 ConVar sv_weapon_encumbrance_per_item
client_panorama.dll!0xce4d48 ConVar sv_weapon_encumbrance_scale
client_panorama.dll!0xcf2d78 ConVar tablet_c4_dist_max
client_panorama.dll!0xcf2d20 ConVar tablet_c4_dist_min
client_panorama.dll!0xceeee0 ConVar test_convar
client_panorama.dll!0xceef38 ConVar test_convar
client_panorama.dll!0xcda640 ConVar think_limit
client_panorama.dll!0xce33a8 ConVar thirdperson_lockcamera
client_panorama.dll!0xceb3a8 ConVar tv_spectator_port_offset
client_panorama.dll!0xcf0128 ConVar ui_inventorysettings_recently_acknowledged
client_panorama.dll!0xcef6e8 ConVar ui_lobby_draft_enabled
client_panorama.dll!0xcefcb0 ConVar ui_nearbylobbies_filter
client_panorama.dll!0xceff18 ConVar ui_playsettings_maps_listen_casual
client_panorama.dll!0xcefe10 ConVar ui_playsettings_maps_listen_competitive
client_panorama.dll!0xceff70 ConVar ui_playsettings_maps_listen_deathmatch
client_panorama.dll!0xcefe68 ConVar ui_playsettings_maps_listen_scrimcomp2v2
client_panorama.dll!0xcefec0 ConVar ui_playsettings_maps_listen_skirmish
client_panorama.dll!0xcefd08 ConVar ui_playsettings_maps_official_casual
client_panorama.dll!0xcefdb8 ConVar ui_playsettings_maps_official_dangerzone
client_panorama.dll!0xcefd60 ConVar ui_playsettings_maps_official_deathmatch
client_panorama.dll!0xceffc8 ConVar ui_playsettings_maps_workshop
client_panorama.dll!0xcf0390 ConVar ui_playsettings_mode_listen
client_panorama.dll!0xcf0338 ConVar ui_playsettings_mode_official_dz
client_panorama.dll!0xcf03e8 ConVar ui_playsettings_survival_solo
client_panorama.dll!0xcf02e0 ConVar ui_playsettings_warmup_map_name
client_panorama.dll!0xcf01d8 ConVar ui_popup_weaponupdate_version
client_panorama.dll!0xcda790 ConVar ui_posedebug_fade_in_time
client_panorama.dll!0xcda738 ConVar ui_posedebug_fade_out_time
client_panorama.dll!0xcf0020 ConVar ui_vanitysetting_itemid
client_panorama.dll!0xcf00d0 ConVar ui_vanitysetting_loadoutslot
client_panorama.dll!0xcf0078 ConVar ui_vanitysetting_team
client_panorama.dll!0xced570 ConVar vgui_message_dialog_modal
client_panorama.dll!0xcca3f8 ConVar view_punch_decay
client_panorama.dll!0xcca450 ConVar view_recoil_tracking
client_panorama.dll!0xcdd6b8 ConVar viewmodel_fov
client_panorama.dll!0xcca8c8 ConVar viewmodel_offset_x
client_panorama.dll!0xcca920 ConVar viewmodel_offset_y
client_panorama.dll!0xcca978 ConVar viewmodel_offset_z
client_panorama.dll!0xcca9d0 ConVar viewmodel_recoil
client_panorama.dll!0xccc1b8 ConVar vm_debug
client_panorama.dll!0xccc210 ConVar vm_draw_always
client_panorama.dll!0xce1360 ConVar voice_modenable
client_panorama.dll!0xcf1058 ConVar weapon_accuracy_forcespread
client_panorama.dll!0xcec810 ConVar weapon_accuracy_logging
client_panorama.dll!0xcf10b0 ConVar weapon_accuracy_nospread
client_panorama.dll!0xceca78 ConVar weapon_accuracy_shotgun_spread_patterns
client_panorama.dll!0xcf1160 ConVar weapon_air_spread_scale
client_panorama.dll!0xcf1210 ConVar weapon_auto_cleanup_time
client_panorama.dll!0xcec970 ConVar weapon_debug_inaccuracy_only_up
client_panorama.dll!0xcec918 ConVar weapon_debug_max_inaccuracy
client_panorama.dll!0xcf1818 ConVar weapon_debug_spread_gap
client_panorama.dll!0xcf1df0 ConVar weapon_debug_spread_show
client_panorama.dll!0xcf0f50 ConVar weapon_land_dip_amt
client_panorama.dll!0xcf1268 ConVar weapon_max_before_cleanup
client_panorama.dll!0xcec8c0 ConVar weapon_near_empty_sound
client_panorama.dll!0xcf1108 ConVar weapon_recoil_cooldown
client_panorama.dll!0xcf0fa8 ConVar weapon_recoil_decay1_exp
client_panorama.dll!0xcf2058 ConVar weapon_recoil_decay2_exp
client_panorama.dll!0xcf2000 ConVar weapon_recoil_decay2_lin
client_panorama.dll!0xcf1000 ConVar weapon_recoil_decay_coefficient
client_panorama.dll!0xcf1ea0 ConVar weapon_recoil_scale
client_panorama.dll!0xcf1e48 ConVar weapon_recoil_scale_motion_controller
client_panorama.dll!0xcebea0 ConVar weapon_recoil_suppression_factor
client_panorama.dll!0xcebe48 ConVar weapon_recoil_suppression_shots
client_panorama.dll!0xcebef8 ConVar weapon_recoil_variance
client_panorama.dll!0xcf20b0 ConVar weapon_recoil_vel_decay
client_panorama.dll!0xceba90 ConVar weapon_recoil_view_punch_extra
client_panorama.dll!0xcf11b8 ConVar weapon_reticle_knife_show
client_panorama.dll!0xcf12c0 ConVar weapon_sound_falloff_multiplier
client_panorama.dll!0xcdd0f8 ConVar zoom_sensitivity_ratio_joystick
client_panorama.dll!0xcdd150 ConVar zoom_sensitivity_ratio_mouse
```

## ConCommands

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

### Addresses

```
client_panorama.dll!0x00cd75d4 ConCommand +alt1
client_panorama.dll!0x00cd761c ConCommand +alt2
client_panorama.dll!0x00cd7370 ConCommand +attack
client_panorama.dll!0x00cd73b8 ConCommand +attack2
client_panorama.dll!0x00cd7130 ConCommand +back
client_panorama.dll!0x00cd773c ConCommand +break
client_panorama.dll!0x00cd4dec ConCommand +camdistance
client_panorama.dll!0x00cd4c3c ConCommand +camin
client_panorama.dll!0x00cd4da4 ConCommand +cammousemove
client_panorama.dll!0x00cd4c84 ConCommand +camout
client_panorama.dll!0x00cd4b64 ConCommand +campitchdown
client_panorama.dll!0x00cd4b1c ConCommand +campitchup
client_panorama.dll!0x00cd4bac ConCommand +camyawleft
client_panorama.dll!0x00cd4bf4 ConCommand +camyawright
client_panorama.dll!0x00ceecb8 ConCommand +cl_show_team_equipment
client_panorama.dll!0x00cd6f80 ConCommand +commandermousemove
client_panorama.dll!0x00ccd484 ConCommand +csm_rot_x_neg
client_panorama.dll!0x00ccd43c ConCommand +csm_rot_x_plus
client_panorama.dll!0x00ccd514 ConCommand +csm_rot_y_neg
client_panorama.dll!0x00ccd4cc ConCommand +csm_rot_y_plus
client_panorama.dll!0x00cd7544 ConCommand +duck
client_panorama.dll!0x00cd70e8 ConCommand +forward
client_panorama.dll!0x00cd76f4 ConCommand +graph
client_panorama.dll!0x00cd78c8 ConCommand +grenade1
client_panorama.dll!0x00cd7910 ConCommand +grenade2
client_panorama.dll!0x00cd74fc ConCommand +jlook
client_panorama.dll!0x00cd7448 ConCommand +jump
client_panorama.dll!0x00cd74b4 ConCommand +klook
client_panorama.dll!0x00cd7058 ConCommand +left
client_panorama.dll!0x00cd71c0 ConCommand +lookdown
client_panorama.dll!0x00cd7934 ConCommand +lookspin
client_panorama.dll!0x00cd7178 ConCommand +lookup
client_panorama.dll!0x00cd7010 ConCommand +movedown
client_panorama.dll!0x00cd7250 ConCommand +moveleft
client_panorama.dll!0x00cd7298 ConCommand +moveright
client_panorama.dll!0x00cd6fc8 ConCommand +moveup
client_panorama.dll!0x00cd758c ConCommand +reload
client_panorama.dll!0x00cd70a0 ConCommand +right
client_panorama.dll!0x00cd7664 ConCommand +score
client_panorama.dll!0x00cd76ac ConCommand +showscores
client_panorama.dll!0x00cd72e0 ConCommand +speed
client_panorama.dll!0x00ceed1c ConCommand +spray_menu
client_panorama.dll!0x00cd7208 ConCommand +strafe
client_panorama.dll!0x00cd7400 ConCommand +use
client_panorama.dll!0x00cd7328 ConCommand +walk
client_panorama.dll!0x00cd77cc ConCommand +zoom
client_panorama.dll!0x00cd7814 ConCommand +zoom_in
client_panorama.dll!0x00cd785c ConCommand +zoom_out
client_panorama.dll!0x00cd75f8 ConCommand -alt1
client_panorama.dll!0x00cd7640 ConCommand -alt2
client_panorama.dll!0x00cd7394 ConCommand -attack
client_panorama.dll!0x00cd73dc ConCommand -attack2
client_panorama.dll!0x00cd7154 ConCommand -back
client_panorama.dll!0x00cd7760 ConCommand -break
client_panorama.dll!0x00cd4e10 ConCommand -camdistance
client_panorama.dll!0x00cd4c60 ConCommand -camin
client_panorama.dll!0x00cd4dc8 ConCommand -cammousemove
client_panorama.dll!0x00cd4ca8 ConCommand -camout
client_panorama.dll!0x00cd4b88 ConCommand -campitchdown
client_panorama.dll!0x00cd4b40 ConCommand -campitchup
client_panorama.dll!0x00cd4bd0 ConCommand -camyawleft
client_panorama.dll!0x00cd4c18 ConCommand -camyawright
client_panorama.dll!0x00ceecdc ConCommand -cl_show_team_equipment
client_panorama.dll!0x00cd6fa4 ConCommand -commandermousemove
client_panorama.dll!0x00ccd4a8 ConCommand -csm_rot_x_neg
client_panorama.dll!0x00ccd460 ConCommand -csm_rot_x_plus
client_panorama.dll!0x00ccd538 ConCommand -csm_rot_y_neg
client_panorama.dll!0x00ccd4f0 ConCommand -csm_rot_y_plus
client_panorama.dll!0x00cd7568 ConCommand -duck
client_panorama.dll!0x00cd710c ConCommand -forward
client_panorama.dll!0x00cd7718 ConCommand -graph
client_panorama.dll!0x00cd78a4 ConCommand -grenade1
client_panorama.dll!0x00cd78ec ConCommand -grenade2
client_panorama.dll!0x00cd7520 ConCommand -jlook
client_panorama.dll!0x00cd746c ConCommand -jump
client_panorama.dll!0x00cd74d8 ConCommand -klook
client_panorama.dll!0x00cd707c ConCommand -left
client_panorama.dll!0x00cd71e4 ConCommand -lookdown
client_panorama.dll!0x00cd7958 ConCommand -lookspin
client_panorama.dll!0x00cd719c ConCommand -lookup
client_panorama.dll!0x00cd7034 ConCommand -movedown
client_panorama.dll!0x00cd7274 ConCommand -moveleft
client_panorama.dll!0x00cd72bc ConCommand -moveright
client_panorama.dll!0x00cd6fec ConCommand -moveup
client_panorama.dll!0x00cd75b0 ConCommand -reload
client_panorama.dll!0x00cd70c4 ConCommand -right
client_panorama.dll!0x00cd7688 ConCommand -score
client_panorama.dll!0x00cd76d0 ConCommand -showscores
client_panorama.dll!0x00cd7304 ConCommand -speed
client_panorama.dll!0x00ceed40 ConCommand -spray_menu
client_panorama.dll!0x00cd722c ConCommand -strafe
client_panorama.dll!0x00cd7424 ConCommand -use
client_panorama.dll!0x00cd734c ConCommand -walk
client_panorama.dll!0x00cd77f0 ConCommand -zoom
client_panorama.dll!0x00cd7838 ConCommand -zoom_in
client_panorama.dll!0x00cd7880 ConCommand -zoom_out
client_panorama.dll!0x00ccdbf0 ConCommand CreateHairball
client_panorama.dll!0x00cecdb8 ConCommand ShowSteamStatsSessionID
client_panorama.dll!0x00ccf134 ConCommand Test_ProxyToggle_EnsureValue
client_panorama.dll!0x00ce3410 ConCommand autobuy
client_panorama.dll!0x00ced3ec ConCommand bench_showstatsdialog
client_panorama.dll!0x00cf36b4 ConCommand buy_stamps
client_panorama.dll!0x00cf07f0 ConCommand buymenu
client_panorama.dll!0x00cd4af8 ConCommand cam_command
client_panorama.dll!0x00cd4d80 ConCommand camortho
client_panorama.dll!0x00ce1990 ConCommand cancelselect
client_panorama.dll!0x00cd3820 ConCommand cc_emit
client_panorama.dll!0x00cd3908 ConCommand cc_findsound
client_panorama.dll!0x00cd3868 ConCommand cc_flush
client_panorama.dll!0x00cd3844 ConCommand cc_random
client_panorama.dll!0x00cd388c ConCommand cc_showblocks
client_panorama.dll!0x00cdd518 ConCommand centerview
client_panorama.dll!0x00cd3420 ConCommand cl_animationinfo
client_panorama.dll!0x00ced718 ConCommand cl_avatar_convert_rgb
client_panorama.dll!0x00cd3158 ConCommand cl_clearhinthistory
client_panorama.dll!0x00cf2fc4 ConCommand cl_cs_dump_econ_item_stringtable
client_panorama.dll!0x00ccd3c0 ConCommand cl_csm_status
client_panorama.dll!0x00ce3a18 ConCommand cl_dev_decaltrace_blood
client_panorama.dll!0x00cd9fb8 ConCommand cl_dump_particle_stats
client_panorama.dll!0x00cd3a90 ConCommand cl_dumpplayer
client_panorama.dll!0x00ccf624 ConCommand cl_dumpsplithacks
client_panorama.dll!0x00ccb6b4 ConCommand cl_ent_absbox
client_panorama.dll!0x00ccb690 ConCommand cl_ent_bbox
client_panorama.dll!0x00ccb6d8 ConCommand cl_ent_rbox
client_panorama.dll!0x00ccb6fc ConCommand cl_find_ent
client_panorama.dll!0x00ccb720 ConCommand cl_find_ent_index
client_panorama.dll!0x00cecb84 ConCommand cl_game_mode_convars
client_panorama.dll!0x00cefac8 ConCommand cl_mainmenu_blog_file
client_panorama.dll!0x00cef1f8 ConCommand cl_mainmenu_hide_blog
client_panorama.dll!0x00cef1d4 ConCommand cl_mainmenu_show_blog
client_panorama.dll!0x00cef890 ConCommand cl_matchstats_print_own_data
client_panorama.dll!0x00ccf6e8 ConCommand cl_modemanager_reload
client_panorama.dll!0x00cdca60 ConCommand cl_panelanimation
client_panorama.dll!0x00cd9ee0 ConCommand cl_particles_dump_effects
client_panorama.dll!0x00cd9fdc ConCommand cl_particles_dumplist
client_panorama.dll!0x00ceb520 ConCommand cl_player_rank_events_spew
client_panorama.dll!0x00cdaebc ConCommand cl_pred_track
client_panorama.dll!0x00cdae98 ConCommand cl_predictioncopy_describe
client_panorama.dll!0x00cf3410 ConCommand cl_quest_events_print
client_panorama.dll!0x00cf33ec ConCommand cl_quest_schedule_print
client_panorama.dll!0x00ceec78 ConCommand cl_reload_hud
client_panorama.dll!0x00ce44a4 ConCommand cl_reloadpostprocessparams
client_panorama.dll!0x00cec5c0 ConCommand cl_remove_all_workshop_maps
client_panorama.dll!0x00ccb66c ConCommand cl_removedecals
client_panorama.dll!0x00cdba50 ConCommand cl_report_soundpatch
client_panorama.dll!0x00ce44ec ConCommand cl_sos_test_get_opvar
client_panorama.dll!0x00ce44c8 ConCommand cl_sos_test_set_opvar
client_panorama.dll!0x00cdb968 ConCommand cl_soundemitter_flush
client_panorama.dll!0x00cdb944 ConCommand cl_soundemitter_reload
client_panorama.dll!0x00ccee78 ConCommand cl_soundscape_flush
client_panorama.dll!0x00cceee4 ConCommand cl_soundscape_printdebuginfo
client_panorama.dll!0x00ccef08 ConCommand cl_ss_origin
client_panorama.dll!0x00ce4538 ConCommand cl_steamscreenshots
client_panorama.dll!0x00cd1774 ConCommand cl_tree_sway_dir
client_panorama.dll!0x00ccb4c4 ConCommand cl_updatevisibility
client_panorama.dll!0x00cd1090 ConCommand commentary_showmodelviewer
client_panorama.dll!0x00ced468 ConCommand commentary_testfirstrun
client_panorama.dll!0x00cecf88 ConCommand condump
client_panorama.dll!0x00cef244 ConCommand confirm_abandon_match
client_panorama.dll!0x00cf0450 ConCommand confirm_activate_itemid_now
client_panorama.dll!0x00cef400 ConCommand confirm_join_friend_session_exit_current
client_panorama.dll!0x00ced19c ConCommand confirm_join_new_session_exit_current
client_panorama.dll!0x00cefc2c ConCommand confirm_join_party_session_exit_current
client_panorama.dll!0x00cf0498 ConCommand confirm_purchase_item_def_now
client_panorama.dll!0x00cef424 ConCommand confirm_watch_friend_session_exit_current
client_panorama.dll!0x00cef784 ConCommand csgo_download_match
client_panorama.dll!0x00ced4f8 ConCommand csgo_econ_action_preview
client_panorama.dll!0x00cf04bc ConCommand debug_purchase_defidx
client_panorama.dll!0x00ccb648 ConCommand dlight_debug
client_panorama.dll!0x00ce3458 ConCommand dm_togglerandomweapons
client_panorama.dll!0x00cee214 ConCommand drawoverviewmap
client_panorama.dll!0x00ceec30 ConCommand drawradar
client_panorama.dll!0x00cf3138 ConCommand dump_particlemanifest
client_panorama.dll!0x00cf3508 ConCommand econ_build_pinboard_images_from_collection_name
client_panorama.dll!0x00cf3434 ConCommand econ_show_items_with_tag
client_panorama.dll!0x00ce347c ConCommand endmatch_votenextmap
client_panorama.dll!0x00ce4634 ConCommand error_message_explain_pure
client_panorama.dll!0x00ce4610 ConCommand error_message_explain_vac
client_panorama.dll!0x00cd4d14 ConCommand firstperson
client_panorama.dll!0x00cd7784 ConCommand force_centerview
client_panorama.dll!0x00cd3ad8 ConCommand g15_dumpplayer
client_panorama.dll!0x00cd3ab4 ConCommand g15_reload
client_panorama.dll!0x00ccdac0 ConCommand gameinstructor_dump_open_lessons
client_panorama.dll!0x00ccda78 ConCommand gameinstructor_reload_lessons
client_panorama.dll!0x00ccda9c ConCommand gameinstructor_reset_counts
client_panorama.dll!0x00ced644 ConCommand gamemenucommand
client_panorama.dll!0x00ce1a8c ConCommand gamepadslot1
client_panorama.dll!0x00ce1ab0 ConCommand gamepadslot2
client_panorama.dll!0x00ce1ad4 ConCommand gamepadslot3
client_panorama.dll!0x00ce1af8 ConCommand gamepadslot4
client_panorama.dll!0x00ce1b1c ConCommand gamepadslot5
client_panorama.dll!0x00ce1b40 ConCommand gamepadslot6
client_panorama.dll!0x00cefaec ConCommand gcmd
client_panorama.dll!0x00cdd5bc ConCommand getpos
client_panorama.dll!0x00cdd5e0 ConCommand getpos_exact
client_panorama.dll!0x00cee238 ConCommand hideoverviewmap
client_panorama.dll!0x00cf3844 ConCommand hidepanel
client_panorama.dll!0x00ceec54 ConCommand hideradar
client_panorama.dll!0x00cf0880 ConCommand hidescores
client_panorama.dll!0x00cdb060 ConCommand highlight_sticker
client_panorama.dll!0x00cd0264 ConCommand hud_reloadscheme
client_panorama.dll!0x00cd43b0 ConCommand hud_subtitles
client_panorama.dll!0x00cd7490 ConCommand impulse
client_panorama.dll!0x00ce19b4 ConCommand invnext
client_panorama.dll!0x00ce1a20 ConCommand invnextgrenade
client_panorama.dll!0x00ce1a44 ConCommand invnextitem
client_panorama.dll!0x00ce1a68 ConCommand invnextnongrenade
client_panorama.dll!0x00ce19d8 ConCommand invprev
client_panorama.dll!0x00cd77a8 ConCommand joyadvancedupdate
client_panorama.dll!0x00ce19fc ConCommand lastinv
client_panorama.dll!0x00cef66c ConCommand launch_warmup_map
client_panorama.dll!0x00cd32f8 ConCommand list_active_casters
client_panorama.dll!0x00ced4b0 ConCommand loadcommentary
client_panorama.dll!0x00cef620 ConCommand localization_quest_item_string_printout
client_panorama.dll!0x00ce34a0 ConCommand mat_reloadwearablecustommaterials
client_panorama.dll!0x00cef6a0 ConCommand matchdraft_debug_sendlog
client_panorama.dll!0x00cedda8 ConCommand menuselect
client_panorama.dll!0x00cef6c4 ConCommand mm_queue_draft_show
client_panorama.dll!0x00ceb360 ConCommand mm_queue_show_stats
client_panorama.dll!0x00ced4d4 ConCommand modelpanel_set_sticker
client_panorama.dll!0x00cdb084 ConCommand peel_sticker
client_panorama.dll!0x00cf3690 ConCommand perfectworld_replenish_funds
client_panorama.dll!0x00cda0a4 ConCommand perfvisualbenchmark
client_panorama.dll!0x00cda0c8 ConCommand perfvisualbenchmark_abort
client_panorama.dll!0x00ce4da0 ConCommand pick_hint
client_panorama.dll!0x00cce020 ConCommand pixelvis_debug
client_panorama.dll!0x00cdb98c ConCommand playgamesound
client_panorama.dll!0x00ccee9c ConCommand playsoundscape
client_panorama.dll!0x00cdcfd8 ConCommand playvideo
client_panorama.dll!0x00cdd020 ConCommand playvideo_end_level_transition
client_panorama.dll!0x00cdd044 ConCommand playvideo_exitcommand
client_panorama.dll!0x00cdd068 ConCommand playvideo_exitcommand_nointerrupt
client_panorama.dll!0x00cdcffc ConCommand playvideo_nointerrupt
client_panorama.dll!0x00ceb544 ConCommand print_achievement_categories
client_panorama.dll!0x00ce5450 ConCommand print_mapgroup
client_panorama.dll!0x00ced620 ConCommand quit_prompt
client_panorama.dll!0x00ce0e30 ConCommand r_cheapwaterend
client_panorama.dll!0x00ce0e0c ConCommand r_cheapwaterstart
client_panorama.dll!0x00cec768 ConCommand r_ropes_holiday_light_color
client_panorama.dll!0x00cde168 ConCommand r_screenoverlay
client_panorama.dll!0x00cd098c ConCommand r_shadowangles
client_panorama.dll!0x00cd09f8 ConCommand r_shadowblobbycutoff
client_panorama.dll!0x00cd09b0 ConCommand r_shadowcolor
client_panorama.dll!0x00cd0968 ConCommand r_shadowdir
client_panorama.dll!0x00cd09d4 ConCommand r_shadowdist
client_panorama.dll!0x00cedd18 ConCommand radio
client_panorama.dll!0x00cedd3c ConCommand radio1
client_panorama.dll!0x00cedd60 ConCommand radio2
client_panorama.dll!0x00cedd84 ConCommand radio3
client_panorama.dll!0x00ce3520 ConCommand rangefinder
client_panorama.dll!0x00ce3434 ConCommand rebuy
client_panorama.dll!0x00cf0474 ConCommand reload_store_config
client_panorama.dll!0x00ce1678 ConCommand script_client
client_panorama.dll!0x00ce16c0 ConCommand script_debug_client
client_panorama.dll!0x00ce1708 ConCommand script_dump_all_client
client_panorama.dll!0x00ce169c ConCommand script_execute_client
client_panorama.dll!0x00ce16e4 ConCommand script_help_client
client_panorama.dll!0x00cdd960 ConCommand shake_stop
client_panorama.dll!0x00cdd984 ConCommand shake_testpunch
client_panorama.dll!0x00ce4058 ConCommand show_loadout_toggle
client_panorama.dll!0x00cf3918 ConCommand showinfo
client_panorama.dll!0x00cf3820 ConCommand showpanel
client_panorama.dll!0x00ce18dc ConCommand slot0
client_panorama.dll!0x00ce1798 ConCommand slot1
client_panorama.dll!0x00ce1900 ConCommand slot10
client_panorama.dll!0x00ce1924 ConCommand slot11
client_panorama.dll!0x00ce1948 ConCommand slot12
client_panorama.dll!0x00ce196c ConCommand slot13
client_panorama.dll!0x00ce17bc ConCommand slot2
client_panorama.dll!0x00ce17e0 ConCommand slot3
client_panorama.dll!0x00ce1804 ConCommand slot4
client_panorama.dll!0x00ce1828 ConCommand slot5
client_panorama.dll!0x00ce184c ConCommand slot6
client_panorama.dll!0x00ce1870 ConCommand slot7
client_panorama.dll!0x00ce1894 ConCommand slot8
client_panorama.dll!0x00ce18b8 ConCommand slot9
client_panorama.dll!0x00cd4e34 ConCommand snapto
client_panorama.dll!0x00cdb9b0 ConCommand snd_playsounds
client_panorama.dll!0x00cdb9d4 ConCommand snd_setsoundparam
client_panorama.dll!0x00ccc04c ConCommand soundscape_dumpclient
client_panorama.dll!0x00cd0120 ConCommand spec_cameraman_set_xray
client_panorama.dll!0x00cd021c ConCommand spec_goto
client_panorama.dll!0x00cf0838 ConCommand spec_gui
client_panorama.dll!0x00cd0240 ConCommand spec_lerpto
client_panorama.dll!0x00cf0814 ConCommand spec_menu
client_panorama.dll!0x00cd018c ConCommand spec_mode
client_panorama.dll!0x00cd0144 ConCommand spec_next
client_panorama.dll!0x00cd01b0 ConCommand spec_player
client_panorama.dll!0x00cd01f8 ConCommand spec_player_by_accountid
client_panorama.dll!0x00cd01d4 ConCommand spec_player_by_name
client_panorama.dll!0x00cdd598 ConCommand spec_pos
client_panorama.dll!0x00cd0168 ConCommand spec_prev
client_panorama.dll!0x00cdc9e0 ConCommand ss_reloadletterbox
client_panorama.dll!0x00cd6ea8 ConCommand ss_teleport
client_panorama.dll!0x00cdd0d4 ConCommand stop_transition_videos_fadeout
client_panorama.dll!0x00cceec0 ConCommand stopsoundscape
client_panorama.dll!0x00cdd08c ConCommand stopvideos
client_panorama.dll!0x00cdd0b0 ConCommand stopvideos_fadeout
client_panorama.dll!0x00cf07cc ConCommand teammenu
client_panorama.dll!0x00cdfa28 ConCommand test_freezeframe
client_panorama.dll!0x00cd3378 ConCommand testhudanim
client_panorama.dll!0x00cd4ccc ConCommand thirdperson
client_panorama.dll!0x00cd4cf0 ConCommand thirdperson_mayamode
client_panorama.dll!0x00cd4d5c ConCommand thirdpersonoverview
client_panorama.dll!0x00cd4d38 ConCommand thirdpersonshoulder
client_panorama.dll!0x00ce4510 ConCommand toggleRdrOpt
client_panorama.dll!0x00ce0de8 ConCommand toggleThreadedBuildRWList
client_panorama.dll!0x00cd797c ConCommand toggle_duck
client_panorama.dll!0x00cf085c ConCommand togglescores
client_panorama.dll!0x00ced294 ConCommand ui_reloadscheme
client_panorama.dll!0x00cddacc ConCommand viewanim_addkeyframe
client_panorama.dll!0x00cdda60 ConCommand viewanim_create
client_panorama.dll!0x00cddb14 ConCommand viewanim_load
client_panorama.dll!0x00cddaa8 ConCommand viewanim_reset
client_panorama.dll!0x00cddaf0 ConCommand viewanim_save
client_panorama.dll!0x00cdda84 ConCommand viewanim_test
client_panorama.dll!0x00ceede0 ConCommand voice_status_test_toggle
client_panorama.dll!0x00ced48c ConCommand workshop_publish
client_panorama.dll!0x00ced54c ConCommand workshop_workbench
client_panorama.dll!0x00cd79c4 ConCommand xlook
client_panorama.dll!0x00cd79a0 ConCommand xmove
```

## Buttons

```
client_panorama.dll!0x051869ac kbutton_t +alt1
client_panorama.dll!0x051869b8 kbutton_t +alt2
client_panorama.dll!0x03114b9c kbutton_t +attack
client_panorama.dll!0x03114b90 kbutton_t +attack2
client_panorama.dll!0x03114bf0 kbutton_t +back
client_panorama.dll!0x051869d0 kbutton_t +break
client_panorama.dll!0x0518690c kbutton_t +camin
client_panorama.dll!0x05186918 kbutton_t +camout
client_panorama.dll!0x051868e8 kbutton_t +campitchdown
client_panorama.dll!0x051868dc kbutton_t +campitchup
client_panorama.dll!0x051868f4 kbutton_t +camyawleft
client_panorama.dll!0x05186900 kbutton_t +camyawright
client_panorama.dll!0x03114bd8 kbutton_t +commandermousemove
client_panorama.dll!0x05186994 kbutton_t +duck
client_panorama.dll!0x03114bcc kbutton_t +forward
client_panorama.dll!0x03114bfc kbutton_t +graph
client_panorama.dll!0x051869dc kbutton_t +grenade1
client_panorama.dll!0x051869e8 kbutton_t +grenade2
client_panorama.dll!0x03114c30 kbutton_t +jlook
client_panorama.dll!0x05186970 kbutton_t +jump
client_panorama.dll!0x05186928 kbutton_t +klook
client_panorama.dll!0x05186934 kbutton_t +left
client_panorama.dll!0x05186958 kbutton_t +lookdown
client_panorama.dll!0x03114c18 kbutton_t +lookspin
client_panorama.dll!0x0518694c kbutton_t +lookup
client_panorama.dll!0x05186988 kbutton_t +movedown
client_panorama.dll!0x03114be4 kbutton_t +moveleft
client_panorama.dll!0x03114c08 kbutton_t +moveright
client_panorama.dll!0x0518697c kbutton_t +moveup
client_panorama.dll!0x051869a0 kbutton_t +reload
client_panorama.dll!0x05186940 kbutton_t +right
client_panorama.dll!0x051869c4 kbutton_t +score
client_panorama.dll!0x051869c4 kbutton_t +showscores
client_panorama.dll!0x03114bc0 kbutton_t +speed
client_panorama.dll!0x03114c24 kbutton_t +strafe
client_panorama.dll!0x05186964 kbutton_t +use
client_panorama.dll!0x03114bb4 kbutton_t +walk
client_panorama.dll!0x03114ba8 kbutton_t +zoom
client_panorama.dll!0x051869dc kbutton_t +zoom_in
client_panorama.dll!0x051869e8 kbutton_t +zoom_out
```

## ClientClasses

<details>
<summary>client_class CAI_BaseNPC</summary>

sizeof: `12080`  
</details>
<details>
<summary>client_class CAK47</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CBRC4Target</summary>

sizeof: `10640`  
</details>
<details>
<summary>client_class CBaseAnimating</summary>

sizeof: `10624`  
</details>
<details>
<summary>client_class CBaseAnimatingOverlay</summary>

sizeof: `10736`  
</details>
<details>
<summary>client_class CBaseAttributableItem</summary>

sizeof: `12816`  
</details>
<details>
<summary>client_class CBaseButton</summary>

sizeof: `2576`  
</details>
<details>
<summary>client_class CBaseCSGrenade</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CBaseCSGrenadeProjectile</summary>

sizeof: `10720`  
</details>
<details>
<summary>client_class CBaseCombatCharacter</summary>

sizeof: `12048`  
</details>
<details>
<summary>client_class CBaseCombatWeapon</summary>

sizeof: `13008`  
</details>
<details>
<summary>client_class CBaseDoor</summary>

sizeof: `2576`  
</details>
<details>
<summary>client_class CBaseEntity</summary>

sizeof: `2520`  
</details>
<details>
<summary>client_class CBaseFlex</summary>

sizeof: `11632`  
</details>
<details>
<summary>client_class CBaseGrenade</summary>

sizeof: `10672`  
</details>
<details>
<summary>client_class CBaseParticleEntity</summary>

sizeof: `2752`  
</details>
<details>
<summary>client_class CBasePlayer</summary>

sizeof: `14448`  
</details>
<details>
<summary>client_class CBasePropDoor</summary>

sizeof: `10736`  
</details>
<details>
<summary>client_class CBaseTeamObjectiveResource</summary>

sizeof: `7440`  
</details>
<details>
<summary>client_class CBaseTempEntity</summary>

sizeof: `3375630023`  
</details>
<details>
<summary>client_class CBaseToggle</summary>

sizeof: `2568`  
</details>
<details>
<summary>client_class CBaseTrigger</summary>

sizeof: `2576`  
</details>
<details>
<summary>client_class CBaseVPhysicsTrigger</summary>

sizeof: `2528`  
</details>
<details>
<summary>client_class CBaseViewModel</summary>

sizeof: `10896`  
</details>
<details>
<summary>client_class CBaseWeaponWorldModel</summary>

sizeof: `10768`  
</details>
<details>
<summary>client_class CBeam</summary>

sizeof: `2688`  
</details>
<details>
<summary>client_class CBeamSpotlight</summary>

sizeof: `2608`  
</details>
<details>
<summary>client_class CBoneFollower</summary>

sizeof: `2528`  
</details>
<details>
<summary>client_class CBreachCharge</summary>

sizeof: `13216`  
</details>
<details>
<summary>client_class CBreachChargeProjectile</summary>

sizeof: `10704`  
</details>
<details>
<summary>client_class CBreakableProp</summary>

sizeof: `10656`  
</details>
<details>
<summary>client_class CBreakableSurface</summary>

sizeof: `3592`  
</details>
<details>
<summary>client_class CC4</summary>

sizeof: `13280`  
</details>
<details>
<summary>client_class CCSGameRulesProxy</summary>

sizeof: `2520`  
</details>
<details>
<summary>client_class CCSPlayer</summary>

sizeof: `48176`  
</details>
<details>
<summary>client_class CCSPlayerResource</summary>

sizeof: `24936`  
</details>
<details>
<summary>client_class CCSRagdoll</summary>

sizeof: `10832`  
</details>
<details>
<summary>client_class CCSTeam</summary>

sizeof: `2936`  
</details>
<details>
<summary>client_class CCascadeLight</summary>

sizeof: `2560`  
</details>
<details>
<summary>client_class CChicken</summary>

sizeof: `10736`  
</details>
<details>
<summary>client_class CColorCorrection</summary>

sizeof: `2848`  
</details>
<details>
<summary>client_class CColorCorrectionVolume</summary>

sizeof: `2872`  
</details>
<details>
<summary>client_class CDEagle</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CDangerZone</summary>

sizeof: `2560`  
</details>
<details>
<summary>client_class CDangerZoneController</summary>

sizeof: `2752`  
</details>
<details>
<summary>client_class CDecoyGrenade</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CDecoyProjectile</summary>

sizeof: `10736`  
</details>
<details>
<summary>client_class CDrone</summary>

sizeof: `10784`  
</details>
<details>
<summary>client_class CDronegun</summary>

sizeof: `10720`  
</details>
<details>
<summary>client_class CDynamicLight</summary>

sizeof: `2552`  
</details>
<details>
<summary>client_class CDynamicProp</summary>

sizeof: `10720`  
</details>
<details>
<summary>client_class CEconEntity</summary>

sizeof: `12816`  
</details>
<details>
<summary>client_class CEconWearable</summary>

sizeof: `12832`  
</details>
<details>
<summary>client_class CEmbers</summary>

sizeof: `2552`  
</details>
<details>
<summary>client_class CEntityDissolve</summary>

sizeof: `2592`  
</details>
<details>
<summary>client_class CEntityFlame</summary>

sizeof: `2544`  
</details>
<details>
<summary>client_class CEntityFreezing</summary>

sizeof: `2760`  
</details>
<details>
<summary>client_class CEntityParticleTrail</summary>

sizeof: `2792`  
</details>
<details>
<summary>client_class CEnvAmbientLight</summary>

sizeof: `2872`  
</details>
<details>
<summary>client_class CEnvDOFController</summary>

sizeof: `2552`  
</details>
<details>
<summary>client_class CEnvDetailController</summary>

sizeof: `2528`  
</details>
<details>
<summary>client_class CEnvGasCanister</summary>

sizeof: `10816`  
</details>
<details>
<summary>client_class CEnvParticleScript</summary>

sizeof: `10864`  
</details>
<details>
<summary>client_class CEnvProjectedTexture</summary>

sizeof: `2904`  
</details>
<details>
<summary>client_class CEnvQuadraticBeam</summary>

sizeof: `2552`  
</details>
<details>
<summary>client_class CEnvScreenEffect</summary>

sizeof: `2528`  
</details>
<details>
<summary>client_class CEnvScreenOverlay</summary>

sizeof: `5136`  
</details>
<details>
<summary>client_class CEnvTonemapController</summary>

sizeof: `2568`  
</details>
<details>
<summary>client_class CEnvWind</summary>

sizeof: `3048`  
</details>
<details>
<summary>client_class CFEPlayerDecal</summary>

sizeof: `2736`  
</details>
<details>
<summary>client_class CFireCrackerBlast</summary>

sizeof: `10688`  
</details>
<details>
<summary>client_class CFireSmoke</summary>

sizeof: `23696`  
</details>
<details>
<summary>client_class CFireTrail</summary>

sizeof: `2832`  
</details>
<details>
<summary>client_class CFish</summary>

sizeof: `10832`  
</details>
<details>
<summary>client_class CFists</summary>

sizeof: `13216`  
</details>
<details>
<summary>client_class CFlashbang</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CFogController</summary>

sizeof: `2600`  
</details>
<details>
<summary>client_class CFootstepControl</summary>

sizeof: `2608`  
</details>
<details>
<summary>client_class CFuncAreaPortalWindow</summary>

sizeof: `2536`  
</details>
<details>
<summary>client_class CFuncBrush</summary>

sizeof: `2520`  
</details>
<details>
<summary>client_class CFuncConveyor</summary>

sizeof: `2528`  
</details>
<details>
<summary>client_class CFuncLadder</summary>

sizeof: `2584`  
</details>
<details>
<summary>client_class CFuncMonitor</summary>

sizeof: `2520`  
</details>
<details>
<summary>client_class CFuncMoveLinear</summary>

sizeof: `2568`  
</details>
<details>
<summary>client_class CFuncOccluder</summary>

sizeof: `2528`  
</details>
<details>
<summary>client_class CFuncReflectiveGlass</summary>

sizeof: `2528`  
</details>
<details>
<summary>client_class CFuncRotating</summary>

sizeof: `2520`  
</details>
<details>
<summary>client_class CFuncSmokeVolume</summary>

sizeof: `3144`  
</details>
<details>
<summary>client_class CFuncTrackTrain</summary>

sizeof: `2536`  
</details>
<details>
<summary>client_class CFunc_Dust</summary>

sizeof: `2832`  
</details>
<details>
<summary>client_class CFunc_LOD</summary>

sizeof: `2528`  
</details>
<details>
<summary>client_class CGameRulesProxy</summary>

sizeof: `2520`  
</details>
<details>
<summary>client_class CGrassBurn</summary>

sizeof: `2552`  
</details>
<details>
<summary>client_class CHEGrenade</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CHandleTest</summary>

sizeof: `2528`  
</details>
<details>
<summary>client_class CHostage</summary>

sizeof: `12224`  
</details>
<details>
<summary>client_class CHostageCarriableProp</summary>

sizeof: `10640`  
</details>
<details>
<summary>client_class CIncendiaryGrenade</summary>

sizeof: `13248`  
</details>
<details>
<summary>client_class CInferno</summary>

sizeof: `10688`  
</details>
<details>
<summary>client_class CInfoLadderDismount</summary>

sizeof: `2520`  
</details>
<details>
<summary>client_class CInfoMapRegion</summary>

sizeof: `2656`  
</details>
<details>
<summary>client_class CInfoOverlayAccessor</summary>

sizeof: `2528`  
</details>
<details>
<summary>client_class CItem</summary>

sizeof: `13344`  
</details>
<details>
<summary>client_class CItemAssaultSuitUseable</summary>

sizeof: `13360`  
</details>
<details>
<summary>client_class CItemCash</summary>

sizeof: `13344`  
</details>
<details>
<summary>client_class CItemDogtags</summary>

sizeof: `13360`  
</details>
<details>
<summary>client_class CItem_Healthshot</summary>

sizeof: `13216`  
</details>
<details>
<summary>client_class CKnife</summary>

sizeof: `13200`  
</details>
<details>
<summary>client_class CKnifeGG</summary>

sizeof: `13200`  
</details>
<details>
<summary>client_class CLightGlow</summary>

sizeof: `2760`  
</details>
<details>
<summary>client_class CMaterialModifyControl</summary>

sizeof: `3344`  
</details>
<details>
<summary>client_class CMelee</summary>

sizeof: `13216`  
</details>
<details>
<summary>client_class CMolotovGrenade</summary>

sizeof: `13248`  
</details>
<details>
<summary>client_class CMolotovProjectile</summary>

sizeof: `10736`  
</details>
<details>
<summary>client_class CMovieDisplay</summary>

sizeof: `2800`  
</details>
<details>
<summary>client_class CParadropChopper</summary>

sizeof: `10656`  
</details>
<details>
<summary>client_class CParticleFire</summary>

sizeof: `6384`  
</details>
<details>
<summary>client_class CParticlePerformanceMonitor</summary>

sizeof: `2528`  
</details>
<details>
<summary>client_class CParticleSystem</summary>

sizeof: `3184`  
</details>
<details>
<summary>client_class CPhysBox</summary>

sizeof: `2528`  
</details>
<details>
<summary>client_class CPhysBoxMultiplayer</summary>

sizeof: `2544`  
</details>
<details>
<summary>client_class CPhysMagnet</summary>

sizeof: `10672`  
</details>
<details>
<summary>client_class CPhysPropAmmoBox</summary>

sizeof: `10736`  
</details>
<details>
<summary>client_class CPhysPropLootCrate</summary>

sizeof: `10752`  
</details>
<details>
<summary>client_class CPhysPropRadarJammer</summary>

sizeof: `10752`  
</details>
<details>
<summary>client_class CPhysPropWeaponUpgrade</summary>

sizeof: `10736`  
</details>
<details>
<summary>client_class CPhysicsProp</summary>

sizeof: `10688`  
</details>
<details>
<summary>client_class CPhysicsPropMultiplayer</summary>

sizeof: `10736`  
</details>
<details>
<summary>client_class CPlantedC4</summary>

sizeof: `10720`  
</details>
<details>
<summary>client_class CPlasma</summary>

sizeof: `21104`  
</details>
<details>
<summary>client_class CPlayerResource</summary>

sizeof: `5656`  
</details>
<details>
<summary>client_class CPointCamera</summary>

sizeof: `2560`  
</details>
<details>
<summary>client_class CPointCommentaryNode</summary>

sizeof: `11440`  
</details>
<details>
<summary>client_class CPointWorldText</summary>

sizeof: `2840`  
</details>
<details>
<summary>client_class CPoseController</summary>

sizeof: `2608`  
</details>
<details>
<summary>client_class CPostProcessController</summary>

sizeof: `2568`  
</details>
<details>
<summary>client_class CPrecipitation</summary>

sizeof: `2728`  
</details>
<details>
<summary>client_class CPrecipitationBlocker</summary>

sizeof: `645224`  
</details>
<details>
<summary>client_class CPredictedViewModel</summary>

sizeof: `11024`  
</details>
<details>
<summary>client_class CPropCounter</summary>

sizeof: `10640`  
</details>
<details>
<summary>client_class CPropDoorRotating</summary>

sizeof: `10736`  
</details>
<details>
<summary>client_class CPropJeep</summary>

sizeof: `10960`  
</details>
<details>
<summary>client_class CPropVehicleChoreoGeneric</summary>

sizeof: `10944`  
</details>
<details>
<summary>client_class CPropVehicleDriveable</summary>

sizeof: `10896`  
</details>
<details>
<summary>client_class CProp_Hallucination</summary>

sizeof: `10672`  
</details>
<details>
<summary>client_class CRagdollManager</summary>

sizeof: `2528`  
</details>
<details>
<summary>client_class CRagdollProp</summary>

sizeof: `11408`  
</details>
<details>
<summary>client_class CRagdollPropAttached</summary>

sizeof: `11472`  
</details>
<details>
<summary>client_class CRopeKeyframe</summary>

sizeof: `3416`  
</details>
<details>
<summary>client_class CSCAR17</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CSceneEntity</summary>

sizeof: `2600`  
</details>
<details>
<summary>client_class CSensorGrenade</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CSensorGrenadeProjectile</summary>

sizeof: `10736`  
</details>
<details>
<summary>client_class CShadowControl</summary>

sizeof: `2544`  
</details>
<details>
<summary>client_class CSlideshowDisplay</summary>

sizeof: `2856`  
</details>
<details>
<summary>client_class CSmokeGrenade</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CSmokeGrenadeProjectile</summary>

sizeof: `10736`  
</details>
<details>
<summary>client_class CSmokeStack</summary>

sizeof: `3016`  
</details>
<details>
<summary>client_class CSnowball</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CSnowballPile</summary>

sizeof: `10640`  
</details>
<details>
<summary>client_class CSnowballProjectile</summary>

sizeof: `10736`  
</details>
<details>
<summary>client_class CSpatialEntity</summary>

sizeof: `2816`  
</details>
<details>
<summary>client_class CSpotlightEnd</summary>

sizeof: `2536`  
</details>
<details>
<summary>client_class CSprite</summary>

sizeof: `2624`  
</details>
<details>
<summary>client_class CSpriteOriented</summary>

sizeof: `2624`  
</details>
<details>
<summary>client_class CSpriteTrail</summary>

sizeof: `4264`  
</details>
<details>
<summary>client_class CStatueProp</summary>

sizeof: `10736`  
</details>
<details>
<summary>client_class CSteamJet</summary>

sizeof: `2904`  
</details>
<details>
<summary>client_class CSun</summary>

sizeof: `2912`  
</details>
<details>
<summary>client_class CSunlightShadowControl</summary>

sizeof: `2848`  
</details>
<details>
<summary>client_class CSurvivalSpawnChopper</summary>

sizeof: `10624`  
</details>
<details>
<summary>client_class CTEArmorRicochet</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTEBSPDecal</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTEBaseBeam</summary>

sizeof: `4291726102`  
</details>
<details>
<summary>client_class CTEBeamEntPoint</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTEBeamEnts</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTEBeamFollow</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTEBeamLaser</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTEBeamPoints</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTEBeamRing</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTEBeamRingPoint</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTEBeamSpline</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTEBloodSprite</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTEBloodStream</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTEBreakModel</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTEBubbleTrail</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTEBubbles</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTEClientProjectile</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTEDecal</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTEDust</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTEDynamicLight</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTEEffectDispatch</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTEEnergySplash</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTEExplosion</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTEFireBullets</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTEFizz</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTEFootprintDecal</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTEFoundryHelpers</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTEGaussExplosion</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTEGlowSprite</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTEImpact</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTEKillPlayerAttachments</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTELargeFunnel</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTEMetalSparks</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTEMuzzleFlash</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTEParticleSystem</summary>

sizeof: `1713899207`  
</details>
<details>
<summary>client_class CTEPhysicsProp</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTEPlantBomb</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTEPlayerAnimEvent</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTEPlayerDecal</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTEProjectedDecal</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTERadioIcon</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTEShatterSurface</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTEShowLine</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTESmoke</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTESparks</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTESprite</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTESpriteSpray</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTEWorldDecal</summary>

sizeof: `0`  
</details>
<details>
<summary>client_class CTablet</summary>

sizeof: `15200`  
</details>
<details>
<summary>client_class CTeam</summary>

sizeof: `2936`  
</details>
<details>
<summary>client_class CTeamplayRoundBasedRulesProxy</summary>

sizeof: `2520`  
</details>
<details>
<summary>client_class CTesla</summary>

sizeof: `2880`  
</details>
<details>
<summary>client_class CTestTraceline</summary>

sizeof: `2528`  
</details>
<details>
<summary>client_class CTest_ProxyToggle_Networkable</summary>

sizeof: `2528`  
</details>
<details>
<summary>client_class CTriggerPlayerMovement</summary>

sizeof: `2584`  
</details>
<details>
<summary>client_class CTriggerSoundOperator</summary>

sizeof: `2584`  
</details>
<details>
<summary>client_class CVGuiScreen</summary>

sizeof: `2672`  
</details>
<details>
<summary>client_class CVoteController</summary>

sizeof: `2576`  
</details>
<details>
<summary>client_class CWaterBullet</summary>

sizeof: `10640`  
</details>
<details>
<summary>client_class CWaterLODControl</summary>

sizeof: `2528`  
</details>
<details>
<summary>client_class CWeaponAWP</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CWeaponAug</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CWeaponBaseItem</summary>

sizeof: `13216`  
</details>
<details>
<summary>client_class CWeaponBizon</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CWeaponCSBase</summary>

sizeof: `13200`  
</details>
<details>
<summary>client_class CWeaponCSBaseGun</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CWeaponCubemap</summary>

sizeof: `13008`  
</details>
<details>
<summary>client_class CWeaponCycler</summary>

sizeof: `13008`  
</details>
<details>
<summary>client_class CWeaponElite</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CWeaponFamas</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CWeaponFiveSeven</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CWeaponG3SG1</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CWeaponGalil</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CWeaponGalilAR</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CWeaponGlock</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CWeaponHKP2000</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CWeaponM249</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CWeaponM3</summary>

sizeof: `13216`  
</details>
<details>
<summary>client_class CWeaponM4A1</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CWeaponMAC10</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CWeaponMP5Navy</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CWeaponMP7</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CWeaponMP9</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CWeaponMag7</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CWeaponNOVA</summary>

sizeof: `13216`  
</details>
<details>
<summary>client_class CWeaponNegev</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CWeaponP228</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CWeaponP250</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CWeaponP90</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CWeaponSCAR20</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CWeaponSG550</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CWeaponSG552</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CWeaponSG556</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CWeaponSSG08</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CWeaponSawedoff</summary>

sizeof: `13216`  
</details>
<details>
<summary>client_class CWeaponScout</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CWeaponTMP</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CWeaponTaser</summary>

sizeof: `13248`  
</details>
<details>
<summary>client_class CWeaponTec9</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CWeaponUMP45</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CWeaponUSP</summary>

sizeof: `13232`  
</details>
<details>
<summary>client_class CWeaponXM1014</summary>

sizeof: `13216`  
</details>
<details>
<summary>client_class CWorld</summary>

sizeof: `3435971421`  
</details>
<details>
<summary>client_class CWorldVguiText</summary>

sizeof: `3368`  
</details>
<details>
<summary>client_class DustTrail</summary>

sizeof: `2912`  
</details>
<details>
<summary>client_class MovieExplosion</summary>

sizeof: `4576`  
</details>
<details>
<summary>client_class NextBotCombatCharacter</summary>

sizeof: `12096`  
</details>
<details>
<summary>client_class ParticleSmokeGrenade</summary>

sizeof: `11800`  
</details>
<details>
<summary>client_class RocketTrail</summary>

sizeof: `2880`  
</details>
<details>
<summary>client_class SmokeTrail</summary>

sizeof: `2872`  
</details>
<details>
<summary>client_class SporeExplosion</summary>

sizeof: `2808`  
</details>
<details>
<summary>client_class SporeTrail</summary>

sizeof: `2904`  
</details>

```
client_panorama.dll!0x00ccaaf0 ClientClass CAI_BaseNPC
client_panorama.dll!0x00cf22dc ClientClass CAK47
client_panorama.dll!0x00ceefc0 ClientClass CBRC4Target
client_panorama.dll!0x00ccaf88 ClientClass CBaseAnimating
client_panorama.dll!0x00ccb22c ClientClass CBaseAnimatingOverlay
client_panorama.dll!0x00cf311c ClientClass CBaseAttributableItem
client_panorama.dll!0x00ccc0e8 ClientClass CBaseButton
client_panorama.dll!0x00cf0bdc ClientClass CBaseCSGrenade
client_panorama.dll!0x00ce2118 ClientClass CBaseCSGrenadeProjectile
client_panorama.dll!0x00ccb248 ClientClass CBaseCombatCharacter
client_panorama.dll!0x00cca1ec ClientClass CBaseCombatWeapon
client_panorama.dll!0x00ccb264 ClientClass CBaseDoor
client_panorama.dll!0x00ccb4a8 ClientClass CBaseEntity
client_panorama.dll!0x00ccb900 ClientClass CBaseFlex
client_panorama.dll!0x00cca3c0 ClientClass CBaseGrenade
client_panorama.dll!0x00cca3dc ClientClass CBaseParticleEntity
client_panorama.dll!0x00ccc030 ClientClass CBasePlayer
client_panorama.dll!0x00cce1ec ClientClass CBasePropDoor
client_panorama.dll!0x00ccf0e0 ClientClass CBaseTeamObjectiveResource
client_panorama.dll!0x00cf4fb0 ClientClass CBaseTempEntity
client_panorama.dll!0x00ccc070 ClientClass CBaseToggle
client_panorama.dll!0x00ccf158 ClientClass CBaseTrigger
client_panorama.dll!0x00ccf198 ClientClass CBaseVPhysicsTrigger
client_panorama.dll!0x00ccaa28 ClientClass CBaseViewModel
client_panorama.dll!0x00cca1d0 ClientClass CBaseWeaponWorldModel
client_panorama.dll!0x00ccaa44 ClientClass CBeam
client_panorama.dll!0x00ccc318 ClientClass CBeamSpotlight
client_panorama.dll!0x00ccb210 ClientClass CBoneFollower
client_panorama.dll!0x00cf0e9c ClientClass CBreachCharge
client_panorama.dll!0x00cf0e80 ClientClass CBreachChargeProjectile
client_panorama.dll!0x00ccc334 ClientClass CBreakableProp
client_panorama.dll!0x00ccd738 ClientClass CBreakableSurface
client_panorama.dll!0x00cf0f20 ClientClass CC4
client_panorama.dll!0x00ce54f0 ClientClass CCSGameRulesProxy
client_panorama.dll!0x00ce3280 ClientClass CCSPlayer
client_panorama.dll!0x00ce3ba0 ClientClass CCSPlayerResource
client_panorama.dll!0x00ce308c ClientClass CCSRagdoll
client_panorama.dll!0x00ce3c18 ClientClass CCSTeam
client_panorama.dll!0x00ccd3e4 ClientClass CCascadeLight
client_panorama.dll!0x00cecddc ClientClass CChicken
client_panorama.dll!0x00ccc400 ClientClass CColorCorrection
client_panorama.dll!0x00ccc41c ClientClass CColorCorrectionVolume
client_panorama.dll!0x00cf265c ClientClass CDEagle
client_panorama.dll!0x00cd136c ClientClass CDangerZone
client_panorama.dll!0x00cd1350 ClientClass CDangerZoneController
client_panorama.dll!0x00cf2678 ClientClass CDecoyGrenade
client_panorama.dll!0x00cf0914 ClientClass CDecoyProjectile
client_panorama.dll!0x00cec5e4 ClientClass CDrone
client_panorama.dll!0x00cef168 ClientClass CDronegun
client_panorama.dll!0x00ccc438 ClientClass CDynamicLight
client_panorama.dll!0x00cce1d0 ClientClass CDynamicProp
client_panorama.dll!0x00cf3100 ClientClass CEconEntity
client_panorama.dll!0x00cf35a8 ClientClass CEconWearable
client_panorama.dll!0x00cf5cf0 ClientClass CEmbers
client_panorama.dll!0x00ccc4c0 ClientClass CEntityDissolve
client_panorama.dll!0x00ccc4dc ClientClass CEntityFlame
client_panorama.dll!0x00ccc550 ClientClass CEntityFreezing
client_panorama.dll!0x00ccc56c ClientClass CEntityParticleTrail
client_panorama.dll!0x00ccc5e0 ClientClass CEnvAmbientLight
client_panorama.dll!0x00ccd560 ClientClass CEnvDOFController
client_panorama.dll!0x00cd1758 ClientClass CEnvDetailController
client_panorama.dll!0x00cef184 ClientClass CEnvGasCanister
client_panorama.dll!0x00ccd598 ClientClass CEnvParticleScript
client_panorama.dll!0x00ccd5b4 ClientClass CEnvProjectedTexture
client_panorama.dll!0x00cf5d0c ClientClass CEnvQuadraticBeam
client_panorama.dll!0x00ccd644 ClientClass CEnvScreenEffect
client_panorama.dll!0x00ccd628 ClientClass CEnvScreenOverlay
client_panorama.dll!0x00ccd660 ClientClass CEnvTonemapController
client_panorama.dll!0x00cf5cd4 ClientClass CEnvWind
client_panorama.dll!0x00cf6a88 ClientClass CFEPlayerDecal
client_panorama.dll!0x00cecf2c ClientClass CFireCrackerBlast
client_panorama.dll!0x00ccd68c ClientClass CFireSmoke
client_panorama.dll!0x00cf6090 ClientClass CFireTrail
client_panorama.dll!0x00ccd700 ClientClass CFish
client_panorama.dll!0x00cf2928 ClientClass CFists
client_panorama.dll!0x00cf2954 ClientClass CFlashbang
client_panorama.dll!0x00ccd57c ClientClass CFogController
client_panorama.dll!0x00cebae8 ClientClass CFootstepControl
client_panorama.dll!0x00ccd71c ClientClass CFuncAreaPortalWindow
client_panorama.dll!0x00ccd754 ClientClass CFuncBrush
client_panorama.dll!0x00ccd770 ClientClass CFuncConveyor
client_panorama.dll!0x00cd1f20 ClientClass CFuncLadder
client_panorama.dll!0x00ccd7e0 ClientClass CFuncMonitor
client_panorama.dll!0x00ccd7fc ClientClass CFuncMoveLinear
client_panorama.dll!0x00ccd818 ClientClass CFuncOccluder
client_panorama.dll!0x00ccd834 ClientClass CFuncReflectiveGlass
client_panorama.dll!0x00ccd858 ClientClass CFuncRotating
client_panorama.dll!0x00ccd8c0 ClientClass CFuncSmokeVolume
client_panorama.dll!0x00ccd8dc ClientClass CFuncTrackTrain
client_panorama.dll!0x00ccd78c ClientClass CFunc_Dust
client_panorama.dll!0x00ccd7c4 ClientClass CFunc_LOD
client_panorama.dll!0x00cd2ad4 ClientClass CGameRulesProxy
client_panorama.dll!0x00cd30e4 ClientClass CGrassBurn
client_panorama.dll!0x00cf2970 ClientClass CHEGrenade
client_panorama.dll!0x00cdc128 ClientClass CHandleTest
client_panorama.dll!0x00ce21cc ClientClass CHostage
client_panorama.dll!0x00ce21b0 ClientClass CHostageCarriableProp
client_panorama.dll!0x00cf2acc ClientClass CIncendiaryGrenade
client_panorama.dll!0x00cecf10 ClientClass CInferno
client_panorama.dll!0x00cd1f3c ClientClass CInfoLadderDismount
client_panorama.dll!0x00ceef9c ClientClass CInfoMapRegion
client_panorama.dll!0x00ccdc14 ClientClass CInfoOverlayAccessor
client_panorama.dll!0x00ccdc50 ClientClass CItem
client_panorama.dll!0x00ccdc6c ClientClass CItemAssaultSuitUseable
client_panorama.dll!0x00cef1b0 ClientClass CItemCash
client_panorama.dll!0x00cf0930 ClientClass CItemDogtags
client_panorama.dll!0x00cf095c ClientClass CItem_Healthshot
client_panorama.dll!0x00cf2a00 ClientClass CKnife
client_panorama.dll!0x00cf2a2c ClientClass CKnifeGG
client_panorama.dll!0x00ccdc88 ClientClass CLightGlow
client_panorama.dll!0x00ccdd00 ClientClass CMaterialModifyControl
client_panorama.dll!0x00cf2a64 ClientClass CMelee
client_panorama.dll!0x00cf2a90 ClientClass CMolotovGrenade
client_panorama.dll!0x00cf0b40 ClientClass CMolotovProjectile
client_panorama.dll!0x00ccdd1c ClientClass CMovieDisplay
client_panorama.dll!0x00ceeff8 ClientClass CParadropChopper
client_panorama.dll!0x00cf5ee8 ClientClass CParticleFire
client_panorama.dll!0x00ccefc0 ClientClass CParticlePerformanceMonitor
client_panorama.dll!0x00ccdd38 ClientClass CParticleSystem
client_panorama.dll!0x00ccdd8c ClientClass CPhysBox
client_panorama.dll!0x00cce224 ClientClass CPhysBoxMultiplayer
client_panorama.dll!0x00ccde90 ClientClass CPhysMagnet
client_panorama.dll!0x00cf08c0 ClientClass CPhysPropAmmoBox
client_panorama.dll!0x00cf08a4 ClientClass CPhysPropLootCrate
client_panorama.dll!0x00cf08f8 ClientClass CPhysPropRadarJammer
client_panorama.dll!0x00cf08dc ClientClass CPhysPropWeaponUpgrade
client_panorama.dll!0x00ccddc4 ClientClass CPhysicsProp
client_panorama.dll!0x00cce240 ClientClass CPhysicsPropMultiplayer
client_panorama.dll!0x00ce3c34 ClientClass CPlantedC4
client_panorama.dll!0x00cce044 ClientClass CPlasma
client_panorama.dll!0x00cce110 ClientClass CPlayerResource
client_panorama.dll!0x00cce12c ClientClass CPointCamera
client_panorama.dll!0x00cce150 ClientClass CPointCommentaryNode
client_panorama.dll!0x00cce17c ClientClass CPointWorldText
client_panorama.dll!0x00cda718 ClientClass CPoseController
client_panorama.dll!0x00cce198 ClientClass CPostProcessController
client_panorama.dll!0x00cf5474 ClientClass CPrecipitation
client_panorama.dll!0x00cf5458 ClientClass CPrecipitationBlocker
client_panorama.dll!0x00cda7f4 ClientClass CPredictedViewModel
client_panorama.dll!0x00cdaee0 ClientClass CPropCounter
client_panorama.dll!0x00cce208 ClientClass CPropDoorRotating
client_panorama.dll!0x00ccf300 ClientClass CPropJeep
client_panorama.dll!0x00ccf1d8 ClientClass CPropVehicleChoreoGeneric
client_panorama.dll!0x00cf5f20 ClientClass CPropVehicleDriveable
client_panorama.dll!0x00cce1b4 ClientClass CProp_Hallucination
client_panorama.dll!0x00cce25c ClientClass CRagdollManager
client_panorama.dll!0x00cdb158 ClientClass CRagdollProp
client_panorama.dll!0x00cdb174 ClientClass CRagdollPropAttached
client_panorama.dll!0x00cce288 ClientClass CRopeKeyframe
client_panorama.dll!0x00cf2528 ClientClass CSCAR17
client_panorama.dll!0x00cceb88 ClientClass CSceneEntity
client_panorama.dll!0x00cf2b30 ClientClass CSensorGrenade
client_panorama.dll!0x00cf0b5c ClientClass CSensorGrenadeProjectile
client_panorama.dll!0x00ccec5c ClientClass CShadowControl
client_panorama.dll!0x00ccec78 ClientClass CSlideshowDisplay
client_panorama.dll!0x00cf2b68 ClientClass CSmokeGrenade
client_panorama.dll!0x00cf0b78 ClientClass CSmokeGrenadeProjectile
client_panorama.dll!0x00cf60d8 ClientClass CSmokeStack
client_panorama.dll!0x00cf2b84 ClientClass CSnowball
client_panorama.dll!0x00cf0b94 ClientClass CSnowballPile
client_panorama.dll!0x00cf0bc0 ClientClass CSnowballProjectile
client_panorama.dll!0x00ccef2c ClientClass CSpatialEntity
client_panorama.dll!0x00ccef48 ClientClass CSpotlightEnd
client_panorama.dll!0x00cdbad8 ClientClass CSprite
client_panorama.dll!0x00cdbaf4 ClientClass CSpriteOriented
client_panorama.dll!0x00cdbb10 ClientClass CSpriteTrail
client_panorama.dll!0x00ccdda8 ClientClass CStatueProp
client_panorama.dll!0x00cf60f4 ClientClass CSteamJet
client_panorama.dll!0x00ccefdc ClientClass CSun
client_panorama.dll!0x00ccf0a8 ClientClass CSunlightShadowControl
client_panorama.dll!0x00ceefdc ClientClass CSurvivalSpawnChopper
client_panorama.dll!0x00cf61b4 ClientClass CTEArmorRicochet
client_panorama.dll!0x00cf65a4 ClientClass CTEBSPDecal
client_panorama.dll!0x00cf61d0 ClientClass CTEBaseBeam
client_panorama.dll!0x00cf625c ClientClass CTEBeamEntPoint
client_panorama.dll!0x00cf62cc ClientClass CTEBeamEnts
client_panorama.dll!0x00cf6338 ClientClass CTEBeamFollow
client_panorama.dll!0x00cf63ac ClientClass CTEBeamLaser
client_panorama.dll!0x00cf642c ClientClass CTEBeamPoints
client_panorama.dll!0x00cf649c ClientClass CTEBeamRing
client_panorama.dll!0x00cf6518 ClientClass CTEBeamRingPoint
client_panorama.dll!0x00cf6534 ClientClass CTEBeamSpline
client_panorama.dll!0x00cf6550 ClientClass CTEBloodSprite
client_panorama.dll!0x00cf656c ClientClass CTEBloodStream
client_panorama.dll!0x00cf6588 ClientClass CTEBreakModel
client_panorama.dll!0x00cf65dc ClientClass CTEBubbleTrail
client_panorama.dll!0x00cf65c0 ClientClass CTEBubbles
client_panorama.dll!0x00cf65f8 ClientClass CTEClientProjectile
client_panorama.dll!0x00cf6614 ClientClass CTEDecal
client_panorama.dll!0x00ccd7a8 ClientClass CTEDust
client_panorama.dll!0x00cf6630 ClientClass CTEDynamicLight
client_panorama.dll!0x00cf66c4 ClientClass CTEEffectDispatch
client_panorama.dll!0x00cf66fc ClientClass CTEEnergySplash
client_panorama.dll!0x00cf6718 ClientClass CTEExplosion
client_panorama.dll!0x00ce3c80 ClientClass CTEFireBullets
client_panorama.dll!0x00cf6754 ClientClass CTEFizz
client_panorama.dll!0x00cf6770 ClientClass CTEFootprintDecal
client_panorama.dll!0x00cd1f04 ClientClass CTEFoundryHelpers
client_panorama.dll!0x00cf5e14 ClientClass CTEGaussExplosion
client_panorama.dll!0x00cf6814 ClientClass CTEGlowSprite
client_panorama.dll!0x00cf6830 ClientClass CTEImpact
client_panorama.dll!0x00cf6860 ClientClass CTEKillPlayerAttachments
client_panorama.dll!0x00cf687c ClientClass CTELargeFunnel
client_panorama.dll!0x00cf616c ClientClass CTEMetalSparks
client_panorama.dll!0x00cf6978 ClientClass CTEMuzzleFlash
client_panorama.dll!0x00cf6994 ClientClass CTEParticleSystem
client_panorama.dll!0x00cf69b0 ClientClass CTEPhysicsProp
client_panorama.dll!0x00ce3cc0 ClientClass CTEPlantBomb
client_panorama.dll!0x00ce3070 ClientClass CTEPlayerAnimEvent
client_panorama.dll!0x00cf6a6c ClientClass CTEPlayerDecal
client_panorama.dll!0x00cf6b00 ClientClass CTEProjectedDecal
client_panorama.dll!0x00ce3c64 ClientClass CTERadioIcon
client_panorama.dll!0x00cf67f8 ClientClass CTEShatterSurface
client_panorama.dll!0x00cf6b1c ClientClass CTEShowLine
client_panorama.dll!0x00cf6b38 ClientClass CTESmoke
client_panorama.dll!0x00cf6b54 ClientClass CTESparks
client_panorama.dll!0x00cf6b70 ClientClass CTESprite
client_panorama.dll!0x00cf6b8c ClientClass CTESpriteSpray
client_panorama.dll!0x00cf6ba8 ClientClass CTEWorldDecal
client_panorama.dll!0x00cf2dd0 ClientClass CTablet
client_panorama.dll!0x00ccf0c4 ClientClass CTeam
client_panorama.dll!0x00cdbd98 ClientClass CTeamplayRoundBasedRulesProxy
client_panorama.dll!0x00ccf0fc ClientClass CTesla
client_panorama.dll!0x00cf6bc4 ClientClass CTestTraceline
client_panorama.dll!0x00ccf118 ClientClass CTest_ProxyToggle_Networkable
client_panorama.dll!0x00ccf174 ClientClass CTriggerPlayerMovement
client_panorama.dll!0x00ccf1b4 ClientClass CTriggerSoundOperator
client_panorama.dll!0x00ccf3c4 ClientClass CVGuiScreen
client_panorama.dll!0x00ccf3e0 ClientClass CVoteController
client_panorama.dll!0x00ccf3fc ClientClass CWaterBullet
client_panorama.dll!0x00ccf418 ClientClass CWaterLODControl
client_panorama.dll!0x00cf2314 ClientClass CWeaponAWP
client_panorama.dll!0x00cf22f8 ClientClass CWeaponAug
client_panorama.dll!0x00cf0bf8 ClientClass CWeaponBaseItem
client_panorama.dll!0x00cf2330 ClientClass CWeaponBizon
client_panorama.dll!0x00cf1478 ClientClass CWeaponCSBase
client_panorama.dll!0x00cf22c0 ClientClass CWeaponCSBaseGun
client_panorama.dll!0x00ce4464 ClientClass CWeaponCubemap
client_panorama.dll!0x00ce4448 ClientClass CWeaponCycler
client_panorama.dll!0x00cf2694 ClientClass CWeaponElite
client_panorama.dll!0x00cf234c ClientClass CWeaponFamas
client_panorama.dll!0x00cf2368 ClientClass CWeaponFiveSeven
client_panorama.dll!0x00cf2384 ClientClass CWeaponG3SG1
client_panorama.dll!0x00cf23a0 ClientClass CWeaponGalil
client_panorama.dll!0x00cf23bc ClientClass CWeaponGalilAR
client_panorama.dll!0x00cf23d8 ClientClass CWeaponGlock
client_panorama.dll!0x00cf23f4 ClientClass CWeaponHKP2000
client_panorama.dll!0x00cf2624 ClientClass CWeaponM249
client_panorama.dll!0x00cf2a48 ClientClass CWeaponM3
client_panorama.dll!0x00cf2410 ClientClass CWeaponM4A1
client_panorama.dll!0x00cf242c ClientClass CWeaponMAC10
client_panorama.dll!0x00cf2464 ClientClass CWeaponMP5Navy
client_panorama.dll!0x00cf2480 ClientClass CWeaponMP7
client_panorama.dll!0x00cf249c ClientClass CWeaponMP9
client_panorama.dll!0x00cf2448 ClientClass CWeaponMag7
client_panorama.dll!0x00cf2af8 ClientClass CWeaponNOVA
client_panorama.dll!0x00cf24b8 ClientClass CWeaponNegev
client_panorama.dll!0x00cf24d4 ClientClass CWeaponP228
client_panorama.dll!0x00cf24f0 ClientClass CWeaponP250
client_panorama.dll!0x00cf250c ClientClass CWeaponP90
client_panorama.dll!0x00cf2544 ClientClass CWeaponSCAR20
client_panorama.dll!0x00cf257c ClientClass CWeaponSG550
client_panorama.dll!0x00cf2b4c ClientClass CWeaponSG552
client_panorama.dll!0x00cf2598 ClientClass CWeaponSG556
client_panorama.dll!0x00cf25b4 ClientClass CWeaponSSG08
client_panorama.dll!0x00cf2b14 ClientClass CWeaponSawedoff
client_panorama.dll!0x00cf2560 ClientClass CWeaponScout
client_panorama.dll!0x00cf25ec ClientClass CWeaponTMP
client_panorama.dll!0x00cf2f08 ClientClass CWeaponTaser
client_panorama.dll!0x00cf25d0 ClientClass CWeaponTec9
client_panorama.dll!0x00cf2608 ClientClass CWeaponUMP45
client_panorama.dll!0x00cf2640 ClientClass CWeaponUSP
client_panorama.dll!0x00cf2f34 ClientClass CWeaponXM1014
client_panorama.dll!0x00ccf434 ClientClass CWorld
client_panorama.dll!0x00ccf460 ClientClass CWorldVguiText
client_panorama.dll!0x00cf60ac ClientClass DustTrail
client_panorama.dll!0x00cf5e30 ClientClass MovieExplosion
client_panorama.dll!0x00cf3998 ClientClass NextBotCombatCharacter
client_panorama.dll!0x00cf5f04 ClientClass ParticleSmokeGrenade
client_panorama.dll!0x00cf5fc8 ClientClass RocketTrail
client_panorama.dll!0x00cf5fac ClientClass SmokeTrail
client_panorama.dll!0x00cf5fe4 ClientClass SporeExplosion
client_panorama.dll!0x00cf6074 ClientClass SporeTrail
```

class CBaseWeaponWorldModel extends C_BaseAnimatingOverlay {
	// field offset: 0xf0
	m_fEffects: Integer,
	// field offset: 0x258
	m_nModelIndex: Short,
	// field offset: 0xa20
	m_nBody: Integer,
	// field offset: 0x29f0
	m_hCombatWeaponParent: EHandle,
}
class CBaseCombatWeapon extends C_BaseFlex {
	// field offset: 0xfc
	m_nNextThinkTick: Integer,
	// field offset: 0x3210
	m_hOwner: EHandle,
	// field offset: 0x3214
	m_nViewModelIndex: Integer,
	// field offset: 0x3218
	m_flNextPrimaryAttack: Float,
	// field offset: 0x321c
	m_flNextSecondaryAttack: Float,
	// field offset: 0x3220
	m_iViewModelIndex: Integer,
	// field offset: 0x3224
	m_iWorldModelIndex: Integer,
	// field offset: 0x3228
	m_iWorldDroppedModelIndex: Integer,
	// field offset: 0x322c
	m_iWeaponModule: Integer,
	// field offset: 0x3230
	m_iNumEmptyAttacks: Integer,
	// field offset: 0x3238
	m_iState: Integer,
	// field offset: 0x323c
	m_iPrimaryAmmoType: Integer,
	// field offset: 0x3240
	m_iSecondaryAmmoType: Integer,
	// field offset: 0x3244
	m_iClip1: Integer,
	// field offset: 0x3248
	m_iClip2: Integer,
	// field offset: 0x324c
	m_iPrimaryReserveAmmoCount: Integer,
	// field offset: 0x3250
	m_iSecondaryReserveAmmoCount: Integer,
	// field offset: 0x3254
	m_flTimeWeaponIdle: Float,
	// field offset: 0x3258
	m_flNextEmptySoundTime: Float,
	// field offset: 0x325c
	m_fMinRange1: Float,
	// field offset: 0x3260
	m_fMinRange2: Float,
	// field offset: 0x3264
	m_fMaxRange1: Float,
	// field offset: 0x3268
	m_fMaxRange2: Float,
	// field offset: 0x326c
	m_fFireDuration: Float,
	// field offset: 0x3274
	m_Activity: Integer,
	// field offset: 0x3278
	m_iPrimaryAmmoCount: Integer,
	// field offset: 0x327c
	m_iSecondaryAmmoCount: Integer,
	// field offset: 0x3280
	m_iszName: Integer,
	// field offset: 0x3284
	m_bRemoveable: Boolean,
	// field offset: 0x3285
	m_bInReload: Boolean,
	// field offset: 0x3286
	m_bFireOnEmpty: Boolean,
	// field offset: 0x3287
	m_bFiresUnderwater: Boolean,
	// field offset: 0x3288
	m_bAltFiresUnderwater: Boolean,
	// field offset: 0x3289
	m_bReloadsSingly: Boolean,
}
class CBaseGrenade extends C_BaseAnimating {
	// field offset: 0x114
	m_vecVelocity: Vector,
	// field offset: 0x2981
	m_bIsLive: Boolean,
	// field offset: 0x2984
	m_DmgRadius: Boolean,
	// field offset: 0x2988
	m_flNextAttack: Float,
	// field offset: 0x2998
	m_flDamage: Float,
	// field offset: 0x29a0
	m_hThrower: EHandle,
}
class CBaseViewModel extends C_BaseAnimating {
	// field offset: 0xf0
	m_fEffects: Integer,
	// field offset: 0x258
	m_nModelIndex: Short,
	// field offset: 0x260
	m_flAnimTime: Float,
	// field offset: 0xa14
	m_flCycle: Float,
	// field offset: 0xa18
	m_flPlaybackRate: Float,
	// field offset: 0xa1c
	m_nSkin: Integer,
	// field offset: 0xa20
	m_nBody: Integer,
	// field offset: 0x28bc
	m_nSequence: Integer,
	// field offset: 0x29c0
	m_nViewModelIndex: Integer,
	// field offset: 0x29c4
	m_nAnimationParity: Integer,
	// field offset: 0x29c8
	m_hWeapon: EHandle,
	// field offset: 0x29cc
	m_hOwner: EHandle,
	// field offset: 0x29d0
	m_flTimeWeaponIdle: Float,
	// field offset: 0x29d4
	m_Activity: Integer,
}
class CBeam extends C_BaseEntity {
	// field offset: 0x70
	m_clrRender: Integer,
	// field offset: 0xac
	m_vecOrigin: Vector,
	// field offset: 0x258
	m_nModelIndex: Integer,
	// field offset: 0x25a
	m_nRenderFX: Integer,
	// field offset: 0x25b
	m_nRenderMode: Integer,
	// field offset: 0x9d8
	m_flFrameRate: Float,
	// field offset: 0x9e8
	m_nNumBeamEnts: Integer,
	// field offset: 0x9f0
	m_nHaloIndex: Integer,
	// field offset: 0x9f4
	m_nBeamType: Integer,
	// field offset: 0x9fc
	m_hAttachEntity: EHandle,
	// field offset: 0xa24
	m_nAttachIndex: Integer,
	// field offset: 0xa4c
	m_fWidth: Float,
	// field offset: 0xa50
	m_fEndWidth: Float,
	// field offset: 0xa54
	m_fFadeLength: Float,
	// field offset: 0xa58
	m_fHaloScale: Float,
	// field offset: 0xa5c
	m_fAmplitude: Float,
	// field offset: 0xa60
	m_fStartFrame: Float,
	// field offset: 0xa64
	m_fSpeed: Float,
	// field offset: 0xa68
	m_flFrame: Float,
	// field offset: 0xa70
	m_vecEndPos: Vector,
}
class C_BaseAnimating extends C_BaseEntity {
	// field offset: 0xa18
	m_flPlaybackRate: Float,
	// field offset: 0xa1c
	m_nSkin: Integer,
	// field offset: 0xa20
	m_nBody: Integer,
	// field offset: 0xa44
	m_nNewSequenceParity: Integer,
	// field offset: 0xa48
	m_nResetEventsParity: Integer,
	// field offset: 0xa54
	m_flEncodedController: Float,
	// field offset: 0xa64
	m_nMuzzleFlashParity: Char,
}
class C_BaseCombatCharacter extends C_BaseFlex {
	// field offset: 0x2d70
	m_flNextAttack: Float,
	// field offset: 0x2d78
	m_iAmmo: Integer,
	// field offset: 0x2df8
	m_hMyWeapons: EHandle,
	// field offset: 0x2ef8
	m_hActiveWeapon: EHandle,
}
class C_BaseEntity {
	// field offset: 0x94
	m_vecAbsVelocity: Vector,
	// field offset: 0xa0
	m_vecAbsOrigin: Vector,
	// field offset: 0xac
	m_vecOrigin: Vector,
	// field offset: 0xb8
	m_vecAngVelocity: Vector,
	// field offset: 0xc4
	m_angAbsRotation: Vector,
	// field offset: 0xd0
	m_angRotation: Vector,
	// field offset: 0xdc
	m_flMaxFallVelocity: Float,
	// field offset: 0xe0
	m_flGravity: Float,
	// field offset: 0xe4
	m_flProxyRandomValue: Float,
	// field offset: 0xe8
	m_iEFlags: Integer,
	// field offset: 0xec
	m_nWaterType: Char,
	// field offset: 0xf0
	m_fEffects: Integer,
	// field offset: 0xf4
	m_iTeamNum: Integer,
	// field offset: 0xf8
	m_iPendingTeamNum: Integer,
	// field offset: 0x100
	m_iHealth: Integer,
	// field offset: 0x104
	m_fFlags: Integer,
	// field offset: 0x108
	m_vecViewOffset: Vector,
	// field offset: 0x114
	m_vecVelocity: Vector,
	// field offset: 0x120
	m_vecBaseVelocity: Vector,
	// field offset: 0x12c
	m_angNetworkAngles: Vector,
	// field offset: 0x138
	m_vecNetworkOrigin: Vector,
	// field offset: 0x144
	m_flFriction: Float,
	// field offset: 0x148
	m_hNetworkMoveParent: EHandle,
	// field offset: 0x14c
	m_hOwnerEntity: EHandle,
	// field offset: 0x150
	m_hGroundEntity: EHandle,
	// field offset: 0x258
	m_nModelIndex: Short,
	// field offset: 0x25a
	m_nRenderFX: Char,
	// field offset: 0x25b
	m_nRenderMode: Char,
	// field offset: 0x25c
	m_MoveType: Char,
	// field offset: 0x25d
	m_MoveCollide: Char,
	// field offset: 0x25e
	m_nWaterLevel: Char,
	// field offset: 0x2cc
	m_flUseLookAtAngle: Float,
	// field offset: 0x31c
	m_Collision: CCollisionProperty,
	// field offset: 0x938
	m_bEverHadPredictionErrorsForThisCommand: Boolean,
}
class CPlayerState {
	// field offset: 0x4
	deadflag: Boolean,
}
class CPlayerLocalData {
	// field offset: 0x3c
	m_nStepside: Integer,
	// field offset: 0x40
	m_nOldButtons: Integer,
	// field offset: 0x44
	m_flFOVRate: Float,
	// field offset: 0x48
	m_iHideHUD: Integer,
	// field offset: 0x4c
	m_nDuckTimeMsecs: Integer,
	// field offset: 0x50
	m_nDuckJumpTimeMsecs: Integer,
	// field offset: 0x54
	m_nJumpTimeMsecs: Integer,
	// field offset: 0x58
	m_flFallVelocity: Float,
	// field offset: 0x60
	m_flStepSize: Float,
	// field offset: 0x64
	m_viewPunchAngle: Vector,
	// field offset: 0x70
	m_aimPunchAngle: Vector,
	// field offset: 0x7c
	m_aimPunchAngleVel: Vector,
	// field offset: 0x88
	m_bDucked: Boolean,
	// field offset: 0x89
	m_bDucking: Boolean,
	// field offset: 0x8c
	m_flLastDuckTime: Float,
	// field offset: 0x90
	m_bInDuckJump: Boolean,
	// field offset: 0x91
	m_bDrawViewmodel: Boolean,
	// field offset: 0x92
	m_bWearingSuit: Boolean,
	// field offset: 0x93
	m_bPoisoned: Boolean,
	// field offset: 0x94
	m_bAllowAutoMovement: Boolean,
}
class C_BasePlayer extends C_BaseCombatCharacter {
	// field offset: 0xfc
	m_nNextThinkTick: Integer,
	// field offset: 0x100
	m_iHealth: Integer,
	// field offset: 0x120
	m_vecBaseVelocity: Vector,
	// field offset: 0x150
	m_hGroundEntity: EHandle,
	// field offset: 0x25e
	m_nWaterLevel: Char,
	// field offset: 0x25f
	m_lifeState: Char,
	// field offset: 0x2fac
	m_flDuckAmount: Float,
	// field offset: 0x2fb0
	m_flDuckSpeed: Float,
	// field offset: 0x2fbc
	m_Local: CPlayerLocalData,
	// field offset: 0x31d0
	pl: CPlayerState,
	// field offset: 0x31e4
	m_iFOV: Integer,
	// field offset: 0x31e8
	m_iFOVStart: Integer,
	// field offset: 0x31ec
	m_afButtonLast: Integer,
	// field offset: 0x31f0
	m_afButtonPressed: Integer,
	// field offset: 0x31f4
	m_afButtonReleased: Integer,
	// field offset: 0x31f8
	m_nButtons: Integer,
	// field offset: 0x31fc
	m_nImpulse: Integer,
	// field offset: 0x3200
	m_ladderSurfaceProps: Integer,
	// field offset: 0x3204
	m_flPhysics: Integer,
	// field offset: 0x3208
	m_flFOVTime: Float,
	// field offset: 0x320c
	m_flWaterJumpTime: Float,
	// field offset: 0x3210
	m_flSwimSoundTime: Float,
	// field offset: 0x3214
	m_ignoreLadderJumpTime: Float,
	// field offset: 0x3218
	m_bHasWalkMovedSinceLastJump: Boolean,
	// field offset: 0x321c
	m_flStepSoundTime: Float,
	// field offset: 0x322c
	m_surfaceFriction: Float,
	// field offset: 0x3230
	m_vecLadderNormal: Vector,
	// field offset: 0x3240
	m_iBonusProgress: Integer,
	// field offset: 0x3244
	m_iBonusChallenge: Integer,
	// field offset: 0x3248
	m_flMaxspeed: Float,
	// field offset: 0x324c
	m_hZoomOwner: EHandle,
	// field offset: 0x325c
	m_vphysicsCollisionState: Integer,
	// field offset: 0x3260
	m_oldOrigin: Vector,
	// field offset: 0x326c
	m_bTouchedPhysObject: Boolean,
	// field offset: 0x326d
	m_bPhysicsWasFrozen: Boolean,
	// field offset: 0x3270
	m_vNewVPhysicsPosition: Vector,
	// field offset: 0x327c
	m_vNewVPhysicsVelocity: Vector,
	// field offset: 0x32ec
	m_afPhysicsFlags: Integer,
	// field offset: 0x32f0
	m_hVehicle: EHandle,
	// field offset: 0x32f4
	m_hLastWeapon: EHandle,
	// field offset: 0x32f8
	m_hViewModel: EHandle,
	// field offset: 0x3324
	m_fOnTarget: Boolean,
	// field offset: 0x342c
	m_nTickBase: Integer,
	// field offset: 0x35a4
	m_vecPreviouslyPredictedOrigin: Vector,
}
class C_ColorCorrectionVolume extends C_BaseToggle {
	// field offset: 0xa2c
	m_Weight: Float,
}
class C_PlayerResource extends C_BaseEntity {
	// field offset: 0x9e0
	m_szName: String,
	// field offset: 0xae4
	m_bConnected: Boolean,
	// field offset: 0xb28
	m_iPing: Integer,
	// field offset: 0xc2c
	m_iKills: Integer,
	// field offset: 0xd30
	m_iAssists: Integer,
	// field offset: 0xe34
	m_iDeaths: Integer,
	// field offset: 0xf38
	m_iTeam: Integer,
	// field offset: 0x103c
	m_iPendingTeam: Integer,
	// field offset: 0x1140
	m_bAlive: Boolean,
	// field offset: 0x1184
	m_iHealth: Integer,
	// field offset: 0x1288
	m_iCoachingTeam: Integer,
}
class C_Team extends C_BaseEntity {
	// field offset: 0x9ec
	m_szTeamname: Char,
	// field offset: 0xa0c
	m_szClanTeamname: Char,
	// field offset: 0xa2c
	m_szTeamFlagImage: Char,
	// field offset: 0xa34
	m_szTeamLogoImage: Char,
	// field offset: 0xa3c
	m_szTeamMatchStat: Char,
	// field offset: 0xb40
	m_scoreTotal: Integer,
	// field offset: 0xb44
	m_scoreFirstHalf: Integer,
	// field offset: 0xb48
	m_scoreSecondHalf: Integer,
	// field offset: 0xb4c
	m_scoreOvertime: Integer,
	// field offset: 0xb58
	m_iClanID: Integer,
	// field offset: 0xb5c
	m_iDeaths: Integer,
	// field offset: 0xb60
	m_iPing: Integer,
	// field offset: 0xb64
	m_iPacketloss: Integer,
	// field offset: 0xb68
	m_iTeamNum: Integer,
	// field offset: 0xb6c
	m_bSurrendered: Integer,
}
class CCollisionProperty {
	// field offset: 0x8
	m_vecMins: Vector,
	// field offset: 0x14
	m_vecMaxs: Vector,
	// field offset: 0x20
	m_usSolidFlags: Short,
	// field offset: 0x22
	m_nSolidType: Char,
	// field offset: 0x23
	m_triggerBloat: Char,
}
class CSprite extends C_BaseEntity {
	// field offset: 0x9e8
	m_hAttachedToEntity: EHandle,
	// field offset: 0x9ec
	m_nAttachment: Integer,
	// field offset: 0x9f0
	m_flSpriteFramerate: Float,
	// field offset: 0x9f4
	m_flFrame: Float,
	// field offset: 0x9f8
	m_flDieTime: Float,
	// field offset: 0x9fc
	m_nBrightness: Integer,
	// field offset: 0xa00
	m_flBrightnessTime: Float,
	// field offset: 0xa04
	m_flSpriteScale: Float,
	// field offset: 0xa08
	m_flScaleTime: Float,
	// field offset: 0xa18
	m_flLastTime: Float,
	// field offset: 0xa1c
	m_flMaxFrame: Float,
}
class IntervalTimer {
	// field offset: 0x4
	m_timestamp: Float,
}
class CountdownTimer {
	// field offset: 0x4
	m_duration: Float,
	// field offset: 0x8
	m_timestamp: Float,
}
class C_CSPlayer extends C_BasePlayer {
	// field offset: 0xa14
	m_flCycle: Float,
	// field offset: 0x390a
	m_bIsScoped: Boolean,
	// field offset: 0x390b
	m_bIsWalking: Boolean,
	// field offset: 0x390c
	m_bResumeZoom: Boolean,
	// field offset: 0x3a7c
	m_nIsAutoMounting: Integer,
	// field offset: 0xa368
	m_flStamina: Float,
	// field offset: 0xa36c
	m_iDirection: Integer,
	// field offset: 0xa370
	m_iShotsFired: Integer,
	// field offset: 0xa374
	m_nNumFastDucks: Integer,
	// field offset: 0xa378
	m_bDuckOverride: Boolean,
}
class CBaseCSGrenade extends CWeaponCSBase {
	// field offset: 0x3390
	m_bRedraw: Boolean,
	// field offset: 0x3392
	m_bPinPulled: Boolean,
	// field offset: 0x339c
	m_flThrowStrength: Float,
}
class CWeaponBaseItem extends CWeaponCSBase {
	// field offset: 0x339c
	m_bRedraw: Boolean,
}
class CC4 extends CWeaponCSBase {
	// field offset: 0x33c0
	m_bStartedArming: Integer,
	// field offset: 0x33c4
	m_fArmedTime: Float,
	// field offset: 0x33c8
	m_bBombPlacedAnimation: Integer,
	// field offset: 0x33c9
	m_bShowC4LED: Integer,
	// field offset: 0x33ca
	m_bIsPlantingViaUse: Integer,
}
class CWeaponCSBase extends CBaseCombatWeapon {
	// field offset: 0x3218
	m_flNextPrimaryAttack: Float,
	// field offset: 0x321c
	m_flNextSecondaryAttack: Float,
	// field offset: 0x3254
	m_flTimeWeaponIdle: Float,
	// field offset: 0x32ec
	m_weaponMode: Integer,
	// field offset: 0x3304
	m_fAccuracyPenalty: Float,
	// field offset: 0x3310
	m_iRecoilIndex: Integer,
	// field offset: 0x3314
	m_flRecoilIndex: Float,
	// field offset: 0x331c
	m_flPostponeFireReadyTime: Float,
	// field offset: 0x3320
	m_bReloadVisuallyComplete: Boolean,
	// field offset: 0x3360
	m_fLastShotTime: Float,
	// field offset: 0x337c
	m_iIronSightMode: Integer,
}
class CWeaponCSBaseGun extends CWeaponCSBase {
	// field offset: 0x3390
	m_zoomLevel: Integer,
	// field offset: 0x3394
	m_iBurstShotsRemaining: Integer,
	// field offset: 0x3398
	m_fNextBurstShot: Float,
}
class CWeaponM3 extends CWeaponCSBase {
	// field offset: 0x3394
	m_reloadState: Integer,
}
class CMelee extends CWeaponCSBase {
	// field offset: 0x3390
	m_flThrowAt: Float,
}
class CWeaponNOVA extends CWeaponCSBase {
	// field offset: 0x3394
	m_reloadState: Integer,
}
class CWeaponSawedoff extends CWeaponCSBase {
	// field offset: 0x3394
	m_reloadState: Integer,
}
class CWeaponTaser extends CWeaponCSBaseGun {
	// field offset: 0x33b0
	m_fFireTime: Float,
}
class CWeaponXM1014 extends CWeaponCSBase {
	// field offset: 0x3394
	m_reloadState: Integer,
}
class CDispSubNeighbor {
	// field offset: 0x0
	m_iNeighbor: Short,
	// field offset: 0x2
	m_NeighborOrientation: Char,
	// field offset: 0x3
	m_Span: Char,
	// field offset: 0x4
	m_NeighborSpan: Char,
}
class DT_LocalActiveWeaponData {
	// field offset: 0x3218
	m_flNextPrimaryAttack: Float,
	// field offset: 0x321c
	m_flNextSecondaryAttack: Float,
	// field offset: 0xfc
	m_nNextThinkTick: Int,
	// field offset: 0x3254
	m_flTimeWeaponIdle: Float,
}
class DT_AI_BaseNPC {
	// field offset: 0x0
	baseclass: DataTable,
	// field offset: 0x25f
	m_lifeState: Int,
	// field offset: 0x2f24
	m_bPerformAvoidance: Int,
	// field offset: 0x2f25
	m_bIsMoving: Int,
	// field offset: 0x2f26
	m_bFadeCorpse: Int,
	// field offset: 0x2f14
	m_iDeathPose: Int,
	// field offset: 0x2f18
	m_iDeathFrame: Int,
	// field offset: 0x2f1c
	m_iSpeedModRadius: Int,
	// field offset: 0x2f20
	m_iSpeedModSpeed: Int,
	// field offset: 0x2f27
	m_bSpeedModActive: Int,
	// field offset: 0x2f28
	m_bImportanRagdoll: Int,
	// field offset: 0x2f10
	m_flTimePingEffect: Float,
}
class DT_LocalPlayerExclusive {
	// field offset: 0x2fbc
	m_Local: DataTable,
	// field offset: 0x108
	m_vecViewOffset[0]: Float,
	// field offset: 0x10c
	m_vecViewOffset[1]: Float,
	// field offset: 0x110
	m_vecViewOffset[2]: Float,
	// field offset: 0x144
	m_flFriction: Float,
	// field offset: 0x3324
	m_fOnTarget: Int,
	// field offset: 0x342c
	m_nTickBase: Int,
	// field offset: 0xfc
	m_nNextThinkTick: Int,
	// field offset: 0x32f4
	m_hLastWeapon: Int,
	// field offset: 0x114
	m_vecVelocity[0]: Float,
	// field offset: 0x118
	m_vecVelocity[1]: Float,
	// field offset: 0x11c
	m_vecVelocity[2]: Float,
	// field offset: 0x120
	m_vecBaseVelocity: Vector,
	// field offset: 0x3340
	m_hConstraintEntity: Int,
	// field offset: 0x3344
	m_vecConstraintCenter: Vector,
	// field offset: 0x3350
	m_flConstraintRadius: Float,
	// field offset: 0x3354
	m_flConstraintWidth: Float,
	// field offset: 0x3358
	m_flConstraintSpeedFactor: Float,
	// field offset: 0x335c
	m_bConstraintPastRadius: Int,
	// field offset: 0x33c0
	m_flDeathTime: Float,
	// field offset: 0x33c4
	m_flNextDecalTime: Float,
	// field offset: 0x33c8
	m_fForceTeam: Float,
	// field offset: 0x3590
	m_flLaggedMovementValue: Float,
	// field offset: 0x31cc
	m_hTonemapController: Int,
}
class DT_BaseToggle {
	// field offset: 0x0
	baseclass: DataTable,
	// field offset: 0x9ec
	m_vecFinalDest: Vector,
	// field offset: 0x9f8
	m_movementType: Int,
	// field offset: 0x9fc
	m_flMoveTargetTime: Float,
}
class DT_ColorCorrectionVolume {
	// field offset: 0x0
	baseclass: DataTable,
	// field offset: 0xa20
	m_bEnabled: Int,
	// field offset: 0xa24
	m_MaxWeight: Float,
	// field offset: 0xa28
	m_FadeDuration: Float,
	// field offset: 0xa2c
	m_Weight: Float,
	// field offset: 0xa30
	m_lookupFilename: String,
}
class DT_DynamicLight {
	// field offset: 0x0
	baseclass: DataTable,
	// field offset: 0x9d8
	m_Flags: Int,
	// field offset: 0x9d9
	m_LightStyle: Int,
	// field offset: 0x9dc
	m_Radius: Float,
	// field offset: 0x9e0
	m_Exponent: Int,
	// field offset: 0x9e4
	m_InnerAngle: Float,
	// field offset: 0x9e8
	m_OuterAngle: Float,
	// field offset: 0x9ec
	m_SpotRadius: Float,
}
class DT_EntityParticleTrail {
	// field offset: 0x0
	baseclass: DataTable,
	// field offset: 0xac0
	m_iMaterialName: Int,
	// field offset: 0xac4
	m_Info: DataTable,
	// field offset: 0xad8
	m_hConstraintEntity: Int,
}
class DT_CascadeLight {
	// field offset: 0x0
	baseclass: DataTable,
	// field offset: 0x9d8
	m_shadowDirection: Vector,
	// field offset: 0x9e4
	m_envLightShadowDirection: Vector,
	// field offset: 0x9f0
	m_bEnabled: Int,
	// field offset: 0x9f1
	m_bUseLightEnvAngles: Int,
	// field offset: 0x9f2
	m_LightColor: Int,
	// field offset: 0x9f8
	m_LightColorScale: Int,
	// field offset: 0x9fc
	m_flMaxShadowDist: Float,
}
class DT_EnvDOFController {
	// field offset: 0x0
	baseclass: DataTable,
	// field offset: 0x9d8
	m_bDOFEnabled: Int,
	// field offset: 0x9dc
	m_flNearBlurDepth: Float,
	// field offset: 0x9e0
	m_flNearFocusDepth: Float,
	// field offset: 0x9e4
	m_flFarFocusDepth: Float,
	// field offset: 0x9e8
	m_flFarBlurDepth: Float,
	// field offset: 0x9ec
	m_flNearBlurRadius: Float,
	// field offset: 0x9f0
	m_flFarBlurRadius: Float,
}
class DT_EnvProjectedTexture {
	// field offset: 0x0
	baseclass: DataTable,
	// field offset: 0x9dc
	m_hTargetEntity: Int,
	// field offset: 0x9e0
	m_bState: Int,
	// field offset: 0x9e1
	m_bAlwaysUpdate: Int,
	// field offset: 0x9e4
	m_flLightFOV: Float,
	// field offset: 0x9e8
	m_bEnableShadows: Int,
	// field offset: 0x9e9
	m_bSimpleProjection: Int,
	// field offset: 0x9ea
	m_bLightOnlyTarget: Int,
	// field offset: 0x9eb
	m_bLightWorld: Int,
	// field offset: 0x9ec
	m_bCameraSpace: Int,
	// field offset: 0x9f0
	m_flBrightnessScale: Float,
	// field offset: 0x9f4
	m_LightColor: Int,
	// field offset: 0xa08
	m_flColorTransitionTime: Float,
	// field offset: 0xa0c
	m_flAmbient: Float,
	// field offset: 0xa18
	m_SpotlightTextureName: String,
	// field offset: 0xb24
	m_nSpotlightTextureFrame: Int,
	// field offset: 0xa10
	m_flNearZ: Float,
	// field offset: 0xa14
	m_flFarZ: Float,
	// field offset: 0xb28
	m_nShadowQuality: Int,
	// field offset: 0xb38
	m_flProjectionSize: Float,
	// field offset: 0xb3c
	m_flRotation: Float,
	// field offset: 0xb2c
	m_iStyle: Int,
}
class DT_CFish {
	// field offset: 0x29e0
	m_poolOrigin: Vector,
	// field offset: 0x29c8
	m_x: Float,
	// field offset: 0x29cc
	m_y: Float,
	// field offset: 0x29d0
	m_z: Float,
	// field offset: 0x29d8
	m_angle: Float,
	// field offset: 0x258
	m_nModelIndex: Int,
	// field offset: 0x25f
	m_lifeState: Int,
	// field offset: 0x29ec
	m_waterLevel: Float,
}
class DT_TEDust {
	// field offset: 0x0
	baseclass: DataTable,
	// field offset: 0x1c
	m_flSize: Float,
	// field offset: 0x20
	m_flSpeed: Float,
	// field offset: 0x24
	m_vecDirection: Vector,
}
class DT_LightGlow {
	// field offset: 0x70
	m_clrRender: Int,
	// field offset: 0x9d8
	m_nHorizontalSize: Int,
	// field offset: 0x9dc
	m_nVerticalSize: Int,
	// field offset: 0x9e0
	m_nMinDist: Int,
	// field offset: 0x9e4
	m_nMaxDist: Int,
	// field offset: 0x9e8
	m_nOuterMaxDist: Int,
	// field offset: 0x9ec
	m_spawnflags: Int,
	// field offset: 0x138
	m_vecOrigin: Vector,
	// field offset: 0x12c
	m_angRotation: Vector,
	// field offset: 0x148
	moveparent: Int,
	// field offset: 0xac4
	m_flGlowProxySize: Float,
	// field offset: 0x0
	HDRColorScale: Float,
}
class DT_MaterialModifyControl {
	// field offset: 0x0
	baseclass: DataTable,
	// field offset: 0x9d8
	m_szMaterialName: String,
	// field offset: 0xad7
	m_szMaterialVar: String,
	// field offset: 0xbd6
	m_szMaterialVarValue: String,
	// field offset: 0xce0
	m_iFrameStart: Int,
	// field offset: 0xce4
	m_iFrameEnd: Int,
	// field offset: 0xce8
	m_bWrap: Int,
	// field offset: 0xcec
	m_flFramerate: Float,
	// field offset: 0xcf0
	m_bNewAnimCommandsSemaphore: Int,
	// field offset: 0xcf4
	m_flFloatLerpStartValue: Float,
	// field offset: 0xcf8
	m_flFloatLerpEndValue: Float,
	// field offset: 0xcfc
	m_flFloatLerpTransitionTime: Float,
	// field offset: 0xd00
	m_bFloatLerpWrap: Int,
	// field offset: 0xd08
	m_nModifyMode: Int,
}
class DT_MovieDisplay {
	// field offset: 0x0
	baseclass: DataTable,
	// field offset: 0x9d8
	m_bEnabled: Int,
	// field offset: 0x9d9
	m_bLooping: Int,
	// field offset: 0x9da
	m_szMovieFilename: String,
	// field offset: 0xa5a
	m_szGroupName: String,
	// field offset: 0xada
	m_bStretchToFill: Int,
	// field offset: 0xadb
	m_bForcedSlave: Int,
	// field offset: 0xadc
	m_bUseCustomUVs: Int,
	// field offset: 0xae0
	m_flUMin: Float,
	// field offset: 0xae4
	m_flUMax: Float,
	// field offset: 0xae8
	m_flVMin: Float,
	// field offset: 0xaec
	m_flVMax: Float,
}
class DT_Plasma {
	// field offset: 0x0
	baseclass: DataTable,
	// field offset: 0x9d8
	m_flStartScale: Float,
	// field offset: 0x9dc
	m_flScale: Float,
	// field offset: 0x9e0
	m_flScaleTime: Float,
	// field offset: 0x9e4
	m_nFlags: Int,
	// field offset: 0x9e8
	m_nPlasmaModelIndex: Int,
	// field offset: 0x9ec
	m_nPlasmaModelIndex2: Int,
	// field offset: 0x9f0
	m_nGlowModelIndex: Int,
}
class DT_Prop_Hallucination {
	// field offset: 0x0
	baseclass: DataTable,
	// field offset: 0x2999
	m_bEnabled: Int,
	// field offset: 0x299c
	m_fVisibleTime: Float,
	// field offset: 0x29a0
	m_fRechargeTime: Float,
}
class DT_RopeKeyframe {
	// field offset: 0xccc
	m_nChangeCount: Int,
	// field offset: 0xa14
	m_iRopeMaterialModelIndex: Int,
	// field offset: 0xcac
	m_hStartPoint: Int,
	// field offset: 0xcb0
	m_hEndPoint: Int,
	// field offset: 0xcb4
	m_iStartAttachment: Int,
	// field offset: 0xcb6
	m_iEndAttachment: Int,
	// field offset: 0xcc8
	m_fLockedPoints: Int,
	// field offset: 0xcc0
	m_Slack: Int,
	// field offset: 0xcbc
	m_RopeLength: Int,
	// field offset: 0xa10
	m_RopeFlags: Int,
	// field offset: 0xcc4
	m_TextureScale: Float,
	// field offset: 0xca8
	m_nSegments: Int,
	// field offset: 0xd50
	m_bConstrainBetweenEndpoints: Int,
	// field offset: 0xcb8
	m_Subdiv: Int,
	// field offset: 0xcd0
	m_Width: Float,
	// field offset: 0xa0c
	m_flScrollSpeed: Float,
	// field offset: 0x138
	m_vecOrigin: Vector,
	// field offset: 0x148
	moveparent: Int,
	// field offset: 0x2ec
	m_iParentAttachment: Int,
	// field offset: 0xa18
	m_iDefaultRopeMaterialModelIndex: Int,
	// field offset: 0x988
	m_nMinCPULevel: Int,
	// field offset: 0x989
	m_nMaxCPULevel: Int,
	// field offset: 0x98a
	m_nMinGPULevel: Int,
	// field offset: 0x98b
	m_nMaxGPULevel: Int,
}
class DT_PropVehicleChoreoGeneric {
	// field offset: 0x0
	baseclass: DataTable,
	// field offset: 0x29f4
	m_hPlayer: Int,
	// field offset: 0x29fc
	m_bEnterAnimOn: Int,
	// field offset: 0x29fd
	m_bExitAnimOn: Int,
	// field offset: 0x2a0c
	m_bForceEyesToAttachment: Int,
	// field offset: 0x2a00
	m_vecEyeExitEndpoint: Vector,
	// field offset: 0x2a90
	m_vehicleView.bClampEyeAngles: Int,
	// field offset: 0x2a94
	m_vehicleView.flPitchCurveZero: Float,
	// field offset: 0x2a98
	m_vehicleView.flPitchCurveLinear: Float,
	// field offset: 0x2a9c
	m_vehicleView.flRollCurveZero: Float,
	// field offset: 0x2aa0
	m_vehicleView.flRollCurveLinear: Float,
	// field offset: 0x2aa4
	m_vehicleView.flFOV: Float,
	// field offset: 0x2aa8
	m_vehicleView.flYawMin: Float,
	// field offset: 0x2aac
	m_vehicleView.flYawMax: Float,
	// field offset: 0x2ab0
	m_vehicleView.flPitchMin: Float,
	// field offset: 0x2ab4
	m_vehicleView.flPitchMax: Float,
}
class DT_VGuiScreen {
	// field offset: 0x0
	baseclass: DataTable,
	// field offset: 0x9e0
	m_flWidth: Float,
	// field offset: 0x9e4
	m_flHeight: Float,
	// field offset: 0xa0c
	m_fScreenFlags: Int,
	// field offset: 0x9e8
	m_nPanelName: Int,
	// field offset: 0xa04
	m_nAttachmentIndex: Int,
	// field offset: 0xa08
	m_nOverlayMaterial: Int,
	// field offset: 0xa68
	m_hPlayerOwner: Int,
}
class DT_CollisionProperty {
	// field offset: 0x8
	m_vecMins: Vector,
	// field offset: 0x14
	m_vecMaxs: Vector,
	// field offset: 0x22
	m_nSolidType: Int,
	// field offset: 0x20
	m_usSolidFlags: Int,
	// field offset: 0x2a
	m_nSurroundType: Int,
	// field offset: 0x23
	m_triggerBloat: Int,
	// field offset: 0x2c
	m_vecSpecifiedSurroundingMins: Vector,
	// field offset: 0x38
	m_vecSpecifiedSurroundingMaxs: Vector,
}
class DT_DangerZone {
	// field offset: 0x0
	baseclass: DataTable,
	// field offset: 0x9d8
	m_vecDangerZoneOriginStartedAt: Vector,
	// field offset: 0x9e4
	m_flBombLaunchTime: Float,
	// field offset: 0x9e8
	m_flExtraRadius: Float,
	// field offset: 0x9ec
	m_flExtraRadiusStartTime: Float,
	// field offset: 0x9f0
	m_flExtraRadiusTotalLerpTime: Float,
	// field offset: 0x9f4
	m_nDropOrder: Int,
	// field offset: 0x9f8
	m_iWave: Int,
}
class DT_GameRulesProxy {
}
class DT_ParticleProperty {
}
class DT_Ragdoll {
	// field offset: 0x0
	baseclass: DataTable,
	// field offset: 0x2aa0
	m_ragAngles[0]: Vector,
	// field offset: 0x0
	m_ragAngles: Array,
	// field offset: 0x2980
	m_ragPos[0]: Vector,
	// field offset: 0x0
	m_ragPos: Array,
	// field offset: 0x2c7c
	m_hUnragdoll: Int,
	// field offset: 0x2c80
	m_flBlendWeight: Float,
	// field offset: 0x2c88
	m_nOverlaySequence: Int,
}
class DT_Sprite {
	// field offset: 0x0
	baseclass: DataTable,
	// field offset: 0x9e8
	m_hAttachedToEntity: Int,
	// field offset: 0x9ec
	m_nAttachment: Int,
	// field offset: 0xa08
	m_flScaleTime: Float,
	// field offset: 0xa04
	m_flSpriteScale: Float,
	// field offset: 0x9f0
	m_flSpriteFramerate: Float,
	// field offset: 0xa10
	m_flGlowProxySize: Float,
	// field offset: 0xa14
	m_flHDRColorScale: Float,
	// field offset: 0x9f4
	m_flFrame: Float,
	// field offset: 0xa00
	m_flBrightnessTime: Float,
	// field offset: 0x9fc
	m_nBrightness: Int,
	// field offset: 0xa0c
	m_bWorldSpaceScale: Int,
}
class DT_ParadropChopper {
	// field offset: 0x0
	baseclass: DataTable,
	// field offset: 0x138
	m_vecOrigin: VectorXY,
	// field offset: 0x140
	m_vecOrigin[2]: Float,
	// field offset: 0x2990
	m_hCallingPlayer: Int,
}
class DT_EnvGasCanister {
	// field offset: 0x0
	baseclass: DataTable,
	// field offset: 0x29f4
	m_flFlightSpeed: Float,
	// field offset: 0x29f8
	m_flLaunchTime: Float,
	// field offset: 0x2a0c
	m_vecParabolaDirection: Vector,
	// field offset: 0x29f0
	m_flFlightTime: Float,
	// field offset: 0x2a18
	m_flWorldEnterTime: Float,
	// field offset: 0x29fc
	m_flInitialZSpeed: Float,
	// field offset: 0x2a00
	m_flZAcceleration: Float,
	// field offset: 0x2a04
	m_flHorizSpeed: Float,
	// field offset: 0x2a08
	m_bLaunchedFromWithinWorld: Int,
	// field offset: 0x29b4
	m_vecImpactPosition: Vector,
	// field offset: 0x29c0
	m_vecStartPosition: Vector,
	// field offset: 0x29cc
	m_vecEnterWorldPosition: Vector,
	// field offset: 0x29d8
	m_vecDirection: Vector,
	// field offset: 0x29e4
	m_vecStartAngles: Vector,
	// field offset: 0x2a1c
	m_vecSkyboxOrigin: Vector,
	// field offset: 0x2a28
	m_flSkyboxScale: Float,
	// field offset: 0x2a2c
	m_bInSkybox: Int,
	// field offset: 0x2a2d
	m_bDoImpactEffects: Int,
	// field offset: 0x2980
	m_bLanded: Int,
	// field offset: 0x29a8
	m_hSkyboxCopy: Int,
	// field offset: 0x2a30
	m_nMyZoneIndex: Int,
	// field offset: 0x138
	m_vecOrigin: VectorXY,
	// field offset: 0x140
	m_vecOrigin[2]: Float,
}
class DT_AttributeContainer {
	// field offset: 0x1c
	m_hOuter: Int,
	// field offset: 0x24
	m_ProviderType: Int,
	// field offset: 0x18
	m_iReapplyProvisionParity: Int,
	// field offset: 0x40
	m_Item: DataTable,
}
class DT_AttributeContainerPlayer {
	// field offset: 0x1c
	m_hOuter: Int,
	// field offset: 0x24
	m_ProviderType: Int,
	// field offset: 0x18
	m_iReapplyProvisionParity: Int,
	// field offset: 0x40
	m_hPlayer: Int,
}
class DT_EconEntity {
	// field offset: 0x0
	baseclass: DataTable,
	// field offset: 0x2d80
	m_AttributeManager: DataTable,
	// field offset: 0x31b0
	m_OriginalOwnerXuidLow: Int,
	// field offset: 0x31b4
	m_OriginalOwnerXuidHigh: Int,
	// field offset: 0x31b8
	m_nFallbackPaintKit: Int,
	// field offset: 0x31bc
	m_nFallbackSeed: Int,
	// field offset: 0x31c0
	m_flFallbackWear: Float,
	// field offset: 0x31c4
	m_nFallbackStatTrak: Int,
}
class DT_BaseAttributableItem {
	// field offset: 0x0
	baseclass: DataTable,
	// field offset: 0x2d80
	m_AttributeManager: DataTable,
	// field offset: 0x31b0
	m_OriginalOwnerXuidLow: Int,
	// field offset: 0x31b4
	m_OriginalOwnerXuidHigh: Int,
	// field offset: 0x31b8
	m_nFallbackPaintKit: Int,
	// field offset: 0x31bc
	m_nFallbackSeed: Int,
	// field offset: 0x31c0
	m_flFallbackWear: Float,
	// field offset: 0x31c4
	m_nFallbackStatTrak: Int,
}
class DT_BaseTempEntity {
}
class DT_EnvWindShared {
	// field offset: 0xc
	m_iMinWind: Int,
	// field offset: 0x10
	m_iMaxWind: Int,
	// field offset: 0x18
	m_iMinGust: Int,
	// field offset: 0x1c
	m_iMaxGust: Int,
	// field offset: 0x20
	m_flMinGustDelay: Float,
	// field offset: 0x24
	m_flMaxGustDelay: Float,
	// field offset: 0x2c
	m_iGustDirChange: Int,
	// field offset: 0x8
	m_iWindSeed: Int,
	// field offset: 0x6c
	m_iInitialWindDir: Int,
	// field offset: 0x70
	m_flInitialWindSpeed: Float,
	// field offset: 0x4
	m_flStartTime: Float,
	// field offset: 0x28
	m_flGustDuration: Float,
}
class DT_SmokeTrail {
	// field offset: 0x0
	baseclass: DataTable,
	// field offset: 0xac4
	m_SpawnRate: Float,
	// field offset: 0xac8
	m_StartColor: Vector,
	// field offset: 0xad4
	m_EndColor: Vector,
	// field offset: 0xae4
	m_ParticleLifetime: Float,
	// field offset: 0xae8
	m_StopEmitTime: Float,
	// field offset: 0xaec
	m_MinSpeed: Float,
	// field offset: 0xaf0
	m_MaxSpeed: Float,
	// field offset: 0xaf4
	m_MinDirectedSpeed: Float,
	// field offset: 0xaf8
	m_MaxDirectedSpeed: Float,
	// field offset: 0xafc
	m_StartSize: Float,
	// field offset: 0xb00
	m_EndSize: Float,
	// field offset: 0xb04
	m_SpawnRadius: Float,
	// field offset: 0xb14
	m_bEmit: Int,
	// field offset: 0xb18
	m_nAttachment: Int,
	// field offset: 0xae0
	m_Opacity: Float,
}
class DT_RocketTrail {
	// field offset: 0x0
	baseclass: DataTable,
	// field offset: 0xac4
	m_SpawnRate: Float,
	// field offset: 0xac8
	m_StartColor: Vector,
	// field offset: 0xad4
	m_EndColor: Vector,
	// field offset: 0xae4
	m_ParticleLifetime: Float,
	// field offset: 0xae8
	m_StopEmitTime: Float,
	// field offset: 0xaec
	m_MinSpeed: Float,
	// field offset: 0xaf0
	m_MaxSpeed: Float,
	// field offset: 0xaf4
	m_StartSize: Float,
	// field offset: 0xaf8
	m_EndSize: Float,
	// field offset: 0xafc
	m_SpawnRadius: Float,
	// field offset: 0xb0c
	m_bEmit: Int,
	// field offset: 0xb10
	m_nAttachment: Int,
	// field offset: 0xae0
	m_Opacity: Float,
	// field offset: 0xb0d
	m_bDamaged: Int,
	// field offset: 0xb20
	m_flFlareScale: Float,
}
class DT_SporeExplosion {
	// field offset: 0x0
	baseclass: DataTable,
	// field offset: 0xac4
	m_flSpawnRate: Float,
	// field offset: 0xac8
	m_flParticleLifetime: Float,
	// field offset: 0xacc
	m_flStartSize: Float,
	// field offset: 0xad0
	m_flEndSize: Float,
	// field offset: 0xad4
	m_flSpawnRadius: Float,
	// field offset: 0xadc
	m_bEmit: Int,
	// field offset: 0xadd
	m_bDontRemove: Int,
}
class DT_SporeTrail {
	// field offset: 0x0
	baseclass: DataTable,
	// field offset: 0xacc
	m_flSpawnRate: Float,
	// field offset: 0xac0
	m_vecEndColor: Vector,
	// field offset: 0xad0
	m_flParticleLifetime: Float,
	// field offset: 0xad4
	m_flStartSize: Float,
	// field offset: 0xad8
	m_flEndSize: Float,
	// field offset: 0xadc
	m_flSpawnRadius: Float,
	// field offset: 0xaec
	m_bEmit: Int,
}
class DT_SteamJet {
	// field offset: 0x0
	baseclass: DataTable,
	// field offset: 0xac4
	m_SpreadSpeed: Float,
	// field offset: 0xac8
	m_Speed: Float,
	// field offset: 0xacc
	m_StartSize: Float,
	// field offset: 0xad0
	m_EndSize: Float,
	// field offset: 0xad4
	m_Rate: Float,
	// field offset: 0xad8
	m_JetLength: Float,
	// field offset: 0xadc
	m_bEmit: Int,
	// field offset: 0xae4
	m_bFaceLeft: Int,
	// field offset: 0xae0
	m_nType: Int,
	// field offset: 0xae8
	m_spawnflags: Int,
	// field offset: 0xaec
	m_flRollSpeed: Float,
}
class DT_TEBeamRingPoint {
	// field offset: 0x0
	baseclass: DataTable,
	// field offset: 0x4c
	m_vecCenter: Vector,
	// field offset: 0x58
	m_flStartRadius: Float,
	// field offset: 0x5c
	m_flEndRadius: Float,
}
class DT_TEBreakModel {
	// field offset: 0x0
	baseclass: DataTable,
	// field offset: 0x10
	m_vecOrigin: Vector,
	// field offset: 0x1c
	m_angRotation[0]: Float,
	// field offset: 0x20
	m_angRotation[1]: Float,
	// field offset: 0x24
	m_angRotation[2]: Float,
	// field offset: 0x28
	m_vecSize: Vector,
	// field offset: 0x34
	m_vecVelocity: Vector,
	// field offset: 0x44
	m_nModelIndex: Int,
	// field offset: 0x40
	m_nRandomization: Int,
	// field offset: 0x48
	m_nCount: Int,
	// field offset: 0x4c
	m_fTime: Float,
	// field offset: 0x50
	m_nFlags: Int,
}
class DT_TEBSPDecal {
	// field offset: 0x0
	baseclass: DataTable,
	// field offset: 0x10
	m_vecOrigin: Vector,
	// field offset: 0x1c
	m_nEntity: Int,
	// field offset: 0x20
	m_nIndex: Int,
}
class DT_TEParticleSystem {
	// field offset: 0x0
	baseclass: DataTable,
	// field offset: 0x10
	m_vecOrigin[0]: Float,
	// field offset: 0x14
	m_vecOrigin[1]: Float,
	// field offset: 0x18
	m_vecOrigin[2]: Float,
}
class DT_TESparks {
	// field offset: 0x0
	baseclass: DataTable,
	// field offset: 0x1c
	m_nMagnitude: Int,
	// field offset: 0x20
	m_nTrailLength: Int,
	// field offset: 0x24
	m_vecDir: Vector,
}
class DT_Beam {
	// field offset: 0x9f4
	m_nBeamType: Int,
	// field offset: 0x9f8
	m_nBeamFlags: Int,
	// field offset: 0x9e8
	m_nNumBeamEnts: Int,
	// field offset: 0x9f0
	m_nHaloIndex: Int,
	// field offset: 0xa58
	m_fHaloScale: Float,
	// field offset: 0xa4c
	m_fWidth: Float,
	// field offset: 0xa50
	m_fEndWidth: Float,
	// field offset: 0xa54
	m_fFadeLength: Float,
	// field offset: 0xa5c
	m_fAmplitude: Float,
	// field offset: 0xa60
	m_fStartFrame: Float,
	// field offset: 0xa64
	m_fSpeed: Float,
	// field offset: 0x9d8
	m_flFrameRate: Float,
	// field offset: 0x9dc
	m_flHDRColorScale: Float,
	// field offset: 0x70
	m_clrRender: Int,
	// field offset: 0x25a
	m_nRenderFX: Int,
	// field offset: 0x25b
	m_nRenderMode: Int,
	// field offset: 0xa68
	m_flFrame: Float,
	// field offset: 0xa6c
	m_nClipStyle: Int,
	// field offset: 0xa70
	m_vecEndPos: Vector,
	// field offset: 0x258
	m_nModelIndex: Int,
	// field offset: 0x138
	m_vecOrigin: Vector,
	// field offset: 0x148
	moveparent: Int,
}
class DT_BaseAnimating {
	// field offset: 0x0
	baseclass: DataTable,
	// field offset: 0x28bc
	m_nSequence: Int,
	// field offset: 0x268c
	m_nForceBone: Int,
	// field offset: 0x2680
	m_vecForce: Vector,
	// field offset: 0xa1c
	m_nSkin: Int,
	// field offset: 0xa20
	m_nBody: Int,
	// field offset: 0x9fc
	m_nHitboxSet: Int,
	// field offset: 0x2748
	m_flModelScale: Float,
	// field offset: 0xa18
	m_flPlaybackRate: Float,
	// field offset: 0x289c
	m_bClientSideAnimation: Int,
	// field offset: 0x26c0
	m_bClientSideFrameReset: Int,
	// field offset: 0x279
	m_bClientSideRagdoll: Int,
	// field offset: 0xa44
	m_nNewSequenceParity: Int,
	// field offset: 0xa48
	m_nResetEventsParity: Int,
	// field offset: 0xa64
	m_nMuzzleFlashParity: Int,
	// field offset: 0x0
	serveranimdata: DataTable,
	// field offset: 0x26f8
	m_flFrozen: Float,
	// field offset: 0x274c
	m_ScaleType: Int,
	// field offset: 0x294a
	m_bSuppressAnimSounds: Int,
	// field offset: 0xa38
	m_nHighlightColorR: Int,
	// field offset: 0xa3c
	m_nHighlightColorG: Int,
	// field offset: 0xa40
	m_nHighlightColorB: Int,
}
class DT_BaseFlex {
	// field offset: 0x0
	baseclass: DataTable,
	// field offset: 0x2bd4
	m_blinktoggle: Int,
	// field offset: 0x29f0
	m_viewtarget: Vector,
}
class DT_EntityFreezing {
	// field offset: 0x0
	baseclass: DataTable,
	// field offset: 0x9d8
	m_vFreezingOrigin: Vector,
	// field offset: 0xaac
	m_flFrozen: Float,
	// field offset: 0xab0
	m_bFinishFreezing: Int,
}
class DT_PlayerResource {
}
class DT_PostProcessController {
	// field offset: 0x0
	baseclass: DataTable,
	// field offset: 0xa04
	m_bMaster: Int,
}
class DT_SceneEntity {
	// field offset: 0x0
	baseclass: DataTable,
	// field offset: 0x9e8
	m_nSceneStringIndex: Int,
	// field offset: 0x9dc
	m_bIsPlayingBack: Int,
	// field offset: 0x9dd
	m_bPaused: Int,
	// field offset: 0x9de
	m_bMultiplayer: Int,
	// field offset: 0x9e4
	m_flForceClientTime: Float,
}
class DT_SlideshowDisplay {
	// field offset: 0x0
	baseclass: DataTable,
	// field offset: 0x9d8
	m_bEnabled: Int,
	// field offset: 0x9d9
	m_szDisplayText: String,
	// field offset: 0xa59
	m_szSlideshowDirectory: String,
	// field offset: 0xb08
	m_fMinSlideTime: Float,
	// field offset: 0xb0c
	m_fMaxSlideTime: Float,
	// field offset: 0xb14
	m_iCycleType: Int,
	// field offset: 0xb18
	m_bNoListRepeats: Int,
}
class DT_VoteController {
	// field offset: 0x0
	baseclass: DataTable,
	// field offset: 0x9e4
	m_iActiveIssueIndex: Int,
	// field offset: 0x9e8
	m_iOnlyTeamToVote: Int,
	// field offset: 0xa04
	m_nPotentialVotes: Int,
	// field offset: 0xa0a
	m_bIsYesNoVote: Int,
}
class DT_CSGameRules {
	// field offset: 0x20
	m_bFreezePeriod: Int,
	// field offset: 0x40
	m_bMatchWaitingForResume: Int,
	// field offset: 0x21
	m_bWarmupPeriod: Int,
	// field offset: 0x24
	m_fWarmupPeriodEnd: Float,
	// field offset: 0x28
	m_fWarmupPeriodStart: Float,
	// field offset: 0x2c
	m_bTerroristTimeOutActive: Int,
	// field offset: 0x2d
	m_bCTTimeOutActive: Int,
	// field offset: 0x30
	m_flTerroristTimeOutRemaining: Float,
	// field offset: 0x34
	m_flCTTimeOutRemaining: Float,
	// field offset: 0x38
	m_nTerroristTimeOuts: Int,
	// field offset: 0x3c
	m_nCTTimeOuts: Int,
	// field offset: 0x44
	m_iRoundTime: Int,
	// field offset: 0x60
	m_gamePhase: Int,
	// field offset: 0x64
	m_totalRoundsPlayed: Int,
	// field offset: 0x68
	m_nOvertimePlaying: Int,
	// field offset: 0x5c
	m_timeUntilNextPhaseStarts: Float,
	// field offset: 0x870
	m_flCMMItemDropRevealStartTime: Float,
	// field offset: 0x874
	m_flCMMItemDropRevealEndTime: Float,
	// field offset: 0x4c
	m_fRoundStartTime: Float,
	// field offset: 0x54
	m_bGameRestart: Int,
	// field offset: 0x50
	m_flRestartRoundTime: Float,
	// field offset: 0x58
	m_flGameStartTime: Float,
	// field offset: 0x6c
	m_iHostagesRemaining: Int,
	// field offset: 0x70
	m_bAnyHostageReached: Int,
	// field offset: 0x71
	m_bMapHasBombTarget: Int,
	// field offset: 0x72
	m_bMapHasRescueZone: Int,
	// field offset: 0x73
	m_bMapHasBuyZone: Int,
	// field offset: 0x74
	m_bIsQueuedMatchmaking: Int,
	// field offset: 0x75
	m_bIsValveDS: Int,
	// field offset: 0x879
	m_bIsQuestEligible: Int,
	// field offset: 0x76
	m_bLogoMap: Int,
	// field offset: 0x77
	m_bPlayAllStepSoundsOnServer: Int,
	// field offset: 0x78
	m_iNumGunGameProgressiveWeaponsCT: Int,
	// field offset: 0x7c
	m_iNumGunGameProgressiveWeaponsT: Int,
	// field offset: 0x80
	m_iSpectatorSlotCount: Int,
	// field offset: 0x99c
	m_bBombDropped: Int,
	// field offset: 0x99d
	m_bBombPlanted: Int,
	// field offset: 0x9a0
	m_iRoundWinStatus: Int,
	// field offset: 0x9a4
	m_eRoundWinReason: Int,
	// field offset: 0x44c
	m_flDMBonusStartTime: Float,
	// field offset: 0x450
	m_flDMBonusTimeLength: Float,
	// field offset: 0x454
	m_unDMBonusWeaponLoadoutSlot: Int,
	// field offset: 0x456
	m_bDMBonusActive: Int,
	// field offset: 0x9a8
	m_bTCantBuy: Int,
	// field offset: 0x9a9
	m_bCTCantBuy: Int,
	// field offset: 0x9ac
	m_flGuardianBuyUntilTime: Float,
	// field offset: 0x444
	m_MatchDevice: Int,
	// field offset: 0x448
	m_bHasMatchStarted: Int,
	// field offset: 0x458
	m_nNextMapInMapgroup: Int,
	// field offset: 0xc70
	m_nEndMatchMapVoteWinner: Int,
	// field offset: 0x878
	m_bIsDroppingItems: Int,
	// field offset: 0xc18
	m_iActiveAssassinationTargetMissionID: Int,
	// field offset: 0x48
	m_fMatchStartTime: Float,
	// field offset: 0x45c
	m_szTournamentEventName: String,
	// field offset: 0x560
	m_szTournamentEventStage: String,
	// field offset: 0x768
	m_szTournamentPredictionsTxt: String,
	// field offset: 0x86c
	m_nTournamentPredictionsPct: Int,
	// field offset: 0x664
	m_szMatchStatTxt: String,
	// field offset: 0x87c
	m_nGuardianModeWaveNumber: Int,
	// field offset: 0x880
	m_nGuardianModeSpecialKillsRemaining: Int,
	// field offset: 0x884
	m_nGuardianModeSpecialWeaponNeeded: Int,
	// field offset: 0x998
	m_nHalloweenMaskListSeed: Int,
	// field offset: 0x890
	m_numGlobalGiftsGiven: Int,
	// field offset: 0x894
	m_numGlobalGifters: Int,
	// field offset: 0x898
	m_numGlobalGiftsPeriodSeconds: Int,
	// field offset: 0x994
	m_numBestOfMaps: Int,
	// field offset: 0xc74
	m_iNumConsecutiveCTLoses: Int,
	// field offset: 0xc78
	m_iNumConsecutiveTerroristLoses: Int,
	// field offset: 0xcf8
	m_SurvivalRules: DataTable,
}
class DT_AttributeList {
}
class WeaponInfo {
	// field offset: 0x14
	primary clip size: Int,
	// field offset: 0x18
	secondary clip size: Int,
	// field offset: 0x1c
	primary default clip size: Int,
	// field offset: 0x20
	secondary default clip size: Int,
	// field offset: 0x24
	primary reserve ammo max: Int,
	// field offset: 0x28
	secondary reserve ammo max: Int,
	// field offset: 0x90
	allow hand flipping: Bool,
	// field offset: 0x91
	model right handed: Bool,
	// field offset: 0x92
	is melee weapon: Bool,
	// field offset: 0x9c
	weapon weight: Int,
	// field offset: 0xc0
	rumble effect: Int,
}
class CSWeaponInfo extends WeaponInfo {
	// field offset: 0xd0
	in game price: Int,
	// field offset: 0xd4
	kill award: Int,
	// field offset: 0xdc
	cycletime: Float,
	// field offset: 0xe0
	cycletime alt: Float,
	// field offset: 0xe4
	time to idle: Float,
	// field offset: 0xe8
	idle interval: Float,
	// field offset: 0xec
	is full auto: Bool,
	// field offset: 0xf0
	damage: Int,
	// field offset: 0xf4
	armor ratio: Float,
	// field offset: 0xf8
	bullets: Int,
	// field offset: 0xfc
	penetration: Float,
	// field offset: 0x100
	flinch velocity modifier large: Float,
	// field offset: 0x104
	flinch velocity modifier small: Float,
	// field offset: 0x108
	range: Float,
	// field offset: 0x10c
	range modifier: Float,
	// field offset: 0x110
	throw velocity: Float,
	// field offset: 0x120
	has silencer: Int,
	// field offset: 0x124
	silencer model: Int,
	// field offset: 0x128
	crosshair min distance: Int,
	// field offset: 0x12c
	crosshair delta distance: Int,
	// field offset: 0x130
	max player speed: Float,
	// field offset: 0x134
	max player speed alt: Float,
	// field offset: 0x138
	attack movespeed factor: Float,
	// field offset: 0x13c
	spread: Float,
	// field offset: 0x140
	spread alt: Float,
	// field offset: 0x144
	inaccuracy crouch: Float,
	// field offset: 0x148
	inaccuracy crouch alt: Float,
	// field offset: 0x14c
	inaccuracy stand: Float,
	// field offset: 0x150
	inaccuracy stand alt: Float,
	// field offset: 0x154
	inaccuracy jump initial: Float,
	// field offset: 0x158
	inaccuracy jump: Float,
	// field offset: 0x15c
	inaccuracy jump alt: Float,
	// field offset: 0x160
	inaccuracy land: Float,
	// field offset: 0x164
	inaccuracy land alt: Float,
	// field offset: 0x168
	inaccuracy ladder: Float,
	// field offset: 0x16c
	inaccuracy ladder alt: Float,
	// field offset: 0x170
	inaccuracy fire: Float,
	// field offset: 0x174
	inaccuracy fire alt: Float,
	// field offset: 0x178
	inaccuracy move: Float,
	// field offset: 0x17c
	inaccuracy move alt: Float,
	// field offset: 0x180
	inaccuracy reload: Float,
	// field offset: 0x184
	recoil seed: Int,
	// field offset: 0x188
	recoil angle: Float,
	// field offset: 0x18c
	recoil angle alt: Float,
	// field offset: 0x190
	recoil angle variance: Float,
	// field offset: 0x194
	recoil angle variance alt: Float,
	// field offset: 0x198
	recoil magnitude: Float,
	// field offset: 0x19c
	recoil magnitude alt: Float,
	// field offset: 0x1a0
	recoil magnitude variance: Float,
	// field offset: 0x1a4
	recoil magnitude variance alt: Float,
	// field offset: 0x1a8
	spread seed: Int,
	// field offset: 0x1ac
	recovery time crouch: Float,
	// field offset: 0x1b0
	recovery time stand: Float,
	// field offset: 0x1b4
	recovery time crouch final: Float,
	// field offset: 0x1b8
	recovery time stand final: Float,
	// field offset: 0x1bc
	recovery transition start bullet: Int,
	// field offset: 0x1c0
	recovery transition end bullet: Int,
	// field offset: 0x1c4
	unzoom after shot: Bool,
	// field offset: 0x1c5
	hide view model zoomed: Bool,
	// field offset: 0x1c8
	zoom levels: Int,
	// field offset: 0x1cc
	zoom fov 1: Int,
	// field offset: 0x1d0
	zoom fov 2: Int,
	// field offset: 0x1d4
	zoom time 0: Float,
	// field offset: 0x1d8
	zoom time 1: Float,
	// field offset: 0x1dc
	zoom time 2: Float,
	// field offset: 0x1e8
	addon scale: Float,
	// field offset: 0x1f4
	tracer frequency: Int,
	// field offset: 0x1f8
	tracer frequency alt: Int,
	// field offset: 0x210
	heat per shot: Float,
	// field offset: 0x21c
	inaccuracy pitch shift: Float,
	// field offset: 0x220
	inaccuracy alt sound threshold: Float,
	// field offset: 0x224
	bot audible range: Float,
	// field offset: 0x230
	wrong team msg: Int,
	// field offset: 0x234
	has burst mode: Bool,
	// field offset: 0x235
	is revolver: Bool,
	// field offset: 0x236
	cannot shoot underwater: Bool,
}
# Server

## Interfaces

```
server.dll!0x00990c0c BotManager001
server.dll!0x00a48b78 GameMovement001
server.dll!0x00a91678 HLTVDirector001
server.dll!0x00987138 IEffects001
server.dll!0x00990c10 PlayerInfoManager001
server.dll!0x00990c08 PlayerInfoManager002
server.dll!0x00990c38 PluginHelpersCheck001
server.dll!0x0098ca80 ServerGameClients004
server.dll!0x0098cb50 ServerGameDLL005
server.dll!0x0098ca18 ServerGameEnts001
server.dll!0x009a26d4 ServerGameTags001
server.dll!0x009a3560 ServerGameTags001
server.dll!0x0098da18 ServerGameTags001
server.dll!0x0098fad0 ServerGameTags001
server.dll!0x00991c48 ServerGameTags001
server.dll!0x00991df4 ServerGameTags001
server.dll!0x00992360 ServerGameTags001
server.dll!0x00992570 ServerGameTags001
server.dll!0x009925b4 ServerGameTags001
server.dll!0x00992e64 ServerGameTags001
server.dll!0x00993728 ServerGameTags001
server.dll!0x00993e2c ServerGameTags001
server.dll!0x00993e4c ServerGameTags001
server.dll!0x00994080 ServerGameTags001
server.dll!0x00984820 ServerGameTags001
server.dll!0x00994740 ServerGameTags001
server.dll!0x0099d638 ServerGameTags001
server.dll!0x009862bc ServerGameTags001
server.dll!0x0099dbd8 ServerGameTags001
server.dll!0x0099eb90 ServerGameTags001
server.dll!0x0099ec10 ServerGameTags001
server.dll!0x009a689c ServerGameTags001
server.dll!0x009a24b4 ServerGameTags001
server.dll!0x009a25b0 ServerGameTags001
server.dll!0x009a260c ServerGameTags001
server.dll!0x009a2678 ServerGameTags001
server.dll!0x009a2694 ServerGameTags001
server.dll!0x009a26a4 ServerGameTags001
server.dll!0x009a26b4 ServerGameTags001
server.dll!0x009a26c4 ServerGameTags001
server.dll!0x0098623c ServerGameTags001
server.dll!0x009a26e4 ServerGameTags001
server.dll!0x009a31f8 ServerGameTags001
server.dll!0x009a3208 ServerGameTags001
server.dll!0x009a3550 ServerGameTags001
server.dll!0x0098c594 ServerGameTags001
server.dll!0x009a3570 ServerGameTags001
server.dll!0x009a3580 ServerGameTags001
server.dll!0x009a3590 ServerGameTags001
server.dll!0x009a35a0 ServerGameTags001
server.dll!0x009a36b8 ServerGameTags001
server.dll!0x009a36c8 ServerGameTags001
server.dll!0x009a36d8 ServerGameTags001
server.dll!0x009a36e8 ServerGameTags001
server.dll!0x009a36f8 ServerGameTags001
server.dll!0x009a3708 ServerGameTags001
server.dll!0x009a3718 ServerGameTags001
server.dll!0x009a3728 ServerGameTags001
server.dll!0x009a3738 ServerGameTags001
server.dll!0x009a3748 ServerGameTags001
server.dll!0x009a3ac8 ServerGameTags001
server.dll!0x009a3ad8 ServerGameTags001
server.dll!0x009a3ae8 ServerGameTags001
server.dll!0x009a3af8 ServerGameTags001
server.dll!0x009a3b08 ServerGameTags001
server.dll!0x009a3b18 ServerGameTags001
server.dll!0x009a3b28 ServerGameTags001
server.dll!0x009a3b38 ServerGameTags001
server.dll!0x009a3b48 ServerGameTags001
server.dll!0x009a3bb0 ServerGameTags001
server.dll!0x009a3bc0 ServerGameTags001
server.dll!0x009a3bd0 ServerGameTags001
server.dll!0x009a3be0 ServerGameTags001
server.dll!0x009a3bf0 ServerGameTags001
server.dll!0x009a3c00 ServerGameTags001
server.dll!0x009a3c10 ServerGameTags001
server.dll!0x009a4370 ServerGameTags001
server.dll!0x009a5478 ServerGameTags001
server.dll!0x009a59fc ServerGameTags001
server.dll!0x00a91ff8 VENGINE_GAMETYPES_VERSION002
server.dll!0x009a8f5c VSERVERTOOLS001
server.dll!0x00a1debc VServerDllSharedAppSystems001
```

## ConVars

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
<summary><code>sv_dz_exploration_payment_amount</code></summary>

Number of cash bundles to award for exploring a new sector

default: `"2"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_dz_hostage_rescue_reward</code></summary>

Number of cash bundles to award for rescuing a hostage

default: `"10"`  
flags: `0x82000`  
</details>
<details>
<summary><code>sv_dz_jointeam_allowed</code></summary>

Whether non-server admins are allowed to use the dz_jointeam command

default: `"0"`  
flags: `0x80000`  
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

### Addresses

```
server.dll!0x9a9040 ConVar BlendBonesMode
server.dll!0x99e060 ConVar CS_WarnFriendlyDamageInterval
server.dll!0x981f18 ConVar achievement_debug
server.dll!0x981f70 ConVar achievement_disable
server.dll!0x9830b0 ConVar ai_LOS_mode
server.dll!0x985e40 ConVar ai_debug_shoot_positions
server.dll!0x983440 ConVar ai_drawbattlelines
server.dll!0x982f90 ConVar ai_report_task_timings_on_limit
server.dll!0x985ef0 ConVar ai_shot_bias_max
server.dll!0x985e98 ConVar ai_shot_bias_min
server.dll!0x982fe8 ConVar ai_think_limit_label
server.dll!0x984478 ConVar ai_vehicle_avoidance
server.dll!0x9967c0 ConVar ammo_338mag_headshot_mult
server.dll!0x996450 ConVar ammo_338mag_impulse
server.dll!0x995dc8 ConVar ammo_338mag_max
server.dll!0x996920 ConVar ammo_357sig_headshot_mult
server.dll!0x9965b0 ConVar ammo_357sig_impulse
server.dll!0x995f28 ConVar ammo_357sig_max
server.dll!0x996030 ConVar ammo_357sig_min_max
server.dll!0x995f80 ConVar ammo_357sig_p250_max
server.dll!0x995fd8 ConVar ammo_357sig_small_max
server.dll!0x9968c8 ConVar ammo_45acp_headshot_mult
server.dll!0x996558 ConVar ammo_45acp_impulse
server.dll!0x995ed0 ConVar ammo_45acp_max
server.dll!0x996660 ConVar ammo_50AE_headshot_mult
server.dll!0x9962f0 ConVar ammo_50AE_impulse
server.dll!0x995c10 ConVar ammo_50AE_max
server.dll!0x996768 ConVar ammo_556mm_box_headshot_mult
server.dll!0x9963f8 ConVar ammo_556mm_box_impulse
server.dll!0x995d70 ConVar ammo_556mm_box_max
server.dll!0x996710 ConVar ammo_556mm_headshot_mult
server.dll!0x9963a0 ConVar ammo_556mm_impulse
server.dll!0x995cc0 ConVar ammo_556mm_max
server.dll!0x995d18 ConVar ammo_556mm_small_max
server.dll!0x996978 ConVar ammo_57mm_headshot_mult
server.dll!0x996608 ConVar ammo_57mm_impulse
server.dll!0x996088 ConVar ammo_57mm_max
server.dll!0x9966b8 ConVar ammo_762mm_headshot_mult
server.dll!0x996348 ConVar ammo_762mm_impulse
server.dll!0x995c68 ConVar ammo_762mm_max
server.dll!0x996818 ConVar ammo_9mm_headshot_mult
server.dll!0x9964a8 ConVar ammo_9mm_impulse
server.dll!0x995e20 ConVar ammo_9mm_max
server.dll!0x996870 ConVar ammo_buckshot_headshot_mult
server.dll!0x996500 ConVar ammo_buckshot_impulse
server.dll!0x995e78 ConVar ammo_buckshot_max
server.dll!0x996298 ConVar ammo_grenade_limit_breachcharge
server.dll!0x9960e0 ConVar ammo_grenade_limit_default
server.dll!0x996138 ConVar ammo_grenade_limit_flashbang
server.dll!0x9961e8 ConVar ammo_grenade_limit_snowballs
server.dll!0x996190 ConVar ammo_grenade_limit_total
server.dll!0x996240 ConVar ammo_item_limit_healthshot
server.dll!0x9a9098 ConVar anim_3wayblend
server.dll!0x9a90f0 ConVar anim_twistbones_enabled
server.dll!0x9a2ac0 ConVar bot_allow_grenades
server.dll!0x9a2a68 ConVar bot_allow_machine_guns
server.dll!0x9a2908 ConVar bot_allow_pistols
server.dll!0x9a2a10 ConVar bot_allow_rifles
server.dll!0x9a2dd8 ConVar bot_allow_rogues
server.dll!0x9a2960 ConVar bot_allow_shotguns
server.dll!0x9a2b18 ConVar bot_allow_snipers
server.dll!0x9a29b8 ConVar bot_allow_sub_machine_guns
server.dll!0x9a2cd0 ConVar bot_auto_follow
server.dll!0x9a2f38 ConVar bot_auto_vacate
server.dll!0x998b60 ConVar bot_autodifficulty_threshold_high
server.dll!0x998b08 ConVar bot_autodifficulty_threshold_low
server.dll!0x9a2d80 ConVar bot_chatter
server.dll!0x9a2f90 ConVar bot_controllable
server.dll!0x9a3b58 ConVar bot_coop_force_throw_grenade_chance
server.dll!0x9a3660 ConVar bot_coop_idle_max_vision_distance
server.dll!0x99e3d0 ConVar bot_crouch
server.dll!0x9a2e88 ConVar bot_debug
server.dll!0x9a2ee0 ConVar bot_debug_target
server.dll!0x9a2e30 ConVar bot_defer_to_human_goals
server.dll!0x9a3040 ConVar bot_defer_to_human_items
server.dll!0x9a2fe8 ConVar bot_difficulty
server.dll!0x9a2c20 ConVar bot_dont_shoot
server.dll!0x9a2c78 ConVar bot_eco_limit
server.dll!0x9a2d28 ConVar bot_flipout
server.dll!0x99e378 ConVar bot_freeze
server.dll!0x9a3608 ConVar bot_ignore_players
server.dll!0x9a3148 ConVar bot_join_after_player
server.dll!0x9a31a0 ConVar bot_join_team
server.dll!0x9a3758 ConVar bot_loadout
server.dll!0x9a35b0 ConVar bot_max_vision_distance_override
server.dll!0x99e320 ConVar bot_mimic
server.dll!0x99e428 ConVar bot_mimic_yaw_offset
server.dll!0x9a2bc8 ConVar bot_profile_db
server.dll!0x9a3098 ConVar bot_quota
server.dll!0x9a30f0 ConVar bot_quota_mode
server.dll!0x9a37b0 ConVar bot_randombuy
server.dll!0x9a3270 ConVar bot_show_battlefront
server.dll!0x9a2800 ConVar bot_show_nav
server.dll!0x9a3218 ConVar bot_show_occupy_time
server.dll!0x9a27a8 ConVar bot_stop
server.dll!0x9a2750 ConVar bot_traceview
server.dll!0x9a2858 ConVar bot_walk
server.dll!0x9a2b70 ConVar bot_zombie
server.dll!0x99c4f8 ConVar cash_player_bomb_defused
server.dll!0x99c4a0 ConVar cash_player_bomb_planted
server.dll!0x99c600 ConVar cash_player_damage_hostage
server.dll!0x99c708 ConVar cash_player_get_killed
server.dll!0x99c5a8 ConVar cash_player_interact_with_hostage
server.dll!0x99c448 ConVar cash_player_killed_enemy_default
server.dll!0x99c3f0 ConVar cash_player_killed_enemy_factor
server.dll!0x99c658 ConVar cash_player_killed_hostage
server.dll!0x99c398 ConVar cash_player_killed_teammate
server.dll!0x99c550 ConVar cash_player_rescued_hostage
server.dll!0x99c6b0 ConVar cash_player_respawn_amount
server.dll!0x99bf20 ConVar cash_team_elimination_bomb_map
server.dll!0x99bec8 ConVar cash_team_elimination_hostage_map_ct
server.dll!0x99be70 ConVar cash_team_elimination_hostage_map_t
server.dll!0x99c290 ConVar cash_team_hostage_alive
server.dll!0x99c340 ConVar cash_team_hostage_interaction
server.dll!0x99c130 ConVar cash_team_loser_bonus
server.dll!0x99c188 ConVar cash_team_loser_bonus_consecutive_rounds
server.dll!0x99c2e8 ConVar cash_team_planted_bomb_but_defused
server.dll!0x99c238 ConVar cash_team_rescued_hostage
server.dll!0x99bf78 ConVar cash_team_survive_guardian_wave
server.dll!0x99be18 ConVar cash_team_terrorist_win_bomb
server.dll!0x99c080 ConVar cash_team_win_by_defusing_bomb
server.dll!0x99c0d8 ConVar cash_team_win_by_hostage_rescue
server.dll!0x99c028 ConVar cash_team_win_by_time_running_out_bomb
server.dll!0x99bfd0 ConVar cash_team_win_by_time_running_out_hostage
server.dll!0x99c1e0 ConVar cash_team_winner_bonus_consecutive_rounds
server.dll!0x991ff0 ConVar cc_showmissing
server.dll!0x983e58 ConVar chet_debug_idle
server.dll!0x9a9148 ConVar choreo_spew_filter
server.dll!0x992fa8 ConVar cl_remove_old_ugc_downloads
server.dll!0x9a8fe8 ConVar cl_simdbones
server.dll!0x9a8f90 ConVar cl_use_simd_bones
server.dll!0x99b718 ConVar contributionscore_assist
server.dll!0x99b928 ConVar contributionscore_bomb_defuse_major
server.dll!0x99b8d0 ConVar contributionscore_bomb_defuse_minor
server.dll!0x99b9d8 ConVar contributionscore_bomb_exploded
server.dll!0x99b980 ConVar contributionscore_bomb_planted
server.dll!0x99bb38 ConVar contributionscore_cash_bundle
server.dll!0x99bb90 ConVar contributionscore_crate_break
server.dll!0x99bae0 ConVar contributionscore_hostage_kill
server.dll!0x99b878 ConVar contributionscore_hostage_rescue_major
server.dll!0x99b820 ConVar contributionscore_hostage_rescue_minor
server.dll!0x99b770 ConVar contributionscore_kill
server.dll!0x99bbe8 ConVar contributionscore_kill_factor
server.dll!0x99b7c8 ConVar contributionscore_objective_kill
server.dll!0x99ba30 ConVar contributionscore_suicide
server.dll!0x99ba88 ConVar contributionscore_team_kill
server.dll!0x99dbe8 ConVar cs_ShowStateTransitions
server.dll!0x99e5e0 ConVar cs_enable_player_physics_box
server.dll!0x99e588 ConVar cs_hostage_near_rescue_music_distance
server.dll!0x9a2150 ConVar custom_bot_difficulty
server.dll!0x9a20c8 ConVar debug_aim_angle
server.dll!0x986e00 ConVar debug_visibility_monitor
server.dll!0x999c98 ConVar dev_reportmoneychanges
server.dll!0x993330 ConVar developer
server.dll!0x9858d0 ConVar ent_messages_draw
server.dll!0x99edd8 ConVar ff_damage_bullet_penetration
server.dll!0x99f210 ConVar ff_damage_reduction_bullets
server.dll!0x99f160 ConVar ff_damage_reduction_grenade
server.dll!0x99f108 ConVar ff_damage_reduction_grenade_self
server.dll!0x99f1b8 ConVar ff_damage_reduction_other
server.dll!0x98b990 ConVar fish_dormant
server.dll!0x98bd28 ConVar func_break_max_pieces
server.dll!0x98b5b0 ConVar fx_new_sparks
server.dll!0x98d258 ConVar g_Language
server.dll!0x990ca0 ConVar g_debug_angularsensor
server.dll!0x98eff8 ConVar g_debug_constraint_sounds
server.dll!0x9913e8 ConVar g_debug_ragdoll_removal
server.dll!0x984db8 ConVar g_debug_trackpather
server.dll!0x993388 ConVar g_debug_vehiclebase
server.dll!0x98ebc0 ConVar g_debug_vehicledriver
server.dll!0x9934c8 ConVar g_debug_vehicleexit
server.dll!0x993470 ConVar g_debug_vehiclesound
server.dll!0x9a09b0 ConVar g_jeepexitspeed
server.dll!0x991440 ConVar g_ragdoll_important_maxcount
server.dll!0x991390 ConVar g_ragdoll_maxcount
server.dll!0x9a2258 ConVar game_mode
server.dll!0x9a2200 ConVar game_online
server.dll!0x9a21a8 ConVar game_public
server.dll!0x9a22b0 ConVar game_type
server.dll!0x99e530 ConVar gg_knife_kill_demotes
server.dll!0x98d700 ConVar global_event_log_enabled
server.dll!0x9a50a0 ConVar healthshot_allow_use_at_full
server.dll!0x9a4ff0 ConVar healthshot_health
server.dll!0x99e6e8 ConVar healthshot_healthboost_damage_multiplier
server.dll!0x9a50f8 ConVar healthshot_healthboost_speed_multiplier
server.dll!0x9a5048 ConVar healthshot_healthboost_time
server.dll!0x985f48 ConVar hl2_episodic
server.dll!0x9a4648 ConVar hostage_debug
server.dll!0x99f318 ConVar hostage_feetyawrate
server.dll!0x9a4430 ConVar hostage_is_silent
server.dll!0x9a3d40 ConVar inferno_child_spawn_interval_multiplier
server.dll!0x9a4210 ConVar inferno_child_spawn_max_depth
server.dll!0x9a4000 ConVar inferno_damage
server.dll!0x9a3fa8 ConVar inferno_debug
server.dll!0x9a3ef8 ConVar inferno_flame_lifetime
server.dll!0x9a3ea0 ConVar inferno_flame_spacing
server.dll!0x9a4318 ConVar inferno_forward_reduction_factor
server.dll!0x9a3f50 ConVar inferno_friendly_fire_duration
server.dll!0x9a3ce8 ConVar inferno_initial_spawn_interval
server.dll!0x9a3d98 ConVar inferno_max_child_spawn_interval
server.dll!0x9a3e48 ConVar inferno_max_flames
server.dll!0x9a4058 ConVar inferno_max_range
server.dll!0x9a3c90 ConVar inferno_per_flame_spawn_duration
server.dll!0x9a4268 ConVar inferno_scorch_decals
server.dll!0x9a3df0 ConVar inferno_spawn_angle
server.dll!0x9a41b8 ConVar inferno_surface_offset
server.dll!0x9a4108 ConVar inferno_velocity_decay_factor
server.dll!0x9a40b0 ConVar inferno_velocity_factor
server.dll!0x9a4160 ConVar inferno_velocity_normal_factor
server.dll!0x98c068 ConVar loopsingleplayermaps
server.dll!0x98e6a0 ConVar mapcycledisabled
server.dll!0x9a23e0 ConVar molotov_throw_detonate_time
server.dll!0x996e48 ConVar mp_afterroundmoney
server.dll!0x98c4e0 ConVar mp_allowNPCs
server.dll!0x98d458 ConVar mp_allowspectators
server.dll!0x997160 ConVar mp_anyone_can_pickup_c4
server.dll!0x98c380 ConVar mp_autocrosshair
server.dll!0x999f58 ConVar mp_autokick
server.dll!0x992b00 ConVar mp_autoteambalance
server.dll!0x995b90 ConVar mp_backup_restore_load_autopause
server.dll!0x9959e8 ConVar mp_backup_round_auto
server.dll!0x995a40 ConVar mp_backup_round_file
server.dll!0x995af0 ConVar mp_backup_round_file_last
server.dll!0x995a98 ConVar mp_backup_round_file_pattern
server.dll!0x992630 ConVar mp_blockstyle
server.dll!0x9928f0 ConVar mp_bonusroundtime
server.dll!0x996c38 ConVar mp_buy_allow_grenades
server.dll!0x996c90 ConVar mp_buy_allow_guns
server.dll!0x98d560 ConVar mp_buy_anywhere
server.dll!0x98d5b8 ConVar mp_buy_during_immunity
server.dll!0x996be0 ConVar mp_buytime
server.dll!0x9971b8 ConVar mp_c4_cannot_be_defused
server.dll!0x999c38 ConVar mp_c4timer
server.dll!0x9926e0 ConVar mp_capdeteriorate_time
server.dll!0x9925d8 ConVar mp_capstyle
server.dll!0x9992f0 ConVar mp_competitive_endofmatch_extra_time
server.dll!0x99ab60 ConVar mp_consecutive_loss_aversion
server.dll!0x99abb8 ConVar mp_consecutive_loss_max
server.dll!0x998a00 ConVar mp_coop_force_join_ct
server.dll!0x9a26f8 ConVar mp_coopmission_bot_difficulty_offset
server.dll!0x998a58 ConVar mp_coopmission_mission_number
server.dll!0x997cf0 ConVar mp_ct_default_grenades
server.dll!0x997be8 ConVar mp_ct_default_melee
server.dll!0x997c98 ConVar mp_ct_default_primary
server.dll!0x997c40 ConVar mp_ct_default_secondary
server.dll!0x99a9a8 ConVar mp_damage_headshot_only
server.dll!0x99a7f0 ConVar mp_damage_scale_ct_body
server.dll!0x99a848 ConVar mp_damage_scale_ct_head
server.dll!0x99a8a0 ConVar mp_damage_scale_t_body
server.dll!0x99a8f8 ConVar mp_damage_scale_t_head
server.dll!0x99aa58 ConVar mp_damage_vampiric_amount
server.dll!0x998950 ConVar mp_death_drop_breachcharge
server.dll!0x9987f0 ConVar mp_death_drop_c4
server.dll!0x9988a0 ConVar mp_death_drop_defuser
server.dll!0x998848 ConVar mp_death_drop_grenade
server.dll!0x998798 ConVar mp_death_drop_gun
server.dll!0x9989a8 ConVar mp_death_drop_healthshot
server.dll!0x9988f8 ConVar mp_death_drop_taser
server.dll!0x99e638 ConVar mp_deathcam_skippable
server.dll!0x998f28 ConVar mp_default_team_winner_no_objective
server.dll!0x998690 ConVar mp_defuser_allocation
server.dll!0x992898 ConVar mp_disable_respawn_times
server.dll!0x9973c8 ConVar mp_disconnect_kills_bots
server.dll!0x997370 ConVar mp_disconnect_kills_players
server.dll!0x998588 ConVar mp_display_kill_assists
server.dll!0x99a690 ConVar mp_dm_bonus_length_max
server.dll!0x99a638 ConVar mp_dm_bonus_length_min
server.dll!0x9982c8 ConVar mp_dm_bonus_percent
server.dll!0x998320 ConVar mp_dm_bonus_respawn
server.dll!0x998530 ConVar mp_dm_bonusweapon_dogtags
server.dll!0x998378 ConVar mp_dm_dogtag_score
server.dll!0x998270 ConVar mp_dm_kill_base_score
server.dll!0x9983d0 ConVar mp_dm_teammode
server.dll!0x998480 ConVar mp_dm_teammode_bonus_score
server.dll!0x9984d8 ConVar mp_dm_teammode_dogtag_score
server.dll!0x998428 ConVar mp_dm_teammode_kill_score
server.dll!0x99a5e0 ConVar mp_dm_time_between_bonus_max
server.dll!0x99a588 ConVar mp_dm_time_between_bonus_min
server.dll!0x996d40 ConVar mp_do_warmup_offine
server.dll!0x996ce8 ConVar mp_do_warmup_period
server.dll!0x99a740 ConVar mp_dogtag_despawn_on_killer_death
server.dll!0x99a798 ConVar mp_dogtag_despawn_time
server.dll!0x99a6e8 ConVar mp_dogtag_pickup_rule
server.dll!0x99dfb0 ConVar mp_drop_grenade_enable
server.dll!0x99df58 ConVar mp_drop_knife_enable
server.dll!0x998c10 ConVar mp_economy_reset_rounds
server.dll!0x9929a0 ConVar mp_enableroundwaittime
server.dll!0x9993f8 ConVar mp_endmatch_votenextleveltime
server.dll!0x999348 ConVar mp_endmatch_votenextmap
server.dll!0x9993a0 ConVar mp_endmatch_votenextmap_keepcurrent
server.dll!0x99bd10 ConVar mp_endmatch_votenextmap_wargames_modes
server.dll!0x99bd68 ConVar mp_endmatch_votenextmap_wargames_nummaps
server.dll!0x99bdc0 ConVar mp_endmatch_votenextmap_wargames_nummodes
server.dll!0x997560 ConVar mp_endwarmup_player_count
server.dll!0x998bb8 ConVar mp_equipment_reset_rounds
server.dll!0x984ff8 ConVar mp_facefronttime
server.dll!0x98c1c8 ConVar mp_falldamage
server.dll!0x984fa0 ConVar mp_feetyawrate
server.dll!0x98c328 ConVar mp_flashlight
server.dll!0x99e908 ConVar mp_flinch_punch_scale
server.dll!0x98c2d0 ConVar mp_footsteps
server.dll!0x996a28 ConVar mp_force_assign_teams
server.dll!0x998ab0 ConVar mp_force_pick_time
server.dll!0x98c278 ConVar mp_forcerespawn
server.dll!0x98e750 ConVar mp_fraglimit
server.dll!0x99cfd8 ConVar mp_free_armor
server.dll!0x999e50 ConVar mp_freezetime
server.dll!0x98d4b0 ConVar mp_friendlyfire
server.dll!0x997b38 ConVar mp_ggprogressive_random_weapon_kills_needed
server.dll!0x997a88 ConVar mp_ggprogressive_round_restart_delay
server.dll!0x997ae0 ConVar mp_ggprogressive_use_random_weapons
server.dll!0x997980 ConVar mp_ggtr_always_upgrade
server.dll!0x9981c0 ConVar mp_ggtr_bomb_defuse_bonus
server.dll!0x998218 ConVar mp_ggtr_bomb_detonation_bonus
server.dll!0x998008 ConVar mp_ggtr_bomb_pts_for_flash
server.dll!0x997fb0 ConVar mp_ggtr_bomb_pts_for_he
server.dll!0x998060 ConVar mp_ggtr_bomb_pts_for_molotov
server.dll!0x997f58 ConVar mp_ggtr_bomb_pts_for_upgrade
server.dll!0x998168 ConVar mp_ggtr_bomb_respawn_delay
server.dll!0x9979d8 ConVar mp_ggtr_end_round_kill_bonus
server.dll!0x998110 ConVar mp_ggtr_halftime_delay
server.dll!0x997a30 ConVar mp_ggtr_last_weapon_kill_ends_half
server.dll!0x997b90 ConVar mp_ggtr_num_rounds_autoprogress
server.dll!0x9986e8 ConVar mp_give_player_c4
server.dll!0x99aab0 ConVar mp_global_damage_per_second
server.dll!0x99a2c8 ConVar mp_guardian_bot_money_per_wave
server.dll!0x99a428 ConVar mp_guardian_loc_adjective
server.dll!0x99a480 ConVar mp_guardian_loc_condition
server.dll!0x99a4d8 ConVar mp_guardian_loc_icon
server.dll!0x99a378 ConVar mp_guardian_loc_mission
server.dll!0x99a320 ConVar mp_guardian_loc_override
server.dll!0x99a3d0 ConVar mp_guardian_loc_weapon
server.dll!0x99a270 ConVar mp_guardian_player_dist_max
server.dll!0x99a218 ConVar mp_guardian_player_dist_min
server.dll!0x99a168 ConVar mp_guardian_special_kills_needed
server.dll!0x99a1c0 ConVar mp_guardian_special_weapon_needed
server.dll!0x9a4880 ConVar mp_guardian_target_site
server.dll!0x99d088 ConVar mp_halftime
server.dll!0x9978d0 ConVar mp_halftime_duration
server.dll!0x997668 ConVar mp_halftime_pausematch
server.dll!0x997610 ConVar mp_halftime_pausetimer
server.dll!0x995990 ConVar mp_heavyassaultsuit_aimpunch
server.dll!0x998e78 ConVar mp_heavyassaultsuit_cooldown
server.dll!0x995938 ConVar mp_heavyassaultsuit_deploy_timescale
server.dll!0x995888 ConVar mp_heavyassaultsuit_speed
server.dll!0x9958e0 ConVar mp_heavybot_damage_reduction_scale
server.dll!0x9a4380 ConVar mp_hostagepenalty
server.dll!0x9a46a0 ConVar mp_hostages_max
server.dll!0x997108 ConVar mp_hostages_rescuetime
server.dll!0x9970b0 ConVar mp_hostages_rescuetowin
server.dll!0x9a4538 ConVar mp_hostages_run_speed_modifier
server.dll!0x9a47a8 ConVar mp_hostages_spawn_farthest
server.dll!0x9a4750 ConVar mp_hostages_spawn_force_positions
server.dll!0x9a46f8 ConVar mp_hostages_spawn_same_every_round
server.dll!0x997058 ConVar mp_hostages_takedamage
server.dll!0x99a110 ConVar mp_humanteam
server.dll!0x99a530 ConVar mp_ignore_round_win_conditions
server.dll!0x985050 ConVar mp_ik
server.dll!0x997ea8 ConVar mp_join_grace_time
server.dll!0x999ea8 ConVar mp_limitteams
server.dll!0x993fc0 ConVar mp_logdetail
server.dll!0x994018 ConVar mp_logdetail_items
server.dll!0x99dcf0 ConVar mp_logdistance_2d
server.dll!0x99dd48 ConVar mp_logdistance_sec
server.dll!0x99dda0 ConVar mp_logloadouts
server.dll!0x99dc98 ConVar mp_logmoney
server.dll!0x997928 ConVar mp_match_can_clinch
server.dll!0x998638 ConVar mp_match_end_changelevel
server.dll!0x9985e0 ConVar mp_match_end_restart
server.dll!0x98e648 ConVar mp_match_restart_delay
server.dll!0x99d030 ConVar mp_max_armor
server.dll!0x996df0 ConVar mp_maxmoney
server.dll!0x9927e8 ConVar mp_maxrounds
server.dll!0x9980b8 ConVar mp_molotovusedelay
server.dll!0x9a4590 ConVar mp_only_cts_rescue_hostages
server.dll!0x996f50 ConVar mp_overtime_enable
server.dll!0x9976c0 ConVar mp_overtime_halftime_pausetimer
server.dll!0x996fa8 ConVar mp_overtime_maxrounds
server.dll!0x997000 ConVar mp_overtime_startmoney
server.dll!0x9a5488 ConVar mp_plant_c4_anywhere
server.dll!0x99a950 ConVar mp_player_healthbuffer_decay_rate
server.dll!0x996ea0 ConVar mp_playercashawards
server.dll!0x997770 ConVar mp_playerid
server.dll!0x9977c8 ConVar mp_playerid_delay
server.dll!0x997820 ConVar mp_playerid_hold
server.dll!0x98d400 ConVar mp_radar_showall
server.dll!0x99d0e0 ConVar mp_randomspawn
server.dll!0x99d190 ConVar mp_randomspawn_dist
server.dll!0x99d138 ConVar mp_randomspawn_los
server.dll!0x998740 ConVar mp_require_gun_use_to_acquire
server.dll!0x997718 ConVar mp_respawn_immunitytime
server.dll!0x999030 ConVar mp_respawn_on_death_ct
server.dll!0x998fd8 ConVar mp_respawn_on_death_t
server.dll!0x992688 ConVar mp_respawnwavetime
server.dll!0x98e800 ConVar mp_restartgame
server.dll!0x997878 ConVar mp_round_restart_delay
server.dll!0x999cf0 ConVar mp_roundtime
server.dll!0x999df8 ConVar mp_roundtime_defuse
server.dll!0x999d48 ConVar mp_roundtime_deployment
server.dll!0x999da0 ConVar mp_roundtime_hostage
server.dll!0x9929f8 ConVar mp_showcleanedupents
server.dll!0x992948 ConVar mp_showroundtransitions
server.dll!0x99d2c8 ConVar mp_solid_teammates
server.dll!0x999fb0 ConVar mp_spawnprotectiontime
server.dll!0x9969d0 ConVar mp_spec_swapplayersides
server.dll!0x996b88 ConVar mp_spectators_max
server.dll!0x992bb0 ConVar mp_stalemate_at_timelimit
server.dll!0x992b58 ConVar mp_stalemate_enable
server.dll!0x992aa8 ConVar mp_stalemate_timelimit
server.dll!0x99ab08 ConVar mp_starting_losses
server.dll!0x996d98 ConVar mp_startmoney
server.dll!0x990168 ConVar mp_suicide_time
server.dll!0x997e50 ConVar mp_t_default_grenades
server.dll!0x997d48 ConVar mp_t_default_melee
server.dll!0x997df8 ConVar mp_t_default_primary
server.dll!0x997da0 ConVar mp_t_default_secondary
server.dll!0x99e888 ConVar mp_tagging_scale
server.dll!0x9a67f0 ConVar mp_taser_recharge_time
server.dll!0x99a0b8 ConVar mp_td_dmgtokick
server.dll!0x99a060 ConVar mp_td_dmgtowarn
server.dll!0x99a008 ConVar mp_td_spawndmgthreshold
server.dll!0x9950c0 ConVar mp_team_timeout_max
server.dll!0x995068 ConVar mp_team_timeout_time
server.dll!0x996ef8 ConVar mp_teamcashawards
server.dll!0x9953b8 ConVar mp_teamflag_1
server.dll!0x995410 ConVar mp_teamflag_2
server.dll!0x98c3d8 ConVar mp_teamlist
server.dll!0x995468 ConVar mp_teamlogo_1
server.dll!0x9954c0 ConVar mp_teamlogo_2
server.dll!0x995620 ConVar mp_teammatchstat_1
server.dll!0x995678 ConVar mp_teammatchstat_2
server.dll!0x995830 ConVar mp_teammatchstat_cycletime
server.dll!0x9957d8 ConVar mp_teammatchstat_holdtime
server.dll!0x9955c8 ConVar mp_teammatchstat_txt
server.dll!0x98d508 ConVar mp_teammates_are_enemies
server.dll!0x995308 ConVar mp_teamname_1
server.dll!0x995360 ConVar mp_teamname_2
server.dll!0x98c170 ConVar mp_teamplay
server.dll!0x995570 ConVar mp_teamprediction_pct
server.dll!0x995518 ConVar mp_teamprediction_txt
server.dll!0x992790 ConVar mp_teams_unbalance_limit
server.dll!0x9956d0 ConVar mp_teamscore_1
server.dll!0x995728 ConVar mp_teamscore_2
server.dll!0x999f00 ConVar mp_tkpunish
server.dll!0x992738 ConVar mp_tournament
server.dll!0x999088 ConVar mp_use_respawn_waves
server.dll!0x99d378 ConVar mp_verbose_changelevel_spew
server.dll!0x9975b8 ConVar mp_warmup_pausetimer
server.dll!0x997508 ConVar mp_warmuptime_all_players_connected
server.dll!0x9a63b8 ConVar mp_weapon_melee_touch_time_after_hit
server.dll!0x9a5a68 ConVar mp_weapon_next_owner_touch_time
server.dll!0x9a5a10 ConVar mp_weapon_prev_owner_touch_time
server.dll!0x99aa00 ConVar mp_weapon_self_inflict_amount
server.dll!0x998d70 ConVar mp_weapons_allow_heavy
server.dll!0x99d480 ConVar mp_weapons_allow_map_placed
server.dll!0x998cc0 ConVar mp_weapons_allow_pistols
server.dll!0x998dc8 ConVar mp_weapons_allow_rifles
server.dll!0x998d18 ConVar mp_weapons_allow_smgs
server.dll!0x998ed0 ConVar mp_weapons_allow_typecount
server.dll!0x998c68 ConVar mp_weapons_allow_zeus
server.dll!0x998f80 ConVar mp_weapons_glow_on_ground
server.dll!0x995118 ConVar mp_weapons_max_gun_purchases_per_weapon_per_match
server.dll!0x98c220 ConVar mp_weaponstay
server.dll!0x997f00 ConVar mp_win_panel_display_time
server.dll!0x992840 ConVar mp_winlimit
server.dll!0x9a6cc8 ConVar nav_area_bgcolor
server.dll!0x9a7838 ConVar nav_area_max_size
server.dll!0x9a6bc0 ConVar nav_coplanar_slope_limit
server.dll!0x9a6c18 ConVar nav_coplanar_slope_limit_displacement
server.dll!0x9a6d20 ConVar nav_corner_adjust_adjacent
server.dll!0x9a7350 ConVar nav_create_area_at_feet
server.dll!0x9a72a0 ConVar nav_create_place_on_ground
server.dll!0x9a6dd0 ConVar nav_debug_blocked
server.dll!0x9a7680 ConVar nav_displacement_test
server.dll!0x9a73a8 ConVar nav_drag_selection_volume_zmax_offset
server.dll!0x9a7400 ConVar nav_drag_selection_volume_zmin_offset
server.dll!0x9a72f8 ConVar nav_draw_limit
server.dll!0x9a8750 ConVar nav_edit
server.dll!0x9a76d8 ConVar nav_generate_fencetops
server.dll!0x9a7730 ConVar nav_generate_fixup_jump_areas
server.dll!0x9a7788 ConVar nav_generate_incremental_range
server.dll!0x9a77e0 ConVar nav_generate_incremental_tolerance
server.dll!0x9a6e80 ConVar nav_max_view_distance
server.dll!0x9a7a88 ConVar nav_max_vis_delta_list_length
server.dll!0x9a6f30 ConVar nav_potentially_visible_dot_tolerance
server.dll!0x9a86f8 ConVar nav_quicksave
server.dll!0x9a7980 ConVar nav_show_approach_points
server.dll!0x9a71f0 ConVar nav_show_area_info
server.dll!0x9a7470 ConVar nav_show_compass
server.dll!0x9a6e28 ConVar nav_show_continguous
server.dll!0x9a79d8 ConVar nav_show_danger
server.dll!0x9a6d78 ConVar nav_show_light_intensity
server.dll!0x9a8910 ConVar nav_show_node_grid
server.dll!0x9a87b0 ConVar nav_show_node_id
server.dll!0x9a8968 ConVar nav_show_nodes
server.dll!0x9a7a30 ConVar nav_show_player_counts
server.dll!0x9a6f88 ConVar nav_show_potentially_visible
server.dll!0x9a75d0 ConVar nav_slope_limit
server.dll!0x9a7628 ConVar nav_slope_tolerance
server.dll!0x9a7248 ConVar nav_snap_to_grid
server.dll!0x9a7538 ConVar nav_solid_props
server.dll!0x9a6c70 ConVar nav_split_place_on_ground
server.dll!0x9a8808 ConVar nav_test_node
server.dll!0x9a8860 ConVar nav_test_node_crouch
server.dll!0x9a88b8 ConVar nav_test_node_crouch_dir
server.dll!0x9a6ed8 ConVar nav_update_visibility_on_edit
server.dll!0x98ea90 ConVar nextlevel
server.dll!0x98ea38 ConVar nextmap_print_enabled
server.dll!0x98eae8 ConVar nextmode
server.dll!0x986a08 ConVar noclip_fixup
server.dll!0x984990 ConVar npc_ally_deathmessage
server.dll!0x9851d0 ConVar npc_height_adjust
server.dll!0x98c600 ConVar occlusion_test_camera_margins
server.dll!0x98c658 ConVar occlusion_test_jump_margin
server.dll!0x98c5a8 ConVar occlusion_test_shadow_length
server.dll!0x98d1a8 ConVar old_radiusdamage
server.dll!0x9aa888 ConVar panel_test_title_safe
server.dll!0x98ef40 ConVar particle_test_attach_attachment
server.dll!0x98eee8 ConVar particle_test_attach_mode
server.dll!0x98ee90 ConVar particle_test_file
server.dll!0x98f108 ConVar phys_debug_check_contacts
server.dll!0x99de50 ConVar phys_headshotscale
server.dll!0x99ddf8 ConVar phys_playerscale
server.dll!0x992518 ConVar phys_pushscale
server.dll!0x98f0b0 ConVar phys_show_active
server.dll!0x990378 ConVar player_debug_print_damage
server.dll!0x99f2c0 ConVar post_jump_crouch
server.dll!0x991250 ConVar props_break_max_pieces
server.dll!0x9912a8 ConVar props_break_max_pieces_perframe
server.dll!0x98c6b0 ConVar pvs_min_player_distance
server.dll!0x98def0 ConVar r_AirboatViewDampenDamp
server.dll!0x98de98 ConVar r_AirboatViewDampenFreq
server.dll!0x98df48 ConVar r_AirboatViewZHeight
server.dll!0x993418 ConVar r_JeepFOV
server.dll!0x98dde8 ConVar r_JeepViewDampenDamp
server.dll!0x98dd90 ConVar r_JeepViewDampenFreq
server.dll!0x98de40 ConVar r_JeepViewZHeight
server.dll!0x98dd38 ConVar r_VehicleViewDampen
server.dll!0x98bb40 ConVar r_vehicleBrakeRate
server.dll!0x9932d8 ConVar r_visualizetraces
server.dll!0x993f10 ConVar radarvisdistance
server.dll!0x993f68 ConVar radarvismaxdot
server.dll!0x993e60 ConVar radarvismethod
server.dll!0x993eb8 ConVar radarvispow
server.dll!0x984c58 ConVar rr_followup_maxdist
server.dll!0x9823b0 ConVar rr_remarkable_max_distance
server.dll!0x982300 ConVar rr_remarkable_world_entities_replay_limit
server.dll!0x982358 ConVar rr_remarkables_enabled
server.dll!0x984cb0 ConVar rr_thenany_score_slop
server.dll!0x991b38 ConVar scene_clientflex
server.dll!0x991b90 ConVar scene_print
server.dll!0x982250 ConVar scene_showfaceto
server.dll!0x985fa0 ConVar scene_showlook
server.dll!0x985ff8 ConVar scene_showmoveto
server.dll!0x986050 ConVar scene_showunlock
server.dll!0x98c0c0 ConVar servercfgfile
server.dll!0x992f50 ConVar showtriggers
server.dll!0x98d0d0 ConVar sk_autoaim_mode
server.dll!0x98d200 ConVar skill
server.dll!0x986590 ConVar smoothstairs
server.dll!0x999450 ConVar snd_music_boost
server.dll!0x991ee8 ConVar snd_prevent_ss_duplicates
server.dll!0x991f40 ConVar snd_sos_show_server_xmit
server.dll!0x992170 ConVar soundpatch_captionlength
server.dll!0x992250 ConVar soundscape_debug
server.dll!0x98ff00 ConVar spec_allow_roaming
server.dll!0x98fdf8 ConVar spec_freeze_deathanim_time
server.dll!0x98fda0 ConVar spec_freeze_panel_extended_time
server.dll!0x98fea8 ConVar spec_freeze_target_fov
server.dll!0x98fe50 ConVar spec_freeze_target_fov_long
server.dll!0x98fc98 ConVar spec_freeze_time
server.dll!0x98fcf0 ConVar spec_freeze_time_lock
server.dll!0x98fd48 ConVar spec_freeze_traveltime
server.dll!0x98eb40 ConVar spec_replay_bot
server.dll!0x98fb90 ConVar spec_replay_cam_delay
server.dll!0x98fbe8 ConVar spec_replay_cam_options
server.dll!0x9948d8 ConVar spec_replay_round_delay
server.dll!0x98fc40 ConVar spec_replay_winddown_time
server.dll!0x9a1eb8 ConVar steam_controller_haptics
server.dll!0x992370 ConVar steamworks_sessionid_server
server.dll!0x98c538 ConVar suitvolume
server.dll!0x98dff8 ConVar sv_accelerate
server.dll!0x98db28 ConVar sv_accelerate_debug_speed
server.dll!0x98dad0 ConVar sv_accelerate_use_weapon_speed
server.dll!0x98cd70 ConVar sv_air_max_horizontal_parachute_ratio
server.dll!0x98cd18 ConVar sv_air_max_horizontal_parachute_speed
server.dll!0x98ccc0 ConVar sv_air_max_wishspeed
server.dll!0x98cdc8 ConVar sv_air_pushaway_dist
server.dll!0x98e158 ConVar sv_airaccelerate
server.dll!0x98e100 ConVar sv_airaccelerate_parachute
server.dll!0x98e0a8 ConVar sv_airaccelerate_rappel
server.dll!0x986250 ConVar sv_allchat
server.dll!0x997268 ConVar sv_allow_thirdperson
server.dll!0x993800 ConVar sv_allow_votes
server.dll!0x9999d0 ConVar sv_alltalk
server.dll!0x9a0a78 ConVar sv_arms_race_vote_to_restart_disallowed_after
server.dll!0x994f08 ConVar sv_auto_adjust_bot_difficulty
server.dll!0x999b88 ConVar sv_auto_full_alltalk_during_warmup_half_end
server.dll!0x994690 ConVar sv_autobunnyhopping
server.dll!0x99e4d8 ConVar sv_autobuyammo
server.dll!0x98dc88 ConVar sv_backspeed
server.dll!0x98ff58 ConVar sv_bonus_challenge
server.dll!0x9a39c0 ConVar sv_bot_buy_decoy_weight
server.dll!0x9a3968 ConVar sv_bot_buy_flash_weight
server.dll!0x9a38b8 ConVar sv_bot_buy_grenade_chance
server.dll!0x9a3a70 ConVar sv_bot_buy_hegrenade_weight
server.dll!0x9a3a18 ConVar sv_bot_buy_molotov_weight
server.dll!0x9a3910 ConVar sv_bot_buy_smoke_weight
server.dll!0x999768 ConVar sv_bot_difficulty_gamepad
server.dll!0x999818 ConVar sv_bot_difficulty_hydra
server.dll!0x999710 ConVar sv_bot_difficulty_kbm
server.dll!0x9997c0 ConVar sv_bot_difficulty_ps3move
server.dll!0x999870 ConVar sv_bot_difficulty_sharpshooter
server.dll!0x995010 ConVar sv_bots_force_rebuy_every_round
server.dll!0x994f60 ConVar sv_bots_get_easier_each_win
server.dll!0x994fb8 ConVar sv_bots_get_harder_after_each_wave
server.dll!0x98e310 ConVar sv_bounce
server.dll!0x9a53e8 ConVar sv_breachcharge_arm_delay
server.dll!0x9a52e0 ConVar sv_breachcharge_delay_max
server.dll!0x9a5288 ConVar sv_breachcharge_delay_min
server.dll!0x9a5230 ConVar sv_breachcharge_distance_max
server.dll!0x9a51d8 ConVar sv_breachcharge_distance_min
server.dll!0x9a5390 ConVar sv_breachcharge_fuse_max
server.dll!0x9a5338 ConVar sv_breachcharge_fuse_min
server.dll!0x9a19a8 ConVar sv_broadcast_ugc_download_progress_interval
server.dll!0x9a1950 ConVar sv_broadcast_ugc_downloads
server.dll!0x994e58 ConVar sv_buy_status_override
server.dll!0x994d50 ConVar sv_chat_proximity
server.dll!0x985de8 ConVar sv_clamp_unsafe_velocities
server.dll!0x99ef90 ConVar sv_clip_penetration_traces_to_players
server.dll!0x990558 ConVar sv_clockcorrection_msecs
server.dll!0x997210 ConVar sv_coach_comm_unrestricted
server.dll!0x99d5e0 ConVar sv_coaching_enabled
server.dll!0x99d1e8 ConVar sv_competitive_minspec
server.dll!0x9998c8 ConVar sv_competitive_official_5v5
server.dll!0x999660 ConVar sv_compute_per_bot_difficulty
server.dll!0x99eee0 ConVar sv_cs_player_speed_has_hostage
server.dll!0x994eb0 ConVar sv_ct_spawn_on_bombsite
server.dll!0x99dea8 ConVar sv_damage_print_enable
server.dll!0x98c8a0 ConVar sv_dc_friends_reqd
server.dll!0x99d320 ConVar sv_deadtalk
server.dll!0x9864e0 ConVar sv_debug_player_use
server.dll!0x9a18f8 ConVar sv_debug_ugc_downloads
server.dll!0x999978 ConVar sv_disable_immunity_alpha
server.dll!0x9951c8 ConVar sv_disable_motd
server.dll!0x994930 ConVar sv_disable_observer_interpolation
server.dll!0x9914b0 ConVar sv_disable_pas
server.dll!0x991300 ConVar sv_disable_querycache
server.dll!0x994c48 ConVar sv_disable_radar
server.dll!0x99e218 ConVar sv_disablefreezecam
server.dll!0x990450 ConVar sv_drowning_damage_initial
server.dll!0x9904a8 ConVar sv_drowning_damage_max
server.dll!0x9a4a40 ConVar sv_dz_autojointeam
server.dll!0x99eb38 ConVar sv_dz_cash_bundle_size
server.dll!0x99e690 ConVar sv_dz_contractkill_reward
server.dll!0x9a6770 ConVar sv_dz_exploration_payment_amount
server.dll!0x997318 ConVar sv_dz_hostage_rescue_reward
server.dll!0x9a49e8 ConVar sv_dz_jointeam_allowed
server.dll!0x9a4990 ConVar sv_dz_player_max_health
server.dll!0x9a4938 ConVar sv_dz_player_spawn_armor
server.dll!0x9a48e0 ConVar sv_dz_player_spawn_health
server.dll!0x9a4ba0 ConVar sv_dz_show_enemy_name_scope_range
server.dll!0x9a4da0 ConVar sv_dz_team_count
server.dll!0x9a4b48 ConVar sv_dz_warmup_tablet
server.dll!0x9a4af0 ConVar sv_dz_warmup_weapon
server.dll!0x986f68 ConVar sv_dz_zone_bombdrop_money_reward
server.dll!0x986fc0 ConVar sv_dz_zone_damage
server.dll!0x986f10 ConVar sv_dz_zone_hex_radius
server.dll!0x994638 ConVar sv_enablebunnyhopping
server.dll!0x9994a8 ConVar sv_endmatch_item_drop_interval
server.dll!0x999608 ConVar sv_endmatch_item_drop_interval_ancient
server.dll!0x9995b0 ConVar sv_endmatch_item_drop_interval_legendary
server.dll!0x999558 ConVar sv_endmatch_item_drop_interval_mythical
server.dll!0x999500 ConVar sv_endmatch_item_drop_interval_rare
server.dll!0x98b400 ConVar sv_env_entity_makers_enabled
server.dll!0x9863d8 ConVar sv_extract_ammo_from_dropped_weapons
server.dll!0x994470 ConVar sv_extreme_strafe_accuracy_fishtail
server.dll!0x994cf8 ConVar sv_falldamage_scale
server.dll!0x99d428 ConVar sv_falldamage_to_below_player_multiplier
server.dll!0x99d3d0 ConVar sv_falldamage_to_below_player_ratio
server.dll!0x9a6110 ConVar sv_fistpoint_delay
server.dll!0x9a60b8 ConVar sv_fistpunch_blocked_damage
server.dll!0x9a5fb0 ConVar sv_fistpunch_damage
server.dll!0x9a61e8 ConVar sv_fistpunch_damage_hard
server.dll!0x9a6008 ConVar sv_fistpunch_damage_to_player_multiplier
server.dll!0x9a6168 ConVar sv_fistpunch_impact_sounds
server.dll!0x9a6060 ConVar sv_fistpunch_viewmove
server.dll!0x9a1ba8 ConVar sv_flashbang_strength
server.dll!0x986380 ConVar sv_footstep_sound_frequency
server.dll!0x98e470 ConVar sv_footsteps
server.dll!0x9a4a98 ConVar sv_force_reflections
server.dll!0x98c798 ConVar sv_force_transmit_ents
server.dll!0x98ffb0 ConVar sv_force_transmit_players
server.dll!0x98dfa0 ConVar sv_friction
server.dll!0x999a28 ConVar sv_full_alltalk
server.dll!0x996a80 ConVar sv_gameinstructor_disable
server.dll!0x98d7a8 ConVar sv_grassburn
server.dll!0x98f780 ConVar sv_grenade_trajectory
server.dll!0x98f8e0 ConVar sv_grenade_trajectory_dash
server.dll!0x98f888 ConVar sv_grenade_trajectory_thickness
server.dll!0x98f7d8 ConVar sv_grenade_trajectory_time
server.dll!0x98f830 ConVar sv_grenade_trajectory_time_spectator
server.dll!0x99e168 ConVar sv_guardian_heavy_all
server.dll!0x99eae0 ConVar sv_guardian_heavy_count
server.dll!0x99e110 ConVar sv_guardian_max_wave_for_heavy
server.dll!0x99e0b8 ConVar sv_guardian_min_wave_for_heavy
server.dll!0x9852f0 ConVar sv_health_approach_enabled
server.dll!0x985348 ConVar sv_health_approach_speed
server.dll!0x9a2308 ConVar sv_hegrenade_damage_multiplier
server.dll!0x9a2360 ConVar sv_hegrenade_radius_multiplier
server.dll!0x9949e0 ConVar sv_holiday_mode
server.dll!0x9a5168 ConVar sv_ignoregrenaderadio
server.dll!0x9865e8 ConVar sv_infinite_ammo
server.dll!0x9944c8 ConVar sv_jump_impulse
server.dll!0x999920 ConVar sv_kick_ban_duration
server.dll!0x994da8 ConVar sv_kick_players_with_cooldown
server.dll!0x9a6268 ConVar sv_knife_attack_extend_from_player_aabb
server.dll!0x98cfc8 ConVar sv_ladder_angle
server.dll!0x98cf70 ConVar sv_ladder_dampen
server.dll!0x98d020 ConVar sv_ladder_scale_speed
server.dll!0x990a90 ConVar sv_lagcompensateself
server.dll!0x990988 ConVar sv_lagcompensationforcerestore
server.dll!0x994520 ConVar sv_ledge_mantle_helper
server.dll!0x9945d0 ConVar sv_ledge_mantle_helper_debug
server.dll!0x994578 ConVar sv_ledge_mantle_helper_dzonly
server.dll!0x994e00 ConVar sv_matchend_drops_enabled
server.dll!0x996b30 ConVar sv_matchpause_auto_5v5
server.dll!0x986488 ConVar sv_max_distance_transmit_footsteps
server.dll!0x98e418 ConVar sv_maxspeed
server.dll!0x990b40 ConVar sv_maxunlag
server.dll!0x990060 ConVar sv_maxusrcmdprocessticks
server.dll!0x9907d0 ConVar sv_maxusrcmdprocessticks_holdaim
server.dll!0x990778 ConVar sv_maxusrcmdprocessticks_warning
server.dll!0x98e2b8 ConVar sv_maxvelocity
server.dll!0x99ee88 ConVar sv_min_jump_landing_sound
server.dll!0x985698 ConVar sv_netvisdist
server.dll!0x98e368 ConVar sv_noclipaccelerate
server.dll!0x990110 ConVar sv_noclipduringpause
server.dll!0x98e3c0 ConVar sv_noclipspeed
server.dll!0x98ca28 ConVar sv_occlude_players
server.dll!0x98d078 ConVar sv_optimizedmovement
server.dll!0x994ca0 ConVar sv_outofammo_indicator
server.dll!0x9972c0 ConVar sv_party_mode
server.dll!0x99f268 ConVar sv_penetration_type
server.dll!0x99e830 ConVar sv_player_parachute_velocity
server.dll!0x994198 ConVar sv_prime_accounts_only
server.dll!0x990f40 ConVar sv_prop_door_open_speed_scale
server.dll!0x98ecd8 ConVar sv_pushaway_clientside
server.dll!0x9911f8 ConVar sv_pushaway_clientside_size
server.dll!0x98ede0 ConVar sv_pushaway_force
server.dll!0x9a4488 ConVar sv_pushaway_hostage_force
server.dll!0x98ee38 ConVar sv_pushaway_max_force
server.dll!0x9a44e0 ConVar sv_pushaway_max_hostage_force
server.dll!0x98ed88 ConVar sv_pushaway_max_player_force
server.dll!0x98ec80 ConVar sv_pushaway_min_player_speed
server.dll!0x98ed30 ConVar sv_pushaway_player_force
server.dll!0x9850c0 ConVar sv_pvsskipanimation
server.dll!0x990008 ConVar sv_regeneration_wait_time
server.dll!0x9a1a58 ConVar sv_remove_old_ugc_downloads
server.dll!0x994988 ConVar sv_reward_drop_delay
server.dll!0x98e520 ConVar sv_rollangle
server.dll!0x98e4c8 ConVar sv_rollspeed
server.dll!0x99d530 ConVar sv_server_graphic1
server.dll!0x99d588 ConVar sv_server_graphic2
server.dll!0x99f040 ConVar sv_server_verify_blood_on_player
server.dll!0x9996b8 ConVar sv_show_bot_difficulty_in_name
server.dll!0x994ae8 ConVar sv_show_team_equipment_force_on
server.dll!0x994a90 ConVar sv_show_team_equipment_prohibit
server.dll!0x99e2c8 ConVar sv_show_voip_indicator_for_enemies
server.dll!0x984e98 ConVar sv_showanimstate
server.dll!0x984f48 ConVar sv_showanimstate_activities
server.dll!0x984ef0 ConVar sv_showanimstate_log
server.dll!0x99ec78 ConVar sv_showbullethits
server.dll!0x99ecd0 ConVar sv_showimpacts
server.dll!0x99ec20 ConVar sv_showimpacts_penetration
server.dll!0x99ed28 ConVar sv_showimpacts_time
server.dll!0x9908d8 ConVar sv_showlagcompensation
server.dll!0x990930 ConVar sv_showlagcompensation_duration
server.dll!0x99ed80 ConVar sv_showplayerhitboxes
server.dll!0x99d4d8 ConVar sv_skirmish_id
server.dll!0x98dc30 ConVar sv_skyname
server.dll!0x9a2558 ConVar sv_snowball_strength
server.dll!0x991f98 ConVar sv_soundemitter_trace
server.dll!0x991e90 ConVar sv_soundemitter_version
server.dll!0x99df00 ConVar sv_spawn_afk_bomb_drop_time
server.dll!0x99ef38 ConVar sv_spawn_rappel_min_duration
server.dll!0x999be0 ConVar sv_spec_hear
server.dll!0x990500 ConVar sv_spec_post_death_additional_time
server.dll!0x99ee30 ConVar sv_spec_use_tournament_content_standards
server.dll!0x98e208 ConVar sv_specaccelerate
server.dll!0x98e1b0 ConVar sv_specnoclip
server.dll!0x98e260 ConVar sv_specspeed
server.dll!0x9942b8 ConVar sv_staminajumpcost
server.dll!0x994310 ConVar sv_staminalandcost
server.dll!0x9943c0 ConVar sv_staminamax
server.dll!0x994368 ConVar sv_staminarecoveryrate
server.dll!0x98ce78 ConVar sv_standable_normal
server.dll!0x98e5d0 ConVar sv_stepsize
server.dll!0x98e050 ConVar sv_stopspeed
server.dll!0x986538 ConVar sv_suppress_viewpunch
server.dll!0x9a6718 ConVar sv_tablet_show_path_to_nearest_resq
server.dll!0x999b30 ConVar sv_talk_after_dying_time
server.dll!0x999a80 ConVar sv_talk_enemy_dead
server.dll!0x999ad8 ConVar sv_talk_enemy_living
server.dll!0x994bf0 ConVar sv_teamid_overhead
server.dll!0x994a38 ConVar sv_teamid_overhead_always_prohibit
server.dll!0x994b98 ConVar sv_teamid_overhead_maxdist
server.dll!0x994b40 ConVar sv_teamid_overhead_maxdist_spec
server.dll!0x994418 ConVar sv_timebetweenducks
server.dll!0x9911a0 ConVar sv_turbophysics
server.dll!0x9a5928 ConVar sv_turning_inaccuracy_angle_min
server.dll!0x9a5980 ConVar sv_turning_inaccuracy_decay
server.dll!0x9a58d0 ConVar sv_turning_inaccuracy_enabled
server.dll!0x9a1a00 ConVar sv_ugc_manager_max_new_file_check_interval_secs
server.dll!0x98caf8 ConVar sv_unlockedchapters
server.dll!0x990608 ConVar sv_usercmd_custom_random_seed
server.dll!0x993658 ConVar sv_voice_proximity
server.dll!0x9936b0 ConVar sv_voice_proximity_positional
server.dll!0x993a10 ConVar sv_vote_allow_in_warmup
server.dll!0x993960 ConVar sv_vote_allow_spectators
server.dll!0x9937a8 ConVar sv_vote_command_delay
server.dll!0x9939b8 ConVar sv_vote_count_spectator_votes
server.dll!0x9938b0 ConVar sv_vote_creation_timer
server.dll!0x993a68 ConVar sv_vote_disallow_kick_on_match_point
server.dll!0x993858 ConVar sv_vote_failure_timer
server.dll!0x9a0ad0 ConVar sv_vote_issue_kick_allowed
server.dll!0x9a0b80 ConVar sv_vote_issue_loadbackup_allowed
server.dll!0x9a0c88 ConVar sv_vote_issue_loadbackup_spec_authoritative
server.dll!0x9a0bd8 ConVar sv_vote_issue_loadbackup_spec_only
server.dll!0x9a0c30 ConVar sv_vote_issue_loadbackup_spec_safe
server.dll!0x9a0fa0 ConVar sv_vote_issue_pause_match_spec_only
server.dll!0x9a0a20 ConVar sv_vote_issue_restart_game_allowed
server.dll!0x9a0b28 ConVar sv_vote_kick_ban_duration
server.dll!0x993908 ConVar sv_vote_quorum_ratio
server.dll!0x993750 ConVar sv_vote_timer_duration
server.dll!0x9a0d38 ConVar sv_vote_to_changelevel_before_match_point
server.dll!0x98ce20 ConVar sv_walkable_normal
server.dll!0x98cc10 ConVar sv_water_movespeed_multiplier
server.dll!0x98cc68 ConVar sv_water_swim_mode
server.dll!0x98db80 ConVar sv_wateraccelerate
server.dll!0x98dce0 ConVar sv_waterdist
server.dll!0x98dbd8 ConVar sv_waterfriction
server.dll!0x99efe8 ConVar sv_weapon_encumbrance_per_item
server.dll!0x9946e8 ConVar sv_weapon_encumbrance_scale
server.dll!0x99e988 ConVar sv_weapon_require_use_grace_period
server.dll!0x994090 ConVar sv_workshop_allow_other_maps
server.dll!0x9a66a8 ConVar tablet_c4_dist_max
server.dll!0x9a6650 ConVar tablet_c4_dist_min
server.dll!0x98f990 ConVar think_limit
server.dll!0x999240 ConVar tr_best_course_time
server.dll!0x9991e8 ConVar tr_completed_training
server.dll!0x98d960 ConVar tv_allow_autorecording_index
server.dll!0x9940e8 ConVar tv_allow_camera_man_steamid
server.dll!0x994140 ConVar tv_allow_camera_man_steamid2
server.dll!0x98d908 ConVar tv_allow_static_shots
server.dll!0x98d858 ConVar tv_delay
server.dll!0x98d8b0 ConVar tv_delay1
server.dll!0x98e7a8 ConVar tv_delaymapchange
server.dll!0x99e008 ConVar tv_relayradio
server.dll!0x986780 ConVar tv_relaytextchat
server.dll!0x9862d0 ConVar view_punch_decay
server.dll!0x986328 ConVar view_recoil_tracking
server.dll!0x9903f8 ConVar vis_force
server.dll!0x986e58 ConVar vismon_poll_frequency
server.dll!0x986eb0 ConVar vismon_trace_limit
server.dll!0x990bb0 ConVar voice_player_speaking_delay_threshold
server.dll!0x9a5668 ConVar weapon_accuracy_forcespread
server.dll!0x9a1e60 ConVar weapon_accuracy_logging
server.dll!0x9a56c0 ConVar weapon_accuracy_nospread
server.dll!0x9a2070 ConVar weapon_accuracy_shotgun_spread_patterns
server.dll!0x9a5770 ConVar weapon_air_spread_scale
server.dll!0x9a5820 ConVar weapon_auto_cleanup_time
server.dll!0x9a1fc0 ConVar weapon_debug_inaccuracy_only_up
server.dll!0x9a1f68 ConVar weapon_debug_max_inaccuracy
server.dll!0x9a5560 ConVar weapon_land_dip_amt
server.dll!0x9a5b70 ConVar weapon_max_before_cleanup
server.dll!0x9a2438 ConVar weapon_molotov_maxdetonateslope
server.dll!0x9a1f10 ConVar weapon_near_empty_sound
server.dll!0x9a5718 ConVar weapon_recoil_cooldown
server.dll!0x9a55b8 ConVar weapon_recoil_decay1_exp
server.dll!0x9a5c20 ConVar weapon_recoil_decay2_exp
server.dll!0x9a5bc8 ConVar weapon_recoil_decay2_lin
server.dll!0x9a5610 ConVar weapon_recoil_decay_coefficient
server.dll!0x9a5b18 ConVar weapon_recoil_scale
server.dll!0x9a5ac0 ConVar weapon_recoil_scale_motion_controller
server.dll!0x9a11b0 ConVar weapon_recoil_suppression_factor
server.dll!0x9a1158 ConVar weapon_recoil_suppression_shots
server.dll!0x9a1208 ConVar weapon_recoil_variance
server.dll!0x9a5c78 ConVar weapon_recoil_vel_decay
server.dll!0x99f098 ConVar weapon_recoil_view_punch_extra
server.dll!0x9a57c8 ConVar weapon_reticle_knife_show
server.dll!0x9a5878 ConVar weapon_sound_falloff_multiplier
server.dll!0x98bc48 ConVar xbox_autothrottle
server.dll!0x98bb98 ConVar xbox_throttlebias
server.dll!0x98bbf0 ConVar xbox_throttlespoof
```

## ConCommands

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

### Addresses

```
server.dll!0x0099e7b0 ConCommand CreatePredictionError
server.dll!0x00992d24 ConCommand Test_EHandle
server.dll!0x00992dac ConCommand Test_InitRandomEntitySpawner
server.dll!0x00992d64 ConCommand Test_ProxyToggle_EnableProxy
server.dll!0x00992d88 ConCommand Test_ProxyToggle_SetValue
server.dll!0x00992df4 ConCommand Test_RandomizeInPVS
server.dll!0x00992e18 ConCommand Test_RemoveAllRandomEntities
server.dll!0x00992dd0 ConCommand Test_SpawnRandomEntities
server.dll!0x0098d360 ConCommand _resetgamestats
server.dll!0x00983d6c ConCommand ai_clear_bad_links
server.dll!0x009846f0 ConCommand ai_debug_node_connect
server.dll!0x009835f8 ConCommand ai_disable
server.dll!0x00983f28 ConCommand ai_drop_hint
server.dll!0x00983f04 ConCommand ai_dump_hints
server.dll!0x009837a8 ConCommand ai_hull
server.dll!0x00983784 ConCommand ai_next_hull
server.dll!0x009837cc ConCommand ai_nodes
server.dll!0x00983760 ConCommand ai_resume
server.dll!0x00984450 ConCommand ai_set_move_height_epsilon
server.dll!0x0098361c ConCommand ai_setenabled
server.dll!0x00983688 ConCommand ai_show_connect
server.dll!0x009836f4 ConCommand ai_show_connect_crawl
server.dll!0x009836d0 ConCommand ai_show_connect_fly
server.dll!0x009836ac ConCommand ai_show_connect_jump
server.dll!0x00983838 ConCommand ai_show_graph_connect
server.dll!0x00983718 ConCommand ai_show_grid
server.dll!0x00983640 ConCommand ai_show_hints
server.dll!0x00983664 ConCommand ai_show_hull
server.dll!0x009837f0 ConCommand ai_show_node
server.dll!0x00983814 ConCommand ai_show_visibility
server.dll!0x0098373c ConCommand ai_step
server.dll!0x00983d90 ConCommand ai_test_los
server.dll!0x00983db4 ConCommand ainet_generate_report
server.dll!0x00983dd8 ConCommand ainet_generate_report_only
server.dll!0x0098f420 ConCommand air_density
server.dll!0x009a3344 ConCommand bot_add
server.dll!0x009a338c ConCommand bot_add_ct
server.dll!0x009a3368 ConCommand bot_add_t
server.dll!0x009a3464 ConCommand bot_all_weapons
server.dll!0x0099eab8 ConCommand bot_control_next_all_teams
server.dll!0x009a3488 ConCommand bot_goto_mark
server.dll!0x009a34ac ConCommand bot_goto_selected
server.dll!0x009a33d4 ConCommand bot_kick
server.dll!0x009a33b0 ConCommand bot_kill
server.dll!0x009a33f8 ConCommand bot_knives_only
server.dll!0x009a341c ConCommand bot_pistols_only
server.dll!0x009a3320 ConCommand bot_place
server.dll!0x009a3440 ConCommand bot_snipers_only
server.dll!0x00986908 ConCommand buddha
server.dll!0x0099ea04 ConCommand buyrandom
server.dll!0x00993ae4 ConCommand callvote
server.dll!0x0098680c ConCommand cast_hull
server.dll!0x009867e8 ConCommand cast_ray
server.dll!0x009906dc ConCommand ch_createairboat
server.dll!0x009906b8 ConCommand ch_createjeep
server.dll!0x0098b364 ConCommand cl_csm_server_status
server.dll!0x009a54e0 ConCommand clear_bombs
server.dll!0x0098eb98 ConCommand clear_debug_overlays
server.dll!0x00993154 ConCommand collision_test
server.dll!0x00986d58 ConCommand commentary_cvarsnotchanging
server.dll!0x00986d7c ConCommand commentary_finishnode
server.dll!0x0098b480 ConCommand create_flashlight
server.dll!0x0098b544 ConCommand creditsdone
server.dll!0x0099e960 ConCommand cs_make_vip
server.dll!0x0098b38c ConCommand dbghist_addline
server.dll!0x0098b3b0 ConCommand dbghist_dump
server.dll!0x009a4858 ConCommand dm_reset_spawns
server.dll!0x00986854 ConCommand drawcross
server.dll!0x00986830 ConCommand drawline
server.dll!0x009a1b74 ConCommand ds_get_newest_subscribed_files
server.dll!0x00993024 ConCommand dump_entity_sizes
server.dll!0x0098d760 ConCommand dump_globals
server.dll!0x00993000 ConCommand dumpentityfactories
server.dll!0x00986724 ConCommand dumpeventqueue
server.dll!0x0098d384 ConCommand dumpgamestringtable
server.dll!0x009a4ca0 ConCommand dz_clearteams
server.dll!0x009a4c7c ConCommand dz_jointeam
server.dll!0x009a4c58 ConCommand dz_shuffle_teams
server.dll!0x009a4d78 ConCommand dz_spawnselect_choose_hex
server.dll!0x0099ac10 ConCommand endround
server.dll!0x009859f0 ConCommand ent_absbox
server.dll!0x00985a38 ConCommand ent_attachments
server.dll!0x00985ca4 ConCommand ent_autoaim
server.dll!0x009859cc ConCommand ent_bbox
server.dll!0x00985b84 ConCommand ent_cancelpendingentfires
server.dll!0x00985d20 ConCommand ent_create
server.dll!0x00985b34 ConCommand ent_dump
server.dll!0x00985ba8 ConCommand ent_info
server.dll!0x009a8f6c ConCommand ent_keyvalue
server.dll!0x0099ac34 ConCommand ent_list_report
server.dll!0x00985bcc ConCommand ent_messages
server.dll!0x00985960 ConCommand ent_name
server.dll!0x00985d68 ConCommand ent_orient
server.dll!0x00985bf0 ConCommand ent_pause
server.dll!0x00985c38 ConCommand ent_pivot
server.dll!0x00985a14 ConCommand ent_rbox
server.dll!0x00985a80 ConCommand ent_remove
server.dll!0x00985aa4 ConCommand ent_remove_all
server.dll!0x0099101c ConCommand ent_rotate
server.dll!0x009859a8 ConCommand ent_script_dump
server.dll!0x00986af0 ConCommand ent_setang
server.dll!0x00985ac8 ConCommand ent_setname
server.dll!0x00986acc ConCommand ent_setpos
server.dll!0x00985c80 ConCommand ent_show_response_criteria
server.dll!0x00985c5c ConCommand ent_step
server.dll!0x00985d44 ConCommand ent_teleport
server.dll!0x00985984 ConCommand ent_text
server.dll!0x00985a5c ConCommand ent_viewoffset
server.dll!0x0098689c ConCommand explode
server.dll!0x009868e4 ConCommand explodevector
server.dll!0x0098b520 ConCommand fadein
server.dll!0x0098b4fc ConCommand fadeout
server.dll!0x00985aec ConCommand find_ent
server.dll!0x00985b10 ConCommand find_ent_index
server.dll!0x00985b58 ConCommand firetarget
server.dll!0x0098bad4 ConCommand foundry_engine_get_mouse_control
server.dll!0x0098baf8 ConCommand foundry_engine_release_mouse_control
server.dll!0x0098bb1c ConCommand foundry_select_entity
server.dll!0x0098bab0 ConCommand foundry_sync_hammer_view
server.dll!0x0098ba8c ConCommand foundry_update_entity
server.dll!0x00986974 ConCommand give
server.dll!0x009903d0 ConCommand givecurrentammo
server.dll!0x0098da28 ConCommand global_set
server.dll!0x00986a84 ConCommand god
server.dll!0x00986aa8 ConCommand gods
server.dll!0x00986c34 ConCommand groundlist
server.dll!0x00993da8 ConCommand hammer_update_entity
server.dll!0x00993dcc ConCommand hammer_update_safe_entities
server.dll!0x009a1b50 ConCommand host_workshop_collection
server.dll!0x009a1b2c ConCommand host_workshop_map
server.dll!0x00986bec ConCommand hurtme
server.dll!0x0099d750 ConCommand itemtimedata_dump_active
server.dll!0x0099d774 ConCommand itemtimedata_dump_total
server.dll!0x0099d798 ConCommand itemtimedata_print_and_reset
server.dll!0x009930a0 ConCommand kdtree_test
server.dll!0x00986878 ConCommand kill
server.dll!0x009868c0 ConCommand killvector
server.dll!0x009916e4 ConCommand listRecentNPCSpeech
server.dll!0x00993ac0 ConCommand listissues
server.dll!0x009a6b28 ConCommand load_master_item_schema
server.dll!0x00991c58 ConCommand logaddress_add_http
server.dll!0x00991c7c ConCommand logaddress_delall_http
server.dll!0x00991ca0 ConCommand logaddress_list_http
server.dll!0x0099cfb0 ConCommand map_setbombradius
server.dll!0x0099cf8c ConCommand map_showbombradius
server.dll!0x0099cf68 ConCommand map_showspawnpoints
server.dll!0x00995b6c ConCommand mp_backup_restore_list_files
server.dll!0x00995be8 ConCommand mp_backup_restore_load_file
server.dll!0x0099072c ConCommand mp_disable_autokick
server.dll!0x0099bc5c ConCommand mp_dump_timers
server.dll!0x00992c50 ConCommand mp_forcerespawnplayers
server.dll!0x00992c2c ConCommand mp_forcewin
server.dll!0x0099bcc8 ConCommand mp_pause_match
server.dll!0x0099bc80 ConCommand mp_scrambleteams
server.dll!0x0099bca4 ConCommand mp_swapteams
server.dll!0x00992c08 ConCommand mp_switchteams
server.dll!0x00992c74 ConCommand mp_tournament_restart
server.dll!0x0099bcec ConCommand mp_unpause_match
server.dll!0x0099748c ConCommand mp_warmup_end
server.dll!0x00997468 ConCommand mp_warmup_start
server.dll!0x009a7bdc ConCommand nav_add_to_selected_set
server.dll!0x009a7c00 ConCommand nav_add_to_selected_set_by_id
server.dll!0x009a85b4 ConCommand nav_analyze
server.dll!0x009a85d8 ConCommand nav_analyze_scripted
server.dll!0x009a820c ConCommand nav_avoid
server.dll!0x009a805c ConCommand nav_begin_area
server.dll!0x009a7e1c ConCommand nav_begin_deselecting
server.dll!0x009a7d20 ConCommand nav_begin_drag_deselecting
server.dll!0x009a7cd8 ConCommand nav_begin_drag_selecting
server.dll!0x009a7c90 ConCommand nav_begin_selecting
server.dll!0x009a7eac ConCommand nav_begin_shift_xy
server.dll!0x009a8668 ConCommand nav_build_ladder
server.dll!0x009a34d0 ConCommand nav_check_connectivity
server.dll!0x009a75ac ConCommand nav_check_file_consistency
server.dll!0x009a7118 ConCommand nav_check_floor
server.dll!0x009a78a8 ConCommand nav_check_stairs
server.dll!0x009a89c0 ConCommand nav_chop_selected
server.dll!0x009a86b0 ConCommand nav_clear_attribute
server.dll!0x009a7c6c ConCommand nav_clear_selected_set
server.dll!0x009a8620 ConCommand nav_clear_walkable_marks
server.dll!0x009a8644 ConCommand nav_compress_id
server.dll!0x009a80a4 ConCommand nav_connect
server.dll!0x009a84dc ConCommand nav_corner_lower
server.dll!0x009a8500 ConCommand nav_corner_place_on_ground
server.dll!0x009a84b8 ConCommand nav_corner_raise
server.dll!0x009a8494 ConCommand nav_corner_select
server.dll!0x009a8110 ConCommand nav_crouch
server.dll!0x009a7b04 ConCommand nav_delete
server.dll!0x009a7b28 ConCommand nav_delete_marked
server.dll!0x009a80c8 ConCommand nav_disconnect
server.dll!0x009a8254 ConCommand nav_dont_hide
server.dll!0x009a8080 ConCommand nav_end_area
server.dll!0x009a7e40 ConCommand nav_end_deselecting
server.dll!0x009a7d44 ConCommand nav_end_drag_deselecting
server.dll!0x009a7cfc ConCommand nav_end_drag_selecting
server.dll!0x009a7cb4 ConCommand nav_end_selecting
server.dll!0x009a7ed0 ConCommand nav_end_shift_xy
server.dll!0x009a7b4c ConCommand nav_flood_select
server.dll!0x009a7914 ConCommand nav_gen_cliffs_approx
server.dll!0x009a856c ConCommand nav_generate
server.dll!0x009a8590 ConCommand nav_generate_incremental
server.dll!0x009a8158 ConCommand nav_jump
server.dll!0x009a8548 ConCommand nav_ladder_flip
server.dll!0x009a8308 ConCommand nav_load
server.dll!0x009a7d8c ConCommand nav_lower_drag_volume_max
server.dll!0x009a7dd4 ConCommand nav_lower_drag_volume_min
server.dll!0x009a7fcc ConCommand nav_make_sniper_spots
server.dll!0x009a8014 ConCommand nav_mark
server.dll!0x009a86d4 ConCommand nav_mark_attribute
server.dll!0x009a8470 ConCommand nav_mark_unnamed
server.dll!0x009a85fc ConCommand nav_mark_walkable
server.dll!0x009a7ff0 ConCommand nav_merge
server.dll!0x009a795c ConCommand nav_merge_mesh
server.dll!0x009a829c ConCommand nav_no_hostages
server.dll!0x009a817c ConCommand nav_no_jump
server.dll!0x009a83e0 ConCommand nav_place_floodfill
server.dll!0x009a8374 ConCommand nav_place_list
server.dll!0x009a8428 ConCommand nav_place_pick
server.dll!0x009a8350 ConCommand nav_place_replace
server.dll!0x009a8404 ConCommand nav_place_set
server.dll!0x009a8134 ConCommand nav_precise
server.dll!0x009a7d68 ConCommand nav_raise_drag_volume_max
server.dll!0x009a7db0 ConCommand nav_raise_drag_volume_min
server.dll!0x009a7bb8 ConCommand nav_recall_selected_set
server.dll!0x009a7c24 ConCommand nav_remove_from_selected_set
server.dll!0x009a7ae0 ConCommand nav_remove_jump_areas
server.dll!0x009a81e8 ConCommand nav_run
server.dll!0x009a82e4 ConCommand nav_save
server.dll!0x009a7938 ConCommand nav_save_selected
server.dll!0x009a7f18 ConCommand nav_select_blocked_areas
server.dll!0x009a7f60 ConCommand nav_select_damaging_areas
server.dll!0x009a7e88 ConCommand nav_select_half_space
server.dll!0x009a7ef4 ConCommand nav_select_invalid_areas
server.dll!0x009a7f3c ConCommand nav_select_obstructed_areas
server.dll!0x009a713c ConCommand nav_select_overlapping
server.dll!0x009a7510 ConCommand nav_select_radius
server.dll!0x009a7f84 ConCommand nav_select_stairs
server.dll!0x009a83bc ConCommand nav_set_place_mode
server.dll!0x009a74c8 ConCommand nav_shift
server.dll!0x009a89e4 ConCommand nav_simplify_selected
server.dll!0x009a80ec ConCommand nav_splice
server.dll!0x009a7fa8 ConCommand nav_split
server.dll!0x009a8278 ConCommand nav_stand
server.dll!0x009a81a0 ConCommand nav_stop
server.dll!0x009a7b94 ConCommand nav_store_selected_set
server.dll!0x009a82c0 ConCommand nav_strip
server.dll!0x009a78f0 ConCommand nav_subdivide
server.dll!0x009a78cc ConCommand nav_test_stairs
server.dll!0x009a7e64 ConCommand nav_toggle_deselecting
server.dll!0x009a7c48 ConCommand nav_toggle_in_selected_set
server.dll!0x009a8398 ConCommand nav_toggle_place_mode
server.dll!0x009a844c ConCommand nav_toggle_place_painting
server.dll!0x009a7b70 ConCommand nav_toggle_selected_set
server.dll!0x009a7df8 ConCommand nav_toggle_selecting
server.dll!0x009a8230 ConCommand nav_transient
server.dll!0x009a8038 ConCommand nav_unmark
server.dll!0x009a70f4 ConCommand nav_update_blocked
server.dll!0x009a70d0 ConCommand nav_update_lighting
server.dll!0x009a832c ConCommand nav_use_place
server.dll!0x009a81c4 ConCommand nav_walk
server.dll!0x009a8524 ConCommand nav_warp_to_mark
server.dll!0x009a74ec ConCommand nav_world_center
server.dll!0x00986a60 ConCommand noclip
server.dll!0x00986bc8 ConCommand notarget
server.dll!0x00983d48 ConCommand npc_ammo_deplete
server.dll!0x0098385c ConCommand npc_bipass
server.dll!0x00983be0 ConCommand npc_combat
server.dll!0x00983c70 ConCommand npc_conditions
server.dll!0x00983968 ConCommand npc_create
server.dll!0x0098398c ConCommand npc_create_aimed
server.dll!0x00983880 ConCommand npc_destroy
server.dll!0x009839b0 ConCommand npc_destroy_unselected
server.dll!0x009838c8 ConCommand npc_enemies
server.dll!0x009838ec ConCommand npc_focus
server.dll!0x009839d4 ConCommand npc_freeze
server.dll!0x00983a1c ConCommand npc_freeze_unselected
server.dll!0x00983b08 ConCommand npc_go
server.dll!0x00983b2c ConCommand npc_go_random
server.dll!0x00983d24 ConCommand npc_heal
server.dll!0x009838a4 ConCommand npc_kill
server.dll!0x00983b74 ConCommand npc_nearest
server.dll!0x00983cb8 ConCommand npc_relationships
server.dll!0x00983b50 ConCommand npc_reset
server.dll!0x00983b98 ConCommand npc_route
server.dll!0x00983bbc ConCommand npc_select
server.dll!0x009839f8 ConCommand npc_set_freeze
server.dll!0x00983a40 ConCommand npc_set_freeze_unselected
server.dll!0x00983c04 ConCommand npc_squads
server.dll!0x00983cdc ConCommand npc_steering
server.dll!0x00983d00 ConCommand npc_steering_all
server.dll!0x00983c4c ConCommand npc_task_text
server.dll!0x00983c28 ConCommand npc_tasks
server.dll!0x00983a88 ConCommand npc_teleport
server.dll!0x00983a64 ConCommand npc_thinknow
server.dll!0x00983c94 ConCommand npc_viewcone
server.dll!0x0099ea94 ConCommand observer_use
server.dll!0x00986c10 ConCommand parachute
server.dll!0x0098ef98 ConCommand particle_test_start
server.dll!0x0098efbc ConCommand particle_test_stop
server.dll!0x0098f2f0 ConCommand physics_budget
server.dll!0x0098f284 ConCommand physics_constraints
server.dll!0x0098f2a8 ConCommand physics_debug_entity
server.dll!0x0098f218 ConCommand physics_highlight_active
server.dll!0x0098f23c ConCommand physics_report_active
server.dll!0x0098f2cc ConCommand physics_select
server.dll!0x00985c14 ConCommand picker
server.dll!0x00995220 ConCommand print_mapgroup_sv
server.dll!0x00990eb0 ConCommand prop_debug
server.dll!0x00990fd4 ConCommand prop_dynamic_create
server.dll!0x00990ff8 ConCommand prop_physics_create
server.dll!0x0099ea4c ConCommand replay_death
server.dll!0x0099ea28 ConCommand replay_start
server.dll!0x0099ea70 ConCommand replay_stop
server.dll!0x0098b260 ConCommand report_entities
server.dll!0x0098b2a8 ConCommand report_simthinklist
server.dll!0x009921c8 ConCommand report_soundpatch
server.dll!0x0098b284 ConCommand report_touchlinks
server.dll!0x0099ac7c ConCommand reset_expo
server.dll!0x0098b23c ConCommand respawn_entities
server.dll!0x00983eb0 ConCommand rr_forceconcept
server.dll!0x009849ec ConCommand rr_reloadresponsesystems
server.dll!0x0098692c ConCommand say
server.dll!0x00986950 ConCommand say_team
server.dll!0x00991708 ConCommand scene_flush
server.dll!0x009916c0 ConCommand scene_playvcd
server.dll!0x00993bd0 ConCommand script
server.dll!0x00993c18 ConCommand script_debug
server.dll!0x00993c60 ConCommand script_dump_all
server.dll!0x00993bf4 ConCommand script_execute
server.dll!0x00993c3c ConCommand script_help
server.dll!0x00993b60 ConCommand script_reload_code
server.dll!0x00993b84 ConCommand script_reload_entity_code
server.dll!0x00993ba8 ConCommand script_reload_think
server.dll!0x00995b48 ConCommand send_round_backup_file_list
server.dll!0x0098d784 ConCommand server_game_time
server.dll!0x00986b5c ConCommand setang
server.dll!0x00986ba4 ConCommand setang_exact
server.dll!0x00986998 ConCommand setmodel
server.dll!0x00986b14 ConCommand setpos
server.dll!0x00986b80 ConCommand setpos_exact
server.dll!0x00986b38 ConCommand setpos_player
server.dll!0x0098b58c ConCommand shake
server.dll!0x00992ed0 ConCommand showtriggers_toggle
server.dll!0x0098e858 ConCommand skip_next_map
server.dll!0x009922a8 ConCommand soundscape_flush
server.dll!0x0098f260 ConCommand surfaceprop
server.dll!0x00987040 ConCommand survival_check_num_possible_final_zone
server.dll!0x00991dd0 ConCommand sv_benchmark_force_start
server.dll!0x0098d834 ConCommand sv_clearhinthistory
server.dll!0x009a6878 ConCommand sv_cs_dump_econ_item_stringtable
server.dll!0x009a4c0c ConCommand sv_dz_reset_danger_zone
server.dll!0x00992128 ConCommand sv_findsoundname
server.dll!0x009a212c ConCommand sv_game_mode_convars
server.dll!0x00994214 ConCommand sv_load_forced_client_names_file
server.dll!0x00994294 ConCommand sv_load_random_client_names_file
server.dll!0x00991358 ConCommand sv_querycache_stats
server.dll!0x009920e0 ConCommand sv_soundemitter_filecheck
server.dll!0x009920bc ConCommand sv_soundemitter_flush
server.dll!0x00992104 ConCommand sv_soundemitter_reload
server.dll!0x0099214c ConCommand sv_soundemitter_spew
server.dll!0x009922cc ConCommand sv_soundscape_printdebuginfo
server.dll!0x009869bc ConCommand test_dispatcheffect
server.dll!0x00987158 ConCommand test_entity_blocker
server.dll!0x0098b568 ConCommand test_outtro_stats
server.dll!0x0099e9e0 ConCommand timeleft
server.dll!0x00997444 ConCommand timeout_ct_start
server.dll!0x00997420 ConCommand timeout_terrorist_start
server.dll!0x0099e8e0 ConCommand traceattack
server.dll!0x0099ac58 ConCommand tv_time_remaining
server.dll!0x0099aca0 ConCommand tweak_ammo_impulses
server.dll!0x009869e0 ConCommand use
server.dll!0x009933e0 ConCommand vehicle_flushscript
server.dll!0x0099310c ConCommand voxeltree_box
server.dll!0x009930e8 ConCommand voxeltree_playerview
server.dll!0x00993130 ConCommand voxeltree_sphere
server.dll!0x009930c4 ConCommand voxeltree_view
server.dll!0x00993d3c ConCommand wc_air_edit_further
server.dll!0x00993d60 ConCommand wc_air_edit_nearer
server.dll!0x00993d18 ConCommand wc_air_node_edit
server.dll!0x00993cac ConCommand wc_create
server.dll!0x00993cd0 ConCommand wc_destroy
server.dll!0x00993cf4 ConCommand wc_destroy_undo
server.dll!0x00993d84 ConCommand wc_link_edit
server.dll!0x009a868c ConCommand wipe_nav_attributes
server.dll!0x009a1b08 ConCommand workshop_start_map
```

# Interfaces

## Interfaces

```
inputsystem.dll!0x0002ed28 InputStackSystemVersion001
inputsystem.dll!0x00031188 InputSystemVersion001
```

## Interfaces

```
materialsystemd.dll!0x000bae10 COLORCORRECTION_VERSION_1
materialsystemd.dll!0x000bcafc ShaderSystem002
materialsystemd.dll!0x000df420 VMaterialSystem080
materialsystemd.dll!0x000bc468 VMaterialSystemConfig004
materialsystemd.dll!0x000bc960 VMaterialSystemStub001
```

## Interfaces

```
shaderapidx9.dll!0x000a5a90 DebugTextureInfo001
shaderapidx9.dll!0x00089488 MaterialSystemHardwareConfig013
shaderapidx9.dll!0x000a5a8c ShaderApi029
shaderapidx9.dll!0x000a5830 ShaderDevice001
shaderapidx9.dll!0x0008a7e8 ShaderDeviceMgr001
shaderapidx9.dll!0x0008a9a8 ShaderShadow010
shaderapidx9.dll!0x00089304 VBAllocTracker001
```

## Interfaces

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

## Interfaces

```
vguimatsurface.dll!0x000f47c4 SchemeSurface001
vguimatsurface.dll!0x000f47c0 VGUI_Surface031
```

## Interfaces

```
vphysics.dll!0x0010aa20 VPhysics031
vphysics.dll!0x0010aa5c VPhysicsCollision007
vphysics.dll!0x00116018 VPhysicsSurfaceProps001
```

## Interfaces

```
vstdlib.dll!0x00038a10 EventSystem001
vstdlib.dll!0x0003c270 VEngineCvar007
vstdlib.dll!0x0003e608 VProcessUtils002
```

## Interfaces

```
matchmaking.dll!0x000831e0 MATCHFRAMEWORK_001
matchmaking.dll!0x00096378 VENGINE_GAMETYPES_VERSION002
```

