module Update exposing (..)


import RemoteData
import Models exposing (Model, Player)
import Msgs exposing (Msg(..))
import Routing exposing (parseLocation)
import Commands exposing (savePlayer)


update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case msg of
    OnFetchPlayers response ->
      ({ model | players = response }, Cmd.none)

    OnLocationChange location ->
      let
        newRoute =
          parseLocation location
      in
        ({ model | route = newRoute }, Cmd.none)

    ChangeLevel player amount ->
      let
        updatedPlayer =
          { player | level = player.level + amount }
      in
        (model, savePlayer updatedPlayer)

    OnPlayerSave (Ok player) ->
      (updatePlayer model player, Cmd.none)

    OnPlayerSave (Err error) ->
      (model, Cmd.none)


updatePlayer : Model -> Player -> Model
updatePlayer model updatedPlayer =
  let
    pick currentPlayer =
      if updatedPlayer.id == currentPlayer.id then
        updatedPlayer
      else
        currentPlayer

    updatePlayerList players =
      List.map pick players

    updatedPlayers =
      RemoteData.map updatePlayerList model.players
  in
    { model | players = updatedPlayers }
