import Html exposing (Html, div, section, label, input, text)
import Html.Attributes exposing (..)
import Html.Events exposing (onClick)


main : Program Never Model Msg
main =
  Html.program
    { init = init
    , update = update
    , view = view
    , subscriptions = subscriptions
    }


type alias Model =
  { fontSize : FontSize
  , content : String
  }

type FontSize = Small | Medium | Large


type Msg = SwitchTo FontSize


init : (Model, Cmd Msg)
init =
  (Model Medium "Hello, world!", Cmd.none)


update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case msg of
    SwitchTo fontSize ->
      ({ model | fontSize = fontSize }, Cmd.none)


subscriptions : Model -> Sub Msg
subscriptions model =
  Sub.none


view : Model -> Html Msg
view model =
  div []
    [ radio (SwitchTo Small) "Small"
    , radio (SwitchTo Medium) "Medium"
    , radio (SwitchTo Large) "Large"
    , section [] [ text model.content ]
    ]


radio : Msg -> String -> Html Msg
radio msg name =
  label []
    [ input [ type_ "radio", onClick msg ] []
    , text name
    ]
