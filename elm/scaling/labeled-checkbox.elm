import Html exposing (Html, fieldset, label, input, text)
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
  { emailNotifications : Bool
  , videoAutoplay : Bool
  , useLocation : Bool
  }


type Msg
  = ToggleEmailNotifications
  | ToggleVideoAutoplay
  | ToggleUseLocation


init : (Model, Cmd Msg)
init =
  (Model True True True, Cmd.none)


update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case msg of
    ToggleEmailNotifications ->
      ({ model | emailNotifications = not model.emailNotifications }, Cmd.none)

    ToggleVideoAutoplay ->
      ({ model | videoAutoplay = not model.videoAutoplay }, Cmd.none)

    ToggleUseLocation ->
      ({ model | useLocation = not model.useLocation }, Cmd.none)


subscriptions : Model -> Sub Msg
subscriptions model =
  Sub.none


view : Model -> Html Msg
view model =
  fieldset []
    [ checkbox ToggleEmailNotifications "Email Notifications"
    , checkbox ToggleVideoAutoplay "Video Autoplay"
    , checkbox ToggleUseLocation "Use Location"
    ]


checkbox : Msg -> String -> Html Msg
checkbox msg name =
  label []
    [ input [ type_ "checkbox", onClick msg ] []
    , text name
    ]
