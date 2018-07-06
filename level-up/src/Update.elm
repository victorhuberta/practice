module Update exposing (..)


import Models exposing (Model, Player)
import Msgs exposing (Msg(..))


update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case msg of
    NoOp ->
      ({ players = [ Player "" "" 1 ] }, Cmd.none)
