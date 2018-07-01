import Html exposing (Html, div, input, button, text)
import Html.Attributes exposing (..)
import Html.Events exposing (onInput, onClick)
import WebSocket


main =
  Html.program
    { init = init
    , update = update
    , view = view
    , subscriptions = subscriptions
    }


type alias Model =
  { input : String
  , messages : List String
  }


type Msg
  = Input String
  | Send
  | NewMessage String


init : (Model, Cmd Msg)
init =
  (Model "" [], Cmd.none)


update : Msg -> Model -> (Model, Cmd Msg)
update msg {input, messages} =
  case msg of
    Input newInput ->
      (Model newInput messages, Cmd.none)

    Send ->
      (Model "" messages, WebSocket.send "ws://echo.websocket.org" input)

    NewMessage msg ->
      (Model input (msg :: messages), Cmd.none)


subscriptions : Model -> Sub Msg
subscriptions model =
  WebSocket.listen "ws://echo.websocket.org" NewMessage


view : Model -> Html Msg
view model =
  div []
    [ div [] (List.map viewMessage model.messages)
    , input [ onInput Input, value model.input, placeholder "Type to send" ] []
    , button [ onClick Send ] [ text "Send" ]
    ]


viewMessage : String -> Html Msg
viewMessage msg =
  div [] [ text msg ]
