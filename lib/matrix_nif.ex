defmodule MatrixNif do
  use Rustler, otp_app: :matrix, crate: :matrix_nif

  def transpose(_m), do: exit(:nif_not_loaded)
end