SECTION "gfx_how_to_play", ROMX[$4000], BANK[$56]
gfx_how_to_play::
INCBIN "gfx/how_to_play/how_to_play.bin"
.end::

SECTION "gfx_how_to_play_sgb", ROMX[$7800], BANK[$56]
gfx_how_to_play_sgb::
INCBIN "gfx/how_to_play/oak_mouth_sgb.bin"
INCBIN "gfx/how_to_play/how_to_play_sgb.bin"
.end::
