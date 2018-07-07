module Players.Edit exposing (..)


import Html exposing (..)
import Html.Attributes exposing (class, href)
import Html.Events exposing (onClick)
import Models exposing (Player)
import Msgs exposing (Msg)
import Routing


view : Player -> Html Msg
view player =
  div []
    [ header player
    , form player
    ]


header : Player -> Html Msg
header player =
  div [ class "clearfix mb2 white bg-black p1" ]
    [ listBtn ]


listBtn : Html Msg
listBtn =
  a [ class "btn regular"
    , href Routing.playersPath
    ]
    [ i [ class "fa fa-chevron-left mr1" ] []
    , text "List"
    ]


form : Player -> Html Msg
form player =
  div [ class "m3" ]
    [ h1 [] [ text player.name ]
    , formLevel player
    ]


formLevel : Player -> Html Msg
formLevel player =
  div [ class "clearfix py1" ]
   [ div [ class "col col-5" ] [ text "Level" ]
   , div [ class "col col-7" ]
     [ span [ class "h2 bold" ] [ text (toString player.level) ]
     , btnLevelDecrease player
     , btnLevelIncrease player
     ]
   ]


btnLevelDecrease : Player -> Html Msg
btnLevelDecrease player =
  a [ class "btn ml1 h1", onClick (Msgs.ChangeLevel player -1) ]
    [ i [ class "fa fa-minus-circle" ] [] ]


btnLevelIncrease : Player -> Html Msg
btnLevelIncrease player =
  a [ class "btn ml1 h1", onClick (Msgs.ChangeLevel player 1) ]
    [ i [ class "fa fa-plus-circle" ] [] ]
