module Message exposing (..)


import Json.Decode
import Json.Encode
import Json.Decode.Pipeline

type alias WeatherData =
    { position : Coordinate
    , temperature : Int
    , humidity : Int
    }

encodeWeatherData : WeatherData -> Json.Encode.Value
encodeWeatherData weatherdata =
    Json.Encode.object
        [ ( "position", encodeCoordinate weatherdata.position )
        , ( "temperature", Json.Encode.int weatherdata.temperature )
        , ( "humidity", Json.Encode.int weatherdata.humidity )
        ]

decodeWeatherData : Json.Decode.Decoder WeatherData
decodeWeatherData =
    Json.Decode.succeed WeatherData
        |> Json.Decode.Pipeline.required "position" decodeCoordinate
        |> Json.Decode.Pipeline.required "temperature" Json.Decode.int
        |> Json.Decode.Pipeline.required "humidity" Json.Decode.int

type alias Coordinate =
    { latitude : Int
    , longitude : Int
    }

encodeCoordinate : Coordinate -> Json.Encode.Value
encodeCoordinate coordinate =
    Json.Encode.object
        [ ( "latitude", Json.Encode.int coordinate.latitude )
        , ( "longitude", Json.Encode.int coordinate.longitude )
        ]

decodeCoordinate : Json.Decode.Decoder Coordinate
decodeCoordinate =
    Json.Decode.succeed Coordinate
        |> Json.Decode.Pipeline.required "latitude" Json.Decode.int
        |> Json.Decode.Pipeline.required "longitude" Json.Decode.int

type MixedData
    = GoodData WeatherData
    | BadData Coordinate

encodeMixedData : MixedData -> Json.Encode.Value
encodeMixedData mixeddata =
    case mixeddata of
        GoodData x ->
            Json.Encode.object
                [ ( "GoodData", encodeWeatherData x )
                ]

        BadData x ->
            Json.Encode.object
                [ ( "BadData", encodeCoordinate x )
                ]


decodeMixedData : Json.Decode.Decoder MixedData
decodeMixedData =
    Json.Decode.oneOf
        [ decodeMixedDataGoodData
        , decodeMixedDataBadData
        ]


decodeMixedDataGoodData : Json.Decode.Decoder MixedData
decodeMixedDataGoodData =
    Json.Decode.succeed GoodData
        |> Json.Decode.Pipeline.required "GoodData" decodeWeatherData


decodeMixedDataBadData : Json.Decode.Decoder MixedData
decodeMixedDataBadData =
    Json.Decode.succeed BadData
        |> Json.Decode.Pipeline.required "BadData" decodeCoordinate
