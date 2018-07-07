module View exposing (..)


import Html exposing (Html, div, text)
import RemoteData
import Models exposing (Model, PlayerId)
import Msgs exposing (Msg)
import Players.List
import Players.Edit


view : Model -> Html Msg
view model =
  div []
    [ page model ]


page : Model -> Html Msg
page model =
  case model.route of
    Models.PlayersRoute ->
      Players.List.view model.players

    Models.PlayerRoute playerId ->
      playerEditPage model playerId

    Models.NotFoundRoute ->
      notFoundPage


playerEditPage : Model -> PlayerId -> Html Msg
playerEditPage model playerId =
  case model.players of
    RemoteData.NotAsked ->
      text ""

    RemoteData.Loading ->
      text "Loading..."

    RemoteData.Success players ->
      let
        maybePlayer =
          players
            |> List.filter (\player -> player.id == playerId)
            |> List.head
      in
        case maybePlayer of
          Just player ->
            Players.Edit.view player

          Nothing ->
            notFoundPage

    RemoteData.Failure error ->
      text (toString error)


notFoundPage : Html msg
notFoundPage =
  div []
    [ text "Not Found"
    ]
