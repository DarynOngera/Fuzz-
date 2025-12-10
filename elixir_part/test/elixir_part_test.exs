defmodule ElixirPartTest do
  use ExUnit.Case
  doctest ElixirPart

  test "greets the world" do
    assert ElixirPart.hello() == :world
  end
end
