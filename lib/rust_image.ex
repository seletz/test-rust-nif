defmodule TestRustNif.RustImage do
  use Rustler, otp_app: :test_rust_nif, crate: "rust_image"

  def add(_a, _b), do: :erlang.nif_error(:nif_not_loaded)

end
