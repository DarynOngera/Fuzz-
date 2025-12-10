defmodule NetworkFuzzer do 
  use ExUnitProperties 

  def packet_gen do 
    gen all magic <- constant(<<0xCA, 0xFE>>),
      version <- integer(0..25),
            typ <- integer(0..25),
            length <- integer(0..2000),
            payload <- binary(length: length) do 
    <<magic::binary, version::8, typ::8, length::32-big, payload::binary>>
    end
  end

  def run_fuzz(count \\ 100)do 
    packet_gen()
    |> Enum.take(count)
    |> Enum.each(fn packet ->
      File.write!("fuzz_input.sh", packet)
      {output, code} = System.cmd("cargo", ["run", "--quiet", "--manifest-path", "../network_target/Cargo.toml", "--bin", "main", "fuzz_input.sh"])
      if code != 0 do 
        IO.puts("Crash found with input: #{inspect(packet, base: :hex)}")
      else
        IO.puts("OK: #{output}")
      end
    end)
  end
end
