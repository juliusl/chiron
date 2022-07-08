module Layout exposing (Model, view)

import Element exposing (..)
import Element.Background as Background
import Html exposing (Html)


type alias Model msg =
    { title : String
    , content : Element msg
    , detail : Element msg
    }


view : Model msg -> Html msg
view model =
    Element.layout [] <|
        column [ width fill, height fill, defaultSpacing ]
            [ viewHeader model
            , viewPage model
            ]


viewHeader : Model msg -> Element msg
viewHeader model =
    row
        [ width fill
        , defaultPadding
        , height (px 80)
        , Background.color (rgb255 0xA5 0xA5 0x8D)
        ]
        [ text model.title ]


viewPage : Model msg -> Element msg
viewPage model =
    row [ width fill, height fill, defaultSpacing, defaultPadding ]
        [ viewContent model
        , viewDetail model
        ]


viewContent : Model msg -> Element msg
viewContent model =
    column
        [ width (fillPortion 2)
        , height fill
        , defaultPadding
        ]
        [ model.content ]


viewDetail : Model msg -> Element msg
viewDetail model =
    column
        [ width (fillPortion 1)
        , height fill
        , defaultPadding
        ]
        [ model.detail ]


defaultPadding : Attribute msg
defaultPadding =
    padding 20


defaultSpacing : Attribute msg
defaultSpacing =
    spacing 20
