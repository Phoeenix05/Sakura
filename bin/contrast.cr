require "luminance"

hex = ARGV[0]
lumi = Luminance.luminance hex
bg_colors = {"121212", "161616", "1A1A1A"}

bg_colors.each { |c| puts(lumi / Luminance.luminance c) }
