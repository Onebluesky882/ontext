export namespace main {

	export class PermissionStatus {
	    accessibility: boolean;
	    microphone: string;

	    static createFrom(source: any = {}) {
	        return new PermissionStatus(source);
	    }

	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.accessibility = source["accessibility"];
	        this.microphone = source["microphone"];
	    }
	}

	export class PasteResult {
	    success: boolean;
	    error?: string;
	
	    static createFrom(source: any = {}) {
	        return new PasteResult(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.success = source["success"];
	        this.error = source["error"];
	    }
	}

}

