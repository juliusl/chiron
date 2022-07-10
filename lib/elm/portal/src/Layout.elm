module Layout exposing (Model, view)

import Element exposing (..)
import Element.Background as Background
import Html exposing (Html)

type alias Model msg =
    { title : String
    , content : Element msg
    , left_detail : Element msg
    , right_detail: Element msg
    }

view : Model msg -> Html msg
view model =
    Element.layout [] <|
        column [ width fill, height fill, defaultSpacing ]
            [ viewHeader model
            , viewPage model
            , viewFooter model
            ]


viewHeader : Model msg -> Element msg
viewHeader model =
    row
        [ width fill
        , defaultPadding
        , height (px 80)
        , Background.color (rgb255 0x95 0xA5 0x8D)
        ]
        [ text model.title ]


viewPage : Model msg -> Element msg
viewPage model =
    row [ width fill, height fill, defaultSpacing, defaultPadding ]
        [ viewDetail model
        , viewContent model
        , viewGutter model
        ]
viewFooter : Model msg -> Element msg
viewFooter _ =
    row
        [ width fill
        , defaultPadding
        , height (px 100)
        , Background.color (rgb255 0x95 0xA5 0x8D)
        ]
        []

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
        [ model.left_detail ]

viewGutter : Model msg -> Element msg
viewGutter model =
    column
        [ width (fillPortion 1)
        , height fill
        , defaultPadding
        ]
        [ model.right_detail ]


defaultPadding : Attribute msg
defaultPadding =
    padding 20


defaultSpacing : Attribute msg
defaultSpacing =
    spacing 20
