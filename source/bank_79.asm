SECTION "gfx_cancel_ok_no_yes", ROMX[$4000], BANK[$79]
gfx_cancel_ok_no_yes::
INCBIN "gfx/menus/cancel_ok_no_yes.2bpp"
.end::

SECTION "compressed_tilemap_attrmap_town_map", ROMX[$5800], BANK[$79]
compressed_tilemap_attrmap_town_map::
INCBIN "gfx/town_map/town_map.tilemap_attrmap.xor"
.end::

SECTION "compressed_gfx_game_select", ROMX[$6ba1], BANK[$79]
compressed_gfx_game_select::
INCBIN "gfx/game_select/game_select.2bpp.xor"
.end::
