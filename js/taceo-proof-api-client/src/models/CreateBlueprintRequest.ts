/* tslint:disable */
/* eslint-disable */
/**
 * Private Proof Delegation-CCL
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 * Contact: hello@taceo.io
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { mapValues } from '../runtime';
import type { BlueprintCurve } from './BlueprintCurve';
import {
    BlueprintCurveFromJSON,
    BlueprintCurveFromJSONTyped,
    BlueprintCurveToJSON,
    BlueprintCurveToJSONTyped,
} from './BlueprintCurve';
import type { BlueprintType } from './BlueprintType';
import {
    BlueprintTypeFromJSON,
    BlueprintTypeFromJSONTyped,
    BlueprintTypeToJSON,
    BlueprintTypeToJSONTyped,
} from './BlueprintType';

/**
 * 
 * @export
 * @interface CreateBlueprintRequest
 */
export interface CreateBlueprintRequest {
    /**
     * 
     * @type {BlueprintCurve}
     * @memberof CreateBlueprintRequest
     */
    curve: BlueprintCurve;
    /**
     * 
     * @type {string}
     * @memberof CreateBlueprintRequest
     */
    name: string;
    /**
     * 
     * @type {number}
     * @memberof CreateBlueprintRequest
     */
    nodeProvider0: number;
    /**
     * 
     * @type {number}
     * @memberof CreateBlueprintRequest
     */
    nodeProvider1: number;
    /**
     * 
     * @type {number}
     * @memberof CreateBlueprintRequest
     */
    nodeProvider2: number;
    /**
     * 
     * @type {BlueprintType}
     * @memberof CreateBlueprintRequest
     */
    typ: BlueprintType;
}



/**
 * Check if a given object implements the CreateBlueprintRequest interface.
 */
export function instanceOfCreateBlueprintRequest(value: object): value is CreateBlueprintRequest {
    if (!('curve' in value) || value['curve'] === undefined) return false;
    if (!('name' in value) || value['name'] === undefined) return false;
    if (!('nodeProvider0' in value) || value['nodeProvider0'] === undefined) return false;
    if (!('nodeProvider1' in value) || value['nodeProvider1'] === undefined) return false;
    if (!('nodeProvider2' in value) || value['nodeProvider2'] === undefined) return false;
    if (!('typ' in value) || value['typ'] === undefined) return false;
    return true;
}

export function CreateBlueprintRequestFromJSON(json: any): CreateBlueprintRequest {
    return CreateBlueprintRequestFromJSONTyped(json, false);
}

export function CreateBlueprintRequestFromJSONTyped(json: any, ignoreDiscriminator: boolean): CreateBlueprintRequest {
    if (json == null) {
        return json;
    }
    return {
        
        'curve': BlueprintCurveFromJSON(json['curve']),
        'name': json['name'],
        'nodeProvider0': json['node_provider0'],
        'nodeProvider1': json['node_provider1'],
        'nodeProvider2': json['node_provider2'],
        'typ': BlueprintTypeFromJSON(json['typ']),
    };
}

export function CreateBlueprintRequestToJSON(json: any): CreateBlueprintRequest {
    return CreateBlueprintRequestToJSONTyped(json, false);
}

export function CreateBlueprintRequestToJSONTyped(value?: CreateBlueprintRequest | null, ignoreDiscriminator: boolean = false): any {
    if (value == null) {
        return value;
    }

    return {
        
        'curve': BlueprintCurveToJSON(value['curve']),
        'name': value['name'],
        'node_provider0': value['nodeProvider0'],
        'node_provider1': value['nodeProvider1'],
        'node_provider2': value['nodeProvider2'],
        'typ': BlueprintTypeToJSON(value['typ']),
    };
}

