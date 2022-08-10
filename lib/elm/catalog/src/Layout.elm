module Layout exposing (..)

import Element exposing (..)
import Element.Background as Background
import Element.Border as Border
import Element.Font as Font
import Element.Input
import Html exposing (Html)

defaultPadding : Attribute msg
defaultPadding =
    padding 20


defaultSpacing : Attribute msg
defaultSpacing =
    spacing 20


type alias Model msg =
    { title : String
    , showWorkspace : Bool
    , content : Element msg
    , workspace : Element msg
    , actions : Element msg
    }


type alias Command msg =
    { onPress : msg, label : Element msg }


view : Model msg -> Html msg
view model =
    Element.layout [] <|
        column [ width fill, height fill, defaultSpacing ]
            [ text "placeholde"
            ]