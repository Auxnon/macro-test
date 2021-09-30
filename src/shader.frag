#version 100
precision lowp float;
varying vec2 uv;
varying vec2 uv_screen;
varying vec2 center;

uniform vec2 ray;
uniform vec2 resolution;
uniform sampler2D normals;
uniform sampler2D albedo;
uniform sampler2D remap;
uniform sampler2D _ScreenTexture;
uniform float ratio;
uniform float time;

mat4 bayerIndex = mat4(
    vec4(00.0/16.0, 12.0/16.0, 03.0/16.0, 15.0/16.0),
    vec4(08.0/16.0, 04.0/16.0, 11.0/16.0, 07.0/16.0),
    vec4(02.0/16.0, 14.0/16.0, 01.0/16.0, 13.0/16.0),
    vec4(10.0/16.0, 06.0/16.0, 09.0/16.0, 05.0/16.0));

void main() {
    float gradient = length(uv);
    vec2 vector=(uv_screen - center);
    vec2 uv_zoom = vector * gradient + center;
    vec2 n=normalize(center);
    vec2 uv2=uv_screen+vec2(0.,3.);
    vec2 guess=vec2(80./resolution.x,48./resolution.y);

    vec2 uv=guess*(uv2);
    vec4 col = texture2D(_ScreenTexture, uv);
    vec4 alb = texture2D(albedo,uv );
    vec2 pixs=floor(vec2(resolution.x*uv_screen.x,resolution.y*uv_screen.y));
    vec4 norms=texture2D(normals,uv); //ints


    float bayerValue = bayerIndex[int(mod(pixs.x,2.))][int(mod(pixs.y,2.))];
    
    if(col.b>0.){
        
        //ints+=.1;
        
        if(norms.b>.0){
            vec3 n=normalize(vec3(.5-norms.r,.5-norms.g,norms.b-.5)); //.5-norms.g
            vec2 v=normalize(vector);
            //float c=normalize(vec2(n.x*v.x,n.y*v.x)).x;
            //t = glm::normalize(t - n * glm::dot(n, t));
            //vec2 uv2 = normalize(ray-n*dot(n,ray));//(uv-0.5*uv_screen.xy)/uv_screen.y;
            vec2 uv3=vec2(uv_screen.x-.5,uv_screen.y-.5);
            vec2 new_ray=((uv_screen)-ray);
            //vec2 pix_ray=(new_ray*resolution)/resolution;

            vec2 pix2=floor(vector*resolution)/resolution;
            float f = dot(vec3(new_ray,1.),n);
            //f*=length(pix2);
            //f*=bayerValue;
            

        
            if(uv3.x>.0){
                
               // gl_FragColor = mix(alb*.6,alb,f);
                //f*=bayerValue;

                float n=0.;
                // if(f>.1)
                // n=.5;
                vec4 c2=texture2D(remap,vec2((floor(alb.r*16.)/256.+floor(alb.b*16.)/16.),n+alb.g/2.));  //(alb.x/16.+alb.z)/16.
                c2=vec4(0,0,0,0);
                gl_FragColor = mix(c2,alb,floor(f));
                // gl_FragColor = vec4(alb);
            }else{
                //.2 to .6
                if(f>=.4 && f<.6){
                    if(mod(pixs.x+pixs.y/2.,4.)==0.){
                        f=0.;
                    }else{
                        f=1.;
                    }
                }else if(f>=.3 && f<.4){
                    if(mod(pixs.x+pixs.y/2.,2.)==0.){
                        f=0.;
                    }else{
                        f=1.;
                    }
                }else if (f<.3 && f>=.1){
                    if(mod(pixs.x+pixs.y,2.)==0.){
                        f=0.;
                    }else{
                        f=1.;
                    }
                }else if(f<.1){
                    f=0.;
                
                }else{
                    f=1.;
                }
                /*if(mod(pixs.x/2.+pixs.y/8.,2.)==0.){
                    f=0.;
                }else{
                    f=1.;
                }*/
                //$246 9
                //gb 48 96 130 255
                //63 63 116 255
                // float r=63./256.;
                // float g=63./256.;
                // float b=116./256.;
                //(floor(r*16.)+floor(b*16.)*16.)/128. +
                //(g+16.)/32.
                vec4 c2=texture2D(remap,vec2((floor(alb.r*16.)/256.+floor(alb.b*16.)/16.),.5+alb.g/2.));  //(alb.x/16.+alb.z)/16.
                
                //vec4 c=texture2D(remap, vec2(uv_screen.x,uv_screen.y)); //vec2((alb.x)/256.+(alb.z)/16.
                gl_FragColor = mix(c2,alb,f);
            }
            //gl_FragColor = vec4(col);//vec4(1,0,0,1);//mix(col*.6,col,f);//vec4(f,0.,0.,f);
        }else{
            gl_FragColor = alb;
        }
        //gl_FragColor=texture2D(remap, vec2(uv_screen.x*2.,uv_screen.y*8.));
     gl_FragColor = alb;
    }else{ 
        gl_FragColor = alb;
    }

    // if (uv2.y<0.1 && uv2.x<0.1){
    //     gl_FragColor.r=0.;
    // }
     //gl_FragColor =mix(alb,vec4(length(vector),0.5,0.5,1.),1.);
    
    //gl_FragColor = vec4(1.,0.,0.,1.);
    // if(norms.b==0.){
    //     gl_FragColor=vec4(1,0,0,1);
    // }
}
