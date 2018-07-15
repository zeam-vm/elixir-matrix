defmodule MatrixNif do
  use Rustler, otp_app: :matrix, crate: :matrix_nif

  def add(_a, _b), do: exit(:nif_not_loaded)
end