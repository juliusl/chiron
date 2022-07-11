module Markdown exposing (viewMarkdown)

import Element exposing (..)
import Element.Background as Background
import Element.Border as Border
import Element.Font as Font
import Element.HexColor exposing (rgbCSSHex)
import Element.Input as Input exposing (button)
import Element.Region as Region
import Html
import Html.Attributes
import Markdown.Block as Block exposing (ListItem(..), Task(..), headingLevelToInt)
import Markdown.Html
import Markdown.Parser
import Markdown.Renderer exposing (..)


viewMarkdown : (String -> msg) -> String -> Result String (List (Element msg))
viewMarkdown onRunmd markdown =
    markdown
        |> Markdown.Parser.parse
        |> Result.mapError (\error -> error |> List.map Markdown.Parser.deadEndToString |> String.join "\n")
        |> Result.andThen (Markdown.Renderer.render (renderer onRunmd))


renderer : (String -> msg) -> Markdown.Renderer.Renderer (Element msg)
renderer onRunmd =
    defaultRenderer onRunmd



-- Custom Renderers (TODO)
-- Default markdown renderer to elm-ui types


defaultRenderer : (String -> msg) -> Markdown.Renderer.Renderer (Element msg)
defaultRenderer onRunmd =
    { heading = heading
    , paragraph =
        Element.paragraph
            [ Element.spacing 15 ]
    , thematicBreak = Element.none
    , text = Element.text
    , strong = \content -> Element.row [ Font.bold ] content
    , emphasis = \content -> Element.row [ Font.italic ] content
    , strikethrough = \content -> Element.row [ Font.strike ] content
    , link =
        \{ destination } body ->
            Element.newTabLink
                [ Element.htmlAttribute (Html.Attributes.style "display" "inline-flex") ]
                { url = destination
                , label =
                    Element.paragraph
                        [ Font.color (Element.rgb255 0 0 255)
                        ]
                        body
                }
    , hardLineBreak = Html.br [] [] |> Element.html
    , image =
        \image ->
            case image.title of
                Just _ ->
                    Element.image [ Element.width Element.fill ] { src = image.src, description = image.alt }

                Nothing ->
                    Element.image [ Element.width Element.fill ] { src = image.src, description = image.alt }
    , blockQuote =
        \children ->
            Element.column
                [ Border.widthEach { top = 0, right = 0, bottom = 0, left = 10 }
                , padding 10
                , Border.color (Element.rgb255 145 145 145)
                , Background.color (Element.rgb255 245 245 245)
                ]
                children
    , unorderedList =
        \items ->
            Element.column
                [ Element.spacing 15
                , Font.size 16
                ]
                (items
                    |> List.map
                        (\(ListItem task children) ->
                            Element.paragraph [ Element.spacing 5 ]
                                [ Element.row
                                    [ Element.alignTop ]
                                    ((case task of
                                        IncompleteTask ->
                                            Input.defaultCheckbox False

                                        CompletedTask ->
                                            Input.defaultCheckbox True

                                        NoTask ->
                                            Element.text "-"
                                     )
                                        :: Element.text " "
                                        :: children
                                    )
                                ]
                        )
                )
    , orderedList =
        \startingIndex items ->
            Element.column [ Element.spacing 15 ]
                (items
                    |> List.indexedMap
                        (\index itemBlocks ->
                            Element.row [ Element.spacing 5 ]
                                [ Element.row [ Element.alignTop ]
                                    (Element.text (String.fromInt (index + startingIndex) ++ " ") :: itemBlocks)
                                ]
                        )
                )
    , html = Markdown.Html.oneOf []
    , table = Element.column []
    , tableHeader = Element.column []
    , tableBody = Element.column []
    , tableRow = Element.row []
    , tableHeaderCell =
        \_ children ->
            Element.paragraph [] children
    , tableCell =
        \_ children ->
            Element.paragraph [] children
    , codeBlock = codeBlock onRunmd
    , codeSpan = code
    }


code : String -> Element msg
code snippet =
    Element.el
        [ Background.color
            (Element.rgba 0 0 0 0.04)
        , Border.rounded 2
        , Element.paddingXY 5 3
        , Font.family
            [ Font.monospace ]
        ]
        (Element.text snippet)


codeBlock : (String -> msg) -> { body : String, language : Maybe String } -> Element msg
codeBlock onRunmd details =
    parseCodeBlock onRunmd details


formatCodeBlock : Element msg -> Element msg
formatCodeBlock content =
    Element.el
        [ Background.color (Element.rgba 0 0 0 0.03)
        , Element.htmlAttribute (Html.Attributes.style "white-space" "pre")
        , Element.padding 10
        , Font.family
            [ Font.monospace ]
        , Font.size 14
        ]
        content


parseCodeBlock : (String -> msg) -> { body : String, language : Maybe String } -> Element msg
parseCodeBlock onRunmd details =
    case details.language of
        Just lang ->
            case lang of
                "yaml" ->
                    formatCodeBlock (Element.text "(TODO) Render a form from this yaml block")

                "markdown" ->
                    case viewMarkdown onRunmd details.body of
                        Ok rendered ->
                            Element.column [ Element.spacing 15 ] rendered

                        Err err ->
                            formatCodeBlock (Element.text err)

                "runmd" ->
                    formatCodeBlock
                        (Element.row
                            [ Element.onLeft
                                (button
                                    [ Font.size 16
                                    , Element.moveLeft 20.0
                                    , padding 8
                                    ]
                                    { onPress = Just (onRunmd details.body), label = Element.text "Dispatch to chiron" }
                                )
                            ]
                            [ Element.text details.body ]
                        )

                _ ->
                    formatCodeBlock (Element.text details.body)

        Nothing ->
            formatCodeBlock (Element.text details.body)


heading : { level : Block.HeadingLevel, rawText : String, children : List (Element msg) } -> Element msg
heading { level, rawText, children } =
    Element.paragraph
        (case level of
            Block.H1 ->
                headingOne (headingLevelToInt level) rawText

            Block.H2 ->
                headingTwo (headingLevelToInt level) rawText

            _ ->
                [ Font.size 14
                , Font.family [ Font.typeface "system-ui" ]
                , Region.heading (headingLevelToInt level)
                , Element.htmlAttribute
                    (Html.Attributes.attribute "name" (rawTextToId rawText))
                , Element.htmlAttribute
                    (Html.Attributes.id (rawTextToId rawText))
                ]
        )
        children


headingOne : Int -> String -> List (Attribute msg)
headingOne level rawText =
    [ Font.size 20
    , Font.bold
    , Font.family [ Font.typeface "system-ui" ]
    , Region.heading level
    , Element.htmlAttribute
        (Html.Attributes.attribute "name" (rawTextToId rawText))
    , Element.htmlAttribute
        (Html.Attributes.id (rawTextToId rawText))
    ]


headingTwo : Int -> String -> List (Attribute msg)
headingTwo level rawText =
    [ Font.size 18
    , Font.bold
    , Font.underline
    , Font.family [ Font.typeface "system-ui" ]
    , Region.heading level
    , Element.htmlAttribute
        (Html.Attributes.attribute "name" (rawTextToId rawText))
    , Element.htmlAttribute
        (Html.Attributes.id (rawTextToId rawText))
    ]


rawTextToId : String -> String
rawTextToId rawText =
    rawText
        |> String.split " "
        |> String.join "-"
        |> String.toLower
