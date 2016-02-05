require 'barby'
require 'barby/barcode/code_128'
require 'barby/outputter/ascii_outputter'

barcode = Barby::Code128B.new('BARBY')

puts barcode.to_ascii #Implicitly uses the AsciiOutputter
