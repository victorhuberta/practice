module Players.List exposing (..)


import Html exposing (..)
import Html.Attributes exposing (class)
import Models exposing (Player)
import Msgs exposing (Msg)


view : List Player -> Html Msg
view players =
  div []
    [ header
    , list players
    ]


header : Html Msg
header =
  div [ class "clearfix mb2 white bg-black" ]
    [ div [ class "left p2" ] [ text "Players" ] ]


list : List Player -> Html Msg
list players =
  div [ class "p2" ]
    [ table []
        [ thead []
            [ tr []
                [ th [] [ text "Id" ]
                , th [] [ text "Name" ]
                , th [] [ text "Level" ]
                , th [] [ text "Actions" ]
                ]
            ]
        , tbody [] (List.map playerRow players)
        ]
    ]


playerRow : Player -> Html Msg
playerRow player =
  tr []
    [ td [] [ text player.id ]
    , td [] [ text player.name ]
    , td [] [ text (toString player.level) ]
    , td [] []
    ]
