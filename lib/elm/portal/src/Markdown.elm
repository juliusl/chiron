module Markdown exposing (viewMarkdown)

import Element exposing (..)
import Element.Background as Background
import Element.Border as Border
import Element.Font as Font
import Element.Input as Input
import Element.Region as Region
import Html
import Html.Attributes
import Markdown.Block as Block exposing (ListItem(..), Task(..))
import Markdown.Html
import Markdown.Parser
import Markdown.Renderer exposing (..)
import Markdown.Block exposing (headingLevelToInt)


viewMarkdown : String -> Result String (List (Element msg))
viewMarkdown markdown =
    markdown
        |> Markdown.Parser.parse
        |> Result.mapError (\error -> error |> List.map Markdown.Parser.deadEndToString |> String.join "\n")
        |> Result.andThen (Markdown.Renderer.render renderer)


renderer : Markdown.Renderer.Renderer (Element msg)
renderer =
    defaultRenderer



-- Custom Renderers (TODO)
-- Default markdown renderer to elm-ui types


defaultRenderer : Markdown.Renderer.Renderer (Element msg)
defaultRenderer =
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
                                [Element.row
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
    , codeBlock = codeBlock
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
            [ Font.external
                { url = "https://fonts.googleapis.com/css?family=Source+Code+Pro"
                , name = "Source Code Pro"
                }
            ]
        ]
        (Element.text snippet)


codeBlock : { body : String, language : Maybe String } -> Element msg
codeBlock details =
    parseCodeBlock details


formatCodeBlock : Element msg -> Element msg
formatCodeBlock content =
    Element.el
        [ Background.color (Element.rgba 0 0 0 0.03)
        , Element.htmlAttribute (Html.Attributes.style "white-space" "pre")
        , Element.padding 10
        , Font.family
            [ Font.external
                { url = "https://fonts.googleapis.com/css?family=Source+Code+Pro"
                , name = "Source Code Pro"
                }
            ]
        , Font.size 14
        ]
        content


parseCodeBlock : { body : String, language : Maybe String } -> Element msg
parseCodeBlock details =
    case details.language of
        Just lang ->
            case lang of
                "yaml" ->
                    formatCodeBlock (Element.text "(TODO) Render a form from this yaml block")

                "markdown" ->
                    case viewMarkdown details.body of
                        Ok rendered ->
                            Element.column [ Element.spacing 15 ] rendered

                        Err err ->
                            formatCodeBlock (Element.text err)

                _ ->
                    formatCodeBlock (Element.text details.body)

        Nothing ->
            formatCodeBlock (Element.text details.body)


heading : { level : Block.HeadingLevel, rawText : String, children : List (Element msg) } -> Element msg
heading { level, rawText, children } =
    Element.paragraph
        (  case level of
        Block.H1 -> 
            heading_one (headingLevelToInt level) rawText 
        Block.H2 -> 
            heading_two (headingLevelToInt level) rawText 
        _ -> 
            [ Font.size 14
            , Font.family [ Font.typeface "system-ui" ]
            , Region.heading  (headingLevelToInt level)
            , Element.htmlAttribute
                (Html.Attributes.attribute "name" (rawTextToId rawText))
            , Element.htmlAttribute
                (Html.Attributes.id (rawTextToId rawText))
            ])
        children

heading_one : Int -> String -> List (Attribute msg)
heading_one level rawText =
    [ Font.size 20
    , Font.bold
    , Font.family [ Font.typeface "system-ui" ]
    , Region.heading level
    , Element.htmlAttribute
        (Html.Attributes.attribute "name" (rawTextToId rawText))
    , Element.htmlAttribute
        (Html.Attributes.id (rawTextToId rawText))
    ]

heading_two : Int -> String -> List (Attribute msg)
heading_two level rawText =
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
