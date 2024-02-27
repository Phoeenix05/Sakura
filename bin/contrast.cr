# https://coolors.co/121212-161616-1a1a1a-ececec-f9f9f9-e3f7de-cdf4c2-a1ef8b
# "121212"
# "161616"
# "1A1A1A"
# "ECECEC"
# "F9F9F9"
# "E3F7DE"
# "CDF4C2"
# "A1EF8B"

def hex_to_rgb(hex : String) : {Int32, Int32, Int32}
  b16 = {
    '0' => 0,
    '1' => 1,
    '2' => 2,
    '3' => 3,
    '4' => 4,
    '5' => 5,
    '6' => 6,
    '7' => 7,
    '8' => 8,
    '9' => 9,
    'A' => 10,
    'B' => 11,
    'C' => 12,
    'D' => 13,
    'E' => 14,
    'F' => 15,
  } of Char => Int32

  chars = hex.each_char
  r, g, b = chars
    .each_slice(2)                         # iter slice
    .map { |(a, b)| b16[a] * 16 + b16[b] } # map ['A', 'B'] => Int32
    .to_a                                  # convert to array

  return {r, g, b}
end

def srgb_lin(v : Float) : Float
  if v <= 0.04045
    return v / 12.92
  else
    return ((v + 0.055)/1.055)**2.4
  end
end

def y_lstar(y : Float) : Float
  if y <= 216/24389
    return y * (24389/27)
  else
    return (y**(1/3)) * 116 - 16;
  end
end

# Y values for luminance R 0.2126, G 0.7152, B 0.0722
def luminance(sR : Int32, sG : Int32, sB : Int32) : Float
  vR = sR / 255
  vG = sG / 255
  vB = sB / 255
  y = 0.2126 * srgb_lin(vR) + 0.7152 * srgb_lin(vG) + 0.0722 * srgb_lin(vB)
  return y_lstar y
end

# :ditto:
def luminance(hex : String) : Float
  r, g, b = hex_to_rgb hex
  return luminance(r, g, b)
end

hex = ARGV[0]
lumi = luminance hex
bg_colors = {"121212", "161616", "1A1A1A"}

bg_colors.each { |c| puts(lumi / luminance c) }
