defmodule TestRustNifTest do
  use ExUnit.Case
  doctest TestRustNif

  test "greets the world" do
    assert TestRustNif.hello() == :world
  end
end
